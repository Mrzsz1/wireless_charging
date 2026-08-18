import json
import os
import subprocess
import sys
import tempfile
import threading
import time
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PIPELINE = ROOT / "tools" / "full_pipeline.py"
EXPECTED_STAGES = [
    "discover",
    "compile_a",
    "lint",
    "graphify_update",
    "verify_graph",
    "rebuild_snapshot",
    "verify",
]


class FullPipelineFixtureTests(unittest.TestCase):
    def run_fixture(self, fixture: Path, *extra: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(PIPELINE), "--fixture-dir", str(fixture), *extra],
            cwd=ROOT,
            text=True,
            capture_output=True,
            check=False,
        )

    def stages(self, fixture: Path) -> list[str]:
        return [
            json.loads(line)["stage"]
            for line in (fixture / "stages.jsonl").read_text(encoding="utf-8").splitlines()
        ]

    def test_success_runs_every_governed_stage_in_order(self) -> None:
        with tempfile.TemporaryDirectory(dir=ROOT) as directory:
            fixture = Path(directory)
            result = self.run_fixture(fixture)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertEqual(self.stages(fixture), EXPECTED_STAGES)
            for stage in EXPECTED_STAGES:
                self.assertTrue((fixture / f"{stage}.done").exists())

    def test_failure_stops_following_stages_and_preserves_evidence(self) -> None:
        with tempfile.TemporaryDirectory(dir=ROOT) as directory:
            fixture = Path(directory)
            (fixture / "config.json").write_text(
                json.dumps({"fail_stage": "lint", "exit_code": 23}), encoding="utf-8"
            )
            result = self.run_fixture(fixture)
            self.assertEqual(result.returncode, 23)
            self.assertEqual(self.stages(fixture), ["discover", "compile_a", "lint"])
            self.assertIn("PIPELINE_STAGE_FAILED lint 23", result.stdout)
            self.assertFalse((fixture / "graphify_update.done").exists())

    def test_compile_stage_prefers_resolved_codex_path_from_parent(self) -> None:
        with tempfile.TemporaryDirectory(dir=ROOT) as directory:
            fixture = Path(directory)
            environment = os.environ.copy()
            environment["CODEX_CLI_PATH"] = str(Path(directory) / "Codex Desktop" / "codex.exe")
            result = subprocess.run(
                [sys.executable, str(PIPELINE), "--fixture-dir", str(fixture)],
                cwd=ROOT,
                text=True,
                capture_output=True,
                check=False,
                env=environment,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            commands = [json.loads(line) for line in (fixture / "stages.jsonl").read_text(encoding="utf-8").splitlines()]
            compile_command = next(item["command"] for item in commands if item["stage"] == "compile_a")
            self.assertEqual(compile_command[0], environment["CODEX_CLI_PATH"])

    def test_pause_file_blocks_the_next_stage_until_removed(self) -> None:
        with tempfile.TemporaryDirectory(dir=ROOT) as directory:
            fixture = Path(directory)
            control = fixture / "pause.control"
            control.write_text("pause\n", encoding="utf-8")
            result_holder: list[subprocess.CompletedProcess[str]] = []

            thread = threading.Thread(
                target=lambda: result_holder.append(
                    self.run_fixture(fixture, "--control-file", str(control.relative_to(ROOT)))
                )
            )
            thread.start()
            time.sleep(0.6)
            self.assertTrue(thread.is_alive())
            self.assertFalse((fixture / "stages.jsonl").exists())
            control.unlink()
            thread.join(timeout=10)
            self.assertFalse(thread.is_alive())
            self.assertEqual(result_holder[0].returncode, 0)
            self.assertIn("PIPELINE_PAUSED discover", result_holder[0].stdout)
            self.assertIn("PIPELINE_RESUMED discover", result_holder[0].stdout)


if __name__ == "__main__":
    unittest.main()
