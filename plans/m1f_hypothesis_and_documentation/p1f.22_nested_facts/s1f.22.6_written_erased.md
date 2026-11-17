# S1f.22.6 — `written` erased; a numeral is checked by the operation that takes it

**Phase:** [P1f.22](README.md)
**Status:** **done 2026-11-18**: `bf9693dc`, `ec9049b5`, `dad4543f`; the gate, the batch, the nightly and
`utils/ci_container.sh` exit 0 on `ec9049b5`; Q-M1f.405 to .411 put; findings N-33 to N-35. Created and started
2026-11-18 on the user's word (below). It runs before
[S1f.22.5](s1f.22.5_save_and_load.md)'s tasks 2 to 4, whose work in progress is paused on the branch
`s1f.22.5-save-load` (`db562a86`) on the same word; S1f.22.5 resumes when this stage is done and the multi-context
drawings are drawn again.
**Ruling:** [Q-M1f.404](../open_questions.md#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it).

## The instruction

The user, 2026-11-18, verbatim, after asking of `written` *"WTF is written ctx???"* and *"When it was added and what
for?"*, and being told it is Q-M1f.272's loader step (E1, `f555c7e0`, 2026-11-14), made for Q-M1f.210:

> revert this decision, completely erase this `written` crutch. insert malformed numbers detection into decimal and
> binary arithmetic operations. all numbers taken from input program will be checked once and labeled valid, and
> numbres produced by operations has to be labelled valid already. put current running stage into branch and pause
> its work, first remove `written` and then again draw multi-context examples and continue p1f.22.5

## What it reverts

- **Q-M1f.272** (a representation module reads what a file writes through `written`), **Q-M1f.275** (`written`
  states every fact a file nests), **Q-M1f.279** (a written fact is derived by the loader's step) and
  **Q-M1f.280** (a program does not write `written`): erased. Their decision files go, and every place that cites
  one cites Q-M1f.404 instead (an id is kept and never reused; a superseded id still cited is rewritten to the id
  that carries the decision).
- **Q-M1f.210**, *flagged where it is written, with no calculation reading it*: that half is reverted. A malformed
  number is found when an operation takes it. **Q-M1f.276**, the check of an integer's and a rational's own shape
  over `written`: the same.
- What other decisions and pages say of `written`: Q-M1f.391's *the loader's written step reads through every
  negation*, Q-M1f.392's cause, the reasoning's pages that call `written` *a context the loader makes*, the
  reserved name and its refusal (`07-diagnostics.md` § 2.30), the KB view's `written` octagons and the `written` in
  a frame's label (finding N-23, which this ends).

## The design the ruling gives, and the readings taken

The ruling's sentences, and how this stage reads each. Each reading is a default, put as a question
(Q-M1f.405 onward) with its alternatives, and stands until the user revises it.

1. **"all numbers taken from input program will be checked once and labeled valid"**: a numeral an operation
   takes as an operand, and that no operation produced, is checked by the representation's rules and gets the
   label *valid*, or is flagged malformed, and the flag derives `(false)` as today. *Once*: the label is a fact,
   so each numeral, each place of it, is checked once whatever reads it.
2. **"numbers produced by operations has to be labelled valid already"**: an operation labels its result valid as
   it produces it, and a result is never checked.
3. **A numeral no operation takes is not checked.** Examples are a numeral in a `let` nobody asks about, in a fact
   of the program's own relation, or in a rule's conclusion that never fires. This is the consequence of putting
   the detection into the operations. The programs that hold such a numeral to be flagged
   (`tests/stdlib/math/decimal/02_in_a_let_nobody_asks_about.ein`, `03_in_a_fact_of_the_programs_own.ein`,
   `04_in_a_rules_conclusion.ein`, and binary's twins) are changed to say that it is not, or deleted.
4. **Where the rules live**: what a well-formed numeral is, and the label, stay the representation's
   (`std.math.decimal`, `std.math.binary`), as Q-M1f.210 placed them. The operations of `std.math.decimal-arith`
   and `std.math.binary-arith` ask for the check of an operand and label their results.
5. **`std.math.int` and `std.math.rat`**: the ruling names decimal and binary. Their checks of an integer's or a
   rational's own shape also read `written`, and erasing it means they move too. By the same principle, they move
   into the operations that take an integer or a rational, with the same label.
6. **The standing rule *no rule of the calculator joins two facts on a number*** stands. The design keeps it,
   for instance by checking an operand where its request is read and carrying the label with the request's tag
   (an operand's fault travels with its tag, Q-M1f.206). If no design keeps it, the stage stops and puts that to
   the user.

## Tasks

1. **The census and the design**, short, in this file's Record. The census covers every place `written` is:
   engine, standard library, `tests/`, `examples/`, `corpus/`, `docs/`, decisions, `utils/`, `AGENTS.md`. The
   design covers the label's form and how each operation of each base reads and writes it. It is shown on real
   programs: a well-formed numeral, a malformed one an operation takes, a malformed one nobody takes, and a result
   fed to a second operation. The readings above go into `open_questions.md` as Q-M1f.405 onward, each with a
   **Default**.
2. **The engine**: the loader's `written` step, the reserved name and its refusal, the internal rule name
   `#written`, and every reader. That includes the KB view's `written` octagons, the state writer's leaving out of
   `written`, the alive invariant's banked lists, and the narration.
3. **The standard library**:
   - the four modules' checks, and the operations of both bases and of `int` and `rat`;
   - the closing rules' lists, with their tests (`arith_theory.rs`, `int_theory.rs`, `rat_theory.rs`,
     `num9_theory.rs`, `binary_theory.rs`);
   - `MANIFEST.sha256`, and the `04-stdlib/` rule pages through their generator;
   - the programs under `tests/stdlib/`, the mutation survivors, and the census.
4. **The pages and the decisions, in the same commits.** `AGENTS.md`'s lines on `written` are edited in place.
5. **Verification**, once, on the last commit:
   - `./run_tests.sh`;
   - the calculator's batch by hand, `cargo test -p ein-infer --features counters --test arith_batch`;
   - the nightly, since this is a large engine and standard-library change;
   - `utils/ci_container.sh`, since the KB view's DOT writer changes.

## Acceptance

- No relation `written` is left in the project. `git grep -n -w written` is shown with every line it still prints,
  each of them the English word.
- Every corpus program's answer is as before, except the programs whose malformed numeral no operation takes
  (reading 3). Each of those is named, with its answer before and after.
- `./run_tests.sh`, the nightly, the batch and `utils/ci_container.sh` exit 0 on the last commit.

## After it

The multi-context drawings in [`diagrams/`](diagrams/) (`render-held-in-two-contexts`,
`render-two-contexts-of-one-relation`, and S1f.22.4's `render-two-contexts`) are drawn again. Then S1f.22.5 resumes:
its branch merges master and continues, in a worktree, and merges back into master.

## Record

### Record (2026-11-18)

**Taken over** from an earlier agent stopped before it committed: its uncommitted edits to `std.math.decimal`,
`std.math.binary`, `std.math.positional` and both bases were kept as the design's core (the ask `(check N N)`, the
walk, the label `(valid N)`, `operand-check`), and changed in two places: the label of a number an operation makes
moved from the five public answer rules to the end of the walk that makes it (`turn-end`, `div-end`), and `z` is
labelled by the closing rule. `int`, `rat`, `convert` and `num9`, the engine, the tests and the pages are this
stage's.

#### Census: where `written` was (master at `0a4a97a3`)

`git grep -n -w written 0a4a97a3 -- . ':!plans' | wc -l` prints 4 174, nearly all the English word. The relation's own:

| where | what |
|---|---|
| `ein-ir` | the loader's step: `state_written`, `written_in`, `written_in_conclusions` (exported), `nested`, `denied`, `ground_nested`, `is_ground`; the refusal of a fact, a negation and a rule's or hrule's conclusion of `written` (`refuse_written_facts`, the rules pass); `ingest_fact` returning the fact's id for the step; a unit test |
| `ein-core` | `terms::WRITTEN`, `WRITTEN_STEP` (`#written`), the kernel's two symbols, `RESERVED` (12 names), `text::INTERNAL_NAMES` (6), `Program::categorise` |
| `ein-infer` | `structure.rs`: the small KB's `Loaded` stated `written` again (`conclusions`, `over`), `Origin::Written` in `h-space.json`'s `stated`; `unheld.rs`: the kernel relations counted written |
| `ein-render`, `ein-cli` | a model's facts and `comparable_facts`, the state file writer and its header line, `kb_to_ein_text`; `factdump.rs`. The KB view drew `written` facts as any fact (its octagons, `written` in a frame's label, N-23) |
| engine tests | `written_facts.rs` (deleted); `answer_round_trip.rs` (held `written` apart as a superset); `negation_of_a_negation.rs` (one test of what the loader states); `every_object_once.rs` (`says, think, written ctx`); `ir_semantics.rs`, `arith_batch.rs`, `alive_invariant.rs` comments; `int_theory.rs`, `rat_theory.rs` (the shape rules over `written`) |
| standard library | twenty rules read it: `std.math.decimal`'s four flags, `std.math.binary`'s four, `std.math.int`'s six shape rules, `std.math.rat`'s six |
| `tests/stdlib/` | `decimal/01` to `05`, `binary/01` to `06`, `decimal-arith/22` to `28`, `31`, `32`, `int/15` to `18`, `23`, `29` to `34`, `rat/14` to `16`, `24`: comments; `decimal/02` to `04` claimed `(false)` |
| `examples/`, `corpus/` | `beliefs/01_common_belief.kb.dot` and `02_packing.kb.dot` (`written` octagons); four notes |
| pages | `07-diagnostics.md` § 2.30; `03-reserved-names.md`; `02-syntax.md`; `03-facts.md`; `02-relations.md`; `14-natural-language.md`; `10-answer-dump.md`; `10-theory.md`; `12-rendering.md` and three KB drawings; the reasoning's knowledge base (`written` as *a context the loader makes*), rewriting rules, saturation, hypothesis generation, cores; `ein.rs/` architecture, data model, IR front end (the load path's step 8, § 9.2), reserved engine strings, the three pages of `08-reasoning-in-code/`, four drawings; the standard library's pages and rule pages; `stdlib/README.md`, `tests/README.md` |
| decisions | Q-M1f.272, .275, .279, .280 whole; Q-M1f.210 and .276 the half *flagged where it is written*; Q-M1f.391's sentence; mentions in Q-M1f.274, .295, .316, .339, .353, .395, .397 |
| process | `AGENTS.md` (three places), `utils/nl_census.py`'s `KERNEL` |

#### The design, as built

- **The check is the representation's, and an operation asks for it.** `std.math.decimal` declares `check` and
  `valid`; `(check N N)` asks, `numeral-step` and `numeral-step-zero` walk a place a firing from the units as
  `(check N REST)`, `numeral-valid` labels `(valid N)` where the walk reaches `z`, and the four flags read the walk:
  a malformed place stops it and is flagged on its own term, from which `(false)` follows as before.
  `std.math.binary` is the same under `bin-`. **Once**: the ask and the label are facts of the numeral.
- **Who asks.** `std.math.positional`'s `operand-check` (78 rules now, 63 parameters, `?CHECK` and `?VALID` among
  them): the numeral an operand stands for, written as the operand or as a `let`'s value. `std.math.convert`'s
  `to-bin-ask` and `to-dec-ask`; `std.math.num9`'s `to9-check` (generated). `std.math.int`'s `int-operand-check`
  asks `(int-check I)` of an integer an operand stands for, its six shape rules read it, `int-check-parts` asks
  decimal's check of each part; `std.math.rat`'s `rat-check-ask` and `rat-check-parts` the same, the numerator
  asked of `int-check`. `std.math.gcd` takes its operands through `operand`, and needs nothing.
- **A number an operation makes is labelled as it is made**: `turn-end` and `div-end` conclude `(VALID S)` beside
  the hand-over, so every answer of every module over the bases, a tagged call's included, is valid already;
  `num9`'s `from9` rules label their number; `z` is valid by the closing rules of `decimal` and `binary`.
- **A numeral no operation takes is not checked.**
- **No rule joins two facts on a number**: each new rule has one premise that holds numbers, beside a row of `nz` or
  `bin-nz`, guards or the probe `named`; the walk carries the numeral, so its end needs no second premise, and no
  operation waits for the label. A number a rule of the program's own hands to a second request is walked again
  (Q-M1f.410).
- **The engine**: the loader states nothing of what a file nests; `written` is an ordinary name; `#written` is gone
  from the names the loader makes up (five); `RESERVED` is eleven names. Nothing else in the engine changed.

The readings, each with its alternatives and a default, are
[Q-M1f.405](../open_questions.md#q-m1f405--which-numerals-are-taken-from-the-input-program-and-what-once-is) to
[Q-M1f.411](../open_questions.md#q-m1f411--a-numeral-with-two-faults-has-one-flag). The decision is
[Q-M1f.404](../../../docs/decisions/stdlib/math/Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md),
in `stdlib/math/`; Q-M1f.272, .275, .279, .280 are deleted and each citation of them rewritten to it.

#### Shown on real programs, with the answers as built (`ein.rs/target/gate/ein`)

1. **A well-formed numeral**, `tests/stdlib/math/decimal/01_well_formed_everywhere.ein`: R = {∅}, the sum answered,
   `(valid ?numeral)` is `z`, `(n (n z 1) 2)` and `(n z 5)` (the operands, by their walks) and `(n (n z 1) 7)`
   (the sum, by `turn-end`); the unused `let`, the program's own facts, the integer, the rational and the rule's
   conclusion are neither checked nor labelled.
2. **A malformed numeral an operation takes**, `tests/stdlib/math/decimal-arith/22_malformed_digit.ein`, `(add (n
   (n (n z 1) x) 7) (n z 5))`: the walk `(check … (n (n (n z 1) x) 7))`, `(check … (n (n z 1) x))`, stops at `x`;
   `(not-decimal (n (n z 1) x) x)`; `states (k_s) 0`, `unsat core (add (n (n (n z 1) x) 7) (n z 5))`, `unsat
   core (std.math.digits.nz 7)`, `refuted by std.math.decimal.not-decimal-is-false`, as before but for the table
   row the walk read.
3. **A malformed numeral nobody takes**, `tests/stdlib/math/decimal/02_in_a_let_nobody_asks_about.ein`, `(let price
   (n (n z 1) x))`: before, `states (k_s) 0`, refuted by `not-decimal-is-false`; now `states (k_s) 1`, R = {∅}, no
   flag, no label on it.
4. **A result fed to a second operation**, `tests/stdlib/math/decimal-arith/19_div_named_and_chained.ein`, whose
   rule `share-out` hands `(product-of price qty ?total)`'s 1500 to `(div ?total people)`: `(valid (n (n (n (n z 1)
   5) 0) 0) :rule std.math.positional.turn-end)` as the product is made; then, as the division's operand, five
   `check` facts of it (`operand-check`, `numeral-step-zero` twice, `numeral-step` twice), its label derived again
   and not stored twice. The answer is as before: `(quot-of (n (n (n (n z 1) 5) 0) 0) people (n (n (n z 2) 1) 4) (n
   z 2))`.

#### Answers: every corpus program's, before and after

Every corpus entry was solved (`ein solve FILE --json-summary`, R, every query's classes, the claim's outcome and
the exit) on master's engine and standard library, and again on each step of the build. **Three programs changed,
each by reading 3**, a malformed numeral no operation takes, each dead before and R = {∅} after:
`tests/stdlib/math/decimal/02_in_a_let_nobody_asks_about.ein`, `03_in_a_fact_of_the_programs_own.ein`,
`04_in_a_rules_conclusion.ein`. Each is rewritten to claim R = {∅}, no flag, and a sum beside it answered (02 also
`valid`'s extent). Binary has no such program: its `02` to `05` put the malformed numeral in a request. No other
program's R, query classes, claim outcome or exit moved; the engine's erasure alone moved none.

Programs edited besides: `decimal/01` and `binary/01` (a query of the label); `decimal/05` and `binary/06`, *what
stays silent*, rewritten so that each silent numeral is a request's operand, the assembling rule gone (N-30's two
programs); `decimal/07_in_a_rules_conclusion_that_fires.ein`, new, a rule that hands a malformed numeral to a
request, `(false)`.

#### Commits

On master: **`bf9693dc`**, the code commit: the engine, the standard library, the tests, the goldens, the pages and
the decisions together, each page being true only with the engine's and the library's change; with it three links
of the process that pointed at the deleted decisions now point at Q-M1f.404 (`open_questions.md`, `analysis.md`,
F24), since the link check reads `plans/` too. Then the record commit, plans only: this Record,
`open_questions.md`'s Q-M1f.405 to .411, the phase's row, `findings.md` and the list of what `git grep -w written`
still prints, [`s1f.22.6_written_lines.txt`](s1f.22.6_written_lines.txt). Between the two, **`ec9049b5`**: the first
gate on `bf9693dc` failed one test, `alive_invariant.rs`, whose banked list still named `binary/02`'s numeral as it
was before the mutation census's edit moved its fault one place in; the entry is taken again, and every check below
is of `ec9049b5`.

#### Checks, on the last code commit

On AC, 32 threads, each once on `ec9049b5`:

| check | exit | time |
|---|---|---|
| `./run_tests.sh` | 0 | 17 min 26 s |
| `cargo test --manifest-path ein.rs/Cargo.toml -p ein-infer --features counters --test arith_batch` | 0, 8 tests | 30 s |
| `./run_tests.sh --nightly` | 0 | 30 min 6 s |
| `utils/ci_container.sh` (Ubuntu 26.04, amd64v3, Graphviz 14.1.2) | 0 | 18 min 13 s |

The container checked out `75fad513`, two commits of P1f.21 past `ec9049b5`, plans only: the code it ran is
`ec9049b5`'s. The first gate, on `bf9693dc`, exited 101 (above). `dad4543f`, one sentence of Q-M1f.404, is docs
only: the link check, the release voice and the doc blocks exit 0 on it.

Before the commit: `utils/stdlib_mutants.py --check` exit 0, the banked set unchanged, 521 of 638 mutants killed
(8 min 44 s; the first run, before the tests' numerals moved, exited 1 on eight new survivors, below);
`utils/stdlib_census.py`: 511 rules over 21 modules, every one activated; `utils/check_generated.py` (with
`EIN_BIN` the gate binary): every generator's file as written; `utils/gen_stdlib_rule_pages.py`: twelve new regions
and their drawings, placed by hand on the pages of their family.

#### What moved, and why

- **The circuits' pins** (`arith_theory.rs`, `binary_theory.rs`, `int_theory.rs`, `rat_theory.rs`, `gcd_theory.rs`,
  `num9_theory.rs`): each size grows by its operands' checks, a firing a place and one for the label, the ask
  besides; depths are the chain's, but binary's 127645 ÷ 0, (12, 10) → (31, 21), where the seventeen-bit walk is
  deeper than the chain a division by zero stops. 123078 + 4567 is (35, 21), was (21, 21). The numbers in
  `stdlib/README.md`, `gcd.ein`'s and `convert.ein`'s headers follow.
- **`refutation_under_absent.rs`**: *forbid*'s message beside an hrule over `words`' rules is five lines, was four:
  `not-decimal-is-false` now rests on the requests those rules derive, as `malformed-is-false` does. The program was
  refused before and is.
- **`alive_invariant.rs`'s banked list**: three entries of the two rewritten *what stays silent* programs gone, one of
  `decimal/07` added. Nothing of the erasure.
- **Goldens**: `corpus_shapes.md5` re-banked over every program (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`): 4 889 of its 28 324 lines moved, of 276 programs (the
new one's 39 lines among them): every program whose file nests a fact lost its `written` facts, and every program
of the calculator gained its checks and labels, in every rendering of the state (`load`, `saturate`, `explain`,
`states`, `dot` above all, 1 786 lines);
  `corpus_exits.txt`: the two rewritten silent programs' `render rules` exit 1 (no rule left, as `02` and `03` have
  always had), and `decimal/07`'s five cells.
- **Drawings**: the beliefs' `kb.dot` and `.svg`, taken again; they were already behind the renderer (every object
  once) before this stage, and drew `written` octagons.

#### Not done, or uncertain

- **Two answers move outside the corpus for a reason the stage file's reading 3 does not name**, each dead on master
  and R = {∅} now, each put as an alternative with the default kept: a malformed numeral handed straight to a
  tagged call, `(add-walk (mine a) (n z x) (n z 1) 0 z)`, since a call looks nothing up (Q-M1f.405 (e)); and a
  malformed natural number as a rational's numerator, `(rat-add (rat (n z x) (n z 2)) Q)`, which the operation
  answers by silence and computes nothing with (Q-M1f.409 (d)). No corpus program writes either, so the stage's
  stop condition, which is the corpus's, did not hold. Where an integer goes, a natural number is still flagged.
- **Binary has no twin of `decimal/02` to `04`**: its programs put every malformed numeral in a request, and none
  changed.
- **A made number a program's rule hands on is walked again** (Q-M1f.410): no design under the rule of the
  calculator avoids it, short of labelling every prefix of every number a walk makes.
- **The mutation census** first found eight new survivors, each a flag or a step of the walk that read the ask in
  place of the place reached, unseen because every malformed numeral of the suite was at fault at its top place
  and no operand had a 0 inside it. `decimal-arith/24` to `26`, `binary/02`, `04`, `05` now put the fault one place
  in, and `decimal/01`'s operand is 107; the second run kills all eight, and the banked set holds (521 of 638
  killed, 117 survivors, as banked).
- **The pages outside the standard library's** (`docs/ein/user/` but `04-stdlib/`, `reasoning/`, `ein.rs/` and the
  decisions it touched) were swept by a fork of this stage's agent, told the design as built, and its diff read
  after; the rest is this agent's.
- **Not this stage's, and next**: the multi-context drawings of [`diagrams/`](diagrams/)
  (`render-held-in-two-contexts`, `render-two-contexts-of-one-relation`, S1f.22.4's `render-two-contexts`) still
  show `written` octagons and frames; drawing them again, and S1f.22.5's resumption on its branch, are the stage
  file's *After it* (N-35 names the five files the branch meets this stage in).
- **`git grep -n -w written -- . ':!plans'`** prints 3 603 lines at `dad4543f`, every one in
  [`s1f.22.6_written_lines.txt`](s1f.22.6_written_lines.txt). None is the relation: no line holds `(written (`,
  `(written ?`, `(written T)`, `#written`, `written ctx`, `kernel.written` or `terms::WRITTEN`, nor names
  `written` as a name of the language (Q-M1f.404's sentence that did is `dad4543f`'s one line). They are 3 052
  lines of prose where *written* is the English word (*as written*, *written out*, *a written fact*), 454 lines of
  Rust where it names a variable, an argument, a field or a test's helper of that sense (*a form as the file writes
  it*, `Choice::Coupled { written }`, `arith_theory.rs`' `written("042")`), 74 `.expect("written")` of a test that
  writes a file, 16 inside a drawing's text, 5 of `send_more_money_written` and its generator, and 2 other
  identifiers (`Order::WRITTEN`, `shared/written.rs`'s list of hand-written programs).
