# S1f.21.13 — the search and rules

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**done 2026-11-20**: the example `2b63821b`, `54cc0002`, `4b2a3026`; the record below.
**Group:** [I](modifications.md#i-the-search-as-rules), items 36 and 37, with the three rules of item 35 that
steer a search.
**Waits on:** [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules),
and [S1f.21.12](s1f.21.12_hypotheses_as_facts.md). **It is planned here under the recommendation: the engine's
search is not changed in this phase**, and the stage keeps what costs nothing and records what is declined and
why. A ruling for either of the user's two uses plans it again. **Since planned**: the user ruled that a rule may read
an `open` fact, in a program that is then not monotone and of which nothing is claimed
([Q-M1f.569](../open_questions.md#q-m1f569--what-may-read-an-open-fact-and-when)): *"That allows experimenting."* So
the example below may be written with `open` once [S1f.21.12](s1f.21.12_hypotheses_as_facts.md) is built, and
what is declined stays declined.
**Writes:** one program under `examples/` with its `corpus.toml` entry and its line of the catalogue; this
folder's pages.

## What is kept

1. **The lattice written as rules, as an example**: [item 34](q-m1f.549_hypotheses_as_facts/README.md#4-the-lattice-written-as-rules-row-5)'s
   `two-worlds.ein` without its last rule. A world is a fact naming its commitment, `(in W F)` says it holds F,
   the program's rule is written under `(in ?w …)`, and `in-R` is a rule over the absence of `(in ?w (false))`. It
   states an `:expect` for `in-R`, for the dead world and for `possible`, which today's engine answers as the
   oracle answers the object program beside it. It is item 37 as a program a reader can run, and shows a search
   artefact read by a rule.
2. **$R$ with no search**, the statement for the reasoning pages (item 13), handed to
   [S1f.21.2](s1f.21.2_the_pages.md).

## What is declined, and why

| what | why | what would reopen it |
|---|---|---|
| **search control from inside Ein**: rules that conclude what to split on, that a debt is forced, that a state is hopeless (items 35 and 36) | the tree route takes the owed instance with the fewest alternatives left, the first by text among equals, and a debt with none has no child (licence L5): *order*, *forced* and *hopeless* are done. No corpus program is known to want another order. Such a rule reads *still open*, so its conclusion is no held fact: it needs a pass over each quiescent state, as an obligation has | a program whose search is slow by its order and fast by another, measured; then the pass is a kernel verb a rule concludes, on S1f.21.12's facts, with no change to them |
| **the search as rules in the engine**, every state a context (item 37) | $2^{\lvert H \rvert}$ worlds where the tree takes 7 saturations of *zebra2*'s $2^{56}$; every rule written again under a context, with no form that lifts one; each world's absences need the strata followed | [M1n](../../m1n_non_monotone_programs/README.md)'s stratum-order admission, and a form that lifts a rule |
| **facts inferred from the search** | the first rule over a second absence answers wrong today ([V-8](findings.md#v-8--the-search-written-as-rules-answers-wrong-at-its-third-stratum)); and a fact the search writes back into a state changes that state, so $R$: the walk's writebacks went for that | M1n's P1n.2; then such a rule is one stratum more of the example above |

## What must hold

The example's claims are held by `ein test`, and the object program beside it by the oracle; a `.ein` under
`examples/` has its corpus entry; no engine file changes.

## Acceptance

`ein test` on the example prints `ok`; V-8 is handed to M1n at the phase's close with its witness whole; the
declined rows are in the phase's close as what was left and where.

## Record

Run 2026-11-20 in a worktree fast-forwarded to `master` at `bbf64c39`, on one release build of that tree (`ein 0.1.0`,
`protocol ein-events/1`) with the checkout's library; the gate's runs on the gate profile. No engine file, page of
`docs/` or library module changed. The probes named below as `build/s13/…` are scratch in the worktree, not in the
tree.

### 1. The example

**Where and what.** Two files under [`examples/lattice/`](../../../examples/lattice/), one number for the pair, as
`06_friends_tables.ein` and its twin share one (Q-M1f.631):

- [`18_two_candidates.ein`](../../../examples/lattice/18_two_candidates.ein), **the object program**: the page's
  `two.ein` in the language as it is now. Its `(hrule guess …)` and `(solve :hrules (guess))` are a rule that
  concludes the hypothesis, `(rule guess (?R) :match (thing ?x) :assert (open (?R ?x)) …)`, and its activator as a
  fact, `(guess s)`; `clash` and the obligation `some-s` are the page's. It states a region claim and the query's.
- [`18_two_candidates_as_rules.ein`](../../../examples/lattice/18_two_candidates_as_rules.ein), **the lattice written
  as rules**: the page's `two-worlds.ein` without its last rule, `always`, and the `std.macro` import only `always`
  read. Its candidates are the page's, facts the program writes, `(question (s a))` and `(question (s b))`, and its
  header says they are the hypotheses the object program's root holds as `(open F)` (Q-M1f.632, below); a verdict the
  page did not state, `owes`, is added, and every relation is declared with its `:why` (Q-M1f.633). The page's
  `(thing a) (thing b)` stay, though no rule here reads them: dropped, `a` and `b` are named only inside a candidate,
  and `ein-infer/tests/alive_invariant.rs`' `every_name_of_the_program_is_a_candidate_object`, whose banked list
  `SEEDED` names each corpus program with such a name, failed the gate on `162e99e9` with the two new pairs
  (`` `a`, in (question (s a)) ``, `` `b`, in (question (s b)) ``); the facts went back in `4b2a3026` rather than the
  list growing, the test file being the engine's.

**What it shows**: each node of the object program's lattice, as `ein oracle --dump` reads it, is a world, and each
verdict a fact root holds:

| the object program, `ein oracle --dump D` (`D/region.md`, *Every node*) | the program as rules, at root |
|---|---|
| `∅`, owing: *owes s of something is owed* | `(owes (w))` |
| `{(s a)}` R; `{(s b)}` R | `(in-R (w (s a)))`, `(in-R (w (s b)))` |
| `{(s a), (s b)}` dead, *core (s a) (s b) by clash*; N = `{(s a), (s b)}` | `(in (w (s a) (s b)) (false))` |
| the query `(s ?x)`: the classes `a` and `b` | `(possible (s a))`, `(possible (s b))` |

`ein saturate --dump` of the program as rules: `split-1`, `split-2`, `clash-in`, `paid-in` at stratum 0, `owes`,
`in-R`, `possible` at stratum 1, `;; Strata — stratified, 2 strata`, `13 firings`.

**What the static check says: neither is flagged.** Each prints `lattice monotone (the static check) — R is convex:
no hole`, and no `WARNING` row.

**Which it is, and how it got there.** The example was first committed reading the candidates as the language now
writes them, `(open (s a))` and `(open (s b))` written by the program, its split rules over `(open ?f)` (`2b63821b`).
That form is flagged, two `open-read` sites (`site split-1: open-read reads open at absent depth 0, concludes a
positive fact — at stake: the state`), and `ein test FILE` held its four claims for whoever asks. **It failed the
gate**: `./run_tests.sh` on `54adb21a` stopped at `answer_round_trip`, on that file alone, *models/01.ein: the first
load is not the state the run saved: … BACKGROUND (18 facts) where it should be (16 facts)*: a written `(open F)`
loads with the saved file and the run's node holds none (V-61). Its other form, the candidates concluded by the
object program's `guess`, prints a loader's `note:` before the `WARNING` line, which `non_monotone_warning.rs` refuses
on a corpus program (V-62). Neither can be fixed in a stage that changes no engine file, so the example went back to
the page's spelling (`54cc0002`): **not reshaped to escape the flag, but to keep the gate green** with the engine as it
is, and the plan's acceptance, `ein test` holds its claims, met by a suite. Both `open` forms are kept beside this
record as the witnesses, [`as_rules_over_open.ein`](s1f.21.13_the_search_and_rules/as_rules_over_open.ein) (V-61) and
[`as_rules_over_open_concluded.ein`](s1f.21.13_the_search_and_rules/as_rules_over_open_concluded.ein) (V-62); once
both are fixed the example can read `open`, and is then flagged and held by no suite.

**`ein test`**, on each:

<!-- output -->
```text
$ ein test -v examples/lattice/18_two_candidates.ein
ok           examples/lattice/18_two_candidates.ein
  solve · :expect (region …) — holds (state route, k_s = 2, k_c = 2)
  :goal (s ?x) — holds (describe: all of R, 2 classes over 2 states)
1 file, 2 expectations: 2 held, 0 FAILED, 0 not checked, 0 errors  (0.01 s)

$ ein test -v examples/lattice/18_two_candidates_as_rules.ein
ok           examples/lattice/18_two_candidates_as_rules.ein
  query 1 of 4 · :goal (in-R ?w) — holds (--single: all of R, 1 class over 1 states)
  query 2 of 4 · :goal (in ?w (false)) — holds (--single: all of R, 1 class over 1 states)
  query 3 of 4 · :goal (owes ?w) — holds (--single: all of R, 1 class over 1 states)
  query 4 of 4 · :goal (possible ?f) — holds (--single: all of R, 1 class over 1 states)
1 file, 4 expectations: 4 held, 0 FAILED, 0 not checked, 0 errors  (0.00 s)
```

Both exit 0.

**`ein oracle`**, on the object program (exit 0; the rows that differ from `ein solve`'s):

<!-- output -->
```text
  ∅                 not in R — root owes 1 instance
    owed            s of something is owed
  N                 1 minimal dead set
  E                 2 minimal discharging sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          2 candidates, 0 dead alone
  route             2, the cube — no licence: asked for by name
  work              4 saturations · 5 facts beyond root

  (region
    ;; state 1 of 2 — 1 candidate · minimal and maximal · from {(s a)}
    (model (s a))
    ;; state 2 of 2 — 1 candidate · minimal and maximal · from {(s b)}
    (model (s b)))
  …
  :expect           holds

  (solve :expect)   holds

the cube   0 dead nodes with an alive superset · 0 discharged nodes with an owing superset — the static verdict holds
```

and on the program as rules, `route 2, the cube — no licence: asked for by name`, `work 1 saturation · 0 facts beyond
root`, R `{∅}`, its four `:expect` each `holds`, *the cube … the static verdict holds*; exit 0.

**The corpus entries** ([`corpus/corpus.toml`](../../../corpus/corpus.toml)):

| file | runs |
|---|---|
| `18_two_candidates.ein` | `solve`, `test`, `saturate`, `render lattice`, `oracle --dump {out}/region` |
| `18_two_candidates_as_rules.ein` | `solve`, `test`, `saturate`, `saturate --dump`, `render kb` |

Every cell exits 0 (`corpus_exits.txt`, ten lines added); the shape golden gained 80 lines, 40 a program, and no
other line moved. The catalogue's `lattice/` row has a sentence for the pair
([`examples/README.md`](../../../examples/README.md)).

**V-8, re-taken on this build**: it holds. The page's `two-worlds.ein`, whole, answers `(always ?f)` with `(s a)` and
`(s b)`, `always … stratum 2`, `3 strata`, and the read-out says `lattice monotone (the static check)`; the `open`
form with `always` added over `(open ?f)` (`build/s13/v8-open.ein`) answers the same, flagged. The witness to hand to
M1n at the close is the page's program, unchanged.

### 2. $R$ with no search, for the pages

The statement for the reasoning pages (item 13), for [S1f.21.14](s1f.21.14_the_pages_of_open.md) to write; the
letters $\mathcal{U}$, $w_C$, $\operatorname{in}$ and $P^{\mathrm{in}}$ are in neither table of
[the notation](../../../docs/ein/reasoning/README.md#notation) and go into the second for the page that states it.

**In words.** $R$ needs no search to be stated. Each commitment $C \subseteq H$ is a world, named by a fact $w_C$, and
each fact $f$ of its state is held once more as $\operatorname{in}(w_C, f)$. The worlds read nothing of one another,
so the whole lattice is one knowledge base, their union, which the program's rules, each written again inside a world,
reach in one saturation. A world is dead where a rule that states the engine's word would match inside it, and since
that word ends a state, every world at once if it were stated, the rule written inside a world concludes
$\operatorname{in}(w, \bot)$ instead; a world owes where an obligation's instance holds inside it unpaid; and $R$ is
the commitments whose world is neither. Nothing in that statement chooses, orders or splits. A search is a way to
decide the same $R$ while building only some of the worlds; the oracle builds all $2^n$ of them, one saturation a
world, and is the statement read by brute force, which every route is held to
([Q-M1e2.32](../../../docs/decisions/answer/Q-M1e2.32-no-two-commands-disagree-and-the-oracle-is.md)). The verdicts
read a world only once its own facts are complete, so they are a stratum above them; a fact over the whole of $R$,
what holds in every state of $R$, reads the verdicts and is one stratum higher again.

**As a formula.** For a rule $\rho$ of $P$, $\rho^{\mathrm{in}}$ is $\rho$ with every premise, absence and conclusion
$g$ written $\operatorname{in}(w, g)$ for one fresh variable $w$, and, where $\rho \in P_{\mathsf{inc}}$, the
conclusion $\operatorname{in}(w, \bot)$ in the place of the word; $P^{\mathrm{in}}$ is every such rule. Then

$$
\begin{aligned}
\mathcal{U} &= \operatorname{sat}_{P^{\mathrm{in}}}\Bigl(\,\bigcup_{C \subseteq H} \lbrace\, \operatorname{in}(w_C, f) \;:\; f \in \mathrm{root} \cup C \,\rbrace \Bigr) \\
\operatorname{st}(C) &= \lbrace\, f \;:\; \operatorname{in}(w_C, f) \in \mathcal{U} \,\rbrace \\
R &= \lbrace\, C \subseteq H \;:\; \operatorname{in}(w_C, \bot) \notin \mathcal{U} \;\land\; \operatorname{Owed}(\operatorname{st}(C)) = \emptyset \,\rbrace
\end{aligned}
$$

where the saturation reads each world's absences at that world's own fixpoint. Two things to say beside it: a dead
world goes on saturating where the engine's state ends at the firing, so it may hold more than the dead state does,
and it is in no $R$ either way; and the second line is the page's $\operatorname{st}(C) = \operatorname{sat}(\mathrm{root}
\cup C)$, of which this is the reading in one knowledge base. The example is this statement for $n = 2$, the rules
lifted by hand, $\operatorname{Owed}$ written as `paid-in` and `owes`, and the last stratum left out (§ 3, row 3).

### 3. What is declined, checked against the engine as it is now

| row | what still holds, checked how | what no longer holds | what would reopen it |
|---|---|---|---|
| **search control from inside Ein** (items 35, 36) | The order is done: `ein solve --single examples/zebra2.ein` prints `route … branching on the owed instance with the fewest alternatives left, the first by text among equals … L5 …`, `universe 56 candidates`, `work 7 saturations`; a describe reads the couplings first once they are read, then the same keys (`order.rs`, `Choice::Coupled`). *Hopeless* is L5, as [Finding R](../../../docs/ein/reasoning/02-kb-evolution/07-finding-r.md) states it. No corpus program is known to want another order: nothing measured since | *"Such a rule reads still open, … it needs a pass over each quiescent state, as an obligation has"*: S1f.21.12 built that pass (`ein_infer::open`: `(open R)` and `(open F)` at each state's fixpoint, in a layer no fork inherits), and a rule may read it (Q-M1f.569), the pass and such rules repeating until nothing changes. So a rule that reads *still open* is written today; what it can conclude is a held fact or the engine's one word, and either flags its program: the probe `build/s13/hopeless.ein`, a rule over `(open s)` and `(absent (open (s ?x)))` that states `:ein inconsistent`, prints `site hopeless: open-read reads open at absent depth 0, states :ein inconsistent — at stake: the state, and dead with it`. **The decline holds, on this reason beside the first**: search control written in Ein today flags its program, and nothing is claimed of a flagged program's answer | a program whose search is slow by its order and fast by another, measured; then a word to the engine under `:ein`, the form a rule has for one since Q-M1f.399 and which has one word (Q-M1f.550), that a rule over `open` facts states, the routes read and no state holds; and a ruling that such a rule does not flag its program |
| **the search as rules in the engine**, every state a context (item 37) | The cube against the tree: *zebra2* is 56 candidates, $2^{56}$ worlds, where `--single` takes 7 saturations and the describe 120 (`work 120 saturations`, `commitments (k_c) 261 982`). No form lifts a rule: a macro wraps a pattern and the rule is written again by hand ([`beliefs/08_global_and_wrapped.ein`](../../../examples/beliefs/08_global_and_wrapped.ein)), and the example lifts `clash` so. Each world's absences need the strata followed: V-8 holds (§ 1) | Nothing of the reason; three of its premises moved, two of them each way. **The contradiction is rules** (S1f.21.11): a program's pair check can be lifted with its other rules, `clash-N` under `(in ?w …)`, where the kernel's check had no rule to lift. **The engine's word ends a state** (S1f.21.10): a lifted rule cannot state it, since it ends every world at once (`build/s13/word-in-a-world.ein`, `clash-in` stating `:ein inconsistent`: `states (k_s) 0`, `refuted by clash-in`, every claim FAILED), so a world's death is a fact, `(in W (false))`, which `std.core`'s rule does not read. **$H$ is held at root** (S1f.21.12): a rule can make the worlds from the `open` facts, as the example's first form did, and its program is flagged | M1n's stratum-order admission, and a form that lifts a rule; as an engine, also a ruling on Q-M1f.569 for a program that reads `open` and builds no node but root, of which nothing can go stale (the example's first form) |
| **facts inferred from the search** | V-8 holds on this build, in the page's spelling, monotone by the static check, and over `open`, flagged (§ 1). A fact the search writes back into a state changes it: written today as a rule that reads an `open` fact and concludes a held fact, which a fork keeps while it drops the `open` fact; [`features/23_a_rule_reads_open.ein`](../../../examples/features/23_a_rule_reads_open.ein) is one, its `(seen r)` held where no `(open r)` is | Nothing: S1f.21.12 made the writeback writable and the static check flags it (`open-read`), so what the row declined is now refused an answer by the flag | M1n's P1n.2, whose D2 is the stratum-order admission; then `always` is one stratum more of the example |

The acceptance's two other lines are the close's: V-8 to M1n with its witness whole (the page's `two-worlds.ein`), and
the three rows above in the close as what was left and where.

### Defaults taken (Q-M1f.631 to Q-M1f.633)

| id | question | default |
|---|---|---|
| Q-M1f.631 | where the example goes and its name | `examples/lattice/`, the commitment lattice and $R$, at the next number, 18; the pair shares it, `18_two_candidates.ein` and `18_two_candidates_as_rules.ein`, as `06`'s twins share theirs |
| Q-M1f.632 | how the program as rules reads the candidates | as the page wrote them, facts of its own, `(question F)`, its header saying they are what the object program's root holds as `(open F)`: the program is monotone and `ein test` holds its claims. The language's form, `(open F)`, read by the split rules, is item 37's *a rule makes the child worlds from the open facts* and flags the program; written by the program it fails the answer dump's round trip (V-61), and concluded by `guess` it prints a `note:` before the `WARNING` line (V-62). It was the first commit, and is kept beside the record as the two witnesses |
| Q-M1f.633 | what the program as rules states besides the page's | `owes`, a world alive that pays nothing, as the object program's root owes, an absence over relations of the first stratum, so that each of the three verdicts is a fact; each relation declared with its `:why`; the query for it with its claim |

Q-M1f.634 and Q-M1f.635 are not used.

### Findings outside the stage (V-61 to V-63)

- **V-61 — a written `(open F)` breaks the answer dump's round trip.** A program that writes an `(open F)` fact holds
  it at root, and only there (Q-M1f.566); the node the run builds for $\emptyset$ is a fork of root, which drops the
  layer, and holds none. `--dump-answer` saves $\emptyset$ as the program, which loads to root, with them.
  `ein-cli/tests/answer_round_trip.rs` refuses the difference: on
  [`as_rules_over_open.ein`](s1f.21.13_the_search_and_rules/as_rules_over_open.ein), as the example's first commit had
  it under `examples/lattice/`, the gate on `54adb21a` failed with *examples/lattice/18_two_candidates_as_rules.ein:
  models/01.ein: the first load is not the state the run saved: line 2: ";; ── BACKGROUND (18 facts) ──" where it
  should be ";; ── BACKGROUND (16 facts) ──" (45 lines against 42)*; `ein saturate --dump` of the saved file lists the
  two `(open (s …))` among the 18. No corpus program that loads writes an `(open …)` fact: the two that write one,
  `examples/ein-bugs/wrong-answers/open-fact-head.ein` and `open-relation-parameter.ein`, are refused at load. Whether $\emptyset$'s state is root's or the fork's, and so
  which side moves, is the question; the states above root were not taken (the example states `(solve :no-search)`).
- **V-62 — a `note:` of the mechanisms precedes a flagged program's `WARNING` on stderr.** Q-M1f.564 and Q-M1f.580 keep
  the `WARNING` line stderr's first, and `ein-cli/tests/non_monotone_warning.rs` holds it on every corpus program; a
  load's `note:` from `ein_infer::mechanisms` is printed before it. Shown by
  [`as_rules_over_open_concluded.ein`](s1f.21.13_the_search_and_rules/as_rules_over_open_concluded.ein), the
  candidates concluded by `guess` beside `(solve :no-search)`: `ein solve --no-search` writes `note: …:62:8: solve: its
  :no-search states the lattice is root alone, so rule 'guess' proposes no candidate` first, then `WARNING: …`. No
  corpus program shows it today (the sweep passes).
- **V-63 — three reports do not see what the `open` writers did.** (a) `ein saturate` counts no firing of the
  saturation that resumes after the `open` pass: `examples/features/23_a_rule_reads_open.ein` prints `6 firings` and a
  per-rule list without `noted`, whose `(seen r)` is under `DERIVED`; `as_rules_over_open_concluded.ein` prints
  `0 firings` beside 17 derived facts. (b) The rung reads `stuck` where a rule's candidates pay what root owes: on
  `18_two_candidates.ein` `ein show hyps` says *H, built by the rung `stuck`: 2 candidates*, and the event `{"e":
  "rung", "mode": "stuck", "owed": 1, "branches": 1, "candidates": 0, …}`, while H is the two candidates `guess`
  concludes, each of which pays the instance, and R is the oracle's; `oblgen.rs` sets `Mode::Stuck` where the
  obligation itself offered nothing (its witness `(?R ?x)` has no guard to range over), though its comment says *a
  candidate a rule of the state already wrote is in H, and pays: no state is stuck for it (Q-M1f.570)*; the page's
  `two.ein`, with its `hrule`, reads the same. (c) A core leaves out the written `(open …)` facts its derivation read:
  `build/s13/word-in-a-world.ein` (the `open` form, `clash-in` stating `:ein inconsistent`, flagged) prints `unsat core
  (before (s a) (s b))` alone. No answer moves in any of the three.

**What the user must see**: the example is in the page's spelling, monotone and held by a suite, and not in the
language's `open` spelling, which fails the gate today (V-61) and is kept as a witness; V-61 itself, $\emptyset$ read
two ways where a program writes an `(open F)`; V-8 still holds on a program the static check calls monotone.

### Commits

- `2b63821b` One lattice twice: `examples/lattice/18_two_candidates.ein` and `18_two_candidates_as_rules.ein` over
  `open`, their corpus entries, the catalogue's sentence, the two goldens' new lines.
- `54adb21a` this record's first version, plans only.
- `54cc0002` `18_two_candidates_as_rules.ein` in the page's spelling, `(question F)`: its runs, the catalogue's
  sentence, the goldens banked again.
- `162e99e9` this record's second version, with the two witnesses beside it, plans only.
- `4b2a3026` `18_two_candidates_as_rules.ein` writes `(thing a) (thing b)` again; its shape lines banked again.
- this record, plans only.
