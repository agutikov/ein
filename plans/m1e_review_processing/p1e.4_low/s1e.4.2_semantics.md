# S1e.4.2 — Semantics (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 1 day
**Depends on:** [AR-M1](../p1e.3_medium/s1e.3.4_architecture.md) — both
findings are instances of its pattern, and `SE-L1`'s fix is one of its four
pairs.
**Findings:** [`SE-L1`](../review/semantics/low.md),
[`SE-L2`](../review/semantics/low.md).

## Context

Two findings, both *the same name or the same event meaning two things*.

**`SE-L1` — two timeline emitters, two key orders.** `MonotonicDumper` emits
`layer / outcome / commitment / kind / firings / facts_merged /
unsat_core_size / nogood_*`
([`dump/state.rs:129-143`](../../../ein.rs/crates/ein-render/src/dump/state.rs));
`LatticeDumper` emits `layer / outcome / commitment / facts_merged / nogood_* /
kind / firings / unsat_core_size`
([`dump/lattice.rs:140-152`](../../../ein.rs/crates/ein-render/src/dump/lattice.rs)).
The JSON writer preserves insertion order **as a document property — its whole
reason to exist** — so `00_timeline.jsonl` records for the same conceptual
event differ in shape between the two dumpers, and nothing says why at either
site.

**`SE-L2` — two different sets both named `RESERVED`.**
[`lex.rs:128`](../../../ein.rs/crates/ein-ir/src/lex.rs) holds 11
SYMBOL-excluded lexer words;
[`terms.rs:191`](../../../ein.rs/crates/ein-core/src/terms.rs) holds 9
shadow-check names including `open`. Both docs are individually accurate —
[`00_ebnf.md:51-62`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) says
*eleven RESERVED words*,
[`defined_behaviour.md:326-332`](../../../docs/kernel/defined_behaviour.md)
says *`open` joined RESERVED* — and a reader of both concludes the lexer set
grew to twelve. It did not, and it must not: `(open ?R)` has to lex as a
SYMBOL.

Note the relationship between the two findings. `SE-L2` is the case where two
copies of a name are **legitimately** different sets, and `CO-H2` was the case
where they were illegitimately different copies of one set. The same surface
symptom, opposite fixes — which is why
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md)'s written rule has to cover
both, and why this stage is where the rule gets its counterexample.

## Acceptance

- One timeline key order, or two with the parity reason stated at both sites.
- No two constants in the tree named `RESERVED` for different sets; the docs'
  arithmetic reconstructs.
- If a golden pins the current key divergence, the unification is named as a
  deliberate re-bless before it happens.

## Tasks

### Task T1e.4.2.1 — `SE-L1`: one key order

First establish whether either order is pinned. `00_timeline.jsonl` is a
dumped artefact, and the JSON writer's insertion-order preservation is
deliberate — so a golden may encode today's divergence. If one does, the fix
is a named re-bless; if not, it is free.

Then extract a shared emitter, which is the fix
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md) wants — one function taking
the fields, used by both dumpers — rather than editing one to match the other,
which leaves two copies that merely agree today.

If a parity constraint genuinely requires the difference (the review does not
think one does, but the dumpers have different provenance), say so **at both
sites**, not at one.

### Task T1e.4.2.2 — `SE-L2`: rename one of the two

The review's suggestion is the right shape: `LEXER_KEYWORDS` versus
`SHADOW_GUARDED`, or equivalent, in **code and docs together**. The two sets
answer different questions — *what may not lex as a SYMBOL* and *what a
declarator may not bind* — and the names should say which.

Then fix the arithmetic in the two pages so a reader can reconstruct each set:
`00_ebnf.md`'s eleven are the lexer's, `defined_behaviour.md`'s nine are the
shadow guard's, and the sentence about `open` joining belongs only to the
second. Add the one line that prevents the confusion recurring: **`open` must
lex as a SYMBOL**, with the reason — `(open ?R)` is an ordinary fact head.

After [CO-H2](../p1e.2_high/s1e.2.1_correctness.md) there should be exactly
two such constants, not three. Confirm that as part of this task; it is the
cheapest possible check that the High fix landed completely.

## Notes

Both fixes are small and both are worth doing even if the phase is otherwise
dropped, because they are the two instances that make
[AR-M1](../p1e.3_medium/s1e.3.4_architecture.md)'s rule complete: one pair
that must be unified, one pair that must be renamed. A rule with only the
first kind of example teaches the wrong lesson.

---

## ✅ Done 2026-09-01 — one name became two, and the docs' twelve became eleven again

**`SE-L1` was closed on the way past**, by
[S1e.3.4](../p1e.3_medium/s1e.3.4_architecture.md) as `AR-M1`'s third pair —
one `Timeline::entering`, two goldens moved, 110 of 8 835 renderings, every one
by the key permutation and nothing else. Nothing was left here for it.

**`SE-L2`: fixed**, by renaming the **lexer's** constant.

### Which one moves, and why it is not arbitrary

`ein-ir`'s private `RESERVED` is now `LEXER_KEYWORDS`; `ein_core::RESERVED`
keeps the word. *Reserved* is the word the **language** uses for that set: the
loader's message is *shadows a reserved kernel name*, six fixtures under
`examples/broken/load/` are named for it, and a whole kernel page is
`06_reserved_names.md`. Renaming that side would have moved goldens, fixture
filenames, corpus paths and a page title to spare a private constant with two
references. The lexer's set is a *grammar* artifact — "the words `SYMBOL`
refuses" — and it says so now.

### What the confusion actually was

Both docs were individually accurate, which is what made it survive: a reader
of `00_ebnf.md`'s *eleven RESERVED words* plus `defined_behaviour.md`'s
*`open` joined RESERVED* concludes the lexer set grew to **twelve**. It did
not, and it must not — `(open ?R)` is an ordinary fact head and has to lex as
a `SYMBOL`.

| | question | membership | how it fails |
|---|---|---|---|
| `ein-ir::LEXER_KEYWORDS` | what may not **lex** as a `SYMBOL`? | 11 grammar words | a parse error, wherever the word appears |
| `ein_core::RESERVED` | what may a declarator not **bind**? | 9 kernel names, `open` among them | a load error, at the declaration |

Four names are in both (`and` `neq` `not` `or`) and neither set contains the
other.

### The three claims the rename is worth, in a test

`lex::tests::the_lexer_keywords_are_eleven_and_are_not_ein_cores_nine`: the
eleven verbatim — which is what keeps `00_ebnf.md`'s production honest, since
nothing parses that page — the four-name intersection, and that `open`,
`open-slot` and `relation` all lex as `SYMBOL`s while `rule` does not.

**The misreading was already unbuildable and nothing said so at either site.**
`imports_semantics::every_ein_core_reserved_name_is_unbindable_through_a_qualified_import`
asserts the parse-refused subset is exactly those four names, so adding `open`
to the lexer list fails a test in a different crate, phrased as a claim about
imports. That is the finding's real content: not *prevent a bug*, but *put the
reason where the reader is*.

### Five sites, and one of them was a false causal claim

| page | what it said | what it says |
|---|---|---|
| [`00_ebnf.md`](../../../docs/kernel/ir/03-ein-lang/00_ebnf.md) | production `RESERVED` | production `LEXER_KEYWORDS`, with the other set named in the comment |
| [`defined_behaviour.md` § 4.2](../../../docs/kernel/defined_behaviour.md) | *"`open` joined `RESERVED`"* | which `RESERVED`, and why `open` must stay out of the other |
| [`06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md) | the shadow list, with no mention of the lexer's | § *Two sets, and the one that is not this page's* — the table above, on the page a puzzle author actually reads |
| [`02-data-model/03_implementation.md`](../../../docs/kernel/ir/02-data-model/03_implementation.md) | *"`STRUCTURAL` / `RESERVED` / `PREDICATES` are here because **the lexer** needs them"* | **false** — `lex.rs` reads nothing from `ein-core` but its counters. The readers are the **loader** (`macros.rs`, `imports.rs`) and the engine |
| [`architecture.md` § The legitimate case](../../../docs/kernel/architecture.md) | prescribed this rename in the **future tense** | records it as taken, as `AR-M1`'s mirror image — the same symptom as the unified pairs, the opposite fix |

That last row is why the stage said both fixes were worth doing even if the
phase were dropped: `AR-M1`'s rule needs one pair that must be *unified* and
one that must be *renamed*, and a rule with only the first kind teaches the
wrong lesson.

### One claim of the review's not taken

The review's *"three copies total (imports.rs carries a third, drifted one)"*
is **spent**: `CO-H2` deleted it at S1e.2.1, `qualify()` calls
`ein_core::is_reserved`, and the confirmation this stage's T2 asked for as
*"the cheapest possible check that the High fix landed completely"* is that
`grep -rn RESERVED ein.rs/crates --include='*.rs'` now finds exactly two
arrays with two different names.

**Gate:** `cargo test --workspace` — **807 tests, 0 failures**. No golden
moved: the renamed constant is private, reaches no rendered output and no error
message.
