# S1f.22.5, task 1 — what a saved solution is, on real programs

**Stage:** [S1f.22.5](s1f.22.5_save_and_load.md), its first task, which ends with the user's look. Nothing in the
engine is built for it. **Ruling it serves:**
[Q-M1f.392](../open_questions.md#q-m1f392--a-state-file-that-reads-back-dead-through-written). **The questions it
puts:** [Q-M1f.400](../open_questions.md#q-m1f400--how-a-saved-file-states-the-choices-made) to
[Q-M1f.403](../open_questions.md#q-m1f403--the-programs-claims-in-a-saved-file), each with a default.

Two kinds of text on this page, and each block says which it is:

- **measured**: a run of the gate build, `ein.rs/target/gate/ein` (current: `cargo build --manifest-path
  ein.rs/Cargo.toml --profile gate -p ein-cli` finished with nothing to build), its command beside it, from the
  repository's root;
- **proposed**: the format the four questions' defaults give. **No `ein` reads a proposed file today.** Each is
  assembled by [`s1f.22.5_propose.py`](s1f.22.5_propose.py) from the program file's bytes, its modules' bytes and
  the commitment today's state file names.

## In plain words

**Today**, saving a solution writes a new program: the program re-printed with its modules pasted in (its comments
and layout gone), then every fact of the solution typed out, where each came from in a comment. Loading it reads
those facts as if the author had typed them. That is the bug of
[`q392_state_file_and_written.md`](q392_state_file_and_written.md), and more: every derived fact comes back as
typed, and a second save is not the first.

**Proposed**, saving a solution writes three things and nothing else: **the program file as it is**, byte for byte;
**each module it imports, as it is**; and **the choices the search made**, one form, `(commit …)`. Loading it does
what the run did: read the program, saturate it, make the same choices, saturate again. The solution comes out the
same, every fact with the origin it had, because it is computed again the same way instead of re-typed. Saving
what was loaded writes the same three things, so the same bytes.

The smallest case, both files in full, is [§ 3.1](#31-q392_demo-no-import-root-the-bug). The one that shows the
choices, [§ 3.2](#32-q392_demo_hypothesis-a-choice-that-has-a-fact-inside-it); a real puzzle,
[§ 3.5](#35-zebra-the-standard-library-one-choice-and-546-facts-derived-from-it), where the whole solution of the
Zebra puzzle is saved as the program plus **one** choice, `(commit (co-located Japanese House-5))`.

## The files

Folder [`s1f.22.5_the_format/`](s1f.22.5_the_format/), one folder a program, built by
[`s1f.22.5_showcase.py`](s1f.22.5_showcase.py). In each: `commands.txt`, every command run, its exit code and its
output, then the commands that would check the proposed file; `today/`, everything `--dump-answer` wrote, today's
second save (`second-*/`), the KB dumps (`dump-*.txt`) and their diffs (`*.diff`); `proposed/`, the proposed file
of each state shown.

| folder | program | what it shows | today's file | proposed file |
|---|---|---|---|---|
| [`00_survey`](s1f.22.5_the_format/00_survey/) | `examples/domain_elim/b_branch.ein` | every writer of a state, run once ([§ 1](#1-what-writes-a-state-today)) | | |
| [`01_q392_demo`](s1f.22.5_the_format/01_q392_demo/) | [`q392_demo.ein`](q392_demo.ein) | no import; root; the bug | 1 013 B | 802 B |
| [`02_q392_demo_hypothesis`](s1f.22.5_the_format/02_q392_demo_hypothesis/) | [its own](s1f.22.5_the_format/02_q392_demo_hypothesis/q392_demo_hypothesis.ein) | no import; a **choice** with a fact inside it | 1 092 B | 858 B |
| [`03_q392_demo_stdlib`](s1f.22.5_the_format/03_q392_demo_stdlib/) | [its own](s1f.22.5_the_format/03_q392_demo_stdlib/q392_demo_stdlib.ein) (`analysis.md`'s `h2.ein`) | **imports the standard library**; the bug through `std.math.decimal` | 180 879 B | 43 062 B |
| [`04_b_branch`](s1f.22.5_the_format/04_b_branch/) | `examples/domain_elim/b_branch.ein` | no import; one choice, facts derived from it | 3 106 B | 2 326 B |
| [`05_zebra`](s1f.22.5_the_format/05_zebra/) | `examples/zebra.ein` | the standard library; **a state that rests on one commitment** of one candidate, 546 facts derived | 113 823 B | 73 677 B |
| [`06_satisfiable`](s1f.22.5_the_format/06_satisfiable/) | `examples/binary_logic/satisfiable.ein` | **a local module** (`full_adder`) that imports the standard library; three choices | 50 915 B | 50 820 B |
| [`07_owing`](s1f.22.5_the_format/07_owing/) | `tests/region/04_owing.ein` | an **owing** node's file (`owing/01.ein`) | 9 684 B | 29 164 B |
| [`08_what_stays_silent`](s1f.22.5_the_format/08_what_stays_silent/) | `tests/stdlib/math/decimal/05_what_stays_silent.ein` | **a corpus program that has the bug today** (N-30) | 276 601 B | 130 796 B |

## 1. What writes a state today

Measured on `examples/domain_elim/b_branch.ein`: every command, its exit code and what it wrote are
[`00_survey/commands.txt`](s1f.22.5_the_format/00_survey/commands.txt), the files beside it.

| writer | command | what it writes of a state | loads as a program | the design |
|---|---|---|---|---|
| `models/NN.ein` | `ein solve FILE --dump-answer DIR` | each state of the part of R asked: the program inlined, then every fact typed, its origin in a comment | yes, and not as the state (§ 2) | **covered**: it becomes the saved file |
| `owing/NN.ein` | the same, where R is empty and its nodes owe | each node of the owing frontier, the same shape | yes | **covered**: the same form; the state owes (§ 3.7) |
| `root.ein` | `ein solve FILE --no-search --dump-answer DIR` | root, alive, owing or dead, the same shape | yes | **covered**: it is the second save of the first invariance |
| `region.json` | `ein oracle FILE --dump DIR [--states]` | each node's commitment as candidate texts (`facts`); with `--states`, its facts beyond root as text, no origin | no (JSON) | not covered: it names nodes, and a node's saved file is the program with `(commit` its `facts` `)` |
| `nodes.json`, `nodes.md` | `ein solve FILE --dump-lattice DIR` | each visited node's `commitment` and `closure` as candidate texts | no | not covered, as `region.json` |
| `forms/print-final-*.txt` | `ein solve FILE -p` / `-P` / `-f` | each listed state's facts beyond the program, no origin | no (a listing) | not covered: a view |
| `--trace`, `--region-trace` | `ein solve FILE --trace T.md` | derivations, as `(trace (step …))` and Markdown | a `(trace …)` form is Ein | not covered: a derivation log, not a state |
| the KB dump | `ein saturate --dump FILE` | root's facts by origin, `:rule r` / `:source "…"`, `written` among them | no: `ein solve` on its lines prints `kb load error: relation 'is-a' has no :why "…"`, then `duplicate relation 'is-a'` | **the dump** the second invariance compares ([Q-M1f.402](../open_questions.md#q-m1f402--what-save-and-load-are-as-commands-the-dump-they-are-held-to-and-a-files-first-lines)) |
| `.einb` | `ein kb save FILE OUT.einb` | the program resolved and import-flattened, and root's sections | yes, by every command | not this format: `.einb` is a private cache. A state saved from one: `ein solve sat.einb --dump-answer D` writes a body byte-identical to the `.ein`'s, on `satisfiable.ein`, so it carries the flattened program ([Q-M1f.401](../open_questions.md#q-m1f401--how-the-program-and-its-modules-are-in-one-file-unchanged) 1) |
| `ein show hyps` | `ein show hyps FILE` | H, one candidate a line | facts, not a state | not covered |

What reads a state file: `ein render kb DIR/models/NN.ein`, `ein render region CUBE --answer DIR` (through
`index.json`), every command given one as its program, and the tests `ein-cli/tests/answer_round_trip.rs` and
`answer_dump.rs`.

## 2. Is a state a function of the program and the choices? (measured)

The design's premise. The command:

```sh
python3 plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/s1f.22.5_measure.py --out build/s5/measure
```

([`s1f.22.5_measure.py`](s1f.22.5_measure.py); `summary.json` and `results.json` in `build/s5/measure/`.) It reads
every corpus entry the round trip reads per commit (neither `slow` nor `heavy_dump`), 870: 480 write at least one
state file, 244 are refused at load, 146 write none (R empty with no owing node, or nothing listed). They write
28 311 state files; the script compares at most 20 a program, spread evenly over its files: **1 267**. For each it
has three sides:

- **S, the state the run saved**: the file's facts, each with the origin its comment names. Its `written` facts are
  root's: only the loader states them, at load (Q-M1f.275, Q-M1f.279), and the file leaves them out;
- **B, the program and the choices as today's language can say it**: the file's program part with the commitment
  its header names appended as facts, `ein saturate --dump`, everything saturated at once;
- **L, today's load**: `ein saturate --dump` on the state file itself.

| | B: program + choices typed, saturated at once | L: today's load of the state file |
|---|---|---|
| facts not the state's | **12 states, 6 programs**, each with an `absent` | 2 states, 2 programs (N-30) |
| dead where the state is alive | 0 | **2 states**, the same two |
| `written` facts beyond root's | 0: no corpus choice has a fact inside it; [§ 3.2](#32-q392_demo_hypothesis-a-choice-that-has-a-fact-inside-it)'s does, and B is dead | **136 states, 132 programs** (the calculator's, mostly) |
| a committed fact read as typed | 2 303 facts, 826 states, 163 programs | the same |
| a derived fact read as typed | 0 | **166 479 facts**, 812 states, 345 programs |
| a derived fact's first rule another | **167 facts, 36 states, 8 programs** | (every derived fact is typed) |

**The writer itself**, on all 28 311 files: byte-identical on a second run and with `-j 4`. Under `--maximal`, 588
files: 540 read from the commitment describe's file of that state was read from, each with the same body byte for
byte; 48 read from another commitment of the same state (the class's maximal one), which differ in the origins of
the facts one commits and the other derives (zebra's: 18 facts `; hypothesis 0` on one side, `; rule …` on the
other).

**Root**, on the 480 programs: the inlined program part of a state file loads to the same root as the program,
fact for fact and origin for origin. Inlining loses no fact; it loses the text.

**Today's two invariances**, on the 1 267 files: **save → load → save** (`ein solve F --no-search --dump-answer D`,
then `cmp F D/root.ein`) is byte-identical for **0**; the second save is 2.9 times as long (1 305 483 lines to
3 773 126), since its program part is the first file re-printed, the state's facts in it. **load → save → load**
(the two loads' `ein saturate --dump` sections) is identical for 1 265: the two that differ are the dead pair, whose
`(false)` becomes a typed fact. So today's file passes the second invariance and fails at the step before both:
**the first load is not the state the run saved**.

**What it decides.**

1. **The load computes the state as the run's node does: root's fixpoint first, then the choices committed**, not
   the program and the choices saturated together. Together moves facts in 6 programs, every one with an `absent`:
   [`examples/features/07_unstratifiable.ein`](../../../examples/features/07_unstratifiable.ein)'s state
   `{(q one)}` holds `(p one)`, derived at root under `(absent (q one))` before the choice; saturated together,
   `(q one)` is there first and `(p one)` is never derived. It moves first rules in 8: in zebra's,
   `(not (co-located House-3 Ivory))` is first `std.slots.slot-occupied`'s in the state and
   `std.algebra.symmetric-negative`'s saturated together.
2. **A choice is no typed fact.** Typed, it reads as given (826 states), and what it has inside it is `written`
   (§ 3.2, dead).
3. **A derived fact is not in the file at all.** It is derived again, by the rule that derived it.
4. **"Identical dumps" can promise everything `ein saturate --dump` prints**: the facts, the `written` ones, each
   fact's origin and first rule, and their order. The node's computation is a function of the program, its
   modules and the commitment, byte for byte in every file the writer wrote; `ein-narration`'s relaxation (a fork
   resumes root's saturation, so two routes may narrate one derivation differently) does not reach it, since the
   saved state and the loaded one are computed by the one node function.

## 3. The format on the programs

Every *today* block is measured (the folder's `commands.txt` has the full output). Every *proposed* block is the
proposal. The commands that would check the two invariances on a proposed file are the same for every program,
`P` the program:

```sh
# proposed: not runnable today
ein solve P --dump-answer A                          # save: A/models/NN.ein is proposed/models/NN.ein
ein solve A/models/NN.ein --no-search --dump-answer B # load it, and save its root again
cmp A/models/NN.ein B/root.ein                       # save -> load -> save: byte for byte
ein saturate --dump A/models/NN.ein > a.txt
ein saturate --dump B/root.ein > b.txt
diff <(sed -n '/SATURATED KB DUMP/,$p' a.txt) <(sed -n '/SATURATED KB DUMP/,$p' b.txt)   # load -> save -> load
```

and, for a root's file, the run's own side: `ein saturate --dump P` against the load of `A/models/01.ein`, the same
section byte for byte (today: `today/run-vs-load-*.diff`).

### 3.1 `q392_demo`, no import, root, the bug

<table>
<tr><th>today: <code>today/models/01.ein</code> (measured)</th><th>proposed: <code>proposed/models/01.ein</code> (no <code>ein</code> reads it today)</th></tr>
<tr><td valign="top">

<pre>
;;; state 1 of 1 — written by `ein solve --dump-answer` (ein-answer/2).
;;; source:     plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/q392_demo.ein
;;; from:       ∅
;;; lines:      minimal, maximal
;;; reached by: 1 commitments
;;; minimal:    ∅
;;;
;;; Below: the program as loaded, imports inlined, and then every fact of
;;; the state (2 facts; the program's own relation declarations are not
;;; repeated). It loads, and saturating it derives nothing new: the state is
;;; a fixpoint of the program (10-answer-dump.md, every state file loads).

(item a)

(rule make () :match (item ?x) :assert (note (bad ?x)) :why "a note about {?x}")

(rule
  no-bad
  ()
  :match (written (bad ?x))
  :assert (false)
  :why "the author typed (bad {?x})")
;; ── the state ──────────────────────────────────────────────────────────

(item a)
(note (bad a)) ; rule make: a note about a
</pre>

</td><td valign="top">

<pre>
;;; A saved state of ein (ein-saved/1): the program as written, every module it
;;; imports as written, and the choices made. It reads no other file.
(program
;; Q-M1f.392, the smallest program that shows it. See q392_state_file_and_written.md beside this file.
;;
;; The author types one fact, (item a), and two rules.

(item a)

;; Rule 1 builds a note about every item. The note has a fact inside it, (bad a),
;; which nobody typed: the rule put it together from ?x.
(rule make ()
  :match  (item ?x)
  :assert (note (bad ?x))
  :why    "a note about {?x}")

;; Rule 2 is a check on what the AUTHOR TYPED: if the text of the program
;; has a (bad ...) inside some fact, the program is wrong.
(rule no-bad ()
  :match  (written (bad ?x))
  :assert (false)
  :why    "the author typed (bad {?x})")

)
</pre>

</td></tr>
</table>


**Today** (measured, `01_q392_demo/commands.txt`): `ein solve q392_demo.ein` prints `states (k_s) 1`; `ein solve
today/models/01.ein` prints `states (k_s) 0`, `unsat core (note (bad a))`, `refuted by no-bad`. The second save,
`ein solve today/models/01.ein --no-search --dump-answer today/second-models-01`, differs from the first by +13 −8
lines ([`save-load-save-models-01.diff`](s1f.22.5_the_format/01_q392_demo/today/save-load-save-models-01.diff)):
the program part is the first file re-printed, so `(item a)` and `(note (bad a))` are in it as typed facts, and the
state gains `(false) ; rule no-bad`. Loaded, that
second file is dead with `unsat core (false)`, `refuted by (given)` (N-32). The run's root against today's load
([`run-vs-load-models-01.diff`](s1f.22.5_the_format/01_q392_demo/today/run-vs-load-models-01.diff)):

```text
-  (note (bad a) :rule make)
+  (note (bad a))
+  (false :rule no-bad)
+  (written (bad a) :rule "#written")
```

**Proposed**: the file has no fact of the state, so its load is the program's root, `dump-program.txt`: `(item a)`
and `(note (bad a) :rule make)`, no `written`, alive.

### 3.2 `q392_demo_hypothesis`, a choice that has a fact inside it

The demo with the note **supposed** by an hrule instead of made by a rule. R is root and `{(note (bad a))}`.

<table>
<tr><th>today: <code>today/models/02.ein</code> (measured)</th><th>proposed: <code>proposed/models/02.ein</code> (no <code>ein</code> reads it today)</th></tr>
<tr><td valign="top">

<pre>
;;; state 2 of 2 — written by `ein solve --dump-answer` (ein-answer/2).
;;; source:     plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts/s1f.22.5_the_format/02_q392_demo_hypothesis/q392_demo_hypothesis.ein
;;; from:       {(note (bad a))}
;;; lines:      maximal
;;; reached by: 1 commitments
;;; minimal:    {(note (bad a))}
;;;
;;; Below: the program as loaded, imports inlined, and then every fact of
;;; the state (2 facts; the program's own relation declarations are not
;;; repeated). It loads, and saturating it derives nothing new: the state is
;;; a fixpoint of the program (10-answer-dump.md, every state file loads).

(item a)

(hrule
  guess
  ()
  :match (item ?x)
  :assert (note (bad ?x))
  :why "suppose a note about {?x}")

(rule
  no-bad
  ()
  :match (written (bad ?x))
  :assert (false)
  :why "the author typed (bad {?x})")
;; ── the state ──────────────────────────────────────────────────────────

(item a)
(note (bad a)) ; hypothesis 0
</pre>

</td><td valign="top">

<pre>
;;; A saved state of ein (ein-saved/1): the program as written, every module it
;;; imports as written, and the choices made. It reads no other file.
(program
;; Q-M1f.392's demo (../../q392_demo.ein) with the note supposed instead of
;; made: an hrule proposes (note (bad a)), so a state rests on a commitment
;; whose fact has a fact inside it.
;;
;; R is two states: root, and {(note (bad a))}. Today's saved file of the
;; second writes the supposed fact as a typed one, so its reload states
;; (written (bad a)) and the check below fires: it loads dead.

(item a)

(hrule guess ()
  :match  (item ?x)
  :assert (note (bad ?x))
  :why    "suppose a note about {?x}")

;; A check on what the author typed, as in the demo.
(rule no-bad ()
  :match  (written (bad ?x))
  :assert (false)
  :why    "the author typed (bad {?x})")

)
(commit
  (note (bad a)))
</pre>

</td></tr>
</table>


**Today** (measured): `models/02.ein` writes the choice as a typed fact, `(note (bad a)) ; hypothesis 0`, and `ein
solve today/models/02.ein` prints `states (k_s) 0`, `unsat core (note (bad a))`, `refuted by no-bad`. B, the choice
typed and saturated with the program, is the same file and dead too: a typed choice is the bug again.
**Proposed**: `(commit (note (bad a)))` is a choice, not text of the program; the load commits it after root's
fixpoint as the run did, origin *hypothesis*, and states no `written (bad a)`.

### 3.3 `q392_demo_stdlib`, the standard library's check

`analysis.md`'s `h2.ein`: a rule assembles the numeral `(n z x)` from a letter, which `std.math.decimal` flags only
where a file writes one.

| | today (measured) | proposed |
|---|---|---|
| the file | 180 879 B, 5 182 lines: a header (11 lines), the program and three modules inlined and re-printed (lines 12–3494), then 1 686 facts of the state, the digit tables among them | 43 062 B, 532 lines: the header (2), `(program` the file's 17 lines `)`, `(module std.math.arith` … `)`, `(module std.math.decimal` … `)`, `(module std.math.digits` … `)`, each as `stdlib/math/*.ein` holds it; no `(commit …)`: root |
| its load | `states (k_s) 0`, `unsat core (reading (n z x))`, `refuted by std.math.decimal.not-decimal-is-false` | root again: `(reading (n z x) :rule assemble)`, no `written (n z x)`, alive |
| second save | +3 395 −21 lines | the same bytes |

### 3.4 `b_branch`, no import, one choice

`examples/domain_elim/b_branch.ein`, R: root and `{(color-loc Blue H1)}`. Today's `models/02.ein` (3 106 B) is the
program re-printed (lines 12–76) and 16 facts: `(color-loc Blue H1) ; hypothesis 0`, the two given `color-loc`
facts with their sources, four `; rule bijective-properties`, eight `is-a` and `bijective`. Proposed (2 326 B): the
program's 57 lines as written, then

```lisp
(commit
  (color-loc Blue H1))
```

**Today** (measured): its load is alive and holds the facts by value, every one typed: the run's
`(functional color-loc :rule bijective-properties)` is `(functional color-loc)` (root's file,
[`run-vs-load-models-01.diff`](s1f.22.5_the_format/04_b_branch/today/run-vs-load-models-01.diff)); the second save is
+41 −11 lines.

### 3.5 `zebra`, the standard library, one choice, and 546 facts derived from it

`examples/zebra.ein`: one state, reached by 261 982 commitments; the one it was read from is a single candidate.

| | today (measured) | proposed |
|---|---|---|
| the file | 113 823 B, 1 209 lines: a header (11 lines, one of them the 63 minimal generators), the program with `std.algebra`, `std.slots` and `std.macro` inlined (lines 12–599), then 608 facts: 546 `; rule …`, 1 `; hypothesis 0`, the rest given | 73 677 B, 1 312 lines: the header (2), `(program` zebra.ein's 346 lines `)`, `(module std.algebra` … `)`, `(module std.macro` … `)`, `(module std.slots` … `)`, then `(commit (co-located Japanese House-5))` |
| its load | alive, the facts by value; 546 derived facts and the choice typed | the state: 546 facts with their rules, the choice `:hypothesis` |
| second save | +1 767 −553 lines | the same bytes |

`--maximal` reads the same state from its maximal commitment, 18 candidates, so its file is another one: the same
program and modules, `(commit` 18 facts `)` ([Q-M1f.400](../open_questions.md#q-m1f400--how-a-saved-file-states-the-choices-made) 3).

### 3.6 `satisfiable`, a local module that imports the standard library

`examples/binary_logic/satisfiable.ein` imports `std.bool.inputs` and `full_adder`, a file beside it, which imports
`std.bool.circuit` and `std.bool.gates`; `std.bool.inputs` brings `std.bool.circuit` and `std.algebra`, which
brings `std.macro`.

| | today (measured) | proposed |
|---|---|---|
| the file | 50 915 B: the program and six modules inlined (lines 12–791), then 229 facts, 122 derived, 3 committed | 50 820 B, 967 lines: `(program` the file's 25 lines `)`, `(module full_adder` … `)` (the 24 lines of `examples/binary_logic/full_adder.ein`), `(module std.algebra` …, `std.bool.circuit`, `std.bool.gates`, `std.bool.inputs`, `std.macro`, then `(commit (in a 1) (in b 1) (in cin 1))` |
| what loading needs | nothing beside it | nothing beside it: `(import full_adder)` in the program is read against the `(module full_adder …)` in the file, never the disk |
| second save | +587 −131 lines | the same bytes |

### 3.7 The owing node

`tests/region/04_owing.ein` states `(solve :no-search)`: R is empty, and root, which owes, is the one owing node,
`owing/01.ein`. **Today** (measured): it loads to the same answer, `every alive node owes`; the second save,
`root.ein`, says `root owes 1 instance` and differs by +16 −4 lines. **Proposed**: the same form as any state, here
with no `(commit …)` (the node is root); nothing in the file marks it owing, and its load says so. It is the one
program here whose proposed file is the larger, 29 164 B against 9 684 B: `std.algebra`'s text is long beside six
facts (§ 4, the size).

### 3.8 `what_stays_silent`, a corpus program that has the bug today

`tests/stdlib/math/decimal/05_what_stays_silent.ein` (and its binary twin) is **the standard library's twin of the
demo, already in the corpus**: its rule `assemble` builds `(price pear (n z x))`, which the module must not flag,
and does not. **Today** (measured): `ein solve today/models/01.ein` exits 1, `states (k_s) 0`,
`unsat core (price pear (n z x))`, `refuted by std.math.decimal.not-decimal-is-false`, and its three claims
`FAILED`. **Proposed**: 130 796 B against 276 601 B; root again, alive, its claims held as in the program.

## 4. The design, point by point

Each point is a question in [`open_questions.md`](../open_questions.md), with its options and a default; the
proposed files above are the defaults.

**[Q-M1f.400](../open_questions.md#q-m1f400--how-a-saved-file-states-the-choices-made), the choices.** Default
**(a)**: a form of its own, `(commit F …)`, after the program and its modules; each `F` a candidate of H in the
program's spelling, committed after root's fixpoint as a node commits it, origin *hypothesis*, no `written` for what
it has inside it; a root's file has none. A choice that is no candidate of H on load is refused; an owing or a dead
node's file is the same form; the commitment written is the one the state was read from.

**[Q-M1f.401](../open_questions.md#q-m1f401--how-the-program-and-its-modules-are-in-one-file-unchanged), the
program and its modules.** Default **(a)**: `(program …)` around the program file's bytes and `(module NAME …)`
around each module's, as the run read them; each read as the file it was; imports resolved against the modules in
the file, never the disk. **A saved file reads no other file.** The size, measured over the 1 267 files: 123.1 MB
today, 32.2 MB proposed; the proposed is smaller for 527 and larger for 740, the larger ones small states of
programs that import a long module (§ 3.7).

**[Q-M1f.402](../open_questions.md#q-m1f402--what-save-and-load-are-as-commands-the-dump-they-are-held-to-and-a-files-first-lines),
the commands, the dump and the first lines.** Default: no new command (save is `--dump-answer`, and `--no-search
--dump-answer` saves a loaded file's root again; load is any command given the file); the dump is `ein saturate
--dump`'s KB section, byte for byte; a loaded file is a program whose root is its state, and a state found above it
is saved with a second `(commit …)`; the first lines are one fixed line, and what today's header says of the run
moves to `index.json`.

**[Q-M1f.403](../open_questions.md#q-m1f403--the-programs-claims-in-a-saved-file), the program's claims.** A saved
file holds the program's `:expect` claims, which are about the program's R. Measured today: of the 826 state files
with a commitment, **188 (40 programs) exit 1** when solved, each a claim of the program checked against the file's
own answer (`examples/binary_logic/knights.ein`'s `models/01.ein`: *the claim lists 2 classes, the query has 1*).
Default: a file with a commitment checks no claim and says so; a root's file checks them as the program does.

## 5. What this task did not settle

- **The grammar of the three forms** (task 2's): `program`, `module` and `commit` become form heads at the top of a
  file as `relation` is, and a file's text inside a form is parsed as the file it was. No tracked program uses the
  three heads: `git grep -nE '^\s*\((program|module|commit)[ )]' -- '*.ein'` prints nothing.
- **Whether a load must check the carried standard library against the binary's**: the default notes a difference
  and loads the carried text; nothing here measured how often the two differ across binaries.
- **A choice's number.** Today's `; hypothesis N` is always `0` (N-29), so nothing today names which candidate a
  choice is; the proposal names it by its text.
- **The cap.** The measurement compares 1 267 of the 28 311 files; the writer's determinism is over all 28 311.
