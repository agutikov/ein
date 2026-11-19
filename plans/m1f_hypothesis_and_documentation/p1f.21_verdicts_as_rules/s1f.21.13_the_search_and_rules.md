# S1f.21.13 — the search and rules

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**not started**.
**Group:** [I](modifications.md#i-the-search-as-rules), items 36 and 37, with the three rules of item 35 that
steer a search.
**Waits on:** [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules),
and [S1f.21.12](s1f.21.12_hypotheses_as_facts.md). **It is planned here under the recommendation: the engine's
search is not changed in this phase**, and the stage keeps what costs nothing and records what is declined and
why. A ruling for either of the user's two uses plans it again.
**Writes:** one program under `examples/` with its `corpus.toml` entry and its line of the catalogue; this
folder's pages.

## What is kept

1. **The lattice written as rules, as an example**: [item 34](review/34-Q-M1f.549/README.md#4-the-lattice-written-as-rules-row-5)'s
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

Nothing yet.
