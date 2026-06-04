"""`std.closure` auto-closure — `functional ∧ total ⇒ (closed R)` (P1.8 S1.8.A6).

`infer-closure` is a parameter-less stdlib rule (opt-in by import; there is no
config flag) that derives `(closed R)` for any single-arg functional+total
relation. `(closed R)` makes the hypothesis generator skip R (`hypgen._is_closed`).

Soundness: this is an *operational* witness, safe only when R's extension is
determined by saturation — NOT for relations needing hypgen branching (the
zebra family). These tests pin the rule's firing logic + the hypgen hand-off,
not full-solve correctness on a branching puzzle.
"""
from __future__ import annotations

import pytest

from ein_bot.inference.hypgen import _is_closed
from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

_IMPORT = "(import std.closure :symbols (infer-closure))\n"


def _closed(props: str, *, imported: bool = True) -> bool:
    src = (_IMPORT if imported else "") + "(relation r A B)\n" + props
    kb = KnowledgeBase.from_ir(parse(src))
    list(Saturator(kb).saturate(max_steps=1000))
    return _is_closed(kb, "r")


def test_functional_and_total_closes():
    assert _closed("(functional r) (total r)") is True


def test_functional_only_does_not_close():
    """Totality is required — `functional ⇒ closed` is too weak."""
    assert _closed("(functional r)") is False


def test_total_only_does_not_close():
    assert _closed("(total r)") is False


def test_not_imported_does_not_close():
    """The import IS the opt-in: without it the rule never fires."""
    assert _closed("(functional r) (total r)", imported=False) is False


def test_infer_closure_is_parameterless():
    """The rule loads as a non-generic (empty-params) rule — a `(?R)` param
    list would make it wait for activators and never fire."""
    kb = KnowledgeBase.from_ir(parse(_IMPORT))
    assert "infer-closure" in kb.rules
    assert kb.rules["infer-closure"].params == ()


def test_closed_relation_skipped_by_hypgen():
    """End-to-end hand-off: once `infer-closure` derives `(closed r)`, hypgen's
    pre-candidate skip fires for r (no hypotheses generated for it)."""
    from ein_bot.inference import hypgen

    kb = KnowledgeBase.from_ir(parse(
        _IMPORT
        + "(relation r Thing Thing)\n"
        + "(functional r) (total r)\n"
        + "(is-a a Thing :source \"(1)\") (is-a b Thing :source \"(2)\")\n"
    ))
    list(Saturator(kb).saturate(max_steps=1000))
    assert _is_closed(kb, "r")
    # r yields no candidate hypotheses because it is closed.
    cands = [h for h in hypgen.generate_hypotheses(kb) if h.relation_name == "r"]
    assert cands == []


@pytest.mark.parametrize("bad", [
    "(import std.closure :symbols (infer-closure))\n"
    "(rule infer-closure () :match (foo ?x) :assert (bar ?x) :why \"w\")\n",
])
def test_local_redefinition_conflicts(bad):
    """Importing `infer-closure` and also defining it locally is a dup-name
    load error (the strict conflict policy, A1 D3)."""
    from ein_bot.kb.from_ir import KBLoadError
    with pytest.raises(KBLoadError, match=r"duplicate rule"):
        KnowledgeBase.from_ir(parse(bad))
