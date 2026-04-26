import os
import subprocess
import time
import requests
import unittest
import json

class TestRemoteMcpServer(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.port = 3001
        cls.auth_token = "test-token"
        cls.workspace_root = os.getcwd()

        # Build the server
        subprocess.run(["cargo", "build", "--features", "remote", "--bin", "kml-mcp-remote"], check=True)

        # Start the server
        cls.server_proc = subprocess.Popen(
            ["./target/debug/kml-mcp-remote"],
            env={
                **os.environ,
                "PORT": str(cls.port),
                "KML_AUTH_TOKEN": cls.auth_token,
                "KML_WORKSPACE_ROOT": cls.workspace_root,
                "RUST_LOG": "debug"
            }
        )
        # Give it a moment to start
        time.sleep(2)

    @classmethod
    def tearDownClass(cls):
        cls.server_proc.terminate()
        cls.server_proc.wait()

    def test_unauthorized_access(self):
        url = f"http://localhost:{self.port}/mcp"
        # Try without token
        resp = requests.post(url, json={"method": "tools/list", "params": {}, "jsonrpc": "2.0", "id": 1})
        self.assertEqual(resp.status_code, 401)

    def test_authorized_access_and_session(self):
        url = f"http://localhost:{self.port}/mcp"
        headers = {
            "Authorization": f"Bearer {self.auth_token}",
            "Accept": "application/json, text/event-stream"
        }

        # 1. Initialize session
        resp = requests.post(url, headers=headers, json={
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "test-client", "version": "1.0"}
            },
            "jsonrpc": "2.0",
            "id": 1
        })
        self.assertEqual(resp.status_code, 200)
        session_id = resp.headers.get("mcp-session-id")
        self.assertTrue(session_id)

        # 2. Call tools/list
        headers["mcp-session-id"] = session_id
        resp = requests.post(url, headers=headers, json={
            "method": "tools/list",
            "params": {},
            "jsonrpc": "2.0",
            "id": 2
        })
        self.assertEqual(resp.status_code, 200)

    def test_workspace_tool_restricted(self):
        url = f"http://localhost:{self.port}/mcp"
        headers = {
            "Authorization": f"Bearer {self.auth_token}",
            "Accept": "application/json, text/event-stream"
        }

        # 1. Initialize session
        resp = requests.post(url, headers=headers, json={
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "test-client", "version": "1.0"}
            },
            "jsonrpc": "2.0",
            "id": 1
        })
        session_id = resp.headers.get("mcp-session-id")
        headers["mcp-session-id"] = session_id

        # 2. Call check_file (restricted)
        resp = requests.post(url, headers=headers, json={
            "method": "tools/call",
            "params": {
                "name": "check_file",
                "arguments": {"path": "README.md"}
            },
            "jsonrpc": "2.0",
            "id": 2
        }, stream=True)
        self.assertEqual(resp.status_code, 200)

        found_error = False
        for line in resp.iter_lines():
            if line:
                decoded_line = line.decode('utf-8')
                if decoded_line.startswith("data: "):
                    data_str = decoded_line[6:].strip()
                    if not data_str:
                        continue
                    data = json.loads(data_str)
                    if "check_file is only available in local mode" in str(data):
                         found_error = True
                         break
        self.assertTrue(found_error)

if __name__ == "__main__":
    unittest.main()
