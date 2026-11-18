# S1f.23.3 — `--diff-models` leaves `ein solve`; `ein test` shows how a claim's models differ from what it found

**Phase:** [P1f.23](README.md)
**Status:** done 2026-11-18 on the user's word, in the worktree `build/wt-s1f233` on the branch
`s1f.23.3-test-diff` (`2fd6827f`, its record `b1e66537`), which holds S1f.23.2's work and master's plan commits merged
in; **merged** into master 2026-11-18 by `34c2d79b`, with `4b152a70` (R-2's refusal texts); one gate,
`utils/ci_container.sh` and one nightly hold the phase's stages on the merged tree. Its readings are Q-M1f.454 to
.466, its findings P1f.23's [R-9 to R-13](findings.md), its (a) R-5.

## The ruling

The user, 2026-11-18, after the main session's look at `--diff-models` (it compares the states a run lists with
models written in a file of its own format, `closes …` and `model` lines around Ein facts; only `utils/review_bundle.py`
uses it, for the wrong-answer register's case pages; the language already states an expected answer as an `:expect`
claim that `ein test` checks; what only it has is the near-miss, *this state is one fact off that expected model*),
verbatim:

> Yes I agree - remove --diff-models as it is now. But near-miss diagnosis: "this state is one fact off that expected
> model" looks useful. can we add it as an option for test ? when ein test see expected models and solution result it
> can diff them (under option) and print stats and/or diffs

So:

1. **`--diff-models` goes from `ein solve`** (and every command that has it), with its file format, its form in
   `--dump-answer`'s `forms/`, its pages and decisions.
2. **`ein test` gains an option** under which, for every claim that states models (`(solve … :expect (model …))`,
   `(region (model …) … (owing (model …)))`, and a query's claim where it lists answers), it compares the models the
   claim states with the states the run found, and prints **stats** (how many match, are missing, are extra, and are
   near, with how many facts off) and/or **the diffs** (for each missing or near model, the facts it lacks and has
   beyond the state nearest it, `-`/`+`, by the one fact printer).

## Readings (defaults; put in `open_questions.md` with the next free ids, and shown on real programs)

- **The option**: one flag with two levels, `--diff stats` and `--diff full` (a bare `--diff` the stats); a name the
  CLI does not use already. It changes no verdict and no exit code: it explains, after each claim's line.
- **What is compared**: what the claim's own check compares (a claim is about what it names,
  [`13-solutions-and-models.md`](../../../docs/ein/user/03-syntax-and-semantics/13-solutions-and-models.md)), the same
  set of facts by the same printer, so a held claim's diff is all matches and a FAILED one's says why.
- **Near**: an unmatched expected model is paired with the found state of fewest differing facts, ties by the
  states' order; *one fact off* is the case the user named, and the stats count each distance.
- **Where it prints**: on stdout after the claim's line, within `ein test`'s output; `--json-report` gains the same
  under the claim, its schema's id moved.
- **What `--diff-models`' diff code becomes**: the matching and the near-miss move to serve `ein test`
  (`models.rs`' `diff`, `near`, `missing`, `spurious`), and the file format (`Expected`, `parse_expected`, `closes`)
  goes.
- **The wrong-answer review bundle** (`utils/review/case.py`, `run.py`), the one user of `--diff-models`: it writes
  the register's right answer as the claim it is, into a copy of the case's program, and runs `ein test` with the
  option; its smoke in the gate stays green.

## Acceptance

- Every claim's verdict and every exit code as before, with and without the option (a test over `tests/` and the
  corpus's claims).
- On real programs: a held claim (all match), a FAILED claim one fact off, a claim with a missing and an extra state;
  their outputs in the record.
- The pages: `05-cli.md`, `06-config.md` (regenerated), `07-diagnostics.md`, `08-read-out.md` (§ 8 without
  `--diff-models`), `10-answer-dump.md`, `ein test`'s page; the decisions that state `--diff-models` (Q-M1e2.34,
  Q-M1f.255 and any other) rewritten.
- The checks of S1f.23.2's acceptance; the gate, `utils/ci_container.sh` and the nightly once, by the main session,
  with the phase's other stages' code.

## Record

### What went

- **`--diff-models`, from `ein solve`**, the one command that had it (`ein oracle`'s call of the forms lost the
  argument it passed as `None`): the option and its help, its id in `--monotone`'s conflicts, its `--count` refusal
  (`COUNT_REFUSES` 11 → 10), the reading of its file before the run, its form in `modes::extras`, and with the form
  what only it read there (`Form::also`, the closures `what` and `also`, `what_all`, `what_shown`, `root_words`,
  `plural`); `--dump-answer`'s `forms/diff-models.txt` and the flag in `--dump-answer`'s help. `ein solve` takes 34
  options. Removed, not retired: clap refuses it as an unknown argument, exit 2.
- **Its file format and what read it**, from `ein-render/src/models.rs`: `Expected`, `ExpectedError`,
  `parse_expected`, `model_facts`, `first_loc`, `comparable_facts` (a state's facts over a file's `closes`),
  `diff_parts`, `diff_lines`, and the format's two unit tests.
- **Its tests**: `ein-cli/tests/diff_models.rs` whole; `region_modes.rs`' `diff_models_reads_a_models_facts_as_ein`,
  the diff half of `the_listed_states_print_as_asked` and the `--count` refusal row; `forms_cut.rs`' fifth form;
  `closed_stdout.rs`' `solve --diff-models` cell.

### What came

- **`ein test --diff [stats|full]`** (`cmdline.rs`, `test.rs`). After the line of every claim a run decided, the
  claim's diff: what was compared with what and over what, then the counts (`match`, `near` with how many facts off,
  `missing`, `extra`); `full` adds each model in the claim's order (`=` matched, `~` near, `-` missing with the found
  item nearest it, the differing facts under each pair, `-` the model's the found item lacks, `+` the found item's
  beyond the model) and each extra found item with the model nearest it. A claim with a diff prints its line at the
  default volume, held or not; `-q` diffs only what it prints. `--json-report` writes `diff` on every row,
  `ein-test-report/5` → `/6`.
- **What a claim's check compares**, in `ein-infer`: `claim::Compared` and `claim::Against` (states, owing,
  classes); `claim::compared`, a region claim's models against R's states or its owing frontier, each state saturated
  again with root's facts as `compare` reads it, over the relations the claim closes and the listed negatives the state
  holds; `read_out::query_compared`, a query's models against its classes.
- **The diff**, `ein-render/src/models.rs` renamed **`claim_diff.rs`** (it holds no models' form): `Level`, `Off`,
  `SetDiff` (`matched`, `near`, `missing` and `extra` each with its nearest, `unpaired`), `diff` and `diff_sets` (the
  pairing, from the old `diff_model_sets`), `lines`, `json`; kept `PAIR_TABLE_BOUND`, `mib`, `group` and `wrap`.
  Matches are a maximum matching of the pairs the check pairs (a listed negative read for its presence), so a held
  claim's diff is all matches; near and nearest are by facts off, a fact replaced counted once, and the candidate pair
  is 32 bytes where it was 24, priced as before.
- **`help_shape.rs`** renders an option whose value may be left out as `arity=0..1` with its metavar, where it read
  `arity=0` and no metavar: `--diff`'s golden line is `metavar=LEVEL arity=0..1 default=None choices=stats|full`. No
  other option has such a value; only the three intended golden lines moved.
- **Tests** (`ein-cli/tests/test_cli.rs`): `corpus_run`, the corpus's `ein test examples tests stdlib -v
  --json-report` run once per test binary and shared with `every_corpus_claims_scope_lines_are_its_rows`, so the
  invariance costs one corpus run more; `the_diff_moves_no_verdict_and_no_exit_code` (stdout under `-v --diff full`
  less its diff blocks is stdout without it, the time aside; every row the same but for `diff`; every decided claim has
  a diff but the region's `(false)`; a held claim's is all matches; a FAILED one's disagrees or its `(merged …)` is
  why; each kind of claim and a listed negative met); `a_claim_one_fact_off_reads_as_one_fact_off`,
  `a_region_claim_names_the_missing_and_the_extra`, `quiet_diffs_only_the_claims_it_prints`,
  `a_bare_diff_before_a_path_is_refused_with_its_levels`; `closed_stdout.rs`' `test --diff` cell; nine unit tests in
  `claim_diff.rs`; `ein-render/tests/diff_declines.rs` on the new names, each side of the bound as before.
- **Pages**: `05-cli.md` § 2.3 (synopsis, the option with a real output, `knights.ein` one fact off) and § 4.4 and § 5
  (the report's `diff` row, `/6`); `06-config.md` (the parser table 34 and 11, the total 90 unmoved; § 3.3's
  presentation class: four forms, `--diff` and its bound; `/6`; the generated block did not move);
  `07-diagnostics.md` (the `open` passage); `08-read-out.md` § 4.2, § 7 (a pointer to `ein test --diff`) and § 8 (the
  row); `10-answer-dump.md`; `12-query.md` (a sentence where `ein test` is told); `01-architecture.md`'s `ein-render`
  row and its module diagram (redrawn: `claim_diff.rs`, fed `Compared`, writing `ein test --diff`'s block and the
  report's diff); `08-reasoning-in-code/02-saturation-and-negation.md` § 6.1 to § 6.3 and
  `03-hypotheses-and-the-answer.md` ($\operatorname{model}(C)$ is `-P`'s, `factdump.rs`); `AGENTS.md`'s command
  list; `examples/README.md`'s generated anchor list (re-banked).
- **Decisions**: Q-M1e2.34 and Q-M1f.255 lose the flag from their lists; Q-M1f.223 named `ein-test-report/5` and now
  names no version. `git grep -- --diff-models` finds nothing outside `plans/`.
- **The process**: `utils/review/case.py` writes the register's right answer into a copy of the witness, `right/NAME`,
  as one `(query …)` per relation of `closes` over the line the row's run asks (`:expect` that relation's facts in each
  right model), and `run.py` runs `ein test COPY --diff full --json-report raw/test-report.json` as the `diff`
  invocation, its stdout verbatim on the case page's § 4 and the run page; the register pass writes every row's claims
  with no run; cross-check 1, which compared the old second solve's summary with the main one, is *the right answer's
  claims are over as many states as this run lists*, where both are whole; `FORM_DIRS` lost `diff`, `verdict_block`
  the old block's head. `utils/README.md`'s bundle row; `check_tool_mirrors.py`'s floor of path-valued `ein solve`
  options, 6 → 5.

### Shown on real programs (`target/gate/ein`, at this stage's commit)

**Held, all match**: a region claim, one with a listed negative, and an owing claim.

```text
$ ein test examples/lattice/14_seats_nothing_proposes.ein examples/features/10_expect.ein tests/region/04_owing.ein --diff full
ok           examples/lattice/14_seats_nothing_proposes.ein
  solve · :expect (region …) — holds
    diff: 4 models against R's 4 states, compared over sits
      4 match, 0 near, 0 missing, 0 extra
      = model 1 is state 1
      = model 2 is state 2
      = model 3 is state 3
      = model 4 is state 4
ok           examples/features/10_expect.ein
  solve · :expect (region …) — holds
    diff: 1 model against R's 1 state, compared over next-to and the presence
        of 1 listed negative
      1 match, 0 near, 0 missing, 0 extra
      = model 1 is state 1
ok           tests/region/04_owing.ein
  solve · :expect (region …) — holds
    diff: 1 model against the owing frontier's 1 node, compared over likes
      1 match, 0 near, 0 missing, 0 extra
      = model 1 is owing node 1
3 files, 3 expectations: 3 held, 0 FAILED, 0 not checked, 0 errors  (0.04 s)
exit 0
```

**A query claim that lists answers, FAILED one fact off**: `examples/binary_logic/knights.ein` with its second claim's
second answer written `(in C 1)`.

```text
$ ein test knights.ein --diff full
FAILED       knights.ein
  query 1 of 2 · :goal (in C ?c) — holds
    diff: 1 model against the 1 class of all of R
      1 match, 0 near, 0 missing, 0 extra
      = model 1 is class 1
  query 2 of 2 · :goal (and (in A ?a) (in B ?b) (in C ?c)) — FAILED
    the claim lists 2 classes, the query has 2: (model (in A 1) (in B 0) (in C 0)) is not listed
    (model (in A 1) (in B 0) (in C 0)) is a class the claim does not list
    the claim lists (model (in A 1) (in B 0) (in C 1)), which is no class
    diff: 2 models against the 2 classes of all of R
      1 match, 1 near (1 fact off), 0 missing, 0 extra
      = model 1 is class 1
      ~ model 2 is class 2, 1 fact off
          - (in C 1)
          + (in C 0)
1 file, 2 expectations: 1 held, 1 FAILED, 0 not checked, 0 errors  (0.02 s)
exit 1
```

Its report row's `diff`: `{"against": "classes", "exact": true, "over": [], "presence": [], "models": 2, "found": 2,
"match": 1, "near": 1, "missing": 0, "extra": 0, "near_off": [{"off": 1, "near": 1}], "unpaired": null, "items":
[{"kind": "match", "model": 1, "found": 1, "off": 0, "lacks": [], "has": []}, {"kind": "near", "model": 2, "found": 2,
"off": 1, "lacks": ["(in C 1)"], "has": ["(in C 0)"]}]}`, schema `ein-test-report/6`.

**A region claim with a missing state and an extra one**, beside a match and a near-miss:
`examples/lattice/14_seats_nothing_proposes.ein` with its second state written three facts off and its fourth one
fact off.

```text
$ ein test seats.ein --diff full
FAILED       seats.ein
  solve · :expect (region …) — FAILED
    R's states and the listed models do not match one to one
      a state of R no model lists: (model (sits Ann S1) (sits Bob S2) (sits Cat B2) (sits Dan B1))
      a state of R no model lists: (model (sits Ann S2) (sits Bob S1) (sits Cat B2) (sits Dan B1))
      a listed model no state of R is: (model (sits Ann B1) (sits Bob S2) (sits Cat S1) (sits Dan B2))
      a listed model no state of R is: (model (sits Ann S2) (sits Bob S1) (sits Cat B2) (sits Dan B2))
    diff: 4 models against R's 4 states, compared over sits
      2 match, 1 near (1 fact off), 1 missing, 1 extra
      = model 1 is state 1
      - model 2: no state is it; the nearest is state 1, 2 facts off
          - (sits Ann B1)
          - (sits Cat S1)
          + (sits Ann S1)
          + (sits Cat B1)
      = model 3 is state 3
      ~ model 4 is state 4, 1 fact off
          - (sits Dan B2)
          + (sits Dan B1)
      + state 2: no model is it; the nearest is model 1, 2 facts off
1 file, 1 expectation: 0 held, 1 FAILED, 0 not checked, 0 errors  (0.02 s)
exit 1
```

`--diff stats` (or a bare `--diff` after the paths) prints the two head lines alone; without the option the file
prints its claim lines as before and exits 1 alike. A query claim refuted from a part, `tests/region/08_query_classes.ein`
with `(model (a-prop X) (a-prop Y))`: `diff: 2 models against the 2 classes of the states found, a part of R`,
`~ model 2 is class 2, 1 fact off` / `- (a-prop Y)`.

**The wrong-answer bundle**, `python3 utils/review_bundle.py W8/features-07 W8/fifo-gap W8/fifo-gap-root`, the three
rows whose right answer states models: each case page's § 4 shows the wrong fact one fact off the right model, e.g.
W8/features-07's copy ends with `(query :goal (p ?a1) :over maximal :expect (model))` and `(query :goal (q ?a1) :over
maximal :expect (model (q one)))`, and its block reads `query 1 of 2 · :goal (p ?a1) — FAILED` … `~ model 1 is class
1, 1 fact off` / `+ (p one)`, the second claim held; cross-check 1 passes on all three.

### Readings (defaults; the main session files them)

1. **The option's spelling** is the stage's, `--diff stats|full`, clap's optional value: a bare `--diff` is `stats`
   where no word follows it, and before a path it takes the path for its level and is refused, exit 2, naming the
   two levels (pinned by a test). *Default: as built.* The alternative, `--diff[=LEVEL]` (`require_equals`), takes a
   bare `--diff` anywhere but reads `--diff full` as the path `full`.
2. **Facts off is an edit distance**: a fact replaced counts once, `max(lacks, has)`, so the user's *one fact off*
   includes a wrong value, `(in C 1)` for `(in C 0)`; pairs are taken fewest facts off first, then fewest facts
   differing, then the found items' order and the models'; near only while at most half the larger off, at least
   one. *Default: as built*; the alternative is the symmetric difference, where a wrong value is two facts off and a
   one-fact model never pairs near.
3. **Matches are what the check pairs**, a maximum matching with a listed negative read for its presence, so a held
   claim's diff is all matches and a region claim listing a model twice against one state shows one match and one
   missing. *Default: as built.*
4. **Which claims have a diff**: every claim a run decided, held or FAILED, but the region's `(false)`, which lists no
   model and is about root. `(region)` has one over no relation (every state extra, each `(model)`), a query's
   `(false)` one whose every class is extra; a NOT CHECKED claim has none, what its run found not being what it is
   about. A query claim refuted from a part has one, marked *a part of R*. *Default: as built.*
5. **Missing and extra**: a missing model is shown against the found item nearest it among all found (matched ones
   too) with its facts; an extra found item names the model nearest it and how far, without facts (the user named
   the missing and near models' facts; an extra state's facts are the read-out's). Past the pair table's bound no
   nearest is sought. *Default: as built.*
6. **An `(owing …)` claim is compared with the owing frontier**, its nodes numbered in the read-out's order, as the
   forms number them (*owing node i*); where R is not empty the frontier is empty and every owing model missing.
   *Default: as built.*
7. **`(merged …)` is not diffed**: the diff is the models; a claim FAILED by its `(merged …)` alone has an all-match
   diff, and the claim's own lines say why. *Default: as built.*
8. **Where it prints**: under `--diff` a claim with a diff prints its line at the default volume, held ones too; the
   diff follows the claim's report lines and precedes the warning and *outside the universe* lines; `-q` diffs only
   the claims it prints. *Default: as built*; the alternative, `--diff` overriding `-q`.
9. **The report**: `diff` on every row, `null` without the option, on a claim no run decided and on the region's
   `(false)`; one shape on every arm, indices 1-based as printed, `items` `null` under `stats`; the id moves to
   `ein-test-report/6`. *Default: as built.*
10. **The bundle's right answer is written as query claims, one per relation of `closes`**, over the line the row's
    run asks, not as a region claim: a region claim closes only the relations its models name (fifo-gap-root's `p`,
    named by no right model, is the row's wrong fact) and is about all of R, where two of the three rows ask
    `--maximal`. Where `right` states more than one model the claims state each relation's facts across them and not
    which go together; the three rows that state models state one each. A witness that imports a module of its own is
    not copied, and its page says so (no open row does). *Default: as built.*
11. **The run page's cross-check 1** compares the `ein test` run's claims, where whole, with the run's listed states
    by count; the summary comparison it made was the old second solve's. *Default: as built.*
12. **The module is `claim_diff`** (S1f.23.2's reading 6 offered `diff.rs`): it holds a claim's diff and no models'
    form. *Default: as built.*
13. **`--diff-models` is removed, not retired**, as S1f.23.2's reading 2 for `--models`. *Default: not retired.*

### Findings (for this phase's `findings.md`, R-n, on merge)

- **(a) The link check in a fresh checkout, known**: `utils/doc_audit.py --links --check --tracked` reports 2 961
  findings, every one under `plans/m1f_hypothesis_and_documentation/review/`, the READMEs linking a run's ignored
  outputs (S1f.23.2's finding (a)); its decision-id pass, 0.
- **(b) S1f.23.2's finding (b) has no part in `utils/`**: every item it lists is under `plans/`, and `git grep --
  '--models\|models grid\|models key' utils` finds nothing. The `utils/` references to `--diff-models` were this
  stage's own (`review/run.py`, `review/case.py`, `README.md`, `check_tool_mirrors.py`'s floor), and are rewritten.
  What still names `--diff-models` is process, left as written: `open_questions.md` (lines 2357, 5549 and 5598, records
  of rulings), `followups/f23_diagnostics_in_eins_words.md` (D-20), P1f.23's README
  (this stage's row), S1f.23.2's record.
- **(c) The report's id and Q-M1f.412**: Q-M1f.412's reading (b) moves the report to `ein-test-report/6` for dropping
  `types_as_objects`; this stage took `/6` for `diff`. Whichever lands second moves it to `/7`.
- **(d) A bless race in `help_surface.rs`**: `the_lattice_views_are_what_their_help_says` reads the golden
  `the_argument_surface_is_stable` writes under `EIN_BLESS=1`, in one test binary on parallel threads, and failed once
  here under bless; without bless it is no race. `EIN_BLESS=1 … --test help_surface the_argument_surface_is_stable`
  blesses alone.
- **(e) `help_surface.rs`' doc count was stale**: *67 options across 12 parsers*, which were 73 before S1f.23.2; it
  says 70 now, the sum it asserts.
- **(f) The bundle's smoke runs no row that states models**, so the per-commit gate runs `ein test --diff` on no right
  answer; the register pass now writes every row's claims with no run, and the three rows were built by hand above.

### Checks (at `2fd6827f`, in the worktree)

- `cargo fmt --all -- --check`, `cargo clippy -p ein-infer -p ein-render -p ein-cli -p ein-corpus --all-targets -- -D
  warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` on the same four: clean.
- The crate tests touched, each alone first: `claim_diff` (9), `diff_declines` (2), `help_surface` (4, the golden
  re-banked alone), `config_reference` (9, the generated block unmoved), `world_anchors` (2, re-banked), the five
  `--diff` tests of `test_cli`.
- `cargo test --profile gate -p ein-infer -p ein-render -p ein-cli`, once, with `TMPDIR` a fresh directory under
  `build/`, left empty: exit 0, 132 test binaries, 1 006 tests passed, none failed or ignored;
  `the_diff_moves_no_verdict_and_no_exit_code` among them over every corpus claim. Run when no other stage's test run
  was going; before it, `the_diff_moves_no_verdict_and_no_exit_code` had run once beside other stages' runs, by a
  name filter, and failed on `region.ms`, a time, which the comparison now leaves out.
- `python3 utils/review_bundle.py --smoke`, as the gate runs it (`target/gate/ein`): exit 0, the register pass 16
  rows and 0 failed, 2 cases and 1 program, 25 SVGs, link check exit 0. The three rows that state models, by hand:
  above.
- `utils/check_doc_blocks.py`: ok, rules 1–5. `utils/check_generated.py --bin ein.rs/target/gate/ein`: 9 generators,
  every file as written. `utils/check_release_voice.py`: ok. `utils/check_gate_steps.py`: ok.
  `utils/check_tool_mirrors.py`: ok, its floor at five. `utils/render_diagrams.py --check`: the one diagram this stage
  edited, drawn.
- `utils/doc_audit.py --links --check --tracked`: exit 1, 2 961 findings, every one finding (a)'s; none in a file
  this stage touched; the decision-id pass, 0.
