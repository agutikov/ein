"""Shared pytest fixtures."""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot import State, load_into

DATA_DIR = Path(__file__).parent / "data"


@pytest.fixture
def conditions_path() -> Path:
    return DATA_DIR / "conditions.txt"


@pytest.fixture
def conditions_lines(conditions_path: Path) -> list[str]:
    return conditions_path.read_text(encoding="utf-8").splitlines()


@pytest.fixture
def tiny_lines() -> list[str]:
    """A minimal three-statement input exercising every parse branch."""
    return [
        "",                       # blank skipped
        "Alice",                  # object decl
        "Bob is Person",          # 3-token relation
        "Carol moves toward Dan", # multi-token relation
    ]


@pytest.fixture
def tiny_state(tiny_lines: list[str]) -> State:
    s = State()
    load_into(s, tiny_lines)
    return s
