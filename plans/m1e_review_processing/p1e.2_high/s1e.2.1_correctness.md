# S1e.2.1 — Correctness: the panic, the guard, the traversal

**Phase:** [P1e.2](README.md) (High)
**Estimate:** 5 days
**Depends on:** [Q6](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md)
for T3(c); [T1e.1.6.2](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md)'s
sweep for T1's class scope.
**Findings:** [`CO-H1`](../review/correctness/high.md),
[`CO-H2`](../review/correctness/high.md),
[`CO-H3`](../review/correctness/high.md).

## Context

Three defects that share a shape: **the engine's contract with its caller is
broken in a way the caller cannot see coming.** None produces a wrong answer
on a corpus program today. All three are the kind of thing that is discovered
by the person who hits it, not by the person who wrote it.

`CO-H1` and `CO-H2` were **reproduced against the release binary** during the
review — the only two findings in sixty-three with that status — so their
disposition is `fixed` or nothing, and the work is the fix, the fixture, and
the class.

`CO-H3` is three defects in the M1d tree traversal, with different
confidences and different fixes:

| | defect | confidence | fix is |
|---|---|---|---|
| (a) | `stop_after` and `max_set_size` silently ignored | **High** — the code path was read end to end | mechanical, and right under any answer to `T1d.10.6.4` |
| (b) | dead branches emit no no-good, no writeback, no `lstate.dead` push → `Contradiction` with an **empty** unsat core, `refuted so far (0 facts)`, zero nogood counters | **High** — same | narrow: either learn, or refuse to print evidence that does not exist |
| (c) | the rung mode is probed **once at root**; a flip at an inner node falls through to the blind enumerator, whose branches are not jointly exhaustive → **silently missing models** | Medium — a fragility argument, not an observed failure | decided by [Q6](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) |

(b) is the one worth being precise about. The traversal is opt-in and
explicitly experimental, which bounds exposure — and it is also the headline
M1d result, **86 enterings against 17 204 592**. A verdict of `Contradiction`
whose stated evidence is empty is not an incomplete read-out; it is a false
one, and it is the same defect S1d.3.3 fixed for `Ambiguity` when it taught a
`k` to say *(a lower bound — the search did not exhaust)* rather than print a
bare number.

## Acceptance

- `(eq ?x)` — and every other built-in arity malformation — is refused at
  **compile time** with a positioned diagnostic, like every sibling
  malformation already is; the runtime `assert!` path is removed or
  unreachable by construction; a `broken/compile/` fixture with its
  `.expected` pins the message.
- `ein-ir` and `ein-core` share **one** reserved-name constant; a test
  asserts it; three fixtures pin the guard on the three declaration routes
  for at least `open` and one control name (`absent`) that already worked.
- The tree honours `-n` and `-m`, or refuses them with a stated reason; the
  `Contradiction` arm under tree mode does not print an unsat core it has not
  got; the rung premise is enforced or argued at `solve.rs:889` per Q6.
- `EIN_TRAVERSAL=tree ein solve examples/zebra2-minus-15-obligations.ein`
  still reaches the same 32 models in **86 enterings** — the S1d.10.6 result
  is a regression target for this stage, and any change to it is a finding.
- `./run_tests.sh` green; `cargo bench` smoke unmoved.

## Tasks

### Task T1e.2.1.1 — `CO-H1`: arity, at compile time

The lowering at
[`compile.rs:594-608`](../../../ein.rs/crates/ein-infer/src/compile.rs)
emits `eq`/`neq` guards with no arity check, and the check that exists is a
runtime `assert!` at
[`match_.rs:776-789`](../../../ein.rs/crates/ein-infer/src/match_.rs) whose
message — *"`eq` needs two arguments; ein.py raises IndexError here"* — is a
parity note, not a diagnostic. Every sibling malformation (nested `or`, empty
`absent`, arity mismatch on a declared relation) is already refused at
load/compile with a `file:line:col`.

The fix:

1. **Check where the siblings are checked**, so the diagnostic comes out in
   the same form and the same place. A `CompileError` — the type the sibling
   refusals already use.
2. **Message** in `defined_behaviour.md` § 4's vocabulary. The page's error
   table currently maps ein.py's `IndexError` here to **nothing**, so the
   table gains a row and the row is normative: this is a new defined
   behaviour, and it should read like the ones beside it rather than like a
   Rust panic rendered politely.
3. **Fixture** under `examples/broken/compile/` with its `.expected`, plus
   the `corpus.toml` entry — a `.ein` with no entry fails the completeness
   check, so the fixture is not optional bookkeeping.
4. **Remove the `assert!`** or make it unreachable by construction. Leaving
   it as a belt is defensible; leaving it as the *only* check is what the
   finding is.
5. **The class, not the instance.**
   [T1e.1.6.2](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md)'s sweep
   enumerates every built-in and structural form at wrong arity; each cell it
   found wrong gets a fixture here. The `.expected` files are cheap and they
   are the only thing that makes *"no well-formed program panics"* a checked
   claim rather than a hope.

Note the second-order defect this closes:
`corpus_cli::every_refusal_carries_a_diagnostic` is a rule the repo enforces
on refusals, and a panic is a refusal that carries a stack trace instead.

### Task T1e.2.1.2 — `CO-H2`: one reserved-name list

Two hand-maintained copies of one semantic list drifted when M1d added `open`
to one of them:

- [`imports.rs:49`](../../../ein.rs/crates/ein-ir/src/imports.rs) —
  `RESERVED_NAMES: [&str; 8]`, no `open`;
- [`terms.rs:191`](../../../ein.rs/crates/ein-core/src/terms.rs) —
  `RESERVED: [&str; 9]`, with `open` since M1d S1d.2.3.

Consequence, verified: `(macro open …)` in a module imported plain or
qualified is **silently renamed** to `mod.open` and loads with exit 0; the
same declaration imported flat via `:symbols (open)`, or written directly, is
rejected. `qualify()`'s own doc comment
([`imports.rs:548-564`](../../../ein.rs/crates/ein-ir/src/imports.rs)) states
the opposite intent — a module illegally defining a reserved name must *keep*
it so the loader rejects it — and that intent holds for `absent`, which is in
both lists.

The fix, in the order that keeps it honest:

1. **Reproduce**, since a fixture is wanted anyway: two module files, one
   `(macro open …)`, imported three ways. Record all three behaviours before
   changing anything.
2. **Unify.** `ein-ir` depends on `ein-core`, so `imports.rs` consumes
   `terms::RESERVED` directly and its own array goes. If some name genuinely
   must differ between the two uses — the review does not think so, but the
   lexer's set genuinely does differ
   ([SE-L2](../p1e.4_low/s1e.4.2_semantics.md)) — then it is a second named
   constant with a comment saying why, not a second copy of the same idea.
3. **Test that they are one.** Trivial with a shared constant; keep it anyway,
   because it is the thing that fails if someone re-forks the list.
4. **Fixture per route.** Three files: direct declaration, `:symbols (open)`,
   qualified `(import mod)`. Each expects the same refusal. This is what makes
   the guard *pinned per route*, which is what it was not.
5. **Delete the comment that predicted the fix.** `imports.rs:42-48` explains
   why the duplication is temporary and names the phase that would end it
   (P1a.3). It never did. Leaving it there after unifying would be a third
   false statement about the same eight lines
   ([MA-L5](../p1e.4_low/s1e.4.8_maintainability.md) is that finding).

Check as you go whether any *other* declarator route exists — the finding
names three, and the loader is where a fourth would hide.

### Task T1e.2.1.3 — `CO-H3`: the traversal's three defects

**(a) Honour the stop policy.** `tree_node`
([`solve.rs:934-1037`](../../../ein.rs/crates/ein-infer/src/solve.rs)) checks
only `check_budget` at `:958`; nothing consults `opts.stop_after` after
`record_node` at `:1030`, and nothing caps depth by `max_set_size`. Since
`ein solve` defaults to `-n 1`, `EIN_TRAVERSAL=tree ein solve file` explores
and records the **entire** tree while being asked for one model.

Fix: return after `k` recorded models with `truncated` already set — it is set
unconditionally at the entry (`solve.rs:915`, *not exhaustion — discharge*),
so the reporting is already right for an early return. For `max_set_size`,
decide between honouring it as a depth cap and refusing it with a message;
the lattice honours `-m 0` as a truncated no-op, and a traversal that
silently ignores a flag the other traversal honours is the *worst* of the
three options. Whatever is chosen goes in the `EIN_TRAVERSAL` block of
`CLAUDE.md`/`README.md` and in the events documentation
([CD-M2](../p1e.3_medium/s1e.3.7_code_doc_consistency.md) is the page).

**(b) Stop printing evidence that does not exist.** The non-`Alive` arm at
`:991-1013` bumps counters and calls `dumper.entering`, then `continue`s —
no `emit_nogood`, no `(not h)` writeback, no `lstate.dead` push. Compare the
lattice's `handle_dead` at `:2253-2257`, which does the first two. So
`finalise`'s `Contradiction` arm unions over an **empty** dead list
(`:2389-2398`) and the table prints *refuted so far (0 facts)*, a `--trace`
proof has empty `dead_commitments`/`learned_nogoods`, and the nogood counters
read 0.

Two candidate fixes, and they are not equivalent:

| fix | what it costs | what it buys |
|---|---|---|
| **learn** — emit the no-good and the writeback on tree deaths, as the lattice does | changes the traversal's search behaviour and therefore its **86 enterings**; that number is a published result and moving it is a measurement, not a side effect | a real core, real counters, one code path instead of two |
| **refuse** — leave the search alone; make the `Contradiction` read-out under tree mode decline to print a core, saying instead that this traversal records none | nothing measurable | the surface stops lying; the search stays the thing that was measured |

**Recommended: refuse**, this milestone. Learning is a search change inside a
traversal whose reporting contract is an open M1d question, and this milestone
is not the place to move a headline number. The refusal is narrow, is right
under either eventual answer, and is the exact move S1d.3.3 made for
`Ambiguity`. Record the choice and the reason where the arm is, and note in
[`open_questions.md`](../open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface)
that learning-on-tree-deaths remains available and unpriced.

**(c) The rung premise — and it is now a ruling, not a choice.** `tree()`
probes the mode once at root (`:889-914`) on the premise that the mode is a
property of the program. **Decided 2026-08-28 by the user: the mode is re-read
at every node.** So this sub-task no longer waits on Q6 and no longer chooses
between a `debug_assert` and a re-probe — it applies the ruling.

Two things that changed with it:

- **The cost argument is void.** A re-probe is *not* a per-node generation
  call: `tree_node` already builds a `HypGenStats`, calls
  `generate_one_branch`, keeps the candidate list and **drops `hs.rung.mode`**
  (`solve.rs:945-956`). The change is to stop discarding the value.
- **The probe arrives later, in another phase.** Q6's construction moved to
  [S1e.1b.6](../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md),
  which runs after this one. So record here that this guard's **regression test
  is owed** and name that stage — a guard shipped without a probe is a guard
  nobody can remove
  ([Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)),
  and the difference that makes it acceptable is that this is applying a ruling
  rather than closing a risk by argument.

What the guard *does* on a flip — decline the traversal, or re-derive the
branch and continue — is **not** this task's: it is
[Q-M1e.11](../open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis),
ruled on by S1e.1b.6 T4. Until then the conservative arm is the one to write:
narrate the flip and decline, which is what the root probe already does for
every other rung.

The one thing the guard *does* enforce — declining on any rung other than
obligations, with a `traversal` event — is real and pinned by
`tests/tree_traversal.rs`. Leave it alone; it is the part that works.

### Task T1e.2.1.4 — Re-measure the headline

After (a) and (b): re-run
`EIN_TRAVERSAL=tree ein solve examples/zebra2-minus-15-obligations.ein -e`
and confirm **32 models in 86 enterings**, verified fact for fact against the
lattice as S1d.10.6 did. (a) changes what happens *after* the models are
found under a stop policy, and (b) by the recommended fix changes nothing in
the search at all, so the number should be untouched — which is precisely why
it is worth confirming rather than assuming. If it moved, something in (a)
was wider than intended.

Record the re-take beside the S1d.10.6 result rather than only in this file:
a published number re-confirmed at a later commit is worth more than a
published number.

## Notes

The `tests/tree_traversal.rs:75-77` comment is currently the **only** place in
the tree that states the `-n`/`-m` behaviour. When (a) lands, that comment
becomes wrong, and it is the kind of wrong that reads as right — a test
comment describing an intentional limitation. Update it in the same commit.
