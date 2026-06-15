"""`std.algebra` — the full relation-algebra signature (P1.8 S1.8.A12).

Extends `test_algebra.py` (the A7 converse / imply seed) with composition, the
Boolean lattice (meet / join / difference / complement / top / empty), the
identity, the single-relation property checks (irreflexive / antisymmetric /
asymmetric / connex / difunctional), and the Tarski/Maddux equational lemmas
(Schröder B10, contravariance B7, converse-over-join B8).

Every rule is generic (parametrised over the operand relations) and
activator-driven — same shape as `converse`. The extensive ops (identity / top /
complement / connex) take the puzzle's instance-type relation + the argument
types `(?isa Dom Ran)` and range over that universe. All rules are exercised
positive and negative; the generative ones are pinned to terminate.
"""
from __future__ import annotations

from ein.inference.saturator import Saturator, SaturatorStepLimitError
from ein.ir import parse
from ein.kb.store import KnowledgeBase


def _firings(src: str, max_steps: int = 4000):
    kb = KnowledgeBase.from_ir(parse(src))
    return [f for f in Saturator(kb).saturate(max_steps=max_steps)
            if not f.redundant]


def _derived(src: str, max_steps: int = 4000):
    return {(d.relation_name, d.args)
            for f in _firings(src, max_steps) for d in f.derived}


def _false(src: str, max_steps: int = 4000):
    return [f for f in _firings(src, max_steps)
            if any(d.relation_name == "false" for d in f.derived)]


def _negatives(src: str, max_steps: int = 4000):
    """The (rel, args) of every derived `(not (rel …))` fact."""
    out = set()
    for f in _firings(src, max_steps):
        for d in f.derived:                      # one firing may derive several
            if d.relation_name == "not":
                inner = d.args[0]                # a nested Fact (Q40)
                out.add((inner.relation_name, inner.args))
    return out


def _imp(*names: str) -> str:
    return f"(import std.algebra :symbols ({' '.join(names)}))\n"


# ── relative (composition) layer ───────────────────────────────────


def test_compose_chains_two_relations():
    """T1.8.A12.6.a — two-right = right-of ; right-of."""
    d = _derived(_imp("compose") + """
    (compose right-of right-of two-right)
    (right-of A B :source "(1)")
    (right-of B C :source "(2)")
    """)
    assert ("two-right", ("A", "C")) in d


def test_compose_into_self_is_transitive_closure():
    """(compose R R R) is exactly transitive closure of R — and terminates."""
    d = _derived(_imp("compose") + """
    (compose lt lt lt)
    (lt A B :source "(1)") (lt B C :source "(2)") (lt C D :source "(3)")
    """)
    assert ("lt", ("A", "C")) in d            # one hop
    assert ("lt", ("A", "D")) in d            # transitive across all three


def test_identity_self_loops_the_extent():
    """identity (1') — extensive: materialise (R a a) over the (isa _ Dom) extent."""
    d = _derived(_imp("identity") + """
    (is-a H1 House) (is-a H2 House)
    (identity same is-a House)
    """)
    assert ("same", ("H1", "H1")) in d
    assert ("same", ("H2", "H2")) in d
    assert ("same", ("H1", "H2")) not in d


# ── Boolean (lattice) layer ────────────────────────────────────────


def test_meet_is_intersection():
    d = _derived(_imp("meet") + """
    (meet owns rents both)
    (owns A B :source "(1)") (owns A C :source "(2)")
    (rents A B :source "(3)") (rents X Y :source "(4)")
    """)
    assert ("both", ("A", "B")) in d          # in both operands
    assert ("both", ("A", "C")) not in d      # owns only
    assert ("both", ("X", "Y")) not in d      # rents only


def test_difference_is_set_minus():
    d = _derived(_imp("difference") + """
    (difference owns rents only-owns)
    (owns A B :source "(1)") (owns A C :source "(2)")
    (rents A B :source "(3)")
    """)
    assert ("only-owns", ("A", "C")) in d     # owns, not rents
    assert ("only-owns", ("A", "B")) not in d  # in both → excluded


def test_join_fans_out_and_unions():
    """T1.8.A12.6.c — derive-join multi-asserts both copiers (A11 non-generic),
    which then (A9 reflective) union the operands."""
    d = _derived(_imp("derive-join", "join-l", "join-r") + """
    (join owns rents has)
    (owns A B :source "(1)")
    (rents C D :source "(2)")
    """)
    # the fan-out activators materialised
    assert ("join-l", ("owns", "rents", "has")) in d
    assert ("join-r", ("owns", "rents", "has")) in d
    # the union itself
    assert ("has", ("A", "B")) in d           # from the left operand
    assert ("has", ("C", "D")) in d           # from the right operand


def test_empty_check_fires_on_any_edge():
    assert _false(_imp("empty") + '(empty foo)\n(foo A B :source "(1)")')


def test_empty_check_silent_when_empty():
    assert not _false(_imp("empty") + "(empty foo)\n(bar A B :source \"(1)\")")


def test_top_fills_the_rectangle():
    d = _derived(_imp("top") + """
    (is-a A1 Dom) (is-a A2 Dom)
    (is-a B1 Ran)
    (top all is-a Dom Ran)
    """)
    assert ("all", ("A1", "B1")) in d
    assert ("all", ("A2", "B1")) in d


def test_complement_materialises_absent_pairs():
    """T1.8.A12.6.d — complement over a 2x2 universe with one positive edge:
    the three absent pairs appear, the present one does not (the positive
    shrinks the complement)."""
    d = _derived(_imp("complement") + """
    (is-a A1 Dom) (is-a A2 Dom)
    (is-a B1 Ran) (is-a B2 Ran)
    (r A1 B1 :source "(1)")
    (complement r co-r is-a Dom Ran)
    """)
    assert ("co-r", ("A1", "B2")) in d
    assert ("co-r", ("A2", "B1")) in d
    assert ("co-r", ("A2", "B2")) in d
    assert ("co-r", ("A1", "B1")) not in d     # the present edge is excluded


# ── single-relation property checks ────────────────────────────────


def test_irreflexive_rejects_self_loop():
    assert _false(_imp("irreflexive")
                  + '(irreflexive r)\n(r A A :source "(1)")')


def test_irreflexive_silent_without_self_loop():
    assert not _false(_imp("irreflexive")
                      + '(irreflexive r)\n(r A B :source "(1)")')


def test_antisymmetric_rejects_distinct_mutual_pair():
    assert _false(_imp("antisymmetric") + """
    (antisymmetric r)
    (r A B :source "(1)") (r B A :source "(2)")
    """)


def test_antisymmetric_allows_self_loop():
    """R(a,a) is permitted by antisymmetry (the neq guard) — unlike asymmetry."""
    assert not _false(_imp("antisymmetric")
                      + '(antisymmetric r)\n(r A A :source "(1)")')


def test_asymmetric_rejects_self_loop():
    """Asymmetry is strictly stronger: even a self-loop is a mutual pair → ⊥."""
    assert _false(_imp("asymmetric")
                  + '(asymmetric r)\n(r A A :source "(1)")')


def test_connex_rejects_incomparable_pair():
    """Two distinct Dom elements with neither orientation present → not total."""
    assert _false(_imp("connex") + """
    (is-a A Dom) (is-a B Dom)
    (connex r is-a Dom)
    """)


def test_connex_silent_when_comparable():
    assert not _false(_imp("connex") + """
    (is-a A Dom) (is-a B Dom)
    (connex r is-a Dom)
    (r A B :source "(1)")
    """)


def test_difunctional_closes_overlapping_rows():
    """R(a,b)∧R(c,b)∧R(c,d) ⟹ R(a,d): rows sharing a column agree elsewhere."""
    d = _derived(_imp("difunctional") + """
    (difunctional r)
    (r A B :source "(1)") (r C B :source "(2)") (r C D :source "(3)")
    """)
    assert ("r", ("A", "D")) in d


# ── property closures (the S1.8.A5 promotion: symmetric/transitive/includes) ──


def test_symmetric_mirrors_edges():
    d = _derived(_imp("symmetric")
                 + '(symmetric knows)\n(knows A B :source "(1)")')
    assert ("knows", ("B", "A")) in d


def test_transitive_closes_a_chain_without_self_loops():
    """The (neq ?a ?c) guard keeps an irreflexive chain irreflexive — unlike a
    bare (compose R R R), which would also add (R A A)."""
    d = _derived(_imp("transitive") + """
    (transitive lt)
    (lt A B :source "(1)") (lt B C :source "(2)") (lt C D :source "(3)")
    """)
    assert ("lt", ("A", "C")) in d
    assert ("lt", ("A", "D")) in d
    assert ("lt", ("A", "A")) not in d          # no self-loop (neq guard)


def test_includes_lifts_subrelation():
    """(includes is-a is-a*): every is-a edge lifts into is-a* (its closure)."""
    d = _derived(_imp("includes") + """
    (includes is-a is-a*)
    (is-a House-1 House :source "(1)")
    """)
    assert ("is-a*", ("House-1", "House")) in d


# ── equational theory (B6-B10 lemmas) ──────────────────────────────


def test_schroder_negative_s():
    """T1.8.A12.6.f — ¬T(a,c) ∧ R(a,b) ⟹ ¬S(b,c) for a closed T = R;S."""
    n = _negatives(_imp("compose-negative-s") + """
    (compose-negative-s right-of right-of two-right)
    (right-of A B :source "(1)")
    (not (two-right A C) :source "(2)")
    """)
    assert ("right-of", ("B", "C")) in n      # ¬S(b,c)


def test_schroder_negative_r():
    """Dual: ¬T(a,c) ∧ S(b,c) ⟹ ¬R(a,b)."""
    n = _negatives(_imp("compose-negative-r") + """
    (compose-negative-r right-of right-of two-right)
    (right-of B C :source "(1)")
    (not (two-right A C) :source "(2)")
    """)
    assert ("right-of", ("A", "B")) in n      # ¬R(a,b)


def test_contravariance_derives_converse_composite():
    """B7 — (compose R S T) + converses ⟹ (compose S° R° T°), then it composes."""
    d = _derived(_imp("compose", "converse", "compose-contravariant") + """
    (compose r s t)
    (converse r rc) (converse s sc) (converse t tc)
    """)
    assert ("compose", ("sc", "rc", "tc")) in d


def test_join_converse_derives_converse_union():
    """B8 — (join R S U) + converses ⟹ (join R° S° U°)."""
    d = _derived(_imp("derive-join", "join-l", "join-r",
                      "converse", "join-converse") + """
    (join r s u)
    (converse r rc) (converse s sc) (converse u uc)
    """)
    assert ("join", ("rc", "sc", "uc")) in d


def test_lemmas_terminate():
    """The reflective B7/B8 lemmas + converse-pair-symmetric must converge."""
    src = (_imp("compose", "converse", "converse-pair-symmetric",
                "compose-contravariant") + """
    (compose r s t)
    (converse r rc) (converse s sc) (converse t tc)
    (r A B :source "(1)") (s B C :source "(2)")
    """)
    kb = KnowledgeBase.from_ir(parse(src))
    try:
        n = sum(1 for _ in Saturator(kb).saturate(max_steps=8000))
    except SaturatorStepLimitError:
        raise AssertionError("relation-algebra lemma loop did not terminate") \
            from None
    assert n < 400
