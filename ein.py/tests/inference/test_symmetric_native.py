"""`__symmetric__` — the kernel-native symmetric mirror (Phase 2b).

The dunder-convention perf-opt counterpart of the stdlib `symmetric` rule: a
relation marked `(__symmetric__ R)` has its extension closed under arg-swap
directly by the saturator — no compiled rule, no `match.run`. Same closure as
`(symmetric R)`, produced faster (bench: `demo/bench_symmetric.py`).
"""
from ein_bot.inference.saturator import SYMMETRIC, Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

# 3 given edges: a normal pair, another pair, and a self-loop.
_EDGES = "(relation knows T T)\n(knows A B)\n(knows C D)\n(knows E E)\n"


def _knows(src: str, max_steps: int = 5000) -> set:
    kb = KnowledgeBase.from_ir(parse(src))
    list(Saturator(kb).saturate(max_steps=max_steps))
    return {f.args for f in kb._facts_by_relation.get("knows", ())}


def test_symmetric_constant():
    assert SYMMETRIC == "__symmetric__"


def test_native_mirror_derives_swap():
    k = _knows("(__symmetric__ knows)\n" + _EDGES)
    assert ("B", "A") in k
    assert ("D", "C") in k


def test_self_loop_not_duplicated():
    """`(R a a)` mirrors to itself — present once, no spurious fact, and the
    closure is exactly the 3 given + 2 swaps."""
    k = _knows("(__symmetric__ knows)\n" + _EDGES)
    assert k == {("A", "B"), ("B", "A"), ("C", "D"), ("D", "C"), ("E", "E")}


def test_parity_with_stdlib_symmetric():
    """Native `__symmetric__` and the stdlib `symmetric` rule produce the
    identical closure — the soundness basis for swapping the rule for the
    kernel opt."""
    native = _knows("(__symmetric__ knows)\n" + _EDGES)
    stdlib = _knows("(import std.algebra :symbols (symmetric))\n"
                    "(symmetric knows)\n" + _EDGES)
    assert native == stdlib


def test_unmarked_is_noop():
    """No `(__symmetric__ R)` ⇒ no mirroring (the mirror path is a no-op when
    nothing is marked — the basis for zero overhead on ordinary puzzles)."""
    assert ("B", "A") not in _knows(_EDGES)


def test_mirror_firing_has_provenance():
    """The mirror is a real Firing (rule `__symmetric__`) carrying the source
    edge as its premise — so it threads provenance and appears in the trace,
    just like a rule firing."""
    kb = KnowledgeBase.from_ir(parse("(__symmetric__ knows)\n" + _EDGES))
    firings = [f for f in Saturator(kb).saturate(max_steps=5000)
               if f.rule == SYMMETRIC]
    assert firings
    mirror = next(f for f in firings if f.derived[0].args == ("B", "A"))
    assert mirror.premises[0].args == ("A", "B")
    assert mirror.derived[0].provenance is not None
