"""`--events` — the oracle event protocol (M1a S1a.0.2).

The T2 parity surface: one JSON object per line describing what the engine did.
The schema and its rationale are
[`conformance/EVENTS.md`](../../conformance/EVENTS.md); what is pinned here is
the contract the harness depends on.

- **Additive.** stdout, stderr and the exit code are unchanged with the flag.
- **Off is free.** No writer, no formatting, no event.
- **Complete.** Every event kind in the schema is reachable, and each is
  reached by some fixture in the corpus. A kind nothing emits is a kind the
  port can drop unnoticed.
- **Faithful.** The stream reproduces across runs and across `PYTHONHASHSEED`,
  because it is a description of a deterministic engine.
"""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

import pytest

REPO = Path(__file__).resolve().parents[2]
EXAMPLES = REPO / "examples"
CMD = [sys.executable, "-m", "ein.cli"]

# Every kind the schema defines, and a fixture that reaches it. `alt`,
# `retire` and `writeback` need shapes no small fixture has, so they are
# checked structurally in the engine's own tests rather than here.
SCHEMA_KINDS = {
    "run", "load", "verdict", "compile", "enqueue", "fire", "mirror",
    "park", "admit", "retire", "quiesce", "alt",
    "hyp", "hypskip", "enter", "nogood", "writeback",
}


def _run(*args: str, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run([*CMD, *args], capture_output=True, text=True,
                          check=check)


def _events(tmp_path: Path, *args: str, level: str = "verbose") -> list[dict]:
    tmp_path.mkdir(parents=True, exist_ok=True)
    out = tmp_path / "events.jsonl"
    proc = _run(*args, "--events", str(out), "--events-level", level)
    assert out.is_file(), proc.stderr
    return [json.loads(line) for line in out.read_text().splitlines() if line]


def _kinds(events: list[dict]) -> set[str]:
    return {e["e"] for e in events}


# ── shape ──────────────────────────────────────────────────────────


def test_the_first_event_identifies_the_schema(tmp_path: Path):
    """A consumer must be able to reject a file it does not understand before
    reading further — so the version rides in the first line, not a header."""
    events = _events(tmp_path, "solve", str(EXAMPLES / "branching/04_two_levels.ein"))
    assert events[0]["e"] == "run"
    assert events[0]["version"] == "ein-events/1"
    assert events[0]["n"] == 0
    assert "config" in events[0] and events[0]["config"]["lattice-order"] == "lex"


def test_the_sequence_is_dense_and_monotonic(tmp_path: Path):
    """`n` is a position. A gap would make the differ's "first difference at
    event k" mean two different things on the two sides."""
    events = _events(tmp_path, "solve", str(EXAMPLES / "branching/04_two_levels.ein"))
    assert [e["n"] for e in events] == list(range(len(events)))


def test_the_last_event_is_the_verdict(tmp_path: Path):
    events = _events(tmp_path, "solve", "-m", "3", "-e",
                     str(EXAMPLES / "branching/04_two_levels.ein"))
    last = events[-1]
    assert last["e"] == "verdict"
    assert last["type"] == "Ambiguity"
    assert last["k"] == 2 and last["exhausted"] is True
    assert last["counters"]["enterings_total"] > 0
    assert len(last["models"]) == 2


# ── additive ───────────────────────────────────────────────────────


def test_recording_changes_nothing_else(tmp_path: Path):
    """Same stdout, stderr and exit code with and without the flag — so one
    invocation can serve T2 and T3 at once, and so the protocol cannot be
    perturbing the run it describes."""
    fixture = str(EXAMPLES / "branching/04_two_levels.ein")
    with_ = _run("solve", fixture, "-m", "2", "-p",
                 "--events", str(tmp_path / "e.jsonl"))
    without = _run("solve", fixture, "-m", "2", "-p")
    assert (with_.stdout, with_.stderr, with_.returncode) == \
           (without.stdout, without.stderr, without.returncode)


def test_off_is_off():
    """With no writer the emitter is inert, not merely quiet: `emit` returns
    without formatting anything and without advancing the counter, so a call
    site that forgot its `if events.ON:` guard is still correct — just not
    free."""
    from ein import events
    assert events.ON is False
    events.emit("fire", rule="never-written")
    assert events.seq() == 0


# ── completeness ───────────────────────────────────────────────────


def test_the_deductive_and_search_layers_both_narrate(tmp_path: Path):
    """A solve that branches reaches both halves of the schema. Before this
    protocol only the search half was visible (`--dump-states`), which is the
    half a port is least likely to get wrong."""
    kinds = _kinds(_events(tmp_path, "solve", "-m", "3", "-e",
                           str(EXAMPLES / "branching/04_two_levels.ein")))
    assert {"compile", "enqueue", "fire", "quiesce"} <= kinds     # deductive
    assert {"hyp", "enter", "nogood", "verdict"} <= kinds         # search


def test_the_naf_boundary_narrates(tmp_path: Path):
    """`park` / `admit` are the two-phase loop's own observables — the part of
    S1.21.8 that no counter outside the Saturator reports."""
    kinds = _kinds(_events(tmp_path, "solve",
                           str(EXAMPLES / "features/03_forall.ein")))
    assert {"park", "admit", "quiesce"} <= kinds


def test_the_symmetric_mirror_narrates(tmp_path: Path):
    """A mirror is a `mirror`, never a `fire` — so a firing is reported
    exactly once whichever path made it."""
    events = _events(tmp_path, "solve",
                     str(EXAMPLES / "features/06_symmetric_native.ein"))
    mirrors = [e for e in events if e["e"] == "mirror"]
    assert len(mirrors) == 3                       # knows, rivals, allies
    assert {m["relation"] for m in mirrors} == {"knows", "rivals", "allies"}
    assert not [e for e in events if e["e"] == "fire"]


def test_every_kind_the_schema_defines_is_emitted_somewhere(tmp_path: Path):
    """The corpus-level completeness claim. A kind nothing reaches is a kind
    the port can silently drop, and the tier that was meant to catch it would
    stay green."""
    seen: set[str] = set()
    for args in (
        ("solve", "-m", "3", "-e", str(EXAMPLES / "branching/04_two_levels.ein")),
        ("solve", str(EXAMPLES / "features/03_forall.ein")),
        ("solve", str(EXAMPLES / "features/06_symmetric_native.ein")),
        ("solve", "-e", str(EXAMPLES / "branching/03_five_hyps_one_alive.ein")),
        ("saturate", str(EXAMPLES / "saturation/transitive/taxonomy.ein")),
    ):
        seen |= _kinds(_events(tmp_path, *args))
    missing = SCHEMA_KINDS - seen
    # `alt` / `retire` / `writeback` need shapes no small fixture has.
    assert missing <= {"alt", "retire", "writeback"}, missing


# ── faithful ───────────────────────────────────────────────────────


@pytest.mark.parametrize("seed", ["0", "42"])
def test_the_stream_is_hash_seed_independent(tmp_path: Path, seed: str):
    """The engine is deterministic, so its narration is too. This is the
    determinism sweep in miniature: a `set` iterated at any instrumented site
    would show up here as a reordered stream."""
    fixture = str(EXAMPLES / "features/03_forall.ein")
    streams = []
    for name, env_seed in (("base", "1"), (f"seed{seed}", seed)):
        out = tmp_path / f"{name}.jsonl"
        proc = subprocess.run(
            [*CMD, "solve", fixture, "--events", str(out),
             "--events-level", "verbose"],
            capture_output=True, text=True,
            env={"PYTHONHASHSEED": env_seed, "PYTHONPATH": ":".join(sys.path)},
        )
        assert proc.returncode == 0, proc.stderr
        events = [json.loads(line) for line in out.read_text().splitlines()]
        # The `run` event's `argv` names the file the *caller* chose, and its
        # `impl` names which engine ran; the differ excludes both for the same
        # reason (`conformance/EVENTS.md` § Lifecycle).
        for e in events:
            if e["e"] == "run":
                e.pop("argv"), e.pop("impl")
        streams.append(events)
    assert streams[0] == streams[1]


def test_verbose_adds_the_high_volume_kinds(tmp_path: Path):
    """`normal` stays hand-readable; `verbose` is what T2 runs at, because a
    dropped redundant firing is exactly what a port loses."""
    # A blind-enumerator fixture: `mini_zebra` declares hrules, and hrule
    # generation never reaches the pre-candidate skips at all.
    fixture = str(EXAMPLES / "features/03_forall.ein")
    normal = _events(tmp_path / "n", "solve", fixture, level="normal")
    verbose = _events(tmp_path / "v", "solve", fixture, level="verbose")
    assert len(verbose) > len(normal)
    assert "hypskip" in _kinds(verbose)
    assert "hypskip" not in _kinds(normal)


def test_an_unknown_level_is_rejected(tmp_path: Path):
    proc = _run("solve", str(EXAMPLES / "branching/04_two_levels.ein"),
                "--events", str(tmp_path / "e.jsonl"),
                "--events-level", "chatty", check=False)
    assert proc.returncode != 0
    assert "chatty" in proc.stderr
