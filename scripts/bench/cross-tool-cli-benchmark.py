#!/usr/bin/env python3
"""Benchmark kml against peer Markdown linter CLIs."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import re
import shlex
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from typing import Any


SCHEMA_VERSION = 1
DEFAULT_RUNS = 5
DEFAULT_WARMUP = 1
COMMON_SUBSET_RULES = [
    "MD001",
    "MD004",
    "MD005",
    "MD009",
    "MD010",
    "MD012",
    "MD013",
    "MD014",
    "MD021",
    "MD022",
    "MD023",
    "MD024",
    "MD025",
    "MD026",
    "MD028",
    "MD029",
    "MD030",
    "MD031",
    "MD033",
    "MD034",
    "MD035",
    "MD036",
    "MD037",
    "MD038",
    "MD039",
    "MD040",
    "MD041",
    "MD046",
    "MD047",
]


@dataclass(frozen=True)
class Tool:
    name: str
    binary: pathlib.Path | None
    required: bool


@dataclass(frozen=True)
class Case:
    tool: Tool
    mode: str
    workflow: str
    corpus_kind: str
    corpus: pathlib.Path
    config: pathlib.Path | None
    source_corpus: pathlib.Path | None = None

    @property
    def name(self) -> str:
        return f"{self.workflow}_{self.corpus_kind}"


def parse_args() -> argparse.Namespace:
    if len(sys.argv) > 1 and sys.argv[1] == "run-case":
        return parse_run_case_args(sys.argv[2:])
    return parse_benchmark_args(sys.argv[1:])


def parse_benchmark_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.set_defaults(command="benchmark")
    parser.add_argument("--mode", choices=["default", "common", "all"], default="all")
    parser.add_argument("--workflow", choices=["check", "fix", "all"], default="all")
    parser.add_argument("--tools", default="kml,mado,rumdl")
    parser.add_argument("--kml", default="target/release/kml")
    parser.add_argument("--mado", default=None)
    parser.add_argument("--rumdl", default=None)
    parser.add_argument(
        "--clean-corpus",
        default="tests/fixtures/cross-tool-benchmark/clean",
    )
    parser.add_argument(
        "--dirty-corpus",
        default="tests/fixtures/cross-tool-benchmark/dirty",
    )
    parser.add_argument("--output", default="target/cross-tool-benchmark.json")
    parser.add_argument("--summary", default="target/cross-tool-benchmark.md")
    parser.add_argument("--runs", type=int, default=DEFAULT_RUNS)
    parser.add_argument("--warmup", type=int, default=DEFAULT_WARMUP)
    parser.add_argument("--no-hyperfine", action="store_true")
    return parser.parse_args(argv)


def parse_run_case_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Run one normalized benchmark case.")
    parser.set_defaults(command="run-case")
    parser.add_argument("--tool", required=True, choices=["kml", "mado", "rumdl"])
    parser.add_argument("--binary", required=True)
    parser.add_argument("--mode", required=True, choices=["default", "common"])
    parser.add_argument("--workflow", required=True, choices=["check", "fix"])
    parser.add_argument("--corpus-kind", required=True, choices=["clean", "dirty"])
    parser.add_argument("--corpus", required=True)
    parser.add_argument("--config")
    return parser.parse_args(argv)


def main() -> int:
    args = parse_args()
    if args.command == "run-case":
        return run_case_command(args)
    return benchmark_command(args)


def benchmark_command(args: argparse.Namespace) -> int:
    if args.runs <= 0:
        raise SystemExit("--runs must be positive")
    if args.warmup < 0:
        raise SystemExit("--warmup must be zero or positive")

    output_path = pathlib.Path(args.output).resolve()
    summary_path = pathlib.Path(args.summary).resolve()
    config_dir = output_path.parent / "cross-tool-benchmark-configs"
    config_dir.mkdir(parents=True, exist_ok=True)
    configs = write_common_configs(config_dir)

    selected_tools = [tool.strip() for tool in args.tools.split(",") if tool.strip()]
    tools = discover_tools(args, selected_tools)
    modes = ["default", "common"] if args.mode == "all" else [args.mode]
    workflows = ["check", "fix"] if args.workflow == "all" else [args.workflow]
    clean_corpus = pathlib.Path(args.clean_corpus).resolve()
    dirty_corpus = pathlib.Path(args.dirty_corpus).resolve()
    validate_corpus(clean_corpus, "clean")
    validate_corpus(dirty_corpus, "dirty")

    use_hyperfine = bool(shutil.which("hyperfine")) and not args.no_hyperfine
    timing_method = "hyperfine" if use_hyperfine else "fallback"

    cases: list[dict[str, Any]] = []
    with tempfile.TemporaryDirectory(prefix="kml-cross-tool-corpus-") as tmp:
        detached_clean = pathlib.Path(tmp) / "clean"
        detached_dirty = pathlib.Path(tmp) / "dirty"
        shutil.copytree(clean_corpus, detached_clean)
        shutil.copytree(dirty_corpus, detached_dirty)
        benchmark_cases = build_cases(
            tools,
            modes,
            workflows,
            detached_clean,
            detached_dirty,
            clean_corpus,
            dirty_corpus,
            configs,
        )
        for case in benchmark_cases:
            if case.tool.binary is None:
                cases.append(skipped_case(case, f"{case.tool.name} binary was not found"))
                continue
            if case.workflow == "fix" and case.tool.name == "mado":
                cases.append(skipped_case(case, "mado fix workflow is not configured"))
                continue
            fix_validation = validate_fix_case(case) if case.workflow == "fix" else None
            if use_hyperfine:
                cases.append(run_hyperfine_case(case, args.runs, args.warmup, fix_validation))
            else:
                cases.append(run_fallback_case(case, args.runs, args.warmup, fix_validation))

    report = {
        "schema_version": SCHEMA_VERSION,
        "generated_by": "scripts/bench/cross-tool-cli-benchmark.py",
        "timing_method": timing_method,
        "runs": args.runs,
        "warmup": args.warmup,
        "common_subset_rules": COMMON_SUBSET_RULES,
        "cases": cases,
    }
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    summary_path.parent.mkdir(parents=True, exist_ok=True)
    summary_path.write_text(markdown_summary(report) + "\n", encoding="utf-8")
    print_summary(report, output_path, summary_path)

    required_failures = [
        case
        for case in cases
        if case["tool"] == "kml" and case["status"] not in {"measured"}
    ]
    return 1 if required_failures else 0


def discover_tools(args: argparse.Namespace, selected: list[str]) -> list[Tool]:
    tools: list[Tool] = []
    for name in selected:
        if name == "kml":
            binary = pathlib.Path(args.kml)
            tools.append(
                Tool("kml", binary.resolve() if binary.exists() else None, required=True)
            )
        elif name == "mado":
            tools.append(Tool("mado", discover_optional_binary(args.mado, "mado"), False))
        elif name == "rumdl":
            tools.append(Tool("rumdl", discover_optional_binary(args.rumdl, "rumdl"), False))
        else:
            raise SystemExit(f"unknown tool: {name}")
    return tools


def discover_optional_binary(explicit: str | None, name: str) -> pathlib.Path | None:
    if explicit:
        path = pathlib.Path(explicit)
        return path.resolve() if path.exists() else None
    discovered = shutil.which(name)
    return pathlib.Path(discovered) if discovered else None


def validate_corpus(path: pathlib.Path, name: str) -> None:
    if not path.is_dir():
        raise SystemExit(f"{name} corpus does not exist: {path}")
    if not list(path.rglob("*.md")):
        raise SystemExit(f"{name} corpus has no Markdown files: {path}")


def write_common_configs(config_dir: pathlib.Path) -> dict[str, pathlib.Path]:
    kml_config = config_dir / ".markdownlint.common.json"
    mado_config = config_dir / "mado.common.toml"
    rumdl_config = config_dir / "rumdl.common.toml"

    kml_payload: dict[str, Any] = {"default": False}
    kml_payload.update({rule: True for rule in COMMON_SUBSET_RULES})
    kml_config.write_text(json.dumps(kml_payload, indent=2) + "\n", encoding="utf-8")

    quoted_rules = ", ".join(json.dumps(rule) for rule in COMMON_SUBSET_RULES)
    mado_config.write_text(
        "[lint]\n"
        "respect-ignore = true\n"
        "respect-gitignore = true\n"
        f"rules = [ {quoted_rules} ]\n",
        encoding="utf-8",
    )
    rumdl_config.write_text(
        "[global]\n"
        f"enable = [ {quoted_rules} ]\n"
        "respect-gitignore = true\n",
        encoding="utf-8",
    )
    return {"kml": kml_config, "mado": mado_config, "rumdl": rumdl_config}


def build_cases(
    tools: list[Tool],
    modes: list[str],
    workflows: list[str],
    clean_corpus: pathlib.Path,
    dirty_corpus: pathlib.Path,
    source_clean_corpus: pathlib.Path,
    source_dirty_corpus: pathlib.Path,
    configs: dict[str, pathlib.Path],
) -> list[Case]:
    cases: list[Case] = []
    for tool in tools:
        for mode in modes:
            config = configs[tool.name] if mode == "common" else None
            if "check" in workflows:
                cases.append(
                    Case(
                        tool,
                        mode,
                        "check",
                        "clean",
                        clean_corpus,
                        config,
                        source_clean_corpus,
                    )
                )
                cases.append(
                    Case(
                        tool,
                        mode,
                        "check",
                        "dirty",
                        dirty_corpus,
                        config,
                        source_dirty_corpus,
                    )
                )
            if "fix" in workflows:
                cases.append(
                    Case(
                        tool,
                        mode,
                        "fix",
                        "dirty",
                        dirty_corpus,
                        config,
                        source_dirty_corpus,
                    )
                )
    return cases


def run_case_command(args: argparse.Namespace) -> int:
    case = Case(
        Tool(args.tool, pathlib.Path(args.binary).resolve(), required=args.tool == "kml"),
        args.mode,
        args.workflow,
        args.corpus_kind,
        pathlib.Path(args.corpus).resolve(),
        pathlib.Path(args.config).resolve() if args.config else None,
    )
    completed = execute_case_once(case)
    if completed.returncode in expected_exit_codes(case):
        return 0
    sys.stderr.write(completed.stderr)
    sys.stderr.write(completed.stdout)
    return completed.returncode or 1


def run_fallback_case(
    case: Case,
    runs: int,
    warmup: int,
    fix_validation: dict[str, Any] | None,
) -> dict[str, Any]:
    version = tool_version(case.tool)
    observed_exit_codes: list[int] = []
    for _ in range(warmup):
        completed = execute_case_once(case)
        if completed.returncode not in expected_exit_codes(case):
            return failed_case(case, version, completed, "warmup", fix_validation)

    samples: list[float] = []
    for _ in range(runs):
        start = time.perf_counter()
        completed = execute_case_once(case)
        elapsed_ms = (time.perf_counter() - start) * 1000.0
        observed_exit_codes.append(completed.returncode)
        if completed.returncode not in expected_exit_codes(case):
            return failed_case(case, version, completed, "measure", fix_validation)
        samples.append(elapsed_ms)

    return measured_case(
        case,
        version,
        "fallback",
        samples,
        observed_exit_codes,
        fix_validation,
    )


def run_hyperfine_case(
    case: Case,
    runs: int,
    warmup: int,
    fix_validation: dict[str, Any] | None,
) -> dict[str, Any]:
    version = tool_version(case.tool)
    assert case.tool.binary is not None
    with tempfile.TemporaryDirectory(prefix="kml-cross-tool-hyperfine-") as tmp:
        export_path = pathlib.Path(tmp) / "hyperfine.json"
        command = [
            sys.executable,
            str(pathlib.Path(__file__).resolve()),
            "run-case",
            "--tool",
            case.tool.name,
            "--binary",
            str(case.tool.binary),
            "--mode",
            case.mode,
            "--workflow",
            case.workflow,
            "--corpus-kind",
            case.corpus_kind,
            "--corpus",
            str(case.corpus),
        ]
        if case.config:
            command.extend(["--config", str(case.config)])
        hyperfine_command = [
            "hyperfine",
            "--warmup",
            str(warmup),
            "--runs",
            str(runs),
            "--export-json",
            str(export_path),
            "--command-name",
            case_key(case),
            shlex.join(command),
        ]
        completed = subprocess.run(
            hyperfine_command,
            text=True,
            capture_output=True,
            check=False,
            env=isolated_env(),
            cwd=isolated_cwd(),
        )
        if completed.returncode != 0:
            return failed_case(case, version, completed, "hyperfine", fix_validation)
        payload = json.loads(export_path.read_text(encoding="utf-8"))
        times = [float(value) * 1000.0 for value in payload["results"][0]["times"]]
        return measured_case(case, version, "hyperfine", times, [], fix_validation)


def validate_fix_case(case: Case) -> dict[str, Any]:
    if case.tool.binary is None:
        return {"status": "skipped", "reason": "tool binary was not found"}

    source = case.source_corpus or case.corpus
    source_digest_before = directory_digest(source)
    with tempfile.TemporaryDirectory(prefix=f"kml-cross-tool-fix-validate-{case.tool.name}-") as tmp:
        workspace = pathlib.Path(tmp) / "workspace"
        shutil.copytree(case.corpus, workspace)
        workspace_digest_before = directory_digest(workspace)
        check_case = Case(
            case.tool,
            case.mode,
            "check",
            case.corpus_kind,
            workspace,
            case.config,
            case.source_corpus,
        )
        before = run_validation_command(check_case, workspace)
        fixed = run_validation_command(case, workspace)
        workspace_digest_after = directory_digest(workspace)
        after = run_validation_command(check_case, workspace)

    source_digest_after = directory_digest(source)
    before_issues = issue_count_from_output(before)
    after_issues = issue_count_from_output(after)
    workspace_changed = workspace_digest_before != workspace_digest_after
    source_changed = source_digest_before != source_digest_after
    status = "passed" if after.returncode == 0 and workspace_changed and not source_changed else "remaining_issues"
    if source_changed:
        status = "source_changed"
    elif not workspace_changed:
        status = "no_changes"

    return {
        "status": status,
        "before_check_exit_code": before.returncode,
        "fix_exit_code": fixed.returncode,
        "after_check_exit_code": after.returncode,
        "before_issues": before_issues,
        "after_issues": after_issues,
        "workspace_changed": workspace_changed,
        "source_changed": source_changed,
    }


def run_validation_command(
    case: Case,
    corpus: pathlib.Path,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        validation_command_for_case(case, corpus),
        text=True,
        capture_output=True,
        check=False,
        env=isolated_env(),
        cwd=isolated_cwd(),
    )


def validation_command_for_case(case: Case, corpus: pathlib.Path) -> list[str]:
    command = command_for_case(case, corpus)
    if case.tool.name == "kml":
        command.extend(["--output", "json"])
    return command


def directory_digest(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    for file in sorted(path.rglob("*")):
        if not file.is_file():
            continue
        digest.update(str(file.relative_to(path)).encode())
        digest.update(b"\0")
        digest.update(file.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


def issue_count_from_output(completed: subprocess.CompletedProcess[str]) -> int | None:
    output = completed.stdout.strip()
    if output.startswith("{"):
        try:
            payload = json.loads(output)
            return int(payload["summary"]["total_issues"])
        except (KeyError, TypeError, ValueError, json.JSONDecodeError):
            return None
    combined = f"{completed.stdout}\n{completed.stderr}"
    if re.search(r"\bNo issues\b", combined) or re.search(r"\bSuccess: No issues\b", combined):
        return 0
    match = re.search(r"Found\s+(\d+)\s+(?:issues|errors)", combined)
    if match:
        return int(match.group(1))
    return None


def execute_case_once(case: Case) -> subprocess.CompletedProcess[str]:
    if case.tool.binary is None:
        raise RuntimeError(f"{case.tool.name} binary is missing")
    if case.workflow == "fix":
        with tempfile.TemporaryDirectory(prefix=f"kml-cross-tool-{case.tool.name}-") as tmp:
            workspace = pathlib.Path(tmp) / "workspace"
            shutil.copytree(case.corpus, workspace)
            command = command_for_case(case, workspace)
            return subprocess.run(
                command,
                text=True,
                capture_output=True,
                check=False,
                env=isolated_env(),
                cwd=isolated_cwd(),
            )
    command = command_for_case(case, case.corpus)
    return subprocess.run(
        command,
        text=True,
        capture_output=True,
        check=False,
        env=isolated_env(),
        cwd=isolated_cwd(),
    )


def command_for_case(case: Case, corpus: pathlib.Path) -> list[str]:
    assert case.tool.binary is not None
    binary = str(case.tool.binary)
    config_args = config_args_for_tool(case)
    if case.tool.name == "kml":
        if case.workflow == "fix":
            return [binary, "check", "--fix", str(corpus), *config_args]
        return [binary, "check", str(corpus), *config_args]
    if case.tool.name == "mado":
        if case.workflow == "fix":
            raise RuntimeError("mado fix workflow is not configured")
        return [binary, *config_args, "check", str(corpus)]
    if case.tool.name == "rumdl":
        if case.workflow == "fix":
            return [binary, "check", "--fix", *config_args, str(corpus)]
        return [binary, "check", *config_args, str(corpus)]
    raise RuntimeError(f"unsupported tool: {case.tool.name}")


def config_args_for_tool(case: Case) -> list[str]:
    if case.config is None:
        return []
    if case.tool.name == "kml":
        return ["--config", str(case.config)]
    if case.tool.name == "mado":
        return ["--config", str(case.config)]
    if case.tool.name == "rumdl":
        return ["--config", str(case.config)]
    return []


def expected_exit_codes(case: Case) -> set[int]:
    if case.workflow == "check" and case.corpus_kind == "clean":
        return {0}
    if case.workflow == "check" and case.corpus_kind == "dirty":
        return {0, 1}
    if case.workflow == "fix":
        return {0, 1}
    return {0}


def isolated_env() -> dict[str, str]:
    env = os.environ.copy()
    isolated_home = pathlib.Path(tempfile.gettempdir()) / "kml-cross-tool-empty-home"
    isolated_home.mkdir(parents=True, exist_ok=True)
    env["HOME"] = str(isolated_home)
    env["XDG_CONFIG_HOME"] = str(isolated_home / ".config")
    return env


def isolated_cwd() -> pathlib.Path:
    path = pathlib.Path(tempfile.gettempdir()) / "kml-cross-tool-empty-cwd"
    path.mkdir(parents=True, exist_ok=True)
    return path


def tool_version(tool: Tool) -> str | None:
    if tool.binary is None:
        return None
    completed = subprocess.run(
        [str(tool.binary), "--version"],
        text=True,
        capture_output=True,
        check=False,
        env=isolated_env(),
        cwd=isolated_cwd(),
    )
    if completed.returncode != 0:
        return "unknown"
    return completed.stdout.strip().splitlines()[0] if completed.stdout.strip() else "unknown"


def measured_case(
    case: Case,
    version: str | None,
    timing_method: str,
    samples: list[float],
    observed_exit_codes: list[int],
    fix_validation: dict[str, Any] | None,
) -> dict[str, Any]:
    return {
        **base_case(case, version),
        "status": "measured",
        "skip_reason": None,
        "failure_reason": None,
        "timing_method": timing_method,
        "runs": len(samples),
        "sample_ms": samples,
        "mean_ms": statistics.fmean(samples),
        "median_ms": statistics.median(samples),
        "min_ms": min(samples),
        "max_ms": max(samples),
        "stddev_ms": statistics.pstdev(samples) if len(samples) > 1 else 0.0,
        "observed_exit_codes": observed_exit_codes,
        "fix_validation": fix_validation,
    }


def skipped_case(case: Case, reason: str) -> dict[str, Any]:
    return {
        **base_case(case, None),
        "status": "skipped",
        "skip_reason": reason,
        "failure_reason": None,
        "timing_method": None,
        "runs": 0,
        "sample_ms": [],
        "mean_ms": None,
        "median_ms": None,
        "min_ms": None,
        "max_ms": None,
        "stddev_ms": None,
        "observed_exit_codes": [],
        "fix_validation": None,
    }


def failed_case(
    case: Case,
    version: str | None,
    completed: subprocess.CompletedProcess[str],
    phase: str,
    fix_validation: dict[str, Any] | None,
) -> dict[str, Any]:
    reason = (
        f"{phase} command exited {completed.returncode}; "
        f"expected {sorted(expected_exit_codes(case))}"
    )
    payload = base_case(case, version)
    payload["detected_limitations"] = [
        *payload["detected_limitations"],
        *output_detected_limitations(completed),
    ]
    return {
        **payload,
        "status": "failed",
        "skip_reason": None,
        "failure_reason": reason,
        "timing_method": None,
        "runs": 0,
        "sample_ms": [],
        "mean_ms": None,
        "median_ms": None,
        "min_ms": None,
        "max_ms": None,
        "stddev_ms": None,
        "observed_exit_codes": [completed.returncode],
        "fix_validation": fix_validation,
    }


def output_detected_limitations(completed: subprocess.CompletedProcess[str]) -> list[str]:
    output = f"{completed.stdout}\n{completed.stderr}".lower()
    config_terms = ("config", "option", "rule", "unknown", "unsupported", "invalid")
    if any(term in output for term in config_terms):
        return ["command output indicates a tool-specific config or rule limitation"]
    return []


def base_case(case: Case, version: str | None) -> dict[str, Any]:
    command = command_display_for_case(case)
    return {
        "tool": case.tool.name,
        "tool_version": version,
        "name": case.name,
        "mode": case.mode,
        "workflow": case.workflow,
        "corpus_kind": case.corpus_kind,
        "corpus": str(case.source_corpus or case.corpus),
        "command": command,
        "expected_exit_codes": sorted(expected_exit_codes(case)),
        "enabled_rules": COMMON_SUBSET_RULES if case.mode == "common" else None,
        "detected_limitations": detected_limitations(case),
    }


def command_display_for_case(case: Case) -> str:
    if case.tool.binary is None:
        return ""
    if case.workflow == "fix" and case.tool.name == "mado":
        return ""
    return shlex.join(command_for_case(case, pathlib.Path("$CORPUS")))


def detected_limitations(case: Case) -> list[str]:
    limitations = []
    if case.mode == "default":
        limitations.append("default mode uses each tool's own enabled rule set")
    if case.mode == "common" and case.tool.name == "rumdl":
        limitations.append("rumdl common mode uses global enable list")
    if case.workflow == "fix" and case.tool.name == "mado":
        limitations.append("mado fix workflow is not configured")
    return limitations


def case_key(case: Case) -> str:
    return f"{case.tool.name}:{case.mode}:{case.name}"


def markdown_summary(report: dict[str, Any]) -> str:
    lines = [
        "# Cross-Tool CLI Benchmark",
        "",
        f"- Schema version: {report['schema_version']}",
        f"- Timing method: {report['timing_method']}",
        f"- Runs: {report['runs']}",
        f"- Warmup: {report['warmup']}",
        "",
        "| Tool | Mode | Case | Status | Median ms | Version | Note |",
        "| --- | --- | --- | --- | ---: | --- | --- |",
    ]
    for case in report["cases"]:
        median = case["median_ms"]
        median_text = f"{median:.3f}" if isinstance(median, (int, float)) else ""
        note = case["skip_reason"] or case["failure_reason"] or ""
        if not note and case.get("fix_validation"):
            note = fix_validation_note(case["fix_validation"])
        lines.append(
            "| {tool} | {mode} | {case} | {status} | {median} | {version} | {note} |".format(
                tool=case["tool"],
                mode=case["mode"],
                case=case["name"],
                status=case["status"],
                median=median_text,
                version=case["tool_version"] or "",
                note=note.replace("|", "/"),
            )
        )
    return "\n".join(lines)


def fix_validation_note(validation: dict[str, Any]) -> str:
    before = validation.get("before_issues")
    after = validation.get("after_issues")
    status = validation["status"]
    if isinstance(before, int) and isinstance(after, int):
        return f"fix validation: {status} ({before}->{after})"
    return f"fix validation: {status}"


def print_summary(
    report: dict[str, Any],
    output_path: pathlib.Path,
    summary_path: pathlib.Path,
) -> None:
    print("Cross-tool benchmark summary:")
    for case in report["cases"]:
        if case["status"] == "measured":
            print(
                "- {tool} {mode} {case}: median={median:.3f}ms".format(
                    tool=case["tool"],
                    mode=case["mode"],
                    case=case["name"],
                    median=case["median_ms"],
                )
            )
        elif case["status"] == "skipped":
            print(
                "- {tool} {mode} {case}: skipped ({reason})".format(
                    tool=case["tool"],
                    mode=case["mode"],
                    case=case["name"],
                    reason=case["skip_reason"],
                )
            )
        else:
            print(
                "- {tool} {mode} {case}: failed ({reason})".format(
                    tool=case["tool"],
                    mode=case["mode"],
                    case=case["name"],
                    reason=case["failure_reason"],
                )
            )
    print(f"JSON report: {output_path}")
    print(f"Markdown summary: {summary_path}")


if __name__ == "__main__":
    raise SystemExit(main())
