#!/usr/bin/env python3
"""Refresh the structural graph and restore reviewed host-agent semantics."""
from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "graphify-out"
SEMANTIC = OUT / ".graphify_semantic_new.json"
GRAPH = OUT / "graph.json"


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=ROOT, check=False)
    if result.returncode:
        raise SystemExit(result.returncode)


def validate_semantics() -> tuple[int, int]:
    if not SEMANTIC.exists():
        return 0, 0
    payload = json.loads(SEMANTIC.read_text(encoding="utf-8"))
    nodes = payload.get("nodes", [])
    edges = payload.get("edges", [])
    if not isinstance(nodes, list) or not isinstance(edges, list):
        raise SystemExit("invalid graphify semantic extraction: nodes/edges must be arrays")
    for node in nodes:
        source = node.get("source_file", "")
        if not node.get("id") or not source or not (ROOT / source).is_file():
            raise SystemExit(f"invalid graphify semantic node: {node.get('id', '<missing>')}")
    return len(nodes), len(edges)


def merge_semantics() -> None:
    interpreter_file = OUT / ".graphify_python"
    interpreter = interpreter_file.read_text(encoding="utf-8").strip() if interpreter_file.exists() else sys.executable
    code = r'''
import json, sys
from pathlib import Path
from graphify.build import build_from_json
from graphify.cluster import cluster
from graphify.export import to_json

graph_path, semantic_path = map(Path, sys.argv[1:3])
base = json.loads(graph_path.read_text(encoding="utf-8-sig"))
semantic = json.loads(semantic_path.read_text(encoding="utf-8"))
nodes = {node["id"]: node for node in base.get("nodes", [])}
nodes.update({node["id"]: node for node in semantic.get("nodes", [])})
links = base.get("links", []) + semantic.get("edges", [])
seen = set()
deduplicated = []
for edge in links:
    key = (edge.get("source"), edge.get("target"), edge.get("relation"), edge.get("source_file"))
    if key not in seen:
        seen.add(key)
        deduplicated.append(edge)
graph = build_from_json({"nodes": list(nodes.values()), "edges": deduplicated, "hyperedges": semantic.get("hyperedges", [])}, root=".", directed=base.get("directed", False))
communities = cluster(graph)
if not to_json(graph, communities, graph_path):
    raise SystemExit("graphify refused semantic merge")
print(f"restored reviewed semantics: {len(semantic.get('nodes', []))} nodes / {len(semantic.get('edges', []))} edges")
'''
    run([interpreter, "-c", code, str(GRAPH), str(SEMANTIC)])


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-update", action="store_true", help="Only validate and merge the reviewed semantic fragment.")
    args = parser.parse_args()
    nodes, edges = validate_semantics()
    if not args.skip_update:
        run([shutil.which("graphify") or "graphify", "update", ".", "--force"])
    if nodes:
        merge_semantics()
        run([shutil.which("graphify") or "graphify", "cluster-only", ".", "--no-label"])
    print(f"Graphify refresh complete: {nodes} reviewed semantic nodes / {edges} edges")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
