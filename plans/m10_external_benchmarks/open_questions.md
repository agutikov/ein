# Open Questions — M10 (External benchmarks)

Milestone-scoped questions. Ids are **sticky** — `Q-M10.<n>`, in the style
[M1a](../../docs/history/m1a_rust/open_questions.md) uses for `Q-M1a.<n>` rather than the
global `Q<n>` sequence in [`plans/open_questions.md`](../open_questions.md),
so the namespaces cannot collide. A closed id is never reused.

**All three arrived 2026-08-23 with the promotion**, where they were
`Q-M1c.3`, `.4` and `.5` in
[M1c](../m1c_external_validation/open_questions.md). The text below is theirs,
unchanged apart from ids and paths; M1c's index keeps the old ids as
redirects, because a sticky id that silently disappears is worse than one that
points somewhere.

## Index

| Q | title | status |
|---|---|---|
| [Q-M10.1](#q-m101--what-makes-an-encoding-fair) | What makes a benchmark encoding fair? | open — recommendation: published where one exists, provenance where it does not, and no tuning against the clock *(was Q-M1c.3)* |
| [Q-M10.2](#q-m102--does-a-proof-assistant-belong-in-a-timing-table) | Does a proof assistant belong in a timing table? | open — recommendation: **(b) keep Lean, drop its time column** *(was Q-M1c.4)* |
| [Q-M10.3](#q-m103--where-does-the-benchmark-live-and-is-any-of-it-a-gate) | Where does the benchmark live, and is any of it a gate? | open — recommendation: a crate, and only the answer half runs unattended *(was Q-M1c.5)* |

---

## Q-M10.1 — What makes an encoding fair?

The benchmark's entire validity rests on this and nothing else. Whoever writes
six encodings of the same puzzle knows one of the six systems far better than
the other five, and a clumsy CLP(FD) program is indistinguishable in the table
from a slow Prolog.

- **(a) Published encodings only.** Cite it or drop the cell. Maximum
  credibility, and it fails immediately: there is no published ein-lang
  n-queens, and there never will be.
- **(b) Idiomatic-per-system, written here, with provenance.** Each file
  records who wrote it, from what, and what was changed. Honest, and it puts
  the reader in a position to discount it.
- **(c) An encoding budget.** The same wall-clock effort per system, recorded.
  Sounds fair, measures the author's fluency, and cannot be audited by a
  reader.

**Recommendation: (a) where a published encoding exists, (b) with provenance
where it does not**, plus [S10.1](s10.1_problem_corpus.md)'s
rule 3 — the first working idiomatic version is the one that is timed, and a
later faster one is added rather than substituted.

**The residue is n-queens in ein-lang**, which has no published prior art by
construction and whose `attacks` relation is generated because the kernel has
no arithmetic. The line the stage draws — *the generator may compute the
board, never the solution* — is the part a reviewer should attack first, and
it is written down so that they can.

## Q-M10.2 — Does a proof assistant belong in a timing table?

Lean 4 is in the user's list, and it is not a solver. `decide` /
`native_decide` over a finite domain is brute force through kernel reduction;
a hand-written proof measures the author's afternoon. Either number next to
Z3's is a category error.

- **(a) Drop Lean.** Clean, and loses the one column where something is said
  that no solver can say.
- **(b) Keep Lean, no time column.** Its cell reports the *artefact*: what had
  to be stated, and what was proved.
- **(c) Keep Lean with a time column and a warning.** Warnings do not survive
  being quoted.

**Recommendation: (b).** And the reason is not diplomacy — a Lean development
can prove that the model is the *only* model, which is exactly the guarantee
the word `exhausted` claims in Ein's own verdict
([Q-M1d.1](../m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)).
Having one column in the corpus where that guarantee is machine-checked is
worth more than a number, and it is the corpus's only link to
[M1d](../m1d_satisfiability/README.md)'s subject.

## Q-M10.3 — Where does the benchmark live, and is any of it a gate?

- **(a) `ein.rs/crates/ein-bench` + `bench/` for data**, mirroring
  `ein-conformance` + `conformance/`: a crate that shells out and links
  nothing.
- **(b) A `utils/` script**, like the M1a measurement set (`bench_env.sh`,
  `e2e_baseline.py`, `profile_ein_rs.py`).
- **(c) In-tree tests**, run by `cargo test --workspace`.

**Recommendation: (a) for the code, and (c) for the answer half only.** After
[P1a.10](../../docs/history/m1a_rust/README.md#p1a10--one-implementation) `cargo test
--workspace` is the whole gate and a shell script is where a check goes to
die — but a per-commit gate that depends on six external programs fails for
reasons that have nothing to do with the commit. So: the harness is a crate;
the answer-parity subset runs in `nightly.yml` where the systems are
installable and reports `missing` where they are not; every number with a
clock in it is taken by a person on a quiet machine.

The sub-question the stage still owes: **do the `bench/` corpus files count as
corpus for the completeness check** that fails when an `.ein` file has no
entry? They are `.ein` files under a new directory, and the answer decides
whether adding a benchmark problem also means adding a conformance entry.
