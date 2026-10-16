# The language states a region — the grammar diff for the ruling

**Stage:** [S1e2.1a.4](s1e2.1a.4_the_language.md), task T1e2.1a.4.1.
**Status:** proposed 2026-10-16, and **all six questions ruled the same day**
(§ 9), with `(merged …)` respelled by the user. The user asked to see real
examples before anything is built: [`region_examples.md`](region_examples.md),
for acceptance. **Nothing in the parser has moved.**
**What it stands on:** the rulings (b), (c) and (e) of
[Q-M1e2.7](../open_questions.md#the-rulings-and-the-questions-back-2026-10-16),
the third answer's sketch
([§ The third answer](../open_questions.md#the-third-answer-2026-10-16)), the
approved interface ([`solve_interface.md`](solve_interface.md)), and one
measurement (§ 2): every corpus `:expect` read against R, taken with
`cargo run --release -p ein-infer --example expect_under_r` on 2026-10-16 at
`b09123d4`.

---

## 1. What is being decided

The user, 2026-10-16, (b) and (c):

> first of all `ein solve` study the solution region topology, and quering
> some specific subset of like min line or max line is secondary, more than
> that query goal could become incompletely specified with some solution, also
> query is not the best place for hrules - need to split solution-related
> features like hrules from queries, we solve once and then queries answer
> questions about region and about specific solutions; […] :expect has to be
> the answer for query, new updated query, and another expect has to be a full
> expected region definition, maybe (solve :hrules ...) and (solution (or
> (model ...) (model ...) ...)) and (query :goal ... :expect <only facts from
> goal, query response, multiple models, deduplicated, maybe different models
> still produce identical goals>)

and the third answer, on (e):

> full region info shows all valid commitments and all states that have
> multiple generating commitments with minimal ones; something like (region
> (model ...) (model ...) ...) (merged (to from) (to from)))

Three forms follow, and each needs a grammar before the parser moves:

1. **`(solve …)`**: the problem's lattice, stated once;
2. **the region claim**: the full expected region, in the form `ein solve`
   prints, so a run's answer reads back as its own expectation;
3. **the query's `:expect`**: an answer to one question about R, made of goal
   facts only, as projection classes.

The migration is the fourth thing. Every corpus `:expect` and every query
keyword that moves is named in § 7 before anything moves.

## 2. Today's forms, measured

Taken 2026-10-16 at `b09123d4` over the corpus roots (`examples`, `tests`,
`stdlib`, `corpus`). The forms were **parsed**, not grepped, with comments and
strings honoured; that is [M1d S1d.4.1](../../../docs/history/m1d_satisfiability/README.md#s1d41--what-closure-costs)'s
lesson. The `:expect`s are `ein test examples tests stdlib corpus
--json-report`'s rows.

| | count |
|---|---:|
| `.ein` files | 252 |
| `(query …)` forms | 169, in 169 files. **No file has two** |
| `:hrules` on a query | 19 |
| `:hypothesis-relations` | 2 |
| `:no-hypothesis` | 71 |
| files carrying at least one of those three | 92 |
| `:expect`, in files that load | 72: 44 `(model …)`, 27 `(false)`, 1 `(or …)` |
| `:expect`, in load-error fixtures | 3 (`broken/load/expect_*.ein`, one of them the `:expct` typo) |
| of the 72, on a program with no candidate (n = 0) | 57 |
| corpus symbols named `solve`, `region`, `solution`, `merged` | 0 |

**Every `:expect`, read against R.** `expect_under_r` computes each program's
cube (the oracle, `--limit 20`). It then reads each claim with today's rules
(as many models as states, matched one to one, each model closing the
relations it names) against every state of R, root's facts included. It does
the same against R's maximal line, against its minimal line, and against every
state cut to the goal's relations. The two `alt-cap-core` files have n = 70 and are past
the oracle. Route 1 says R is empty with root dead on both, and both claim
`(false)`.

| class | claims | what it is |
|---|---:|---|
| **holds over all of R** | 49 | 27 `(model …)` and 22 `(false)`. Every `(false)` here is the first kind of empty R, no alive node. 44 are in `tests/` |
| **holds on R's maximal line only** | 2 | `features/11_expect_ambiguity.ein` (R has 7 states, the line 2) and `ein-bugs/alive-set-fresh-name-declared.ein` (4 states, the line 1). Both list goal facts only |
| **pins the facts of an owing root** | 13 | n = 0 and root owes, so R is **empty of the second kind**: alive nodes, all owing. The claim lists root's facts, which today's checker reads off the open state. Under R there is no state for it to be about |
| **states a walk wrong answer** | 8 | each is a register row's witness, and keeps today's answer until the switch moves it (the register's rule) |

The 13 are `tests/stdlib/`:

- `algebra/23_total_owed`, `algebra/25_surjective_owed`;
- `bijection/01_setup_and_negatives`, `02_domain_elimination`,
  `03_range_elimination`;
- `closure/03_closed_and_owing`;
- `slots/02_negative`, `03_fill`, `04_elimination`, `07_spatial_prune`,
  `09_owed_room`, `11_owed_fill`, `13_adjacent_bwd_neg_direction`.

They are M1d S1d.2.6's twelve that moved to `Open`, plus the corner it banked.

The 8 and their rows:

| file | claim | R | row |
|---|---|---|---|
| `branching/15_lookahead_two_step_on` | 1 model | 5 states | W1 |
| `branching/16_lookahead_two_step_off` | `(false)` | 5 states | W1 |
| `ein-bugs/complete-records-stale` | 1 model | 5 states | W9 |
| `ein-bugs/alive-empty-phase1` | `(false)` | {∅} | W9, W12 |
| `ein-bugs/naf-upward-closure` | 1 model | 3 states | W3 |
| `ein-bugs/wrong-answers/fresh-name-nospawn` | `(false)` | {∅} | W1 |
| `ein-bugs/alive-set-fresh-name` | `(false)` | {∅} | W4: the oracle shares its candidate list, so R is {∅} and still not the row's `right` |
| `ein-bugs/alive-empty-interlayer` | `(false)` | 4 states | none yet: [S1e2.1a.10](s1e2.1a.10_the_register_under_r.md) files it |

**What the claims are about.** Of the 27 `(model …)` claims that hold over all
of R, **19 list facts the goal does not match**. 17 of them list facts of other
relations or other instances, and 2 list only negatives. That is
`01_grammar.md`'s rule 2, *more is allowed*, and the `tests/stdlib/` idiom of
pinning a consequence at a distance. Only 8 are pure goal projections. So
today's `(model …)` is a claim about **states**, and the corpus's claims belong
to the region claim, not to (c)'s query answer.

## 3. `(solve …)` — the lattice, stated once

```lisp
(solve
  [:hrules (<activator> …)]                ; hypothesis-generator activators
  [:hypothesis-relations (<rel> …)]        ; speculate only these
  [:no-hypothesis (<rel> …)]               ; …or never these
  [:expect <region-claim>])                ; the expected region (§ 4)
```

- **All three lattice keywords move, not only `:hrules`.** Each chooses the
  candidate list, and the candidate list is the lattice.
  [`definitions.md`](definitions.md) already files `:no-hypothesis` as a
  hypothesis hint that moves under (b). On a `(query …)`, each becomes a load
  error that names `(solve …)`.
- **At most one per file.** A second is a load error: two would be two
  lattices, which is what (b) removes. With none, the lattice is the default
  ladder's with no restriction, as it is today for a query that carries none
  of the three.
- **`solve` becomes a declarator**, so it lexes as a keyword everywhere.
  No corpus symbol is named `solve`.
- **Every query asks over the one R.** Today
  `ein-cli/src/solve.rs::run_query` re-loads and re-solves per query, because
  `:hrules` was per query. After the split, several queries share one solve.
  No corpus file has two queries, so nothing moves with it.

## 4. The region claim

```lisp
;; features/11_expect_ambiguity.ein: two people, two seats, restrictions only.
;; R is 7 states: ∅, the four single seatings, the two full ones.
(solve :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2))
  :expect (region
            (model)                                   ; ∅: root itself, no seat fact
            (model (seat Ann S1))
            (model (seat Ann S2))
            (model (seat Bob S1))
            (model (seat Bob S2))
            (model (seat Ann S1) (seat Bob S2))
            (model (seat Ann S2) (seat Bob S1))))

;; synth/merged_states.ein: A brings B, so {A} and {A,B} are one state.
(solve :expect (region
                 …
                 (model (h A) (h B))
                 (merged (model (h A) (h B)) (model (h A)))))

;; tests/stdlib/closure/03_closed_and_owing.ein: root owes, so R is empty.
(solve :no-hypothesis (is-a closed-here)
  :expect (region (owing (model …))))         ; root's facts, as today's claim lists them
```

- **A `(model …)` names a state of R**, as (e) rules. Root's facts are part
  of the state, as they are in today's checker.
- **Naming a relation closes it, across the whole claim.** The relations any
  `(model …)` names are closed in every one of them, so `(model)` beside
  `(model (seat Ann S1))` is a state with no `seat` fact. Today a model in an
  `(or …)` closes only what it names itself, which is why `:expect` cannot
  say *one model with no q fact*
  ([Q-M1e.13](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e13--expect-cannot-state-an-answer-with-an-empty-goal-extent)).
  The register spells that case with `closes`. Stored negatives stay unclosed,
  and a listed `(not …)` is checked for presence, as today.
- **The models are R's states, one to one.** Their number is the claim's
  k_s. A state of R that matches no model, or a model that matches no state,
  makes the claim false.
- **`(merged <to> <from>)`**, as the user spelled it on 2026-10-16. It gives a
  state reached by several commitments, with its minimal ones:
  - `<to>` is `(model …)`, one of the states the claim lists;
  - `<from>` is `(model …)` for its one minimal commitment, or `(or (model …)
    …)` for several. These models are **commitments**: the candidates
    committed, with nothing derived and nothing from root.

  It is **all or nothing**. A claim with no `(merged …)` is about states, and
  k_c is not claimed. A claim with one lists every such state, and then it pins
  k_c, which follows by inclusion–exclusion.
- **`(owing (model …) …)`** gives the alive, owing nodes with no alive node one
  fact up: the frontier the read-out reports when R is empty and alive nodes
  exist. It is what the 13 fixtures of § 2 state. On them it is root itself, so
  each claim keeps its facts and becomes `(region (owing (model …)))`, where
  today it reads as a `Solution`.
- **`(false)`** keeps its spelling: **every node is dead**, the first kind of
  empty R. **`(region)`** with nothing in it is R empty, either kind.

**`ein solve` prints this form.** `ein-render` gets one printer, and
S1e2.1a.5's read-out uses it for the region. A test holds that a run's printed
region, read back as `:expect`, holds on the run that printed it.

## 5. The query asks one question about R

```lisp
(query
  :goal <expr>
  [:goal-text <STRING>]
  [:over all | minimal | maximal]          ; which states it asks about (default: all)
  [:expect <answer>])
```

- **`:expect` lists projection classes.** A state's projection is the goal's
  atoms instantiated under every binding that satisfies the whole goal in it.
  A class is one projection, and `:expect` lists the classes, deduplicated: the
  user's *"maybe different models still produce identical goals"*. The forms
  are today's spellings:
  - `(model …)` is one class;
  - `(or (model …) …)` is several;
  - `(false)` is **no class**, because the chosen set is empty.
- **Only goal facts.** A listed fact that unifies with no atom of the goal is a
  load error, which names the region claim as the place for it. Today's rule 1
  (*the goal's relations are mandatory*) goes, because the closed set is now
  the goal itself.
- **An undetermined goal is `(model)`**: the class of states where the goal has
  no binding. This is the case (b) calls *incompletely specified*, ∅ among
  them. In a program with restrictions only, ∅ ∈ R and the goal has no row
  there, so a question about *the answer* is asked `:over maximal`.
- **`:over`** is the subset ruling (b) calls secondary. It takes `all` (the
  default), `minimal` or `maximal`, and nothing else (§ 9, question 6).

## 6. A flag beside a form

[`solve_flags.md` § 10](solve_flags.md#10-decided-here-unless-the-user-objects)
already decided this, and the approved interface keeps it. **Both are checked,
and neither wins.** `--single` is the run's expectation and the region claim
is the program's. Each is judged against what the run computed.

A claim the computed set cannot decide is `NOT CHECKED`, with its stderr line,
and exits 1. That covers a region claim under `--maximal`, and a query `:over
maximal` under `--minimal`. Bare `ein solve` computes R, which decides every
claim. `ein test` computes what its claims need and no more: the union of the
sets they ask about.

## 7. The migration, named

**When.** In two steps, so that no claim has two readings live at once:

- **S1e2.1a.4 (this stage):**
  - `(solve …)` and the keyword move, in the 92 files of § 2. They are
    answer-neutral, since every file has one query: every summary, trace and
    exit code is held byte-identical, and a shape digest that moves is named
    first;
  - the region claim, checked against R through `ein_infer::explore`, never
    through the walk, with new fixtures under `tests/`.

  Today's query `:expect` keeps its meaning until the switch.
- **S1e2.1a.5's switch:** the query's `:expect` becomes § 5's, and every
  existing claim moves as below, in the commit that flips the register's rows.

| class (§ 2) | claims | becomes |
|---|---:|---|
| holds over all of R | 49 | `(solve … :expect (region (model …)))`, the same facts. Meaning unchanged: a claim with one model claimed k = 1 and claims k_s = 1. The 22 `(false)` become `(solve :expect (false))` |
| holds on the maximal line only | 2 | a query `:over maximal` with the same `(or (model …))` or `(model …)`. Both are pure goal projections |
| pins an owing root | 13 | `(solve :expect (region (owing (model …))))`, the same facts |
| states a walk wrong answer | 8 | R's answer, from the `right` blocks S1e2.1a.10 restates, moved with the row's flip. `alive-set-fresh-name` moves to R's {∅}, which is still W4's wrong answer: the row is re-recorded, not fixed |

**What else moves with this stage:**

- `01_grammar.md` § Query is split into § Solve and § Query. The § 2 table
  gains the `solve` row.
- `06_reserved_names.md`: `solve` joins the declarators and the lexer keywords.
- `defined_behaviour.md`: the new load errors. They are:
  - a lattice keyword on a query;
  - a second `(solve …)`;
  - a malformed region claim;
  - a `(merged …)` whose `to` is not a listed state, or whose `from` is not
    under its `to`;
  - a query fact outside the goal;
  - a class listed twice;
  - an unknown `:over` value.
- `configuration.md`: nothing, since no `(config …)` field or CLI option is
  added here.
- `ein test --json-report` gains the fields that describe a claim:
  - `place` (`solve` / `query`);
  - `shape` (`region` / `false` / `owing` / `classes`);
  - `over`;
  - the k_s it claims, and whether it pins k_c;
  - the outcome.
- **Fixtures**, one or more per new form, under `tests/` with an `:expect`
  each: region, `merged`, `owing`, `(region)`, projection, `(model)`, and
  `:over maximal`.

## 8. What this does not do

- **No region shapes.** There is no sub-cube `[a … b]` and no `:convex`
  claim. No corpus claim needs them, and the explicit list covers every region
  the oracle reaches. A 728-state region such as `features/02`'s is the
  read-out's problem first (S1e2.1a.5), and it states no claim today (§ 9,
  question 5).
- **No component subset.** `:over` does not name a component (§ 9,
  question 6).
- **No non-monotone meaning.** On a flagged program the claim is checked
  against the cube, under *assumes a monotone lattice*, as every answer is
  until P1e2.1b.

## 9. What needs the user's ruling

**Ruled 2026-10-16, all six**, verbatim:

> Where the region claim lives, and its name: A; A state's generators:
> (merged (model <to model>) (or (model <from model 1>) (model <from model 2>)
> ...)) - so (merged <to model> (from models)); The second kind of empty R:
> yes; The query's answer: yes; Region shapes, Component subsets - don't need
> now both, the region is already defined by it's nodes; Show real examples
> for syntax to review and accept before continue

So: 1 is (A), named `region`. 2 is the user's `(merged <to> <from>)`, which
§ 4 now spells. 3 and 4 are yes. 5 and 6 are not built, *"the region is
already defined by it's nodes"*. The examples are
[`region_examples.md`](region_examples.md).

The questions as they were put:

1. **Where the region claim lives, and its name.**
   - **(A), proposed:** `(solve … :expect (region …))`. That makes one new
     declarator, and the claim sits beside the lattice it is about: the
     user's *"another expect"*, literally.
   - (B) a top-level `(solution (or (model …) …))`, the second answer's
     spelling.
   - (C) a top-level `(region …)`.

   `region` is proposed over `solution` in both (A) and (C), because under
   [`definitions.md`](definitions.md) a solution is one node of R and the
   claim states all of R.
2. **A state's generators**: `(merged (to …) (from …))` inside the claim, all
   or nothing (§ 4).
3. **The second kind of empty R**: `(owing (model …) …)`, for the 13
   fixtures. The alternative is a query asked over root, which is not a
   solution and would add a fourth `:over` value.
4. **The query's answer**: projection classes by goal rows, with `(model)` for
   an undetermined goal, `:over all` as the default, and only goal facts
   allowed. With this, the corpus's existing claims go to the region claim,
   because 19 of 27 pin more than the goal.
5. **Region shapes, not now** (§ 8). Proposed: built when a program needs one.
6. **Component subsets, not now** (§ 8). Proposed: built when a question
   needs one. `:over` takes `all`, `minimal` and `maximal` only.

## 10. Decided here unless the user objects

- **The three lattice keywords move together** (§ 3), and on a query they are
  a load error that names `(solve …)`.
- **The keyword move lands in this stage; the claim migration lands at the
  switch** (§ 7). The region claim is checked against R from the day it
  lands.
- **Closure is across the whole claim** (§ 4). It is new, and it moves no
  existing claim, because the one `(or …)` in the corpus goes to a query.
- **`(false)` keeps its spelling and means every node dead.** `(region)` means
  R empty, either kind.
- **Several queries share one solve.**
- **`ein test`'s exit codes do not change**: 1 for a false claim, 2 for a load
  error.
- **Re-estimate: 3 days**, from 2.5:
  - the parser and loader (`solve`, the region claim, `:over`, the
    projection), 1 day;
  - the checker against `explore`, and the report, 1 day;
  - the pages, `defined_behaviour.md`, the fixtures and the keyword move,
    1 day.

  The claim migration is the switch's, in S1e2.1a.5's estimate.
