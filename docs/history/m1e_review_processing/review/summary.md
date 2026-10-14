# Review summary — ein (2026-08-27, master @ 9aa598a)

## System model

Ein is a graph-native reasoner for Zebra-style logic puzzles. A problem is a flat S-expression program (ein-lang): typed relations, facts, rules/hrules, obligations, macros, imports, config, and one or more `(query …)` blocks that may state their own answer (`:expect`). The engine loads it into a layered copy-on-write KB over globally interned terms, **saturates** rules to a least fixpoint (semi-naive, purely positive closure with negation-as-failure judged only at a closure/world boundary, one admission per round), then **searches a commitment lattice** (size-k hypothesis subsets by Apriori prefix-join, pruned by learned no-goods, downward closure and lookahead), and reads one of four verdict words off the result: **Solution** (k=1), **Ambiguity** (k>1), **Contradiction** (k=0, with a provenance-derived unsat core), **Open** (consistent, quiescent, but a program-stated obligation is unwitnessed — scoped to programs that declare obligations). Since M1d, hypothesis generation is a ladder — puzzle `hrule` → facts that would discharge stated obligations → blind enumerator — and an experimental second traversal (`EIN_TRAVERSAL=tree`) branches on one owed instance's jointly-exhaustive alternatives.

## Architecture

Eight crates in `ein.rs/`: `ein-core` (interning, Value/FactId, layered COW KB, provenance arena, program registries) → `ein-ir` (lex → parse → macros → imports → load; embedded stdlib with `$EIN_STDLIB` → checkout → embedded resolution) → `ein-infer` (compile → register matcher → saturator/NAF boundary → lattice/tree search → verdict, obligations, `:expect` evaluation) — then forking: `ein-einb` (binary KB cache; the one crate permitted `unsafe`, in one audited module) and `ein-render` (DOT views, markdown trace, dumps, CPython-shaped JSON) as siblings, `ein-cli` (five subcommands: solve/saturate/render/test/kb) on both. `ein-corpus` and `ein-parity` are dev-only. The verification surface: a 197-entry corpus manifest with completeness checks, 56 expectation-carrying stdlib conformance programs, golden files (EIN_BLESS), invariance sweeps (id-space permutation, `--jobs`, shuffle), and a gate (`run_tests.sh`) of five static checks + `cargo test --workspace` + a bench smoke, mirrored by CI.

## Core invariants (verified as stated-and-enforced during reconstruction)

- Interning is not belief; belief is per-KB bitsets — what makes fork O(1).
- Layered view ≡ rebuild; sealed layers immutable; `branch()` asserts a sealed top.
- No id-assignment order reaches an observable (no `Ord` on Symbol/Value; rank tables; content-based candidate order; a CI lint against hash-map iteration; id-permutation and `--jobs` sweeps).
- Closure purely positive; `(absent …)` judged once at the boundary against a positive fixpoint; one admission per round (a soundness requirement, tested).
- `try_commitment_set` is pure w.r.t. root; only no-goods, `(not h)` writebacks and forced-positive promotion touch root mid-search.
- Node identity is the sorted canonical fact list, never a hash; verdict is read from the result in exactly one constructor (`finalise`).
- Fork provenance dies with the fork (bit-31 region, stale-id panic); promotion copies cited records deterministically.
- `:expect` claims are validated at load and settled only on exhausted searches (`NOT CHECKED` takes a failing exit).

Two important invariants are **stated but unenforced**: the M1 alive-set invariant licensing state-key dedup (`review/state-model/medium.md`), and the reserved-name guard across one of three declaration routes (`review/correctness/high.md`).

## Evidence base and method

- Full gate run on this machine: **exit 0 — 738 tests, 0 failures**, five static checks clean, bench smoke green.
- A seven-reader reconstruction pass over all crates, docs, tests, stdlib, corpus and tooling (~2.2M tokens of reading), producing the system map above, ~180 claim citations and 80 suspicions; several suspicions were **verified against the release binary** during that pass (the `(eq ?x)` panic, the reserved-name import bypass, the Q-M1a.8 non-reproduction, `ein ir parse` absence, version/manifest checks).
- A second stage — 13 per-dimension deep finders plus adversarial verification of every finding — was launched and **aborted by an external session limit before returning results**. Findings below therefore carry the reading pass's confidence labels; three are binary-verified reproductions. Coverage holes from the aborted stage are recorded honestly in `open-questions.md` Q9 (algorithms/pathology analysis, the `cast.rs` unsafe audit, fuzz-style probing, micro-CSP ground-truth verdict checks).

## Overall assessment

The engine's default path — load, saturate, NAF boundary, lattice search, verdict — is in unusually strong shape: the invariants that matter are written down, most are mechanically enforced, determinism is designed-in rather than hoped-for, and the test suite tests semantics (invariance sweeps, negative worlds, accounting identities) rather than implementation details. The gate is green and honest.

The defects cluster in three places. **(1) The newest surface**: the M1d tree traversal ships with a wrong-answer-shaped defect cluster (ignores the stop policy, produces Contradiction with an empty unsat core, rests on an unenforced exhaustiveness premise). **(2) Validation corners**: a well-formed program can panic the process (`(eq ?x)`), and the reserved-name guard is bypassable through one import tier because a semantic list exists as two drifted hand-copies — a mechanism (parallel hand-maintained copies) that recurs four times in the tree and has now bitten twice. **(3) Documentation debt with unusual leverage**: the canonical `docs/kernel` tree — which the project's method makes the *only* statement of intent that is not also the implementation — presents removed or never-built machinery as current across at least six pages, contradicts itself about the shipped Open verdict, and its one self-declared latent bug (Q-M1a.8) fails to reproduce. Meanwhile every prose count not pinned by a test has drifted, exactly as the project's own thesis predicts.

## Findings by severity

| Severity | Count |
|---|---:|
| Critical | 0 |
| High | 6 |
| Medium | 36 |
| Low | 21 |
| **Total** | **63** |

## Most consequential findings

1. **`(eq ?x)` panics the process at match time** from well-formed input — `correctness/high.md` (binary-verified).
2. **Reserved-name guard bypassed via import qualification** (`(macro open …)` silently renamed; two drifted RESERVED lists) — `correctness/high.md` (binary-verified).
3. **Tree-traversal cluster**: `-n`/`-m` ignored; dead branches learn/record nothing → Contradiction with empty unsat core; root-only rung probe with an unenforced exhaustiveness premise — `correctness/high.md`.
4. **docs/kernel presents removed/never-built machinery as current** across ≥6 pages of the tree the project declares canonical — `code-doc-consistency/high.md`.
5. **M1d landed unevenly**: canonical pages contradict each other on whether the Open verdict exists — `code-doc-consistency/high.md`.
6. **defined_behaviour.md §3.2's "preserved bug" does not reproduce** — the normative behavior page's one self-declared latent bug is unverified and likely mis-stated — `code-doc-consistency/high.md`.
7. **The alive-set invariant — the soundness warrant for state-key dedup and model counting — is enforced nowhere** — `state-model/medium.md`.
8. **Gate blind spots**: the zebra2-variant byte check silently skips without python3; non-vacuity floors drifted to ~half the corpus; gate=CI held by convention that already failed once — `tests/medium.md`.

## Report index

- `summary.md` — this file
- `correctness/high.md` (3) · `correctness/medium.md` (6) · `correctness/low.md` (1)
- `error-handling/medium.md` (2) · `error-handling/low.md` (2)
- `semantics/medium.md` (3) · `semantics/low.md` (2)
- `state-model/medium.md` (1) · `state-model/low.md` (1)
- `architecture/medium.md` (2)
- `tests/medium.md` (8) · `tests/low.md` (5)
- `code-doc-consistency/high.md` (3) · `code-doc-consistency/medium.md` (8) · `code-doc-consistency/low.md` (3)
- `documentation/medium.md` (2) · `documentation/low.md` (2)
- `maintainability/medium.md` (4) · `maintainability/low.md` (5)
- `open-questions.md` (10)
