# S1f.21.8 — rules the standard library states

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts)); its third module is part 2's
**Status:** **done 2026-11-19**, on the user's word to start it (*"start s1f.21.6 and s1f.21.8"*); planned the
same day. **Merged in part**: its first commit, `std.negation` and `std.contradiction` (`b0ee99f4`), is on master by
the merge `ca98dec2`, the shape golden re-banked on the merged tree. **Its second commit, `std.debts` (`6b76c164`),
is on master as `edbe6c3b`**, merged 2026-11-19 on the user's ruling
([Q-M1f.560](../open_questions.md#q-m1f560--stddebts-with-its-stage-or-with-the-second-part) (b), *merged now*),
after being held on its branch while the user left what a state owes to the second part
([§ The merge of `std.debts`](#the-merge-of-stddebts)). Reading 1 was not taken as its default reads: three modules, and
`std.debts`' relations renamed and not declared ([§ Record](#the-readings)); the readings are in the ledger,
Q-M1f.555 and Q-M1f.556. Below, a file of `std.debts` is named and not linked: it is not on master.
**Group:** [D](modifications.md#d-rules-the-standard-library-could-state-today), items 17 to 19.
**Waits on:** where each rule lives, which is this stage's first reading. Nothing in the engine.
**Writes:** `stdlib/` and its manifest; `tests/stdlib/`; `corpus/corpus.toml`; `stdlib/README.md`; the rules pages
under `docs/ein/user/04-reference/04-stdlib/`, written by their generator.

## What it does

Three sets of rules that load and run on today's engine, each a module a program imports. **Additive**: a
program that does not import one answers as it did. They are what
[S1f.21.11](s1f.21.11_kill_and_three_conventions.md) later gives every program with no import line.

1. **The elimination of a double negation** (item 17): one rule a count of arguments, the relation a parameter,
   as `examples/beliefs/06_double_negation.ein` writes it for two arguments.
2. **The contradiction** (item 18): `clash-0` to `clash-6`, one rule a count, and `clash-of-0` to `clash-of-6`,
   which derive each activator from a declaration ([`design.md` A.1](design.md#a1-the-contradiction-rule-set-contrainc)).
3. **The requirement, the payment and `discharged`** (item 19): `due` from an obligation's trigger, `paid` where a
   witness is held, `discharged` where no requirement is unpaid
   ([`design.md` A.2](design.md#a2-the-requirement-and-the-payment-emuinc), [A.3](design.md#a3-stdslots-two-obligations-slots-emuinc)).

**Reading 1, where they live.** Default: two modules, `std.negation` for 1 and 2 and `std.debts` for 3, each
rule under an activator of its own, so that importing a module beside `std.algebra` moves no fact of a program
that does not activate it. `std.logic` stays free for M1g. The other readings: 3 inside `std.algebra` and
`std.slots`, beside the obligations it restates; 1 and 2 as two modules.

**Reading 2, which counts.** Default: 0 to 6 for the contradiction, as the design has it, and the same for the
elimination; the count the corpus needs is taken by a census before the rules are written.

## What must hold

- **Each rule has its program** under `tests/stdlib/` that activates it, with an `:expect`
  (`ein-infer/tests/stdlib_coverage.rs`); a refutation rule has two, one where it fires and one where it is
  loaded, activated and satisfied; each file has its `corpus.toml` entry.
- Each rule has its region on its family's rules page and its drawing (`utils/gen_stdlib_rule_pages.py`, then
  `utils/render_diagrams.py`), and the manifest is written again (`utils/stdlib_manifest.py --write`).
- **The two invariances of the survey, re-taken with the modules imported and not appended**:
  [`design.md` § 2.1](design.md#21-from-a-fact-and-its-negation-false)'s, every corpus program `ein solve` answers
  with the contradiction module imported answers the same, but the seven *forbid* refuses
  ([V-1](findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none)); and
  [§ 2.4](design.md#24-the-verdicts-together-which-may-be-a-fact)'s, the unpaid requirements are the kernel's
  `owes` at every node of the seats program and of `examples/zebra2-obligations.ein`.
- The module's page says what V-1 found: a program that imports the contradiction rules and rests a refutation on
  an `absent` of that shape is refused. No corpus program imports the module but its tests.
- `utils/stdlib_mutants.py`'s survivors for the new rules are banked.

## Acceptance

`ein test tests/` holds every claim; `stdlib_coverage.rs` and `utils/check_generated.py` pass;
`examples/beliefs/06_double_negation.ein` imports the rule it wrote itself, and answers as before; one gate.

## Record

**2026-11-19.** Built in a worktree, on the branch `worktree-agent-a8ead811ae8dbe86c`, which was fast-forwarded to
master's `a237cefd` first: the branch's base was `e4a5f8fb`, two commits behind, and this file was not in it. The
engine was not changed. `AGENTS.md`, the phase's `README.md`, `findings.md`, `overview.md` and `modifications.md`
and the milestone's `open_questions.md` were not edited; what should change in them is the last section here.

### What was built

Three modules where the default read two (Reading 1 below), each additive: a program that imports none answers as
it did, and a program that imports one and asks for nothing holds what it held.

| module | rules | what turns each on |
|---|---|---|
| `std.negation` ([`negation.ein`](../../../stdlib/negation.ein)) | `eliminate-0` to `eliminate-6` `(?R)`: `(not (not (?R x…)))` ⟹ `(?R x…)` | its activator, `(eliminate-2 likes)` |
| `std.contradiction` ([`contradiction.ein`](../../../stdlib/contradiction.ein)) | `clash-0` to `clash-6` `(?R)`: `(?R x…)` ∧ `(not (?R x…))` ⟹ `(false)` | its activator, `(clash-2 seats)` |
| | `clash-of-0` to `clash-of-6` `()`: a declaration of that count ⟹ `(clash-N ?R)` | the fact `(clash-of-declared)`, a premise of each |
| `std.debts` (`debts.ein`) | `total-due`, `total-paid`, `surjective-due`, `surjective-paid` `(?R ?isa)`; `room-due`, `room-paid`, `fill-due`, `fill-paid` `(?R ?isa ?sub ?super ?index)`: `(debt-due R J)` from the obligation's trigger, `(debt-paid R J)` from the trigger with its witness | its activator, `(total-due seats is-a)` |
| | `debts-of-total`, `debts-of-surjective`, `debts-of-room`, `debts-of-fill` `()`: an obligation's activator ⟹ its pair's two activators | the fact `(debts-of-owed)`, a premise of each |
| | `discharged-from-nothing-unpaid` `()`: no `debt-due` without its `debt-paid` ⟹ `(debts-discharged)` | the same fact |

No module declares a relation or writes a fact. `std.debts` imports `std.algebra` and `std.slots`, whose
obligations' names its four `debts-of-…` rules read.

**The commits**, on the branch:

- `b0ee99f4`: `std.negation` and `std.contradiction`, their 11 programs and corpus entries,
  `contradiction_rules.rs`, page 20 and its 21 regions, the READMEs' sections, and
  `examples/beliefs/06_double_negation.ein` importing `eliminate-2`;
- `6b76c164`: `std.debts`, its 10 programs and corpus entries, `debts_rules.rs`, page 21 and its 13 regions, the
  READMEs' sections, the obligations page's pointer;
- this record.

**With the rules**: 21 programs under `tests/stdlib/` (`negation/01`; `contradiction/01` to `10`; `debts/01` to
`10`), each with its `corpus.toml` entry, and an entry for each module; two in-process tests,
[`ein-infer/tests/contradiction_rules.rs`](../../../ein.rs/crates/ein-infer/tests/contradiction_rules.rs) and
`debts_rules.rs`, for what no claim can state; two pages
of the reference, [20](../../../docs/ein/user/04-reference/04-stdlib/20-negation-and-contradiction.md) and
21, added at the end of the folder so no page moved,
their 34 rule regions and drawings written by `utils/gen_stdlib_rule_pages.py`; a section each in
`stdlib/README.md` and `tests/README.md`, rows in the two indexes and in the top `README.md`; the manifest
rewritten, 27 modules. `examples/beliefs/06_double_negation.ein` imports `eliminate-2` in the place of its own
`eliminate` and keeps `eliminate-in`, the same rule under `(think ?p …)`, which is a program's.

**Beside the stage's *Writes* line**, which names the library, its tests, the manifest and the rules pages: three
files under `ein-infer/tests/` (the two new tests, and `alive_invariant.rs`'s pinned list), the two goldens,
`examples/beliefs/06_double_negation.ein` and one sentence of `03_liar_denial.ein`'s header, `examples/README.md`,
`tests/README.md`, the top `README.md`, and three pages that speak of the elimination or of what an obligation
owes (`06-constraints.md`, `08-obligations.md`, `08-reasoning-in-code/01-static-ir.md`). No file under
`ein.rs/crates/*/src/` and none under `utils/` was changed.

### The readings

**Reading 1, where they live. The default was not taken as written**, on two points, each for something the rules
showed when run.

1. **Three modules, not two: the contradiction is `std.contradiction`, apart from `std.negation`.** This is the
   stage's own other reading, *1 and 2 as two modules*. `eliminate-N` concludes `(?R …)` with whatever the double
   negation held as its arguments, and the loader reads such a conclusion as writing into every rule of that count
   of parameters whose name the program writes as an argument
   ([Q-M1f.42](../../../docs/decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)): four
   corpus programs are refused on importing it, `examples/lattice/02_genuine_3set_death.ein`,
   `03_state_hash_collision.ein`, `15_union_label_dead.ein` and `16_pair_through_closure.ein`, each naming its
   hrule `guess (?R ?T)` more than once in `(solve :hrules …)`. An import is whole, so in one module the
   contradiction rules would be refused with them, and § 2.1's invariance would read *seven by forbid and four by
   the elimination*. Apart, the contradiction module loads wherever the program does. `std.logic` stays free.
2. **`std.debts`' relations are `debt-due`, `debt-paid` and `debts-discharged`, and no module's**, where the design
   has `due`, `paid` and `discharged` declared. Three forms were run over the corpus with the module imported and
   nothing asked for:
   - **declared, as the design writes them**: an import moves answers. A declaration's labels become names of the
     program, which a free `neq` or `eq` binds over, and a declared relation of two labels needs its held extent
     closed against the blind enumerator, by a `(__closed__ R)` a rule can read. Four corpus programs' claims
     fail: `examples/features/16_eq_neq_goal.ein`, `17_eq_neq_premise.ein`,
     `examples/ein-bugs/wrong-answers/goal-neq-free.ein` and `tests/stdlib/closure/02_closed_and_satisfied.ein`;
   - **not declared, under the design's words**: six corpus programs declare `(relation paid Guest)` and are
     refused, since the module writes `paid` with two arguments, and
     `tests/stdlib/math/decimal-arith/10_sub_named.ein` names a number `paid`, which stops being a guessable atom;
   - **not declared, under words no corpus program uses**: nothing moves. Taken.
   The same holds of the two facts that ask for everything, `(clash-of-declared)` and `(debts-of-owed)`: relations
   a module reads and does not declare, as `std.algebra` reads `(bijective R)`.

*Each rule under an activator of its own* holds as far as a rule can have one: the rules with a parameter have
their activator, and the twelve with none each read a fact the program writes. Importing `std.negation` or
`std.contradiction` moves no fact; importing `std.debts` moves none of its own, and brings `std.algebra` and
`std.slots`, which is the reading's *beside `std.algebra`*.

**Reading 2, which counts. The default is taken**: 0 to 6 for the contradiction and for the elimination. The
census, taken before the stage's programs were in the manifest (`python3 count_census.py REPO census.tsv`, the
script below; root's fixpoint of every corpus entry that saturates):

899 corpus entries, of which 675 saturate and 224 are refused. Each cell is *occurrences / programs*:

| count | declared relations, by labels | facts held at root | stored negations, by the denied fact's count | negations of a negation, by the innermost fact's | facts under an arrow |
|---:|---:|---:|---:|---:|---:|
| 0 | 285 / 259 | 173 / 172 | 1 / 1 | | 7 / 4 |
| 1 | 4 938 / 492 | 69 158 / 560 | 8 / 7 | | 17 581 / 114 |
| 2 | 12 025 / 531 | 43 619 / 529 | 1 386 / 67 | 5 / 4 | 2 528 462 / 309 |
| 3 | 7 198 / 292 | 60 738 / 331 | | | 37 764 / 185 |
| 4 | 3 039 / 292 | 84 737 / 315 | | | 34 902 / 87 |
| 5 | 3 279 / 292 | 337 668 / 314 | | | 6 590 / 63 |
| 6 | 604 / 215 | 1 218 / 46 | | | 5 955 / 61 |
| 7 | 383 / 214 | 138 / 25 | | | 4 256 / 40 |
| 8 | 428 / 214 | 4 813 / 37 | | | 4 256 / 40 |
| 9 | 676 / 214 | | | | 53 109 / 56 |
| 10 | 231 / 214 | | | | 4 805 / 45 |

Past six: 29 names as the dumps print them, all the calculator's (`std.math.decimal-arith`'s division, `div-choose`,
`div-trial`, `div-m6` to `div-m9` and `multiples`; `std.math.num9`'s `num9`, `row9`, `multiples9` and `div9-at`;
`std.math.rat`'s `rat-cross`, `rat-cross-run`, `rat-mul-call` and `rat-mul-run`), declared in 214 programs.

So the corpus needs the contradiction at counts 0 to 2 and the elimination at 2, and 0 to 6 covers every count at
which any corpus program holds a negation. The relations it leaves out are the calculator's, of seven to ten
labels, and no corpus program holds a negation of a fact of one at root. The census reads root's fixpoint only: a
negation a rule concludes only under a commitment is not in it.

### What differs from `design.md`'s rule sets

- **A.1.** `clash-N` as written. `clash-of-N` each read the fact `(clash-of-declared)` beside the declaration,
  where the design's fire on every program that holds them; and `clash-of-0`'s `:why` says *is declared*, since it
  reads every declaration and not only those of no argument.
- **The elimination** is `eliminate-0` to `eliminate-6`, the example's rule at each count, its `:why` writing the
  fact as `({?R} {?a} {?b})`.
- **A.2 and A.3.** The relations' names, and none declared (Reading 1). `emulate-total` and `emulate-surjective`
  are `debts-of-total` and `debts-of-surjective`, each reading `(debts-of-owed)`; `debts-of-room` and
  `debts-of-fill` are new, where A.3 wrote the activators by hand. `discharged-from-nothing-unpaid` reads
  `(debts-of-owed)` too, so nothing concludes `(debts-discharged)` for a program that only imports the module.
  The instance relations are the design's, `total-owed-of`, `surjective-owed-of`, `slot-owed-room-of`,
  `slot-owed-fill-of`.

### The two invariances, with the modules imported

Both taken on the tree as committed, by `sweep_modules.py` (below), which is the survey's `sweep_clash.py` with the
rule set imported in the place of appended: each corpus entry under `examples/` or `tests/` with a `solve` run is
solved as it stands and with lines appended to a scratch copy, and the two `--json-summary` files compared in the
survey's fields. `sh run_sweeps_parallel.sh REPO VARIANT…`, a 120 s limit a run, none reached.

852 programs: 245 are not answered as they stand (the corpus's refused programs, and one whose claim fails), one
is a saved file, `tests/saved/04_a_saved_node.ein`, which holds one `(program …)` and one `(commit …)` and takes no
form after them, and **606 are swept**.

| appended to each program | the same | differ | not answered |
|---|---:|---:|---:|
| `(import std.negation)` | 602 | 0 | 4 |
| `(import std.contradiction)` | 606 | 0 | 0 |
| `(import std.debts)` | 599 | 3 | 4 |
| `(import std.algebra)` `(import std.slots)`, what `std.debts` imports | 599 | 3 | 4 |
| **(a)** `(import std.contradiction)` `(clash-of-declared)` | 598 | 0 | 7 + 1 |
| `(import std.debts)` `(debts-of-owed)` | 597 | 4 | 4 + 1 |

**(a) § 2.1's: with the contradiction module imported and its activators derived, 598 of the 606 answer the same**
in every field of `region` the survey compared, the queries' classes, the claim's verdict, the static verdict and
the number of saturations; 102 of them hold a stored negation at root or write `(not `. **Seven are refused, each
by *forbid*, and they are the survey's seven**: `examples/features/07_unstratifiable.ein`,
`examples/lattice/08_absent_witness_hole.ein`, `examples/lattice/10_fifo_gap.ein`, and `f07-fpoff.ein`,
`f07-rules-swapped.ein`, `kw-neq-in-absent.ein`, `kw-neq-in-absent-twin.ein` under
`examples/ein-bugs/wrong-answers/`, each naming `std.contradiction.clash-0` under an activator. The eighth is this
stage's `tests/stdlib/contradiction/10_clash_of_not_asked.ein`, whose claims say that `(clash-of-declared)` is not
written, and fail once it is appended. Before the stage's 21 programs were in the manifest the same sweep read 831
programs, 585 swept: 578 the same and the seven.

**An import alone**, which the survey did not take (the three rows above it; these also compare root's facts,
relation by relation, root's verdict and what hypothesis generation counted):

- `std.contradiction` moves nothing: 606 of 606.
- `std.negation`: the four refused are Reading 1's four lattice programs, by `eliminate-2`.
- `std.debts`: the three that differ and the four not answered are `std.algebra`'s, not the module's rules':
  importing the two modules it imports, and not `std.debts`, gives the same seven. The three are
  `examples/domain_elim/ab.ein`, `b_branch.ein` and `b_only.ein`, which write `(bijective color-loc)` for a rule of
  their own and gain `std.algebra`'s four markers at root; the four are the same lattice programs, refused by
  `std.algebra`'s `converse` and its other copiers.

**With `std.debts` asked for**, the last row: 597 the same. Of the four that differ, two differ in
`work.saturations` alone (`examples/beyond_arith/send_more_money.ein` and its `_written` twin, 18 189 where they took
18 129, the same R), and two are descriptions that stop at the region budget and stop sooner
(`examples/zebra2-minus-15.ein` and `-obligations.ein`: `k_c`, `stopped`, `states` and the saturations). The one
beside the four lattice programs is this stage's `tests/stdlib/debts/08_debts_not_asked.ein`, as `10` above.

**(b) § 2.4's: the requirements with no payment are the kernel's `owes`**, with `(import std.debts)` and
`(debts-of-owed)` appended to a scratch copy of each program.

- **`examples/lattice/05_owed_seats_bounded.ein`, the 16 nodes of the oracle's cube.** `python3 variant.py
  examples/lattice/05_owed_seats_bounded.ein seats-debts.ein '(import std.debts)' '(debts-of-owed)'`, `ein oracle
  seats-debts.ein --dump dseats --states`, `python3 cmp_owes.py seats-debts.ein dseats`: root requires 4 and has
  paid none, and `16 nodes, 0 differ`. `python3 cmp_discharged.py dseats`: `(debts-discharged)` is held at the
  seven nodes that owe nothing, the two of R and the five dead above them, and at none of the nine that owe.
- **`examples/zebra2-obligations.ein`, the 120 nodes `ein solve` enters.** `ein solve z2o-debts.ein --events
  z2o-debts.jsonl`, `python3 cmp_events.py z2o-debts.jsonl`: `root: due 50 paid 14 unpaid 36`, `24 dead same`,
  `64 in-r same`, `32 owing same`. `python3 cmp_discharged.py z2o-debts.jsonl`: held at the 64 nodes of R and at
  none of the other 56. `python3 cmp_sum.py z2o.sum.json z2o-debts.sum.json`: `region: same 25 fields; differ:
  none`, 120 saturations both, the static check monotone both.
- **The eight obligation fixtures at root** (`cmp_root.py`, `algebra/23` to `26` and `slots/09` to `12`, each with
  the two lines appended): the requirements with no payment are root's `owes` on each, 1, 0, 1, 0, 3, 0, 3, 0, and
  `(debts-discharged)` is held on the four that owe nothing.

They are the survey's numbers, with the rules imported and their activators derived from the obligations'.

Both are now held per commit by `ein-infer/tests/debts_rules.rs`, in-process and by the node function: the sixteen
nodes of `tests/stdlib/debts/10_discharged_over_r.ein`'s cube, every node a description of
`examples/zebra2-obligations.ein` enters with the module asked for, and the roots of the eight obligation fixtures.
The test fails on a copy of the library with `surjective-paid`'s arrow read the other way round (`EIN_STDLIB`).

### Goldens and pinned lists that moved

- **`ein-cli/tests/golden/corpus_exits.txt`** (`EIN_BLESS=1 cargo test … -p ein-cli --test corpus_cli`): 117 lines
  added, none changed, none removed. They are the cells of the 24 new entries: exit 0 everywhere but `render rules`
  on the 21 programs, exit 1, as on every program of `tests/stdlib/` that declares no rule of its own.
- **`ein-render/tests/golden/corpus_shapes.md5`** (`EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-render
  --test id_order_invariance`): 985 lines added and 25 removed. 960 are the 24 new files', 40 each. The other 25
  replace 25 of `examples/beliefs/06_double_negation.ein`'s 40: `load`, `saturate`, `plan`, `match`, the four
  `ir[…]`, the five `dot[ir…]`, the three `dot[rules…]`, the five `dot[kb…]`, the two `dot[constraints…]`,
  `region-trace` and `states`, each of which shows the rule's name or the imported rules. Its 15 others did not
  move: `naf`, both `explain`, the five read-outs, `region-lattice`, the five `cores` and `oracle`. No other
  program's line moved, `examples/beliefs/03_liar_denial.ein`'s among them, whose header's one sentence about `06`
  was reworded.
- **`ein-infer/tests/alive_invariant.rs`**'s `SEEDED`: two programs, ten names, each written only under a stored
  negation (`negation/01_eliminate.ein`, `contradiction/08_clash_satisfied.ein`).
- **`examples/README.md`'s zebra anchor list** (`EIN_BLESS=1 cargo test … -p ein-cli --test world_anchors`): one
  line, `ein-infer/tests/debts_rules.rs`, which reads `examples/zebra2-obligations.ein`.
- **`tests/mutation_survivors.txt`: not moved.** `python3 utils/stdlib_mutants.py --check --bin
  ein.rs/target/gate/ein --jobs 16`, the whole sweep: 676 mutants, 559 killed, 117 survivors, the banked set
  exactly. The new rules have 43 mutants, by `-k`: `clash` 3, `eliminate` 1, `-due` 12, `-paid` 20, `debts-of-` 6,
  `discharged` 1, all killed. Two survived the first run, `room-paid` and `fill-paid` without their `neq`, and
  `debts/03` and `04` gained `(co-loc S1 S2)`, a seat beside a seat, which kills them.

`examples/beliefs/06_double_negation.ein` answers as before: `ein solve -p` prints the same bytes, its three claims
hold, and in its `--json-summary` the one field that differs is root's count of the activator, under `eliminate-2`
where it was under `eliminate`.

### Checks

- **`ein test tests/`**: `305 files, 595 expectations: 593 held, 0 FAILED, 2 not checked, 0 errors`. The two not
  checked are `tests/saved/04_a_saved_node.ein`'s, a saved file's, as before the stage.
- **`cargo test … -p ein-infer --test stdlib_coverage`**: every rule of the library is activated by a program under
  `tests/stdlib/`, the 34 new ones among them.
- **`cargo test --manifest-path ein.rs/Cargo.toml --profile gate --workspace --no-fail-fast`**: 164 test results,
  none failed. A first run without `--no-fail-fast` stopped at `ein-cli`'s `world_anchors`, which named the line
  missing from `examples/README.md`'s list; the list was re-banked and the workspace run again, whole.
- **Over every corpus program** (`EIN_CORPUS_SLOW=1`), the sweeps that read `tests/` only in their slow half:
  `id_order_invariance` (the bless, then run plain), `renaming_invariance`, `dot_wellformed`, `jobs_invariance`:
  each passes.
- **The first commit by itself**, with what is `std.debts`' moved out of the tree: the manifest, the four doc
  checks, `check_generated`, `ein test tests/` (550 of 552 held, the same two not checked), `-p ein-corpus`,
  `-p ein-ir --lib`, `stdlib_coverage`, `alive_invariant`, `contradiction_rules`, `corpus_cli`, `world_anchors`
  and `id_order_invariance` over every program: each passes.
- `python3 utils/stdlib_manifest.py`: 27 modules match. `python3 utils/check_generated.py --bin
  ein.rs/target/gate/ein`: 9 generators, every file as written. `python3 utils/doc_audit.py --links --check
  --tracked`: 0 findings in links, 0 in decision ids. `python3 utils/check_release_voice.py`: none.
  `python3 utils/check_doc_blocks.py`: rules 1 to 5 hold. `python3 utils/render_diagrams.py --check`: every picture
  drawn from its source as it stands.
- `cargo fmt --all --check`: clean. `cargo clippy -p ein-infer --tests -- -D warnings`: clean. The workspace-wide
  clippy and `cargo doc` were not run: no crate's source changed, only three files under `ein-infer/tests/`.
- `python3 utils/stdlib_mutants.py --check --bin ein.rs/target/gate/ein --jobs 16`: exit 0, above.
- `python3 utils/stdlib_census.py --check --bin ein.rs/target/gate/ein`: `570 rules over 24 modules; 570 activated,
  0 never activate`, and **exit 1** for one sweep failure that is not this stage's,
  `tests/saved/04_a_saved_node.ein [test]: exit 0 and no event file` (point 7 below).

### Found outside the stage's scope

None is fixed here. Each is for [`findings.md`](findings.md) or a question, which this stage does not edit.

1. **The elimination cannot be given to every program as it stands.** Reading 1's four refusals are of programs
   that import nothing today. [S1f.21.11](s1f.21.11_kill_and_three_conventions.md) gives every program these rules
   with no import line; `eliminate-N` would then refuse every program that names, as an argument, an N-parameter
   rule whose parameter stands first in a pattern. The contradiction rules have no such trouble: `clash-N`
   concludes `(false)`, and `clash-of-N` an activator whose argument is an argument of `(relation …)`.
2. **Q-M1f.42's reading counts an hrule named in `(solve :hrules …)` only from its second naming.**
   `:hrules (guess (chosen Drink))` loads beside `std.negation`, and `:hrules (guess (chosen Drink) guess (served
   Drink))` is refused: the list is read as one form, whose first name is its head and no argument. The same
   program, by what a run could derive, either way.
3. **A declaration an import brings is not inert.** Its labels are names of the importing program, so a free `neq`
   or `eq` goal binds over them, and a `(__closed__ R)` a module writes or derives is a fact a program's rule
   reads. Three corpus programs' claims list *every name of the program*
   (`examples/features/16_eq_neq_goal.ein`, `17_eq_neq_premise.ein`, `ein-bugs/wrong-answers/goal-neq-free.ein`)
   and one reads every closure (`tests/stdlib/closure/02_closed_and_satisfied.ein`): each fails with any module
   imported that declares a relation, the calculator's among them. It bears on
   [S1f.21.9](s1f.21.9_the_requirement_and_the_payment_as_facts.md) and S1f.21.11, wherever a module every program
   gets declares something.
4. **A state `clash-N` refutes holds two contradictions, and `refuted by` names one by the relation's name.** The
   pair and the derived `(false)` have one core; the explanation's tie-break is the witnesses' rank, by text, the
   pair's fact against `(false)`. So `tests/stdlib/contradiction/04_clash_3.ein`, whose relation is `between`,
   reads `refuted by (given)`, and its six siblings, whose relations sort after `false`, `refuted by clash-N`.
   Deterministic, kept by the renaming sweep (its bijection keeps every order by name), and an explanation
   ([F22](../../followups/f22_exact_core_past_the_cap.md)'s ground). It is S1f.21.11's task 1.2, *`refuted by`,
   which names the module's rule*, seen on today's engine. The read-out still says *root's own fixpoint derives
   (false)* of the same root without the rule, which is
   [V-2](findings.md#v-2--the-surfaces-say-derives-false-of-a-state-that-holds-none).
5. **With the requirements and payments asked for, a description's work moves and its R does not.**
   `examples/beyond_arith/send_more_money.ein` and its `_written` twin take 18 189 saturations where they took
   18 129, the same R. `examples/zebra2-minus-15.ein` and `-obligations`, whose description stops at the region
   budget, stop sooner, the states' facts being what the budget counts: 32 states both ways, and the lower bound on
   `k_c` 97 932 226 where it was 138 621 243.
6. **Importing `std.algebra` moves a program that uses `bijective` for itself.**
   `examples/domain_elim/ab.ein`, `b_branch.ein` and `b_only.ein` write `(bijective color-loc)` for a rule of their
   own, and with `std.algebra` imported its `bijective-properties` derives its four markers beside theirs. Known of
   a relation a module reads and does not declare; it is what the eight words `std.debts` now shares can do too.
7. **`utils/stdlib_census.py --check` exits 1, for a run this stage did not touch.** Its sweep counts
   `tests/saved/04_a_saved_node.ein [test]` as a failure, *exit 0 and no event file*: a saved file's claims are not
   checked, and `ein test` writes no events for it. The census itself is whole, 570 of 570 rules activated. It is a
   dev-host step, so no gate reads it.

### What should change elsewhere

- **The phase's [`README.md`](README.md)**: this stage's row, *done*; S1f.21.11's *waits on*, less this stage.
- **[`findings.md`](findings.md)**: the seven points above, as V-n, the first three bearing on S1f.21.9 and
  S1f.21.11.
- **[`modifications.md`](modifications.md) § D**: items 17 to 19 are built; item 18's home is `std.contradiction`
  and item 19's relations are `debt-due`, `debt-paid` and `debts-discharged`.
- **The milestone's `open_questions.md`**: Reading 1 as taken, for the user to revise: three modules and their
  names; `std.debts`' relations renamed and undeclared; the two facts that ask for everything,
  `(clash-of-declared)` and `(debts-of-owed)`. Reading 2 stands at its default.
- **`AGENTS.md`**, § Where things live, `stdlib/`: the three modules, that they declare no relation, the eight
  words `std.debts` shares and the two facts that ask; `tests/stdlib/negation/`, `contradiction/` and `debts/`;
  that a `clash-N` firing and an owe against the requirements are asserted in-process
  (`contradiction_rules.rs`, `debts_rules.rs`). § Where the work is: this stage done.

### Not done

- `./run_tests.sh`, the nightly and `utils/ci_container.sh` were not run: the coordinator's, after the merge.
- `utils/censuses/stdlib-census.md` was not re-taken as a page; `utils/stdlib_census.py --check` was run, above.
- No decision file was written for the readings: each is a default taken, to be ruled or left.
- The census of counts reads root's fixpoint; no rule's conclusions were read statically for a negation of three
  or more arguments.
- No time was measured.

### The instruments

Scripts of the scratch directory, kept here whole. `REPO` is the worktree, and the engine is its gate build.
[`design.md`'s](design.md#appendix-b-the-instruments) `cmp_owes.py`, `cmp_events.py` and `cmp_root.py` were run
with `debt-due` and `debt-paid` in the place of `due` and `paid`, and nothing else changed but `cmp_root.py`'s
last line, which also says whether root holds `(debts-discharged)`.

<details><summary><code>sweep_modules.py</code> and <code>run_sweeps_parallel.sh</code>: the corpus, as it stands and with a module imported</summary>

```python
#!/usr/bin/env python3
"""sweep_modules.py REPO OUT.tsv VARIANT — every corpus program `ein solve` answers, solved as it stands and
solved with a module of the standard library imported, and the two JSON summaries compared.

The survey's `sweep_clash.py` (design.md, Appendix B.1) with the rule set imported in place of appended.
VARIANT is one of

    import-negation, import-contradiction, import-debts    the module imported, nothing activated
    import-debts-deps    (import std.algebra) (import std.slots), the two modules std.debts imports, alone
    clash    (import std.contradiction) (clash-of-declared)
    debts    (import std.debts) (debts-of-owed)

The programs are copied to ./corp-VARIANT/ (examples/ and tests/), so the repository is not written; the stdlib
is the repository's ($EIN_STDLIB) and the engine its gate build."""
import json
import os
import re
import shutil
import subprocess
import sys
import tomllib
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
repo = Path(sys.argv[1]).resolve()
out_tsv = Path(sys.argv[2])
variant = sys.argv[3]
EIN = str(repo / "ein.rs/target/gate/ein")
LINES = {
    "import-negation": "(import std.negation)\n",
    "import-contradiction": "(import std.contradiction)\n",
    "import-debts": "(import std.debts)\n",
    "import-debts-deps": "(import std.algebra)\n(import std.slots)\n",
    "clash": "(import std.contradiction)\n(clash-of-declared)\n",
    "debts": "(import std.debts)\n(debts-of-owed)\n",
}[variant]
TIMEOUT = int(os.environ.get("SWEEP_TIMEOUT", "120"))
corp = HERE / f"corp-{variant}"
if corp.exists():
    shutil.rmtree(corp)
for d in ("examples", "tests"):
    shutil.copytree(repo / d, corp / d)
env = dict(os.environ, EIN_STDLIB=str(repo / "stdlib"))

entries = tomllib.loads((repo / "corpus/corpus.toml").read_text())["entry"]
paths = [
    e["path"]
    for e in entries
    if e["path"].split("/")[0] in ("examples", "tests") and any(r == "solve" or r.startswith("solve ") for r in e["runs"])
]


def solve(path: Path):
    summ = path.with_suffix(".sum.json")
    try:
        p = subprocess.run(
            [EIN, "solve", str(path), "--json-summary", str(summ)],
            capture_output=True, text=True, env=env, timeout=TIMEOUT,
        )
    except subprocess.TimeoutExpired:
        return "timeout", None, ""
    if p.returncode != 0 or not summ.exists():
        return f"exit {p.returncode}", None, (p.stderr or p.stdout)[:400].replace("\n", " | ")
    return "ok", json.loads(summ.read_text()), ""


#: What a module's activation adds to what a summary lists: the activators the rules derive, and the marker.
ADDED = re.compile(r"\((?:std\.contradiction\.clash-\d|std\.debts\.[a-z-]+|clash-of-declared|debts-of-owed|debt-due|debt-paid|debts-discharged)[ )]")


def strip(x):
    if isinstance(x, list):
        return [strip(v) for v in x if not (isinstance(v, str) and ADDED.match(v))]
    if isinstance(x, dict):
        return {k: strip(v) for k, v in x.items()}
    return x


REGION = ["k_s", "k_c", "exhausted", "stopped", "root", "root_owes", "empty", "minimal", "maximal", "components",
          "subcubes", "merged", "universe", "states", "route", "licences"]


def one(rel):
    src = corp / rel
    text = src.read_text()
    st0, s0, msg0 = solve(src)
    if st0 != "ok":
        return rel, "base-" + st0, "", msg0
    if re.search(r"(?m)^\(program\b", text):
        # a saved file is one (program …) and one (commit …), and takes no form after them
        return rel, "saved-file", "", ""
    var = src.with_name(src.stem + "__with.ein")
    var.write_text(text + "\n" + LINES)
    st1, s1, msg1 = solve(var)
    neg_root = s0["root"]["facts_by_relation"].get("not", 0)
    neg = "neg" if (neg_root or "(not " in text) else "-"
    if st1 != "ok":
        return rel, "variant-" + st1, neg, msg1
    diffs = []
    for k in REGION:
        if strip(s0["region"].get(k)) != strip(s1["region"].get(k)):
            diffs.append("region." + k)
    for k in ("core", "refuted_by"):
        if s0["region"].get(k) != s1["region"].get(k):
            diffs.append("explain." + k)
    q0 = [(q["goal"], strip(q["classes"]), q.get("expect")) for q in s0["queries"]]
    q1 = [(q["goal"], strip(q["classes"]), q.get("expect")) for q in s1["queries"]]
    if q0 != q1:
        diffs.append("queries")
    if s0.get("verdict") != s1.get("verdict") or s0.get("claim") != s1.get("claim"):
        diffs.append("claim")
    if s0["monotonicity"]["monotone"] != s1["monotonicity"]["monotone"]:
        diffs.append("monotone")
    if s0["work"]["saturations"] != s1["work"]["saturations"]:
        diffs.append("work.saturations")
    if variant.startswith("import-"):
        # nothing activated: root holds the same facts, relation by relation, and the search proposes the same
        f0, f1 = dict(s0["root"]["facts_by_relation"]), dict(s1["root"]["facts_by_relation"])
        if f0 != f1:
            diffs.append("root.facts_by_relation")
        if s0["root"]["hypgen"] != s1["root"]["hypgen"]:
            diffs.append("root.hypgen")
        if s0["root"]["owes"] != s1["root"]["owes"] or s0["root"]["dead"] != s1["root"]["dead"]:
            diffs.append("root.verdict")
    return rel, "same" if not diffs else "differs", neg, ",".join(diffs)


with ThreadPoolExecutor(max_workers=int(os.environ.get("SWEEP_JOBS", "8"))) as pool:
    rows = list(pool.map(one, paths))
out_tsv.write_text("".join("\t".join(r) + "\n" for r in rows))
tally = {}
for _, status, neg, _ in rows:
    tally[(status, neg)] = tally.get((status, neg), 0) + 1
for k in sorted(tally):
    print(tally[k], *k)
print(len(rows), "programs")
```

```sh
#!/bin/sh
# run_sweeps_parallel.sh REPO VARIANT… — the variants' sweeps side by side, each to VARIANT.out.
cd "$(dirname "$0")" || exit 1
repo=$1
shift
for v in "$@"; do
  SWEEP_TIMEOUT=120 SWEEP_JOBS=5 python3 sweep_modules.py "$repo" "$v.tsv" "$v" > "$v.out" 2>&1 &
done
wait
for v in "$@"; do
  echo "== $v"
  cat "$v.out"
done
```

</details>

<details><summary><code>count_census.py</code>: the counts of arguments the corpus's facts have</summary>

```python
#!/usr/bin/env python3
"""count_census.py REPO [OUT.tsv] — the counts of arguments the corpus's facts have.

Every corpus entry under examples/, tests/ and stdlib/ is saturated (`ein saturate --dump FILE`) and the dump
read: the declarations, by their count of labels; every fact root's fixpoint holds, by its count of arguments;
every stored negation, by the count of the fact it denies, a double negation apart; and every fact that stands
under an arrow of another. A program the load refuses has no dump and is counted as refused."""
import os
import re
import subprocess
import sys
import tomllib
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

repo = Path(sys.argv[1]).resolve()
EIN = str(repo / "ein.rs/target/gate/ein")
env = dict(os.environ, EIN_STDLIB=str(repo / "stdlib"))
TIMEOUT = int(os.environ.get("CENSUS_TIMEOUT", "120"))
TOKEN = re.compile(r'"(?:\\.|[^"\\])*"|[()]|[^\s()]+')


def parse(line):
    """One s-expression of a dump line, as nested lists of tokens."""
    toks = TOKEN.findall(line)
    pos = 0

    def read():
        nonlocal pos
        t = toks[pos]
        pos += 1
        if t != "(":
            return t
        out = []
        while toks[pos] != ")":
            out.append(read())
        pos += 1
        return out

    return read()


def cut(form):
    """A fact's head and arguments: the dump writes its origin after them, from the first `:keyword`."""
    args = []
    for a in form[1:]:
        if isinstance(a, str) and a.startswith(":"):
            break
        args.append(a)
    return form[0], args


def one(rel):
    try:
        p = subprocess.run([EIN, "saturate", "--dump", str(repo / rel)], capture_output=True, text=True, env=env,
                           timeout=TIMEOUT)
    except subprocess.TimeoutExpired:
        return rel, "timeout", None
    if p.returncode != 0 or "SATURATED KB DUMP" not in p.stdout:
        return rel, "refused", None
    dump = p.stdout.split("SATURATED KB DUMP")[-1]
    declared, held, denied, double, nested = [], [], [], [], []

    def under(form):
        for a in cut(form)[1]:
            if isinstance(a, list) and a:
                h, xs = cut(a)
                if h != "not":
                    nested.append((h, len(xs)))
                under(a)

    for line in dump.splitlines():
        if line.startswith("(relation "):
            head, args = cut(parse(line))
            declared.append((args[0], len(args) - 1))
        elif line.startswith("  ("):
            head, args = cut(parse(line))
            form = [head, *args]
            if head == "relation":
                continue
            if head == "not":
                inner = args[0]
                depth = 1
                while isinstance(inner, list) and inner and inner[0] == "not":
                    inner, depth = inner[1], depth + 1
                h, xs = cut(inner) if isinstance(inner, list) else (inner, [])
                (denied if depth == 1 else double).append((h, len(xs)))
            else:
                held.append((head, len(args)))
            under(form)
    return rel, "ok", (declared, held, denied, double, nested)


entries = tomllib.loads((repo / "corpus/corpus.toml").read_text())["entry"]
paths = [e["path"] for e in entries if e["path"].split("/")[0] in ("examples", "tests", "stdlib")]
with ThreadPoolExecutor(max_workers=int(os.environ.get("CENSUS_JOBS", "8"))) as pool:
    rows = list(pool.map(one, paths))

status = Counter(s for _, s, _ in rows)
print(len(rows), "corpus entries:", ", ".join(f"{v} {k}" for k, v in sorted(status.items())))
KINDS = ["declared relations, by labels", "facts held at root, by arguments", "stored negations, by the denied fact's arguments",
         "negations of a negation, by the innermost fact's arguments", "facts under an arrow, by arguments"]
for i, kind in enumerate(KINDS):
    facts, programs, names = Counter(), defaultdict(set), defaultdict(set)
    for rel, s, data in rows:
        if s != "ok":
            continue
        for name, n in data[i]:
            facts[n] += 1
            programs[n].add(rel)
            names[n].add(name)
    print(f"\n{kind}\n  count  occurrences  programs  distinct names")
    for n in sorted(facts):
        print(f"  {n:5d}  {facts[n]:11d}  {len(programs[n]):8d}  {len(names[n]):14d}")
    over = sorted({name for n in names if n > 6 for name in names[n]})
    if over:
        print(f"  over six: {len(over)} names, in {len({r for n in programs if n > 6 for r in programs[n]})} programs: "
              + ", ".join(over[:40]) + (" …" if len(over) > 40 else ""))
if len(sys.argv) > 2:
    with open(sys.argv[2], "w") as f:
        for rel, s, data in rows:
            if s != "ok":
                f.write(f"{rel}\t{s}\n")
                continue
            for i, kind in enumerate(("declared", "held", "denied", "double", "nested")):
                c = Counter(n for _, n in data[i])
                f.write(f"{rel}\t{kind}\t" + " ".join(f"{n}:{c[n]}" for n in sorted(c)) + "\n")
```

</details>

<details><summary><code>cmp_discharged.py</code>: where a node holds <code>(debts-discharged)</code>, against where it owes nothing</summary>

```python
#!/usr/bin/env python3
"""cmp_discharged.py DUMPDIR | EVENTS.jsonl — where a node holds the fact (debts-discharged), against where the
kernel says it owes nothing.

DUMPDIR is an oracle dump (`ein oracle FILE --dump DIR --states`): every node's state is read from region.json.
EVENTS.jsonl is a run's events (`ein solve FILE --events F`): a node holds the fact where a firing since the
`enter` before it derived it, root's firings being those before the first `enter`."""
import json
import sys
from collections import Counter
from pathlib import Path

FACT = "(debts-discharged)"
arg = Path(sys.argv[1])
tally = Counter()
if arg.is_dir():
    d = json.loads((arg / "region.json").read_text())
    for n in d["region"]["nodes"]:
        held = any(f.split(" :rule")[0].replace(" )", ")").strip() in (FACT, "(std.debts.debts-discharged)") or
                   f.strip().startswith("(debts-discharged") for f in n["state"])
        ok = held == (n["owes"] == 0)
        tally[(n["status"], "held" if held else "not held", "agrees" if ok else "DIFFERS")] += 1
else:
    held, root_held, seen_root = False, False, False
    for line in arg.open():
        e = json.loads(line)
        if e["e"] == "fire":
            held |= any(x.startswith("(debts-discharged") for x in e["derived"])
        elif e["e"] == "enter":
            if not seen_root:
                root_held, seen_root = held, True
            h = root_held or held
            ok = h == (e["owes"] == 0)
            tally[(e["kind"], "held" if h else "not held", "agrees" if ok else "DIFFERS")] += 1
            held = False
for k in sorted(tally):
    print(tally[k], *k)
print(sum(tally.values()), "nodes,", sum(v for k, v in tally.items() if k[2] == "DIFFERS"), "differ")
```

</details>

<details><summary><code>variant.py</code>: a scratch copy of a corpus program with lines appended</summary>

```python
#!/usr/bin/env python3
"""variant.py SRC DST LINE… — a scratch copy of a corpus program with lines appended (never a corpus file)."""
import sys
from pathlib import Path

src, dst, lines = Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3:]
dst.parent.mkdir(parents=True, exist_ok=True)
dst.write_text(src.read_text() + "\n" + "\n".join(lines) + "\n")
print(dst)
```

</details>

### The merge of `std.debts`

`edbe6c3b`, 2026-11-19, by an agent of its own on the coordinator's brief, on top of S1f.21.15's close: the one
commit `6b76c164` taken with `git cherry-pick -n`, the branch's record commit left.

- **The rules are the commit's, byte for byte**: `stdlib/debts.ein`, its ten programs, `debts_rules.rs` and the 26
  drawing sources. **No added program is flagged**: each is `lattice  monotone`, and describe, `--count` and the
  oracle agree on each.
- **Fitted to master**: the page is `22-debts.md`, after `21-core.md`, no other page moving; its drawings renamed
  with it and its regions written again by the generator; the page's one flagged example shows the WARNING row; a
  sentence on the module in `AGENTS.md`'s paragraph on the standard library. Seven conflicts, the generated files
  each resolved by its generator.
- **Goldens**: `corpus_exits.txt` +54 lines, the commit's own; `corpus_shapes.md5` +440, 40 a program, no other
  line moving; `tests/mutation_survivors.txt` unmoved (676 mutants, 557 killed, the module's 39 all killed).
- **Old against new**: `ein solve FILE --json-summary` over the 921 paths that existed before, identical in exit,
  stdout, stderr and summary on every one.
- **What an import of it changes** is what its own two imports change: appended to each of 902 programs, the
  import moves 11 of them, each the same way `(import std.algebra) (import std.slots)` alone moves it (V-14 and V-16
  again). Three programs already flagged list two more static sites.
- **Checks, the agent's**: the workspace's tests, 166 of 167 binaries; the 167th, `int_theory`, was killed for
  memory with 13 GB of the session's scratch on the machine's tmpfs and passes 12 of 12 with that freed; the slow
  sweeps, fmt, clippy, rustdoc and the process's checks pass; `ein test tests/` holds 594 of 596 expectations, 2
  not checked, 0 failed.
- **Found outside it**: [V-28](findings.md#v-28--stddebts-says-debts-discharged-is-kept-by-every-larger-state) to
  [V-31](findings.md#v-31--the-module-tables-have-no-row-for-stdcore).
- **The gate and the nightly in the runner's image**, run by the coordinator with S1f.21.15's: `utils/ci_container.sh`
  on `edbe6c3b`, **exit 0**; `utils/ci_container.sh --nightly` on `c125113f`, the same code, **exit 0**.

