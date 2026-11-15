# S1f.10.15 — The rung a loaded rule chooses

**Phase:** [P1f.10](README.md), track H (what H is)
**Status:** planned 2026-11-15, at the re-plan. Not started. **Its question is put at the phase's one stop**
([S1f.10.12](s1f.10.12_the_structure_shown.md)), and nothing changes before the user replies.
**Depends on:** nothing in the phase.
**Owns:** E-51 of the calculator's [`findings.md`](../p1f.6_calculator/findings.md), given to this phase on
2026-11-14 and until the re-plan in no stage file.

## The finding

Hypothesis generation is a ladder: an hrule's proposals if the program declares one, else what would pay the
debts the state owes, else the blind enumerator. **The second rung is taken wherever the program *loads* an
obligation rule**, activated or not (`hypgen::generate_with`, `!program.obligations.is_empty()`), and a second
rung that owes nothing at root proposes nothing. So the blind enumerator does not run and H is empty.

That was M1d's scope rule, *discharge where stated*, read by the obligation rules a program states. Two things
have changed under it:

- **Every import brings its module whole.** A program that imports `std.algebra` for `symmetric` loads
  `total-owed` and `surjective-owed` with it, so it is on the second rung without having stated a debt, and
  its own open relations are not guessed about. *States* no longer means what it meant.
- **The mechanisms are read as declared at root.** `mechanisms.rs` reads an obligation as *a plan root's
  activators compile*: an obligation rule nothing activates is not among a program's mechanisms there, and it
  still chooses the rung here. Two readers, two readings of one declaration.

The witnesses of 2026-11-14, on the binary of that day: `(total-owed at is-a)` discharged at root, H empty, R
{∅}; the same with the activator deleted and the import kept, H empty, R {∅}; with the import deleted too, 22
candidates and R at least 110 057 states at the budget.

And one met at the re-plan, 2026-11-15, the release binary at `dbedc535`: a program that imports
`std.algebra :symbols (connex)`, declares `lt` over three nodes, states two `lt` facts and names no obligation
anywhere. Its read-out: *universe — no candidate: R is {∅} or empty, decided by root alone*, and *outside — lt
— no candidate: **no obligation owes it***. The program stated no obligation; the import did.

## Tasks

1. **T1f.10.15.1 — the three witnesses rebuilt** under `build/` on the binary and the stdlib as they stand,
   each read-out kept, and a fourth: a program that imports `std.algebra :symbols (symmetric)` beside a
   declared relation of its own that nothing closes.
2. **T1f.10.15.2 — the census**, by the `mechanism_census` example: the corpus programs that load an
   obligation rule no fact of root activates; of those, the ones with a relation the blind enumerator would
   propose about; and for each, R under each option below, by `ein oracle` within its limit.
3. **T1f.10.15.3 — the question**, under the next free id, put at the phase's stop:
   - (a) **as it is**: the rung is chosen by a rule loaded. The read-out says so where it bites: a `universe`
     row that names the rule which took the program off the blind enumerator, and `(solve :no-search)` remains
     the way to say *judge root alone* on purpose.
   - (b) **by an obligation activated at root**: the second rung where root's activators compile a plan of an
     obligation rule, the reading `mechanisms.rs` has. A program that imports a module for something else is
     on the third rung again. A discharged state under an activated obligation still proposes nothing and is
     complete, so *complete means discharged* is untouched.
   - (c) **by a debt owed at root**: the second rung only where root owes. Rejected by M1d for a reason that
     stands, that a discharged root would fall to the blind enumerator and exhaustion would be back in charge
     of completeness; listed so that the reply can say so.

   **Recommended: (b).** It makes the two readers agree and makes an import of one name not change what is
   guessed. **It restates a ruling of M1d's**, so it is the user's to take and is not acted on as a default.
4. **T1f.10.15.4 — as ruled.** Under (b): every corpus program whose R moves listed first, each a register
   row filed `open` before the fix where today's answer is wrong by the ruling; the switch in
   `hypgen::generate_with`; `06_reserved_names.md` § Hypothesis / query control and the ladder's sentence in
   `oblgen.rs`'s and `hypgen.rs`'s headers restated; the rows flipped in the fixing commit. Under (a): the
   read-out's row, with [`read_out.md`](../../../docs/kernel/inference/read_out.md) and its reference test in
   the same commit. E-51 closed in the calculator's `findings.md` with what was done.

## Acceptance

- The witnesses and the census are on the stage's record, each with its command and date.
- The user has ruled, in their words, in `open_questions.md`.
- Under (b): `import_invariance` still green, every moved R on the list made before the change, and the
  phase's README names this stage as an exception to *no answer changes*.
- Under (a): a program that an unactivated rule took off the blind enumerator says so in its read-out.

## What it must not do

- **Not change the ladder before the ruling.**
- **Not reopen what a discharged state under a stated obligation proposes**: nothing, and it is complete.
- **Not touch what the blind enumerator proposes** once it runs: that is
  [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md)'s.

## The standing rules

A wrong answer is a row in `corpus/wrong_answers.toml`, filed `open` before its fix. Config never changes an
answer. The gate is `./run_tests.sh`, once on the last commit of the batch.
