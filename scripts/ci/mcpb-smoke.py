#!/usr/bin/env python3
import argparse
import json
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path

# Reuse logic from mcp-stdio-smoke.py by wrapping it or duplicating necessary parts
# Since we need to test the .mcpb artifact, we must extract it first.

class McpbSmoke:
    def __init__(self, mcpb_path: Path):
        self.mcpb_path = mcpb_path
        self.process = None

    def run(self) -> None:
        with tempfile.TemporaryDirectory(prefix="kml-mcpb-smoke-") as directory:
            extract_dir = Path(directory)
            print(f"Extracting {self.mcpb_path} to {extract_dir}...")
            with zipfile.ZipFile(self.mcpb_path, 'r') as zip_ref:
                zip_ref.extractall(extract_dir)

            manifest_path = extract_dir / "manifest.json"
            if not manifest_path.exists():
                raise AssertionError("manifest.json missing in .mcpb")

            manifest = json.loads(manifest_path.read_text())
            binary_name = manifest["server"]["mcp_config"]["command"]
            binary_path = extract_dir / binary_name

            if not binary_path.exists():
                raise AssertionError(f"Binary {binary_name} missing in .mcpb")

            # Make it executable
            binary_path.chmod(0o755)

            print(f"Running smoke test on {binary_path}...")
            self.run_stdio_smoke(binary_path, extract_dir)

    def run_stdio_smoke(self, binary_path: Path, workspace: Path) -> None:
        # We can't easily import from scripts/ci/mcp-stdio-smoke.py without hacks
        # so we run it as a subprocess pointing to the extracted binary
        smoke_script = Path(__file__).parent / "mcp-stdio-smoke.py"
        subprocess.run([sys.executable, str(smoke_script), "--bin", str(binary_path)], check=True)

def main() -> int:
    parser = argparse.ArgumentParser(description="Smoke test .mcpb bundle.")
    parser.add_argument("--mcpb", required=True, type=Path, help="Path to .mcpb bundle.")
    args = parser.parse_args()

    if not args.mcpb.exists():
        print(f".mcpb bundle not found: {args.mcpb}", file=sys.stderr)
        return 1

    try:
        McpbSmoke(args.mcpb).run()
        print("MCPB smoke passed")
        return 0
    except Exception as e:
        print(f"MCPB smoke failed: {e}", file=sys.stderr)
        return 1

if __name__ == "__main__":
    raise SystemExit(main())
