"""Colour helpers used by ``State.dot()``.

The Graphviz output of :class:`ein_bot.state.State` colours each relation
with a deterministic colour derived from its name (so re-renders stay
visually stable) and exposes a separate "throwaway" random colour helper
that was used during PoC experimentation.
"""
from __future__ import annotations

import zlib

import numpy as np


def random_dot_color() -> str:
    """Return a random ``#RRGGBB`` colour (PoC-era helper, kept for parity)."""
    rgb = np.random.choice(range(256), size=3)
    return "#" + "".join(f"{int(c):02X}" for c in rgb)


def hash_color(s: str) -> str:
    """Return a deterministic ``#RRGGBB`` colour from a CRC32 of *s*."""
    crc = zlib.crc32(s.encode("utf-8"))
    return f"#{crc & 0xFFFFFF:06X}"
