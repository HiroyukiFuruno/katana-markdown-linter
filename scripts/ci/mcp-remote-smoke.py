#!/usr/bin/env python3
import argparse
import http.client
import json
import os
import socket
import subprocess
import sys
import time
from pathlib import Path


class McpRemoteSmoke:
    token = "kml-remote-smoke-token"
    endpoint = "/mcp"

    def __init__(self, command: list[str]):
        self.command = command
        self.port = self.find_port()
        self.process = None

    def run(self) -> None:
        self.process = self.start_server()
        try:
            self.wait_until_ready()
            self.assert_unauthorized()
            self.assert_initialized()
            self.assert_tools_are_text_only()
            self.assert_check_text()
            self.assert_size_limit()
        finally:
            self.stop_server()

    def start_server(self) -> subprocess.Popen[str]:
        env = os.environ.copy()
        env["KML_MCP_REMOTE_ADDR"] = f"127.0.0.1:{self.port}"
        env["KML_MCP_REMOTE_TOKEN"] = self.token
        env["KML_MCP_REMOTE_MAX_BODY_BYTES"] = "4096"
        env["KML_MCP_REMOTE_TIMEOUT_MS"] = "5000"
        env["KML_MCP_REMOTE_MAX_CONCURRENCY"] = "2"
        return subprocess.Popen(
            self.command,
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )

    def wait_until_ready(self) -> None:
        deadline = time.time() + 10
        while time.time() < deadline:
            if self.process is not None and self.process.poll() is not None:
                raise RuntimeError(self.stderr_message("remote MCP server exited"))
            try:
                with socket.create_connection(("127.0.0.1", self.port), timeout=0.2):
                    return
            except OSError:
                time.sleep(0.1)
        raise RuntimeError(self.stderr_message("remote MCP server did not open the port"))

    def assert_unauthorized(self) -> None:
        status, _, body = self.post_json(self.initialize_request(1), token=None)
        self.assert_equal(status, 401, f"unauthorized status: {body}")

    def assert_initialized(self) -> None:
        status, _, body = self.post_json(self.initialize_request(2))
        self.assert_equal(status, 200, f"initialize status: {body}")
        payload = json.loads(body)
        server_info = payload["result"]["serverInfo"]
        self.assert_equal(server_info["name"], "kml-mcp-remote", "server name")

    def assert_tools_are_text_only(self) -> None:
        response = self.request(3, "tools/list", {}, protocol_header=True)
        tool_names = {tool["name"] for tool in response["result"]["tools"]}
        required = {"check_text", "fix_text", "config_validate", "rule_list", "rule_get"}
        forbidden = {"check_file", "check_directory", "fix_file_preview", "fix_file_apply"}
        if not required.issubset(tool_names):
            raise AssertionError(f"missing remote text tools: {sorted(required - tool_names)}")
        if forbidden.intersection(tool_names):
            raise AssertionError(f"remote server exposed workspace tools: {sorted(forbidden.intersection(tool_names))}")

    def assert_check_text(self) -> None:
        response = self.call_tool(4, "check_text", {"content": "#Title\n"})
        if response["issue_count"] <= 0:
            raise AssertionError(f"check_text should report diagnostics: {response}")

    def assert_size_limit(self) -> None:
        oversized = self.initialize_request(5)
        oversized["params"]["clientInfo"]["name"] = "x" * 5000
        status, _, body = self.post_json(oversized)
        self.assert_equal(status, 413, f"size-limit status: {body}")

    def call_tool(self, request_id: int, name: str, arguments: dict[str, object]) -> dict:
        payload = self.request(request_id, "tools/call", {"name": name, "arguments": arguments}, True)
        result = payload["result"]
        if result.get("isError"):
            raise AssertionError(f"tool call failed: {result}")
        return result.get("structuredContent") or json.loads(result["content"][0]["text"])

    def request(self, request_id: int, method: str, params: dict[str, object], protocol_header: bool = False) -> dict:
        status, _, body = self.post_json(
            {"jsonrpc": "2.0", "id": request_id, "method": method, "params": params},
            protocol_header=protocol_header,
        )
        self.assert_equal(status, 200, f"{method} status: {body}")
        payload = json.loads(body)
        if "error" in payload:
            raise AssertionError(f"{method} returned JSON-RPC error: {payload}")
        return payload

    def post_json(self, payload: dict[str, object], token: str | None = token, protocol_header: bool = False) -> tuple[int, dict[str, str], str]:
        body = json.dumps(payload, separators=(",", ":"))
        headers = {
            "Accept": "application/json, text/event-stream",
            "Content-Type": "application/json",
            "Host": f"127.0.0.1:{self.port}",
        }
        if token is not None:
            headers["Authorization"] = f"Bearer {token}"
        if protocol_header:
            headers["MCP-Protocol-Version"] = "2025-06-18"
        connection = http.client.HTTPConnection("127.0.0.1", self.port, timeout=5)
        try:
            connection.request("POST", self.endpoint, body=body, headers=headers)
            response = connection.getresponse()
            data = response.read().decode("utf-8")
            return response.status, dict(response.getheaders()), data
        finally:
            connection.close()

    @staticmethod
    def initialize_request(request_id: int) -> dict[str, object]:
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "kml-mcp-remote-smoke", "version": "1"},
            },
        }

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
    def find_port() -> int:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            sock.bind(("127.0.0.1", 0))
            return int(sock.getsockname()[1])

    @staticmethod
    def assert_equal(actual: object, expected: object, label: str) -> None:
        if actual != expected:
            raise AssertionError(f"{label}: expected {expected!r}, got {actual!r}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Smoke test kml-mcp-remote over Streamable HTTP.")
    parser.add_argument("--bin", type=Path, help="Path to kml-mcp-remote executable.")
    parser.add_argument("--command", nargs=argparse.REMAINDER, help="Command that starts kml-mcp-remote.")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.command:
        command = args.command[1:] if args.command[0] == "--" else args.command
    elif args.bin:
        command = [str(args.bin)]
    else:
        print("--bin or --command is required", file=sys.stderr)
        return 1
    if args.bin and not args.bin.exists():
        print(f"kml-mcp-remote binary not found: {args.bin}", file=sys.stderr)
        return 1
    McpRemoteSmoke(command).run()
    print("MCP remote smoke passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
