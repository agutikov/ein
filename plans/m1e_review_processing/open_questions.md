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
| [Q-M1e.1](#q-m1e1--what-is-the-standard-of-proof-for-refuted) | What is the standard of proof for **refuted**? | open — decided in [S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) T1, applied everywhere after |
| [Q-M1e.2](#q-m1e2--may-a-review-finding-be-closed-by-a-comment) | May a finding be closed by a comment rather than a check? | open — the `accepted` disposition's rule |
| [Q-M1e.3](#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted) | Who owns a `docs/kernel` page that should be neither fixed nor deleted? | open — [S1e.2.2](p1e.2_high/s1e.2.2_code_doc_consistency.md) decides per page; the *rule* is here |
| [Q-M1e.4](#q-m1e4--does-the-repo-want-an-exact-count-in-prose-at-all) | Does the repo want an exact count in prose at all? | open — [S1e.3.8](p1e.3_medium/s1e.3.8_documentation.md) |
| [Q-M1e.5](#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) | Is *experimental* a licence to ship a surface whose read-out is false? | open — [S1e.2.1](p1e.2_high/s1e.2.1_correctness.md), and M1d's `T1d.10.6.4` is the co-owner |

---

## Q-M1e.1 — What is the standard of proof for **refuted**?

Sixty of the sixty-three findings are one reader's reading; the review's
verification stage never ran ([`review/summary.md`](review/summary.md)
§ Method). So the milestone will refute some of them, and *refuted* needs a
bar, because the cheap version — "I read the code and disagree" — is the same
epistemic move that produced the finding.

The proposed bar, to be ratified in
[S1e.1.1](p1e.1_open_questions/s1e.1.1_search_soundness_probes.md) T1 and then
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

## Q-M1e.3 — Who owns a page that should be neither fixed nor deleted?

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
[S1e.2.1](p1e.2_high/s1e.2.1_correctness.md) T3 takes it. The general rule is
this question, and it belongs with `T1d.10.6.4` when M1d's traversal work
resumes.
