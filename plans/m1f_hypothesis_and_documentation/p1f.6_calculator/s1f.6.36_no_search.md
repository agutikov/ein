# S1f.6.36 — Judging root without a search: `(solve … :no-search)` and `--no-search`

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-14, on the user's words of 2026-11-13 and 2026-11-14. **T1 done 2026-11-14, stopped
there for the coordinator** ([§ Record](#record-2026-11-14)): the design on real programs, computed on a prototype;
defaults Q-M1f.254 to .260; E-57 filed. **An engine stage and a language addition.**
**Depends on:** [S1f.6.34](s1f.6.34_hypothesis_mechanisms_in_conflict.md), whose conflict check it joins and
whose corpus corrections (Q-M1f.246) it revisits; [S1f.6.35](s1f.6.35_imports_by_full_name.md), which it runs
after, since both move corpus files. Independent of the stdlib layout; it runs before
[S1f.6.31](s1f.6.31_the_stdlib_layout.md).

## The ruling

S1f.6.34 refused a program whose `:no-hypothesis` lists a relation its own obligations owe (*"filtered obligations
are nonsense by definition"*, the user, 2026-11-13). Twenty-four corpus programs did that on purpose, to switch the
search off and let the obligations judge root alone; S1f.6.34 corrected them, three with an hrule `as-stated` that
re-proposes only what root holds. Asked whether Ein should have a sanctioned way instead, the user, 2026-11-13:

> maybe some explicit flag in (solve ) ? instead of abusing bugs; or cmdline flag that disable search; or both

and, on the coordinator's objection that a flag may not change R (the ruling of 2026-10-15), 2026-11-14:

> No conflicts! flag that disables search doesn't change R, it disables discovering of R. R is not an output, it
> is complete result when we do full R search.

So **both**: a keyword the program writes, and a flag a command takes.

- **`(solve … :no-search)`** is the program's statement that its lattice is root alone: no candidate from any
  rung, whatever hrules, obligations or lists it also declares, so R is {∅} where root is alive and discharged
  and empty otherwise. It is part of the program, as `:hypothesis-relations` is, and every command reads it.
- **`--no-search`** is a command's choice not to discover R: it saturates root and reports root (its facts, alive
  or dead, discharged or what it owes), and says that R was not searched. It changes no R: a run without it
  discovers the R it would have declined to.

## Tasks

1. **T1f.6.36.1 — the design on real programs, shown before anything is built.** The 24 programs S1f.6.34
   corrected, each written with `:no-search` in place of its correction where its intent was *judge root alone*
   (the record says which were and which were not), with what every command answers; E-47's witness and the
   conflict-check fixtures with `:no-search` beside each mechanism. And for `--no-search`, what each command
   prints on a sample of the corpus: `ein solve` (its read-out rows, the `R` row saying *not searched*), `ein
   test` (which claims root decides, a `(solve :expect (false))` on a dead root among them, and which stay NOT
   CHECKED, saying why), `ein oracle` (refused, or the root alone: the stage proposes), the `--json-summary` and
   `--dump-answer` artefacts. Recorded as the record's first section.
2. **T1f.6.36.2 — the keyword.** The grammar's `solve` gains `:no-search` (a flag keyword, no argument), a row of
   the grammar page's table; the loader reads it; the candidate generator proposes nothing under it. S1f.6.34's
   check: `:no-search` beside obligations is silent (the intended use); beside an hrule, `:hypothesis-relations`
   or `:no-hypothesis`, the other declaration filters or proposes nothing, a `note:` as § 4.27's dead lists have.
3. **T1f.6.36.3 — the flag.** `--no-search` on `ein solve` and `ein test` (and on any other command where T1
   shows it means something), in `configuration.md`'s census with *does it change the answer: no — it declines to
   discover it*; `lever_invariance.rs` holds that every run with the flag reports root as the run without it
   reaches it, and never states an R.
4. **T1f.6.36.4 — the corpus.** The programs T1 names rewritten to `:no-search`, S1f.6.34's workarounds removed
   (the `as-stated` hrules among them), each claim held; the mutants S1f.6.34's corrections killed stay killed
   (`utils/stdlib_mutants.py --check`). Goldens counted before blessing.
5. **T1f.6.36.5 — the pages and the records:** the grammar, `defined_behaviour.md`, `configuration.md`,
   `read_out.md` (the row that says R was not searched), `tests/README.md`'s idioms (*judge root alone*: write
   `:no-search`), Q-M1f.246 restated, the phase README's row, the stage's record.

## Acceptance

- A program with obligations and `(solve :no-search)` loads, proposes nothing, and is judged at root; the 24
  fixtures say what they meant with it, and none carries a workaround.
- `--no-search` changes no R: on every corpus program, what it reports of root is what the full run reaches at
  root, and it states no R.
- Every page that lists the `solve` keywords or the CLI's options lists the two.

## What it must not do

- Not make `--no-search` a lever under which a complete search reports another R: it declines to search, and
  says so.
- Not relax S1f.6.34's conflicts: `:no-hypothesis` over an owed relation stays refused; `:no-search` is the way to
  say *no search*.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every answer exits 0. The gate runs once over
the batch this stage belongs to (the user's cadence of 2026-11-13).

## Record, 2026-11-14

### T1f.6.36.1 — the design on real programs, done

**How it was computed.** On a prototype: the stage's own draft of T2 and T3, in the working tree and not committed
(saved as `build/s1f.6.36/proto.patch`, gitignored), its release binary `build/s1f.6.36/ein-proto`; HEAD's binary,
built from `git archive 27197ea3`, is `build/s1f.6.36/ein-head`. Every answer below is that binary's, 2026-11-14.
The scripts are `build/s1f.6.36/t1_fixtures.py` (the 24), `gen_witnesses.py` and `run_witnesses.py` (the
mechanisms), and `sweep.py` (the corpus). The prototype passes `cargo test --release` of `ein-ir` (the EBNF blocks
blessed), `ein-infer`, `ein-render` and `ein-cli` but for the three binaries that pin the pages T3 and T5 write
(`answer_dump.rs`, the summary's new keys; `help_surface.rs`, the two options; `read_out_reference.rs`, the new
table and row).

**What the prototype is.** The keyword: `ein.l` and `ein.y` take `(solve … :no-search)`, a flag whose pair holds
`()`, the grammar's first keyword with no value; a `(query …)` that writes it is refused at the parse with the note
the other lattice keywords get (*:no-search chooses the lattice and belongs on (solve …)*); `(solve :no-search (at))`
is a parse error. `hypgen::declares_no_search` reads it, and the ladder proposes nothing under it, before any rung
(`oblgen::Mode::NoSearch`, the rung's word `no-search`); `eligible_relations`
is empty, `scope::outside` names nothing (every relation scoped out by the program's own statement), and the S1f.5.18
check of `from_ir` reads no commitment's facts under it. The flag: `explore::Options::no_search` takes root's node
through the node function every route uses (`Space::eval` of ∅) and builds no other; its answer is a description of
what root alone establishes, `Route::Root`, partial (`Stopped::NoSearch`) unless root is dead, where every node is
(L0) and it is exhausted as every route reads it on one saturation. `ein solve --no-search` prints its own read-out
(`render_root`), `ein test --no-search` asks that one question and no fallback, and `ein oracle --no-search` is
refused.

#### The keyword on the twenty-four

S1f.6.34's record says which of its 24 corrected programs meant *judge root alone*: its groups A, B (R empty) and B
(root's saturation), 21 programs, each switching the obligations' generating role off so that the search had
nothing to do. The other three did not: `tests/stdlib/bijection/06` means the blind enumerator (its obligation is
written to make the rung decline); `tests/stdlib/closure/02` was O + C, and its obligation cannot come back beside
the closure, which `:no-search` does not relax (Q-M1f.254); `examples/ein-bugs/closed-and-owing.ein` is O + C's
refusal fixture. Each of the 21 written with `:no-search` is its program as it was before S1f.6.34 (`e45c4c18^`),
its `:no-hypothesis (…)` replaced by `:no-search` and nothing else: no `as-stated` hrule, no fan-out written out, no
negative added to make a debt unpayable or R small (`build/s1f.6.36/fx/`):

| group | programs | today (S1f.6.34's text) | with `:no-search` | claim |
|---|---|---|---|---|
| A, root owes nothing | `tests/stdlib/algebra/24`, `26`, `tests/stdlib/slots/01`, `08`, `10`, `12`, `examples/ein-bugs/minimal-line-stated-obligation` | R {∅}, no candidate, a `:no-hypothesis` naming what no rung proposes left from the correction | R {∅}; `universe` *no candidate: the program states :no-search, so R is {∅} or empty, decided by root alone* | each held, by `ein solve`, `ein test` and `ein oracle` (the last states none) |
| B, R empty | `tests/region/04_owing`, `05_empty` | R empty, root owes 1, by Ann's two likings stored false | R empty, root owes 1, the negatives gone | held, the same claim |
| | `tests/stdlib/algebra/23`, `25` | R 3 states each, the obligation proposing what pays root's debt | R empty, root owes 1 | the pre-S1f.6.34 claim `(region (owing (model (likes Bob Stew))))` held |
| | `tests/stdlib/slots/09`, `11` | R 1 state each, three negatives keeping it one where it would be 27 | R empty, root owes 3, the negatives gone | `(owing (model (co-loc Ann S1)))` and `(… (co-loc S1 Ann))` held |
| B, root's saturation | `tests/stdlib/bijection/01`, `02`, `03` | R empty, root owes 4, by an hrule `as-stated` proposing only the `wears` root holds | R empty, root owes 4, no hrule | `(owing (model …))` of root's facts held, unchanged |
| | `tests/stdlib/slots/02`, `03`, `04`, `07`, `13` | R {root}, `slot-partition`'s fan-out written out without its two obligations | R empty, root owing 10 (`02`, `03`, `04`), 12 (`07`) or 16 (`13`), `(slot-partition co-loc …)` back | the pre-S1f.6.34 `(owing (model …))` over root's facts held |

`ein oracle` agrees with `ein solve` on all 21 (the same `k_s`), and `ein test` holds every claim; no rewritten
program prints a note (none declares an hrule or a list beside `:no-search`). The three not rewritten answer as
today.

**The mutants S1f.6.34's corrections killed.** `utils/stdlib_mutants.py --targets build/s1f.6.36/mt/tests/`, a copy
of `tests/` with the 20 rewritten programs under it (the 21st is in `examples/`), the prototype binary:
431 of 502 killed, 71 survivors, **the banked set exactly** (`tests/mutation_survivors.txt`, none new and none gone; `build/s1f.6.36/mutants-ns.json`, 2026-11-14). The six mutants S1f.6.34 banked when the bijection fixtures first let the obligations propose (`bijective-setup`'s activators of the two negative-completion rules, the elimination rules' type premises), killed again by its `as-stated` hrule, are killed by root alone with `:no-search`, as they were by the `:no-hypothesis` it replaced; the two S1f.6.34 killed (`bijective-setup`'s `total-owed` and `surjective-owed` activators swapped) stay killed. So no fixture needs the workaround for the suite's sensitivity.

#### The keyword beside each mechanism, and E-47's witness

S1f.6.34's base (`at` declared, two colours, two houses, `(near H1 H2)`), `build/s1f.6.36/w/`. Every command
(`saturate`, `solve`, `oracle`, `test`, `render program`) answers alike on each, as T2 builds the check:

| witness | declares beside `:no-search` | answer | the check |
|---|---|---|---|
| `plain.ein` | nothing, and no `:no-search` (the control) | R 4 096 states, 12 candidates | — |
| `ns_alone.ein` | nothing | R {∅}, no candidate | silent |
| `ns_o.ein` | `(total-owed at is-a)`, root owing 2 | R empty, root owes 2 (without the flag: 9 states) | silent: the intended use |
| `ns_o_paid.ein` | the same, root paying | R {∅} | silent |
| `ns_c.ein` | `(__closed__ at)` | R {∅} | silent: a closure also keeps a relation out of the read-out's `outside` row (E-52), and is often a library's own (Q-M1f.254) |
| `ns_h.ein` | `(hrule guess …)`, `:hrules (guess (at Color House))` | R {∅} | `note: ns_h.ein:9:41: solve: its :no-search states the lattice is root alone, so hrule 'guess' proposes no candidate` |
| `ns_h_o.ein` | the hrule and the obligation (the zebra form) | R empty, root owes 2 | the hrule's note |
| `ns_hr.ein` | `:hypothesis-relations (at)` | R {∅} | `note: ns_hr.ein:5:31: solve: its :hypothesis-relations filters no candidate — the program states :no-search, whose lattice is root alone` |
| `ns_nh.ein` | `:no-hypothesis (near)` | R {∅} | the same, of `:no-hypothesis` |
| `ns_o_nh.ein`, `ns_o_hr.ein`, `ns_o_c.ein` | O + NH, O + HR, O + C on `at` | refused, exit 1 (`ein test` 2) | each S1f.6.34's conflict, word for word: `:no-search` relaxes none (the stage's *must not*) |
| `ns_h_nh.ein`, `ns_h_hr.ein`, `ns_h_c.ein`, `ns_hr_nh.ein` | H + NH, H + HR, H + C, HR + NH | refused | each S1f.6.34's conflict: read as declared, as an obligation is (Q-M1f.243), so they stand (Q-M1f.254) |
| `ns_dead.ein` | a rule concluding `(false)` at root, `:expect (false)` | R empty, every node dead | silent; claim held |
| `ns_twice.ein` | `:no-search :no-search` | R {∅} | silent; a repeated flag says it twice |
| `ns_query.ein`, `ns_value.ein` | `(query … :no-search)`; `(solve :no-search (at))` | refused at the parse | *:no-search chooses the lattice and belongs on (solve …)*; *unexpected `(`* |

**E-47's witness**, re-created from today's `examples/word_problems/kinds_listed.ein` (S1f.6.35 rewrote its
imports) with the hrule `maybe-apples` and `(solve :hrules (maybe-apples) :no-hypothesis (has))`: refused for H + NH
and H + C, two lines, as since S1f.6.34. With `:no-search` beside the list: the same two. With `:no-search` in the
list's place: H + C alone, *hrule 'maybe-apples' proposes facts of has, and (__closed__ has), which rule 'words'
derives, says has is closed*. Its twin without `listed_is_all` alike. So `:no-search` does not mend E-47's program:
under the default of Q-M1f.254 an hrule is read as declared, and the program says what it means by dropping the
hrule, which is `kinds_listed.ein` itself (its `:no-hypothesis (has)`, which `words` already closes, prints S1f.6.34's
note).

#### The flag

**`ein solve --no-search`.** Its read-out (`read_out.md` § 4.2 when built): `asked`, `∅` and its `owed` block as
describe prints them, a new `R` row, `lattice` (the static check; on a flagged program its sites, `status` not known),
`strata`, `universe` (H built, *none visited*), `route` (*none*), `work` (one saturation); a dead root's core; each
query read where ∅ is in R; each claim as root decides it. On `tests/region/04_owing.ein`:

```text
solve · tests/region/04_owing.ein --no-search
──────────────────────────────────────────────────────────────
  asked             root alone — --no-search: root's fixpoint, and R is not searched
  ∅                 not in R — root owes 1 instance
    owed            likes owes Ann a Food.
  R                 not searched — --no-search builds no node past root: a bare `ein solve FILE` describes R
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate
  route             none — --no-search: root's node, and no node past it
  work              1 saturation · 0 facts beyond root

  query             (likes ?who ?what)
  over              all of R — --no-search read root alone, and root owes 1 instance, so ∅ is not in R: no state to answer from, and R was not searched

  (solve :expect)   NOT CHECKED
                    the claim is about all of R, and --no-search reads root alone: root owes 1 instance, so ∅ is not in R, and R was not searched
```

On `tests/stdlib/algebra/09_irreflexive_violated.ein`, a dead root: `∅  dead — root's own fixpoint derives (false)`,
the `unsat core (loops A A)` and `refuted by irreflexive` rows, the query `over all of R — root is dead, and a fork
only adds, so every node is (L0): no state to answer from`, `as :expect (false)`, and `(solve :expect) holds`. On
`examples/word_problems/kinds_listed.ein`, root in R: each query's class read at ∅, `(∅ alone: R was not searched
past it)`, and each claim NOT CHECKED. On `examples/zebra2.ein`, root owes 36 (four `owed` rows and the cut line), 56
candidates none visited, and `--models grid` draws what root alone deduced, the form's label *root — not in R, owes
36*:

```text
  | House | color-loc | nation-loc | drink-loc | smoke-loc | pet-loc |
  |---|---|---|---|---|---|
  | House-1 | Yellow | Norwegian | Water | Kools | — |
  | House-2 | Blue | — | — | — | Horse |
  | House-3 | — | — | Milk | — | — |
```

Root's facts are what the forms draw: `-p`, `-P`, `-f`, `--models` and `--diff-models` draw root, whatever it is,
labelled with its standing; `-H` lists H, *dead alone not known*; `--trace FILE` tells root's derivation (337 steps on
`zebra2`) and no state; `--dump-lattice DIR` writes one node. `--json-summary` writes `ein-summary/2` with
`asked.ask = "no-search"`, `region` and `verdict` `null`, the queries and the claim as checked, `work` one
saturation, and root's standing in the `root` block's three new keys, on every arm: `"dead": false, "owes": 1,
"owed": ["likes owes Ann a Food."]` on `04_owing` (Q-M1f.259). `--dump-answer DIR` writes `answer.txt`,
`answer.json`, `root.ein` (root's state, loadable, as a state file is, headed `root: not in R — owes 1`, `R: not
searched`) and `forms/`; no `index.json` or `nogoods.json`. With `-n`, `--single`, `--minimal`, `--maximal`,
`--count` or `--monotone` it is refused by the parser, exit 2 (Q-M1f.258). Every run exits 0 but a FAILED claim's,
1, as `ein solve` does.

**`ein test --no-search`.** One question, root alone, no fallback. On five corpus files:

```text
NOT CHECKED  examples/word_problems/kinds_listed.ein
  query 1 of 3 · :goal (and (asked ?q) (value ?q ?n)) — NOT CHECKED (--no-search: all of R, 1 class over 1 states)
    the claim is about all of R, and --no-search reads root alone: ∅ is in R, and R was not searched past it
  …
ok           tests/stdlib/algebra/09_irreflexive_violated.ein
  solve · :expect (false) — holds (--no-search: root alone)
NOT CHECKED  tests/stdlib/algebra/23_total_owed.ein
  solve · :expect (region …) — NOT CHECKED (--no-search: root alone)
    the claim is about all of R, and --no-search reads root alone: root owes 1 instance, so ∅ is not in R, and R was not searched
ok           tests/region/06_false.ein
  solve · :expect (false) — holds (--no-search: root alone)
NOT CHECKED  tests/region/01_states.ein
  …
5 files, 7 expectations: 2 held, 0 FAILED, 5 not checked, 0 errors  (0.07 s)
```

exit 0 (Q-M1f.257). **Which claims root decides** (Q-M1f.256): what a description stopped after root decides,
under S1f.6.27's refute rule: on a dead root every claim, as the full run, which reads root alone there too (L0); on
an alive root, `(solve :expect (false))` refuted, and where ∅ is in R a query whose class at ∅ the claim does not
list refuted; every other claim NOT CHECKED, saying whether ∅ is in R. `build/s1f.6.36/w/refute_region.ein` (∅ in R,
`(solve :expect (region))`, a query expecting `(model (at Red H1))`) shows both halves: the query FAILED, *all of R has
a class it does not list: (model) … the run read root alone (--no-search) and did not search all of R*, and the
region claim NOT CHECKED although ∅ in R refutes it: S1f.6.27's rule refutes `(false)` alone among region claims (E-57).

**`ein oracle --no-search`**: `error: ein oracle computes R by brute force, every subset of the root candidates
saturated, and --no-search declines to discover R: `ein solve FILE --no-search` reports root alone`, exit 2
(Q-M1f.258). `ein saturate`, `ein render` and `ein kb` never search, and take no flag.

#### The invariance, over the corpus

`build/s1f.6.36/sweep.py`, the 856 tracked `.ein` files, six at a time, 2026-11-14:

- **Neutral where neither is written**: `ein solve` and `ein test`, HEAD's binary against the prototype: no exit,
  no stdout and no stderr moved, on all 856.
- **`--no-search` reports root as the full run reaches it**: on the 517 programs whose full run writes a summary
  (516 that load and answer, and one register witness whose claim fails), the full run's `region.root`,
  `region.root_owes` and `region.root_owed` equal `--no-search`'s root block's `dead`, `owes` and `owed` on every one
  (381 in R, 56 owing, 79 dead, and the witness), and the two `root` blocks are byte-equal; `region` is `null` and no
  row of R's (`states (k_s)`, `commitments (k_c)`, `exhausted`, `empty`, the lines, `components`, `sub-cubes`,
  `merged`, `N`, `E`) is printed on any. The other 339 are refused at the load or by *forbid*, with and without the
  flag alike.
- **`ein test --no-search` never reports another outcome**: of every claim row, 418 held by the full run are not
  checked, 57 held are held (every one a `(solve :expect (false))` on a dead root), the one FAILED is FAILED (the
  register's `W77/goal-macro-expect-model`, a `fixed` row whose witness states the old wrong answer, refuted by root's
  class), and no outcome moves otherwise; no exit code moves (268 files 0, 587 2, 1 1).

#### Defaults taken, each the user's to revise

- **Q-M1f.254**, the conflicts beside `:no-search`: read as declared, every S1f.6.34 conflict stands, and the
  keyword adds notes only (an hrule proposes nothing, a list filters nothing); a closure beside it is silent.
- **Q-M1f.255**, what `ein solve --no-search` prints: the rows above, the `R` row *not searched* on every program, a
  dead root's included; root's facts through the forms and `root.ein`, not listed on stdout by default.
- **Q-M1f.256**, which claims root decides: what a description stopped after root decides, S1f.6.27's refute rule
  unextended.
- **Q-M1f.257**, `ein test --no-search`'s exit: a claim not checked moves no exit code under the flag.
- **Q-M1f.258**, where the flag is taken: `ein solve` and `ein test`; refused by `ein oracle`, and beside the five
  question flags and `--monotone`; every artefact and form flag over root.
- **Q-M1f.259**, the summary: `asked.ask = "no-search"`, `region` and `verdict` `null`, and root's standing as three
  keys of the `root` block on every arm.
- **Q-M1f.260**, which of the 24 are rewritten: the 21 whose question was root alone, each back to its pre-S1f.6.34
  program and claim; `tests/stdlib/algebra/23`, `25` and `tests/stdlib/slots/09`, `11` lose the R S1f.6.34 gave them.

**Finding**: E-57, S1f.6.27's refute rule refutes no region claim but `(false)`, so a partial run that met a state of
R does not refute `(region)`, `(region (owing …))` or a `(region (model …))` that lists none of its models; not a
wrong answer (not checked is no answer).

**For the user**, mostly Q-M1f.254 (whether `:no-search` should dissolve the hrule's conflicts, which would let
E-47's program say *no search* and keep its hrule), Q-M1f.257 (an exit 0 for a run most of whose claims are not
checked, by asking) and Q-M1f.260 (four fixtures go back to their first question).

**T1's commit**: the records alone (this section, Q-M1f.254 to .260, E-57, the phase README's row); the prototype
stays out of the tree until T2.
