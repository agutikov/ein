# S1f.21.24 — a hypothesis rule reads its `absent` premises

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator, under the user's word to run the phase to its end on defaults
([the README](README.md#the-word-to-run-to-the-end)); **built 2026-11-20** (the record below).
**Writes:** `ein-infer/src/hypothesis_rules.rs`, `open.rs` and what reads a rule's `(open F)` conclusions, their
tests; `plans/wrong_answers/register.toml` and its witness, then `corpus/regressions.toml` and the corpus; the
oracle's listing of a node; two help texts; the pages and goldens that move.

## What it is

[V-89](findings.md#v-89--a-rule-that-concludes-a-hypothesis-reads-none-of-its-absent-premises), a wrong answer the renders stage found and the coordinator checked: a rule that
concludes `(open F)` is matched without its `absent` premises. Review item 62's probe,
[`probe-hypothesis-rule-absent.ein`](review/part-2/62-what-a-fork-drops/programs/probe-hypothesis-rule-absent.ein):
one match, `(and (p ?x) (absent (q ?x)))`, under a rule that concludes a fact and under one that concludes a
hypothesis; with `(q a)` written the first concludes `(s b)` alone and the second writes `(open (r a))` and
`(open (r b))`. The program is monotone by the static check, so the engine claims $H$ of two and $R$ of four
states where the match gives one and two, and `ein oracle` agrees, building $H$ the same way. The engine before the
phase did the same.

**The reading taken, a default ([Q-M1f.685](../open_questions.md#q-m1f685--a-rule-that-concludes-a-hypothesis-reads-its-absent-premises))**:
the `absent` is read, as every rule's is. The syntax reference says a hypothesis rule's `:match` reads the state it
is asked about, and the user ruled a hypothesis rule a rule that concludes `(open F)` and nothing apart
(Q-M1f.567).

## Tasks

1. **The wrong answer filed**, by [the register's rules](../../wrong_answers/README.md): a row of
   `plans/wrong_answers/register.toml` with its witness under `plans/wrong_answers/witnesses/`, owned by this stage,
   in a commit of its own, `python3 utils/wrong_answers.py` green on it.
2. **The fix.** Where a rule's `(open F)` conclusions are read, at root for $H$ and at each state the pass writes,
   the guards the plan lifts out of the match are judged as the saturator judges them for a rule that concludes a
   fact: against the state's fixpoint. A rule that concludes a fact and a hypothesis in one `:assert` gives both
   of the same matches.
   - **The static check.** A hypothesis rule whose `absent` reads a relation a commitment can change proposes at
     root what a state above it would not: say what the check's sites make of it today, and whether a site is
     owed. A change to what is flagged is shown, program by program.
3. **The corpus.** Every program whose $H$ or $R$ moves is shown with both answers. Each is a program whose
   hypothesis rule has an `absent` premise the filter of $H$ did not already cover. Its new answer is its match's;
   an `:expect` that stated the old one flips in the fixing commit, as the register's rule has it. **Stop and
   report**, with the program, where what a program meant is not plain from its text.
4. **The row moved**, in the fixing commit: into `corpus/regressions.toml`, its witness into
   `examples/ein-bugs/wrong-answers/` with its corpus entry and its line in the catalogue.
5. **[V-91](findings.md#v-91--no-command-prints-the-open-facts-of-a-state-above-root-outside-r)**: the oracle's listing of a node lists the node's `open` facts, as a listing of
   a state does (Q-M1f.572); `-P` leaves them out (Q-M1f.577).
6. **[V-93](findings.md#v-93--pages-and-help-texts-that-say-what-the-binary-does-not)'s help texts and [V-94](findings.md#v-94--a-test-of-the-obligations-reports-checks-proposals-by-short-names-only)**: `ein show --help` and
   `ein show hyps --help` in the words the engine uses; `08-read-out.md`'s sentence on the licence; the test of
   the obligations' reports compares full names.

## What must hold

- Every mode agrees with `ein oracle` on every corpus program, and **the oracle is right on the probe**: a test
  holds $H$ to the match by a second route that does not read the function under repair, the rule that concludes a
  fact of the same match.
- A program whose hypothesis rules have no `absent` premise does not move: its `ein solve -p` and the digest of
  its events are byte for byte what they were.
- No config option, lever or flag changes $R$. A flagged program is claimed nothing of.
- The gate on the last commit, and the two slow render sweeps.

## Record

Run 2026-11-20 in a worktree fast-forwarded to `master` at `3fdf6c97`; `master` did not move during the stage.
Scratch named `build/s24/…` is the worktree's: the release binary of `3fdf6c97` (`ein-base`), of the fix alone
(`ein-new1`) and of the stage's tip (`ein-final`); `run_one.sh`, the three runs taken on every corpus file, and their
outputs under `base/`, `new/` and `final/`; the probes of the static check under `probes/`.

### Commits

| commit | task | what |
|---|---|---|
| `90b43fd2` | 1 | W99 filed open: the row `W99/hypothesis-rule-absent`, owned by this stage, its witness `plans/wrong_answers/witnesses/hypothesis-rule-absent.ein` (item 62's probe with a header written for the corpus); `python3 utils/wrong_answers.py` green on it, 10 open rows each still answered wrong |
| `9c72cdcb` | 2–4 | the fix, the second route's test, the row moved to `corpus/regressions.toml` and its witness to `examples/ein-bugs/wrong-answers/` with a corpus entry and an `:expect`; Q-M1f.685's decision file; Q-M1f.129, `02-syntax.md`, `09-hypotheses.md` and the reasoning-in-code page; the shape golden |
| `66a17ce8` | 5 | the oracle's listing of a node lists every `open` fact it holds (V-91) |
| `146a29d4` | 6 | the two help texts, `08-read-out.md`'s licence sentence (V-93), `obligation_reports.rs`'s full names (V-94) |
| this record | | plans only |

### 1. The row, as filed and as moved

Filed as `W99/hypothesis-rule-absent`, case W99 (the next free id after W98), class *model set*, `question`
Q-M1f.129, Q-M1f.567, Q-M1f.685 and V-89. `[case.now]`: `solve`, all of R, k 4, relations `r`, outside `s`, models
∅, {(r a)}, {(r a), (r b)}, {(r b)}; `[case.region]` the state route, monotone, k_s = k_c = 4, minimal ∅, maximal
{(r a), (r b)}. `[case.right]` under Q-M1f.685: k 2, models ∅ and {(r b)}, derived from the match, which holds of `b`
alone as `plain`'s one conclusion `(s b)` shows; its one alternative the other reading of Q-M1f.685, today's four.
`[case.wrong]` records the run on `ein-base`, where `ein show hyps` and `ein oracle` say the same.

Moved in the fixing commit as the regression `hypothesis-rule-absent`: `decisions` Q-M1f.129, Q-M1f.567, Q-M1f.685;
`[case.region]` k_s = k_c = 2, minimal ∅, maximal {(r b)}; `[case.right]` and `[case.now]` k 2, models ∅ and
{(r b)}, outside `s`. The witness is `examples/ein-bugs/wrong-answers/hypothesis-rule-absent.ein`, its header in the
current state and naming no row or plan, with `(solve :expect (region (model) (model (r b))))`, which `ein test`
holds; its corpus entry runs `solve`, `saturate`, `test` and `saturate --dump`. The register is back to its nine
rows. No `:expect` stated the wrong answer, so none flipped. The examples catalogue has one line for the folder and
none per file, so it gained none.

### 2. The cause and the fix

**The cause is the one S1f.21.19 read.** `HypothesisRules::candidates_by` ran `Matcher::run` over each plan and
resolved every `(open F)` template under every match, but the compiler lifts a top-level `(absent …)` out of the
match into a `NafGuard` (`Match::guards`), and a walk of the positive steps is not yet a match: the saturator judges
the guards at the boundary (`first_failing`, `Matcher::holds`), a goal's are judged in `verdict.rs`, an obligation's
in `oblgen::owed_by`, and this function judged none.

**Every reader of a rule's `(open F)` goes through it**, so one change reaches all of them: `hypgen::generate_with`
(H at root, and the mode it reports), `hypgen::eligible_relations`, `open::hypotheses_at` (the pass that writes a
state's `open` facts, at root and above it), and through them `ein show hyps`, `ein render h-space`
(`structure::hypotheses`), every route and the oracle (`region::compute` takes H from the same generator).
`mechanisms`, `scope` and the static check read the plans' shapes, not their matches. No other caller runs a
hypothesis rule's plan.

**The fix**: inside the matcher's callback, a match one of whose guards' queries has a match (`Matcher::holds`
against the state, a fixpoint at every caller) is dropped, as `verdict.rs` drops a goal's. A rule that concludes a
fact beside `(open F)` fires its fact through the saturator, whose boundary judges the same guards, so both
conclusions are of the same matches (the shape `both` below). Where a rule's every match is dropped, root holds no
`open` fact from it, and the blind enumerator runs if nothing else writes (Q-M1f.576), as it did for a rule with no
match; no corpus program is in that case.

**The second route**, `ein-infer/tests/hypothesis_rule_absent.rs`: on the witness and six shapes, each program with a
rule that concludes `(open (r …))` and one that concludes `(s …)` over the same `:match`, H's facts of `r` (the
oracle's candidates) are root's facts of `s` renamed, and at every node of the cube the node's `open` facts of `r`
are its facts of `s` renamed (`Nodes::with_fixpoints`, the pass written). The shapes: an `absent` over a relation a
rule derives at root (H `(r b)`), a nested `absent` (`(r a)`), two disjuncts one guarded (`(r b)`, `(r c)`), an
`absent` before its binding, *is there no q at all* (none; a written `(open (u z))` keeps the blind enumerator out),
`std.macro`'s `unknown` (`(r c)`), and one rule that concludes both (`(r b)`). Each is monotone by the static check,
asserted, and R is every commitment over H. A third test is the `absent` over the hypothesis's own relation (§ 3).
**On the function as it was, all three tests fail** (H `(r a)`, `(r b)` on the witness; every node holding both).

### 3. The static check

`monotone::classify` reads a hypothesis rule's plans for two sites: a read of `open` (`open-read`, Q-M1f.569) and a
read of a relation in D at even depth (`hypothesis`, Q-M1f.574), where growing what is read adds a match and a state
above root can hold a hypothesis root does not. **At odd depth no site is owed** (Q-M1f.686 below): growing the
relation takes a match away, so a state above root proposes fewer of root's hypotheses and none root does not; R is
the commitments over root's H that are alive and discharged, and neither reads what a state above root proposes (a
rule that reads `open` is its own site). The probes, on `ein-base` and `ein-final`:

| probe (`build/s24/probes/`) | the `absent` | verdict, both binaries | R | above root, before → after |
|---|---|---|---|---|
| `odd.ein` | `(absent (r ?x))`, the hypothesis's own relation, depth 1 | monotone | 4 states, both | every state holds `(open (r a))`, `(open (r b))` → each the ones it does not hold as facts; {(r a), (r b)} none |
| `odd-other.ein` | `(absent (t ?y))`, `t` derived from `r`, depth 1 | monotone | 4 states, both | every state holds both → none above root |
| `even.ein` | `(absent (and (p ?y) (absent (r ?y))))`, depth 2 | flagged, 1 site (`hypothesis`) | 16 states → 4 | H was four candidates, `(u a)` and `(u b)` proposed at root though not every p is r; now H is `(r a)`, `(r b)`, and {(r a), (r b)} holds `(open (u a))`, `(open (u b))`, first held above root |

So the even-depth site, written for a match that reads its `absent`, now names a read the engine performs (V-97).
**No corpus program changes flag**: every `lattice` row of `ein solve -p` is what it was (§ 4). The third test holds
`odd.ein`'s shape: monotone, R every commitment, each state's `open` facts root's less what the state holds.

### 4. The corpus

All 931 entries of `corpus/corpus.toml` at `3fdf6c97`, on `ein-base` and on `ein-final` (and on `ein-new1`, the fix
alone, with the same result): `ein show hyps FILE`, `ein solve FILE -p`, and the md5 of `ein solve FILE --events
build/s24/ev/<file>.jsonl`, one path a file, each with its exit code. **H and R move on no program.** Two programs
move, both with a hypothesis rule that reads an `absent`, and only in what their states hold under `open` and in one
`hyp` event:

| program | its rule's `:match` | before | after |
|---|---|---|---|
| `examples/ein-bugs/wrong-answers/hrule-reads-absent-not.ein` | `(and (is-a ?g Guest) (absent (not (paid ?g))))`, `(not (paid Bob))` written | H `(paid Ann)`, R ∅ and {(paid Ann)}; both states hold `(open (paid Ann))` and `(open (paid Bob))`; the events narrate `(paid Bob)` filtered as `negated_fact` | the same H and R; both states hold `(open (paid Ann))` alone; no `hyp` event for `(paid Bob)`, which no match gives |
| `examples/ein-bugs/wrong-answers/hrule-reads-unknown.ein` | `(and (is-a ?g Guest) (unknown (paid ?g)))` | the same H and R; both states hold both `open` facts | the same H and R; ∅ holds `(open (paid Ann))` alone and {(paid Ann)} holds none, its `(absent (paid Ann))` failing there |

Every other program is byte for byte what it was on the three runs. The corpus's other hypothesis rules with an
`absent` premise are `corpus/fuzz_findings/hrule-reads-not.ein`'s `hyp-0`, which has no fact to match, and two
programs under `examples/broken/load/`, refused at load. Nothing was unclear in what a program meant: no stop.

**The shape golden** moved on exactly those two programs' `dot[kb-since]` and `states` lines, and gained the
witness's lines; looked at before banking.

### 5. The oracle's listing of a node (V-91)

`region::node_in_region` kept, under `--states`, the node's facts beyond root's, which drops every `open` fact root
holds too, though a fork holds none of root's and the node's pass writes its own. The listing is now the node's
facts beyond root's and every `open` fact it holds: on `05_owed_seats_bounded.ein` the node {(seats Ann C1)} lists
four, `(open (seats Ann C2))`, `(open (seats Bob C1))`, `(open (seats Bob C2))`, `(open seats)`, where it listed
none, and root's node its five. The node's key, class and counts are unchanged; only the text moves, which no golden
reads. `ein oracle -P` lists the states of R without their `open` facts, as before (Q-M1f.577). `region_dump.rs`
holds the four and the five; the help text of `--states`, `11-region-dump.md` and `05-cli.md` say so.

### 6. Help texts and a test (V-93, V-94)

`ein show hyps`' `about`, which `ein show --help` prints too, said the note names *which rung* built H: it names
*which writers*, the hypotheses pages' word. `08-read-out.md`'s `route` row no longer says L9's relation is one *the
kernel does not mirror*. `obligation_reports.rs`'s `a_blind_run_never_proposes_an_obligation_activator` checks each
obligation rule's name under every spelling `ein_corpus::spellings` gives, the full names among them, which it now
asserts. The two pictures of V-93 are item 77's and were not touched.

### Defaults taken (Q-M1f.686 to Q-M1f.689)

| id | question | default | the other reading |
|---|---|---|---|
| Q-M1f.686 | is a site owed for a hypothesis rule that reads, under an odd number of `absent`s, a relation a commitment can change | no: such a read takes a hypothesis away above root and adds none, so nothing is first held above root (Q-M1f.574), and R, over root's H, reads nothing a state above root proposes; the check is left as it is | a site, `hypothesis` at every depth: a program whose states above root propose other than root did is flagged, and nothing is claimed of its answer |
| Q-M1f.687 | what the oracle's listing of a node shows, and `-P` beside it | `--states` lists the node's facts beyond root's and every `open` fact it holds, root's node included; `-P` keeps leaving them out of the states of R it lists, and no positive form of `--states` is added | the listing keeps to what is beyond root, an `open` fact listed only where root does not hold it; or `-P` beside `--states` drops the `open` and `(not …)` facts from every node's listing |
| Q-M1f.688 | whether Q-M1f.685, a default, gets a decision file now | yes, in the fixing commit, as Q-M1f.590, a default, has one: the regression's `decisions`, a code comment and the pages cite it, and a decision taken by default and one ruled are alike (the decisions' README) | it stays in `open_questions.md` until the user rules it, and the project cites Q-M1f.129 and Q-M1f.567 alone |
| Q-M1f.689 | what the moved witness states | a `(solve :expect (region (model) (model (r b))))`, the row's answer, with a `test` run; none while the row was open, so the register's `solve` run exited 0 | no `:expect`, the row alone holding the answer |

### Findings outside the stage (V-95 to V-97)

- **V-95 — review item 62's stand-in moves with the fix.** `programs/seats-asked.built.ein` reads its debts through
  `(absent (debt-paid …))` in its hypothesis rules: its H (four candidates, `union`) and R (two states) are
  unchanged, but its two states of R no longer hold the four `(open (seats …))` facts, the debts being paid there
  (`ein solve -p`: 23 facts → 19 each). The item's runs on the built engine were taken before the fix, so what its
  page says the states of R hold under `open` is the engine's before this stage. Not touched: the review folder is
  the coordinator's.
- **V-96 — a test comment still says *which rung built H*.** `ein-infer/tests/later_debts.rs`' module comment, the
  word the two help texts dropped. Not changed: outside the stage's list.
- **V-97 — the even-depth `hypothesis` site was, until this fix, one no run could exercise through a nested
  `absent`.** A hypothesis rule's nested query was not read, so `even.ein` (§ 3) proposed `(u a)` and `(u b)` at root
  on a program the check flags; now the state that makes the query hold is the one that holds them, found in flight.
  No corpus program has such a rule, so no answer moved.

### The gate

GATE-PENDING

### What the user must see

- **The wrong answer is fixed**: a rule that concludes `(open F)` reads its `absent` premises as every rule does, so
  the probe's H is `(r b)` and R two states, where the engine claimed four on a monotone program. The second route,
  the rule that concludes a fact over the same match, holds it at root and at every node; the oracle builds H through
  the same function and could not catch it.
- **The corpus**: H and R move on no program; two programs' states hold fewer `open` facts, the hypotheses their own
  match never gave; every other program byte for byte as it was. No program changes flag.
- **The static check owes no site** for a hypothesis rule's odd-depth read of what a commitment changes (Q-M1f.686,
  a default): such a state proposes fewer of root's hypotheses and none of its own.
- **The oracle's `--states` lists a node's `open` facts** (V-91), and the help texts, the read-out page and the
  obligations' test say what the engine does (V-93, V-94).
- **Item 62's stand-in** holds other `open` facts on its states of R since this fix (V-95).
