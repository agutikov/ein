"""Shared fixtures for KB tests."""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.ir import parse
from ein.kb import KnowledgeBase

REPO_ROOT = Path(__file__).resolve().parents[3]
EXAMPLES = REPO_ROOT / "examples"


@pytest.fixture(scope="session")
def zebra_kb() -> KnowledgeBase:
    """Load examples/zebra.ein into a KnowledgeBase."""
    text = (EXAMPLES / "zebra.ein").read_text()
    return KnowledgeBase.from_ir(parse(text))


@pytest.fixture(scope="session")
def zebra2_kb() -> KnowledgeBase:
    """Load examples/zebra2.ein (the unified `is-a` model)."""
    text = (EXAMPLES / "zebra2.ein").read_text()
    return KnowledgeBase.from_ir(parse(text))
