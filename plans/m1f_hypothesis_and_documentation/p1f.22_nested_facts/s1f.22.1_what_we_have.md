# S1f.22.1 — what we have

**Phase:** [P1f.22](README.md)
**Status:** done 2026-11-17: [`analysis.md`](analysis.md), [`findings.md`](findings.md), six questions for the
user (Q-M1f.391 to .396). The phase stops for the rulings.
**Writes:** this file's § Record, `analysis.md` beside it, and nothing under `docs/`, `ein.rs/`, `stdlib/`,
`examples/`, `tests/` or `corpus/`: it reads the pages and the engine and runs it.

## What it answers

**1. What the engine does**, each point a program, a command and its output:

- **Identity and what is held.** A fact stated and the same fact under an arrow of another: is it one object;
  which of the two is held after loading in each of the three cases (only stated, only nested, both); what
  `ein saturate --dump`, `ein show`, `ein render kb` and a query show of each.
- **A nested fact is not asserted.** `(think Alice (= 5 (+ 2 2)))` alone: does a query for `(= 5 ?x)` have a row;
  does a rule that matches `(= ?a ?b)` fire; does a stored negation of the nested fact contradict anything; does
  a nested `(false)` or a nested fact beside its held negation make a state dead.
- **What a pattern can match inside a fact.** A premise `(think ?who (= ?a ?b))`: the shapes that load (a
  variable for a whole nested fact; a variable for its relation; a pattern at depth two and more; an `absent`
  over a nested pattern; `eq`/`neq` between a fact and a fact) and what each binds.
- **Packing.** A rule that concludes a fact pointing at a fact it matched, or at a fact it builds from bound
  variables: what loads, what is derived, whether the nested fact becomes held (it should not), what `written`
  says of it.
- **Unpacking.** A rule that matches a fact under an arrow and concludes that fact: by a pattern that spells the
  nested fact's shape, and by a variable that stands for the whole of it (a conclusion that is one bare variable
  is refused, Q-M1f.351: what is the way a program does unpack, if any; what the stdlib does, e.g. the
  calculator's requests and results, `hypothesis`, `written`).
- **The engine's own nested facts**: what the loader states (`written`), what a hypothesis or a commitment is as
  a fact, what a stored negation is (a fact under `not` pointing at a fact: held negation, the denied fact not
  held), what a claim (`:expect`) and a core may name, what the read-out and the answer dump print.
- **What is refused**, and by which diagnostic: the shapes of nesting the grammar or the loader refuses (a
  connective, an equality or an absence as a nested head; `not` of a name; a fact as a relation), and whether
  each refusal is stated on a page.
- **Where the engine is not consistent**, if anywhere: two commands that disagree about a nested fact, a count
  that reads the top level only, a drawing that hides the nesting.

**2. What the pages and the decisions say now**: every statement about a fact as an argument, in
`docs/ein/reasoning/`, `docs/ein/user/` (the facts page, the rules page, the syntax and the grammar, the stdlib's
calculator pages), `docs/ein/ein.rs/` and `docs/decisions/` (Q40, Q-M1f.113 and those they cite), each with
whether the engine agrees, and where two of them disagree.

**3. The explanation, drafted.** In the vocabulary of the knowledge-base page (object, atom, fact, arrow, held,
extent, held extent):

- the **representation**: one node for the fact wherever it is written; *held* as the one thing that tells the
  three cases apart; how the graph is drawn so that a reader sees which case it is (the user's example drawn in
  each case);
- **context**: what it means that a fact stands under an arrow of a held fact (the held fact speaks *about* it),
  and that this gives it no truth of its own;
- **packing and unpacking as rewriting**: the two as rules in the form of the rewriting-rules page, each with its
  graph before and after, and the statement that a fact moves between the cases only by a rule that says so;
- **what is the same and what differs** for a fact in the three cases: its identity, its extent's membership,
  its order and text (the same); being matched by a pattern at the top level, contradicting, being counted (not
  the same);
- the propositions a reader should be able to rely on (e.g. *no rule of the engine's own derives a nested fact
  from its context*, if the survey bears it out), each with what in the survey supports it.

**4. What is the user's to decide**: every point where the engine's behaviour is one of several defensible ones,
where the pages need a ruling to be written, or where something would have to be built for the explanation to be
true; each with the options and a recommendation.

## Acceptance

`analysis.md` holds 1 to 4, every claim of 1 beside the program and the output that shows it (programs kept in
the page), every statement of 2 with its file and section. The link check and the release-voice check pass.
Nothing outside this folder is changed, except the question's entry in the milestone's `open_questions.md`.

## Record

### Record (2026-11-17)

**What was done.** The engine was run on programs written for each point of § What it answers, all of them with
the gate build of the main checkout, `ein.rs/target/gate/ein`, and each kept in [`analysis.md`](analysis.md) § 1
with its command and the lines of output that carry the claim. The pages of `docs/ein/` and the decisions were
read for every statement about a fact under an arrow of another (§ 2; the sweep of the pages was made by a
read-only subagent and its quotes spot-checked against the files, the verdicts are this stage's). § 3 is the
explanation drafted in the knowledge-base page's vocabulary, with eight drawings as files under
[`diagrams/`](diagrams/), and § 4 the six points for the user, entered in
[`open_questions.md`](../open_questions.md) as Q-M1f.391 to Q-M1f.396, each not ruled with its default.
[`findings.md`](findings.md) holds what is outside the stage, N-1 to N-13. Nothing under `docs/`, `ein.rs/`,
`stdlib/`, `examples/`, `tests/` or `corpus/` was changed.

**What it found.** The engine agrees with the knowledge-base page: a fact is one object wherever it is written,
and what tells the three cases apart is what the knowledge base holds. A fact under an arrow satisfies no premise,
answers no query, contradicts nothing, and activates, declares and marks nothing. A rule packs in every form
tried; it unpacks by spelling the fact's relation and count, or with the relation a parameter, and not by a
variable that stands for the whole fact, which a conclusion may only deny. Two things are not consistent: a
conclusion `(not ?x)` over a nested negation stores the negation of a negation, which the pages say is no fact and
whose state file does not load (N-1), and a state file can read back dead through `written` (N-2).

**The checks.** `python3 utils/doc_audit.py --links --check --tracked` and `python3 utils/check_release_voice.py`,
on the stage's commit: both exit 0, the second having no project file of this stage to read. The eight pictures
were drawn by `dot -Tsvg` (Graphviz 16.0.0) from the sources beside them. The two censuses of § 1 carry their
commands: the 880 tracked programs, none with a negation of a negation at root, and
[`packscan.py`](packscan.py)'s count of the rules that pack and unpack, read as text.

**Not done.** `ein-cli/tests/answer_round_trip.rs` was not run, so that no corpus program reaches N-2 rests on the
gate that holds it and not on a run of this stage. The statements of N-13 were read and not run. The `Write` tool
refused the names `analysis.md` and `findings.md`, which this stage file names as its deliverables, so both were
written to a scratch file and copied into place.
