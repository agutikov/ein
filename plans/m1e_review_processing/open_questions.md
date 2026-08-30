# Open Questions — M1e (review processing)

Questions **this milestone raises**, with sticky `Q-M1e.<n>` ids. Do not
reuse a closed id.

The review's own ten questions are **not** here: they are the subject of
[P1e.1](p1e.1_open_questions/README.md), they keep the review's `Q1`–`Q10`
numbering, and they live in
[`review/open-questions.md`](review/open-questions.md) with their answers
recorded in the stage that answers them. A review question that turns out
*not* to be answerable within this milestone is re-filed here with a fresh
`Q-M1e.<n>` and a named owner — that re-filing is a result, and the stage
records which question became which id.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1e.1](#q-m1e1--what-is-the-standard-of-proof-for-refuted) | What is the standard of proof for **refuted**? | **decided 2026-08-28**, written into [`docs/kernel/standard_of_proof.md`](../../docs/kernel/standard_of_proof.md); ratified in [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md#task-t1e111--ratify-the-standard-of-proof--both-rules) T1 **together with Q-M1e.2** ([D5](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d5_does_t1_ratify_q_m1e2.md), option A, 2026-08-28), applied everywhere after |
| [Q-M1e.2](#q-m1e2--may-a-review-finding-be-closed-by-a-comment) | May a finding be closed by a comment rather than a check? | **decided 2026-08-28** — *an argument suffices when its premise is itself enforced*, written into [`docs/kernel/standard_of_proof.md`](../../docs/kernel/standard_of_proof.md). The `accepted` disposition's rule. **Owned since 2026-08-28** by [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md#task-t1e111--ratify-the-standard-of-proof--both-rules) T1, which ratifies it beside Q-M1e.1 rather than leaving the first `accepted` to decide it implicitly |
| [Q-M1e.3](#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted) | Who owns a `docs/kernel` page that should be neither fixed nor deleted? | **answered 2026-08-30 — (c)**, by [S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) T1. The destination was already written into [`docs/history/README.md`](../../docs/history/README.md): M1's survivors went to `docs/kernel/inference/` at P1.22, so `m1_core/` is a directory the tree **declined**, and what a superseded page needed was the banner. The general rule — *a page is moved into an existing milestone record, never made into one* — plus the triage of all 40 pages |
| [Q-M1e.4](#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all) | Does the repo want an exact count in prose at all? | open — [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md) |
| [Q-M1e.5](#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) | Is *experimental* a licence to ship a surface whose read-out is false? | open — **but its instance is gone**: [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) fixed all three of `CO-H3` on 2026-08-29 and the tree's read-out is now true rather than merely refused. The *rule* is still unwritten, and M1d's `T1d.10.6.4` is the co-owner. [S1f.10.7](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.7_tree_calibration_and_flag.md)'s `--traversal` flag waits on it |
| [Q-M1e.6](#q-m1e6--what-is-a-solution-and-what-is-a-model) | What is a **solution**, and what is a **model**? | **decided 2026-08-28** by the user; binding on [Q5](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) and on [P1f.10](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/README.md) |
| [Q-M1e.7](#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model) | The read-out prints the solution **KB** and calls it a model | **decided 2026-08-28 — A**: the recorded object is the *state*, `model` is a projection of it, and § 2 is evaluated on the state. Unblocks Q-M1e.8. S1e.3.2 applies it to the vocabulary, P1e.5's S1e.5.3 to the read-out |
| [Q-M1e.8](#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set) | `exhausted` certifies the **lattice**, not the model set | open — raised by Q-M1e.6; `lattice/02 -e -L` is the witness. The **record-site conformance check** is [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md#task-t1e114--the-record-site-conformance-check) T4 ([D3](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d3_q_m1e8_file_or_take.md), option B); the fix files to P1e.2 and, since [Q-M1e.7](#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model) was ruled on 2026-08-28, **is chosen**: re-saturate and re-check before recording |
| [Q-M1e.9](#q-m1e9--is-dead-really-upward-closed-under-absent) | Is `dead` really upward-closed under `absent`? | **answered 2026-08-28 — no.** Reproduced; three shipped mechanisms read the premise. **Ruled 2026-08-28: B now, C filed.** The containment is [S1e.2.3](p1e.2_high/s1e.2.3_naf_refutation_diagnostic.md) (a diagnostic, 1 d, P1e.2); the real fix is [F18](../followups/f18_world_aware_negatives.md); and the **language** half — *may a refutation rest on an `absent` at all?* — is [S1f.10.8](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md)'s |
| [Q-M1e.10](#q-m1e10--two-config--flags-are-inert) | Two `(config …)` flags are **inert** — `print-alive`, `candidate-order-seed` | open — raised by [S1e.5.1](p1e.5_documentation_and_other/s1e.5.1_config_reference.md); owner unassigned |
| [Q-M1e.13](#q-m1e13--expect-cannot-state-an-answer-with-an-empty-goal-extent) | `:expect` cannot state an answer with an **empty goal extent** — nor detect a shrinking model set | open — found 2026-08-28 while banking [D1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d1_q4_which_route_reaches_the_site.md)'s fixtures; **owner unassigned** |
| [Q-M1e.12](#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object) | The blind rung is **untyped**, and a model binds a type as an object | open — raised by [D8](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d8_branching06_untyped_models.md) 2026-08-28; **owner unassigned**, three readings recorded |
| [Q-M1e.11](#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis) | What happens to an obligation **derived under a hypothesis**? | open — **handed to [S1f.10.6](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md)** 2026-08-28 by the user. The guard **shipped** 2026-08-29 in [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) T3 — the mode is re-read at every node the tree expands — and its regression test is owed to that stage, because no corpus program can flip the mode |
| [Q-M1e.14](#q-m1e14--the-corpus---jobs-sweeps-per-layer-census-coverage-is-vacuous) | The corpus `--jobs` sweep's per-layer census coverage is **vacuous** | open — raised 2026-08-29 by [S1e.1.2](p1e.1_open_questions/s1e.1.2_determinism_under_jobs.md) T3, which closed the unit half; **owner unassigned**, and the corpus half costs a golden |
| [Q-M1e.15](#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported) | The **alternatives cap** decides which unsat core is reported | open — raised 2026-08-29 by [S1e.1.3](p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md), which is the review's `Q2` answered **yes**; witnessed by a fixture pair, **owner unassigned**, and no shipped puzzle is changed by it |
| [Q-M1e.16](#q-m1e16--the-binding-key-compares-two-register-layouts-as-one) | The **binding key** compares two register layouts as one | open — raised 2026-08-29 by [S1e.1.4](p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md), which is the review's `Q3` answered and **`Q-M1a.8` closed as stated**. A well-formed program loses a derivation in a release build and trips a `debug_assert` in a test one; **owner unassigned**, and no corpus program can reach it |
| [Q-M1e.17](#q-m1e17--three-py_int-options-silently-reinterpret-a-negative) | Three `py_int` options **silently reinterpret a negative** | open — raised 2026-08-29 by [S1e.1.5](p1e.1_open_questions/s1e.1.5_cli_semantics.md), which closed the third of them (`-n`) by refusing it. `-m` and `-E` still clamp, and `-E`'s abort line **prints the clamped number**; **owner unassigned** |
| [Q-M1e.18](#q-m1e18--three-kernel-primitives-are-not-shape-pinned-and-drop-their-extra-arguments) | Three kernel primitives are **not shape-pinned**, and drop their extra arguments | **answered 2026-08-29 — candidate (2)**, by [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) T1, which took the whole class rather than `CO-H1`'s one cell: `eq` and `absent` are checked at compile time and refused with a positioned `CompileError`. The grammar and the lexer's `SYMBOL` set are untouched, so no program that worked still works differently. `false` needed nothing |
| [Q-M1e.19](#q-m1e19--algorithmic-pathology-has-no-owner) | **Algorithmic pathology** has no owner | open — the one of `Q9`'s four unswept surfaces with no home. The other three have one: this stage swept the parser/CLI edges, `cast.rs` goes with `ein-einb`'s next change, and micro-CSP ground truth is [M10](../m10_external_benchmarks/README.md)'s thesis |
| [Q-M1e.20](#q-m1e20--two-renderers-are-produced-tested-and-unreachable) | **Two renderers are produced, tested and unreachable** — the per-hypothesis lattice dump and the unified KB DOT view | open — raised 2026-08-30 by [S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) T3, which was told to route the decision rather than take it in a doc pass; **owner unassigned**, and both are `pub` and golden-pinned today |

---

## Q-M1e.1 — What is the standard of proof for **refuted**?

> **Decided 2026-08-28 and written down.** Both this and
> [Q-M1e.2](#q-m1e2--may-a-review-finding-be-closed-by-a-comment) are now
> [`docs/kernel/standard_of_proof.md`](../../docs/kernel/standard_of_proof.md),
> which carries the two rules, the four-row calibration table and the worked
> example. This entry stays as the argument that produced them. Later stages
> **cite the page**, not this entry.

Sixty of the sixty-three findings are one reader's reading; the review's
verification stage never ran ([`review/summary.md`](review/summary.md)
§ Method). So the milestone will refute some of them, and *refuted* needs a
bar, because the cheap version — "I read the code and disagree" — is the same
epistemic move that produced the finding.

The proposed bar, to be ratified in
[S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md) T1 and then
binding on every stage:

- A finding claiming a **behaviour** is refuted only by an executed probe —
  a program, a command, an output — banked as a test that would fail if the
  behaviour ever appeared. [CD-H3](README.md#the-findings) is the model: the
  review refuted a documented bug with one probe, and the review's own
  recommendation is to *bank the probe both ways*.
- A finding claiming an **absence** (no test holds X, no page states Y) is
  refuted by naming the test or the page. That is cheap and it is enough.
- A finding claiming a **risk** (this is unenforced, this could drift) cannot
  be refuted by argument at all — only `fixed`, `accepted` with the argument
  written at the site, or `deferred`. Saying "it cannot happen" *is* the
  written argument, and it goes beside the code.

## Q-M1e.2 — May a review finding be closed by a comment?

> **Decided 2026-08-28, beside Q-M1e.1** — *an argument suffices when its
> premise is itself enforced* — and written into
> [`docs/kernel/standard_of_proof.md`](../../docs/kernel/standard_of_proof.md)
> as Rule 2.

Several findings are of the form *this is stated but not enforced*
([ST-M1](README.md#the-findings), [CO-M1](README.md#the-findings),
[CO-H3](README.md#the-findings)(c)). For each, the honest options are a
check, or a written argument — and the repo's method has used both:
`design/02` is an argument, `check_hashmap_iteration.py` is a check.

The question is when an argument is sufficient. The proposed rule: an
argument suffices when its **premise is itself enforced**. The alive-set
invariant's argument rests on *rules assert no new objects or relations* —
which nothing checks, so the argument is not sufficient and
[ST-M1](README.md#the-findings) needs the cheap post-fixpoint check.
Contrast [ST-L1](README.md#the-findings) (`EqClasses` auto-vivification),
whose premise is *nothing fires equality propagation* — enforced by
`naf_semantics::matching_does_not_resolve_equality_classes`, an existing
named test — so a comment at the future wiring point is enough.

If that rule holds, it decides most of the `accepted` dispositions
mechanically, and it should be written into
[`docs/kernel/defined_behaviour.md`](../../docs/kernel/defined_behaviour.md)
or `design/`-style prose rather than living only here.

### Owner, assigned 2026-08-28 — ratified beside Q-M1e.1

[D5](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d5_does_t1_ratify_q_m1e2.md),
option **A**: this question had **no owning stage**, which meant *"whoever
reaches it"* was nobody and the first `accepted` disposition would have decided
it implicitly. It is now
[S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md#task-t1e111--ratify-the-standard-of-proof--both-rules)
T1's, ratified in the same half-day as Q-M1e.1 because the two are one rule
read from two ends — Q-M1e.1 says a **risk** cannot be argued away; this one
says what an acceptable argument looks like when a finding is `accepted`
anyway.

**What decided it was a fourth calibration row, supplied while the question was
still open.** design/08's *`dead` is monotone* is a premise that was written
down and never enforced, and
[Q-M1e.9](#q-m1e9--is-dead-really-upward-closed-under-absent) broke it with a
twenty-line program. Three of the four rows are the repo's own precedents; the
fourth is the failure the rule exists to prevent, and it arrived four days
before the rule would have been needed.

## Q-M1e.3 — Who owns a page that should be neither fixed nor deleted?

> **Answered 2026-08-30 — (c), and the answer was already written down.**
> [`docs/history/README.md`](../../docs/history/README.md) says where M1's
> surviving documents live: *"**M1** (core graph reasoning, shipped
> 2026-06-17) predates this directory: what survived its plan tree went to
> `docs/kernel/inference/` and `plans/followups/` at P1.22, and the rest is in
> git history."* So candidate (a) is not a directory that has not been created
> — it is a directory the tree has already **declined** to create, and the page
> is where P1.22 put it. What was missing was not the destination but the
> **banner**.
>
> The rule, generalised from that and applied to all 40 pages in
> [S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) T1:
>
> 1. Does every claim on the page hold of the engine that ships? → **current**.
> 2. Otherwise, is the page still *read* — cited by another page, or the record
>    of a measurement nothing can re-take? If **no**, and nothing links to it →
>    **delete**; git holds it.
> 3. If **yes** it is superseded, and *where* it goes is decided by one
>    question: **does a `docs/history/` entry for its milestone already
>    exist?** If it does → **move** into that record. If it does not →
>    **banner in place**. A page is moved *into* a milestone record; it is
>    never made *into* one.
>
> Rule 3 is what makes the answer mechanical rather than a judgment call, and
> it is the shape `docs/history/README.md` already has: the directory is
> indexed **by milestone**, its three entries are milestone records, and
> creating `m1_core/` to hold one page would assert a record nobody wrote.
>
> On `algorithm_layer_n.md` specifically, step 2 also answers "half a reason"
> the other way: the page has **five** referrers, and one of them is
> [`docs/history/m1a_rust/design/07_search_layer.md`](../../docs/history/m1a_rust/design/07_search_layer.md),
> which cites it as *"the per-step contract"* — a shipped milestone record
> naming it as the specification the port had to reproduce. Deleting a page a
> history record cites as its specification falsifies the record. So deletion
> was never available, and the count settles it without weighing halves.

[CD-H1](README.md#the-findings) covers pages in three states, and the review
names the triage: *current* (fix), *superseded with a banner*, *moved to
`docs/history/`*. The rule that put a document into `docs/history/` is written
down — *it is still read, as a specification, as evidence, or as the reason
something is the way it is* — but `algorithm_layer_n.md` fails it in an
awkward way: nothing reads it as a specification (its three solve entries do
not exist), and it is not evidence, but it **is** the reason
`architecture_and_algorithms.md` §41-48 records a removed soundness bug. Half
a reason.

Three candidate answers, none obviously right: (a) `docs/history/m1_core/` —
a directory that does not exist, for the milestone whose plans were deleted
at P1.22; (b) delete, since git history holds it and the surviving reason is
already stated where the refutation is; (c) keep in place with a banner as
strong as `parity_baselines.md`'s. This question is the *rule*;
[S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) T1 applies it per page
and records which pages the rule sent where.

## Q-M1e.4 — Does the repo want an exact count in prose at all?

[DO-M1](README.md#the-findings) is not a list of typos: it is one mechanism,
observed eight times, and the repo already knows the mechanism — *a page
nothing runs goes stale*. Every count a test pins is exactly right; every
count only prose states has drifted.

So the question is not *fix the numbers* (a one-day pass that rots again by
M2) but whether a count belongs in prose. Three shapes are available and the
repo uses all three somewhere: the **generated** count (the embedding page's
marked region, diffed by a test), the **census-owned** count (say *the census
prints it* and link, as `corpus_cost.md` does), and the **dated** count (*as
of the M1a close, 616*). A fourth — a markdown-level check that a stated
number matches a script's output — does not exist and would be new machinery.

The answer decides whether [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md)
is a counting pass or a de-counting pass, and it is worth taking before the
pass rather than during it.

## Q-M1e.5 — Is *experimental* a licence to ship a lying surface?

`EIN_TRAVERSAL=tree` is opt-in, undocumented as stable, and honestly recorded
as open (`T1d.10.6.4`). It also reports `Contradiction` with an empty unsat
core, ignores `-n` and `-m`, and reads `refuted so far (0 facts)` — which is
not an incomplete read-out but a false one
([CO-H3](README.md#the-findings)(b)).

The two positions are both defensible in this repo's terms. *Experimental
means the surface may be absent or may change* — but the project's own
discipline is that a **verdict** is never qualified by how the search got
there, which is exactly why `Ambiguity` learned to say *(a lower bound)* at
S1d.3.3 rather than keep printing a bare `k`. Under that reading, an empty
core printed as evidence is the same defect S1d.3.3 fixed, and the experiment
flag does not license it.

The narrow fix — make the arm refuse to print evidence it does not have — is
available without answering the design question, and
[S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) T3 was to take it. The general
rule is this question, and it belongs with `T1d.10.6.4` when M1d's traversal
work resumes.

### What S1e.2.1 did instead, 2026-08-29 — and what is still open

**It did not refuse; it recorded.** The stage's own table offered two fixes for
the empty core — *learn* (emit the clause and the writeback on a tree death, as
the lattice does) and *refuse* (decline to print a core under tree mode) — and
priced the first as a search change that would move the published 86 enterings.
There is a third the table did not have: the lattice's `handle_dead` does
**three** things and only two of them touch the search. Pushing the refuted
commitment onto `lstate.dead` without emitting the clause or the writeback
costs nothing the search can see — re-measured, 86 enterings and the same 32
models, fact for fact — and it makes the core *true* rather than absent. On the
smallest program that reaches the arm the tree now prints the same two-fact
core the lattice does.

So the surface is no longer lying, and the counters that read `emitted=0` are
honest because nothing was learned. The three shipped defects are closed:
`-n` honoured, `-m` refused with a reason at exit 2, the core real, and the
rung re-read per node.

**What stays open is the rule, and it was never the instance.** Two things this
question is still the right home for:

1. *Learning on tree deaths remains available and unpriced.* It is the option
   that would make the tree prune, and it is a search change: it belongs with
   `T1d.10.6.4` and with
   [S1f.10.7](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.7_tree_calibration_and_flag.md),
   not with a review-processing stage.
2. *The general rule.* `EIN_TRAVERSAL=tree` is still the only surface in the
   system the word **experimental** applies to
   ([`configuration.md` § 2.3](../../docs/kernel/configuration.md)), and what
   that word licenses is undecided. What S1e.2.1 establishes is only a lower
   bound on the answer: on this occasion the true read-out was **cheaper** than
   the refusal, so the question was not tested.

## Q-M1e.6 — What is a **solution**, and what is a **model**?

**Decided 2026-08-28**, by the user, in answer to
[S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md)'s Q5 —
which had asked which of two engine configurations is right and was told,
correctly, that the question is prior to both. Recorded verbatim:

> Solution is a KB state after saturation (saturated KB state) without open
> obligations or if other hypgen used — with some integrated subset of
> hypothesis and not consistent with any other hypothesis (e.g. set of 3 hyp
> facts integrated, then saturation derives 10 other facts with other 5 from
> hypothesis list, then this state has 8 hypothesis integrated, and it would be
> a solution iff all other hypothesis are inconsistent with this KB). Model is
> a positive part of solution KB minus positive initial KB. Initial KB is
> before first saturation, no derived facts only problem statements.

### The definition, restated

**Solution.** A KB state `S` is a solution iff

1. `S` is **saturated** — quiescent under the rule set; and
2. `S` is **consistent** — no `(false)`, no same-layer `X ∧ ¬X`; and
3. **either** the obligations rung is in play and `S` **owes nothing**,
   **or** — under any other generator — for every hypothesis `h` of that
   generator's list that is not already in `S`, `S ∪ {h}` is **inconsistent**.

Clause 3's second arm is a **maximality** criterion, and the user's worked
example is the part an implementation gets wrong: *integrated* counts the
hypotheses saturation **derived**, not only the ones committed. Commit three,
saturate, and if five more members of the hypothesis list appear among the ten
derived facts, the state has integrated **eight**; the test is over the
remaining ones.

**Model.** `positives(S) \ positives(initial KB)`, where the initial KB is the
loaded program **before the first saturation** — problem statements only, no
derived facts. So a model carries no `(not …)`, no `(is-a …)` the file
declared, no `(relation …)`, and no rule-application marker that was written
down: it is *what the puzzle did not say and the solve established*.

### What it settles, immediately

- **Q5's OFF side is wrong.** With `-L`,
  `examples/lattice/02_genuine_3set_death.ein` **exhausts** and prints *"No
  solution — the constraints are contradictory"* with a three-fact core, on a
  program whose solutions are `{h₁,h₂}`, `{h₁,h₃}`, `{h₂,h₃}` by inspection.
  Under clause 3 each of those is a solution: the third candidate is
  inconsistent with the state. `-L` makes `complete` under-report, and the
  default's `k=3` is right.
- **`complete` is an approximation, and the definition says in which
  direction.** `complete(S) ≡ the generator proposes nothing`
  ([`hypgen.rs:902`](../../ein.rs/crates/ein-infer/src/hypgen.rs)) is
  generator-relative; clause 3 is not. The one-step lookahead only drops a
  candidate it can **prove** dies, so using it inside `complete` is *sound* —
  every state it calls complete is a solution — and *incomplete*: a candidate
  that dies in two firings is missed and a real solution goes unrecorded.
  Turning the lookahead off does not make the test honest, it makes it
  strictly weaker. **Both configurations under-report; `-L` under-reports
  more.**
- **The `-K` fact-set difference is not a defect.** With the kill cache off,
  `lattice/02`'s recorded states lose their `(not (c-prop X))` facts. Those
  are negatives, so they were never part of the **model**; what changed is the
  solution **KB**. See [Q-M1e.7](#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model).

### What *all other hypotheses* quantifies over — settled 2026-08-28

Not the generator's per-node list. The user's second clarification, verbatim:

> "All other hypothesis" mean exactly ALL other than integrated n L1
> hypothesis (which is one L{n} hypothesis) — so no other L{n+1} would be
> consistent — so for this L{n} hypothesis exhausive search ends.

So the reference set is the **layer-1 hypothesis set** — `alive₀`, the set the
lattice enumerates subsets of — and a state reached by an L{n} commitment `C`
(which *is* n L1 hypotheses) is a solution exactly when **no L{n+1} extension
of it is consistent**.

### The operational form, which is the useful one

```
solution(C)  ≡  C is alive  ∧  ∀ h ∈ alive₀ \ integrated(C):  C ∪ {h} is dead
```

*"For this L{n} hypothesis exhaustive search ends"* is the same sentence read
as a stopping rule: **a solution node is a maximal alive commitment — one with
no live child.** Three things follow immediately, and they are why this
clarification is worth more than the declarative form.

**1. The lattice already computes it, one layer later.** Layer `n+1` enters
the supersets of every surviving `C`, and the ones apriori declines to
generate are the ones a subset already proved dead. So *"did any superset of
`C` survive?"* is answered by layer `n+1`'s own results, with **no extra
fork**. The engine computes it and throws it away: `a_layer` becomes `a_prev`
and is never asked which of its members had a live child.

That step **inherits a premise rather than adding one**: it needs `dead` to be
upward-closed, which is exactly what apriori's pruning and the no-good store
already need. The repo asserts it —
[design/08](../../docs/history/m1a_rust/design/08_parallelism.md) § The
objects: *"`dead(X)` … **Monotone**: `X ⊆ Y ∧ dead(X) ⇒ dead(Y)`, because the
KB is append-only and nothing retracts"* — and
[Q-M1e.9](#q-m1e9--is-dead-really-upward-closed-under-absent) is why that
sentence is worth re-reading before anything new leans on it.

**2. `complete()` is an approximation of that lattice property, and it is
sound in one direction only.** `hypgen::complete` asks the generator *at the
node, now*, and its answer is filtered by the pipeline
([`hypgen.rs:422`](../../ein.rs/crates/ein-infer/src/hypgen.rs)). The
lookahead only drops a candidate it can **prove** dies in one firing, so:

| | holds? | consequence |
|---|---|---|
| `complete(S)` ⟹ `S is maximal` | **yes**, with the lookahead on or off | the lookahead only drops a candidate it can **prove** dies in one firing |
| `solution(S)` ⟹ `complete(S)` | **no** | a remaining candidate that needs two firings to die is still proposed, so a real solution goes unrecorded |
| `complete(S)` ⟹ `solution(S)` | **no** | ⚠ **this row said *yes* — *"the engine never records a false model"* — until 2026-08-28.** Maximality is one conjunct of three, and what is recorded is not the state `complete` was asked about: the generator's own kill cache writes into it. Three witnesses at three record sites; the row was rewritten on the kernel page the same day ([D9](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d9_kernel_page_overclaims.md), [D3](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d3_q_m1e8_file_or_take.md)'s check) |

So the engine **under-reports**, always — and, separately, can record a state
its own rules refute. `-L` under-reports far more —
with the lookahead off, `complete` is true only when every remaining candidate
is already asserted or already negated in the KB.

**3. The filters are not part of the definition, and at root they are
harmless.** A candidate the lookahead kills at root is one `root ∪ {h}`
refutes; by monotonicity `S ∪ {h}` refutes it at every descendant too. So
excluding it from `alive₀` loses no solution — F3 at root is
definition-preserving. The damage is at the deeper nodes, where the
*approximation*, not the filtering, is what misses a maximal state.

### The premise this inherits

Quantifying over `alive₀` presupposes that `alive₀` is the whole hypothesis
space — that no fork derives an object or relation that would have made a new
hypothesis possible. That is precisely the **alive-set invariant** of
[ST-M1](README.md#the-findings), which the review found *"is enforced
nowhere"*. The definition and the dedup warrant now rest on the same
unchecked premise, which raises ST-M1 from a Medium tidy-up to the thing the
semantics stands on.

### Where it goes

Into `docs/kernel/` as a normative page —
[P1e.5](p1e.5_documentation_and_other/README.md)'s proposed S1e.5.2 — not into this
file. A ruling that lives only in a plan is the shape
[Q-M1e.1](#q-m1e1--what-is-the-standard-of-proof-for-refuted) forbids.

## Q-M1e.7 — The read-out prints the solution **KB** and calls it a model

Raised by [Q-M1e.6](#q-m1e6--what-is-a-solution-and-what-is-a-model), which
defines a model as the positive part of the solution KB minus the positive
initial KB. Nothing in the engine computes that object.

What is printed and stored instead is the whole fact list of the solution
state: `verdict.solutions[i].facts` in `--json-summary`, the `model n/k`
blocks in `ein solve`, the `BTreeSet<Vec<String>>` that
[`tree_traversal.rs`](../../ein.rs/crates/ein-infer/tests/tree_traversal.rs)
compares fact for fact, and the variables
[`model_set_census.py`](../../utils/model_set_census.py) derives. All four
include the negatives, the ontology and the rule markers —
`lattice/02`'s "model" is nine facts of which **six** are `(relation …)`.

Three consequences, and the second is the one that bites:

- The **name is wrong** in four surfaces, which is
  [SE-M1](README.md#the-findings)'s vocabulary defect in a second place.
- Two runs that agree on every model can **disagree on the recorded fact
  sets** — `-K` does exactly this. Any test or census that compares fact sets
  is comparing solution KBs, so it is sensitive to levers that provably do not
  change the answer. `tree_traversal.rs`'s comparison is the load-bearing one.
- `model_set_census.py`'s *varying slots* and determining keys are computed
  over an object that includes facts the program itself supplied, which can
  only inflate them.

Not obviously a defect to *fix* — printing the solution KB is defensible and
`--models key` already projects it. But the two objects need two names, and
the fact-set comparisons need to say which one they mean.

### Owner, assigned 2026-08-28 — and why it is not the two candidates first named

This entry used to say *owner unassigned; candidates are
[S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md) (the vocabulary split) and P1e.5's
proposed S1e.5.3 (the read-out reference)*. Both still want the **application**
— the names, the read-out, the reference page — but neither can be the owner of
the **ruling** any more, because both run after the thing that now waits on it:
[D3](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d3_q_m1e8_file_or_take.md)'s
record-site check found that two candidate fixes for
[Q-M1e.8](#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set) report
**different `k` on the same program**, and which is right is this question. The
fix's home is P1e.2; S1e.3.2 is P1e.3 and S1e.5.3 is P1e.5.

| | what | who |
|---|---|---|
| **the ruling** — which object the criteria are evaluated on | [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md#task-t1e114--the-record-site-conformance-check) **T1e.1.1.4**, the task that found the dependency, in the phase whose job is to rule | **owner** |
| the vocabulary — two objects, two names, in four surfaces | [S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md), with `SE-M1` | applies it |
| the read-out — what `ein solve` and `--json-summary` print | P1e.5's S1e.5.3 | applies it |
| which object is recorded and compared | the Q-M1e.8 fix stage, in P1e.2 | consumes it |

The precedent is
[Q-M1e.1](#q-m1e1--what-is-the-standard-of-proof-for-refuted): ruled once in
S1e.1.1 T1, cited rather than re-argued by every stage after.

### The three readings, and what each does to `k`

The probes in
[`s1e.1.1…/probes/`](p1e.1_open_questions/s1e.1.1_search_soundness_probes/probes/)
make this concrete rather than terminological. Write `S` for the recorded KB
and `K` for what the search wrote into it — the lookahead kill cache's `(not
h)` and the singleton writeback's.

| | ruling | the Q-M1e.8 fix it selects | `k` on `complete-records-stale.ein` |
|---|---|---|---|
| **A** | the recorded object is the **solution state**, and `model` is a *projection* of it (positive part minus the initial KB) computed at the read-out. § 2's conjuncts are evaluated on the state, `K` included | **re-saturate and re-check** before recording | **1** |
| **B** | the **model** is the object: `K` is bookkeeping, never part of what is recorded or compared, and the criteria are evaluated on the projection | **strip `K`** — a read-out change, no search change | **2** |
| **C** | status quo, made explicit: the state is the object and `K` is part of it | none; `-K` legitimately changes every recorded fact set | 2, with model 1 refutable |

### Ruled 2026-08-28 — **A**

*The recorded object is the solution **state**. `model` is a projection of it,
computed at the read-out: the positive part minus the positive initial KB. § 2's
conjuncts are evaluated on the state, `K` included — so a recorded state must be
**saturated** and **consistent** with everything in it, whoever wrote it.*

Taken on the recommendation below, at the user's direction, after D3's check
made the fork visible. What settled it was not cost but **entailment**:

> **`K` is not bookkeeping — it is a set of consequences.** A kill-cache
> negative `(not h)` means *`S ∪ {h}` derives `(false)` in one firing*, and a
> writeback negative means *the fork for `{h}` died*. Both are entailed by `S`.
> A rule that reads one is therefore reading a true consequence of the state,
> and a state whose own rules refute it **is** inconsistent. Hiding the
> negatives (B) would hide an entailed contradiction; keeping them unread (C)
> records the contradiction and calls it a model.

That also settles what looked like A's worst consequence. `-K` reports `k = 2`
on `complete-records-stale.ein` where A reports `k = 1`, and the earlier reading
of that was *a lever changes the answer*. It does not: `{(q A)}` entails
`(not (p A))` and `(not (p B))` through `kill-p`, and those entail `(false)`
through `totality`, so **`k = 1` is right and `-K`'s `k = 2` is the cache being
less complete** — the engine simply never noticing an entailment it had no
reason to materialise. The three probes agree: under A every one of them
answers what `-L` answers, which is what the hand derivations support.

### What the ruling selects

| | consequence |
|---|---|
| [Q-M1e.8](#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set) / [CO-M1](README.md#the-findings) | **fix (ii)** — re-saturate and re-check before recording, or the equivalent *dirty since its last saturation* guard ([D1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d1_q4_which_route_reaches_the_site.md) option B). Fixes (i) and (iii) are off the table: neither reaches the inter-layer site, and (iii) would hide an entailed contradiction |
| the read-out | a **projection** has to exist. `model` = positive part − positive initial KB, and `ein solve` / `--json-summary` print it under that name. P1e.5's S1e.5.3 |
| the vocabulary | two objects, two names, in the four surfaces Q-M1e.7 lists — `SE-M1`'s defect at a second site. [S1e.3.2](p1e.3_medium/s1e.3.2_semantics.md) |
| every fact-set comparison | [`tree_traversal.rs`](../../ein.rs/crates/ein-infer/tests/tree_traversal.rs), the corpus goldens, `model_set_census.py`'s variables — each has to say **which object** it compares. Comparing states is right for a search-equivalence test and wrong for a model-set one, and today they all compare states without saying so |
| the corpus | the fix can only move an entry whose program **reads** a fact the search wrote. Expected zero; it is the fix stage's to measure, not to assume |

### The three readings, as they were put

The probes in
[`s1e.1.1…/probes/`](p1e.1_open_questions/s1e.1.1_search_soundness_probes/probes/)
make this concrete rather than terminological. Write `S` for the recorded KB
and `K` for what the search wrote into it — the lookahead kill cache's `(not
h)` and the singleton writeback's.

| | ruling | the Q-M1e.8 fix it selects | `k` on `complete-records-stale.ein` |
|---|---|---|---|
| **A** ✅ | the recorded object is the **solution state**, and `model` is a *projection* of it (positive part minus the initial KB) computed at the read-out. § 2's conjuncts are evaluated on the state, `K` included | **re-saturate and re-check** before recording | **1** |
| **B** | the **model** is the object: `K` is bookkeeping, never part of what is recorded or compared, and the criteria are evaluated on the projection | **strip `K`** — a read-out change, no search change | 2 |
| **C** | status quo, made explicit: the state is the object and `K` is part of it | none; `-K` legitimately changes every recorded fact set | 2, with model 1 refutable |

B is also **wrong on the inter-layer probe** independently of the entailment
argument: its trigger is the *singleton writeback*, and evaluating the criteria
on the projection there reports a model of **∅**, which is neither answer the
hand derivation admits. C keeps a recorded state its own rules refute and makes
`tree_traversal.rs`'s fact-for-fact comparison lever-sensitive by design.

A costs the most: a projection at the read-out, a saturation before recording,
and a decision about which object every fact-set test compares. It is the only
one of the three that leaves § 2 true of the thing the engine records.

## Q-M1e.8 — `exhausted` certifies the **lattice**, not the model set

Raised by [Q-M1e.6](#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
operational form, and it has a witness in the corpus today.

`exhausted = !truncated` ([`solve.rs:2388`](../../ein.rs/crates/ein-infer/src/solve.rs))
is set when the search did not stop early — no depth cap hit with a live
frontier, no `stop_after` cut, not the tree. It is read by the verdict as a
**certification**: `Ambiguity` prints a bare `k` rather than *"(a lower
bound)"*, and `Contradiction` prints *"No solution — the constraints are
contradictory"* rather than the hedged *"the search did not exhaust the
lattice"*.

It certifies the wrong thing. Run, measured 2026-08-27:

```
ein solve -e -L examples/lattice/02_genuine_3set_death.ein
  solutions (k)   0            exhausted = true      7 enterings, 3 layers
  verdict         No solution — the constraints are contradictory
```

That program has **three** solutions under Q-M1e.6 — `{h₁,h₂}`, `{h₁,h₃}`,
`{h₂,h₃}` — and the search *found all three states*: it entered each pair,
each survived, and it then proved every triple dead. Every fact the right
answer needs was in `lstate`. What failed is that no surviving pair was
flagged `solved`, because `complete()` with the lookahead off still proposes
the third candidate — and `finalise` reads only `lstate.nodes`, which is
empty, so the Contradiction arm unions the dead cores and asserts
unsatisfiability.

**Two completeness notions wear one word.** *The lattice was walked to the
end* and *every solution in it was recognised* are different claims, and only
the first is what `truncated` tracks. A verdict that says *the constraints are
contradictory* is asserting the second.

The narrow fix is available and is **cheaper than making the lookahead
unconditional**: a surviving commitment whose every superset died is a
solution by construction, layer `n+1` already computes that, and retaining it
costs one bitset over `a_prev` per layer — no new fork, no new saturation. Under
`-e` it is free; under `-n 1` it defers a model by one layer, which is a
trade-off to measure rather than assume.

Owner unassigned. It is the same seam as
[CO-H3](README.md#the-findings)(b) (a `Contradiction` whose evidence is empty)
and [Q-M1d.6](../../docs/history/m1d_satisfiability/open_questions.md)
(may `Contradiction` be said with `exhausted = false`) — but strictly worse
than either, because here `exhausted` is **true**.

## Q-M1e.9 — Is `dead` really upward-closed under `absent`?

**Answered 2026-08-28: no.** Filed the same day as *two kernel pages appear to
disagree*, and **not** promoted to a finding pending a probe. The probe was
run. The full account, the attribution matrix and the four disposition options
are [D4](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md);
this entry is the ledger row.

The premise, as [design/08 § The objects](../../docs/history/m1a_rust/design/08_parallelism.md)
states it:

> `dead(X)` — `X` holds a contradiction. **Monotone**: `X ⊆ Y ∧ dead(X) ⇒
> dead(Y)`, because the KB is append-only and nothing retracts.

*Append-only* makes `sat` **inflationary**, not monotone in its input, and
`absent` is what separates the two — which
[C3](../../docs/kernel/inference/absent_semantics.md) already says from the
other direction.

**The counterexample is twenty lines**
([`probes/naf_upward_closure.ein`](p1e.1_open_questions/s1e.1.1_search_soundness_probes/probes/naf_upward_closure.ein),
re-taken by the `.sh` beside it): one rule `(and (p ?x) (absent (q ?x))) ⇒
(false)`. `{(p A)}` is dead; `{(p A), (q A)}` is alive; the single solution is
`{(p A), (q A)}`. **Five of the six shipped configurations do not report it**,
and every one of them says `exhausted = true`:

| configuration | enterings | recorded |
|---|---:|---|
| default | 0 | `(q A)`, `(not (p A))` — **wrong** |
| `-K` | 0 | `(p A)`, `(q A)` — right, and by accident |
| `-L` / `-L -K` | 2 | `(q A)`, `(not (p A))` — **wrong** |
| singleton-writeback off | 0 | `(q A)`, `(not (p A))` — **wrong** |
| singleton-writeback off + `-L` | 2 | **`k = 0`, Contradiction** |

Three shipped mechanisms read the premise, each sufficient on its own: the
**lookahead kill cache** (writes `(not h)` with empty provenance — C6
violated by a cache), the **singleton writeback** (design/08 claim (1) failing
on its own terms), and the **no-good store with apriori's filter** (a width-1
clause subsuming a live superset). `(config :warn-derived-naf true)` emits
nothing: it watches rule-derived relations, and here the relation is
hypothesis-eligible.

**Owner undecided.** D4 sets out four options; its recommendation is a
load-time refusal now — the compiler already knows every guard's watched
relations and every hypothesis-eligible relation — with the real fix filed,
starting from `Prov::absent`, which has recorded the negative premises since
S1.21.8 and which *"no walk yet interprets"*.

## Q-M1e.10 — Two `(config …)` flags are inert

**Raised 2026-08-28** by
[S1e.5.1](p1e.5_documentation_and_other/s1e.5.1_config_reference.md), which
had to write a *what it changes* column for all seventeen flags and found two
with nothing to put in it. The stage's own rule — *"anything found gets a
`Q-M1e.<n>`, not a quiet fix in a doc stage"* — is why this is here rather
than a patch.

`print-alive` and `candidate-order-seed` are, like the other fifteen:

- in [`FIELDS`](../../ein.rs/crates/ein-core/src/config.rs), so the loader
  accepts them and **rejects a wrong-typed value with a positioned
  diagnostic** (`examples/broken/load/config_bad_value.ein` is
  `:print-alive 7`);
- printed by `ein solve --dump-config` in declaration order;
- echoed into `--json-summary`'s `config` block
  ([`summary.rs`](../../ein.rs/crates/ein-cli/src/summary.rs));
- part of `rendered_fields`, so they are **rendered into the KB-shape digest**
  that every corpus shape golden is taken over;
- read and written by the `.einb` container's meta accessors
  ([`meta.rs`](../../ein.rs/crates/ein-einb/src/meta.rs)).

And read by **no code path**. `grep -rn 'print_alive\|candidate_order_seed'
ein.rs --include='*.rs'` returns the loader, the four renderers, the container
and their tests, and nothing in `ein-infer` or `ein-render`. Probed from the
outside as well, because a grep is an argument about absence: appending
`(config :print-alive true)` or `(config :candidate-order-seed 7)` to
`examples/branching/04_two_levels.ein` and
`examples/lattice/02_genuine_3set_death.ein` leaves `ein solve -e -p`'s stdout
**byte-identical**. That probe is banked as
`ein-cli/tests/config_reference.rs::the_two_inert_flags_are_still_inert`, so
the claim fails loudly if either flag is ever wired up.

### Why it is a question and not a defect

Because the two are **port gaps whose surface crossed and whose behaviour did
not**, and the port's own oracle could not have caught it: T0–T3 compared two
implementations' *observable output*, and a knob that does nothing produces
identical output in both engines when it does nothing in both — but ein.py's
did something. [`docs/api/inference.md`](../../docs/api/inference.md) is the
frozen contract of the engine that was:

| flag | what ein.py did |
|---|---|
| `print_alive` | *"Diagnostic — log inherited alive-set size + per-filter prune counts per `_explore`."* |
| `candidate_order_seed` | *"`< 0` → deterministic content-sort branch order; `≥ 0` → a deterministic per-branch permutation (shuffle-invariance probing)."* |

Neither is a hole in today's engine. `print_alive`'s read-out exists under
other names — `--verbose`, `--layer-progress`, and `--events`'s `layer`
counters — and `candidate_order_seed`'s job is done one level up by
`lattice_order_seed`, which permutes a *layer* rather than a branch and is
what `id_order_invariance` and the `--shuffle` corpus cells actually drive.
Which is very likely why nobody missed them.

**`config.rs` still describes the behaviour of one of them.** Its
`candidate_order_seed` doc comment reads *"Negative means the S1.5a.1a content
sort; non-negative applies a per-branch deterministic permutation of it"* —
[MA-M2](README.md#the-findings)'s class (*stale rustdoc contradicting the code
it documents*) at a third site. S1e.5.1 left the sentence and added the
inertness beside it rather than deleting it, because deleting it would erase
the only statement of what the flag was **for**, and that is the input to
option (a) below.

### The options

| | what | what it costs |
|---|---|---|
| **(a)** | wire them up | `candidate_order_seed` is a real second probe axis; `print_alive` is a duplicate of three existing read-outs. Only the first is worth anything |
| **(b)** | delete them from `FIELDS` | **a surface change.** A program that sets one becomes a load error; the KB-shape digest changes for every entry, so every shape golden in the corpus re-blesses; `.einb`'s meta loses two accessors. Two flags' worth of tidiness for a corpus-wide re-bless |
| **(c)** | keep, and say so | what S1e.5.1 did: the row reads **inert** in both judgement columns, and a test holds it |
| **(d)** | keep, and *refuse* them at load | a deprecation diagnostic naming the replacement. Costs a `broken/load/` fixture apiece and is still a surface change, but a *diagnosed* one |

**Recommendation: (c) stands, and (b) is the one to take if any milestone is
already re-blessing the shape goldens** — the cost is entirely in that
re-bless, so it is free inside a change that pays it anyway. (a) only if
something wants a per-branch order probe that `lattice-order-seed` cannot
give; nothing does today.

Owner unassigned. The natural readers are
[S1e.3.9](p1e.3_medium/s1e.3.9_maintainability.md) (MA-M2's site is its) and
[P1f.10](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/README.md)
[S1f.10.5](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.5_ordering.md), which is already
holding the other knob that does less than its name says
(`hypgen-scoring: most-constrained`, a constant `0.0`).

---

## Q-M1e.11 — What happens to an obligation **derived under a hypothesis**?

**Raised by** [D2](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d2_q6_which_decline_to_construct.md)
on 2026-08-28, which split the review's `Q6` in two.
**Owner: [S1f.10.6](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md)**
— the stage D2 became the same day, by the user's instruction. Its T4 is what
rules on this; the ruling is written into this file with the date, and beside
the code that computes the structure.

### What is decided, and is *not* this question

**The rung mode must be re-read at every node.** It is probed once at root
([`solve.rs:889-914`](../../ein.rs/crates/ein-infer/src/solve.rs)) on the
premise that *"the mode is a property of the program rather than of the node,
so asking once is asking enough"*, and that premise is false: an activator is
an ordinary fact, and a rule head can derive one inside a fork
([`compile.rs:54-69`](../../ein.rs/crates/ein-infer/src/compile.rs)). The
value is in fact **already computed at every node and thrown away** —
`tree_node` builds a `HypGenStats`, calls `generate_one_branch`, keeps the
candidate list and drops `hs.rung.mode` (`solve.rs:945-956`) — so re-reading
it costs nothing that is not already paid. That is a **guard**; it is decided;
[S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) T3 writes it.

### The question the guard does not answer

A guard says *stop*. It does not say what the search should **do** when the
theory acquires an obligation it did not have at root — and that is a question
about the **structure of the hypothesis set**, not about a traversal:

- The candidate set **grew underneath a branch** that was entered because its
  alternatives were jointly exhaustive.
  [`domain_contract.md`](../../docs/history/m1d_satisfiability/domain_contract.md)
  C4 states the contract exactly — *a branch is jointly exhaustive only while
  the candidate set cannot grow underneath it* — and says nothing about what
  to do when it does.
- The growth is **monotone**: the KB is append-only, so an obligation can
  appear under a hypothesis and can never be retracted under a deeper one.
  Whatever the answer is, it does not have to handle a set that shrinks.
- **Discharge changes meaning.** M1d S1d.2.6 scoped `Open` so that *a state is
  judged by discharge when it has been told what it owes*. A node told at
  depth 3 what root was not owes what its ancestors did not, so `complete` at
  that node is not the same predicate as `complete` at its parent — which is
  the mechanism by which a model goes missing, per
  [D2](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d2_q6_which_decline_to_construct.md)
  § The loss mechanism.

### Three shapes, which may not have one answer

| | shape | why it may differ |
|---|---|---|
| **a** | the new obligation is on a relation the branch structure already covers | the group gains members: the branch taken may still be exhaustive over the old alternatives and not over the new ones |
| **b** | the new obligation is on a **new** relation | a whole group appears mid-search, and nothing above it ever branched on it |
| **c** | the activator is derived on one path and not on its sibling | the two paths are then answering different questions, which is what makes their model sets hard to compare at all |

### Candidate answers, none taken

| | what | consequence |
|---|---|---|
| **A** | re-derive the branch at the node where the set grew, and continue | the honest one; needs `complete` to be relative to the node's own obligation set, not root's |
| **B** | decline the traversal at the flip and fall back to the lattice | safe, and it throws away the descent so far. The lattice has no such premise to lose, which is why it is the fallback |
| **C** | refuse at load: every obligation activator must be root-derivable | a diagnostic instead of a wrong answer — the repo's usual move, and the shape of [Q-M1e.9](#q-m1e9--is-dead-really-upward-closed-under-absent)'s option B. It forbids a program nobody has yet written |
| **D** | accept the loss and state it | needs a witness first, which is what `Q6`'s probe is for |

### Why P1f.10 owns it

That phase's founding sentence is *"the search enumerates subsets of a fixed
`alive` set"*, and its ladder rests on a branch structure that is a property
of the **program** — *"it cannot flip under a hypothesis, which is the
property Q6 found the tree's rung probe lacks"*. A derived obligation is
exactly the case where the set is **not** fixed. So the phase either says what
its groups mean when the set can grow, or states that its structure is
computed only for programs where it cannot — and either is an answer.

---

## Q-M1e.12 — The blind rung is untyped, and a model binds a type as an object

**Raised by** [D8](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d8_branching06_untyped_models.md)
on 2026-08-28, out of S1e.1.1's reconnaissance. **Owner unassigned** — filed
because a question the milestone found belongs in the milestone's ledger, not
because anything is scheduled to answer it.

### What was measured

`examples/branching/06_lookahead_on.ein` answers `Ambiguity k = 22`, and
**20 of the 22 models bind `?h` to `Color` or `House`** — the types, not the
houses. Model 1 contains `(co-located Blue Color)`.

The mechanism is not a bug in the search. `candidate_objects` collects every
object the KB mentions; `(is-a Color T)` makes `Color` an object; the blind
enumerator (rung 3) is **untyped**, so `(co-located Blue Color)` is a candidate
like any other, and nothing in the program forbids it —
`(relation co-located T T)` says both arguments are `T`, and `Color` *is* a
`T`.

Two consequences that are already load-bearing elsewhere:

- the fixture cannot be the Q5 pair, because *"its solution set is derivable in
  a paragraph"* is false of it — which is why
  [D6](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d6_the_new_q5_fixture.md)
  builds a new one;
- it is the **standing proof** for
  [S1f.10.6](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.6_obligations_under_hypothesis.md)'s
  loss mechanism — the blind rung keeps proposing long after any real debt is
  discharged, which is why a tree node that flips to it stops recognising
  solutions. Without this fixture that argument is hypothetical.

### Three readings, and the question is which one it is

| | reading | what it would mean |
|---|---|---|
| **a** | **the program is under-specified** | fix the fixture, not the engine: `(relation co-located T T)` is what admits it, and `examples/branching/12_typed_blind_solve.ein` already shows the typed alternative |
| **b** | **the enumerator is right and the read-out is wrong** | a query goal should not print a binding its author cannot mean. A presentation question, next to [SE-M1](README.md#the-findings) / `AR-M2` |
| **c** | **`candidate_objects` should exclude types** | an engine change with corpus-wide reach, and the kind of thing S1.7.23 refused on purpose: *the kernel commits to no type system* |

**(c) is the one that cannot be taken casually.** A type is not a kind in this
language — it is an object that happens to appear on the right of an `is-a` —
so "exclude types" has to be spelled as a rule about facts, and every such rule
is a type system arriving by the back door. (a) is the likely eventual answer;
(b) is cheap and would make the symptom stop being *read* as a defect without
deciding whether it is one.

**Not filed as a defect**, deliberately: under every one of the three readings
the engine does what the program says. What is wrong is that nobody chose which
reading the repo holds, and a corpus fixture is quietly demonstrating the
consequence.

---

## Q-M1e.13 — `:expect` cannot state an answer with an empty goal extent

**Found 2026-08-28** while banking
[D1](p1e.1_open_questions/s1e.1.1_search_soundness_probes/d1_q4_which_route_reaches_the_site.md)'s
fixtures under option (i) — *bank them stating today's answer*. Two of the
three could not state it. **Owner unassigned.**

### What happens

The three witnesses answer `Solution` on a model that contains **no fact of the
relation the query asked about** — that is the defect they exist to show. Given

```lisp
(query :goal (p ?x) :expect (model (not (p A)) (not (p B))))
```

the loader refuses:

```
kb load error: :expect does not name p, which the query's :goal asks about
```

and so does `(model)`, and so does any list of facts that does not mention `p`.
The rule is
[`01_grammar.md` § Query](../../docs/kernel/ir/03-ein-lang/01_grammar.md#query)'s
first: *the goal's relations are mandatory; an expectation that does not pin
what the query asked is not an expectation.* Its reasoning is sound. Its
consequence is that **an answer whose goal extent is empty is unstatable**,
because the only way to name a relation is to list a fact of it, and there are
none.

**And every arm is checked separately.** For a two-model answer whose members
populate *different* relations —
`examples/ein-bugs/complete-records-stale.ein`'s `{(q A)}` and
`{(p A), (p B)}` — no `(or (model …) (model …))` loads, whichever relation is
the goal, because one arm can never name it.

### Why it matters beyond the three fixtures

- **It is the one mechanism that makes a verdict change visible in a golden.**
  The [golden audit](p1e.1_open_questions/s1e.1.1_search_soundness_probes/golden_audit.md)
  found that neither `corpus_exits.txt` (both sides exit `0`) nor
  `corpus_shapes.md5` (the KB shape does not change) moves when a verdict does.
  Only a failing `:expect` moves an exit code. So a fixture that cannot carry
  one cannot be banked *against* a fix.
- **An `(or …)` cannot detect a shrinking model set either.** A `k = 1` answer
  still matches one arm of a two-arm `or`, so an `Ambiguity` fixture cannot
  state a claim that fails when the set shrinks — which is exactly what the
  selected Q-M1e.8 fix does.

### What the workaround costs

The two `alive-empty-*` fixtures moved their `:goal` to `is-a`, a relation the
model does populate. It works, and it is a lie of omission: the file's question
is *what does the engine do about `p`*, and its query now asks about something
else, with a header paragraph explaining why. The third carries no expectation
at all.

### The options, unowned

| | | consequence |
|---|---|---|
| **a** | allow a **closed empty extent** — a spelling for *the goal relation has no facts*, e.g. `(model (closed p))` or bare `(model :closes p)` | the smallest change that makes the answer statable; needs one new keyword and a load-time rule |
| **b** | check the goal-naming rule against the **`(or …)` as a whole** rather than per arm | fixes the second half only, and weakens the rule's own reasoning: an arm that does not pin the goal is not an expectation of that arm |
| **c** | grow `:expect` a word for the **verdict** (`Contradiction`, `Ambiguity`, `Open`) | P1d.4 considered and declined it — *"tests stay exhaustive by default, `:expect` stays closed by default"* — but it is what these fixtures actually want to say, and it would make both halves moot |
| **d** | accept, and say so where the rule is stated | costs a paragraph in `01_grammar.md`; leaves a class of answer no program can claim |

(c) is the most useful and the most decided-against; the honest first move is
(d) plus a decision on whether (a) is cheap enough to be worth a keyword.

---

## Q-M1e.14 — The corpus `--jobs` sweep's per-layer census coverage is vacuous

> **Raised 2026-08-29 by
> [S1e.1.2](p1e.1_open_questions/s1e.1.2_determinism_under_jobs.md)
> T1e.1.2.3**, which went looking for where `dropped_nogood` is compared across
> job counts, found exactly one route, and measured it as a column of zeroes.
> **Owner unassigned.**

`LayerCensus::dropped_nogood` — how many of the next layer's joined candidates
a learned clause removed — is the **read** side of the shared no-good store,
where `nogoods_emitted` / `nogoods_subsumed` are the write side. It is
deliberately not in `MonotonicStats` (per layer, not per run), so its only
observable surfaces are the `layer` event and `ein-render`'s progress dumper.

**The corpus-wide sweep reaches it through exactly one op** —
`Op::Dump("progress")`, whose `layer N gen:` line carries the column — and that
op runs under `dump_shape`'s budget: `max_set_size = 3`, `max_enterings = 60`.
Measured 2026-08-29, all **202** corpus entries at that budget:

| | entries with a nonzero column in the progress dump |
|---|---:|
| per-layer `nogoods_emitted` (the write side) | **16** |
| `dropped_nogood` (the read side) | **0** |

So `jobs_invariance` compares the write side for real and the read side as a
column of zeroes agreeing with itself — 20 712 cells of coverage that is not
coverage, on the one counter the review's `Q1` names as the first a shared
clause store would perturb.

S1e.1.2 closed the gap where it could, in the unit sweep: `search_invariants`'s
three `--jobs` tests now compare **every layer's census row**, with a
non-vacuity assertion that some candidate somewhere is actually dropped by a
clause. That is one file's worth —
`examples/branching/02_one_dead_one_alive.ein`, which drops 65 of 312
candidates at layer 3 and 16 of 166 at layer 4 — rather than the corpus's.

**What is open** is whether to make the corpus route non-vacuous, and it is a
cost question rather than a design one:

1. **Raise `dump_shape`'s `max_enterings`.** One line, and it re-blesses every
   `dump[progress]` cell of `corpus_shapes.md5` — a deliberate golden move,
   which [P1e.1's acceptance](p1e.1_open_questions/README.md#acceptance) says
   has to be named in a stage file *before* it happens. S1e.1.2 did not predict
   it, so S1e.1.2 does not take it.
2. **A second `Op` at a larger budget**, which costs a sweep column rather
   than a golden — and is what `Op::Solve` already does with three modes.
3. **Leave it**, on the argument that the read side is a pure function of
   `(joined, alive, store)` and all three are pinned. That is an argument whose
   premise is enforced, so it is admissible under
   [`standard_of_proof.md`](../../docs/kernel/standard_of_proof.md) Rule 2 —
   but it should then be *written*, because "we compare that column" and "that
   column is entailed by three we compare" are different claims and the tree
   currently makes the first.

---

## Q-M1e.15 — The alternatives cap decides which unsat core is reported

> **Raised 2026-08-29 by
> [S1e.1.3](p1e.1_open_questions/s1e.1.3_unsat_core_completeness.md)**, which
> is the review's **Q2** answered *yes*. The stage establishes the promise; the
> fix is engine work and is **not** taken there. **Owner unassigned.**

`Kb::record_justification` keeps at most `MAX_ALT_JUSTIFICATIONS = 32`
alternatives per fact, sorted by **premise count**, refusing an arrival no
shorter than the longest kept. `explain.rs` minimises **frontier size** over
what survived. The two metrics disagree — premise count is local, frontier size
is transitive — so a derivation whose frontier is *smaller* can be refused
because it has *more* premises, and the search never sees it.

**Witnessed, not argued.** `examples/ein-bugs/alt-cap-core.ein` and
`examples/ein-bugs/alt-cap-core-reordered.ein` differ in one `:priority` and
nothing else:

| | `(false)`'s primary | reported core |
|---|---|---:|
| `alt-cap-core.ein` | `narrow` — one premise, three givens deep | **3** |
| `alt-cap-core-reordered.ein` | `wide` — two premises over givens | **2** |

Both files hold `(w1 X)` and `(w2 X)`, and `wide` fires in both. Verified
against the cause rather than inferred: at `MAX_ALT_JUSTIFICATIONS = 1_000_000`
the first file reports the same 2-fact core as the second. Banked as
`ein-infer/tests/explain_semantics.rs::the_alternatives_cap_can_enlarge_the_reported_core`.

**The sharper half is not the size, it is the order.** `explain.rs` exists
because walking one justification per fact made the core depend on `:priority`;
this pair shows the cap giving that back one level up. The search is
order-independent over what the store *kept*, and what the store keeps depends
on firing order — so `glossary.md`'s *"Independent of the order in which the
rules fired"* was true of the search and not of the pipeline, and S1e.1.3
re-worded it along with the five other statements of the promise —
`README.md`, `explain.rs`, `reserved_engine_strings.md`,
`architecture_and_algorithms.md` §O6 and `ir/02-data-model/01_entities.md`
§3.1, the last of which said the cap *"retains the shortest derivations — the
ones a minimum-cardinality explanation can use"* and was simply false.

**It is not urgent, and the measurement says why.** All 202 corpus entries,
2026-08-29:

| | |
|---|---:|
| entries recording any alternative | 50 |
| entries that reach the cap | **1** — `examples/ein-bugs/zebra2-bad.ein` |
| arrivals refused there | 1 017 of 2 425 |
| longest list without the cap | 1 049 (next-longest entry anywhere: **8**) |
| evictions corpus-wide | **0** |

**The cap can only change an answer on an entry that reaches it.** One does,
and its core is `(color-loc Green House-1)` — one fact — at 32 and at 10⁶
alike; every corpus file's *root* explanation, alternatives on and off over an
unbudgeted saturation, is byte-identical between the two caps. So the cap is
live on exactly the program the README names as the unsat-core fixture and
costs it nothing.

**The three fixes, and none is obviously right.**

1. **Retain by a frontier estimate** rather than premise count — e.g. the sum
   of the premises' recorded depths. Right metric, and it makes retention
   depend on a quantity that changes as the KB grows, so the list would have to
   be re-sorted rather than inserted into.
2. **Raise the cap.** The measurement bounds the cost: the worst list would be
   1 049 and the second-worst 8. Cheap, and it converts a wrong answer into a
   less likely wrong answer rather than into a right one.
3. **Record the minimum separately** — keep, beside the capped list, the single
   arrival with the smallest premise closure seen so far, so the search always
   has the best candidate whatever the cap did. Bounded, exact for the one
   thing the search asks, and it needs a closure size the recorder does not
   have.

The stage did not choose, because choosing is the fix and the fix has to be
measured on something that is not one synthetic fixture.

## Q-M1e.16 — The binding key compares two register layouts as one

> **Raised 2026-08-29 by
> [S1e.1.4](p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md)**, which
> is the review's **Q3** answered and
> [`Q-M1a.8`](../../docs/history/m1a_rust/open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args)
> closed *as stated*. This is what was left when the probe came back: not the
> bug that entry described, but a neighbour of it. The stage states the
> behaviour; the fix is engine work and is **not** taken there.
> **Owner unassigned.**

`BindingKey` is `(rule, activator, values)`. `activator` is an interned
`plan.activator_args` — the **symbol** arguments — and `values` is the plan's
register file, which the activator seeds for every argument that **binds a
parameter**: symbols and integers, but not a nested `Fact`. So an `int` and a
nested `Fact` in the same activator position produce two plans that share an
`activator` and disagree on their **register layout**:

| activator | `activator_args` | `reg_names` |
|---|---|---|
| `(note edge 1)` | `[edge]` | `[?R, ?f, ?a, ?b]` |
| `(note edge (src Y))` | `[edge]` | `[?R, ?a, ?b, ?f]` |

The identity then compares `(?R ?f ?a ?b)` against `(?R ?a ?b ?f)` position by
position. A vector that is a legitimate match of both — `(edge 1 2 3)` below —
makes the second application look like a repeat of the first, and it is
dropped before it is enqueued.

**Witnessed, not argued.** Thirteen lines, and the losing application is the
only one that would have derived `(noted 1 3)`:

```lisp
(relation edge  Node Node)
(relation holds Node)
(relation noted Node Node)

(rule note (?R ?f)
  :match  (and (?R ?a ?b) (holds ?f))
  :assert (noted ?a ?f))

(edge 1 2) (edge 2 3)
(holds 1)  (holds 3)

(note edge 1)
(note edge (src Y))
```

| the program | derived |
|---|---|
| the nested-`Fact` activator alone | `(noted 1 1)` `(noted 1 3)` `(noted 2 1)` `(noted 2 3)` |
| the `int` activator alone | `(noted 1 1)` `(noted 2 1)` |
| **both** | `(noted 1 1)` `(noted 2 1)` `(noted 2 3)` |

**Adding an activator removed a conclusion**, and swapping the two lines puts
it back — the plan that fires first wins, and plan order is source order. It
is deterministic and it is not a
[design/02](../../docs/history/m1a_rust/design/02_determinism_and_order.md)
violation, since the order two facts were written is part of the input; it is
the same *shape* as the unsorted goal row in
[`defined_behaviour.md` §6](../../docs/kernel/defined_behaviour.md#6-what-is-not-defined-and-is-filed),
one severity up, because here the answer is wrong rather than arbitrary.

**Two profiles, and only one of them is quiet.** `Engine::check_layout`
asserts exactly this invariant where the plan list is built — under
`debug_assertions`, so the program above **panics** `cargo test` and answers
wrongly in a release build. The assertion's own doc comment called the shape
*"a shape no rule application has"*. That is why the reproducer is **not** a
corpus fixture: the corpus runs through a debug binary. Both halves are banked
in `ein-infer/tests/rule_semantics.rs` as
`an_int_beside_a_nested_fact_in_one_position_loses_a_derivation`, which expects
the panic in one profile and the missing fact in the other.

**No corpus program can reach it**, which is why it has never been seen. Every
plan compiled by `ein solve -m 2` over all **204** `.ein` files under
`examples/`, `stdlib/` and `tests/` — forks' plans included — binds against
**153 143** activator arguments, and **every one is a symbol**: not one
integer, not one nested `Fact`, and **0** `(rule, activator)` spaces holding
more than one plan (measured 2026-08-29, M1e S1e.1.4). The same number is why
the first fix below is free.

**The three fixes, and none is obviously right.**

1. **Give `ActivatorId` every argument**, not the symbol-filtered list — the
   compile-cache key already computes exactly that string, so the two keys
   would agree and each plan would get its own space. It is the smallest
   change and, on today's corpus, **provably golden-neutral**: with every
   activator argument a symbol, filtered and unfiltered lists are the same
   list. What it also changes is what `fire` / `enqueue` / `compile` print in
   their `activator` field, and what `naf_deps.rs` and `obligations.rs` key
   on, for any program that does have a non-symbol argument.
2. **Put the layout in the key** — an interned `reg_names` beside the
   activator id. Narrower: it makes the comparison sound without touching what
   any event prints, and it deliberately leaves the harmless `Fact`/`Fact`
   collapse collapsed. It costs one more word per key on the engine's hottest
   path.
3. **Refuse the program.** Promote `check_layout` to a load-time or
   compile-time error in every profile: *two activators of `note` bind
   different parameters and would share an identity*. Cheapest, and it turns a
   wrong answer into a diagnostic — but it refuses a well-formed program,
   which is a language decision and not a repair.

The stage did not choose, because choosing is the fix, and (1) and (2) differ
in what they promise a program that *does* carry a non-symbol activator
argument — a question no corpus entry asks.

## Q-M1e.17 — Three `py_int` options silently reinterpret a negative

> **Raised 2026-08-29 by
> [S1e.1.5](p1e.1_open_questions/s1e.1.5_cli_semantics.md)**, the review's
> **Q7**, which ruled on one of the three and measured the other two on the way
> past. Refusing them is a validator apiece; *what* to refuse is the decision,
> and it is not the same decision as `-n`'s. **Owner unassigned.**

`ein solve` builds its budget fields with `.max(0)`, so a negative becomes
zero and the zero then means whatever that flag's zero means. Q7 asked the
question of `-n` and the answer there was *refuse*, because `-n 0` had no
reading. The other budget flags are not in that position: **their zeros are
defined**, so the question is only about the negative, and it has a different
answer available — *honour it*, which is what a fourth option already does.

Measured 2026-08-29, `examples/zebra2.ein`, the `=` spelling throughout (a
bare `-E -3` is `clap`'s own *unexpected argument*):

| option | `0` | a negative | verdict |
|---|---|---|---|
| `-n`/`--solutions` | was `-n 1` | was `-n 1` | **refused since S1e.1.5** |
| `-m`/`--max-set-size` | a truncation, defined by M1d T1d.10.5.0 | **clamped to 0** — `-m=-3` answers `Contradiction k=0 exhausted=false` on a puzzle with a model | open |
| `-E`/`--max-enterings` | `** aborted: max-enterings (0) reached **` | **clamped to 0** — and see below | open |
| `saturate --max-steps` | the limit is hit at once | **honoured**: the message says `max_steps=-3` | fine, and it is the precedent |
| `-g`/`--progress-every` | documented *0 disables* | disabled, by the same `> 0` guard | fine |
| `-d`/`--seed` | — | **normative**: `--seed=-7` is `--seed 7`, [`defined_behaviour.md` §3.1](../../docs/kernel/defined_behaviour.md) | fine |

**`-E`'s is the one that is more than untidy.** The abort line is built from
`self.opts.max_enterings` — the *clamped* value — so a run given `-E=-3`
prints:

```text
** aborted: max-enterings (0) reached **
```

A diagnostic reporting a number the user did not type is a different defect
from a flag accepting a value it should not: the first is unfalsifiable from
the output. `saturate --max-steps` shows the alternative costs nothing —
`saturator hit max_steps=-3` names what was asked and lets the reader see the
mistake.

**Three candidate rulings, and the stage that files this did not choose:**

1. **Refuse below zero** on `-m` and `-E`, `solutions_spec`-style. Consistent
   with `-n` and with `--jobs`, and it keeps both defined zeros. It refuses a
   spelling nobody uses — no corpus entry, `utils/` script or doc passes a
   negative to either.
2. **Honour the negative** the way `--max-steps` does: drop the `.max(0)`,
   carry the number into the message, and let a negative budget be a budget
   that is already spent. Cheapest, and it makes the diagnostic true; it also
   leaves `-m=-3` meaning `-m 0`, which is a fact a reader must be told.
3. **Refuse the negative, keep the clamp for the caller.** A library caller
   builds `SolveOptions` directly and is not bound by the CLI's validator, so
   the clamp is a cast guard rather than a policy — which is what
   `ein-cli/src/solve.rs` now says at `-n`'s site.

(2) and (3) are not exclusive; (1) and (2) are. What decides between them is
whether a *budget* flag should be strict or forgiving, and that is a question
about the CLI's manners rather than about the engine — which is why it is a
question and not a fix.

## Q-M1e.18 — Three kernel primitives are not shape-pinned, and drop their extra arguments

> **Raised 2026-08-29 by
> [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md)** T1e.1.6.2 — the
> parser/CLI edge sweep the review's `Q9` says never happened. The panic half
> is [`CO-H1`](README.md#the-findings) and belongs to
> [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md); **this is the other half**, and
> it is the half with no diagnostic at all.
>
> **Answered 2026-08-29 — candidate (2)**, by that same stage, on the day this
> was filed. `CO-H1` arrived as one cell and the sweep had already made it a
> rule, so fixing the cell alone would have left five worse ones standing; T1
> took the rule. See § The answer at the end.

[`00_ebnf.md` §2](../../docs/kernel/ir/03-ein-lang/00_ebnf.md) has a block
headed *Kernel meta-primitives (shape-pinned)* with **four** productions —
`NotForm`, `NeqForm`, `AndForm`, `OrForm`. The engine has **seven** such
primitives. `eq`, `absent` and `false` are ordinary `GenericList`s, so their
arity is checked by whatever happens to read them, and what happens to read
them is not the same in the three cases:

| written | a reader expects | today |
|---|---|---|
| `(eq)` · `(eq ?x)` | a diagnostic | **panic**, exit 101 — `match_.rs`'s `assert!(args.len() >= 2)`. This is `CO-H1` |
| `(eq ?x A B)` with `A ≠ B` | a diagnostic, or a guard that fails | **fires** — `guard_holds` reads `args[0]` and `args[1]` and drops the rest |
| `(absent)` | a diagnostic — `(absent ?x)` gets a *CompileError* saying the guard can never pass | **silence**: the rule is retired for the run and nothing says so |
| `(absent (q ?x) (p ?x))`, `p` non-empty | a diagnostic, or a guard that fails | **fires** — everything past the first argument is dropped |

**The two dropped-argument rows are worse than the panic**, and that is the
finding. A panic is loud and stops the run; a guard that silently evaluates a
weaker condition than the one written is a **wrong answer with a success exit
code**. A three-way equality reads as a two-way one; a two-subject `absent`
reads as a one-subject one.

**The sweep that found it is a rule, not a list.** Every cell of the pinned
four is a positioned parse error; every cell that panics or misbehaves is one
of the unpinned three. Both halves are banked as
`ein-cli/tests/primitive_arity.rs`'s two tests, which pin today's behaviour
*including the defects*, so a fix has to move them.

**Three candidate fixes:**

1. **Pin the other three in the grammar** — `EqForm ::= '(' 'eq' Value Value
   ')'`, an `AbsentForm` with one `Value`, a `FalseForm` with none. It makes
   every cell a positioned parse error, which is the cheapest diagnostic the
   engine has and the one a *generated* program most needs (M2 is the
   generator). It also moves `eq`, `absent` and `false` into `RESERVED`, which
   changes what lexes as a `SYMBOL` — the one thing on this list that could
   break a program that works today.
2. **Check the arity where it is read**, leaving the grammar alone: a
   `CompileError` from the compiler for `eq`/`absent`, the way `(absent ?x)`
   already gets one. Narrower, no lexical change, and it puts the message
   where the rule author is looking — but it leaves three primitives whose
   shape is checked in three different places.
3. **Refuse only what is unambiguous, and define the rest.** `(eq a b c)` could
   *mean* an n-ary equality and `(absent a b)` an n-ary conjunction; both are
   defensible languages, and either is a feature rather than a repair.

(1) and (2) are the repair; (3) is a language decision and would need
[`00_ebnf.md`](../../docs/kernel/ir/03-ein-lang/00_ebnf.md) and
`defined_behaviour.md` to move together. What none of them may do is nothing:
the current state is that two of the seven primitives read fewer arguments
than they are given, in silence.

### The answer — (2), 2026-08-29

[S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) T1 checks the arity **where the
form is read**: `Compiler::premise` refuses any `eq`/`neq` that is not binary
and any `absent` that is not unary, with a `CompileError` naming the form and
its position. All seven wrong cells of the sweep are now `Refused`, exit 1, and
the fourteen that were right are unmoved.

**Why (2) and not (1).** Pinning the three in the grammar is the cheaper
diagnostic and the one a *generated* program most wants — but it moves `eq`,
`absent` and `false` into `RESERVED`, which changes what lexes as a `SYMBOL`,
and that is the single item on this menu that could break a program working
today. This stage's mandate was three shipped defects, not a language change,
and (2) leaves (1) available: a grammar production would subsume the compiler's
check without contradicting it. (3) stays a feature and is not filed as one.

**Two things the answer added that the question did not ask for.** The
`CompileError` carries a **position** — `at Loc(file=…, line=…, col=…)` — which
none of the four S1.22.0 refusals beside it does, because a premise is a
`generic_list` and that is the one production the parser hands a `Loc`; and the
matcher's `assert!` became a `debug_assert_eq!` against
`Pred::arity`, which turns it from the only check on the program into a claim
about the compiler.

**`false` needed nothing.** It is the third unpinned primitive and the sweep
found no cell of it wrong: silence in a `:match` at every arity is what a
reader expects. So the row it does not have is deliberate, and this question's
title — *three* kernel primitives — over-counts by one on the repair side.

## Q-M1e.19 — Algorithmic pathology has no owner

> **Raised 2026-08-29 by
> [S1e.1.6](p1e.1_open_questions/s1e.1.6_coverage_gaps.md)** T1e.1.6.2. Of the
> four surfaces the review's aborted stage never reached, three now have a
> home and this one does not. **Owner unassigned.**

*Where does the search degrade, and is there an input class that makes it
degrade catastrophically?* Nothing in the tree asks. What exists is adjacent
and is not the same question:

- [`corpus_cost.md`](../../docs/history/m1a_rust/measurements/corpus_cost.md)
  prices the corpus as it is, and `slow = true` is a measured claim about
  **these** files.
- [`scaling.md`](../../docs/history/m1a_rust/measurements/scaling.md) measures
  `--jobs` on four workloads.
- [`layer_census.md`](../../docs/history/m1d_satisfiability/layer_census.md)
  measures what a layer costs and what a clause removes, again on the corpus.

All three measure the corpus. **A pathology pass measures what is *not* in the
corpus** — a family parameterised by n, run until it stops finishing, with the
exponent read off. The lattice is `Σₖ C(alive, k)` on 25 of the 49 entries that
search at all (`layer_census.md`), so the shape of the answer is already known;
what is not known is which programs put `alive` where nobody expects it.

There is one number in the tree that is already this question's: the tree
traversal reaches the same 32 models in **86** enterings where the lattice
needs **17 204 592** on `zebra2-minus-15-obligations.ein`. That is a 200 053×
gap on one input, and nothing says which inputs have it.

**Why it is not a task here.** A pathology pass is a measurement phase — a
family of generated inputs, a harness, a fitted exponent and a written
conclusion — and this stage's budget is one surface, which went to the
parser/CLI edges because that is the one with a demonstrated hit rate. Filed so
that the milestone cannot close claiming the tree was swept.

## Q-M1e.20 — Two renderers are produced, tested and unreachable

[S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) T3 was asked whether the
per-hypothesis lattice dump *should* be reachable, and told to route the
decision through the render crate rather than settle it while fixing prose.
Routing it here, with what the stage measured.

**It is not one artifact, it is two, and they are in the same state.**

| artifact | what it is | who can ask for it |
|---|---|---|
| the `LatticeDumper` tree | every commitment tested at every layer with the firings each emitted — `enterings/`, `proof_summary.json` ([`lattice_dump.md`](../../docs/kernel/inference/lattice_dump.md)) | Rust only: `LatticeDumper::new(Some(dir))` into `solve`'s `dumper`. No CLI flag; `--dump-states` builds the *other* dumper |
| `ein_render::kb_dot` | the unified KB DOT view — the whole fact graph on one page ([`04_dot_rendering.md`](../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md)) | Rust only. `ein render` offers `rules` / `rule` / `constraints` / `lattice` and no `kb`; `ein kb dot` was removed in P1.11 |

Both are `pub`, both are exercised by the gate — `golden_dump.rs` +
`dump_parity.rs` for the first, `dot_wellformed.rs` over the whole corpus for
the second — and neither is documented anywhere a caller would look:
[`docs/api/rust.md`](../../docs/api/rust.md)'s worked example stops at
solve-and-render. So the failure is not that the code rotted; it is that the
**surface** never grew a door, and two pages spent a milestone describing
rooms behind it.

[`utils/render_examples.sh`](../../utils/render_examples.sh) reached the same
fork at M1a S1a.10.4 and wrote the reason for declining, which is still the
right reason: *"Making them browsable again means putting them back on the CLI
(`ein render ir|kb`), which is a decision about the shipping surface and not
one a `utils/` clean-up should take."* A doc pass has even less standing.

Three options, and the third is the cheapest and is not obviously wrong:

- **(a) a flag each** — `ein solve --dump-lattice DIR` and `ein render kb`.
  Two CLI options against `configuration.md`'s 52, both wired to code that
  already exists and is already tested. Costs: the option census, a
  `--help` golden, and the standing question of whether a private debugging
  artifact belongs on the shipped surface at all.
- **(b) a documented library call** — a section in
  [`docs/api/rust.md`](../../docs/api/rust.md). Its worked example is a marked
  region of `ein-cli/tests/embedding.rs` that a test diffs, so this is a real
  test to write and it would keep working, which is the page's whole method.
- **(c) leave both unreachable and say so.** Done, as of S1e.2.2 —
  [`lattice_dump.md` § Reachability](../../docs/kernel/inference/lattice_dump.md)
  and `04_dot_rendering.md` § How to ask for this view both state plainly what
  can and cannot ask. Under that reading the *documentation* defect is closed
  and only the capability question is open, which is what this entry now is.

What tips it is a question this milestone has not asked: **who wants the
per-hypothesis dump?** It was built for debugging rule sets, `utils/` no longer
drives it, and [M20](../m20_gui/README.md) is the plausible consumer of the KB
view. If the answer is *the GUI*, (a) is premature and the feed is
[`--events`](../../docs/kernel/inference/events.md); if the answer is *a
puzzle author chasing a rule that will not fire*, (a) is the answer and the
flag is small.

