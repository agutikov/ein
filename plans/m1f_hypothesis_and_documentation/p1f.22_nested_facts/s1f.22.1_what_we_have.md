# S1f.22.1 — what we have

**Phase:** [P1f.22](README.md)
**Status:** next.
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

(the stage writes what it did here)
