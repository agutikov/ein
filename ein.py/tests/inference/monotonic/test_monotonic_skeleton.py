"""S1.5b.4 skeleton — imports + arg-parsing smoke.

The real test surface lands in S1.5b.5 (backbone) + S1.5b.9
(parity). This file is the import-only contract: the public
surface exists, the stub raises ``NotImplementedError``, and
two backbone placeholders skip with documented reasons.
"""
from __future__ import annotations

import pytest

from ein_bot.inference.monotonic import monotonic_solve
from ein_bot.inference.monotonic.solver import monotonic_solve as _direct
from ein_bot.inference.monotonic.state_dump import MonotonicDumper


def test_imports_resolve():
    assert monotonic_solve is _direct
    assert MonotonicDumper is not None


def test_solver_raises_not_implemented():
    """Until S1.5b.5 lands."""
    from ein_bot.kb.store import KnowledgeBase
    kb = KnowledgeBase()
    with pytest.raises(NotImplementedError):
        monotonic_solve(kb, max_set_size=1)


@pytest.mark.skip(reason="backbone — implemented in S1.5b.5")
def test_zebra_solves(): ...


@pytest.mark.skip(reason="backbone — implemented in S1.5b.5")
def test_branching_demos_parity(): ...
