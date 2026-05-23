"""Back-prop write + consume-loop tests — S1.5.7 T1.5.7.2 / .5 / .6.

``back_propagate`` writes ``(not h)`` into a KB on an unconditional
death; ``solver._consume`` drives it from the hypothesis loop. As of
2026-05-23 the ``enable_back_prop_unconditional`` flag defaults on —
the rest of the inference suite exercises the consume loop as the
*default* path; these tests pin the new behaviour explicitly and
contrast it against the opt-out (static) path.
"""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.back_prop import back_propagate
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.solver import Ambiguity, Solution, solve
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
BRANCHING = REPO / "examples" / "branching"


# ── back_propagate — the (not h) write (T1.5.7.2) ──────────────────


def _empty_kb() -> KnowledgeBase:
    kb = KnowledgeBase()
    kb.rebuild_indexes()
    return kb


def test_back_propagate_writes_negation():
    """``(not h)`` lands with ``<back-prop-unconditional>`` provenance
    citing the unsat-core frontier."""
    kb = _empty_kb()
    src = kb.add_fact(Fact(
        relation_name="s", args=("A",), layer=Layer.FACT,
        provenance=Provenance.from_source("(1)"),
    ))
    kb._index_fact(src)
    h = Fact(relation_name="co-located", args=("White", "H1"),
             layer=Layer.REASONING)
    not_h = back_propagate(kb, h, frozenset({src}))
    assert not_h.relation_name == "not"
    assert not_h.args == (h,)
    assert not_h.provenance.kind == "rule"
    assert not_h.provenance.rule == "<back-prop-unconditional>"
    assert not_h.provenance.premises_raw == (("s", ("A",)),)


def test_back_propagate_updates_negated_index():
    """The write feeds ``_negated_facts`` — the O(1) ``_prune_alive`` drop."""
    kb = _empty_kb()
    h = Fact(relation_name="co-located", args=("White", "H1"),
             layer=Layer.REASONING)
    back_propagate(kb, h, frozenset())
    assert ("co-located", ("White", "H1")) in kb._negated_facts


def test_back_propagate_is_idempotent():
    """Repeating a back-prop write is a no-op — no double-index."""
    kb = _empty_kb()
    h = Fact(relation_name="co-located", args=("White", "H1"),
             layer=Layer.REASONING)
    first = back_propagate(kb, h, frozenset())
    n_not = len(kb._facts_by_relation.get("not", ()))
    second = back_propagate(kb, h, frozenset())
    assert first is second
    assert len(kb._facts_by_relation.get("not", ())) == n_not


# ── consume loop, end-to-end (T1.5.7.5 / .6) ───────────────────────


def _solve(name: str, *, back_prop: bool):
    kb = KnowledgeBase.from_ir(parse((BRANCHING / name).read_text()))
    return solve(kb, config=SolverConfig(
        enable_back_prop_unconditional=back_prop))


def _solutions(verdict) -> tuple:
    if isinstance(verdict, Solution):
        return (verdict,)
    if isinstance(verdict, Ambiguity):
        return verdict.branches
    return ()


def _goal_bindings(kb) -> set:
    """Bindings of the query ``:goal`` pattern against ``kb``."""
    from ein_bot.inference.compile import JoinPlan, compile_pattern
    from ein_bot.inference.match import run as match_run
    if kb is None or kb.query is None:
        return set()
    goal = next((kp.value for kp in kb.query.kw_pairs
                 if getattr(kp, "key", None) is not None
                 and kp.key.name == "goal"), None)
    if goal is None:
        return set()
    plan = JoinPlan(rule_name="<q>", activator_args=(), bindings_seed={},
                    steps=tuple(compile_pattern(goal, {})),
                    assert_template=None, why="")
    return {tuple(sorted(b.items())) for b, _ in match_run(plan, kb)}


def _answer(verdict) -> set:
    """Union of the query goal's bindings across all solution branches."""
    rows: set = set()
    for s in _solutions(verdict):
        rows |= _goal_bindings(s.kb)
    return rows


def test_flag_on_off_same_verdict_and_answer():
    """Flag-on must not change the answer — same verdict class and the
    same query-goal bindings (T1.5.7.6.d / .7)."""
    for name in ("02_one_dead_one_alive.ein", "03_five_hyps_one_alive.ein"):
        off = _solve(name, back_prop=False)
        on = _solve(name, back_prop=True)
        assert type(on) is type(off), name
        assert _answer(on) == _answer(off), name
        assert _answer(on), name             # non-empty — the puzzle solved


def test_flag_on_never_grows_the_tree():
    """Back-prop caches deaths — the flag-on tree has ≤ the flag-off
    node count (T1.5.7.7)."""
    for name in ("02_one_dead_one_alive.ein", "03_five_hyps_one_alive.ein"):
        off = _solve(name, back_prop=False)
        on = _solve(name, back_prop=True)
        assert len(on.tree.nodes) <= len(off.tree.nodes), (
            name, len(on.tree.nodes), len(off.tree.nodes),
        )


# ── T1.5.7.3 — symmetric-dead promotion ────────────────────────────


def _kb_with_symmetric(rel: str) -> KnowledgeBase:
    """KB carrying a `(symmetric <rel>)` activator."""
    kb = _empty_kb()
    sym = Fact(relation_name="symmetric", args=(rel,), layer=Layer.ONTOLOGY)
    stored = kb.add_fact(sym)
    kb._index_fact(stored)
    return kb


def test_back_propagate_symmetric_writes_both_orderings():
    """``(symmetric R)`` ⇒ back-propping ``(not (R a b))`` also writes
    ``(not (R b a))`` — T1.5.7.3."""
    kb = _kb_with_symmetric("co-located")
    h = Fact(relation_name="co-located", args=("White", "H1"),
             layer=Layer.REASONING)
    back_propagate(kb, h, frozenset())
    assert ("co-located", ("White", "H1")) in kb._negated_facts
    assert ("co-located", ("H1", "White")) in kb._negated_facts


def test_back_propagate_symmetric_self_edge_no_mirror():
    """A self-edge ``(R a a)`` has no distinct mirror — only the primary."""
    kb = _kb_with_symmetric("co-located")
    h = Fact(relation_name="co-located", args=("X", "X"),
             layer=Layer.REASONING)
    n_before = len(kb._facts_by_relation.get("not", ()))
    back_propagate(kb, h, frozenset())
    assert len(kb._facts_by_relation.get("not", ())) == n_before + 1


def test_back_propagate_promote_symmetric_false_skips_mirror():
    """``promote_symmetric=False`` opts out of the mirror write."""
    kb = _kb_with_symmetric("co-located")
    h = Fact(relation_name="co-located", args=("White", "H1"),
             layer=Layer.REASONING)
    back_propagate(kb, h, frozenset(), promote_symmetric=False)
    assert ("co-located", ("White", "H1")) in kb._negated_facts
    assert ("co-located", ("H1", "White")) not in kb._negated_facts


def test_back_propagate_asymmetric_relation_no_mirror():
    """No ``(symmetric R)`` activator ⇒ no mirror written even for a
    2-arg fact."""
    kb = _empty_kb()
    h = Fact(relation_name="right-of", args=("A", "B"),
             layer=Layer.REASONING)
    back_propagate(kb, h, frozenset())
    assert ("right-of", ("A", "B")) in kb._negated_facts
    assert ("right-of", ("B", "A")) not in kb._negated_facts


# ── T1.5.7.4 — lookahead composition ───────────────────────────────


def test_lookahead_kills_back_prop_into_kb():
    """When the S1.5.6 lookahead kills a candidate during root
    enumeration, the kill is cached as a ``<lookahead-dies-immediately>``
    ``(not h)`` in the KB — T1.5.7.4.a."""
    kb = KnowledgeBase.from_ir(parse(
        (BRANCHING / "03_five_hyps_one_alive.ein").read_text()
    ))
    verdict = solve(kb, config=SolverConfig(
        enable_back_prop_unconditional=True,
        enable_pre_branch_lookahead=True,
    ))
    root_kb = verdict.tree.nodes[verdict.tree.root].kb_snapshot
    lookahead_kills = [
        f for f in root_kb.facts
        if f.relation_name == "not"
        and f.provenance is not None
        and f.provenance.rule == "<lookahead-dies-immediately>"
    ]
    assert lookahead_kills, "expected lookahead kills to back-prop"


def test_flag_on_back_prop_actually_fires():
    """On a puzzle with unconditionally-dead siblings the consume loop
    writes ``<back-prop-unconditional>`` negations into the KB.

    The S1.5.6 lookahead pre-filters most unconditional deaths *before*
    they reach the consume loop, so this test disables it — that
    isolates the ``_consume`` back-prop path. T1.5.7.4 will give the
    lookahead its own back-prop write and the assertion will hold
    with lookahead on too.
    """
    kb = KnowledgeBase.from_ir(parse(
        (BRANCHING / "03_five_hyps_one_alive.ein").read_text()
    ))
    verdict = solve(kb, config=SolverConfig(
        enable_back_prop_unconditional=True,
        enable_pre_branch_lookahead=False,
    ))
    root_kb = verdict.tree.nodes[verdict.tree.root].kb_snapshot
    back_propped = [
        f for f in root_kb.facts
        if f.relation_name == "not"
        and f.provenance is not None
        and f.provenance.rule == "<back-prop-unconditional>"
    ]
    assert back_propped, "expected back-prop to fire on the five-hyps demo"
