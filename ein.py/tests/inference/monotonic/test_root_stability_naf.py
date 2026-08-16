"""Root stability under NAF — P1.21 R2 regression.

Pins the counterexample that retired the "unconditional fact"
extraction (P1.21 R2 report §3 —
``plans/m1_core_graph_reasoning/p1.21_review_response/reports/``). An
``absent`` guard contributes **no premise** to a firing
(``match.py``), so a fork fact derived through NAF carries no
provenance edge to the commitment whose absence licensed it — the
retired positive-chain walk read it as "provably true at root"
while a sibling commitment's genuinely consistent world refutes
it. The live model is **keep root stable**: mid-search root writes
are limited to the singleton-death ``(not h)`` writeback and the
forced-positive cascade; fork facts never merge back. See the
historical note in ``docs/kernel/inference/README.md``.

Two pin groups:

1. **Primitive level** (the probe from the report, verbatim):
   under commitment ``{g(b)}`` the fork derives ``(y s)`` via
   ``absent (x a)`` and the root is untouched; the sibling
   ``{x(a)}`` world is alive *without* ``(y s)``; simulating the
   retired merge (root' = root + ``(y s)``) flips that consistent
   world to dead-post — the soundness bug the removal closed.
2. **Solve level**: a search whose forks derive ``(y …)`` facts
   through NAF leaves the root's fact set byte-identical (this
   fixture triggers no sound root write either), and the
   ``{x(a)}``-style world stays satisfiable afterwards.
"""
from __future__ import annotations

from ein.inference.commitment import try_commitment_set
from ein.inference.config import SolverConfig
from ein.inference.monotonic import solve
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.provenance import Provenance
from ein.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _ids(facts) -> set[tuple[str, tuple]]:
    return {(f.relation_name, f.args) for f in facts}


# Keep every death on the try_commitment_set path (no lookahead
# kill-cache `(not h)` root writes muddying the stability check).
_NO_LOOKAHEAD = SolverConfig(
    enable_pre_branch_lookahead=False,
    enable_lookahead_kill_cache=False,
)


# ── 1) Primitive level — the report's §3 probe ────────────────────

# `(y s)` is derivable from the ROOT fact `(seed s)` + the NAF guard
# `absent (x a)`; the retired walk therefore classified it
# "unconditional" under any unrelated commitment — yet the `{x(a)}`
# world (genuinely consistent) refutes it.
PROBE_FIXTURE = """
(rule y-when-no-x ()
  :match  (and (seed ?s) (absent (x a)))
  :assert (y ?s)
  :why    "no x(a) -> y {?s}"
  :priority 100)
(rule x-y-clash ()
  :match  (and (x ?o) (y ?s))
  :assert (false)
  :why    "x and y together are inconsistent"
  :priority 250)
(relation x T) (relation y T) (relation seed T) (relation g T)
(is-a a T) (is-a s T) (is-a b T)
(seed s)
"""


def test_absent_derived_fork_fact_never_touches_root():
    """Under the unrelated commitment ``{g(b)}`` the fork derives
    ``(y s)`` via ``absent (x a)``; the root keeps its exact fact
    set — nothing is merged back."""
    kb = _kb(PROBE_FIXTURE)
    root_ids_before = _ids(kb.facts)

    result = try_commitment_set(kb, (("g", ("b",)),))

    assert result.kind == "alive"
    assert ("y", ("s",)) in _ids(result.kb.facts)
    assert _ids(kb.facts) == root_ids_before
    assert ("y", ("s",)) not in _ids(kb.facts)


def test_sibling_x_world_is_alive_without_y():
    """``{x(a)}`` is a genuinely consistent world in which ``(y s)``
    does NOT hold — the direct refutation of "``(y s)`` is true at
    root": a root-true fact must hold in every consistent extension
    of root."""
    kb = _kb(PROBE_FIXTURE)

    result = try_commitment_set(kb, (("x", ("a",)),))

    assert result.kind == "alive"
    assert ("y", ("s",)) not in _ids(result.kb.facts)


def test_simulated_merge_refutes_the_consistent_x_world():
    """Performing the retired merge (root' = root + ``(y s)``) flips
    the consistent ``{x(a)}`` world to dead-post — undercounting
    ``k`` and thereby corrupting the verdict. This is why the
    extraction was removed rather than kept dormant."""
    kb = _kb(PROBE_FIXTURE)
    kb.add_and_index_fact(Fact(
        relation_name="y",
        args=("s",),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(
            rule="<retired-merge-simulation>", premises_raw=(),
        ),
    ))

    result = try_commitment_set(kb, (("x", ("a",)),))

    assert result.kind == "dead-post"


# ── 2) Solve level — root fact set stable across the search ───────

# The NAF rule's positive premise is the *hypothesis* `(h ?s ?t)`,
# so `(y …)` facts arise only inside forks (a root-grounded NAF
# firing would already have happened during Phase-1 saturation).
# Pair commitments `{h(…), x(s,a)}` die via the clash (fork-level
# NAF derivations demonstrably happen); no singleton ever dies and
# the alive set never becomes a singleton, so this run performs
# ZERO sound root writes — root must come out byte-identical.
# (Relations are binary: hypgen's blind enumerator only fills
# arity-2 slots in M1.)
SOLVE_FIXTURE = """
(rule y-when-h-no-x ()
  :match  (and (h ?s ?t) (absent (x a s)))
  :assert (y ?s ?t)
  :why    "h {?s} {?t} and no x(a,s) -> y {?s} {?t}"
  :priority 100)
(rule x-y-clash ()
  :match  (and (x ?o ?p) (y ?s ?t))
  :assert (false)
  :why    "x and y together are inconsistent"
  :priority 250)
(relation x Thing Thing)
(relation h Thing Thing)
(relation y Thing Thing)
(relation never T)
(is-a Thing T)
(is-a a Thing) (is-a s Thing)

(query
  :goal  (never ?q)
  :hypothesis-relations (x h))
"""


def test_solve_keeps_root_fact_set_stable_under_naf():
    """Exhaustive solve: forks derive `(y …)` via NAF (witnessed by
    the dead pairs), yet the root's fact set is unchanged and the
    `{x(a,s)}`-style world stays satisfiable afterwards."""
    kb = _kb(SOLVE_FIXTURE)
    root_ids_before = _ids(kb.facts)

    _verdict, stats = solve(kb, max_set_size=4, config=_NO_LOOKAHEAD)

    # Fork-level NAF derivations happened: {h(…), x(s,a)} pairs
    # clash — the fork derived (y …) via `absent (x a s)` first.
    assert stats.enterings_dead_post >= 2
    assert frozenset(
        {("h", ("a", "s")), ("x", ("s", "a"))},
    ) in kb._nogoods
    # …but the root fact set is exactly what Phase-1 left: no
    # `absent`-derived fork fact leaked, no other root write ran.
    assert _ids(kb.facts) == root_ids_before
    assert not any(rn == "y" for rn, _args in _ids(kb.facts))
    assert stats.facts_merged == 0
    assert stats.forced_positives == 0

    # The x-world stays satisfiable against the post-solve root.
    result = try_commitment_set(kb, (("x", ("a", "s")),))
    assert result.kind == "alive"
    assert not any(rn == "y" for rn, _args in _ids(result.kb.facts))

    # And the h-world still derives its NAF fact fork-locally only.
    result_h = try_commitment_set(kb, (("h", ("a", "s")),))
    assert result_h.kind == "alive"
    assert ("y", ("a", "s")) in _ids(result_h.kb.facts)
    assert not any(rn == "y" for rn, _args in _ids(kb.facts))
