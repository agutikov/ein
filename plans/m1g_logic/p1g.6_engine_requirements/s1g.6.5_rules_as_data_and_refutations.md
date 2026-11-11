# S1g.6.5 — Rules as data, and a refutation a rule can read: priced first

**Phase:** [P1g.6](README.md) — candidates C-10, C-11
**Status:** planned 2026-11-11, not started. Provisional until P1g.1's stop. **A design and a price, for the
user; a build only on their word.**
**Depends on:** its witnesses: [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md) (discharge),
[S1g.5.3](../p1g.5_peano_and_induction/s1g.5.3_induction.md) (the scoped induction hypothesis),
[S1g.5.4](../p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md) (self-reference).
**Pulls in:** [F5](../../followups/f5_rules_as_data.md), *operate IR rules as data*, the part of it that
logic needs; [`08_self_describing.md`](../../../docs/kernel/ir/03-ein-lang/08_self_describing.md) § 5, *the
real frontier: rules as data*; [idea 06](../../ideas/06-inference-rules-completeness.md)'s row *hypothesis +
contradiction*; [F21](../../followups/f21_non_monotone_programs_decided.md), because the walk's writeback
and its removal are part of the history.
**For the user:** [Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact).

## Why the logic asks

Two things a proof system does that an Ein program cannot, both about **reasoning one level up**:

**C-11: conclude from a refutation, or from another state.** M1g's probe 4: assume `(Not p)`, derive a
contradiction; the engine reports `{(holds (Not p))}` in N with its core, and root holds nothing new. Natural
deduction's `Not` introduction concludes `(Not (Not p))` there; `Imp` introduction concludes `(Imp A B)` from
*B holds wherever A is assumed*; proof by cases concludes what every case derives. In Ein each is a property
of the lattice, visible in the read-out, and no rule's premise.

**C-10: a rule as a value.** An Ein rule cannot be named by, matched by, or concluded by a rule. The logic
meets that in three places: a derived rule (hypothetical syllogism *as a rule*, not as two firings); a rule
that holds only inside a sub-proof (the induction hypothesis used for rewriting, S1g.5.3's way 4); and a
statement about Ein's own derivability (S1g.5.4).

They are one stage because each is a form of the same thing, the object level reaching the meta level, and
because the cheap answer to both is the same: **stay at the object level**, where formulas, contexts,
proofs and rewrite rules are all nested facts and Ein's ordinary rules interpret them. Every theory of P1g.3
to P1g.5 is planned that way. So the stage's first duty is to say what, if anything, that leaves out.

## What staying at the object level costs

To be written from the theory stages' experience, with numbers:

- **An interpreter's overhead.** An object-level rule is data that Ein rules interpret: a rewrite fact
  matched by a generic *apply a rewrite* rule, where a native rule would be compiled to a plan. The ratio,
  measured on S1g.5.3's rewriting.
- **No help from the engine's own machinery.** An object-level assumption gets no core, no minimal dead set,
  no `refuted by`: the theory must carry its own contexts and reasons. A meta-level hypothesis gets all of
  it for free and cannot be discharged. That trade is the stage's central table: for each of reductio,
  `Imp` introduction and cases, what reading 1 gives for nothing and cannot conclude, and what reading 2
  concludes and has to build.
- **Two languages.** A user who wants to prove something *about their zebra program* cannot: the program is
  meta-level, and the proof system is about formula objects. Bridging means translating Ein rules into
  formula objects, which is rules as data from the other side: `ein render`'s formula view
  ([S1g.6.3](s1g.6.3_seeing_nested_facts.md)) emits exactly that translation as text, and whether it should
  also emit it as *facts a program can import* is the smallest form of F5 there is. **To be tried** as a
  script outside the engine first.

## The options, to be priced and not assumed built

**For C-11:**

- **(a) Nothing.** The read-out is where a refutation is read. Documented as the answer to *what is reductio
  in Ein*.
- **(b) A refutation written back, on request.** A program declares that a relation's dead singletons are
  stored as negations at root: `{(holds A)}` dead alone gives `(not (holds A))`, or a fact the program
  names. The walk did a form of this (`writeback`), and M1e2 replaced the walk with routes that describe R
  and write nothing into root ([`solution_semantics.md` § 6.3](../../../docs/kernel/inference/solution_semantics.md):
  *the search writes into root, and its universe is not H*, listed there as a way the walk differed from the
  definition). So the first task is the history: what exactly was wrong with it, whether a declared, scoped
  form avoids that, and what it does to the definition of R, since root would then depend on the lattice
  that is defined over root. Expected: sound for monotone programs (a dead singleton stays dead in every
  superset, so its negation is consistent with every alive state), and it changes H's own fixpoint, which is
  a change to the semantics page and not a feature flag. **A proposal only.**
- **(c) A premise over states.** A rule that reads *in the state committing A, B holds*. This makes the
  lattice a thing rules quantify over: modal or hypothetical reasoning in the rule language. The price is
  expected to be very high (saturation of a node would depend on other nodes), and the stage's job is to say
  so with a concrete account, not to design it.

**For C-10:** F5's own list of what is missing stands (an `:assert` that targets a rule; provenance for
rules; a termination story for rules that make rules). The stage adds the logic's three use-cases as
requirements with sizes, and for each the object-level detour M1g actually used, and recommends, expected,
that F5 stay a followup with these three as its first concrete consumers. Self-modification (F2, F5's other
half, F6) is not this milestone's.

## Tasks

1. **T1g.6.5.1 — the trade table:** reading 1 against reading 2 for each discharging rule, from S1g.3.3's
   programs, with the interpreter's overhead measured.
2. **T1g.6.5.2 — the writeback's history,** from M1e2's record and `git log`: what it did, why it went, what
   a scoped form would have to guarantee. Written down with its sources.
3. **T1g.6.5.3 — option (b), specified on paper and prototyped outside the engine:** a script that reads
   `--json-summary`'s N, appends the negations to the program, and re-runs, to a fixpoint. That shows what
   the semantics would be, on S1g.3.3's programs and on a zebra program, without touching the engine; R
   before and after is compared, and it must not move on a monotone program.
4. **T1g.6.5.4 — rules as facts, exported:** the script that writes a program's rules as formula objects, on
   the stdlib and one zebra program; one small theorem *about a program's rules* checked at the object level
   with it (that two stdlib rules compose into a third).
5. **T1g.6.5.5 — the page for the user:** the options with their prices, Q-M1g.5, and a recommendation;
   F5's file annotated with the three use-cases.
6. **T1g.6.5.6 — a build, only if ruled:** then as its own stages with their own register and sweep
   discipline, planned at that time.

## Acceptance

- The trade table exists and each cell has a program.
- The writeback's history is written with its sources, and option (b)'s prototype shows R unmoved on every
  monotone program it was run on, or the program on which it moved.
- The user has ruled on Q-M1g.5; the ruling is recorded verbatim.
- F5 lists the logic's three use-cases; nothing of F5 is claimed done that is not.

## What it may find

- **That option (b) is unsound or ill-defined** even for monotone programs once obligations are involved (a
  written-back negation could discharge or block an obligation and so change which states are in R). The
  prototype is designed to find exactly that.
- **That the exported-rules bridge is useful by itself:** a program's rules as formula objects is what M2's
  *reasoning as a program* and M10's translations to other provers both want. Then it is recommended as a
  small tool, and the user decides where it lives.
