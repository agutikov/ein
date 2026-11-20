# S1f.21.22 — what the pages found in the build

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator, under the user's word to run the phase to its end on defaults
([the README](README.md#the-word-to-run-to-the-end)); **not started**.
**Waits on:** [S1f.21.20](s1f.21.20_what_the_first_uses_of_open_found.md), which moves where the pass runs and
the check in flight. It runs beside [S1f.21.21](s1f.21.21_the_contradictions_cost.md), which writes other files.
**Writes:** `ein-cli/src/render.rs` and the DOT views of `ein-render`, `ein-infer/src/mechanisms.rs`, the
read-out's printer, `ein-infer/src/open.rs`, their tests; comments of the engine and of two library files, with
the library's manifest; the pages and goldens that move with what is drawn or printed.
**Not its files:** `saturator.rs`, `contradiction.rs`, `compile.rs`, `plan.rs`, `match_.rs`, `ein-core/src/kb.rs`,
which are S1f.21.21's.

## What it is

[S1f.21.14](s1f.21.14_the_pages_of_open.md) wrote the pages of the second part by running the binary against
each ruling, and found the binary short of two rulings and of two of its own contracts. This stage builds those.

## Tasks

1. **[V-72](findings.md#v-72--the-kb-and-constraints-renders-draw-no-open-fact-root-holds): a drawing of root draws its `open` facts.** `ein render kb` and
   `ein render constraints` run the pass at root as every command that shows a state does, and draw what it
   writes; the view without them is the one the user ruled (Q-M1f.572, Q-M1f.577). Root is a state that is
   printed, so the pass runs there under the ruling of Q-M1f.655. The user's drawing rules hold: no arrow to or
   from an arrow, a layered layout, and a test that checks the structure and not the layout.
2. **[V-73](findings.md#v-73--a-written-open-fact-is-not-judged-against-the-lists-and-a-closure): a written `(open …)` is judged as a rule's is.** The two lists and a closure
   agree with the `open` facts root holds (Q-M1f.578): one the program writes is among them, and the load error
   names where it is written. A corpus program the check newly refuses is the stage's to show, not to keep
   loading.
3. **[V-74](findings.md#v-74--a-querys-printed-claim-over-open-does-not-read-back): what the read-out prints as a claim reads back.** A goal over `open` is
   answered; its `as :expect` row prints a claim the parser takes, or no row. Which is the stage's default.
4. **[V-77](findings.md#v-77--the-hypothesis-found-in-flight-may-depend-on-the-count-of-workers): the hypothesis found in flight is one at any count of workers.** Run it first:
   a program flagged in flight, at one job and at several, the summary's `monotonicity.in_flight` and the
   warning's line compared. If they differ, the hypothesis named is the first in the units' order, as every
   pooled result is read back. `ein-render/tests/jobs_invariance.rs` holds it.
5. **[V-75](findings.md#v-75--comments-of-the-engine-and-the-library-in-words-the-part-retired): the comments in the part's words.** Each place the record of S1f.21.14 lists,
   reworded to what the engine does; a comment that says a form is refused that loads is corrected to the
   grammar. The library's two files change in comments alone, and its manifest is written again.
6. **[V-66](findings.md#v-66--the-outside-row-does-not-see-a-programs-written-hypotheses): the `outside` row sees a program's written hypotheses.** Found by
   S1f.21.20 on the example it added: the row says no hypothesis rule proposes a relation the program's own
   `(open F)` facts are of. The row reads what the program writes as it reads what a rule concludes.

## What must hold

Every corpus answer as before; every mode against the oracle; no config option, lever or flag changes $R$; a
flagged program is claimed nothing of. `dot_wellformed.rs` on every view that moves; the shape golden's lines
that move are the two renders' and no other. A DOT writer moves, so the container's gate is part of done: the
coordinator runs it once on the phase's last commit.

## Record

Run 2026-11-20 in a worktree fast-forwarded to `master` at `0a6e6e3b`, then rebased onto `master` at `fc9173c6`
and again at `cde9ac4c` (plans, pages, READMEs and one generator; no code, and no golden moved with either). Six tasks: the plan's five, and V-66, which the brief added from
S1f.21.20's record. Scratch named `build/s22/…` is the worktree's, not the tree's. The machine was shared with
other stages throughout (load average 20 to 24 at its peak), so no time below is a measurement.

### Commits

| commit | task | what |
|---|---|---|
| `f73e4fe1` | 4 | the hypothesis found in flight is the first in the nodes' order; `jobs_invariance.rs` holds it |
| `ffba2a8f` | 6 | the `outside` row reads the hypotheses a program writes |
| `60ddcdca` | 2 | a written `(open …)` judged against the two lists and a closure |
| `af52d610` | 3 | a class that lists an `open` fact: its `as :expect` is not printed |
| `8eafcc0c` | 1 | `ein render kb` and `ein render constraints` draw the `open` facts root holds |
| `d62cfe45` | 5 | the comments, the library's two files and its manifest |
| `c72e388a` | 4 | `open.rs`' module doc on where a worker's finding is recorded |
| `f7c0285d` | | this record's first version, plans only |
| `0cd68f13` | 4 | `fan_out`'s speculated nodes under a type alias: the gate's clippy refused the tuple type |
| this record | | plans only |

The hashes are the branch's after its rebase onto `master` at `cde9ac4c`.

### Task 4 first: the hypothesis found in flight (V-77)

**Run.** X9 of the examples page, `ein solve --in-flight --jobs N examples/lattice/06_friends_tables.ein
--json-summary`, at `N` = 1, 2, 4, 8, 16: `monotonicity.in_flight` was `(sits Dan S1)` at every count, the read-out's
`in flight` row the same, and stderr byte for byte the same. That program did not show it; the code could. A probe
(`build/s22/t4/race2.ein`): eight written hypotheses `(open (s a))` … `(open (s h))`, a rule `(s ?x) → (open (t ?x))`,
and `(about (t a))` … so that each `(t x)` is numbered at root and a worker meets it without being handed back. Each of
the eight nodes of the first layer holds another hypothesis root does not. At `--jobs 1` the run names `(t a)`; at
`--jobs 16`, `(t b)` in two runs of six. **The stderr line cannot differ**: it is the `WARNING` line, which names the
file and no hypothesis (`common::read_program`'s hook ignores the text it is handed). **The summary and the read-out
could.**

**Built.** A worker's node keeps its finding (`open::deferring`, a thread-local the worker's node writes into instead
of the run's cell); `region::fan_out` records the first in the nodes' order as it reads every node back
(`open::found_above`), as every pooled result is read; a node handed back is computed again in order on the caller,
which records directly. `region.rs` is not in the brief's list of my files, and not S1f.21.21's: the fold is the one
place the units' order is known. `jobs_invariance.rs`: `the_hypothesis_found_in_flight_is_the_one_found_at_one_job`,
the probe in process under `open::at_every_state`, `(t a)` at one job, and the same at 2, 4, 8 and 16 workers, eight
runs each. Before the fix it failed (`--jobs 8, run 5: (t b)`); after it, six runs of the test and fourteen of the
CLI probe all name `(t a)`.

### Task 6: the `outside` row and written hypotheses (V-66)

**Built.** `scope::outside`'s rung where the rules and the program's own `(open F)` built H reads the written ones'
relations beside the rules', as its union arm did (one helper, `written_relations`, both arms). The module's clause 3
says so. `examples/features/24_written_hypotheses.ein`'s read-out has no `outside` row. Unit test
`a_relation_the_program_writes_hypotheses_of_is_not_outside`; `read_out.rs`'
`the_universes_scope_is_one_sentence_on_every_surface` holds the example to no row on every surface (the seven
questions, the summary, `ein oracle`'s dump). **The shape golden**: the example's seven lines moved
(`read-out[describe|maximal|minimal|single|n2]`, `region-trace`, `oracle`), each one line shorter, and no other.

### Task 2: a written `(open …)` against the lists and a closure (V-73)

**Built.** The check was right and never ran: `mechanisms::may_conflict`, which spares a program with at most one
mechanism the saturation, did not count a written `(open …)` as a writer. It does. The line names where the fact is
written: `Words::open_fact` falls back to the form that writes it, found at any depth of its argument (`form_of`
matches a nested fact now), *written at FILE:L:C*, as § 2.26 of the diagnostics page states; a closure's conflict is
placed at the written fact where the closure has no position of its own. The four shapes of the finding:

| program | the first line, after `kb load error: FILE:` |
|---|---|
| `(open (q b))`, `:hypothesis-relations (p)` | `5:31: solve: its :hypothesis-relations leaves out q, and root holds (open (q b)), written at hr.ein:4:2 — …` and a second line, *names p, and root holds no open fact of p* |
| `(open (q b))`, `:no-hypothesis (q)` | `5:24: solve: its :no-hypothesis names q, and root holds (open (q b)), written at nh.ein:4:2 — …` |
| `(open (q b))`, `(__closed__ q)` | `5:2: root holds (open (q b)), written at closed.ein:4:2, and (__closed__ q), written at closed.ein:5:2, says q is closed — …` |
| `(open q)`, `:no-hypothesis (q)` | `5:24: solve: its :no-hypothesis names q, and root holds (open q), written at nh_r.ein:4:2 — …` |

`mechanism_conflicts.rs` holds the four refused on all twenty commands, one line each, no container written, and one
that agrees (`(open (at Red H1))` beside `:hypothesis-relations (at)`) accepted, silent. **No corpus program is newly
refused**: `ein solve --no-search` on every `.ein` under `examples/`, `tests/`, `stdlib/` and `corpus/` (942 files,
`build/s22/t2/sweep.py`) refuses for a mechanism the two fixtures of the refusal and no other, each with the line it
had (`examples/broken/load/no_hypothesis_beside_open.ein`, `examples/ein-bugs/closed-and-owing.ein`).

### Task 3: a query's printed claim over `open` (V-74)

**Chosen** (Q-M1f.666): the row prints no claim, and says why. A claim lists no `open` fact (AGENTS.md: an obligation
concludes `(open R)`, "which no claim lists"), and the parser refuses one both as a listed fact and under `absent`
(`(model (absent (open (q a))))` is refused at the parse; `(model (not (open (q a))))` and `(model (think Ann (open
(q a))))` parse). So where a class lists an `open` fact, or the absence of one, `as :expect` is *not printed: these
classes list an open fact, which no claim lists*, before the other reasons. `read_out::Query::lists_open`, read off the
goal's atoms each match holds by (`names_open`: an atom headed `open`, or `absent` of one); under an ordinary relation
`open` is a name, and that class's claim is printed. `08-read-out.md`'s row names the reason. `read_out.rs`:
`a_claim_no_query_can_state_is_not_printed`, the three goals under describe, `--single`, `-n 4` and `--maximal`, and
the printed claim of the third pasted into its query and held by `ein test`, exit 0. No corpus goal reads `open` in
the kernel's position, so no golden moved and no example of the generated page.

### Task 1: the drawings of root (V-72)

**Built.** `open::root_state`: a freshly loaded root saturated and its `open` facts written by the pass, as `ein
saturate --dump` lists them. `ein render kb` reads it for a file with no `(commit …)`, a program or a saved ∅; a saved
node's file is drawn as before (its node's pass wrote its own). `ein render constraints` reads it for every file's
root. Each `open` fact is drawn by the KB view's code as any fact: an octagon whose arrow `1` is at `open` and arrow
`2` at the relation owed or at the hypothesis's node, which stands in the frame `open ctx` where root does not hold it,
as `not ctx` frames a stored negation's. No arrow to or from an arrow; the layers as before. The constraint view keeps
`(open R)`, a fact about a relation, and not `(open F)` (Q-M1f.667). `ein-render`'s `shape.rs` reads root through
the same function, so the shape golden draws what the commands draw (Q-M1f.669).

**The shape golden**: 434 lines moved, every one a view drawn from root's fixpoint and no other op:
`dot[kb-since]` 282 programs, `dot[constraints]` 76, `dot[constraints-fold]` 76, banked with `EIN_CORPUS_SLOW=1`
(the per-commit set and `tests/` alike). `dot_wellformed`, `EIN_CORPUS_SLOW=1`: 5 passed, Graphviz accepting every
view, none past the 1 000-edge skip outside `SKIPPING`. `render_doors.rs`:
`a_drawing_of_root_draws_the_open_facts_root_holds`: the seats program's five `open` facts, one arrow `2` at `seats`
and four at facts; its constraint view's one; `24_written_hypotheses.ein`, its saved ∅ (`models/01.ein`, two) and a
saved state above it (none). **The pages**: `12-rendering.md` says the state's `open` facts are drawn, with a row of
the KB view's table and `(open seats)` among the constraint view's facts. Its four pictures were checked against the
binary: the context and KB-view pictures are byte for byte what it draws, and the layers picture's program was given
`(solve :no-search)` (Q-M1f.672), with which the binary draws the page's `.dot` byte for byte; without it root holds
56 guesses of the blind enumerator. The other pages' pictures are V-85.

### Task 5: the comments (V-75)

Each place S1f.21.14's record lists, reworded to what the engine does: `obligations.rs`' module doc (*the verdict
atom*, *never stored*, *state of the node*), `scope.rs` (the ladder, the rung: *the writers that built H*),
`hypgen.rs`' `NO_SEARCH` and two docs of `mechanisms.rs` (*no rung runs*, *the generation ladder's second rung*),
`oblgen.rs` (*this module proposes the union*, *and what such a debt would*), `structure.rs` and `show.rs` (*which
writers built H*), `grammar.rs`' `notes()` (*at an open the position does not take, what it takes*), `from_ir.rs`'
test (`(not (open ?R))` loads, as a rule that concludes a stored negation; the test's doc also said `open` is no name
and an `(open …)` premise a parse error, both untrue since Q-M1f.569 and Q-M1f.571, reworded), `grammar_pages.rs`'
module doc, `obligation_rung.rs`' module doc and five of its section and test comments (the test names stay: a page
cites them), `std.bool.inputs` (*the facts that would pay what a state owes*, not the obligations rung) and
`std.closure`'s header and caveat (a closure configures the blind enumerator; beside an `open` fact of R root holds
it is refused at load, Q-M1f.578, which the caveat's *the puzzle will fail to solve* no longer was). The library's
files changed in comments alone; `python3 utils/stdlib_manifest.py --write` (28 modules), and `python3
utils/check_generated.py`: 9 generators, every file as written. `check_release_voice.py`: none speaks of the
development.

### Defaults taken (Q-M1f.666 to Q-M1f.672)

| id | question | default | the other reading |
|---|---|---|---|
| Q-M1f.666 | what `as :expect` prints where a class lists an `open` fact, or the absence of one | *not printed: these classes list an open fact, which no claim lists*, the first of the reasons a claim is not printed; an `open` under an ordinary relation is a name, and the claim is printed | no row at all; or a claim grammar that takes `open`, which the standing rule that no claim lists one refuses |
| Q-M1f.667 | which of root's `open` facts `ein render constraints` keeps | `(open R)`, a fact about a relation by the slice's own definition (Q-M1f.523), and no `(open F)`, whose argument is a fact | every `open` fact root holds, outside the slice's definition |
| Q-M1f.668 | a view of a drawing without its `open` facts | none: `ein render kb` has no option that leaves them out; `-P` is a listing's (Q-M1f.577) | a `-P`-like option on `ein render kb`, positive facts alone |
| Q-M1f.669 | the shape golden's `kb-since` view, root's fixpoint against the loaded root | drawn with root's `open` facts, as the commands draw root, so its lines move with the two renders' | a bare saturation, so only the constraint views' lines move |
| Q-M1f.670 | where the load error places a written `open` fact | *written at FILE:L:C*, the position of its head's word, a top-level fact keeping none of its own (07-diagnostics.md § 1.5), as a written closure is placed; a closure conflict at the closure where it is written, else at the written fact | the list's position alone, with no word on where the fact is |
| Q-M1f.671 | when a worker's in-flight finding is said on stderr | when the fan-out reads its node back, in order, with the hypothesis recorded; a node computed on the caller says it at once, as before | said by the worker at once, the hypothesis alone recorded in order |
| Q-M1f.672 | `12-rendering.md`'s layers picture, whose program's root holds 56 guesses of the blind enumerator | the program states `(solve :no-search)`, so the picture is still what the binary draws, byte for byte | the picture redrawn with the 56 hypotheses |

### Findings outside the stage (V-84 to V-85)

- **V-84 — `obligations.rs` still names `:priority` and a bare `(open)`.** Its module doc says `:priority` keeps a
  residual meaning, the report order among obligation rules, and `Owes`' doc orders the report by `(:priority, …)`;
  `:priority` is gone (AGENTS.md), and the order is the rules' load order (`plans_for`: *no rule has a priority*).
  `Owed::relation`, `Owes::by_relation` and `open_argument` speak of *the bare `(open)`*, which the parser refuses
  (*open takes one argument*). Not among V-75's places; comments only, no answer.
- **V-85 — nine pictures of the pages are no longer what `ein render kb` draws.** A survey (a subagent, read-only,
  the stage's binary) of every picture a page presents as `ein render kb` or `ein render constraints` output,
  `12-rendering.md`'s aside: 17 stay accurate, their programs holding no `open` fact at root (`:no-search`, or
  nothing to guess); 9 do not, root holding hypotheses the blind enumerator, a rule or an obligation writes, which none of the pictures draws:
  `user/02-guide/01-objects-and-relations.md` l. 135 (`01-tiny-program`: 5 guesses; its sentence *draws all of it*
  is untrue), `02-first-rules.md` l. 112 and l. 160 (`02-transitive-fired`, 3; `02-co-located-fired`, 12; each *cut
  to* a relation whose guesses are now drawn in an `open ctx` frame), `03-rule-families.md` l. 73
  (`03-range-elimination`, zebra2's 130, the five facts drawn being guessed hypotheses themselves), and four pictures
  of `reasoning/` drawn from `ein.rs/08-reasoning-in-code/` programs: `03-rule-evolution/diagrams/01-instance-of-instance`
  (18), `02-kb-evolution/diagrams/01-before-firing` and `01-after-firing` (21), `02-note-fixpoint` (2),
  `03-soup-saturated` (61). Sentences that say what `ein render kb` or `constraints` draws without the `open` facts:
  `ein.rs/08-reasoning-in-code/01-static-ir.md` l. 301–303 (zebra2's constraint view now draws root's five
  `(open R)` facts, its `kb` view 130 `open` facts), `user/04-reference/05-cli.md` l. 247–248. Each fix is a page's: redraw with the
  hypotheses, or give the program `(solve :no-search)` as Q-M1f.672 did. The pages are the third agent's, editing
  them now, so the stage changed none of them. The survey's table is the subagent's report, kept in this record's
  words above; its scratch is `build/s22/survey/`.

V-86 to V-89 are not used.

### The gate

`./run_tests.sh` on `0cd68f13`, the branch rebased onto `master` at `cde9ac4c`, with the default 16 test workers:
**exit 0**, every step: the ten static checks, `cargo test --profile gate --workspace` (168 test binaries, 1 388
tests, none failed, by the sum of their `test result` lines, `corpus_cli`'s cost check among them), the process's
checks (`check_generated.py`: 9 generators, every file as written) and the bench smoke. Load average 21.5 when it
started and 5.1 when it ended. Two runs before it: the first, on the tip before the second rebase, stopped at
`cargo clippy -D warnings` (`type_complexity` on `fan_out`'s tuple type, fixed in `0cd68f13`); the second, on
`fb561f3b`, that tip with the fix, exited 0 with the same counts.

On `0cd68f13`, `EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test dot_wellformed`: 5 passed, Graphviz accepting every view of every corpus program; and `… --test id_order_invariance`: 4 passed, the shape golden as banked on every corpus file, `tests/` among them.

Not run, as the brief says: the nightly, `--dev-host` and `utils/ci_container.sh`; a DOT writer moved (`ein render
kb` and `ein render constraints` draw root's `open` facts), so the container's gate is the coordinator's, on the
phase's last commit.

### What the user must see

- **Every drawing of root now draws root's `open` facts** (Q-M1f.572 as ruled), and for a program with declared
  relations and no other writer that includes **every guess of the blind enumerator**: the guide's four-line
  `tiny.ein` draws five hypotheses beside its five facts, `zebra2.ein`'s KB view 130 `open` facts. Nine pictures of
  the pages are no longer what the binary draws (V-85); the stage changed none of them, and `12-rendering.md`'s
  layers picture was kept true by giving its program `(solve :no-search)` (Q-M1f.672). Whether a drawing should
  draw the blind enumerator's guesses, or a page's picture be redrawn with them, is the user's to say.
- Task 4's race was real: `--jobs 16` named another hypothesis in flight than `--jobs 1` in two runs of six on a
  probe; never on X9; never on stderr, whose line names no hypothesis.
- A written `(open …)` beside a list or a closure that disagrees with it is now refused at load (Q-M1f.578); no corpus
  program was.
- The `as :expect` row says *not printed* for a class listing an `open` fact (Q-M1f.666).
