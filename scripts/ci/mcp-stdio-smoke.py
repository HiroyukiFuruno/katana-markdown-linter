#!/usr/bin/env python3
import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path


class McpStdioSmoke:
    def __init__(self, binary: Path):
        self.binary = binary
        self.process = None

    def run(self) -> None:
        with tempfile.TemporaryDirectory(prefix="kml-mcp-stdio-") as directory:
            workspace = Path(directory)
            target = workspace / "bad.md"
            target.write_text("#Title\n", encoding="utf-8")
            (workspace / ".markdownlint.json").write_text('{"default": true}\n', encoding="utf-8")
            self.process = self.start_server(workspace)
            try:
                self.assert_initialized()
                self.assert_tools()
                self.assert_check_file()
                self.assert_preview_does_not_write(target)
                self.assert_apply_requires_confirmation()
                self.assert_apply_writes_file(target)
                self.assert_root_boundary()
            finally:
                self.stop_server()

    def start_server(self, workspace: Path) -> subprocess.Popen[str]:
        return subprocess.Popen(
            [str(self.binary), "--workspace-root", str(workspace)],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )

    def assert_initialized(self) -> None:
        self.send(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "kml-mcp-stdio-smoke", "version": "1"},
                },
            }
        )
        payload = self.receive(1)
        server_info = payload["result"]["serverInfo"]
        self.assert_equal(server_info["name"], "kml-mcp", "server name")
        self.send({"jsonrpc": "2.0", "method": "notifications/initialized", "params": {}})

    def assert_tools(self) -> None:
        self.send({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}})
        payload = self.receive(2)
        tool_names = {tool["name"] for tool in payload["result"]["tools"]}
        required = {"check_file", "fix_file_preview", "fix_file_apply"}
        if not required.issubset(tool_names):
            raise AssertionError(f"missing MCP tools: {sorted(required - tool_names)}")
        if "fix_directory_apply" in tool_names:
            raise AssertionError("fix_directory_apply must not be exposed")

    def assert_check_file(self) -> None:
        response = self.call_tool(3, "check_file", {"path": "bad.md"})
        if response["issue_count"] <= 0:
            raise AssertionError(f"check_file should report diagnostics: {response}")

    def assert_preview_does_not_write(self, target: Path) -> None:
        response = self.call_tool(4, "fix_file_preview", {"path": "bad.md"})
        if not response["changed"]:
            raise AssertionError(f"fix_file_preview should report a change: {response}")
        if "-#Title" not in response["diff"] or "+# Title" not in response["diff"]:
            raise AssertionError(f"fix_file_preview should include heading diff: {response}")
        self.assert_equal(target.read_text(encoding="utf-8"), "#Title\n", "preview write guard")

    def assert_apply_requires_confirmation(self) -> None:
        response = self.call_tool(5, "fix_file_apply", {"path": "bad.md", "apply": False})
        if not response.get("isError"):
            raise AssertionError(f"fix_file_apply must reject apply=false: {response}")
        text = response["content"][0]["text"]
        if "apply: true" not in text:
            raise AssertionError(f"fix_file_apply error should mention apply=true: {response}")

    def assert_apply_writes_file(self, target: Path) -> None:
        response = self.call_tool(6, "fix_file_apply", {"path": "bad.md", "apply": True})
        if not response["changed"]:
            raise AssertionError(f"fix_file_apply should change file: {response}")
        self.assert_equal(target.read_text(encoding="utf-8"), "# Title\n", "apply output")

    def assert_root_boundary(self) -> None:
        response = self.call_tool(7, "check_file", {"path": "../bad.md"})
        if not response.get("isError"):
            raise AssertionError(f"root boundary must reject parent traversal: {response}")
        text = response["content"][0]["text"]
        if "inside the workspace root" not in text:
            raise AssertionError(f"root boundary error should be explicit: {response}")

    def call_tool(self, request_id: int, name: str, arguments: dict[str, object]) -> dict:
        self.send(
            {
                "jsonrpc": "2.0",
                "id": request_id,
                "method": "tools/call",
                "params": {"name": name, "arguments": arguments},
            }
        )
        payload = self.receive(request_id)
        if "error" in payload:
            return {"isError": True, "content": [{"text": payload["error"]["message"]}]}
        result = payload["result"]
        if result.get("isError"):
            return result
        return result.get("structuredContent") or json.loads(result["content"][0]["text"])

    def send(self, message: dict[str, object]) -> None:
        if self.process is None or self.process.stdin is None:
            raise RuntimeError("MCP server is not running")
        self.process.stdin.write(json.dumps(message, separators=(",", ":")) + "\n")
        self.process.stdin.flush()

    def receive(self, request_id: int) -> dict:
        if self.process is None or self.process.stdout is None:
            raise RuntimeError("MCP server is not running")
        while True:
            line = self.process.stdout.readline()
            if not line:
                raise RuntimeError(self.stderr_message("MCP server closed stdout"))
            payload = json.loads(line)
            if payload.get("id") == request_id:
                return payload

    def stop_server(self) -> None:
        if self.process is None:
            return
        self.process.terminate()
        try:
            self.process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait()

    def stderr_message(self, message: str) -> str:
        if self.process is None or self.process.stderr is None:
            return message
        stderr = self.process.stderr.read()
        if not stderr:
            return message
        return f"{message}\n{stderr}"

    @staticmethod
    def assert_equal(actual: object, expected: object, label: str) -> None:
        if actual != expected:
            raise AssertionError(f"{label}: expected {expected!r}, got {actual!r}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Smoke test kml-mcp over MCP stdio JSON-RPC.")
    parser.add_argument("--bin", required=True, type=Path, help="Path to kml-mcp executable.")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if not args.bin.exists():
        print(f"kml-mcp binary not found: {args.bin}", file=sys.stderr)
        return 1
    McpStdioSmoke(args.bin).run()
    print("MCP stdio smoke passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
