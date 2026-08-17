"""S1.22.4 T1.22.4.2-.5 — relation reflection, bare declarations, unary
hypotheses and unary rendering.

The origin (user, 2026-08-17) is a single complaint: *"other rules could
check if argument is name of relation by `:match (relation ?R)`"* — which
did not work, because matching is **arity-coupled** and the loader stored
only the arity-N signature mirror. These tests pin the four changes that
answer it, and the invariants each one must not break.
"""
from __future__ import annotations

import pytest

from ein.inference.hypgen import generate_hypotheses
from ein.inference.saturator import Saturator
from ein.ir import parse, to_dot
from ein.kb import KnowledgeBase
from ein.kb.from_ir import KBLoadError


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _saturate(kb: KnowledgeBase) -> None:
    list(Saturator(kb).saturate())


def _heads(kb: KnowledgeBase, head: str) -> set:
    return {f.args for f in kb._facts_by_relation.get(head, ())}


# ── T1.22.4.2 — the arity-1 membership fact ────────────────────────

DECLS = """
(relation adult Person)
(relation likes Person Drink)
(relation between Person Person Person)
"""


def test_each_declaration_emits_a_membership_fact():
    """`(relation R)` is stored alongside the arity-N signature mirror."""
    kb = _kb(DECLS)
    stored = {f.args for f in kb._facts_by_relation["relation"]}
    assert ("adult",) in stored
    assert ("likes",) in stored
    assert ("between",) in stored
    # …and the signature mirrors are untouched.
    assert ("adult", "Person") in stored
    assert ("likes", "Person", "Drink") in stored
    assert ("between", "Person", "Person", "Person") in stored


def test_membership_pattern_is_arity_independent():
    """`:match (relation ?R)` sees every declaration regardless of arity.

    This is the origin ask. Before T1.22.4.2 it matched *nothing*, while
    `(relation ?R ?A ?B)` silently saw only the binary declarations — so
    `std.bijection` / `std.algebra` / `std.typing` ignored the others.
    """
    kb = _kb(DECLS + """
    (rule sees-any    () :match (relation ?R)       :assert (is-rel ?R))
    (rule sees-unary  () :match (relation ?R ?A)    :assert (is-unary ?R))
    (rule sees-binary () :match (relation ?R ?A ?B) :assert (is-binary ?R))
    """)
    _saturate(kb)
    assert _heads(kb, "is-rel") == {("adult",), ("likes",), ("between",)}
    # The arity-coupled patterns still behave exactly as before — the new
    # fact adds a channel, it does not change the old one.
    assert _heads(kb, "is-unary") == {("adult",)}
    assert _heads(kb, "is-binary") == {("likes",)}


def test_membership_facts_cover_declared_relations_only():
    """Auto-vivified relations (property-tag carriers) get no declaration,
    hence no membership fact — `(relation ?R)` means *declared* relation."""
    kb = _kb("""
    (relation likes Person Drink)
    (symmetric likes)
    """)
    assert "symmetric" in kb.relations          # auto-vivified from the tag
    membership = {
        f.args[0] for f in kb._facts_by_relation["relation"] if len(f.args) == 1
    }
    assert membership == {"likes"}


# ── T1.22.4.4 — bare `(relation R)` declarations ───────────────────


def test_bare_declaration_loads_with_an_empty_signature():
    kb = _kb("(relation opaque)")
    rel = kb.relations["opaque"]
    assert rel.signature == ()
    assert rel.declared is True


def test_bare_declaration_emits_exactly_one_fact():
    """For an empty signature the mirror *is* the membership fact, so the
    loader must not store it twice under a different shape."""
    kb = _kb("(relation opaque)")
    assert {f.args for f in kb._facts_by_relation["relation"]} == {("opaque",)}


def test_bare_declaration_is_not_a_hypothesis_target():
    """Signature *presence* is the kernel's "declared domain relation"
    signal; an empty signature deliberately fails it."""
    kb = _kb("""
    (relation opaque)
    (relation likes Person Drink)
    (is-a Jack Person) (is-a Jill Person)
    """)
    guessed = {f.relation_name for f in generate_hypotheses(kb)}
    assert "opaque" not in guessed
    assert "likes" in guessed


def test_wrapped_arg_form_is_still_rejected():
    """R10 regression guard. `(relation R (T1 T2))` parses as a generic
    fact; with an empty signature now legal, only an explicit non-Atom-arg
    check keeps it from silently loading as a *bare* declaration."""
    with pytest.raises(KBLoadError, match=r"malformed .relation."):
        _kb("(relation lives-in (Person House))")


def test_headless_relation_form_is_still_rejected():
    with pytest.raises(KBLoadError, match=r"needs a name"):
        _kb("(relation)")


def test_bare_declaration_still_rejects_duplicates_and_shadowing():
    with pytest.raises(KBLoadError, match=r"duplicate relation"):
        _kb("(relation opaque)\n(relation opaque)")
    with pytest.raises(KBLoadError, match=r"shadows a reserved kernel name"):
        _kb("(relation eq)")


# ── T1.22.4.3 — unary hypothesis targets ───────────────────────────


def test_unary_relation_is_enumerated():
    """One candidate per focal object — no filler loop, no self-edge."""
    kb = _kb("""
    (relation adult Person)
    (is-a Jack Person) (is-a Jill Person)
    """)
    unary = {f.args for f in generate_hypotheses(kb) if f.relation_name == "adult"}
    assert unary == {("Jack",), ("Jill",)}


def test_unary_and_binary_relations_are_enumerated_together():
    kb = _kb("""
    (relation adult Person)
    (relation likes Person Person)
    (is-a Jack Person) (is-a Jill Person)
    """)
    by_rel: dict[str, set] = {}
    for f in generate_hypotheses(kb):
        by_rel.setdefault(f.relation_name, set()).add(f.args)
    assert by_rel["adult"] == {("Jack",), ("Jill",)}
    assert by_rel["likes"] == {("Jack", "Jill"), ("Jill", "Jack")}


def test_arity_three_is_still_unenumerated():
    """Only arities 1 and 2 are filled; ≥ 3 remains out of scope."""
    kb = _kb("""
    (relation between Person Person Person)
    (is-a Jack Person) (is-a Jill Person)
    """)
    assert not [f for f in generate_hypotheses(kb) if f.relation_name == "between"]


# ── T1.22.4.5 — unary rendering in the compact view ────────────────


def _edges(dot: str, needle: str) -> list[str]:
    return [ln.strip() for ln in dot.splitlines() if "->" in ln and needle in ln]


def test_compact_view_draws_a_unary_fact_as_a_self_loop():
    dot = to_dot(parse("(symmetric next-to)")[0])
    assert _edges(dot, "next-to") == [
        '"next-to" -> "next-to" [label="symmetric", color="#7f7f7f", '
        'fontcolor="#7f7f7f", style=solid];'
    ]
    assert "octagon" not in dot


def test_levi_view_keeps_the_one_armed_octagon():
    dot = to_dot(parse("(symmetric next-to)")[0], levi=True)
    assert "octagon" in dot
    assert _edges(dot, "next-to") == [
        '"h_1_symmetric" -> "next-to" [label="1"];'
    ]


def test_kb_renderer_draws_unary_facts_as_self_loops():
    """The KB renderer is a separate emitter from `ir.to_dot` and needs the
    same convention — otherwise every membership fact becomes an octagon."""
    kb = _kb("(relation likes Person Drink)")
    dot = kb.to_dot()
    assert '"likes" -> "likes" [label="relation"' in dot
    # The arity-3 signature mirror still gets its octagon.
    assert "shape=octagon" in dot


def test_negation_is_not_drawn_as_a_self_loop():
    """`not` keeps its own encoding; a bare `(not X)` is a negation marker,
    not a property of `X`."""
    dot = to_dot(parse("(not Raining)")[0])
    assert '"Raining" -> "Raining"' not in dot
