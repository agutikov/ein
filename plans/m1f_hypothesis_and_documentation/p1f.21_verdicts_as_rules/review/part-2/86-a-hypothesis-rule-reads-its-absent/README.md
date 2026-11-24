# 86 · A hypothesis rule reads its `absent` premises: a wrong answer, fixed

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.24_a_hypothesis_rule_reads_its_absent.md#record) ·
[the finding](../../../findings.md#v-89--a-rule-that-concludes-a-hypothesis-reads-none-of-its-absent-premises)

**Built without asking, on a default. This item is where you say it was the wrong reading.**

> **The pictures are files under each part's `runs/`, which git ignores**, embedded as `<img>`. If one is missing,
> `bash tools/run.sh` writes every part's runs and drawings in a second, on the final binary and on the engine just
> before the fix (the release build of `2ed5bfb1`, its engine `7b72bd84`'s, kept as
> `build/review-bin/ein-before-fix-2ed5bfb1`).

## In one paragraph

A rule that concludes a hypothesis, `(open F)`, was matched **without its `absent` premises**. *Suspect anyone
present who has no alibi* suspected Ann, who has one. Since the fix it reads them as every rule does (default 1).
Four smaller choices came with the fix: whether such a read is a site of the static check (default 2, the one with a
consequence); what the oracle's listing of a node shows (default 3); and two of bookkeeping, a decision file
written before your ruling and the witness's `:expect` (defaults 4 and 5).

| part | what | the default |
|---|---|---|
| [**1**](1-the-absent-is-read/README.md) | **the `absent` is read**: the wrong answer and its fix, on a small story and the corpus witness | 1, Q-M1f.685 |
| [**2**](2-a-read-under-absent-flags-nothing/README.md) | **a read under `absent` flags nothing**: above root such a read only takes hypotheses away; a positive read adds one, and is flagged | 2, Q-M1f.686 |
| [**3**](3-the-oracles-listing/README.md) | **the oracle's listing of a node**: its facts beyond root's and every `open` fact it holds | 3, Q-M1f.687 |
| [**4**](4-the-witness-and-its-decision/README.md) | **the witness and its decision**: how the wrong answer was filed, fixed and held | 4 and 5, Q-M1f.688, Q-M1f.689 |

## What was wrong

Running the review items on the final engine found it. A rule that concludes `(open F)` was matched without its
`absent` premises. On the probe of [item 62](../62-what-a-fork-drops/README.md), one match under two rules:

```lisp
(p a) (p b)
(q a)
(rule plain () :match (and (p ?x) (absent (q ?x))) :assert (s ?x)        :why "…")
(rule guess () :match (and (p ?x) (absent (q ?x))) :assert (open (r ?x)) :why "…")
```

| | before | since |
|---|---|---|
| `plain` concludes | `(s b)` | `(s b)` |
| `guess` writes | `(open (r a))` and `(open (r b))` | `(open (r b))` |
| $H$ | 2 candidates | 1 |
| $R$ | 4 states | 2 states |

The program is monotone by the static check, so the engine claimed the four states. `ein oracle` agreed with it:
it builds $H$ by the same function. The engine before the phase did the same, the rule written as an `hrule`. The
runs of both engines are in item 62's `runs/built-before-the-fix/` and `runs/built/`.

## What was built

- **The reading, a default (1, [Q-M1f.685](../../../../open_questions.md#q-m1f685--a-rule-that-concludes-a-hypothesis-reads-its-absent-premises))**:
  the `absent` is read, judged against the state as a rule's that concludes a fact. It is what the syntax
  reference says of a hypothesis rule's `:match`, and what your ruling makes of a hypothesis rule, a rule that
  concludes `(open F)` and nothing apart (Q-M1f.567). The other reading: a hypothesis rule's `absent` is no part of
  what it proposes, and the pages say so.
- **By the rules of the wrong answers**: filed as a row of the register in a commit of its own, fixed, and the row
  moved to the regressions with its witness in the corpus, `examples/ein-bugs/wrong-answers/hypothesis-rule-absent.ein`.
- **A test that the oracle cannot be**: `ein-infer/tests/hypothesis_rule_absent.rs` holds $H$, and each node's
  `open` facts, to the facts a rule that concludes a fact of the same match concludes, on the witness and six
  shapes of match. It fails on the function as it was.
- **$H$ and $R$ moved on no corpus program**, and none changed flag: each of the 931 was run before and after. Two
  programs' states hold fewer `open` facts, both witnesses of a hypothesis rule that reads an `absent`.

## The defaults

| # | question | what was asked | the default taken | the other reading |
|---|---|---|---|---|
| 1 | [Q-M1f.685](../../../../open_questions.md#q-m1f685--a-rule-that-concludes-a-hypothesis-reads-its-absent-premises) | whether a rule that concludes a hypothesis reads its `absent` premises | it does, as every rule | it does not: its `absent` is no part of what it proposes |
| 2 | [Q-M1f.686](../../../../open_questions.md#q-m1f686--whether-a-hypothesis-rules-read-under-one-absent-is-a-site-of-the-static-check) | is a site owed for a hypothesis rule that reads, under an odd number of `absent`s, a relation a commitment can change | no: such a read takes a hypothesis away above root and adds none, so nothing is first held above root (Q-M1f.574), and R, over root's H, reads nothing a state above root proposes; the check is left as it is | a site, `hypothesis` at every depth: a program whose states above root propose other than root did is flagged, and nothing is claimed of its answer |
| 3 | [Q-M1f.687](../../../../open_questions.md#q-m1f687--what-the-oracles-listing-of-a-node-shows) | what the oracle's listing of a node shows, and `-P` beside it | `--states` lists the node's facts beyond root's and every `open` fact it holds, root's node included; `-P` keeps leaving them out of the states of R it lists, and no positive form of `--states` is added | the listing keeps to what is beyond root, an `open` fact listed only where root does not hold it; or `-P` beside `--states` drops the `open` and `(not …)` facts from every node's listing |
| 4 | [Q-M1f.688](../../../../open_questions.md#q-m1f688--whether-a-defaults-decision-file-is-written-before-the-user-rules-it) | whether Q-M1f.685, a default, gets a decision file now | yes, in the fixing commit, as Q-M1f.590, a default, has one: the regression's `decisions`, a code comment and the pages cite it, and a decision taken by default and one ruled are alike (the decisions' README) | it stays in `open_questions.md` until the user rules it, and the project cites Q-M1f.129 and Q-M1f.567 alone |
| 5 | [Q-M1f.689](../../../../open_questions.md#q-m1f689--what-the-moved-witness-of-the-hypothesis-rules-absent-states) | what the moved witness states | a `(solve :expect (region (model) (model (r b))))`, the row's answer, with a `test` run; none while the row was open, so the register's `solve` run exited 0 | no `:expect`, the row alone holding the answer |

**Default 2 is the one with a consequence**: a hypothesis rule that reads, under one `absent`, a relation a
commitment can change is **not** flagged. Such a read takes a hypothesis away at a state above root and adds none,
so nothing is first held above root, and $R$ is over root's $H$. The states above root then hold fewer of root's
hypotheses among their `open` facts.

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 86. Blank keeps the five as built.
