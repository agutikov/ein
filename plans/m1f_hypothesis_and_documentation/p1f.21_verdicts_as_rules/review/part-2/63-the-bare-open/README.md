# 63 · The bare `(open)`: `06_blind_enumeration.ein`, its claim another way, and what it can no longer ask

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[your answers](../ANSWERS.md)

**What it shows.** Task 1.6 of the stage. You ruled the bare `(open)` refused at load (Q-M1f.566). One corpus program
that answers today writes it:
[`tests/stdlib/bijection/06_blind_enumeration.ein`](../../../../../../tests/stdlib/bijection/06_blind_enumeration.ein),
under `tests/`, with a `test` run and a regression row. It writes `(open)` so that the obligations rung steps aside
and the blind enumerator runs beside the obligations `bijective-setup` activates. That is its purpose: the Rust test
`ein-infer/tests/obligation_reports.rs`, `a_blind_run_never_proposes_an_obligation_activator`, reads its events and
asserts that no proposal names an obligation rule, as a fact's relation or as an argument, and that something was
proposed.

**The answer, short.** Once built the file is refused. **Its `:expect` can be kept two ways**, and holds on today's
engine both ways. **What the Rust test asks can be kept in half**: that an obligation rule's name is never a
candidate object, with the four obligation rules loaded and none activated. **The other half cannot be asked any
more**: *the blind enumerator, running where an obligation activator is held, proposes none*, because where one is
held its obligation is activated, and then the blind enumerator does not run (Q-M1f.576, Q-M1f.578).

## 1. The runs

Today's engine, `build/review-bin/ein-head-edbe6c3b`, with the checkout's `stdlib/`. `tools/run.sh` re-takes them
into `runs/` (ignored by git; mirrored under `build/review-incoming/p1f21-s12-examples/63-the-bare-open/`), each
program's read-out, `ein test`, `ein show hyps`, `ein saturate --dump`, its events and `rung` line, the summary,
the answer as files, the knowledge base drawn; `runs/hyps-check.txt` is the Rust test's check read off each run's
events.

| program | what changes | rung today | H | $R$ | `:expect` | the Rust test's check |
|---|---|---|---|---|---|---|
| [a, as it is](../../../../../../tests/stdlib/bijection/06_blind_enumeration.ein) | nothing | `declined`: *a bare `(open)` names no relation to branch on* | 16 | {∅} | holds | 40 proposals, none names an obligation rule |
| [b](programs/b-obligation-deleted.ein) | `sorted-somewhere`, the obligation that writes `(open)`, deleted | `obligations`: root owes nothing | none | {∅} | holds | **0 proposals: vacuous** |
| [c](programs/c-open-is-a.ein) | `(open is-a)` in place of `(open)` | refused at load: *its `:no-hypothesis` names `is-a`, and obligation `sorted-somewhere` owes `is-a`* | | | | no events |
| [d](programs/d-no-obligation-activated.ein) | `bijective-setup`'s activators written by hand but the two obligations'; `std.algebra`, `std.bijection` and `std.slots` imported whole | `blind`: nothing proposes and no obligation is activated | 16 | {∅} | holds | 40 proposals, none names an obligation rule |

Once built: **a** is refused; **b** answers as today, the blind enumerator never running; **c** is refused today by
the list's conflict with what the program declares, and once built the list is judged against the `open` facts root
holds (Q-M1f.578, and the question Q-M1f.597 on [item 65](../65-the-lists-and-a-closure/README.md)): root owes
nothing there, so it holds none, and by the ruling's words **c** loads, the blind enumerator still not running; **d**
answers as today, since nothing in it writes an `open` fact.

## 2. What each way keeps

- **b** keeps the file's claim, the bijection's region at root, and nothing of the Rust test: with an obligation
  activated and nothing owed, no rung proposes, which is what the rulings make of such a program. The Rust test's
  own guard, *nothing was proposed, so the check is vacuous*, would fail.
- **d** keeps the claim and the half of the Rust test that is about **names**: the four obligation rules of the
  library are in the program (`total-owed` and `surjective-owed` from `std.algebra`, `slot-owed-room` and
  `slot-owed-fill` from `std.slots`), the blind enumerator runs, and none of their names is a candidate object.
  It drops `bijective-setup` (its activators are written instead), so the file no longer exercises that rule;
  three other programs of `tests/stdlib/bijection/` name it (`01`, `02`, `03`).
- **Neither keeps the half about activator facts.** With a `(total-owed seats is-a)` held, its obligation is
  activated, and the blind enumerator does not run: there is no run in which it could propose one. Once built that
  is the ladder itself, and no question of the enumerator.

**Also once built: no corpus program steps aside.** `06_blind_enumeration.ein` is the one program whose rung declines
for a bare `(open)`, and the eight others decline for a guard that reads what a commitment can change, which no
longer steps aside (Q-M1f.579, [item 64](../64-an-obligation-that-no-longer-steps-aside/README.md)).
`ein-infer/tests/obligation_rung.rs` names this file as its case of the rung `declined`; the one decline left in
the engine, a projection that does not resolve per activator, no corpus program reaches today.

## 3. Your call

**Q-M1f.600, put here: what becomes of `06_blind_enumeration.ein`.**

- **(a)** It is rewritten as **d**: the bijection's activators written but the obligations', the library's four
  obligation rules loaded, the blind enumerator running; `obligation_reports.rs` keeps its check on names, and drops
  the half on activator facts, which no run can ask. **Default.**
- **(b)** It is rewritten as **b**, keeping `bijective-setup`; the Rust test's check moves to a new file written as
  **d**.
- **(c)** It is deleted, with the Rust tests' cases that name it.
