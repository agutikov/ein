# S1f.21.21 — the contradiction's cost: a pair matched by a probe, an index over contexts

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator on the user's word, *"plan to build everything recommended, then measure again and get final table with 3 cols: before, after, improved after"*
([Q-M1f.657](../open_questions.md#q-m1f657--the-cost-of-the-contradiction-as-rules-what-is-built)); **not
started**.
**Waits on:** [S1f.21.20](s1f.21.20_what_the_first_uses_of_open_found.md), which writes the same files of the
saturator and whose task 7 is the first of the four things recommended.
**Writes:** `ein-infer/src/` (`saturator.rs`, `contradiction.rs`, `compile.rs`, `plan.rs`, `match_.rs`),
`ein-core/src/kb.rs`, their tests; the engine's design pages that state the indexes
(`docs/ein/ein.rs/03-design/05-matcher.md`, `03-data-model.md`, `02-algorithms.md`); the goldens' lines that list
a step's probes.

## What it is for

[The cost page](costs/README.md#why-the-contradiction-as-rules-costs) finds what the contradiction as rules added
in three parts, and [its analysis of an index over contexts](costs/README.md#an-index-over-contexts-analysed)
ends on four things recommended. The user rules all four built, and the cost measured again.

| # | recommended | built by |
|---|---|---|
| 1 | the agenda's tables kept with the snapshot, not built again at every state | [S1f.21.20](s1f.21.20_what_the_first_uses_of_open_found.md), its task 7 |
| 2 | a rule of the pair's shape matched by one probe for each new fact | task 1 |
| 3 | the relation of a nested fact a key of the indexes: (context, relation) | task 2 |
| 4 | one instance of the pair's rule a relation in place of two | task 3: **not as worded**, below |

## Tasks

1. **A pair matched by a probe.** A rule of the pair's shape has two premises, a fact $F$ and a fact holding $F$
   as its one argument, and nothing else in its match; `contradiction::Pairs` tells such a plan by its shape, and
   by no name. Today each new fact is handed to the plan through the matcher, twice: after each step by
   `match_ends`, and by the pending pass.
   - **The probe.** For a new fact $f$ of relation $R$ and a pair's plan over $R$ with wrapper $W$, of $f$'s
     count of arguments: *does the state hold $(W\ f)$?* For a new fact $(W\ g)$: *does the state hold $g$?* One
     lookup by the fact's id, with no term built through the registers: the fact store's own probe of
     $(W, [f])$ with `Kb::contains`, or a set of the facts a state holds under a wrapper, as `Kb::is_negated` is
     for one of them, whichever measures better. **Keyed by the wrapper the rule's shape names, never by a name
     the kernel knows**: the renaming invariance holds it.
   - **Queued only where the probe hits**, as the match would have queued it: the same plan, the same bindings,
     the same premises in the same order, the same key, so that what was seen stays seen and the candidate takes
     the same place. The rule fires as a rule: its firing's record, its `:why`, the explanation that reads
     through it.
   - **Such a plan is seeded by neither path**: not by `match_ends`, not by the pending pass. A plan compiled at a
     state, which is matched whole once, still is.
   - **Every other rule that ends a state** keeps the path it has: `std.core`'s over `(false)`, a program's own
     that states `:ein inconsistent`, and a rule that refutes a pair with anything more in its match.
   - **A debug check** holds the probe to the matcher on the gate's build: what the probe queues is what a seeded
     match of the same fact would queue. It is bounded, a check a fact and not a walk of the state, which is
     what V-65 found costly.
2. **An index over contexts: (context, relation).** The participation index keys a name at an argument, and one
   level in, a name at an argument of the fact there. It does not key that fact's relation.
   - **The key.** `Kb`'s indexing of a fact adds, for an argument that is a fact, the relation of that fact: a
     position of `SlotKey::inner` of its own.
   - **The matcher's use.** A nested pattern whose relation is known, a constant or a bound register, offers that
     key as a probe where no argument of it is bound, so `(W (R ?a ?b))` with nothing bound walks the facts held
     under $W$ of $R$ and not every fact of $W$. A bound argument's bucket stays the first choice: it is the
     narrower. The scan the debug build compares a probe with (`candidates_scan`) is taught the same order.
   - **The saturator's use.** `pos_index` and the index of the rules that end a state are keyed by the premise's
     relation and, where the premise holds a nested pattern of a known relation, by that relation too. A new
     fact $(W\ g)$ is handed to the plans keyed by $(W, R_g)$ and to those whose premise leaves the nested
     relation open, in the plans' order, which is the order today's list gives the same plans.
   - Every source yields facts in the extent's order, so the choice of a source changes how many are offered and
     not the order of the matches.
3. **The instance of no argument (recommended as *one instance a relation in place of two*).** What the
   recommendation did not say: the second instance is no accident of the library. Every declaration stores the
   membership fact `(relation R)` beside `(relation R LABEL…)`, which the syntax reference states, and
   `clash-of-0` matches it, so a relation declared with labels has the pair's rule for its fact of no argument
   too. Taking that instance away is one of: the membership fact not stored (the language changes); `clash-of-0`
   guarded by an `absent` over the declarations of every other count (the library gains an `absent` over
   `relation`, and a program whose rule concludes a declaration is flagged); or the instance not activated, and
   `(R)` beside `(not (R))` no contradiction for a relation declared with labels (what a program means changes).
   **Default taken (Q-M1f.658): the instance stays, and is made free.** After task 1 it is matched by a compare
   of the fact's count of arguments with the plan's and no probe, and after S1f.21.20's task 7 it is one row of
   a table built once. The task: measure what it still costs, by the same programs with `clash-of-0` taken out
   of a copy of the library, and record the figure with the three ways above for the user.
4. **Measured.** Instructions executed, user space, on the five programs of the cost page, against A and D; the
   page's script on its nineteen entries with the built engine a point of its own; the gate's build of
   `equation_two_unknowns.ein` against release. Under `utils/bench_env.sh --cores PT`, with the load said.

## What must hold

- **Every corpus answer as before, and the order of every firing.** No core, no trace, no explanation, no
  digest of a derivation moves. `ein-infer/tests/agenda.rs` holds the agenda,
  `ein-infer/tests/contradiction_rules.rs` the firing of the pair's rules.
- **What a golden may show**: a step's list of probes, where a dump prints it, gains the nested relation's key.
  Nothing else. The goldens are banked only after a diff that shows those lines alone.
- **The kernel reads no theory's name** (Q-M1f.436): the probe and the index are of a wrapper a rule's shape
  names. `ein-render/tests/renaming_invariance.rs`.
- No config option, lever or flag changes $R$; a flagged program is claimed nothing of.
- The gate on the last commit, and the two slow render sweeps, since the engine moves.

## After it: the coordinator's part

**The final table**, the user's word: three columns, *before*, *after*, *improved after*.

- *before*: `edbe6c3b`, the engine before the contradiction became rules.
- *after*: `1655f19d`, the engine after the phase's builds and before any of the four.
- *improved after*: the engine with S1f.21.20 and this stage in it.

On the cost page's nineteen entries in milliseconds, each engine on the corpus of its own commit, and on its
five programs in instructions executed; on an idle machine, on AC, no agent running. It goes on
[the cost page](costs/README.md). Then the corpus's costs are taken again and the manifest's flags and recorded
costs move once.

## Record

Nothing yet.
