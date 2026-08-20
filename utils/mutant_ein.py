#!/usr/bin/env python3
"""The parity gate's own negative control — a deliberately broken `ein`.

**Superseded.** The control moved into `cargo test` at
[S1a.10.3](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md)
as `ein.rs/crates/ein-infer/tests/event_cut_control.rs`, which applies these
same three mutations to a stream it produced in-process. The mutation was
always applied to the *artefact*, so the two processes and the harness that
diffed them bought nothing but a way to produce one — and the harness is gone.
This script no longer has a runner; deleting it is
[S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md)
T1a.10.4.1's.

    ein-conformance run --tier T2 \\
        --impl-a ".venv-pypy/bin/python -m ein.cli" \\
        --impl-b "python3 utils/mutant_ein.py ein.rs/target/release/ein"

[S1a.6.10](../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)
narrowed T2 from "the whole event stream" to "what each fork derived", because
since [S1a.6.9](../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
the two engines narrate different amounts of the same derivation on purpose
([D3](../plans/m1a_rust/divergences.md)). A relaxation that cannot be *shown*
to still catch the thing it was relaxed around is a hole rather than a
decision, so this is the showing: a wrapper that runs the real binary and then
edits one event out of the log it wrote, which the gate must still report.

Three mutations, `EIN_MUTANT=` (default `productive`):

| value | what it deletes | the gate must |
|---|---|---|
| `productive` | the first `fire` with `redundant = false` | **report it** — a derivation went missing |
| `redundant` | the first `fire` with `redundant = true` | pass — that is the narration the cut elides |
| `enqueue` | the first `enqueue` | pass — likewise |

The last two are the *positive* controls: if the gate reported them it would
still be comparing narration, and D3 would still cost it 97 cells. Run all
three and the relaxation is calibrated in both directions rather than asserted.

Nothing about the engine changes: the mutation is applied to the artefact, so
the binary under test is the shipping one, byte for byte.
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

MUTATIONS = {
    "productive": lambda e: e.get("e") == "fire" and not e.get("redundant"),
    "redundant": lambda e: e.get("e") == "fire" and e.get("redundant"),
    "enqueue": lambda e: e.get("e") == "enqueue",
}


def events_path(argv: list[str]) -> Path | None:
    for i, tok in enumerate(argv):
        if tok == "--events" and i + 1 < len(argv):
            return Path(argv[i + 1])
    return None


def mutate(path: Path, which: str) -> bool:
    """Delete the first matching event. True if one was deleted."""
    picked = MUTATIONS[which]
    lines = path.read_text(encoding="utf-8", errors="replace").splitlines(True)
    for i, line in enumerate(lines):
        if not line.strip():
            continue
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if picked(event):
            del lines[i]
            path.write_text("".join(lines), encoding="utf-8")
            return True
    return False


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__, file=sys.stderr)
        return 2
    which = os.environ.get("EIN_MUTANT", "productive")
    if which not in MUTATIONS:
        print(f"mutant_ein: unknown EIN_MUTANT={which!r} "
              f"(one of {', '.join(MUTATIONS)})", file=sys.stderr)
        return 2
    binary, argv = sys.argv[1], sys.argv[2:]
    # stdout/stderr are inherited, so the harness captures the real engine's
    # streams unchanged: the *only* thing this wrapper touches is the log.
    code = subprocess.run([binary, *argv]).returncode
    log = events_path(argv)
    if log is not None and log.is_file():
        mutate(log, which)
    return code


if __name__ == "__main__":
    sys.exit(main())
