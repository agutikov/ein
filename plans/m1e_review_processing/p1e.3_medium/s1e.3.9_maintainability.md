# S1e.3.9 — Maintainability (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 2 days
**Depends on:** nothing.
**Findings:** [`MA-M1`](../review/maintainability/medium.md) …
[`MA-M4`](../review/maintainability/medium.md).

## Context

Four findings, all of the form *a reader of this code would be misled by
something written next to it*. Three are comments that state the opposite of
the shipping behaviour; one is a counter that reports zero for a thing the
engine demonstrably does.

None affects an answer. All four affect the next person to change that code,
which in this project is the whole point of the code being commented at all.

**`MA-M1` — `phase_2_done`.** Declared `false`
([`solve.rs:1160`](../../../ein.rs/crates/ein-infer/src/solve.rs)), tested by
two `break`s (`:1162`, `:1525`), never assigned `true` — grep confirms exactly
four sites — and the loop body ends with `let _ = &mut phase_2_done;` at
`:1566`, an explicit warning-suppressor for a variable that cannot change.
Both breaks are unreachable. It reads like the residue of a removed early
exit, and it misleads a reader of the solve loop's control flow into believing
there is a second termination path.

**`MA-M2` — two rustdoc comments that state the opposite of the code.**
[`commitment.rs:109-116`](../../../ein.rs/crates/ein-infer/src/commitment.rs)
claims `resume` is never `Some` on shipping paths; the default path passes
root's snapshot at four call sites
([`solve.rs:790-799`](../../../ein.rs/crates/ein-infer/src/solve.rs)).
[`solve.rs:193-198`](../../../ein.rs/crates/ein-infer/src/solve.rs) claims
`--dump-states` sets `store_lattice`; only `--trace` does
([`ein-cli/src/solve.rs:584`](../../../ein.rs/crates/ein-cli/src/solve.rs)) —
`--dump-states` builds a `MonotonicDumper` without a proof.

**`MA-M3` — `state_key_merges`.** Declared, zeroed, copied into the proof,
serialised to JSON — and **never incremented**
(`solve.rs:156, 501, 627, 2468` against `dump/lattice.rs:351`), while
`record_node`'s replacement path at `:2198-2208` *is* a state-key merge, with
a comment nearby recording *"calls this 1 221 times to keep 22 nodes"* on
`branching/06`. `lattice_semantics.rs:27-29` frames the zero as deliberate
port scope — which is a reason for the counter to be absent, not a reason for
it to be present and lying.

**`MA-M4` — numeric drift in load-bearing comments.** 119/146 against 92/121
fixpoint-entry counts; *eleven* against *twelve* entries moved to `Open`;
126/39 against 123/38 `zebra2-bad` witnesses. The comments disagree with each
other and with the censuses
(`solve.rs:739-741, 2494-2500`, `verdict.rs:52-53`,
`ein-render/src/answer.rs:558-560`, `expect.rs:264-265` against
`explain.rs:544-548`), and a reader cross-checking cannot tell which snapshot
is current.

## Acceptance

- No comment in the four cited files states the opposite of the code beside
  it.
- `phase_2_done` is gone, or the early exit it was for is restored **with a
  test** — an unreachable `break` restored without one would be the same
  finding again.
- `state_key_merges` either counts, or is not in the emitted proof. A field in
  `proof_summary.json` that always reads 0 while the engine merges is an
  invitation to a wrong conclusion from a machine-readable artefact.
- Every number remaining in an in-code comment either **cites the census that
  owns it** or carries a date.

## Tasks

### Task T1e.3.9.1 — `MA-M1`: delete `phase_2_done`, or restore what it was for

Establish first, from git history, what the variable was for: the removed
early exit's shape and why it went. Two outcomes:

- **It was removed deliberately** (the exit was wrong, or superseded by the
  budget check): delete the variable, both `break`s and the suppressor, and
  leave a one-line comment at `:1160`'s site only if the *absence* of a second
  termination path is surprising — it probably is not, and a comment
  explaining that nothing is there is its own kind of noise.
- **It was removed accidentally** (the exit was real and its assignment was
  lost in a refactor): that is a behaviour finding, not a cleanup. Restore it
  with a test that reaches the exit, and note it in the milestone index as a
  finding upgraded from Maintainability.

The second is unlikely and worth ten minutes of `git log -S phase_2_done`
before assuming the first.

### Task T1e.3.9.2 — `MA-M2`: fix the two comments

Both are two-line edits, and both are the kind of comment a reviewer would
otherwise trust as the contract:

- `commitment.rs:109-116` — say what `resume` is actually for and name the
  four call sites that pass it. The comment is presumably a survivor of a
  time when the snapshot was not passed; S1a.6.9 is the change that made it
  wrong (the tree's own note at `solve.rs:985-990` says passing `None`
  *"was not a trade-off but a re-introduction of what S1a.6.9 removed"*), so
  the fix can cite that.
- `solve.rs:193-198` — `--trace` sets `store_lattice`; `--dump-states` does
  not. While there, check whether the *behaviour* is intended: a
  `MonotonicDumper` without a proof is a documented choice or an oversight,
  and the comment being wrong is a reason to ask. If it is intended, the
  comment says why.

### Task T1e.3.9.3 — `MA-M3`: count it, or drop it

Prefer **count it**: the increment goes where the replacement path is
(`:2198-2208`), it is one line, and the number is genuinely interesting — the
nearby comment's *1 221 merges to keep 22 nodes* is exactly the kind of fact
the proof summary exists to expose. Add it to whatever asserts the summary's
counter identities so it cannot silently return to zero.

If it is dropped instead, drop it from the emitted proof and from
`LatticeStats`, not just from the JSON — a struct field nobody sets is the
same finding one layer down. And update `lattice_semantics.rs:27-29`, which
currently documents the zero as intentional and would otherwise become the
next reader's evidence that it should be zero.

### Task T1e.3.9.4 — `MA-M4`: cite the census instead of inlining it

The rule, and it is [DO-M1](s1e.3.8_documentation.md)'s rule applied to code:
a number in a comment either **names the document that owns it** — the M1d
censuses own all four of these — or carries the date it was taken.

Go through the six cited sites. For each, decide which census owns the number
(`openness_census.md` for the fixpoint entries and the `Open` moves,
`explain.rs`'s own witnesses for `zebra2-bad`), and replace the inline value
with a citation, keeping the value only where a reader needs it *at the site*
to understand the code — in which case it gets a date.

The one that matters most is the 92/121 pair, because two comments disagree
and one of them is in `verdict.rs`, next to the code that decides a verdict
word. A reader checking whether the verdict logic matches the corpus reads
that comment first.

## Notes

Two days is generous for four small fixes, and the slack is deliberate:
T1e.3.9.1 could turn into a behaviour investigation, and T1e.3.9.3 could turn
into a question about what else in the proof summary is inert. If the second
happens, sweep the whole emitted proof for fields that are constant across the
corpus — that is a twenty-minute check with `--json-summary` over the manifest
and it would find any sibling of `state_key_merges` in one pass.
