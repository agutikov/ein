# S1f.6.30 — Name clashes between imported theories: investigation, proposals, and a stop

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. Not started. **An investigation that stops
for the user** before any change to the grammar or to what an import means; its first task, the tagged calls
as an interface, changes neither and is built.
**Depends on:** nothing in the phase. It is the last of the stages independent of the stdlib layout, and its
proposals are read before [S1f.6.31](s1f.6.31_the_stdlib_layout.md) moves the modules, since a proposal may
bear on how the moved modules name their internals.

## The ruling

[Question 8](questions.md#q8) asked whether to rename the theory's internal relations with a common prefix:
(a) no rename, a clash refused with the name in the message and `:as` for a program that wants one of the
names; or (b) a prefix on all of them, sixty relations renamed. The user took neither, 2026-11-12:

> usage of imported theories should be convenient, and allow mixing and connecting theories, if they have
> common things (like digits.ein) they are already connected, potential names clash is a gap of import
> mechanism, plan a stage for investigation, proposals and fix

So the clash is **the import mechanism's gap,** not the theory's to dodge by spelling. This stage is the
investigation and the proposals; the fix follows the user's choice among them.

**The tagged calls.** [Question 41](questions.md#q41), *are `std.arith`'s tagged calls a public interface*,
got *"I don't understand the question"*. [Round 2, item 41](questions_round2.md#q41) explains it and takes
**(a), yes, they are an interface,** meanwhile: *documented as what a theory built on the calculator calls,
and held by a test so they cannot change shape unnoticed*, and *the stage on import name clashes (your 8)
takes their names into account.* **The user's reply there, as it stood in the working copy when this plan was written (2026-11-12, not yet committed): *"(a)"*.**

## What is known

From [`malformed_numbers.md` § 6](malformed_numbers.md#6-the-names-the-theory-occupies-what-a-clash-does-today)
(S1f.6.12's T2, 2026-11-11), on the shipped theory:

- a second *declaration* of any name of the theory is refused at load, with the name in the message;
- a written *fact* of one of the theory's relations is not, and `(done (sum 2 2) 5)` is an answer nobody
  computed ([reply 7](questions.md#q7); the false-facts question is P1f.8's);
- `z` and the digits are names and are shared harmlessly;
- a qualified import prefixes the relations and not `z`.

From the phase's [`findings.md`](findings.md), the import mechanism's other rough places, each a neighbour:

- **E-8:** a qualified import brings the module's `(query …)`, `(solve …)` and `(config …)`
  ([S1f.6.25](s1f.6.25_the_section_j_fixes.md) fixes it);
- **E-20:** a flat import of a theory built on a theory leaves the lower theory's rules behind in silence
  (S1f.6.25 adds a line);
- **E-32:** a file-relative import cannot reach a sibling directory, so `examples/binary_logic/bits.ein` is
  a copy;
- and from S1f.6.11: a qualified import renames a fact's relation too, so a library of facts imported `:as`
  lands in `o.is-a` and does not join the program's.

The circuit's own relations have plain names, `done`, `b`, `sum`, `order`, `short` among them
(`questions.md` 8), and the tagged calls are `add-walk`, `mul-call`, `div-call`, `cmp-walk`, answered by
`done`, `div-done`, `order-is` (`questions.md` 41).

## Tasks

1. **T1f.6.30.1 — the tagged calls, an interface (built; no stop).** `std.arith`'s header and
   [`stdlib/README.md`](../../../stdlib/README.md) name the seven relations as what a theory built on the
   calculator calls, each with its shape and what answers it, apart from the circuit's own relations. A test
   in `arith_theory.rs` holds their declared shapes, and holds that every module of the stdlib and every
   example that calls one writes it in that shape, so a change of shape fails per commit. `std.int`'s and
   `std.rat`'s tagged calls (`int-norm`, `int-add-call`, `int-mul-call`, `int-cmp-call`, and `std.rat`'s)
   are listed the same way. Q-M1f.189 is marked answered, with the user's reply.
2. **T1f.6.30.2 — what mixing and connecting two theories does today, by probes on the release binary.**
   Each a real program with its output, in one table:
   - two theories imported flat that both import `std.digits`: the tables arrive once (S1f.6.11's diamond),
     the program's facts connect them; this is the user's *"already connected"*, shown;
   - two theories imported flat that each declare a relation of one name (`done`): the refusal, as worded;
   - the same with one of them `:as`: what is prefixed (its relations, its internal ones too), what is not
     (`z`, the digits, the names inside facts), **whether the shared tables arrive twice,** once as `fa`
     and once as `a.fa`, and whether the two theories still connect through them;
   - a program's own relation named as an internal one of a theory it imports flat, and as a public one;
   - a theory imported twice, flat and `:as`;
   - the calculator with `std.int` and `std.rat` and the binary examples in one program, flat: every clash
     named;
   - a program that calls a qualified theory's tagged call from a rule of its own.
3. **T1f.6.30.3 — the census:** over every stdlib module and every example theory, the relations it
   declares, which a program is meant to write (requests, `let`, the tagged calls) and which are the
   circuit's own; and every pair of modules that cannot be imported flat together today, with the name that
   stops them.
4. **T1f.6.30.4 — the proposals, each on real computed examples.** The user's rule for a syntax or design
   change is real programs first, so each proposal is shown as the probes of T2 rewritten under it, with
   what each would print, computed on a prototype where the proposal can be prototyped outside the engine
   (by rewriting module text) and marked *not computed* where it cannot. The stage does not pre-judge the
   list; what it must cover:
   - **what a module exports:** whether a module can say which of its relations a program sees, so that
     `done` of one theory and `done` of another never meet. What the unseen ones are called in a state, a
     trace and a core is part of the proposal, since every surface prints them;
   - **what `:as` shares:** whether a qualified import of a theory should share the modules that theory
     imports (the tables) with the importer's other imports;
   - **renaming on import** of a chosen name;
   - **a convention only** (a prefix by module, option (b) of question 8), priced: sixty relations renamed,
     every state, trace and golden of the calculator moved;
   - **nothing,** with the page saying how to mix theories today.
   Each with: what it changes in `ein.y` or `ein_ir::imports`, which corpus answers and goldens move, and
   what it does to E-8, E-20 and E-32.
5. **T1f.6.30.5 — the page and the stop.** `import_clashes.md` in this directory: the probes, the census,
   the proposals with their examples, a recommendation, and the questions, filed under the next free ids.
   **The stage stops here.** Nothing in the grammar or in what an import brings changes before the user
   rules.
6. **T1f.6.30.6 — the fix, as ruled:** planned after the stop, as tasks of this file or a stage of its own,
   with register rows first where today's behaviour is ruled a wrong answer.

## Acceptance

- The tagged calls are named on the module's page and held by a test.
- Every probe of T2 has its program and its output on the page.
- Each proposal is shown on the same programs, with what would move.
- The user has ruled, and T6 is planned to the ruling; or the page says the user chose to leave it.

## What it must not do

- **Not change `ein.l`, `ein.y` or `ein_ir::imports` before the stop.**
- **Not rename a relation of a shipped module** before the stop; and the layout move renames modules, not
  relations.
- **Not reserve names in the loader** ([reply 7](questions.md#q7), (a): not in this phase).
- **Not fix E-32** (a sibling directory) as a side effect; it is named in the proposals and ruled with them.

## The standing rules

A wrong answer of the engine met by a probe is a row in `corpus/wrong_answers.toml`, filed `open` before
its fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Syntax and design changes show real
computed examples first. Config never changes an answer; exit non-zero only on error. The gate is
`./run_tests.sh` for T1's commit, then `./run_tests.sh --nightly`; `utils/ci_container.sh` is not needed.
