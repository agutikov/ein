# S1f.6.36 — Judging root without a search: `(solve … :no-search)` and `--no-search`

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-14** ([§ Record](#record-2026-11-14)); planned 2026-11-14, on the user's words of 2026-11-13
and 2026-11-14. T1 shown on a prototype and stopped for the coordinator (`2b9ea84a`), accepted as recorded the same
day, defaults Q-M1f.254 to .260 kept; T2 to T5 built it. **An engine stage and a language addition.**
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

### T1f.6.36.2 and .3 — the keyword and the flag, done

The prototype of T1 (`build/s1f.6.36/proto.patch`), applied and finished; T1's coordinator review accepted Q-M1f.254
to .260 as written.

**The keyword.** `ein.l` and `ein.y`: `:no-search`, a flag, its pair holding `()`; `grammar.rs`: a lattice keyword
(on a `(query …)` refused with the note the other three get), and a value after it refused with the note *:no-search
is a flag and takes no value: it states that the lattice is root alone*; `dump.rs`: a flag prints without its value
(`FLAGS`), so a program read back, a `.einb`'s stored text, an import-invariance rewrite and every `--dump-answer`
state file load as written (the first full run found all four refusing `:no-search ()`). `hypgen::declares_no_search`
and `NO_SEARCH`: under it the ladder runs no rung, before anything is compiled, `oblgen::Mode::NoSearch`
(`no-search`), never narrated; `eligible_relations` is empty; `scope::outside` names nothing; `from_ir` reads no
commitment's facts (S1f.7.4's W84 check: a rule concluding `(not ?x)` over a relation the blind enumerator would
guess is refused without the keyword and loads with it). `mechanisms`: `Mechanisms::no_search`, `may_conflict` counts
it, and `judge` writes, after the conflicts, which stand, one note beside the hrules and one beside each list, in
place of S1f.6.34's notes; silent beside an obligation or a closure. The read-out's `universe` row on such a program:
*no candidate: the program states :no-search, so R is {∅} or empty, decided by root alone*.

**The flag.** `explore::Options::no_search`, `Route::Root`, `Stopped::NoSearch` and `root_alone`: root's node through
`Space::eval`, a description of what root alone establishes (`partial_cube`), exhausted on a dead root (L0) and partial
otherwise. `region::RootStats` and the summary's `root` block carry root's standing on every arm, `dead`, `owes`,
`owed`. `ein-render`: `Row::R`, the `no-search` table, `render_root`; the query block and the claims' lines say *root
alone*; `region_answer::write_root` (`answer.txt`, `answer.json`, `root.ein`, `forms/`) and `summary_blocks`
(`asked.ask` `no-search`, `region` `null`). `ein-cli`: the option on `solve` (refused beside `-n`, `--single`,
`--minimal`, `--maximal`, `--count`, `--monotone`) and on `test` (one question, no fallback; a claim not checked moves
no exit code); `ein oracle --no-search` refused with its sentence (`oracle::REFUSED`); the forms draw root.

**Tests.** New: `ein-cli/tests/no_search.rs` (the oracle and the question flags refuse it; the read-out on a root in
R, an owing one and a dead one, no row of R's; the summary and the dump, `root.ein` loading back; `ein test`'s
decisions and exits; no commitment binds a name under the keyword); `lever_invariance.rs`'s
`no_search_reports_root_as_the_full_run_reaches_it_and_states_no_r` (every program `programs()` holds, at least 400
compared: a refusal alike, root's standing and `root` block equal to the full run's, `region` null, no R row, no claim
outcome but the full run's or not checked); `mechanism_conflicts.rs`'s `no_search_relaxes_no_conflict` (each of
S1f.6.34's eleven refused shapes, `:no-search` beside it, the same line on `solve`, `test` and `saturate`) and
`no_search_is_silent_beside_an_obligation_and_a_note_beside_what_it_makes_dead` (nine shapes); `grammar_pages.rs`'s
table, two rows; `dump.rs`'s `a_flag_prints_without_its_value`. Moved: `obligation_rung.rs` (its stuck and its
discharged fixtures are programs of its own, the two corpus files stating `:no-search` since; `04_owing` is the
ladder's case of no rung at all), `obligation_reports.rs` (an owing fixture's R is empty, root its owing node, as
before S1f.6.34), `help_surface.rs` (35 and 10 options, 84 in all), `read_out_reference.rs` (the `no-search` table),
`fuzz_properties.rs` (`solve`'s list), and the four matches over `Route` and `Stopped` in tests and examples.

### T1f.6.36.4 — the corpus, done

`build/s1f.6.36/t4_rewrite.py`: the 21 programs of Q-M1f.260 back to their text before S1f.6.34 (`e45c4c18^`), the
`:no-hypothesis (…)` replaced by `:no-search`, and a comment above each `(solve …)` saying what it meant, what S1f.6.34
refused and which workaround the keyword replaced; no `as-stated` hrule, no fan-out written out, no added negative
remains. `ein test tests/` holds all 249 expectations, and every claim of the 21 holds under `ein solve`, `ein test`
and `ein oracle`.

**Goldens, counted before blessing** (`EIN_BLESS=1`, 2026-11-14):

- `corpus_shapes.md5`: **506 digests moved, none new and none gone, over exactly the 21 programs** (13 to 39 each:
  their text, so `ir[…]`, `load` and the IR views, 21 each; `hyp` and `hyp+closed`, the rung `no-search`, 21 each;
  the read-out and the region trace, the `universe` row, 21; and where what a run visits moved, the modes' read-outs
  and cores on 13, the oracle on 11, the states and the lattice dump on 9). No other program moved, so the stage is
  behaviour-neutral outside the keyword and the flag.
- `corpus_exits.txt`: one cell, `tests/stdlib/bijection/01`'s `render rules`, 0 to 1: with its `as-stated` hrule gone
  the file holds no rule form again, as before S1f.6.34 (*no rule forms*), the cell S1f.6.34 moved the other way.
- `trace_owing.md` (`04_owing`'s trace): one line, the `universe` row naming `:no-search`.
- `grammar_table.txt`: two rows added (389, 390), the query's `:no-search` and a value after the flag.
- `help_shape.txt`: two options added.
- `docs/kernel/ir/03-ein-lang/00_ebnf.md` (blessed): one token and one production.
- `read_out_examples.md` (`utils/read_out_examples.py`): X18 and X19 added; X6, `04_owing`, two lines and its prose
  (the `universe` row; `--single`'s route the state route where it was the obligation tree, the state no longer
  branching on Ann's two refuted candidates).

**The mutants** (`utils/stdlib_mutants.py --check`, the release binary of this stage): 431 of 502 killed, 71 survivors, the banked set, exit 0 (`build/s1f.6.36/mutants-check.txt`), as T1's prototype measured it.

**The invariance** (the user's rule of 2026-11-14, a change shown right by two equivalent forms): each rewritten
fixture's claims hold under describe, `ein test` and the oracle; `--no-search`'s root equals the full run's on every
corpus program (`lever_invariance.rs`); `:no-search` beside each refused shape refuses with the same line
(`mechanism_conflicts.rs`); and no corpus digest moved outside the 21.

### T1f.6.36.5 — the pages and the records, done

- `01_grammar.md`: the token table (a keyword is followed by its value, but `:no-search`), the form table, the
  keyword table's row, § Solve's form and two paragraphs (the flag; it relaxes no conflict).
- `06_reserved_names.md`: the `solve` row, a `no-search` row, the query keywords' paragraph, the mechanisms table's
  `NS` row and *`:no-search` beside each*, the `Open` paragraph's fixtures.
- `defined_behaviour.md`: § 4.27's paragraph and pins; § 4.7's commands *forbid* refuses (`--no-search` too); § 5's
  exit table and paragraph.
- `configuration.md`: the counts (84; 35 and 10), the header's history line, § 3.3's class *The question not asked*
  with *does it change the answer: no — it declines to discover it*, § 2.5's last paragraph.
- `read_out.md`: § 4 is `--monotone` and `--no-search`, § 4.2 and its table, § 10's two stderr rows;
  `read_out_examples.md` regenerated; `answer_dump.md`: the `root` block's three keys and `--no-search`'s directory.
- `tests/README.md`: the idiom, *to judge root alone, write `(solve :no-search)`*.
- `AGENTS.md`: the hypothesis-mechanisms bullet (`:no-search`), the answer bullet and the lever bullet
  (`--no-search`).
- Records: Q-M1f.246 restated; the phase README's row; this record.

### The stage, closed

**The commits**: T1, the records, `2b9ea84a`; T2 to T5, the keyword, the flag, the corpus, the tests, the goldens,
the pages and this record, in the commit that follows it.

**The acceptance, point by point.**

- *A program with obligations and `(solve :no-search)` loads, proposes nothing, and is judged at root; the 24 fixtures
  say what they meant with it, and none carries a workaround*: the 21 that meant root alone state it, R root or empty
  as each claims; `bijection/06` means the blind enumerator, `closure/02` and the refusal's fixture are O + C, which
  the keyword does not relax (Q-M1f.260).
- *`--no-search` changes no R: on every corpus program, what it reports of root is what the full run reaches at root,
  and it states no R*: `lever_invariance.rs`'s new test, and T1's sweep (517 programs).
- *Every page that lists the `solve` keywords or the CLI's options lists the two*: the grammar's three tables and §
  Solve, `06_reserved_names.md`, `configuration.md`'s counts and § 3.3, `defined_behaviour.md` § 5, `read_out.md`,
  `AGENTS.md`.

**What it must not do**: `--no-search` states no R and is no lever (the new test); `:no-hypothesis` over an owed
relation stays refused, beside `:no-search` too (`no_search_relaxes_no_conflict`).

**E-57 and the NOT CHECKED count.** `ein test --no-search` leaves more claims NOT CHECKED than root alone could
decide: a state of R that root is refutes `(region)`, `(region (owing …))` and a `(region (model …))` that does not
list it, and S1f.6.27's rule, which this stage keeps (the coordinator, out of scope), refutes none of them. E-57 stays
filed, no owner.

**Checks run, 2026-11-14**, on this tree: `cargo test --release --workspace` whole, 140 test binaries and doc-test runs, all green after the goldens above (the one run before them failed where T1 said it would, the pages' tests and the goldens, and found the flag printed with its `()`, fixed in `dump.rs`); `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` (all in release), clean; the gate's link check over its paths, 558 pages, 0 findings; `utils/stdlib_mutants.py --check`, exit 0; `examples/gen_zebra2_variants.py --check` and `utils/stdlib_manifest.py`, unchanged. Not run: `./run_tests.sh` and the nightly, the coordinator's, over the batch.
