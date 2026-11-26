# E-114 — A partial program's H is set by its relations' count of places, not by what it states

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.1, 2026-11-26, on `98125f61`
**Owner:** none; [E-13](../../../evaluation/hypotheses/E-13-a-program-that-names-no-hypothesis-mechanism-is-answered-by.md)
is its whole-program half

## What

**A partial program's H is set by its relations' count of places, not by what it states.** A step's program before
its theory names no mechanism, so the blind enumerator writes H, and it guesses facts of a relation of one or two
places only. The Zebra puzzle's step 1, nineteen statements over nine verbs of two places, has 9 896 candidates (root
saturates to 9 960 facts, the candidates' `open` facts among them) and is not searched; the seating's step 1 has 89,
the youngest's 7; and each word problem's step 1, whose relations have four places or more, has none: R is {∅},
`--single` answers `Solution`, *one*, and the answer query has no row. Under `--no-search`, the judgement at root the
steps take, every one of the eighteen step programs is *not decided*, which says that it loads, is not flagged and
has a live root, and nothing of how far the formalization got.

## Evidence

`einaf/contract/*/step1.ein` and `step2.ein`; `ein show hyps FILE` (the writer and the candidates), `einaf outcome FILE
--single`, `ein saturate FILE` (root's facts); held by `einaf/tests/contract.rs`'s
`each_steps_program_is_judged_as_the_page_says`, whose table is
[the contract's § 7.1](../../../../../docs/einaf/04-contract.md#71-each-steps-program-judged).

## What it costs the application

The judgement between steps cannot use the outcome. A step's program asked a question that searches is either a
search of thousands of candidates nobody stated (the Zebra puzzle's step 1) or a *one* that means nothing (a word
problem's); asked `--no-search`, it is always *not decided*. What tells a step's progress is the answer query's rows
and what derives them ([E-116](../answer/E-116-a-query-with-no-row-does-not-say-which-of-its-premises-matched.md)).
