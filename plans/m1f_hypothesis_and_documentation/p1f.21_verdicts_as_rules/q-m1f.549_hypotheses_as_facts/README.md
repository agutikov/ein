# 34 · Q-M1f.549: hypotheses as facts — held, in which reading, and how far the search goes into rules

**Answered 2026-11-19, and out of the review bundle**, where it was item 34 of part 2. Your answer and its four
questions are taken up in [`open_and_owe.md`](../open_and_owe.md). The page is kept, with its programs, as the
survey the second part's stages cite. Its outputs and its tool (`runs/` and `tools/` below) are no longer beside
it: they are on this machine under `build/review-archive/p1f21-part-2/34-Q-M1f.549/`, as they were run.

**Answered in its main call, 2026-11-19.** You: *"regarding part 2: all hypotheses generation engines should emit (open H) facts and add them into KB? I think yes. Now already possible to emit new hypotheses at L1+ (dependent hypotheses) of search making search space still been lattice but not hypercube anymore. But it is non-monotone program by definition, so not to be decided in this phase."*
Read as (b) of § 5, by every rung, the fact written `(open H)` and held by the knowledge base; a candidate proposed
above root is not this phase's ([V-21](../findings.md#v-21--dependent-hypotheses-a-candidate-proposed-above-root)).
What is left of the item is the two uses of § 5, each recommended declined for now, and one thing to confirm: that
nothing is removed, so an `(open H)` stays in the states that have decided H.

[← the review bundle, part 2](../review/part-2/README.md) · [← the phase](../README.md) · [the overview](../overview.md) ·
[the ledger](../../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules)

**Raised** 2026-11-19 by the user, [the fifth instruction](../README.md#the-fifth-instruction): *"what if we
would make hypotheses just ordinary facts hidden in (open ...) context? … 1. search control from inside ein 2.
implement search with rules inside one stage, have search artefeacts seen by rules and infer facts from search. …
we can establish the foundation for this in this phase or can decline it for some pragmatic reason."*

**Not ruled.** Nothing is built: every run below is today's engine, the gate build of the main tree at `e4a5f8fb`
(`ein 0.1.0`, `protocol ein-events/1`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
all into `runs/` (both ignored, so on this machine only); a path below written as code is a file there. Three
drawings are among them, `runs/negation.kb.svg`, a negation in the knowledge base, `runs/seats-asked.kb.svg`, root
with its candidates held as facts, and `runs/two-worlds.kb.svg`.

## 1. What the runs show, in five lines

| | run | result |
|---|---|---|
| 1 | *zebra2* with its candidates concluded by an ordinary rule as held facts `(question F)`, and one generic hrule that supposes what is asked | **the read-out is the same, byte for byte, the path apart**: 56 candidates, 1 state, 261 982 commitments, 120 saturations; `--single` 7 saturations in both |
| 2 | the seats program (the obligations rung) the same way | `ein solve` and `ein oracle` the same, the path apart; `ein show hyps` the same 4 candidates, the rung's name apart |
| 3 | a rule that reads what is asked, and a held fact beside it | monotone, every licence kept, the same $R$ |
| 4 | a rule that stores *asked and still undecided* as a fact | the static check flags the program, the route is the cube, and **15 of 16 nodes hold a stale fact**; both states of $R$ hold `(still-open …)` of four seats, two of which they hold |
| 5 | a two-candidate program with its lattice written as rules, one saturation, no search | $R$ and the dead set come out as facts, the oracle's; a rule over them, *holds in every state of $R$*, **answers wrong**: the dump prints three strata and the agenda does not follow them |

So: hypotheses can be ordinary held facts today, at no change of answer, **in the reading *asked*** (1 to 3);
*still open* cannot be a held fact (4), which is [`design.md` § 4.2](../design.md#42-what-becomes-of-property-2)'s
proposition met again; and the search written as rules runs into the engine's open ground, a stratified program
(5).

## 2. The candidates as ordinary facts (rows 1 and 2)

`runs/zebra2-asked.ein` is `examples/zebra2.ein` with its `(hrule guess …)` and its `(solve :hrules (guess …))`
clause replaced by [`programs/zebra2-asked.blocks.ein`](programs/zebra2-asked.blocks.ein):

```lisp
(rule propose (?R ?T1 ?T2)
  :match  (and (is-a ?a ?T1) (is-a ?b ?T2))
  :assert (question (?R ?a ?b))
  :why    "ask: ({?R} {?a} {?b})?")
(propose color-loc  Color       House)       ; and the four other relations
(hrule ask (?R)
  :match  (question (?R ?a ?b))
  :assert (?R ?a ?b)
  :why    "suppose ({?R} {?a} {?b}): it is asked")
(solve :hrules (ask (color-loc) (nation-loc) (drink-loc) (smoke-loc) (pet-loc)))
```

`propose` is a rule like any other: it is in the agenda and its conclusions are facts root holds. `ask` is the
whole of what stays the generator's: *a candidate is a fact root holds under `question`*.

- `runs/invariance.diff.txt`: `ein show hyps`, `ein solve` and `ein solve --single` of the two programs differ in
  nothing but the path.
- `runs/zebra2-asked.saturate-dump.txt`: root holds 520 facts where *zebra2*'s holds 390; **125** of the difference
  are facts of `question`, the five activators the rest. **$H$ is 56 of the 125**: the generator drops the 69 that
  root already holds or denies. So the facts held are *what is asked*, and $H$ is *what of it root has not
  decided*, a reading and not a second set of facts.
- [`programs/seats-asked.ein`](programs/seats-asked.ein) is the seats program
  (`examples/lattice/05_owed_seats_bounded.ein`) with one rule that asks every seating and the same hrule. Its
  obligations still judge every node: `ein oracle` prints the same sixteen nodes.

## 3. What a rule can read of them (rows 3 and 4)

[`programs/seats-asked-read.ein`](programs/seats-asked-read.ein) adds a rule over what is asked and held:

```lisp
(rule answered-yes ()
  :match  (and (question (seats ?g ?c)) (seats ?g ?c))
  :assert (answered (seats ?g ?c))
  :why    "{?g} at {?c} was asked, and is held")
```

`runs/seats-asked-read.solve.txt`: `lattice monotone (the static check)`, the route and its licences those of the
program without the rule, `work 10 saturations` as there; each state answers its two seats.

[`programs/seats-asked-open.ein`](programs/seats-asked-open.ein) adds the rule that stores the reading:

```lisp
(rule still-open ()
  :match  (and (question (seats ?g ?c)) (unknown (seats ?g ?c)))
  :assert (still-open (seats ?g ?c))
  :why    "{?g} at {?c} is asked and not decided")
```

`runs/seats-asked-open.solve.txt`:

```text
  lattice           not monotone: 1 site (the static check) — no hole in this cube
    site            still-open: absent reads seats at absent depth 1, concludes a positive fact — at stake: the state
    state           stale facts — 15 of the cube's 16 nodes hold one: a fact whose absent guard the node's own fixpoint falsifies
  route             2, the cube — no licence: the static check flags the program
  work              16 saturations · 73 facts beyond root
```

and the query `(still-open ?f)` answers all four seats in both states of $R$. A fork inherits root's facts, and
root's `(still-open (seats Ann C1))` is root's fact for good. **This is why the two readings are two things**:

| | *asked* | *still open* |
|---|---|---|
| says | F is a candidate of this program | the state holds neither F nor `(not F)` |
| in a larger state | still true | false once F is decided |
| as a held fact | an ordinary one: any rule reads it | stale above the state that derived it |
| who can state it soundly | a rule, in the agenda | a pass over one quiescent state, whose result no fork inherits |

## 4. The lattice written as rules (row 5)

[`programs/two.ein`](programs/two.ein) is the object program: two candidates `(s a)` and `(s b)`, a rule that
kills a state holding both, an obligation that something be `s`. `runs/two.oracle.txt`: $R$ is `{(s a)}` and
`{(s b)}`, one minimal dead set.

[`programs/two-worlds.ein`](programs/two-worlds.ein) is the same lattice in one saturation under `(solve
:no-search)`: a world is a fact naming its commitment, `(w (s a))`; `(in W F)` says the world holds F; the
program's rule is written again under `(in ?w …)` by hand; `in-R` is a rule over the absence of `(in ?w (false))`.
`runs/two-worlds.solve.txt`:

| query | answers | |
|---|---|---|
| `(in-R ?w)` | `(w (s a))`, `(w (s b))` | the oracle's $R$ |
| `(in ?w (false))` | `(w (s a) (s b))` | the oracle's dead set |
| `(possible ?f)`, a fact inferred from the search | `(s a)`, `(s b)` | right |
| `(always ?f)`, *F holds in every state of $R$* | `(s a)`, `(s b)` | **wrong**: each is absent from a state of $R$; the answer is none |

`runs/two-worlds.saturate-dump.txt` prints `in-R` and `possible` at stratum 1, `always` at stratum 2 and
`;; Strata — stratified, 3 strata`; the boundary judges `always`'s nested absence over `in-R` before any `in-R` is
derived. It is the family of the register's `W8/fifo-gap-root`, whose owner is
[M1n](../../../m1n_non_monotone_programs/README.md)'s P1n.2 (stratum-order admission), and it is entered as
[V-8](../findings.md#v-8--the-search-written-as-rules-answers-wrong-at-its-third-stratum).

So *search artefacts seen by rules* works as far as one absence deep, and *infer facts from the search* is the
second absence, which the engine does not order today.

## 5. The options

- **(a) Declined.** $H$ stays the generator's list beside the knowledge base, as today. The phase keeps to the
  verdicts ([groups A to G](../modifications.md#the-list-at-a-glance)).
- **(b) Held, as *asked*.** Whatever rung proposes a candidate, root holds it as a fact, `(open F)` or
  `(open F J)` under whatever name [Q-M1f.399](../../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open)
  gives it; a fork inherits it; nothing is ever removed. $H$ is that extent at root less what root decides, and
  *still open* stays a reading the kernel makes per state and stores nowhere. Any rule may read what is asked. An
  hrule is then a rule that concludes it. **Recommended**, as the foundation.
- **(c) Held, as *still open*, and moved.** `(open F J)` in a part of the node no fork inherits, written and
  moved by the kernel after each fixpoint, read by rules in a stratum of their own that conclude no held fact
  ([`open_facts.md` § 5](../open_facts.md#5-the-move-open-held-not) and
  [§ 6](../open_facts.md#6-what-rules-can-do-with-open-facts); group H as
  [`modifications.md`](../modifications.md#h-open-facts) lists it).
- **(d) The search itself as rules**, every state a context, one saturation
  ([`open_facts.md` § 7](../open_facts.md#7-the-search-as-rules), (B)).

And, on either of (b) and (c), the user's two uses:

- **1. Search control from inside Ein**: rules that conclude what to split on, that a debt is forced, that a
  state is hopeless ([`open_facts.md` § 6](../open_facts.md#6-what-rules-can-do-with-open-facts), rows 2 to 4).
- **2. Search artefacts seen by rules, facts inferred from the search.**

## 6. The recommendation, and its reasons

**Establish (b) in this phase; decline (c), (d) and both uses for now, each for a reason a run or a count gives.**

- **(b) is the unification the phase is about, and it adds no mechanism.** A negation is a held fact that points
  at a fact not held; a candidate becomes the same thing under another relation. The kernel then reads one more
  context, once, at root, where today it reads a declarator, a clause of `solve`, two lists and a reserved
  relation ([the overview § 3](../overview.md#3-the-modifications-distilled)). Rows 1 and 2 are its invariance,
  taken before anything is built.
- **(c) adds what (b) shows is not needed**: a part of a node, a stratum, a class of rule that may conclude no held
  fact, and what the copy-on-write layers and the container are asked for was never read
  ([item 30](../modifications.md#h-open-facts)). Its one gain over (b) is that the fact's name is literally
  true in every state; (b) pays for that with the name (*asked*, not *open*).
- **Use 1 is not needed yet, and (b) does not close it.** The tree route already branches on the owed instance
  with the fewest alternatives left, which is *forced* and *order*, and a debt with no alternative left has no
  child, which is *hopeless* (licence L5). No corpus program is known to want another order: *zebra2* is decided in
  7 saturations. Advice that reads *still open* needs the pass of (c), so it is the same build, later, on the same
  facts.
- **Use 2 and (d) wait on M1n.** Row 5: the second absence is wrong today. Beside it, (d) is the cube, $2^{|H|}$
  worlds where the tree takes 7 saturations of *zebra2*'s $2^{56}$; every rule is written again under a context,
  and no form lifts a rule ([`open_facts.md` § 10.3](../open_facts.md#103-macros-as-the-wrapping)); and a fact
  the search writes back into a state changes that state's facts, so $R$, which is why the walk's writebacks went
  with the walk. What is kept of it costs nothing: the statement of $R$ with no search on the reasoning pages
  ([item 13](../modifications.md#b-the-pages-todays-kernel-stated-as-rules)), and `two-worlds.ein` less its
  last rule as an example, [S1f.21.13](../s1f.21.13_the_search_and_rules.md).

**What (b) leaves to say**, each a question of [S1f.21.12](../s1f.21.12_hypotheses_as_facts.md) with a default:
the name of the fact; whether it carries the debt, `(open F J)`; that a candidate concluded above root is no
candidate ($H$ is root's, or $R$ is no longer a region of one lattice); when the kernel's own rungs write theirs;
and whether `hrule` stays as a spelling of the rule.
