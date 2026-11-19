# S1f.21.12 — hypotheses as facts

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**not started**.
**Group:** [H](modifications.md#h-open-facts), items 30 to 35.
**Waits on:** [Q-M1f.549](../open_questions.md#q-m1f549--hypotheses-as-facts-held-in-which-reading-and-how-far-the-search-goes-into-rules),
which decides what the stage is. Under (a) it is not run. **It is planned here under (b), the recommendation**:
the candidates held as *asked*. Under (c) it is the design of
[`open_facts.md` § 5](open_facts.md#5-the-move-open-held-not) and [§ 6](open_facts.md#6-what-rules-can-do-with-open-facts)
instead, and is planned again. Also [S1f.21.7](s1f.21.7_the_debts_term.md), the term a candidate carries, and
[S1f.21.10](s1f.21.10_the_kernels_prefix.md), its name.
**Writes:** the grammar and the loader, `ein-infer/src/` (`hypgen.rs`, `hrule.rs`, `oblgen.rs`, `region.rs`,
`mechanisms.rs`), `ein show hyps`, the read-out, the drawings, the corpus where a spelling goes.

## What it is, under (b)

**A candidate is a fact root holds under one relation, and $H$ is that extent less what root has decided.**
Written `(open F)` below, whatever its name becomes. A fork inherits it and nothing removes it: in a state that
holds F it says *asked, and held*, beside `(not F)` *asked, and denied*, alone *still open*, and that last is a
reading the kernel makes per state and stores nowhere
([item 34 § 3](review/34-Q-M1f.549/README.md#3-what-a-rule-can-read-of-them-rows-3-and-4)). Of
[`modifications.md`](modifications.md#h-open-facts)'s items: 31 and 33 are built in this reading; 30, 32 and 34,
the part of a node, the move and the stratum, are not needed; of 35's rules, *propose* is built and *report* is
any rule, and the others are [S1f.21.13](s1f.21.13_the_search_and_rules.md)'s.

## Task 1 — the examples (stops for the user)

Questions 2 to 4 of [`open_facts.md` § 9](open_facts.md#9-what-s1f214-answers), which were S1f.21.4's, on the
corpus, with today's binary and a stand-in relation where the parse refuses `open`.

1. **The invariance over the corpus**, where item 34 ran two programs: every corpus program with an `hrule`
   rewritten by a script to a rule that concludes the stand-in and one generic hrule over it, and every program on
   the other two rungs given its `ein show hyps` list as facts of the stand-in; each answers as before, by its
   summary's `region` and its `ein show hyps`.
2. **The form**: `(open F)` or `(open F J)`, and J for each rung: the debt's term for the obligations rung, the
   rule's instance for an hrule, none for the blind rung. Default: `(open F J)` where a debt or a rule proposed it,
   one fact a candidate and reason, as the user's form has it.
3. **$H$ is root's.** A rule that concludes a candidate is a rule and may fire above root. Default: what a state
   above root asks is no candidate, since $R$ is a region of one lattice; the read-out says how many such facts a
   run met. The other reading, a lattice that grows, changes what $R$ is and is not this phase's.
4. **When the kernel's rungs write theirs.** The obligations rung reads an absence at root, and the blind rung the
   program's names. Default: one pass on root's fixpoint, as today; its facts written; root's saturation resumed,
   since a rule may read them; $H$ read then, once.
5. **`hrule`**: 143 uses in 117 files. Default: the declarator stays as a spelling of *a rule that concludes a
   candidate, activated by `(solve :hrules …)`*, so no corpus file changes; the other reading rewrites them and
   drops the word.
6. **`:hypothesis-relations`, `:no-hypothesis`, `__closed__`**: default, left as they are in this phase, read as
   filters on the extent; as rules they are a later step on the same facts, and `mechanisms.rs` stays.
7. **What a drawing and a dump show.** Root gains a fact a candidate, 125 on *zebra2* against 390. Default: a
   knowledge base's views and `ein saturate --dump` show them as any fact, since they are; the read-out's states
   list them no more than they list root's other facts.

## Task 2 — the build, on the rulings

1. The grammar admits the fact as a premise and as a rule's conclusion; the loader holds it to a fact a state
   could hold under its arrow.
2. The generator writes its list into root (task 1.4), every rung one form; `ein show hyps` prints the extent less
   the decided, and is what it printed.
3. A rule that concludes a candidate proposes it: the hrule's road and the rule's give one $H$.
4. The read-out's `universe` and `outside` rows, the events' `hyp` lines, the dumps and the drawings.

## What must hold

- **$R$ of every corpus program**, by the corpus sweep and the oracle; `ein show hyps` of every corpus program the
  same set; `ein-cli/tests/lever_invariance.rs`; `ein-render/tests/id_order_invariance.rs`.
- **No state's facts beyond root move**: the candidates are root's, so `work`'s count is as it was.
- The mechanisms' conflicts are reported as before (`ein_infer::mechanisms`).
- A `.einb` of a root holds its candidates as it holds its facts; `ein solve x.einb` is `ein solve x.ein`.
- The drawings are DOT writers': `utils/ci_container.sh`; and the nightly, a large change to the engine.

## Acceptance

Task 1's invariance holds on the built engine with no stand-in: *zebra2* written with a rule that concludes its
candidates answers byte for byte as `examples/zebra2.ein`; the gate, the nightly and the container gate.

## Record

Nothing yet.
