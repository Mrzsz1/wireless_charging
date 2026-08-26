from __future__ import annotations

import copy
import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "qa_eval_metadata", ROOT / "tools" / "qa_eval_metadata.py"
)
assert SPEC and SPEC.loader
qa_eval_metadata = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = qa_eval_metadata
SPEC.loader.exec_module(qa_eval_metadata)


class QaEvalMetadataTests(unittest.TestCase):
    def providers(self) -> dict[str, dict[str, str]]:
        return {
            "answer": {"provider": "codex", "model": "gpt-fixture"},
            "embedding": {"provider": "fastembed", "model": "embedding-fixture"},
            "reranker": {"provider": "fastembed", "model": "reranker-fixture"},
            "verification": {"provider": "compatible_api", "model": "nli-fixture"},
        }

    def envelope(self) -> dict[str, object]:
        return qa_eval_metadata.build_metadata_envelope(
            dataset_version="fixture-v1",
            dataset_payload={"b": 2, "a": [1, "二"]},
            runtime_config={"timeoutSeconds": 30, "maxCandidates": 80},
            providers=self.providers(),
            git_commit="a" * 40,
            generated_at_utc="2026-08-26T12:00:00Z",
            platform_info={
                "system": "FixtureOS",
                "release": "1",
                "machine": "x86_64",
                "python": "3.12.0",
            },
            hardware_info={"cpu": "Fixture CPU", "logicalCpuCount": 8, "memoryBytes": 16},
        )

    def test_canonical_hash_ignores_object_key_order_but_not_values(self) -> None:
        first = {"a": 1, "b": {"x": "无线", "y": [2, 3]}}
        reordered = {"b": {"y": [2, 3], "x": "无线"}, "a": 1}
        changed = {"a": 1, "b": {"x": "无线", "y": [3, 2]}}
        self.assertEqual(
            qa_eval_metadata.canonical_json_sha256(first),
            qa_eval_metadata.canonical_json_sha256(reordered),
        )
        self.assertNotEqual(
            qa_eval_metadata.canonical_json_sha256(first),
            qa_eval_metadata.canonical_json_sha256(changed),
        )

    def test_canonical_file_hash_uses_json_content_not_formatting(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "dataset.json"
            path.write_text('{\n  "b": 2, "a": 1\n}\n', encoding="utf-8")
            self.assertEqual(
                qa_eval_metadata.canonical_json_file_sha256(path),
                qa_eval_metadata.canonical_json_sha256({"a": 1, "b": 2}),
            )

    def test_envelope_contains_only_hashes_for_dataset_and_runtime_config(self) -> None:
        envelope = self.envelope()
        qa_eval_metadata.validate_metadata_envelope(envelope)
        self.assertEqual(envelope["schemaVersion"], "qa-eval-metadata-v1")
        self.assertEqual(len(envelope["dataset"]["sha256"]), 64)
        self.assertEqual(len(envelope["runtimeConfigSha256"]), 64)
        serialized = qa_eval_metadata.canonical_json_bytes(envelope).decode("utf-8")
        self.assertNotIn("timeoutSeconds", serialized)
        self.assertNotIn("maxCandidates", serialized)
        self.assertNotIn('"a":[1,"二"]', serialized)

    def test_validator_rejects_secret_raw_content_and_absolute_paths(self) -> None:
        for field, value in [
            ("apiKey", "secret-value"),
            ("question", "raw research question"),
            ("path", r"E:\\models\\reranker"),
        ]:
            envelope = copy.deepcopy(self.envelope())
            envelope["platform"][field] = value
            with self.assertRaises(qa_eval_metadata.MetadataValidationError, msg=field):
                qa_eval_metadata.validate_metadata_envelope(envelope)

    def test_validator_fails_closed_on_incomplete_provider_matrix(self) -> None:
        envelope = self.envelope()
        del envelope["providers"]["verification"]
        with self.assertRaises(qa_eval_metadata.MetadataValidationError):
            qa_eval_metadata.validate_metadata_envelope(envelope)


if __name__ == "__main__":
    unittest.main()
