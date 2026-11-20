# 64 · An obligation that no longer steps aside: the eight programs, and `oblig-derived-domain.ein`

[← part 2](../README.md) · [← the phase](../../../README.md) · [the stage](../../../s1f.21.12_hypotheses_as_facts.md) ·
[the ruling's page](../../../q-m1f.579_an_obligation_that_steps_aside/README.md) · [your answers](../ANSWERS.md)

**What it shows.** Task 1.7 of the stage: the eight corpus programs whose obligations rung steps aside today, each
as it answers today and as it would with its obligation's guard read at root, as you ruled (Q-M1f.579 (b)). And
`examples/ein-bugs/wrong-answers/oblig-derived-domain.ein`, which you asked to see: what the stand-in answers, and
what would make it solvable or not. Your expectation, *"I expect it will still be solvable, despite
non-monotonicity. This is my expectation, not requirement. Let's review it after implementation"*, is not turned
into anything here; the program is shown to you again after the build, as the stage's plan says. The runs on the
final engine are in [On the built engine](#on-the-built-engine), at the foot of the page: every $H$, $R$ and claim
the stand-in gave is what the built engine gives, all eight are flagged, and `oblig-derived-domain.ein` answers an
empty $R$, while each of the two changes of § 3 makes it answer `(likes Ann Pie)`, flagged.

**The answer, short.** Once built, **all eight are flagged** (six newly), and nothing is claimed of their answers.
On the stand-in, four answer an empty $R$, `oblig-derived-domain.ein` among them; the four `-declared` twins,
which state the derived food at root, keep a state: two the same one state as today, two fewer states than today. `oblig-derived-domain.ein` answers
`(likes Ann Pie)` on the stand-in **if Pie is a food at root, or if a rule says what Ann may like**; it does not
otherwise, and reading the guard at every state does not change that.

## 1. How it was taken

Nothing is built: every run is today's engine, `build/review-bin/ein-head-edbe6c3b`, with the checkout's
`stdlib/`, on the corpus at `669d7d43`. `tools/run.sh` re-takes every run into `runs/` (ignored by git; mirrored
under `build/review-incoming/p1f21-s12-examples/64-an-obligation-that-no-longer-steps-aside/`): for each program its
read-out, `-p`, `ein show hyps`, `ein saturate --dump`, the static check alone, the oracle with every node's state,
the events and their `rung` line, the summary, the answer as files and the knowledge base drawn.

**The stand-in.** Each `programs/*.guard-at-root.ein` is the corpus program whole, with one hypothesis rule more,
`guard-at-root`, whose `:match` is the obligation's trigger, its `absent` and its guard, and whose `:assert` is the
fact that would pay: exactly the candidates the obligation's guard gives at root for each instance root owes. Today a
hypothesis rule overrides the obligations rung, so the stand-in's H is those candidates and nothing else, which is
what the obligation writes once it no longer steps aside. `tools/gen.py` wrote them; their `:expect` is the
original's.

**What the stand-in cannot show: the flag.** Today's static check calls six of the eight monotone, and the stand-ins
too (`runs/*.monotone.txt`). Once built, the guard of each reads a relation a commitment can change (`food`, which
the rule `special` derives once someone likes something; `is-a` in the two that use `std.algebra`'s `total-owed`),
which is the new site of the static check, so each is flagged. That is a reading of the ruling, taken from D as
today's static check computes it ([item 58](../58-the-corpus-under-the-rulings/README.md), § 6).

## 2. The eight

All under `examples/ein-bugs/wrong-answers/`.

| program | who proposes today | H today → guard at root | $R$ today → stand-in | its claim today → stand-in | flagged today → built | read by today |
|---|---|---|---|---|---|---|
| [`oblig-derived-domain.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain.ein) · [stand-in](programs/oblig-derived-domain.guard-at-root.ein) | the blind enumerator | 6 → 1 | 1 state → **empty** | holds → fails | no → **yes** | a regression row |
| [`oblig-derived-domain-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein) · [stand-in](programs/oblig-derived-domain-declared.guard-at-root.ein) | the blind enumerator | 6 → 2 | 1 → 1 | holds → holds | no → **yes** | a `test` run |
| [`oblig-fresh-name.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name.ein) · [stand-in](programs/oblig-fresh-name.guard-at-root.ein) | the blind enumerator | 6 → 1 | 1 → **empty** | holds → fails | no → **yes** | a regression row |
| [`oblig-fresh-name-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-declared.ein) · [stand-in](programs/oblig-fresh-name-declared.guard-at-root.ein) | the blind enumerator | 6 → 2 | 1 → 1 | holds → holds | no → **yes** | a `test` run |
| [`oblig-fresh-name-two.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-two.ein) · [stand-in](programs/oblig-fresh-name-two.guard-at-root.ein) | the blind enumerator | 12 → 2 | 12 → **empty** | holds → fails | no → **yes** | a regression row |
| [`oblig-fresh-name-two-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-fresh-name-two-declared.ein) · [stand-in](programs/oblig-fresh-name-two-declared.guard-at-root.ein) | the blind enumerator | 12 → 4 | 12 → 3 | holds → fails | no → **yes** | a `test` run |
| [`oblig-stdlib-total-owed.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein) · [stand-in](programs/oblig-stdlib-total-owed.guard-at-root.ein) | the blind enumerator | 6 → 1 | 16 → **empty** | holds → fails | yes → yes | none |
| [`oblig-stdlib-total-owed-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein) · [stand-in](programs/oblig-stdlib-total-owed-declared.guard-at-root.ein) | the blind enumerator | 6 → 2 | 16 → 1 | holds → fails | yes → yes | none |

*Holds* and *fails* are what the run prints of the claim. **Once built nothing is claimed of any of the eight**:
no `test` run, regression row or claim-checking run may hold a flagged program's answer (Q-M1f.564), so the build
takes the first six's out (§ 4).

Why the candidates shrink: the blind enumerator guesses `likes` over every name (`(likes Pie Ann)`, `(likes Soup
Pie)` and the like, a person only being required of the one who likes); the guard proposes only what pays the
debt, a food at root. Why the `-declared` twins keep a state and the others lose theirs: the twin states at root the
food a commitment would otherwise derive (`(food Pie)`, `(food Z)`, `(is-a Z Food)`), so the guard reads it there.

## 3. `oblig-derived-domain.ein`

Ann is a person; Soup is a food; Pie is a dish. Ann owes a liked food, and may not like Soup. Once anyone likes
anything, every dish is a food (`special`). Its claim: $R$ is the one state where Ann likes Pie.

| | who writes Ann's candidates at root | H | $R$ | flagged once built |
|---|---|---|---|---|
| [today](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain.ein) | the blind enumerator, the obligations rung having stepped aside | 6, `(likes Ann Pie)` among them | `(likes Ann Pie)` | — (today: no) |
| [the guard at root](programs/oblig-derived-domain.guard-at-root.ein), the ruling | the obligation: `(food ?b)` at root is Soup | `(likes Ann Soup)` | **empty**: root owes, the one candidate is dead | yes |
| [`(food Pie)` at root](programs/oblig-derived-domain-declared.guard-at-root.ein), its twin | the obligation: Soup and Pie | `(likes Ann Pie)` `(likes Ann Soup)` | `(likes Ann Pie)` | yes |
| [a rule says what Ann may like](programs/oblig-derived-domain.a-rule-proposes.ein) ([as ruled](programs/oblig-derived-domain.a-rule-proposes.target.ein)) | the obligation and the rule `may-like`, the union | `(likes Ann Pie)` `(likes Ann Soup)` | `(likes Ann Pie)` | yes |

So, on the stand-in of the ruling as built, **the program is not solvable: $R$ is empty**, and the read-out says root
owes and nothing pays. Two things make it answer `(likes Ann Pie)`: Pie a food at root, or a rule that proposes it
beside the obligation (`may-like` concludes `(open (likes ?a ?d))` for every dish; its file in the spelling ruled is
refused at the parse today, `runs/oblig-derived-domain.a-rule-proposes.target.solve.txt`). Either way the guard still
reads `food`, so the program is flagged and nothing is claimed of what it answers.

**What does not make it solvable.** Reading the guard at every state, as the pass does once built (Q-M1f.569): the
one commitment root proposes, `(likes Ann Soup)`, is dead, and in it Ann's debt is paid by Soup, so no state above
root writes `(open (likes Ann Pie))`; the check in flight finds nothing here, and would change no answer if it did
(Q-M1f.574: the answer stands under the warning). Supposing what is first held above root, which you parked (V-21),
would not either, for the same reason.

## 4. What the build changes for them

- Each of the eight is flagged; the read-out's first row, stderr's first line and the summary's warning say so.
- `oblig-derived-domain-declared`, `oblig-fresh-name-declared` and `oblig-fresh-name-two-declared` lose their
  `test` run; `oblig-derived-domain`, `oblig-fresh-name` and `oblig-fresh-name-two` their regression row; each of
  the six its claim, or its `solve` run, since that run checks the claim.
- `oblig-derived-domain.ein` is run again on the built engine and shown to you beside this page's stand-in.

## 5. Your call

The ruling stands as you gave it; this page reports what it gives.

- **(a)** Build as ruled. `oblig-derived-domain.ein` is shown to you after the build beside these runs. **Default.**
- **(b)** As (a), and the build adds `oblig-derived-domain.a-rule-proposes.ein`'s program, in the spelling ruled, to
  the corpus beside the witness, so the corpus has the program that answers `(likes Ann Pie)` under the ruling
  (flagged, with no claim).

## On the built engine

**How it was taken.** The final binary, `build/review-bin/ein-final-7b72bd84`, the release build of `7b72bd84`, with
the checkout's `stdlib/`, on the corpus at `7b72bd84`; its `--version` is `runs/built/version.txt`.
`tools/run-built.sh` re-takes every run into `runs/built/` (ignored by git); the stand-in's runs in `runs/` are kept
as they were. For each program: the read-out (`X.solve.txt`), `-p`, `--in-flight -p`, `--monotone`, `ein test`,
`ein show hyps`, `ein saturate --dump`, the oracle with every node's state (`X.oracle/`, and again under
`--in-flight`), the events with their `rung` line and the summary, the answer and the visited lattice as files, and
the knowledge base, the lattice (both views) and the region drawn (`X.kb.svg`, `X.lattice.svg`,
`X.lattice-full.svg`, `X.region.svg`, each beside its `.dot`).

**The programs.**

- **The eight corpus programs** changed between `edbe6c3b` and `7b72bd84` in their comments only, in one commit, the
  build's corpus task: each header now says that the obligation's guard reads a relation a commitment can change, so
  the program is flagged by an `obligation-guard` site, and that its `:expect` states what it was once held to and
  no run of the corpus checks it. Their rules, facts and `:expect` are as they were (compared with the comments
  stripped). The same commit changed what reads them (the last column below).
- **The stand-ins** `programs/*.guard-at-root.ein` are kept unchanged and are not run again. On the built engine the
  obligation writes its guard's candidates at root itself, so each corpus program as it is is what its stand-in
  stood for. No built copy of them is written: each corpus program's $H$ and $R$ below are its stand-in's exactly,
  so a copy would show nothing the corpus program does not.
- **[`oblig-derived-domain.a-rule-proposes.target.ein`](programs/oblig-derived-domain.a-rule-proposes.target.ein)**
  is run as it is. It loads now; the stand-in engine refused it at the parse.
- **One program added**,
  [`oblig-derived-domain.a-rule-proposes.built.ein`](programs/oblig-derived-domain.a-rule-proposes.built.ein): the
  stand-in `oblig-derived-domain.a-rule-proposes.ein` in the spelling the built engine reads. Its hypothesis rule
  `guard-at-root` stood for the obligation and is left to it, as in the eight; `may-like` concludes
  `(open (likes ?a ?d))`. So it is the target with the stand-in's `:expect`, which the target does not state: it
  gives `ein test` a claim to read.

### The eight, stand-in beside built

Each cell gives the stand-in's figure, taken on `edbe6c3b` (the second figure of § 2's table), then **the built
engine's**. $H$ is from `runs/built/X.show-hyps.txt`, $R$ from the `states (k_s)` row and the `(region …)` of
`runs/built/X.solve.txt` (the oracle's `runs/built/X.oracle.txt` prints the same region on each), the claim from the
`(solve :expect)` row there and from `runs/built/X.test.txt`, the flag from `runs/built/X.monotone.txt`. *Fails* and
*holds* are what the run prints of the claim. All under `examples/ein-bugs/wrong-answers/`.

| program | $H$: stand-in → **built** | $R$: stand-in → **built** | claim: stand-in → **built** | flagged: `edbe6c3b` → **built** | read by: before the build → **now** |
|---|---|---|---|---|---|
| `oblig-derived-domain.ein` | 1 → **1**, `(likes Ann Soup)` | empty → **empty** | fails → **fails** | no → **yes**: `obligation-guard` on `food` | a `solve` run and a regression row → **`solve --count`** |
| `oblig-derived-domain-declared.ein` | 2 → **2** | 1 state → **1**, `(likes Ann Pie)` | holds → **holds** | no → **yes**: the same site | a `solve` and a `test` run → **`solve --count`** |
| `oblig-fresh-name.ein` | 1 → **1** | empty → **empty** | fails → **fails** | no → **yes**: the same site | a `solve` run and a regression row → **`solve --count`** |
| `oblig-fresh-name-declared.ein` | 2 → **2** | 1 → **1**, `(likes Ann Z)` | holds → **holds** | no → **yes**: the same site | a `solve` and a `test` run → **`solve --count`** |
| `oblig-fresh-name-two.ein` | 2 → **2** | empty → **empty**, `{(likes Bob Soup)}` owing | fails → **fails** | no → **yes**: the same site, and found in flight | a `solve` run and a regression row → **`solve --count`** |
| `oblig-fresh-name-two-declared.ein` | 4 → **4** | 3 → **3** | fails → **fails** | no → **yes**: the same site | a `solve` and a `test` run → **`solve --count`** |
| `oblig-stdlib-total-owed.ein` | 1 → **1** | empty → **empty** | fails → **fails** | yes, 1 site (`obligation-trigger` on `is-a`) → **yes, 2 sites** (and `obligation-guard` on `is-a`) | `solve --count` → **`solve --count`** |
| `oblig-stdlib-total-owed-declared.ein` | 2 → **2** | 1 → **1**, `(likes Ann Z)` | fails → **fails** | yes, 1 site → **yes, 2 sites** | `solve --count` → **`solve --count`** |

*Read by* is [`corpus/corpus.toml`](../../../../../../corpus/corpus.toml) and
[`corpus/regressions.toml`](../../../../../../corpus/regressions.toml) at each commit. Now each of the eight has one
run, `solve --count`, which reads no state and checks no claim; none has a regression row, and none a `test` run, so
no suite runs `ein test` on it: only `ein test FILE`, asked by hand, reads its `:expect`, as `runs/built/X.test.txt`
does. Beyond those two files, the corpus's shape golden digests their load, saturation and drawings and none of their
answers.

**Where they differ, and why.**

- **$H$, $R$ and the claim do not differ on any of the eight.** The stand-in's $H$ was exactly what the obligation's
  guard gives at root, and on the built engine the obligation writes that itself: `ein show hyps` says
  ``H, from `obligations` `` on each (`runs/built/X.show-hyps.txt`), and the events' `rung` line has mode
  `obligations` (`runs/built/X.rung-line.jsonl`), where on `edbe6c3b` the corpus program's was `declined` (the blind
  enumerator built $H$) and the stand-in's was the rung `hrules`.
- **The flag differs on six**, as § 1 said it would: `edbe6c3b`'s static check had no `obligation-guard` site, and
  the built engine flags each of the six by it (Q-M1f.579). The two programs on `std.algebra`'s `total-owed` were
  flagged already, by the trigger's site, and gain the guard's. Each of the eight shows the warning three ways: the
  read-out's first row, `WARNING  THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER.`
  (`runs/built/X.solve.txt`), stderr's first line, `WARNING: examples/…/X.ein: THIS PROGRAM IS NOT MONOTONE: NOTHING
  IS CLAIMED OF THIS ANSWER (Q-M1f.564)` (the same file, which holds both streams), and the summary's
  `monotonicity.warning` (`runs/built/X.summary.json`). Nothing is claimed of their answers; *holds* and *fails*
  above say only what the run prints.
- **The route follows the flag**: on the built engine each is answered by the cube, route 2 ("no licence: the static
  check flags the program"), where the six stand-ins the static check called monotone took the state route. The
  `work` rows differ with the route; no state and no count does.
- **Found in flight, on `oblig-fresh-name-two.ein` alone.** Its read-out's `lattice` rows add `in flight  a state
  the run built holds (open (likes Ann Z)), which root does not hold` (`runs/built/oblig-fresh-name-two.solve.txt`):
  at the owing state `{(likes Bob Soup)}`, `special` derives `(food Z)`, Ann still owes, and the obligation writes
  what would pay her there, `(likes Ann Z)`, which no candidate at root names (`-p` lists `(open (likes Ann Z))` in
  that state, `runs/built/oblig-fresh-name-two.solve-p.txt`). Under `--in-flight`, and in the oracle, whose
  `--states` saves every node, the row names `(open (likes Bob Z))` instead, held at the dead state
  `{(likes Ann Soup)}`, where Bob still owes (`runs/built/oblig-fresh-name-two.solve-in-flight.txt`;
  `runs/built/oblig-fresh-name-two.oracle/region.json`, the node `dead+owing`): the check names the first such
  hypothesis it meets, and the option moves no other line (Q-M1f.656). The program is flagged by its site already, so stderr says the warning once ([item
  69](../69-S1f.21.12-defaults/README.md), default 10). On the seven others and on both a-rule-proposes programs
  nothing is found in flight: `monotonicity.in_flight` is `null` in each summary and in each
  `runs/built/X.oracle-in-flight/region.json`.

### `oblig-derived-domain.ein` on the built engine

**What it answers.** The program is **flagged**, and nothing is claimed of its answer. `ein solve --monotone` says
`monotone  no — 1 site`, the site `owes-food: obligation-guard reads food at absent depth 1, concludes (open …) —
at stake: the hypotheses: a state can hold one root does not` (`runs/built/oblig-derived-domain.monotone.txt`). The
read-out opens with the `WARNING` row, stderr's first line is `WARNING:
examples/ein-bugs/wrong-answers/oblig-derived-domain.ein: THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS
ANSWER (Q-M1f.564)`, and the summary's `monotonicity.warning` says the same (`runs/built/oblig-derived-domain.solve.txt`,
`runs/built/oblig-derived-domain.summary.json`). Under that warning:

- **$H$ is one candidate**, `(likes Ann Soup)`, written by the obligation (``H, from `obligations`: 1 candidate``,
  `runs/built/oblig-derived-domain.show-hyps.txt`): at root the guard's `(food ?b)` finds Soup alone.
- **Root holds two `open` facts**, both written by `owes-food`: `(open likes)`, what Ann owes, and
  `(open (likes Ann Soup))`, what would pay it (`open (2)` in `runs/built/oblig-derived-domain.saturate-dump.txt`).
- **What root owes and what pays it**, the read-out's rows: `owing  ∅ — owes 1`, under it `Ann owes a liked food`;
  `empty  every alive node owes — no commitment pays every debt`; and `(region (owing (model)))`. The one candidate
  would pay the debt and is dead alone: in the cube, of 2 nodes, `{(likes Ann Soup)}` is the one minimal discharging
  set and the one minimal dead set, refuted by `no-soup` (`discharging`, `nogoods` and the node's `refuted_by` in
  `runs/built/oblig-derived-domain.oracle/region.json`).
- **$R$ is empty**: `states (k_s)  0`, `commitments (k_c)  0`.
- **Its claim** `(model (likes Ann Pie))` is not met: `(solve :expect)  FAILED — the claim lists 1 state and R is
  empty`; `ein test` says `FAILED` and exits 1 (`runs/built/oblig-derived-domain.test.txt`).
- **Nothing is found in flight.** `--in-flight` changes no line of the read-out
  (`runs/built/oblig-derived-domain.solve-in-flight.txt`), and the oracle under it finds nothing
  (`in_flight: null`, `runs/built/oblig-derived-domain.oracle-in-flight/region.json`): the one state above root,
  `{(likes Ann Soup)}`, is dead and owes nothing there (Soup pays Ann's debt), and its state beyond root's holds no
  `open` fact, `(open (likes Ann Pie))` least of all.

So on the built engine the program is not solvable: $R$ is empty, as the stand-in showed (§ 3) and as
[item 69](../69-S1f.21.12-defaults/README.md) reports. No option moves it: none changes $R$.

**What makes it answer `(likes Ann Pie)`**: either change of § 3, as the binary answers them. Each cell is the built
engine's; the stand-in's figures (§ 3's table) are the same $H$ and the same $R$.

| | who writes Ann's candidates at root | $H$ | $R$ | claim | flagged |
|---|---|---|---|---|---|
| the program, [`oblig-derived-domain.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain.ein) | the obligation | 1, `(likes Ann Soup)` | empty; root owes 1 | fails | yes |
| `(food Pie)` at root, [`oblig-derived-domain-declared.ein`](../../../../../../examples/ein-bugs/wrong-answers/oblig-derived-domain-declared.ein) | the obligation: Soup and Pie | 2, `(likes Ann Pie)` `(likes Ann Soup)` | 1 state, `(likes Ann Pie)` | holds | yes, `obligation-guard` on `food` |
| a rule says what Ann may like, [the target](programs/oblig-derived-domain.a-rule-proposes.target.ein) | the obligation and `may-like`: ``H, from `union` `` | 2, the same | 1 state, `(likes Ann Pie)` | none stated: `ein test` prints `(no expect)` and exits 2, *nothing to check* | yes, the same site alone |
| the same, [with the stand-in's `:expect`](programs/oblig-derived-domain.a-rule-proposes.built.ein) | the same | 2, the same | 1 state, `(likes Ann Pie)` | holds | yes, the same site alone |

The files: `runs/built/oblig-derived-domain-declared.*`, `runs/built/oblig-derived-domain.a-rule-proposes.target.*`
and `runs/built/oblig-derived-domain.a-rule-proposes.built.*` (`show-hyps`, `solve`, `test`, `monotone`). `may-like`
reads `person` and `dish`, which no commitment changes, so the rule adds no site of its own: the one site is the
obligation's guard, as on the program.

What root owes and what pays it, in both: `∅  not in R — root owes 1 instance`, `owed  Ann owes a liked food`;
`E  2 minimal discharging sets`, `{(likes Ann Pie)}` and `{(likes Ann Soup)}`; `N  1 minimal dead set`,
`{(likes Ann Soup)}` (the read-outs, and `discharging` and `nogoods` in each `X.oracle/region.json`). In the twin,
root holds `(food Pie)` already, and the one state adds `(likes Ann Pie)` alone
(`runs/built/oblig-derived-domain-declared.solve-p.txt`). In the target, `special` derives `(food Pie)` once Ann
likes Pie, which pays her debt; `-p` lists the state as `(food Pie)`, `(likes Ann Pie)` and `(open (likes Ann Pie))`
(`runs/built/oblig-derived-domain.a-rule-proposes.target.solve-p.txt`): `may-like` concludes its hypothesis at every
state, so it stands beside the fact it supposes, as [item 60](../60-a-state-with-its-open-facts/README.md) shows for
a rule.

Both changes are flagged by the same site as the program, so nothing is claimed of what they answer: that each
answers `(likes Ann Pie)` is what the binary prints, not a claim about it.
