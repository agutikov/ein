#!/usr/bin/env python3
"""Rasterise every ```dot block of a markdown file — M1e2 S1e2.1.5 T2.

Lifted out of `utils/zebra2_trace.sh`, which had it inline for one hard-coded
puzzle; that script now calls this, and the review bundle's trace pages call
`rasterise_text`, so there is one copy (M1e `AR-M1`'s *one artefact, one
owner*, applied to a script).

A block Graphviz refuses is **kept**, as the refusal text and a link to its
DOT, and counted — never dropped, because a DOT the renderer cannot draw is a
`dot_wellformed.rs` gap and the reader should see it where it happened.

    python3 utils/review/rasterise.py TRACE.md OUT_DIR [--name VIEW.md]

writes OUT_DIR/img/blockNNNN.svg (+ .dot) and OUT_DIR/VIEW.md (default:
the input's stem + `.view.md`), and prints how many blocks it drew.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from review.util import SvgJob, image, render_svgs  # noqa: E402

FENCE = re.compile(r"```dot\n(.*?)\n```", re.DOTALL)


def rasterise_text(md: str, img_dir: Path, page: Path, start: int = 0,
                   prefix: str = "block") -> tuple[str, int, list[str]]:
    """`md` with every dot fence replaced by its SVG reference, as seen from
    `page`; the number of blocks; and one message per refusal."""
    blocks = [m.group(1) for m in FENCE.finditer(md)]
    jobs = [SvgJob(b, img_dir / f"{prefix}{start + i + 1:04d}.svg") for i, b in enumerate(blocks)]
    results = render_svgs(jobs)
    refused = [f"{r.svg.with_suffix('.dot').name}: {r.error}" for r in results if not r.ok]
    it = iter(results)

    def sub(_m: re.Match) -> str:
        r = next(it)
        return "\n".join(image(r, page, r.svg.stem))

    return FENCE.sub(sub, md), len(blocks), refused


def main(argv: list[str]) -> int:
    if len(argv) < 2 or argv[0] in ("-h", "--help"):
        print(__doc__)
        return 0 if argv and argv[0] in ("-h", "--help") else 2
    md_path, out_dir = Path(argv[0]), Path(argv[1])
    name = md_path.stem + ".view.md"
    if "--name" in argv:
        name = argv[argv.index("--name") + 1]
    view = out_dir / name
    text, n, refused = rasterise_text(md_path.read_text(encoding="utf-8"), out_dir / "img", view)
    view.parent.mkdir(parents=True, exist_ok=True)
    view.write_text(text, encoding="utf-8")
    for r in refused:
        print(f"warn: dot refused {r}", file=sys.stderr)
    print(f"rendered {n - len(refused)} of {n} dot blocks -> {out_dir / 'img'}/ ; viewable: {view}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
