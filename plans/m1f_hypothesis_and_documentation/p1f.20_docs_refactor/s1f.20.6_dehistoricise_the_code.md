# S1f.20.6 — De-historicise the code, and dissolve `docs/history/`

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 5 days.
**Status:** not started. It was `T1f.5.20.6` until 2026-11-08.
**Depends on:** [S1f.20.3](s1f.20.3_user.md), [S1f.20.4](s1f.20.4_reasoning.md) and
[S1f.20.5](s1f.20.5_ein_rs.md): a code comment points at the `docs/ein/` page that states its behaviour, so the
page must exist first.

## What it does

The largest mechanical piece. On 2026-08-28 it was **963 id-bearing lines in 175 `.rs` files**, plus 303 in
`.ein` headers and 47 in `corpus.toml`. [S1f.20.1](s1f.20.1_the_two_checks.md)'s census gives today's count.

Four substitutions, in decreasing frequency:

1. **`(M1d P1d.2 S1d.2.4)` → a link to the `docs/ein/` page that states the behaviour.** This is the user's
   own example, and the common case.
2. **A link into `docs/history/m1a_rust/design/NN` → the same file's new home under
   `docs/ein/ein.rs/design/`.** It is mechanical, and the link check verifies it. S1f.20.3 to S1f.20.5 have
   done this already for every page they moved ([P1f.20 § Two rules](README.md#two-rules-across-the-stages)).
3. **A sentence whose *subject* is the development event**, where the id carries the paragraph's meaning:
   *"Empty before S1a.6.9 and not because there was nothing to say"*. These do not substitute. They are
   **rewritten as statements about the code** (*"A fork that resumes root's saturation does not re-derive
   root's closure, so the givens are told first"*), and the history is dropped. There are on the order of a
   hundred, and they are the only slow ones.
4. **A reference to a question.** Since the user's rulings of 2026-11-15 a decision may be cited: a question
   with an answer, ruled or a standing default, is a file under `docs/decisions/`
   ([S1f.20.8](s1f.20.8_decisions.md)), and the comment cites its id, linked. A question still open is a work
   item: it stays in `plans/`, and the code comment states the *behaviour* and its uncertainty, without the id.
5. **A date that dates a development event** (*"since 2026-11-03"*): deleted, the sentence restated as what
   is true now (the same rulings).

`AGENTS.md` and `README.md` are the last pair touched. They are exempt from the release-voice check, but not
from the link check, so every link they hold into `docs/history/` is rewritten.

**The dissolve of `docs/history/` is [S1f.20.9](s1f.20.9_the_history_sweep.md)'s** since 2026-11-15, with the
finished plans, once every decision is a file: this stage rewrites the references, that one deletes.

## The method

### Task T1f.20.6.1 — the crates

Substitutions 1, 3 and 4 over every shipped crate's doc comments and the tests. The release-voice baseline is
lowered as each file reaches 0.

### Task T1f.20.6.2 — `.ein` headers and `corpus.toml`

The same, over the corpus.

### Task T1f.20.6.3 — `AGENTS.md` and `README.md`

The pair's links rewritten. The deletions and the empty baseline are [S1f.20.9](s1f.20.9_the_history_sweep.md)'s.

## Acceptance

- **0 history-referencing lines outside `plans/`**, by S1f.20.1's census, but for those S1f.20.9's deletions
  resolve; a decision id is not one.
- No reference into `docs/history/` is left for S1f.20.9 to break: not in a `.md`, a Rust doc comment, a `.ein`
  comment, `corpus.toml` or `AGENTS.md`.
- `RUSTDOCFLAGS="-D warnings" cargo doc` is green: 464 doc comments change.
- `./run_tests.sh` is green.
