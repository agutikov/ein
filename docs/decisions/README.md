# Decisions

Every question about Ein that has an answer, as that answer: what holds today, why, and what holds it. Together they
are the current state of the project's choices, the ones the code, the tests and the pages carry out. A decision is
the final answer alone: the options a question weighed and the answers it had before are not kept
([Q-M1f.342](docs/Q-M1f.342-a-decision-is-a-question-with-its-final-answer.md)), and the decisions are about Ein,
not about how the work on it is done ([Q-M1f.346](docs/Q-M1f.346-the-decisions-are-about-ein-not-about-how.md)).

**Every decision may be reopened.** An answer taken by default and one ruled are alike. A reopened decision is a
question again until it has an answer, and then its file states the new one under the same id.

## A decision's file

One file a decision, `<subject>/<id>-<slug>.md`
([Q-M1f.344](docs/Q-M1f.344-each-decision-is-one-file-in-a-folder-by-subject.md)). The id is the decision's name,
kept from the question it answers, `Q-M1e2.24`, `Q-M1f.313`, `Q23`; it points at nothing else. The slug is the
title in lower case, an apostrophe dropped, every other run of characters outside `[a-z0-9]` one `-`, cut at the last
whole word within 48 characters, a trailing connective dropped. The file reads:

```markdown
# <id> — <the answer, as a sentence>

What holds today.

**Why.** The reason.

**Held by.** The tests, the code and the pages of the project that hold it.
```

Each folder's `README.md` lists its decisions, one line each in id order: the id, linked to its file, then the
title.

## Citing one

A page cites a decision as a link to its file. A code comment, a doc comment, an `.ein` comment and a note of the
corpus manifest cite it by its bare id, which names exactly one file here
([Q-M1f.343](docs/Q-M1f.343-a-page-or-a-comment-cites-a-decision-by-its-id.md)). A question the project cites is
one with an answer: where Ein postpones something, the decision says what it does today and why
([Q-M1f.347](docs/Q-M1f.347-no-open-question-is-cited-by-the-project.md)).

## The subjects

| folder | what its decisions are about |
|---|---|
| [`language/`](language/README.md) | the language: grammar, names, values, strings, keywords, rules, macros, and what the loader refuses |
| [`imports/`](imports/README.md) | modules and imports: full and short names, and what an import brings |
| [`diagnostics/`](diagnostics/README.md) | the line a refusal prints: its place, its wording, its note |
| [`answer/`](answer/README.md) | the region R, a node's status, cores, claims, and the questions about R |
| [`hypotheses/`](hypotheses/README.md) | hrules, obligations, the generation ladder, the hypothesis mechanisms, and the structure of H |
| [`read-out/`](read-out/README.md) | what `ein solve` and `ein test` print: rows, words, JSON, state files, the trace |
| [`cli/`](cli/README.md) | the commands, their options and exit codes, and the views |
| [`engine/`](engine/README.md) | `ein.rs` inside: the parser, the compiler's invariants, the agenda, the `.einb` format, determinism |
| [`stdlib/`](stdlib/README.md) | the standard library's layout, and its modules outside `std.math` and `std.bool` |
| [`stdlib/math/`](stdlib/math/README.md) | the calculator's shared parts: the tables, `std.math.positional`, `std.math.arith`, `std.math.convert`, the tagged calls |
| [`stdlib/math/decimal/`](stdlib/math/decimal/README.md) | `std.math.decimal` and `std.math.decimal-arith` |
| [`stdlib/math/int/`](stdlib/math/int/README.md) | `std.math.int` |
| [`stdlib/math/rat/`](stdlib/math/rat/README.md) | `std.math.rat` |
| [`stdlib/math/gcd/`](stdlib/math/gcd/README.md) | `std.math.gcd` |
| [`stdlib/math/binary/`](stdlib/math/binary/README.md) | `std.math.binary` and `std.math.binary-arith` |
| [`stdlib/math/num9/`](stdlib/math/num9/README.md) | `std.math.num9` |
| [`stdlib/bool/`](stdlib/bool/README.md) | `std.bool.*`: the sixteen functions, their properties, circuits |
| [`corpus/`](corpus/README.md) | the corpus and its tests: the manifest, fixtures, examples, the test harnesses, regression tests |
| [`docs/`](docs/README.md) | the documentation: what a page may state and cite, how a claim is warranted, how the pages are named and what each is written in |
