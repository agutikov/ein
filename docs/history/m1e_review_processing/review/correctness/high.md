# Correctness — High

## A one-argument built-in guard panics the process at match time

**Severity:** High
**Confidence:** High (reproduced against the release binary)
**Topic:** Correctness
**Classification:** code bug

**Locations**
- `ein.rs/crates/ein-infer/src/match_.rs:776-789`
- `ein.rs/crates/ein-infer/src/compile.rs:594-608`

### Finding

`(rule r () :match (and (a ?x) (eq ?x)) :assert …)` loads and compiles cleanly; the arity error is only discovered by a runtime `assert!` inside the matcher, so `ein saturate` / `ein solve` dies with a panic rather than a diagnostic:

```
thread 'main' panicked at crates/ein-infer/src/match_.rs:778:
`eq` needs two arguments; ein.py raises IndexError here
```

### Evidence

The compiler lowers `eq`/`neq` without any arity check (`compile.rs:594-608`), deferring to the `assert!` at `match_.rs:776-789`. Reproduced on `ein.rs/target/release/ein` with a two-line probe program. Every sibling malformation (nested `or`, empty `absent`, arity mismatch on a declared relation) is refused at load/compile with a positioned diagnostic.

### Impact

A crash from well-formed surface input. It bypasses the repo's own discipline that every refusal carries a stderr diagnostic (`corpus_cli::every_refusal_carries_a_diagnostic`), and the `defined_behaviour.md` §4 error table maps ein.py's `IndexError` here to nothing. Any generated or hand-written program with a malformed guard takes down the process instead of getting a `file:line:col` error.

### Recommendation

Check built-in predicate arity at compile time (a `CompileError`, where the sibling malformations are already refused), add a negative fixture under `examples/broken/` so the corpus pins the diagnostic, and remove the runtime `assert!` path or make it unreachable-by-construction.

### Cross-references

- `review/tests/medium.md` — no negative fixture exists for this shape.
- `docs/kernel/defined_behaviour.md` §4 (the error table this falls outside of).

---

## The reserved-name guard is bypassed by import qualification: `(macro open …)` loads silently

**Severity:** High
**Confidence:** High (reproduced against the release binary)
**Topic:** Correctness
**Classification:** code bug

**Locations**
- `ein.rs/crates/ein-ir/src/imports.rs:49-51` (RESERVED_NAMES — 8 names, no `open`)
- `ein.rs/crates/ein-ir/src/imports.rs:548-564` (`qualify()` and its stated intent)
- `ein.rs/crates/ein-core/src/terms.rs:184-193` (RESERVED — 9 names, with `open` since M1d S1d.2.3)

### Finding

Two hand-maintained copies of the reserved-name list drifted when M1d added the verdict atom `open` to only one of them. Consequence, verified by experiment: a module declaring `(macro open …)` imported via plain/qualified `(import mod)` is **silently renamed** to `mod.open` and the program loads with exit 0, while the same declaration imported flat via `:symbols (open)` — or written directly in the file — is rejected with "macro 'open' shadows a reserved kernel name".

### Evidence

`qualify()`'s own doc comment (imports.rs:548-550) states the opposite intent: a module illegally defining a reserved name must *keep* it so the loader rejects it, never be silently renamed. That holds for `absent` (which is in both lists) but not for `open`. The comment at imports.rs:42-44 ("`open`/`forall` are deliberately not here — they migrated into std.macro") describes the pre-M1d world — the macro is now `unknown` (`stdlib/macro.ein:23`) and `open` is the reserved verdict atom. imports.rs:46-48 even predicts the fix that never happened ("P1a.3 brings the registries over and this becomes a query against them").

### Impact

Same name, different guard, depending on the import tier — exactly the class of silent inconsistency the reserved-name check exists to prevent. The blast radius is bounded today (the renamed macro is inert under its qualified name), but the guard's contract — a declarator may not bind a RESERVED name — is not actually enforced through one of the three declaration routes, and the mechanism (two hand-maintained copies of one semantic list) will drift again the next time the list grows.

### Recommendation

Make imports.rs consume `ein-core`'s RESERVED (or a single shared constant) instead of carrying its own list; add a test asserting the two lists are one; add a fixture pair (direct declaration vs qualified import of each reserved name) so the guard is pinned per route.

### Cross-references

- `review/architecture/medium.md` — hand-maintained parallel lists as the recurring drift mechanism.
- `review/state-model/medium.md` — the same "stated but unenforced" pattern for the alive-set invariant.

---

## The tree traversal (EIN_TRAVERSAL=tree) ignores the stop policy, learns nothing from dead branches, and rests on a root-only mode probe

**Severity:** High
**Confidence:** High for (a) and (b) (decisive code paths read end-to-end); Medium for (c) (fragility argument, not an observed failure)
**Topic:** Correctness
**Classification:** code bug (a, b); design ambiguity (c)

**Locations**
- `ein.rs/crates/ein-infer/src/solve.rs:934-1037` (`tree_node`)
- `ein.rs/crates/ein-infer/src/solve.rs:991-1013` (the dead-branch arm)
- `ein.rs/crates/ein-infer/src/solve.rs:889-914` (the root-only mode probe)
- `ein.rs/crates/ein-infer/src/hypgen.rs:340-378`, `ein.rs/crates/ein-infer/src/oblgen.rs:241-265`

### Finding

Three related defects in the M1d S1d.10.6 tree traversal:

**(a) The stop policy is silently ignored.** `tree_node` checks only `check_budget` (max_enterings / max_time, solve.rs:958). Nothing consults `opts.stop_after` after `record_node` (:1030) and nothing caps depth by `max_set_size`. `ein solve` defaults to `-n 1`, so `EIN_TRAVERSAL=tree ein solve file` explores and records the **entire** tree despite asking for one model; `-m 0`, which the lattice honours as a truncated no-op (:1152-1159), is also ignored. The README's contract — "the only choice is the stop policy" — is false under this traversal, and the depth-unbounded behaviour is mentioned only in a test comment (`tests/tree_traversal.rs:75-77`), not in any doc.

**(b) Dead branches learn nothing and record nothing.** The non-Alive arm only bumps counters and calls `dumper.entering` (solve.rs:991-1013): no `emit_nogood`, no writeback, no `lstate.dead` push. A tree run that finds zero models therefore returns **Contradiction with an empty unsat core** (finalise's Contradiction arm unions over an empty dead list, :2389-2398); the table prints "refuted so far (0 facts)", a `--trace` proof has empty `dead_commitments`/`learned_nogoods`, and nogood counters read 0. None of this is stated where the lattice's contract is.

**(c) "Asking once is asking enough" is argued, not enforced.** `tree()` probes the generation-ladder mode once at root on the premise the mode is a property of the program (solve.rs:889-893). But oblgen's mode per node depends on activator **facts**, which can be derived inside a fork; a mode flip at an inner node falls through to the **blind enumerator** (hypgen.rs:340-378), whose branches are *not* jointly exhaustive — the tree would then treat a non-exhaustive branch set as exhaustive and **miss models**. No `debug_assert` re-checks the mode at inner nodes. Today's stdlib activators are root-asserted, so the corpus never hits it — a fragility, not an observed bug.

### Evidence

Code paths cited above; the enforcement absence in (b) is visible by diffing the tree's dead arm against the lattice's `handle_dead` (solve.rs:2253-2257, which emits the nogood and the writeback). The one thing the guard *does* enforce — declining on any rung other than obligations, with a `traversal` event — is real (solve.rs:894-914) and pinned by `tests/tree_traversal.rs`.

### Impact

The traversal is opt-in and explicitly experimental (T1d.10.6.4 open), which bounds exposure — but it is also the headline M1d result (86 enterings vs 17 204 592), and in its current state it: contradicts the CLI's stated stop-policy semantics; produces a Contradiction verdict whose stated evidence (the core) is empty; and has an unenforced soundness premise whose failure mode is silently missing models — the one failure class the project's own discipline treats as worst.

### Recommendation

Honour `stop_after` in `tree_node` (return after k recorded models with `truncated` already true); either learn no-goods on tree deaths or make `finalise` refuse to print "refuted so far" from an empty dead list under tree mode; re-probe (or `debug_assert`) the rung mode at every node and hard-decline on a flip; document the `-n`/`-m` interaction wherever `EIN_TRAVERSAL` is documented.

### Cross-references

- `review/semantics/medium.md` — tree-mode reporting semantics under-specified.
- `review/code-doc-consistency/medium.md` — the `traversal` event is absent from `events.md`.
