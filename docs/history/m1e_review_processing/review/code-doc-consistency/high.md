# Code ↔ doc consistency — High

## The canonical docs/kernel tree presents removed or never-built machinery as current, across at least six pages

**Severity:** High
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug (with elements of architectural drift)

**Locations**
- `docs/kernel/inference/algorithm_layer_n.md` (whole file)
- `docs/kernel/inference/lattice_dump.md:50-79, 107-113, 148-166, 188-204, 249-256, 263-264`
- `docs/kernel/inference/README.md:916, 1016-1024, 1040-1064, 1069-1080`
- `docs/kernel/ir/02-data-model/02_store.md:8-10, 27, 69, 112-113, 150-156, 329-331`
- `docs/kernel/ir/03-ein-lang/02_patterns.md:18-41, 84-98`; `docs/kernel/glossary.md:194-198`
- `docs/kernel/README.md:5-6, 38-48`
- `docs/kernel/ir/03-ein-lang/04_dot_rendering.md:8-11, 326-343, 385-394`
- `docs/kernel/inference/lattice_diagrams.md:39-48, 203-252, 284-291`
- `docs/kernel/inference/zebra_walkthrough.md:16-21`

### Finding

CLAUDE.md declares docs/kernel canonical and load-bearing ("this tree is now the only statement of intent that is not also the implementation … a claim here is checked by `cargo test` and by nothing else"). Substantial parts of it describe an engine that no longer exists — or never did — with no superseded banner:

- **algorithm_layer_n.md** is a P1.5b design document presented as live specification: three public solve entries (`monotonic_solve`/`gaps_solve`/`contradictions_solve`) where solve.rs has one `pub fn solve` (:594); unconditional-fact flat root-merge (retired P1.21 R2 as NAF-unsound — the retirement is a headline section of inference/README.md); state-hash dedup as identity (replaced by state_key representation-identity, P1.21 R1); multi-parent integrate (dropped). Its links were mechanically re-aimed at `ein.rs/crates/ein-infer/src/solve.rs`, so it *reads* current. inference/README.md:1040-1064 still cites it as "Algorithm spec" (with a nonexistent anchor §3d.vii), and architecture_and_algorithms.md:41-48 records the sibling-entries design as a removed **soundness bug** — the tree simultaneously asserts the design and its refutation.
- **lattice_dump.md** documents a `kb_index/` artifact tree (state_hash.txt, canonical_set.json, labels.json, verdict.txt) that the Rust LatticeDumper never writes (dump/lattice.rs:24-30, 284-285: kb_index is an empty array by construction), a "### Programmatically" section in Python importing `ein.inference.monotonic` (the engine deleted at S1a.10.5), a debugging workflow that depends on kb_index, and a wrong implementation pointer (LatticeDumper attributed to state.rs; it lives in lattice.rs). lattice_diagrams.md:284-291 tells readers a CLI invocation produces `proof_summary.json + kb_index/`, which the wired dumper does not emit — the per-hypothesis lattice dump is unreachable by any documented means.
- **02_store.md** documents singular last-wins `query` (:27, :69) against `program.rs`'s `queries: Vec` and 01_grammar's multi-query semantics (an M1c change never propagated); the deleted `add_type`/`add_instance` mutation API (:329-331) though §6 of the same file says that view is gone; the `_kb` back-pointer fork caveat as intentional current behavior (:150-156) though two sibling pages declare it historical; and lists kb.rs twice under "Sources of truth".
- **02_patterns.md + glossary.md** describe a predicate registry (`unique-remaining`, `no-remaining-option`, `forbidden-by-exclusion`) that appears nowhere in the crates — predicates.rs implements exactly `eq`/`neq`; `instance` is not a grammar-reserved head since S1.7.6, and the claim that an `(instnce ?a ?T)` typo is caught at parse time is false (any generic head parses). A reader credits the engine with aggregate machinery it does not have.
- **docs/kernel/README.md** — the tree's entry point — lists the P1.7c-removed six-block-forms surface language (:38-41) and calls the inference engine a "placeholder, P1.3" / "Stub before P1.3" (:5-6, 44-48) while its own §What's-M1 says the engine shipped.
- **04_dot_rendering.md** carries a runnable-looking Python section against the deleted engine, claims a `EIN_RENDER_LEVI` env var that greps empty in the whole tree (the only levi switch is library-level `DotOpts.levi`), and promises `from_dot` "when implemented in P1.2" — P1.2 closed in May 2026 and no from_dot exists.
- **inference/README.md**'s un-bannered strata contradict its bannered ones: "Two engines, two termination criteria" presents the removed engine split as current; the Budget section documents a Python exception replaced by `Answer::Aborted` (verdict.rs:103-117); the closing "When P1.3 work begins, this stub becomes a hub" contradicts the header. The header banner (:13-15) claims only two sections describe removed machinery, which is not true.
- **zebra_walkthrough.md:16-21** routes embedders to the historical Python API contract ("docs/api/ein.md … lands in P1a.9") while README.md:87 of the same tree says those pages are history and the live surface is docs/api/rust.md.

### Evidence

Each item cross-checked against the current implementation at the cited code locations during the reconstruction pass (single `pub fn solve`, empty kb_index by construction, `queries: Vec`, predicates.rs's two predicates, absent `from_dot`/`EIN_RENDER_LEVI` by grep).

### Impact

This is the project's highest-leverage documentation problem because of its own stated method: the tree is the *only* statement of intent that is not also the implementation, docs/history was pruned on the rule "still read as a specification", and a reimplementer or reviewer "starting here" (as CLAUDE.md instructs) gets actively wrong answers about the solve API surface, the dump artifacts, the query model, and the predicate vocabulary. The repo demonstrates its own thesis — every number a test pins is exactly right; every page nothing runs has rotted.

### Recommendation

Triage each page into one of three states the tree already uses: current (fix the content), superseded-with-banner (as parity_baselines.md and README's marked sections already do), or moved to docs/history. algorithm_layer_n.md and the Python sections are banner/move candidates; 02_store.md, 02_patterns.md, README.md and lattice_dump.md need content fixes. Add the missing pages to whatever doc-pass checklist milestones run (S1a.10.6 missed all of these).

---

## M1d landed unevenly across docs/kernel: live pages flatly contradict each other about the shipped Open verdict

**Severity:** High
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug (internal contradiction about current behavior)

**Locations**
- `docs/kernel/ir/03-ein-lang/01_grammar.md:412-414`
- `docs/kernel/ir/03-ein-lang/06_reserved_names.md:153-157, 228-233`
- `docs/kernel/inference/README.md:83`
- `docs/kernel/inference/architecture_and_algorithms.md:200` (vs its own §2)
- `docs/kernel/inference/implementation.md:55, 97`
- versus: `docs/kernel/defined_behaviour.md:360-370`, `docs/kernel/inference/events.md:105-118`, `architecture_and_algorithms.md:115-135`, and code (`verdict.rs:61`, `obligations.rs:145`)

### Finding

01_grammar.md says of the obligation tally "nothing reads the tally … legal and inert"; 06_reserved_names.md says "no verdict word … S1d.2.6 is where the word is decided" and gives `:expect`'s third form as `none` instead of `(false)`; inference/README.md:83 and architecture_and_algorithms.md's §3 verdict row still list **three** verdict words while §2 of the same file lists four; implementation.md's module map has no Open and no tree. Meanwhile defined_behaviour.md, events.md and architecture_and_algorithms.md §2 document the shipped Open verdict and obligation pass, and the code confirms them.

### Impact

The same canonical tree answers "how many verdict words are there" with both three and four, and "who reads the obligation tally" with both nobody and the verdict. A reader cannot tell which page lags without the code.

### Recommendation

One M1d doc pass over the five stale pages; the census documents under docs/history already contain the correct statements to copy from.

---

## defined_behaviour.md §3.2 documents a "preserved bug" a direct probe could not reproduce

**Severity:** High
**Confidence:** Medium (one decisive probe, one plausible alternative trigger untested)
**Topic:** Code-doc consistency
**Classification:** documentation bug — or an unverified normative claim naming the wrong trigger

**Locations**
- `docs/kernel/defined_behaviour.md:223-237`
- `ein.rs/crates/ein-infer/src/firing.rs:219-224, 242-249`; `ein.rs/crates/ein-infer/src/compile.rs:94-101, 440-453`
- `ein.rs/crates/ein-infer/src/saturator.rs:1182-1204` (`refresh_collision_risk` spends conservatism on the same asymmetry)

### Finding

§3.2 — flagged by the page itself as "the one item on this page that is a latent bug rather than a quirk" (Q-M1a.8) — promises that two activators differing only in an integer argument can suppress each other's firings with no diagnostic. A direct probe (two activators `(walk edge 1)` / `(walk edge 2)`) produced **both** firings, because `BindingKey.values` includes the int-seeded register. Either the engine does not have the documented latent bug (the page is false), or the collision exists only in a narrower shape (nested-Fact activator args, which bind nothing and genuinely stay out of both key halves) and every statement of it — "integer argument", "a puzzle whose rule parameters are integers can lose a firing" — names the wrong trigger. No fixture or test anywhere pins the claimed suppression in either direction.

### Impact

The page's entire purpose is to replace the deleted Python source as the statement of behavior; its one self-declared latent bug is unverified and probably mis-stated. Anyone triaging Q-M1a.8 (it is cited in README's Known gaps and the open-questions ledger) starts from a wrong reproduction recipe.

### Recommendation

Write the decisive probe both ways (int args; nested-Fact args) as a test, then either fix the engine claim in §3.2 to the real trigger or delete the item and close Q-M1a.8 as not-a-bug.

### Cross-references

- `review/open-questions.md` Q4.
