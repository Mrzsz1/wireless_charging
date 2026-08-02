#!/usr/bin/env python3
"""Prepare the two core books for chapter-first Markdown ingestion.

The source PDFs in ``raw/inbox/manual-drop`` are never modified.  This tool
creates deterministic, <=180-page staging parts and a manifest containing
both physical PDF pages and the printed page range used by retrieval.
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path
from typing import Any

import fitz  # PyMuPDF

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_INPUT = ROOT / "work" / "core-books" / "inputs"
DEFAULT_OUTPUT = ROOT / "work" / "core-books"
MAX_PAGES = 180


def slug(value: str) -> str:
    value = re.sub(r"[^A-Za-z0-9]+", "_", value).strip("_")
    return value or "section"


def approx_toc(doc: fitz.Document) -> list[dict[str, Any]]:
    """Read chapter rows from the printed table of contents.

    The book has no PDF outline; the chapter rows are present in the text
    layer and are more reliable than guessing from running headers.
    """
    # MuPDF exposes the chapter number and title as separate text spans on
    # this Springer PDF.  Poppler's layout mode preserves the TOC row, so use
    # it when available and retain a MuPDF fallback for portable installs.
    text = "\n".join(doc[i].get_text() for i in range(min(25, len(doc))))
    try:
        with tempfile.NamedTemporaryFile(suffix=".txt", delete=False) as handle:
            tmp = Path(handle.name)
        subprocess.run(["pdftotext", "-layout", "-f", "1", "-l", "25", str(doc.name), str(tmp)], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        text = tmp.read_text(encoding="utf-8", errors="replace")
        tmp.unlink(missing_ok=True)
    except (OSError, subprocess.SubprocessError):
        pass
    pattern = re.compile(r"^\s*(\d{1,2})\s+(.+?)\s+(?:\.\s*){3,}(\d{1,3})\s*$")
    rows: list[dict[str, Any]] = []
    for line in text.splitlines():
        match = pattern.match(line)
        if not match:
            continue
        number, title, printed = int(match.group(1)), match.group(2).strip(), int(match.group(3))
        if 1 <= number <= 30:
            rows.append({"id": f"ch-{number:02d}", "number": number, "title": title, "printed_start": printed})
    if len(rows) != 30:
        raise RuntimeError(f"Approximation Algorithms TOC parse expected 30 chapters, got {len(rows)}")
    return rows


def agt_toc(doc: fitz.Document) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for level, title, page in doc.get_toc():
        if level != 1 or not title.startswith("9780521872829c"):
            continue
        match = re.search(r"c(\d{2})", title)
        if not match:
            continue
        number = int(match.group(1))
        rows.append({"id": f"ch-{number:02d}", "number": number, "title": f"Chapter {number}", "printed_start": int(page)})
    if len(rows) != 29:
        raise RuntimeError(f"Algorithmic Game Theory outline expected 29 chapters, got {len(rows)}")
    # The first outline target is the part divider. The chapter title page is
    # the next two physical pages, while all later outline targets are exact.
    rows[0]["physical_start"] = 24
    for row in rows[1:]:
        row["physical_start"] = row["printed_start"]
    # Some bookmarks target a part divider rather than the chapter title.
    # Locate the actual CHAPTER n page within a small forward window.
    for row in rows:
        marker = re.compile(rf"^CHAPTER\s+{row['number']}\s*$", re.I | re.M)
        for page_no in range(max(1, row["physical_start"] - 1), min(len(doc), row["physical_start"] + 4) + 1):
            if marker.search(doc[page_no - 1].get_text()):
                row["physical_start"] = page_no
                break
    # Cambridge's outline stores chapter filenames, not display titles. Read
    # the title page so retrieval can match real concepts (not just "Chapter
    # 12"). The title may wrap across two lines before the author line.
    for row in rows:
        lines = [x.strip() for x in doc[row["physical_start"] - 1].get_text().splitlines() if x.strip()]
        try:
            pos = next(i for i, line in enumerate(lines) if re.match(r"CHAPTER\s+\d+", line, re.I))
            title_lines = []
            for line in lines[pos + 1 :]:
                if re.search(r"\b(Authors?|Editors?|Abstract)\b", line, re.I):
                    break
                if re.match(r"^[A-Z][A-Z .&'-]{3,}$", line) or len(title_lines) < 2:
                    title_lines.append(line)
                if len(title_lines) >= 3:
                    break
            if title_lines:
                row["title"] = " ".join(title_lines).strip()
        except StopIteration:
            pass
    return rows


def add_ranges(rows: list[dict[str, Any]], page_count: int, front_end: int, printed_offset: int | None = None) -> list[dict[str, Any]]:
    if printed_offset is not None:
        for row in rows:
            row["physical_start"] = row["printed_start"] + printed_offset
    for idx, row in enumerate(rows):
        row["physical_end"] = (rows[idx + 1]["physical_start"] - 1 if idx + 1 < len(rows) else page_count)
        row["printed_end"] = (rows[idx + 1]["printed_start"] - 1 if idx + 1 < len(rows) else row["printed_start"] + row["physical_end"] - row["physical_start"])
    front = {"id": "front-matter", "number": 0, "title": "Front matter", "physical_start": 1, "physical_end": front_end, "printed_start": None, "printed_end": None}
    return [front] + rows


def split_book(pdf: Path, book_id: str, rows: list[dict[str, Any]], out_root: Path, source_pdf: str) -> dict[str, Any]:
    doc = fitz.open(pdf)
    book_dir = out_root / book_id
    parts_dir = book_dir / "parts"
    parts_dir.mkdir(parents=True, exist_ok=True)
    parts: list[dict[str, Any]] = []
    part_no = 0
    for row in rows:
        start, end = int(row["physical_start"]), int(row["physical_end"])
        if start > end:
            continue
        # A chapter is normally below the service limit. If a future edition
        # has a very large chapter, split it without crossing a 180-page part.
        cursor = start
        while cursor <= end:
            stop = min(end, cursor + MAX_PAGES - 1)
            part_no += 1
            name = f"{book_id}-{row['id']}-part-{part_no:03d}.pdf"
            target = parts_dir / name
            if not target.exists() or target.stat().st_size == 0:
                piece = fitz.open()
                piece.insert_pdf(doc, from_page=cursor - 1, to_page=stop - 1)
                piece.save(target, garbage=4, deflate=True)
                piece.close()
            part = {
                "part_id": target.stem,
                "path": str(target.relative_to(ROOT).as_posix()),
                "chapter_id": row["id"],
                "chapter_number": row["number"],
                "chapter_title": row["title"],
                "source_page_start": cursor,
                "source_page_end": stop,
                "printed_page_start": row.get("printed_start"),
                "printed_page_end": row.get("printed_end"),
                "page_count": stop - cursor + 1,
            }
            parts.append(part)
            cursor = stop + 1
    manifest = {
        "schema": "core-book-parts-v1",
        "book_id": book_id,
        "source_pdf": source_pdf,
        "pdf_page_count": len(doc),
        "max_part_pages": MAX_PAGES,
        "parts": parts,
        "chapter_count": len([r for r in rows if r["number"]]),
        "toc_method": "pdf_outline" if book_id == "algorithmic-game-theory" else "text_toc",
    }
    (book_dir / "part-manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    doc.close()
    return manifest


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input-dir", type=Path, default=DEFAULT_INPUT)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    approx = args.input_dir / "Approximation_Algorithms.pdf"
    agt = args.input_dir / "Algorithmic_Game_Theory.pdf"
    if not approx.is_file() or not agt.is_file():
        raise SystemExit("Expected Approximation_Algorithms.pdf and Algorithmic_Game_Theory.pdf in input directory")
    approx_doc = fitz.open(approx)
    approx_rows = approx_toc(approx_doc)
    approx_rows = add_ranges(approx_rows, len(approx_doc), front_end=18, printed_offset=18)
    approx_doc.close()
    agt_doc = fitz.open(agt)
    agt_rows = agt_toc(agt_doc)
    agt_rows = add_ranges(agt_rows, len(agt_doc), front_end=23)
    agt_doc.close()
    manifests = [
        split_book(approx, "approximation-algorithms", approx_rows, args.output_dir, "raw/inbox/manual-drop/PDF_A.pdf"),
        split_book(agt, "algorithmic-game-theory", agt_rows, args.output_dir, "raw/inbox/manual-drop/PDF_B.pdf"),
    ]
    print(json.dumps({"books": [{"book_id": m["book_id"], "pages": m["pdf_page_count"], "parts": len(m["parts"]), "chapters": m["chapter_count"]} for m in manifests]}, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
