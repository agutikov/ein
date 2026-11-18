# S1f.23.10 — the user's answers on the language and three decisions' sentences, built

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-19 on the user's word (*"answered plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/ANSWERS.md - process"*),
in a worktree; merged by the main session, and one `utils/ci_container.sh` and one nightly hold it with R-42's fix.
**The answers:** the user's file as written, `b0e9c412`
(`git show b0e9c412:plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/ANSWERS.md`), and the
removed items' pages beside it in that commit (`…/04-language-and-decisions/NN-…/README.md`), with their programs.

## The answers

| item | the question | the user's answer |
|---|---|---|
| V-7 | a rule can conclude a fact of a relation the program does not count | *"What does mean "program does not count relation" ? relation is almost any atom if it is 1st in the tuple. Also for nested facts. Its enough to be the first item in the tuple to became a relation. (XXX) - is a relation fact with relation 'XXX'. Relation declaration `(relation = Formula Formula)` is optional fact that can be used to handle some random labels associated with the relation, even maybe not arguments types, not arguments count. It is just by convention is "relation declaration", that is used to reason about arguments domains. You easily can use `(rel-arg-types <relation_name> Type Type)`, `(rel-arg-types <relation_name> (list Type Type))`, `(defrel <relation_name> Type Type)` WHATEVER for the same purpose. But `relation` is already used by SOME OF stdlib modules. That is it."* |
| F21 N-15 | *everybody's thoughts are false* with `absent` comes out dead | F21's; filed there by the main session, nothing built here |
| Q-M1f.412 | the `is-a` row dropped: does a schema id move | a question, answered by the main session (the kernel reads `is-a` nowhere); the default (a) stands; one stale comment, task 1 |
| L-92 | Q-M1f.362's sentence on the fact index | *"(a)"* |
| L-93 | two stale sentences of the pages (a third found) | *"both, make docs consistent with the code and clear"* |
| L-94 | Q-M1f.119 misses a fact's argument | *"(a)"* |

## What it builds

### Task 1 — the sentences (L-92, L-93, L-94, and one comment of Q-M1f.412's)

Documentation and comments only; no answer moves. The text each item proposed is on its page in `b0e9c412`.

1. **L-92 (a)**: [Q-M1f.362](../../../docs/decisions/stdlib/math/Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)'s
   **Why** re-worded to say both halves, the item's § 3 text, with one correction the main session found: zero is the
   name `z`, so the text says *a number other than `z`* (or *a numeral `(n …)`*) is a fact, not *a number is a fact*.
   With it, the item's § 7: `ein-core/src/kb.rs`' comment in `Kb::index_fact` (*"Only one level: the corpus's
   nesting is `(not (R …))` …"*) says the one level serves a stored negation and a context, as
   `03-data-model.md` § 6.1 does; `SlotKey`'s doc comment links § 6.1, the participation key, not § 5.
2. **L-93, both, *"consistent with the code and clear"***: the diagnostics page gains the item's **§ 1.7** (its
   option (b): the normative page of refusals states the parser's nesting limit, citing Q-M1f.185 and
   `generated_parser.rs`' `the_nesting_limit`), and `04-stdlib/08-natural-numbers.md` § How long a number may be
   links it; that paragraph's third sentence re-worded (the item's § 3.1: a saved state loads again). The
   reserved-strings page's `#nested` row re-worded (the item's § 3.2: the parse refuses every such form, so no program
   reaches it), and `NESTED_HEAD`'s doc comment (`ein-core/src/text.rs`) with it, which says the same stale thing.
   The constant stays (no code change).
3. **L-94 (a)**: [Q-M1f.119](../../../docs/decisions/language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)
   names *"writes as the relation of a fact that is an argument"* (the item's § 3.1, today's text with that clause
   added). With it, under L-93's *"make docs consistent with the code and clear"*, the item's § 3.3: `12-query.md` and
   `02-syntax.md` § Query list *"passed by an activator where a relation's name is held"*. **F23's D-23, the
   refusal's line, is not touched**: a code message, F23's, not ruled. If task 2 changes what a relation of the
   program is, these texts are written to task 2's notion, once.
4. **Q-M1f.412's comment**: `ein-ir/src/from_ir.rs` above `program_relations` (line 664) still lists `is-a` among a
   program's relations; Q-M1f.436 took it out. The comment says what the function does.

### Task 2 — V-7: a relation is any atom first in a tuple

**The ruling** (filed as **Q-M1f.522**, the next free id, a decision file of its own under `docs/decisions/language/`):
an atom is a relation where it is the first item of a tuple the program writes, at any depth: a stated fact, a
fact nested in another, a rule's, an hrule's or an obligation's pattern, a constant head or one an activator binds
for a parameter that heads a conclusion. A declaration, `(relation R …)`, is an optional fact, a convention some
stdlib modules use to reason about a relation's arguments; nothing makes a relation of the program depend on it.

**Built so:**

- **One notion, one function.** The engine's lists of the program's relations agree: the registry
  (`ein_core::Program::relations`, which decides what the blind rung takes for an object), the alive-set invariant's
  baseline (`invariant.rs`' `Universe::of_facts`), the goal's and claim's list (`from_ir.rs`' `program_relations`) and
  the reasoning's $\operatorname{rels}(P)$ ([`02-saturation.md`](../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md)).
  Where they can be one computation they are; where one must stay apart, the stage says why.
- **"Almost any atom"**: the forms the kernel reads as forms today (`and`, `or`, `absent`, `eq`, `neq`, the macro
  calls, and whatever else `ein_core::terms::Kernel` holds) are not made relations by heading a tuple; the stage lists
  them and takes that as a reading with a default. P1f.21's direction (`false`, `not` and `relation` stdlib
  conventions) is P1f.21's and not built here.
- **What moves**: R moves where the blind rung guessed about such a name as an object (the item's `v7_blind.ein`, 8
  states today, 1 under the ruling). **A census before the build**: every corpus program's R and H by today's binary
  and by the stage's, each program that moves named with both answers; an `:expect` that states the old answer is
  brought to the ruling, as S1f.23.9 did. If a move is not the ruling's (an answer moves for another reason), stop
  and report it.
- **What does not move**: the argument-count check (Q-M1f.447, open, P1f.21's; the user's *"not arguments count"* is
  recorded there and decides nothing here), the declarations' own refusals, every answer the blind rung does not
  reach.
- **Pages**: the knowledge-base page and the user pages that say what a relation of the program is (the goal's
  refusal, `12-query.md`, `02-syntax.md`, `07-diagnostics.md` § 2.20 and § 2.21, Q-M1f.114 and Q-M1f.119 under task
  1), the read-out's `relation names` row ([`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md))
  where its words move, and the saturation page's $\operatorname{rels}(P)$.

## Acceptance

- Task 1: `utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`,
  `utils/check_release_voice.py`, `cargo doc` with `-D warnings`; no golden moves.
- Task 2: the census, every moved program named; `alive_invariant.rs`, `goal_premises.rs`, `solve_lists.rs`,
  `region.rs`, `explore.rs`, `corpus_cli`, `renaming_invariance.rs`, `stdlib_coverage.rs`; the shape golden re-banked
  (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`), the kinds that move named.
- The full gate is the main session's, once on the merged tree: `utils/ci_container.sh` and the nightly.

## Record

Not started.
