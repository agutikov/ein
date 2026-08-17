"""`__symmetric__` — the kernel-native symmetric mirror (Phase 2b).

The dunder-convention perf-opt counterpart of the stdlib `symmetric` rule: a
relation marked `(__symmetric__ R)` has its extension closed under arg-swap
directly by the saturator — no compiled rule, no `match.run`. Same closure as
`(symmetric R)`, produced faster (bench: `demo/bench_symmetric.py`).

The last two tests pin **hazard H1** from the M1a determinism audit
(`plans/m1a_rust/design/02_determinism_and_order.md` §5, closed by S1a.0.1):
the mirror's cold seed used to iterate the marked-relation `frozenset`, so with
two or more markers the firing order — and every derivation-order observable
downstream of it — depended on `PYTHONHASHSEED`.
"""
import subprocess
import sys

from ein.inference.saturator import SYMMETRIC, Saturator
from ein.ir import parse
from ein.kb.store import KnowledgeBase

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


# ── H1: the seed order is content-determined ───────────────────────

# Three markers, listed out of sorted order so a seed that followed *file*
# order would not pass either. One edge each ⇒ three mirror firings, and their
# order is the whole observable.
_MULTI = (
    "(relation knows T T)\n(relation rivals T T)\n(relation allies T T)\n"
    "(__symmetric__ knows)\n(__symmetric__ rivals)\n(__symmetric__ allies)\n"
    "(knows Ann Bob)\n(rivals Cid Dee)\n(allies Eve Fay)\n"
)

# The cold seed walks the markers sorted (allies, knows, rivals) and
# `_mirror_queue` is a LIFO, so the last-seeded relation mirrors first.
_MULTI_ORDER = ["rivals", "knows", "allies"]

_DRIVER = """
import sys
from ein.inference.saturator import SYMMETRIC, Saturator
from ein.ir import parse
from ein.kb.store import KnowledgeBase
kb = KnowledgeBase.from_ir(parse(sys.argv[1]))
print(" ".join(f.derived[0].relation_name
               for f in Saturator(kb).saturate(max_steps=5000)
               if f.rule == SYMMETRIC))
"""


def test_multi_marker_mirror_order_is_sorted():
    """With several markers the firing order is a function of the input: the
    marked relations are seeded in name order into a LIFO, so they mirror in
    reverse name order. Pins the *content* of the fix, in-process."""
    kb = KnowledgeBase.from_ir(parse(_MULTI))
    order = [f.derived[0].relation_name
             for f in Saturator(kb).saturate(max_steps=5000)
             if f.rule == SYMMETRIC]
    assert order == _MULTI_ORDER


def test_multi_marker_mirror_order_survives_hash_seeds():
    """…and pins the *property*: the same order under every `PYTHONHASHSEED`.

    The in-process test above would only catch a regression to `frozenset`
    iteration on the seeds where the set order happens to differ — i.e.
    flakily. This one is the real assertion, and it needs a fresh interpreter
    per seed because the hash salt is fixed at startup.
    """
    seen = {}
    for seed in ("0", "1", "42", "12345"):
        proc = subprocess.run(
            [sys.executable, "-c", _DRIVER, _MULTI],
            capture_output=True, text=True,
            env={"PYTHONHASHSEED": seed, "PYTHONPATH": ":".join(sys.path)},
        )
        assert proc.returncode == 0, proc.stderr
        seen[seed] = proc.stdout.strip()
    assert set(seen.values()) == {" ".join(_MULTI_ORDER)}, seen
