"""Apriori-gen unit tests — S1.5b.2 T1.5b.2.2.

Pins the textbook prefix-join + filter helpers in
:mod:`ein_bot.inference.apriori`. The module is pure
set-arithmetic, so every test is a hand-computed fixture; no
kb / saturator / engine state involved.
"""
from __future__ import annotations

from ein_bot.inference.apriori import (
    apriori_prefix_join,
    canonicalise,
    filter_candidate,
    generate_layer,
    layer_1,
)

# Reusable fact-id literals. The natural tuple order is alphabetical
# on the second-position arg, since the relation name is constant.
A = ("p", ("a",))
B = ("p", ("b",))
C = ("p", ("c",))
D = ("p", ("d",))


# ── canonicalise ───────────────────────────────────────────────────


def test_canonicalise_round_trip_from_frozenset():
    assert canonicalise(frozenset({C, A, B})) == (A, B, C)


def test_canonicalise_idempotent():
    once = canonicalise([B, A, C])
    twice = canonicalise(once)
    assert once == twice == (A, B, C)


def test_canonicalise_dedups():
    assert canonicalise([A, B, A, C, B]) == (A, B, C)


def test_canonicalise_sort_matches_tuple_order():
    # FactId = (relation_name, args); natural tuple ordering applies.
    x = ("rel-a", (1,))
    y = ("rel-a", (2,))
    z = ("rel-b", (1,))
    # rel-a < rel-b regardless of args; within rel-a, args ordering.
    assert canonicalise([z, y, x]) == (x, y, z)


# ── apriori_prefix_join ────────────────────────────────────────────


def test_prefix_join_singletons_yield_all_pairs():
    result = list(apriori_prefix_join([(A,), (B,), (C,)]))
    assert result == [(A, B), (A, C), (B, C)]


def test_prefix_join_triangle_yields_single_triple():
    # Closed-triangle a_prev: {(A,B), (A,C), (B,C)} → (A,B,C) once.
    result = list(apriori_prefix_join([(A, B), (A, C), (B, C)]))
    assert result == [(A, B, C)]


def test_prefix_join_disjoint_pairs_yield_nothing():
    # (A,B) and (C,D) share no prefix → no emission.
    result = list(apriori_prefix_join([(A, B), (C, D)]))
    assert result == []


def test_prefix_join_emits_each_candidate_once():
    # A 4-element complete a_prev (all 6 pairs) produces every
    # 3-subset exactly once.
    a_prev = [(A, B), (A, C), (A, D), (B, C), (B, D), (C, D)]
    expected = [(A, B, C), (A, B, D), (A, C, D), (B, C, D)]
    assert sorted(apriori_prefix_join(a_prev)) == expected


# ── filter_candidate ───────────────────────────────────────────────


def test_filter_candidate_passes_when_alive_and_no_clause_subset():
    assert filter_candidate(
        (A, B, C),
        alive=frozenset({A, B, C}),
        nogoods=[frozenset({A, D})],  # has D, not a subset of {A,B,C}
    ) is True


def test_filter_candidate_drops_on_alive_miss():
    assert filter_candidate(
        (A, B, C),
        alive=frozenset({A, C}),  # B dropped from alive
        nogoods=[],
    ) is False


def test_filter_candidate_drops_on_nogood_subset():
    assert filter_candidate(
        (A, B, C),
        alive=frozenset({A, B, C}),
        nogoods=[frozenset({A, B})],  # {A,B} ⊂ {A,B,C}
    ) is False


def test_filter_candidate_drops_on_both():
    # Even if just one check fails it drops; this guards against
    # short-circuit bugs that skip the second check.
    assert filter_candidate(
        (A, B, C),
        alive=frozenset({A}),
        nogoods=[frozenset({A, B})],
    ) is False


def test_filter_candidate_empty_nogoods_is_no_op():
    assert filter_candidate(
        (A, B),
        alive=frozenset({A, B}),
        nogoods=[],
    ) is True


# ── generate_layer ────────────────────────────────────────────────


def test_generate_layer_4elem_with_alive_miss_and_nogood():
    # 4 elements; a_prev is the complete pair-set; B died after
    # a_prev closed (alive shrinks); one 2-clause nogood {A,B}
    # exists. The four triples that prefix-join produces are all
    # dropped except (A,C,D): B-containing triples fail alive,
    # and (A,C,D) escapes the nogood because B is not in it.
    a_prev = [(A, B), (A, C), (A, D), (B, C), (B, D), (C, D)]
    survivors = generate_layer(
        a_prev,
        alive=frozenset({A, C, D}),  # B back-propagated dead
        nogoods=[frozenset({A, B})],
    )
    assert survivors == [(A, C, D)]


def test_generate_layer_returns_canonical_order():
    # With no filters, generate_layer returns survivors in the same
    # canonical-sort order that apriori_prefix_join yields.
    a_prev = [(A, B), (A, C), (A, D), (B, C), (B, D), (C, D)]
    survivors = generate_layer(
        a_prev,
        alive=frozenset({A, B, C, D}),
        nogoods=[],
    )
    assert survivors == [(A, B, C), (A, B, D), (A, C, D), (B, C, D)]


# ── layer_1 ────────────────────────────────────────────────────────


def test_layer_1_emits_every_singleton_sorted():
    assert layer_1(frozenset({C, A, B})) == [(A,), (B,), (C,)]


def test_layer_1_on_empty_alive():
    assert layer_1(frozenset()) == []
