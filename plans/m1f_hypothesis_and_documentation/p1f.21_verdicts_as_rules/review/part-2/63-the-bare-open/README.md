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
proposed. What the final binary makes of the file as the build rewrote it, and of a, b, c and d, is
[On the built engine](#on-the-built-engine), at the foot of the page.

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

## On the built engine

**In short.** Every prediction of § 1 holds on the final binary. **a** is refused at the parse; the build rewrote
the corpus file as **d**, the default (a) of Q-M1f.600, and it answers as **d** did; **b** answers as on the stand-in
engine, its check vacuous; **c**, refused on the stand-in engine, loads, with no rung that proposes.

**How it was taken.** The final binary, `build/review-bin/ein-final-7b72bd84`, the release build of 7b72bd84 (its
five lines: `runs/built/version.txt`), with the checkout's `stdlib/`. `tools/run-built.sh` re-takes every run into
`runs/built/`: for each program the read-out, `-p`, `ein test`, `ein show hyps`, `ein saturate --dump`, the oracle,
the events and the summary with the `rung` line, the answer and the visited nodes as files, and the knowledge base,
the lattice and the region drawn; `tools/hyps-built.py` reads the Rust test's check off each run's events into
`runs/built/hyps-check.txt`. The programs: the corpus file as it is now,
[`06_blind_enumeration.ein`](../../../../../../tests/stdlib/bijection/06_blind_enumeration.ein); **a** as the corpus
had it before the build, written here as [`a-as-it-was.ein`](programs/a-as-it-was.ein), three lines of header added;
and [b](programs/b-obligation-deleted.ein), [c](programs/c-open-is-a.ein) and
[d](programs/d-no-obligation-activated.ein) as they are, since the built engine reads their spelling: none writes
`hrule`, `:hrules` or `question`, and c's `(open is-a)` is an obligation's conclusion, which the built engine reads
as one. Where $H$ is 16 candidates the oracle's read-out is kept and its dump is not: the cube is 65 536 nodes and its
`region.json` 54 MB, so no region is drawn for those two (`runs/built/06_blind_enumeration.oracle.txt`, *65 536
saturations*).

**What the build did to the file.** It is **d**: its forms are d's, form for form, and its header is rewritten to say
why the enumerator runs where no obligation is activated (Q-M1f.576). Its corpus entry keeps a `test` run, and its
regression row stays.

| program | | the stand-in, on edbe6c3b (§ 1) | § 1's *once built* | the built engine | where |
|---|---|---|---|---|---|
| **a**, as it was | rung | `declined` | refused | refused at the parse: ``programs/a-as-it-was.ein:113:16: unexpected `)` ``, with the note *open takes one argument …*; `ein test` an error, exit 2 | `runs/built/a-as-it-was.solve.txt`, `.test.txt` |
| | $H$, $R$, `:expect` | 16, {∅}, holds | — | none: no event, no answer | `runs/built/hyps-check.txt` |
| **the file now** (= d) | rung | — | — | no `rung` line: no obligation is activated, and the blind enumerator writes $H$ | `runs/built/06_blind_enumeration.rung-line.jsonl` (empty), `.show-hyps.txt` |
| | $H$, $R$, `:expect` | — | — | 16 from `blind`; {∅}, `(model (seats Ann C1) (seats Bob C2))`; holds | `.show-hyps.txt`, `.solve.txt`, `.test.txt` |
| | the Rust test's check | — | — | 40 `hyp` lines, 16 emitted; none names an obligation rule | `runs/built/hyps-check.txt` |
| **b** | rung | `obligations`: root owes nothing | as today | `obligations`, owed 0, no candidate | `runs/built/b-obligation-deleted.rung-line.jsonl` |
| | $H$, $R$, `:expect` | none, {∅}, holds | as today | none, {∅}, holds | `.show-hyps.txt`, `.solve.txt`, `.test.txt` |
| | the Rust test's check | 0 proposals: vacuous | vacuous | 0 `hyp` lines: vacuous | `runs/built/hyps-check.txt` |
| **c** | rung | refused at load: `:no-hypothesis` names `is-a`, which obligation `sorted-somewhere` owes | loads, the blind enumerator not running | loads; `obligations`, owed 0, no candidate | `runs/built/c-open-is-a.solve.txt`, `.rung-line.jsonl` |
| | $H$, $R$, `:expect` | — | — | none, {∅}, holds | `.show-hyps.txt`, `.solve.txt`, `.test.txt` |
| | the Rust test's check | no events | — | 0 `hyp` lines: vacuous | `runs/built/hyps-check.txt` |
| **d** | rung | `blind` | as today | no `rung` line; $H$ from `blind` | `runs/built/d-no-obligation-activated.show-hyps.txt` |
| | $H$, $R$, `:expect` | 16, {∅}, holds | as today | 16, {∅}, holds | `.show-hyps.txt`, `.solve.txt`, `.test.txt` |
| | the Rust test's check | 40 proposals, none names an obligation rule | as today | 40 `hyp` lines, 16 emitted, none names an obligation rule | `runs/built/hyps-check.txt` |

`.show-hyps.txt` and the like are the same program's files in `runs/built/`. A `rung` line is written only where an
obligation is activated, so the file and d have none, as d had none on the stand-in engine (its `blind` is the word
`ein show hyps` printed there); a's was `declined`.

**The Rust tests on the built engine.** `ein-infer/tests/obligation_reports.rs` reads the file as it is, and its check
holds (the row *the file now*): the half on names, as § 2 says **d** keeps. [`obligation_rung.rs`](../../../../../../ein.rs/crates/ein-infer/tests/obligation_rung.rs)
names the file as a program on which no rung is taken, where it named it as its case of `declined`. The test compares
each token of a proposal with an obligation rule's short name, and the file imports its modules whole, so a proposal
that named one would name it by its full name, `std.algebra.total-owed`, which that comparison does not see;
`tools/hyps-built.py` reads full names too, and finds none (`runs/built/hyps-check.txt`, *by full name too: 0 name
one*).
