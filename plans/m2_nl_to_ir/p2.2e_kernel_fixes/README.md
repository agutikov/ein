# P2.2e — Ein's two bugs, and a clock

**Status:** **planned 2026-11-27** on the user's word, *"Ok I agree, do it"*, to the routing of P2.2's results at
its close (the two bugs of Ein that P2.2 filed with no owner, fixed in a short phase before the benchmark is
built). **Started 2026-11-27** on the user's word, *"start p2.2e"*: its stages run one by one, each in a subagent
that commits its work, S2.2e.1 to S2.2e.4; S2.2e.4's first half to the user's ruling of
[Q-M2.24](../open_questions/kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md), the same
day: a guarantee Ein states, and no test for now. **S2.2e.6 added 2026-11-27** on the user's word, *"E-130 - yes, add
S2.2e.6"*, for the core a question that prints none takes of a dead root; it runs before S2.2e.5's recording.
**S2.2e.7 added 2026-11-27** on the user's word, *"E-131 - yes, add S2.2e.7"*, for the two other cores S2.2e.6 found
taken and printed nowhere; it runs last.
**S2.2e.6 done 2026-11-27** (`6b359cce`): a dead root's core taken by a description and `--no-search` alone, E-130
closed, E-131 filed; S2.2e.5's recording is next.
The phase stops at its end with a report for the user's review, and is not closed until the user says so.
**Depends on:** the kernel as it stands; [P2.2](../p2.2_formalizer/README.md), closed, for the probes that hold
the evidence ([`zebra_formulations/probes/`](../p2.2_formalizer/zebra_formulations/probes/README.md)) and the
model's Zebra program that shows the clock's leak.
**Blocks:** [P2.3](../p2.3_benchmark/README.md): the benchmark's attribute programs follow the formalizer's
target shape, which selects `std.slots` ([S2.3.1](../p2.3_benchmark/s2.3.1_families_and_seed_set.md)).
**Changes:** **Ein**, under Ein's rules: `ein.rs/`, `stdlib/`, `tests/`, `corpus/`, `docs/ein/`,
`docs/decisions/`. It names nothing of EinAf there.
**Ruling it rests on:** [Q-M2.16](../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md):
the kernel may gain any feature and any bug fix that leaves Ein's syntax and semantics as they are.
**The question it answers by default:** [Q-M2.42](../open_questions/kernel/Q-M2.42-are-e-104-and-e-105-fixed-before-the-benchmark-is-built.md).

## Goal

**Nothing of the benchmark is written against a rule of the library known to be wrong, and no program of a
model's is refused that Ein's own decision says loads.** Three entries of [the evaluation](../evaluation/README.md)
that the application met, each a change the ruling allows; and two small ones the same phase can write if the
user says.

| the entry | what is wrong | what it cost |
|---|---|---|
| [E-104](../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md), a bug | a relation declared a second time is refused where the program imports a module, *conflicting definitions of 'has'*, against [Q-M1f.447](../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md); with no import the same program loads | the model's `unused` in both conditions of the one shot; `examples/zebra3.2.ein` cannot declare `the` at its two counts |
| [E-105](../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md), a bug | `std.slots`' `slot-prune-fwd` and `-bwd` exempt the partner's own position, so two values stated next to each other may share a house, against the rule's page | with clue 15 dropped the Zebra puzzle over `std.slots` answers 55 states, 23 of them no reading of *next to* admits; over `std.bijection`, 32 |
| [E-126](../evaluation/bounds/E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md), a limit | a root that dies on a link several rules derive takes its time past every clock: the model's Zebra program 72 s under `-T 5` | every kernel call on such a program over a minute; `einaf from-text` on the puzzle 292 s, of which the model's three calls 42 s |

## Why before the benchmark

**The coordinator's inference, to be checked by the phase's first task and not run yet.** The benchmark's
attribute programs follow the formalizer's target shape, `zebra3.1.ein`, which selects `std.slots`. An
under-determined instance's canonical program would then state its classes over 55 states where its generator's own
count says 32 ([S2.3.2](../p2.3_benchmark/s2.3.2_generators.md): the generator's count and the kernel's check are
two routes to one answer, and a disagreement is a finding). A model's program that is right would be judged
against the same wrong count. And a fix after a split is frozen changes content its digest is taken over.

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.2e.1](s2.2e.1_declared_again_under_an_import.md) | A relation declared again, under an import | a program that imports loads its relation's second declaration as a program that does not; the imports pass's de-duplication held to Q-M1f.447; the test of the decision with an import in its programs; the diagnostics page's row saying what is still refused |
| [S2.2e.2](s2.2e.2_slots_neighbours.md) | `std.slots`: two neighbours do not share a place | the pruning rules as their page states them; held by an invariance, the under-determined Zebra puzzle answered alike over `std.slots` and over `std.bijection`; a program under `tests/stdlib/slots/` for the case; every corpus answer that moves re-stated, with why |
| [S2.2e.3](s2.2e.3_clock_and_a_dead_root.md) | The clock, and a root that dies | where root's time goes on the model's Zebra program, and the clock read there, so that `-T` returns *not decided* within its bound; the answer of a run the clock does not stop unchanged |
| [S2.2e.4](s2.2e.4_order_and_table_rows.md) | Two small ones: the order of a program's forms, and two rows of the table | on the user's ruling of Q-M2.24 (2026-11-27), the decision that an answer does not depend on the order of a program's forms, a guarantee, and its row of the determinism page, no test; the two shapes the banked table of refused shapes lacks |
| [S2.2e.5](s2.2e.5_recordings_follow_the_library.md) | EinAf's recordings follow the library: the digest out of what the model sees | added 2026-11-27 on the user's choice for L-M2.31 (*"2"*): the catalogue a prompt shows carries no digest and the record does; the two recorded runs recorded again with the model, once; the probe line judged again with no model; a replay that survives a library change leaving the catalogue's text alone. EinAf only |
| [S2.2e.6](s2.2e.6_a_core_only_where_printed.md) | A dead root's core taken only where a question prints it | added 2026-11-27 on the user's word for E-130 (*"E-130 - yes, add S2.2e.6"*): root's core taken by a description, `--no-search` and the surfaces that print a visited node's core, and by no other question; no answer and no printed byte moved; the model's Zebra program answered by `--single` in what its root costs, with no clock and under `-T 5` |
| [S2.2e.7](s2.2e.7_two_more_cores_only_where_printed.md) | Two more cores taken only where a question prints them | added 2026-11-27 on the user's word for E-131 (*"E-131 - yes, add S2.2e.7"*): the cube's minimal holes' cores taken by a description and `--count` alone, the whole node's only where `Cores::visited` asks it; no answer and no printed byte moved |

### Order

S2.2e.1 and S2.2e.2 do not wait for each other; S2.2e.2 is the one P2.3 waits for. S2.2e.3 is independent of
both. S2.2e.4's first half is the user's ruling of Q-M2.24, written down. S2.2e.6 follows S2.2e.3, whose clock it leaves as it is,
and comes before S2.2e.5's recording, which runs the bounded call on the model's programs. S2.2e.5 runs, its
recording when no other stage holds the GPU. S2.2e.7, added after the others ran, is the phase's last, and the
phase's gate runs after it.

**Each stage is a change of Ein and runs Ein's gate**: `./run_tests.sh` on the batch's last commit, the nightly
where a stage changes what the nightly alone reads, the generated files of the library and its pages re-run with
the rule that changed. A stage that changes a rule of the calculator runs the batch by hand; none here does.

## Acceptance

- `ein solve` loads a relation's second declaration with and without an import, and
  `ein-cli/tests/declared_counts.rs` holds both.
- With clue 15 dropped, the Zebra puzzle answers the same states over `std.slots` as over `std.bijection`, on
  every encoding the corpus has; the full puzzle still answers `Solution` on each.
- `ein solve --single -T 5` on the model's Zebra program returns within its bound, *not decided*; with no clock it
  answers as before. *Since S2.2e.6*: `--single` takes no core it does not print, and answers within the bound.
- Every answer of a corpus program that moved is listed in the stage's record with the reason, and no claim is
  re-stated to a wrong answer.
- Each fix has its decision under `docs/decisions/` where it decides something, and the entry of the evaluation it
  closes says which stage closed it.
- The gate is green on the phase's last commit.

## Risks

- **A library rule's fix moves answers the corpus banks.** `slot-prune` fires in every program over
  `std.slots`; the shape golden, the stdlib census, the mutation survivors and any `:expect` over an
  under-determined slots program are read again. A full puzzle's answer does not move: the probe's copy of the
  library answers `zebra.ein` and `zebra3.1.ein` `Solution`.
- **The guard may be right for a relation that is not irreflexive.** The page states
  $S(a, b) \iff S(\operatorname{slot}(a), \operatorname{slot}(b))$; for a reflexive `S` two values in one place
  satisfy it. The stage decides by the page and by `zebra2.ein`'s own comment on the same guard, and says which.
- **The de-duplication exists for a reason.** A module reached by two paths comes once; what the pass refuses of
  a rule or a macro declared twice with two bodies stays refused.
- **A clock read more often costs every run.** The measurement is on the corpus's saturations, through
  `bench_env.sh`.
- **Scope.** The evaluation holds more that the application met (a query with no row that does not say which
  premise matched nothing, no value that says whether a selecting fact was taken up). Those are features with a
  design, and wait for the stage of the loop that needs one or for the iteration of Ein.

## Defaults taken at the plan

Each is the plan's, recorded so that the user can correct it; none is a ruling.

1. **A phase of its own, before P2.3** ([Q-M2.42](../open_questions/kernel/Q-M2.42-are-e-104-and-e-105-fixed-before-the-benchmark-is-built.md)'s
   default), where the frame's default 11 would make each a task of the stage that needs it.
2. **E-126's fix is the kernel's clock**, a bound the ruling allows. Whether EinAf's call also sets the lever that
   keeps one justification a fact stays [Q-M2.34](../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md)'s,
   and what the smallest core costs stays [F22](../../followups/f22_exact_core_past_the_cap.md)'s.
3. **S2.2e.4's first half waits for the user's word** on
   [Q-M2.24](../open_questions/kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md); its
   second half, two rows of a test's table, is taken with the phase. *Resolved 2026-11-27*: the user ruled it, a
   guarantee stated with no test for now.
4. **Nothing goes upstream and no syntax changes**: a string glued to a block comment
   ([E-120](../evaluation/language/E-120-a-string-glued-to-a-block-comment-is-refused-where-a-name-glued-to-one-is-not.md))
   is the lexer's and waits for the iteration of Ein.
5. **EinAf follows in the same batch** where a change of Ein's API or of an answer reaches it
   ([L-M2.4](../findings/einaf/L-M2.4-eins-api-change-leaves-einaf-unbuilt-until-its-own-commit.md)): its
   fixtures and the page that says a model's Zebra program costs past the clock.

## Connections

- [The evaluation of Ein](../evaluation/README.md): the three entries, and what is left to the iteration.
- [`docs/ein/user/04-reference/04-stdlib/05-slots.md`](../../../docs/ein/user/04-reference/04-stdlib/05-slots.md),
  the rule's page; [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md), the refusals.
- P2.1's S2.1.4, *four findings fixed*, the precedent for a stage that fixes what a report found.

## Ledgers

[`findings/`](findings/README.md), [`open_questions/`](open_questions/README.md) and
[`evaluation/`](evaluation/README.md), one item a file under a topic, ids the milestone's next free.
