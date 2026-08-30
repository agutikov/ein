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
  *(Four routes: `:as` is a fourth and was equally broken.)*
- The tree honours `-n` and `-m`, or refuses them with a stated reason; the
  `Contradiction` arm under tree mode does not print an unsat core it has not
  got; the rung premise is enforced or argued at `solve.rs:889` per Q6.
- `EIN_TRAVERSAL=tree ein solve examples/zebra2-minus-15-obligations.ein`
  still reaches the same 32 models in **86 enterings** — the S1d.10.6 result
  is a regression target for this stage, and any change to it is a finding.
  *(Read with `-e`, as T4 writes it. Without it the run is at `-n 1`, and
  answering 32 there is the defect (a) exists to fix — this bullet and its own
  task contradict each other, and the task is the one that is right.)*
- `./run_tests.sh` green; `cargo bench` smoke unmoved.

## Tasks

### Task T1e.2.1.1 — `CO-H1`: arity, at compile time ✅

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

### Task T1e.2.1.2 — `CO-H2`: one reserved-name list ✅

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

### Task T1e.2.1.3 — `CO-H3`: the traversal's three defects ✅

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
  [S1f.10.6](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md),
  which runs after this one. So record here that this guard's **regression test
  is owed** and name that stage — a guard shipped without a probe is a guard
  nobody can remove
  ([Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)),
  and the difference that makes it acceptable is that this is applying a ruling
  rather than closing a risk by argument.

What the guard *does* on a flip — decline the traversal, or re-derive the
branch and continue — is **not** this task's: it is
[Q-M1e.11](../open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis),
ruled on by S1f.10.6 T4. Until then the conservative arm is the one to write:
narrate the flip and decline, which is what the root probe already does for
every other rung.

The one thing the guard *does* enforce — declining on any rung other than
obligations, with a `traversal` event — is real and pinned by
`tests/tree_traversal.rs`. Leave it alone; it is the part that works.

### Task T1e.2.1.4 — Re-measure the headline ✅

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

## What landed — 2026-08-29

| | |
|---|---|
| **`CO-H1`** | **fixed as the class, not the case.** `Compiler::premise` refuses `eq`/`neq` at any arity but 2 and `absent` at any but 1, with a `CompileError` that names the form *and its position*. All **seven** wrong cells of the S1e.1.6 sweep are now exit 1; the fourteen right ones are unmoved |
| **`CO-H2`** | **fixed, and wider than reported** — four declarators and **four** routes, `:as` being a fourth the finding did not name. One list (`ein_core::RESERVED`), `imports.rs`'s copy and `MA-L5`'s comment deleted |
| **`CO-H3`** | **all three fixed.** `-n` honoured, `-m` refused with a stated reason at exit 2, dead branches **recorded**, the rung re-read per node |
| **the headline** | **86 enterings, 32 models, identical fact for fact** to the lattice's 48 745 — re-measured and banked in [the M1d record](../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal) |
| answered | [Q-M1e.18](../open_questions.md#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) — candidate **(2)**, check the arity where the form is read |
| dispositioned | `CO-H1` · `CO-H2` · `CO-H3` **fixed**; `MA-L5` **fixed** here rather than in [S1e.4.8](../p1e.4_low/s1e.4.8_maintainability.md), which is what its *Depends on* line said |
| owed | `CO-H3`(c)'s regression test, to [S1f.10.6](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md) — recorded at the site, in the stage, and in [Q-M1e.11](../open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis) |
| goldens moved | two, both **additions only**: `corpus_exits.txt` (+17 cells, every one a new fixture at exit 1) and `corpus_shapes.md5` (+304 renderings, 0 modified — which is the phase's *"the measured cost should be zero"* checked rather than assumed) |
| new fixtures | 4 under `examples/broken/compile/`, 4 under `examples/broken/load/`, 8 corpus entries |
| new tests | `primitive_arity`'s 21 cells re-pinned + `a_refused_arity_names_its_position`; `imports_semantics`'s 32-cell matrix + the unreserved control + the all-names sweep; `tree_traversal`'s stop policy and dead-branch core; `cli_semantics`'s `-n`/`-m` pair |
| gate | `./run_tests.sh` green; `cargo bench --bench engine -- --test` unmoved |

### Five things the tasks did not predict

**1. The premise about "positioned diagnostics" was false.** `CO-H1` and this
stage both say every sibling malformation is refused *"at load/compile with a
`file:line:col`"*. Measured: a **parse** error is positioned, a **load** error
ends in `at None` (§1.5 of `defined_behaviour.md` says why — a top-level form
carries no `Loc`), and a **compile** error carries no position at all. So there
was nothing to be consistent with. The new messages are positioned anyway,
because a premise *is* a `generic_list` and that is the one production the
parser hands a `Loc` — which makes them the only refusals in the family that
can say where, and is why `examples/broken/compile/` gained a `{FILE}`
placeholder its README used to say it had no use for.

**2. `CO-H2` is four declarators and four routes, not one and three.** The
finding is written about `(macro open …)` through three routes. The 32-cell
matrix — `{rule, hrule, relation, macro}` × `{open, absent}` × `{direct,
:symbols, qualified, :as}` — says **eight** cells loaded with exit 0, not two:
every declarator, through both routes that go via `qualify()`. The stage's own
instruction to *"check as you go whether any other declarator route exists"* is
what found `:as`, and a fixture set covering only the three named routes would
have left it unpinned.

**3. (b) had a third option, and it is better than both.** The task's table
offers *learn* (moves the 86) and *refuse* (prints nothing), and recommends
refusing. But the lattice's `handle_dead` does **three** things and only two
touch the search: the learned clause and the `(not h)` writeback change what
happens next; pushing the commitment onto `lstate.dead` changes only what the
answer may say. **Recording without learning** costs the search nothing —
re-measured, 86 enterings — and makes the core *true* instead of absent. On the
smallest program that reaches the arm, the tree now prints the same two-fact
core the lattice does, where it printed `refuted so far (0 facts)`. The
counters stay honest at `emitted=0`, because nothing was learned. Refusing
would have thrown away evidence the run had in hand.

**4. `-m` had to be refused, and the reason is a number.** The task leaves the
choice open between honouring it as a depth cap and refusing it. The 32 models'
commitment sizes are **3:3, 4:9, 5:14, 6:6** — six of them past
`--max-set-size`'s default of **5** — so the obvious repair would have deleted
a fifth of the headline at stock settings, and the stage's own regression
target forbids that. Refusal is also the only arm that does not answer
`T1d.10.6.4`: mapping a lattice layer index onto a tree depth is the input side
of exactly the conflation `layers_explored` already carries on the output side.
Exit **2** with a stderr `error:`, the code and shape the `--json-summary`
one-path refusal beside it already uses, and only for an *explicit* `-m` —
`default_value("5")` is the lattice's default, not a statement about the tree,
so the question is `value_source` rather than `get_one`. It is refused in
**three** subcommands, not the one the finding names: `solve`, `test` and
`render lattice` all take the flag and all three solve, so all three meet the
traversal — and `ein test` is doubly wrong under it, since an expectation is a
claim about the *exhausted* answer and a tree reports `exhausted = false` by
construction.

**5. `tests/tree_traversal.rs` raced, and a third test made it fail.** Every
test in that file sets `EIN_TRAVERSAL` at its own top, and cargo runs a file's
tests as threads of **one** process — the file's own header explains the
per-file isolation and then relies on something it does not have inside the
file. Two tests with one slow arm each happened not to collide. Adding a third
failed on the first run, with `the lattice arm is not the known baseline / left:
32 right: 28` — a *model count* wrong because of an environment variable.
`solve_path` now takes the traversal as an argument and holds a `Mutex` across
the solve, so the variable is written and read under one lock.

*(Two comments that said `RESERVED` is **eight** names went with the list:
`ir_semantics.rs` still called `open` a stdlib macro and tested four reachable
names where there are five, and `kb_semantics.rs` listed the same four. Both
now say nine and five, and the first tests `open`. They were never a shipped
defect — they are `CO-H2`'s drift one layer out, in the tests that would have
caught it.)*

*(One repair that came with (a), because making `tree_traversal` **public**
published it: `resume_forks`'s doc comment sat above `tree_traversal` and
`resume_forks` had none, so rustdoc was about to tell readers that
`EIN_TRAVERSAL` is *"Does a fork resume root's saturation?"*. Moved to the
function it describes. Pre-existing, and the same class as `MA-L1..L5`.)*

### And two findings for other owners

- **`ein saturate` cannot load a file-relative import.** `saturate.rs` calls
  `ein_ir::load(…, None)` where `solve.rs` passes `path.parent()`, and its
  comment says the import *"resolves against the working directory here"* —
  it does not resolve at all: `(import mod)` is a load error under `saturate`
  and loads under `solve`. Found while reproducing `CO-H2`, which is why three
  of the four new `broken/load/` fixtures declare no `saturate` run. It is a
  code↔doc inconsistency plus a subcommand asymmetry, and belongs with
  [S1e.3.7](../p1e.3_medium/s1e.3.7_code_doc_consistency.md) rather than here.
- **The tree under-reports `Open` states.** On a two-person program where each
  of two owed instances has one alternative, the lattice records **2** open
  states and the tree **1**: the tree commits the first instance's only
  alternative, finds the state complete-but-owing, and never visits the
  sibling. The completeness argument in
  [`completeness.md`](../../../docs/history/m1d_satisfiability/completeness.md)
  is about *models* — every model extends one of a jointly-exhaustive
  instance's alternatives — and an open state is by construction not one. The
  read-out is not lying (`exhausted = false`, so the count is a lower bound),
  which is why it is not in `CO-H3`'s scope; but *what a tree reports where a
  lattice reports layers* now has a second concrete instance, and it is
  `T1d.10.6.4`'s.

## Notes

The `tests/tree_traversal.rs:75-77` comment is currently the **only** place in
the tree that states the `-n`/`-m` behaviour. When (a) lands, that comment
becomes wrong, and it is the kind of wrong that reads as right — a test
comment describing an intentional limitation. Update it in the same commit.

*(Done — and it needed the opposite edit to the one predicted. The comment says
the tree takes no cap; that is still true, and what changed is that it is now a
**decision** with a number behind it rather than an omission. The paragraph
beside it says so.)*
