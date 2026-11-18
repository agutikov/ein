# S1f.23.5 — a `:why` reads the form it is written on, `{?n}` its n-th arrow

**Phase:** [P1f.23](README.md)
**Status:** planned 2026-11-18 on the user's word. **Task 1, the change shown on real programs, runs now and stops
for the user's look** (the user's standing rule: a syntax shown on real corpus programs, its output computed, before
it is built). Task 2, the build, runs on the user's word after that look, and after the phase's other stages are
merged (it touches the standard library, the examples and the tests they touch).

## The ruling

The user, 2026-11-18, verbatim:

> also in this p1f.23 phase - universal renderings of NL, :why "{1}" - {1} is the same as 1-arrow of this fact, {2}
> 2nd-arrow, etc... So (relation owns Person Pet :why "a {?3} {?2} a {?4}") produce "a Person owns a Pet"

and, asked how a fact of the relation then reads and which spelling a placeholder takes, the user's choices:

- **"Declaration only"**: a `:why` reads only the form it is written on. A fact of `owns`, `(owns Ann Dog)`, has no
  template from the declaration, and reads by the fallback (`owns(Ann, Dog)` or what the fallback ladder gives),
  unless something else gives it one.
- **"{?n}, renumbered"**: today's spelling `{?n}`, counted from the relation as 1, so every relation's `:why` of the
  standard library and the corpus is renumbered.

So: `(relation owns Person Pet :why "a {?3} {?2} a {?4}")` is the fact `(relation owns Person Pet)`, its arrows
`relation` (1), `owns` (2), `Person` (3), `Pet` (4), and reads *a Person owns a Pet*. Today the same `:why` is the
template of every fact of `owns`, `{?1}` its first argument
([`14-natural-language.md` § 1](../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md#1-the-four-texts)).

## Task 1 — the change on real programs (stops for the user)

In `s1f.23.5_examples.md` beside this file, with the programs it runs:

1. **What reads a `:why` today**, each surface with a line of it: the trace's sentence after a derived fact, the
   read-out's, `ein render`'s labels if any, `--json-summary`, the narration, `utils/nl_census.py`; and the fallback
   ladder (§ 3) as it is.
2. **Real programs, today and after**: `examples/zebra2.ein` (a step of the trace and a state's reading), a
   calculator program, a beliefs program (a fact under an arrow), a standard-library module's relations
   (`std.algebra`, `std.slots`): for each relation, its `:why` today, its `:why` renumbered, what the declaration
   reads as after, and what one fact of it read as today and reads as after (the fallback). The *after* is computed
   by hand from the rule and marked as such, since nothing is built.
3. **What the ruling leaves open**, each with a default: what a rule's, an hrule's and a macro's `:why` read (their
   forms are no facts; their variables `{?x}` as today); whether a fact may carry a `:why` of its own (*"unless
   something else gives it one"*); whether `:why` stays required on a relation; what `--no-require-why` becomes; the
   fallback's form; how a `{?n}` past the form's arrows prints; the renumbering as a generated, checked rewrite of
   every relation's `:why`.
4. **What the build would touch**: the code (`git grep -l 'why' -- ein.rs/crates/*/src`), the standard library's
   relations (counted), the corpus's, the tests (`natural_language_text.rs`, `natural_language_page.rs`, the trace's
   goldens), the pages (`14-natural-language.md`, the walkthrough, the read-out), the decisions (Q-M1f.58, Q-M1f.112,
   Q-M1f.159 and every one that states `{?1}` is a first argument).

## Task 2 — the build

Planned from task 1 and the user's look; not started without the user's word.

## Record

- Task 1 done, `11a113cf`, waiting for the user's look: [`s1f.23.5_examples.md`](s1f.23.5_examples.md).
