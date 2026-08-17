"""M1a hazard H2 — mixed-type hypothesis args crash `apriori.layer_1`.

`layer_1` opens the search with `sorted(alive)` over `(relation_name, args)`
tuples. Two candidates of one relation whose slot *i* is `str` in one and `int`
in the other are incomparable, and CPython raises.

Recorded, not repaired — the reasoning is in
[`examples/ein-bugs/mixed-type-hypothesis.ein`](../../../examples/ein-bugs/mixed-type-hypothesis.ein)
and Q-M1a.4. What these tests pin is the *scope* of the hazard, because scope
is what decides whether it is worth a re-baselining fix:

- blind hypgen **cannot** reach it (its candidates are built from `kb.names`,
  which only admits `str` args), so no existing puzzle is affected;
- an `hrule` **can**, because its `:assert` args come from bindings.

If a future change makes the blind enumerator produce a non-string arg, the
second test fails and the trade-off in Q-M1a.4 has to be re-decided.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.inference.hypgen import generate_hypotheses
from ein.inference.monotonic import solve
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
FIXTURE = REPO / "examples" / "ein-bugs" / "mixed-type-hypothesis.ein"


def test_the_fixture_loads_and_saturates():
    """The input is well-formed — this is a search-layer crash, not a broken
    file. (It is why the fixture lives in `ein-bugs/`, not in `broken/`.)"""
    kb = KnowledgeBase.from_file(str(FIXTURE))
    assert "guess" in kb.hrules
    assert kb._fact_by_id("slotval", ("x", 1)) is not None
    assert kb._fact_by_id("slotval", ("y", "left")) is not None


def test_solving_it_raises_typeerror_in_layer_1():
    """The hazard itself. ein.rs will answer this input instead of raising —
    an accepted divergence, with this test as ein.py's half of the pin.

    The *type* is pinned and the message is not, deliberately: which operand
    lands on the left of the failing `<` depends on the `frozenset` iteration
    order inside `sorted`, so `'str' and 'int'` and `'int' and 'str'` both
    occur across `PYTHONHASHSEED` values. That is why the corpus compares a
    crash by exception class and not by its first stderr line (Q-M1a.14).
    """
    kb = KnowledgeBase.from_file(str(FIXTURE))
    with pytest.raises(TypeError, match=r"'<' not supported between instances of"):
        solve(kb, stop_after=None)


def test_blind_hypgen_cannot_produce_a_non_string_arg():
    """The scope claim: only hrule-generated candidates can carry a non-string
    arg, so no puzzle without an hrule can hit this."""
    kb = KnowledgeBase.from_file(str(FIXTURE))
    kb.hrules.clear()                      # force the blind enumerator
    for fact in generate_hypotheses(kb):
        for arg in fact.args:
            assert isinstance(arg, str), (fact.relation_name, fact.args)
