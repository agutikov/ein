"""Shared relation-colour palette for the DOT renderers — S1.6.0.

A single deterministic ``relation name → colour`` map, so the same
relation is drawn in the same colour across *every* view (the per-form
IR renderer, the unified KB renderer, the derivation slices). The eye
then groups by relation regardless of which diagram it is looking at.

Extracted from :mod:`ein_bot.kb.render` (S1.2.4) when S1.6.0 made the
per-form IR renderer share the same compact, colour-by-relation look.
"""
from __future__ import annotations

import hashlib

# Distinct, mid-saturation colours readable on both light and dark
# backgrounds. From d3.schemeCategory10 with a couple of swaps for
# legibility on print.
PALETTE: tuple[str, ...] = (
    "#1f77b4",  # blue
    "#ff7f0e",  # orange
    "#2ca02c",  # green
    "#d62728",  # red
    "#9467bd",  # purple
    "#8c564b",  # brown
    "#e377c2",  # pink
    "#7f7f7f",  # gray
    "#bcbd22",  # olive
    "#17becf",  # cyan
)


def hash_color(name: str) -> str:
    """Stable colour per relation name — deterministic across runs."""
    h = hashlib.sha1(name.encode("utf-8")).hexdigest()
    return PALETTE[int(h, 16) % len(PALETTE)]


__all__ = ["PALETTE", "hash_color"]
