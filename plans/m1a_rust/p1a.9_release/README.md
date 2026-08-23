# P1a.9 — Release

**Milestone:** [M1a — Rust port](../README.md)
**Status:** ✅ **shipped 2026-08-23** — three stages, and **all three found
something the plan had assumed**. S1a.9.0 found twelve of seventeen `slow`
entries were never slow; S1a.9.3 found two of three feature flags gating
nothing and one that does not exist; S1a.9.4 found two published measurements
that were wrong rather than stale. That is not a run of luck — it is what
happens when a release phase is made to *re-measure* the claims it is about to
ship instead of collecting them.
**Estimate:** 1.2 weeks (6 days of stages)
**Depends on:** nothing outstanding.
[P1a.10](../p1a.10_single_implementation/README.md) **shipped 2026-08-21**
and this phase closes the milestone behind it.

> **Scope change 2026-08-21 — the bindings are deferred and the phase is
> re-topiced.** This was **Bindings and release** (4 stages, 8 days): a PyO3
> module, an API contract suite over it, packaging, documentation. The two
> binding stages — S1a.9.1 (the PyO3 surface) and S1a.9.2 (API parity tests) —
> are **cut**, and the stage numbers are left as a gap rather than reused, so
> a link that meant "the PyO3 stage" does not silently come to mean something
> else. Both files are in git history.
>
> **Why.** The phase justified PyO3 with one consumer, and the census does not
> support it. [M20](../../m20_gui/README.md) links the crates into a Tauri
> backend; [M1c](../../m1c_external_validation/README.md)'s benchmark runner is
> `ein-bench`, Rust, and
> [S10.2](../../m10_external_benchmarks/s10.2_systems_and_install.md)
> requires that it *shell out* — "`cargo build` never needs Z3" — because a
> linked rival and a subprocess rival are not comparable measurements;
> `utils/`'s seventeen scripts drive the binary by `$EIN_BIN` precisely so a
> number says which build produced it. That leaves
> [M2](../../m2_nl_to_ir/README.md), and M2's reason does not survive contact
> with M2's own plan:
>
> - **The llama.cpp argument was never true.**
>   [P2.2](../../m2_nl_to_ir/p2.2_llm_infra/README.md) is a `llama-server`
>   container and a thin HTTP client — no llama.cpp bindings anywhere. The
>   pattern it mirrors is [acva](../../../../acva/), and acva's client is
>   **C++**; P2.2's own README says "same pattern, Python client *this time*
>   instead of C++". The *this time* was because ein was Python, and
>   [S1a.10.5](../p1a.10_single_implementation/s1a.10.5_removal.md) ended that.
> - **The one thing that wanted a binding is the thing that argues against
>   it.** M2's validator/repair loop
>   ([S2.4.2](../../m2_nl_to_ir/p2.4_nl_to_ir_pipeline/s2.4.2_validator_reprompt.md))
>   is written as `validate(facts: list[IRNode], ontology: IRNode)` and needs
>   *why* a load failed, not the message text — a structured-diagnostics
>   surface the engine does not have and the CLI cannot grow, because
>   [`defined_behaviour.md` §1/§4](../../../docs/kernel/defined_behaviour.md)
>   pins those diagnostics as **strings**. A Rust frontend links `ein-ir` and
>   `ein-infer` and has all of it with no boundary, no mirrored data model and
>   no exception hierarchy to design. The strongest argument for building the
>   binding is the strongest argument for not needing it.
>
> **What replaces the two stages** is one that was never in the plan and is
> the milestone's own reflex — measure before building
> ([S1a.6.1](../p1a.6_performance/s1a.6.1_profile_baseline.md),
> [S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md)):
> [S1a.9.0](s1a.9.0_slow_corpus.md), the corpus's slow tail, whose seventeen
> entries are still priced against an engine that left the tree.
>
> **This is a deferral with a trip-wire, not a cancellation** —
> [Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
> records the three conditions that would bring it back, and the likeliest one
> ([M5](../../m5_presentation/README.md): a paper whose artifact reviewers
> expect `pip install`) is a milestone away, not a phase.

## Goal

Ship it. Static binaries for three platforms, a version string that says
exactly what is in them, a release process that cannot publish a red gate, and
the documentation that makes ein.rs the engine of record — by now the only
one. And, first, the measurement the phase would otherwise ship without: what
the corpus's seventeen `slow = true` entries actually cost, on the engine that
is being released.

> **That measurement is taken** —
> [`corpus_cost.md`](corpus_cost.md), 2026-08-22. **Twelve of the seventeen
> were never slow** by any threshold worth stating — the flagship `zebra2.ein`
> among them, at 16 ms — two more stopped being slow when a run that asked the
> fixture nothing was dropped, and the runs excluded for "outliving a 150 s
> budget under CPython" turn out to end in the **OOM killer** on any engine.
> What the release gate now sweeps is 641 cells in 19.4 s, where it was 660 in
> 242.6 s.
> [Q-M1d.6](../../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
> is what the stage found and did not fix.
>
> **And the second one is taken** — [`feature_cost.md`](feature_cost.md),
> 2026-08-23. S1a.9.3's `--no-default-features` task was written as a
> compile check; run as a measurement it found that two of the three features
> that build was documented to drop were not being dropped, one of them
> because it does not exist. The phase's reflex held twice: **measure the
> thing the plan assumed.**

## Stages

| stage | title | est. | |
|---|---|---|---|
| [S1a.9.0](s1a.9.0_slow_corpus.md) | The slow corpus, re-priced | 3 d | ✅ **shipped 2026-08-22** — 17 slow entries → 3, and the nightly tier from four minutes to 19 s |
| [S1a.9.3](s1a.9.3_packaging.md) | Packaging and release | 2 d | ✅ **shipped 2026-08-23** — `ein --version`, a six-job release workflow, and two feature flags that were not gating what they said |
| [S1a.9.4](s1a.9.4_documentation.md) | Documentation | 1 d | ✅ **shipped 2026-08-23** — `docs/api/`'s subject changed to the crates, with a worked example the gate runs; two published measurements corrected; the milestone closed |

**S1a.9.1 and S1a.9.2 do not exist.** See the scope change above; the gap is
deliberate.

## Acceptance for the phase

- ✅ The seventeen slow entries carry a **measured** cost against ein.rs, every
  `slow` flag and every `no solve -e` exclusion is either re-justified on that
  measurement or removed, and no surviving note explains a cost by naming
  CPython ([S1a.9.0](s1a.9.0_slow_corpus.md), record:
  [`corpus_cost.md`](corpus_cost.md)). Three entries are slow, `cost_ms` is
  what says so, and two tests hold the flag to it.
- **Written, unrun** — `ein` binaries for Linux (x86_64 + aarch64), macOS
  (universal2) and Windows (x86_64), each sweeping the corpus on its own
  platform. **The matrix is written and reviewed; the first `v*` tag is what
  runs it** — this
  repository has never been built for any of those three platforms, and a
  cross-build without a runner proves the linker was willing rather than that
  the corpus sweeps
  ([S1a.9.3 § What CI has not yet proved](s1a.9.3_packaging.md#what-ci-has-not-yet-proved)).
  What the stage *could* do from here it did: `.gitattributes`, so Windows
  fails for Windows reasons; a T3 sweep before every upload, so a failure
  names a cell; a musl leg that cannot block a release.
- ✅ `ein --version` reports engine version, protocol version, feature flags
  and the stdlib manifest hash — the last of those as SHA-256 of the manifest
  **as resolved**, naming which of the three resolution steps answered, so an
  installed binary and a checkout can be told apart in one line. Ten tests.
- ✅ Release artefacts carry checksums — one per leg plus a set-wide
  `SHA256SUMS` cross-checked against them — and `publish` `needs:` the gate,
  the `--jobs` cross-diff and the dependency-light build, so a red gate cannot
  ship a binary.
- ✅ A `--no-default-features` build still compiles and passes the unit suite
  (610 of 619) — **and now drops what it claimed to.** It did not:
  [`feature_cost.md`](feature_cost.md) found `rayon` linked into the
  dependency-light binary, an `events` feature that never existed, and a
  documented allocator cost four days stale (15.9 % → +25.2 %). The gating is
  asserted as a dependency-graph check in both directions, because compiling
  without a feature was never the claim.
- ✅ `docs/api/` describes a surface that **exists** — the Rust embedding one,
  [`rust.md`](../../../docs/api/rust.md) — and the Python contract it used to
  specify is filed as history rather than left as a promise
  ([S1a.9.4](s1a.9.4_documentation.md)). The page's worked example is the
  marked region of `ein-cli/tests/embedding.rs`, diffed against the page by a
  test in that file: the substitute for the deferred S1a.9.2's contract suite,
  and stronger on the one axis that matters — it cannot rot without the gate
  going red. Which is exactly how the five Python pages went stale: they were
  verified once, with a date on it.
- ✅ `plans/README.md`'s status table records M1a as **shipped 2026-08-23**,
  with its date and its measured outcome; `m1a_rust/README.md` carries the
  closing paragraph, `divergences.md` its final state and `open_questions.md`
  each question's resolution — including the two that stay open on purpose.

## Notes

- **`docs/api/`'s subject changes, and it is no longer the PyO3 module.** Six
  pages and 1 051 lines specify a Python embedding API whose implementation
  this phase was going to build. With the binding deferred, the honest move is
  the one [S1a.9.4](s1a.9.4_documentation.md) T1a.9.4.3 already half-wanted: a
  **Rust** embedding page, which is the surface [M20](../../m20_gui/README.md)
  binds against and which nothing documents today, with the Python pages moved
  to history beside
  [`defined_behaviour.md`](../../../docs/kernel/defined_behaviour.md)'s
  treatment of the engine that used to implement them. They are not deleted:
  if [Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  trips, they are the specification again, unchanged.
- **The ex-S1a.9.5 note is folded into
  [S1a.9.4](s1a.9.4_documentation.md).** It read *"forget about removed
  ein.py — find all occurrences, analyze, if it is a reference to removed
  ein.py then reword for ein.rs or delete"*, and S1a.9.4's acceptance already
  carries it as the standing requirement it is: `git grep -i 'ein\.py'`
  returns only history. A stage whose whole content is another stage's
  acceptance item is a duplicate, not a stage.
- **Do not install the `ein` binary onto `$PATH` in developer setups by
  default.** `utils/` names its binary (`$EIN_BIN` / `--bin`) so that a
  measurement says which build it measured, and an ambiguous `ein` on `$PATH`
  has burned every project that allowed it.
- **The Python *package* name is free** and stays free. Nothing in this phase
  claims a name on PyPI; a name claimed before there is something to publish
  under it is a name that has to be renamed.

## Cross-links

- [design/12 — Toolchain](../design/12_toolchain_and_layout.md) — §1's
  reserved `ein-py` crate, §2's `pyo3`/`maturin` row and §3's `python`
  feature, all now marked deferred rather than upcoming
- [Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  — the trip-wire
- [M1d](../../m1d_satisfiability/README.md) — where
  [S1a.9.0](s1a.9.0_slow_corpus.md)'s findings go if they are about the search
  rather than about the manifest
- [M2 — NL → IR](../../m2_nl_to_ir/README.md) — the frontend-language question
  this phase hands it
