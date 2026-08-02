#!/usr/bin/env python3
"""Run the governed local knowledge pipeline as one supervised process."""
from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def command(name: str) -> str:
    return shutil.which(name) or name


def wait_for_resume(name: str, control_file: Path | None) -> None:
    if control_file is None or not control_file.exists():
        return
    print(f"PIPELINE_PAUSED {name}", flush=True)
    while control_file.exists():
        time.sleep(0.25)
    print(f"PIPELINE_RESUMED {name}", flush=True)


def run_stage(
    name: str,
    command: list[str],
    cwd: Path = ROOT,
    control_file: Path | None = None,
) -> None:
    wait_for_resume(name, control_file)
    print(f"PIPELINE_STAGE_START {name}", flush=True)
    result = subprocess.run(command, cwd=cwd, check=False)
    if result.returncode:
        print(f"PIPELINE_STAGE_FAILED {name} {result.returncode}", flush=True)
        raise SystemExit(result.returncode)
    print(f"PIPELINE_STAGE_COMPLETED {name}", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input-path")
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--control-file")
    args = parser.parse_args()
    control_file = (ROOT / args.control_file).resolve() if args.control_file else None

    discovery = [sys.executable, "tools/paper_search.py", "--preset", "wireless-charging-scheduling", "--new-only"]
    if args.download:
        discovery.append("--download")
    run_stage("discover", discovery, control_file=control_file)

    if args.input_path:
        source = (ROOT / args.input_path).resolve()
        if not source.exists() or ROOT.resolve() not in source.parents and source != ROOT.resolve():
            raise SystemExit("pipeline input must exist inside the repository")
        parse = [sys.executable, "tools/mineru_to_md.py", str(source), "--output-root", str(ROOT / "raw/canonical")]
        if args.force:
            parse.append("--force")
        run_stage("parse", parse, control_file=control_file)

    run_stage("compile_a", [
        command("codex"), "-a", "never", "-s", "workspace-write", "exec", "-C", str(ROOT),
        "--skip-git-repo-check", "--ephemeral",
        "Read AGENTS.md and schema/agent-a-compile.md. Compile every pending_ingest through the Agent A protocol. Never write wiki/problems or wiki/ideas, never edit vocab.yaml, never delete files, and update index, library-status and logs.",
    ], control_file=control_file)
    run_stage("lint", [sys.executable, "tools/wiki_lint.py", "--write-report"], control_file=control_file)
    run_stage("graphify_update", [command("graphify"), "update", ".", "--force"], control_file=control_file)
    run_stage("rebuild_snapshot", [sys.executable, "tools/export_desktop_data.py"], control_file=control_file)
    run_stage("verify", [command("npm"), "run", "verify:p5"], ROOT / "apps/desktop", control_file=control_file)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
