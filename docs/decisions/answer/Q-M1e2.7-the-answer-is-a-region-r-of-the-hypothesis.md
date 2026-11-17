# Q-M1e2.7 — The answer is a region R of the hypothesis lattice

The answer to a program is **R = { C ⊆ H : sat(root + C) is alive and discharged }**, a set of nodes of the lattice
over H, the candidate list built at root. What a run reports of it:

- **two counts**, `k_s` in states and `k_c` in commitments, which differ where saturation derives a candidate;
- **the minimal and maximal lines**, R's minimal and maximal members; restrictions only make the minimal line {∅},
  obligations only make the maximal line ⊤ on a monotone program, and neither line is enumerated where the shape
  says it is trivial;
- **∅ ∈ R exactly when root is alive and discharged**, whatever the generator proposes;
- **components**, each a set of models and never a model; a query answers with projection classes;
- **`exhausted`** means the reported region is all of R; a run a budget stopped reports a part, and says so;
- **two kinds of empty R**, every node dead or every alive node owing; the verdict words judge a stated
  expectation and are `--single`'s.

On a monotone program R has no hole and its two lines describe it. On a program the static check flags, R is the
region of the engine's one construction of each node, every command agrees on it, and nothing more is claimed.

**Why.** *Which commitments are solutions* has no single answer once obligations bound the lattice from below and
restrictions from above: the solutions are every node between the two, and their shape is the answer.

**Held by.** [Solution semantics](../../ein/reasoning/02-kb-evolution/06-solution-semantics.md) § 2; `ein oracle`, R by brute
force; `ein-infer/tests/region.rs`: `the_seven_programs_have_the_tables_regions`; `ein-infer/tests/explore.rs`:
`every_mode_is_the_oracle`.
