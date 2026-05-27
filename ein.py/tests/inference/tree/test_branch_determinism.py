"""Branch-exploration determinism — S1.5a.1a T1.5a.1a.2.

Asserts that `solve()` visits hypothesis branches in the same order
across Python processes, regardless of `PYTHONHASHSEED`. The risk
the test guards against: any frozenset/set whose iteration order
leaks into the user-visible branch sequence will trip these
assertions, because each subprocess starts with a fresh randomised
hash seed and would re-order otherwise.

The single observable boundary today is
[`solver._candidates_for`](../../src/ein_bot/inference/solver.py),
which sorts by [`_candidate_sort_key`](../../src/ein_bot/inference/solver.py).
The score component (`hypgen.score_hypothesis`) is a stub returning
0 in M1, so the effective sort is content-based
``(fact.args, fact.relation_name)`` — neither depends on `hash(str)`.

If any future change re-introduces a hash-ordered iteration on the
hypothesis loop's hot path, `test_solve_branch_order_stable_across_hash_seeds`
fails.
"""
from __future__ import annotations

import subprocess
import sys
import textwrap
from pathlib import Path

REPO = Path(__file__).resolve().parents[4]
DEMO = REPO / "examples" / "branching" / "03_five_hyps_one_alive.ein"


_DRIVER = textwrap.dedent("""
    import sys
    from pathlib import Path
    sys.path.insert(0, {src!r})

    from ein_bot.inference.tree.solver import solve
    from ein_bot.ir import parse
    from ein_bot.kb.store import KnowledgeBase

    text = Path({demo!r}).read_text()
    kb = KnowledgeBase.from_ir(parse(text))
    verdict = solve(kb, max_depth=2)
    tree = verdict.tree
    for nid in sorted(tree.nodes):
        node = tree.nodes[nid]
        if node.hypothesis is None:
            continue
        args = " ".join(
            a if isinstance(a, str) else f"<fact:{{a.relation_name}}>"
            for a in node.hypothesis.args
        )
        print(f"b{{nid}} {{node.hypothesis.relation_name}} {{args}}")
""")


def _run_with_seed(seed: str) -> str:
    """Invoke the driver as a subprocess with the given PYTHONHASHSEED."""
    src = REPO / "ein.py" / "src"
    script = _DRIVER.format(src=str(src), demo=str(DEMO))
    result = subprocess.run(
        [sys.executable, "-c", script],
        capture_output=True,
        text=True,
        env={"PYTHONHASHSEED": seed, "PATH": "/usr/bin:/bin"},
        timeout=60,
    )
    assert result.returncode == 0, (
        f"driver failed (seed={seed}):\nstdout:\n{result.stdout}\n"
        f"stderr:\n{result.stderr}"
    )
    return result.stdout


def test_solve_branch_order_stable_across_hash_seeds():
    """Two subprocesses with different PYTHONHASHSEED visit branches
    in the same order. The smoke gun: without the
    `_candidate_sort_key` sort in `solver._candidates_for`, the
    `kb.alive` frozenset iterates in a different order per process,
    and the two outputs diverge.
    """
    out_a = _run_with_seed("0")
    out_b = _run_with_seed("12345")
    out_c = _run_with_seed("random")
    assert out_a == out_b, (
        f"branch order differs between hash seeds 0 and 12345:\n"
        f"--- seed=0 ---\n{out_a}\n"
        f"--- seed=12345 ---\n{out_b}"
    )
    assert out_a == out_c, (
        f"branch order differs between hash seeds 0 and random:\n"
        f"--- seed=0 ---\n{out_a}\n"
        f"--- seed=random ---\n{out_c}"
    )


def test_solve_branch_order_stable_in_process():
    """Two `solve()` calls in the same process produce the same
    branch sequence — basic sanity check that the sort is
    deterministic within a process, independent of any hash-seed
    concern."""
    from ein_bot.inference.tree.solver import solve
    from ein_bot.ir import parse
    from ein_bot.kb.store import KnowledgeBase

    text = DEMO.read_text()

    def _run_once() -> list[tuple[str, tuple]]:
        kb = KnowledgeBase.from_ir(parse(text))
        verdict = solve(kb, max_depth=2)
        tree = verdict.tree
        return [
            (n.hypothesis.relation_name, n.hypothesis.args)
            for nid in sorted(tree.nodes)
            for n in (tree.nodes[nid],)
            if n.hypothesis is not None
        ]

    a = _run_once()
    b = _run_once()
    assert a == b, f"in-process branch order is non-deterministic:\n  {a}\n  vs\n  {b}"
    assert a, "sanity: solve should produce at least one branch"


def test_candidate_sort_key_is_content_based():
    """`_candidate_sort_key` reads only `fact.args` and
    `fact.relation_name` (plus a constant score in M1). Hash of any
    string never reaches the returned tuple.
    """
    from ein_bot.inference.tree.solver import _candidate_sort_key
    from ein_bot.ir import parse
    from ein_bot.kb.entities import Fact, Layer
    from ein_bot.kb.store import KnowledgeBase

    kb = KnowledgeBase.from_ir(parse("(ontology (relation r T T))"))
    f1 = Fact(relation_name="r", args=("a", "b"), layer=Layer.REASONING,
              provenance=None)
    f2 = Fact(relation_name="r", args=("a", "c"), layer=Layer.REASONING,
              provenance=None)
    k1 = _candidate_sort_key(f1, kb)
    k2 = _candidate_sort_key(f2, kb)
    # Score is 0 in M1; key shape is (-0, args, rel).
    assert k1 == (0, ("a", "b"), "r")
    assert k2 == (0, ("a", "c"), "r")
    # Ordering: f1 < f2 because args differ at slot 1.
    assert k1 < k2
