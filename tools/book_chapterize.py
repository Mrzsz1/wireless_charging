#!/usr/bin/env python3
"""Merge MinerU chapter parts into stable core-book Markdown.

Each generated chapter keeps source PDF page anchors and the original MinerU
part directory as provenance.  The generated files are derived artifacts; the
uploaded PDFs and per-part MinerU results remain untouched.
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]


def strip_frontmatter(text: str) -> str:
    if text.startswith("---"):
        end = text.find("\n---", 3)
        if end >= 0:
            return text[end + 4 :].lstrip()
    return text


def poppler_text(pdf: Path, start: int, end: int) -> str:
    with tempfile.NamedTemporaryFile(suffix=".txt", delete=False) as handle:
        target = Path(handle.name)
    subprocess.run(["pdftotext", "-layout", "-f", str(start), "-l", str(end), str(pdf), str(target)], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    text = target.read_text(encoding="utf-8", errors="replace")
    target.unlink(missing_ok=True)
    return text


def build(book_dir: Path, canonical_dir: Path, inputs_dir: Path) -> dict[str, Any]:
    manifest = json.loads((book_dir / "part-manifest.json").read_text(encoding="utf-8"))
    out_dir = canonical_dir / manifest["book_id"]
    chapters_dir = out_dir / "chapters"
    mineru_dir = out_dir / "mineru"
    chapters_dir.mkdir(parents=True, exist_ok=True)
    mineru_dir.mkdir(parents=True, exist_ok=True)
    pdf = inputs_dir / ("Approximation_Algorithms.pdf" if manifest["book_id"] == "approximation-algorithms" else "Algorithmic_Game_Theory.pdf")
    grouped: dict[str, list[dict[str, Any]]] = {}
    for part in manifest["parts"]:
        grouped.setdefault(part["chapter_id"], []).append(part)
    chapter_rows = []
    for chapter_id, parts in grouped.items():
        parts.sort(key=lambda p: p["source_page_start"])
        first = parts[0]
        body: list[str] = []
        for part in parts:
            part_stem = Path(part["path"]).stem
            source_dir = canonical_dir / part_stem
            md = source_dir / "full.md"
            if not md.is_file():
                raise FileNotFoundError(md)
            body.append(f"\n<!-- source-pages: {part['source_page_start']}-{part['source_page_end']}; printed-pages: {part.get('printed_page_start')}-{part.get('printed_page_end')}; mineru-part: {part_stem} -->\n")
            body.append(strip_frontmatter(md.read_text(encoding="utf-8", errors="replace")))
        title = first["chapter_title"]
        filename = f"{chapter_id}-{re.sub(r'[^a-z0-9]+', '-', title.lower()).strip('-') or 'section'}.md"
        target = chapters_dir / filename
        mineru_target = mineru_dir / filename
        frontmatter = {
            "type": "book-chapter",
            "book_id": manifest["book_id"],
            "chapter_id": chapter_id,
            "chapter_number": first["chapter_number"],
            "chapter_title": title,
            "source_pdf": manifest["source_pdf"],
            "source_page_start": first["source_page_start"],
            "source_page_end": parts[-1]["source_page_end"],
            "printed_page_start": first.get("printed_page_start"),
            "printed_page_end": parts[-1].get("printed_page_end"),
            "part_ids": [p["part_id"] for p in parts],
            "ingest_engine": "mineru-precise-v4",
            "ingest_status": "pending_quality",
        }
        yaml = "---\n" + "\n".join(f"{k}: {json.dumps(v, ensure_ascii=False) if isinstance(v, (str, list, dict)) else ('null' if v is None else v)}" for k, v in frontmatter.items()) + "\n---\n\n"
        # Poppler's layout extraction is page-faithful for these born-digital
        # books and is the retrieval source. MinerU remains beside it for
        # figures, tables, and semantic layout review.
        page_text = poppler_text(pdf, first["source_page_start"], parts[-1]["source_page_end"])
        target.write_text(yaml + f"# {title}\n\n" + page_text.strip() + "\n", encoding="utf-8")
        mineru_target.write_text(yaml + f"# {title} (MinerU semantic layer)\n\n" + "\n".join(body).strip() + "\n", encoding="utf-8")
        chapter_rows.append({**frontmatter, "path": str(target.relative_to(ROOT).as_posix()), "mineru_path": str(mineru_target.relative_to(ROOT).as_posix()), "char_count": target.stat().st_size})
    index = {"schema": "core-book-chapters-v1", "book_id": manifest["book_id"], "chapters": chapter_rows}
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "chapter-index.json").write_text(json.dumps(index, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return index


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work-dir", type=Path, default=ROOT / "work" / "core-books")
    parser.add_argument("--canonical-dir", type=Path, default=ROOT / "raw" / "canonical")
    parser.add_argument("--inputs-dir", type=Path, default=ROOT / "work" / "core-books" / "inputs")
    args = parser.parse_args()
    results = []
    for book in ("approximation-algorithms", "algorithmic-game-theory"):
        results.append(build(args.work_dir / book, args.canonical_dir, args.inputs_dir))
    print(json.dumps({"books": [{"book_id": x["book_id"], "chapters": len(x["chapters"])} for x in results]}, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
