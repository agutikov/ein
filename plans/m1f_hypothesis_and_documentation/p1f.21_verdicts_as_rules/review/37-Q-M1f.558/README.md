# 37 · Q-M1f.558: *forbid*, once the contradiction is rules

[← the topic](../README.md) · [← the phase](../../README.md) ·
[the ledger](../../../open_questions.md#q-m1f558--forbid-and-the-contradiction-as-rules) ·
[V-1](../../findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none)

**Part 1. Not ruled; a default is proposed.** [S1f.21.11](../../s1f.21.11_kill_and_three_conventions.md)'s task
2.1 runs on it.

**The question.** *Forbid* refuses, at load, a program whose refutation could rest on an `(absent …)` over a
relation a commitment can still extend. It reads **rules**. The kernel's own contradiction, a fact beside its
negation, is no rule, so *forbid* does not read it. Once the contradiction is the standard library's rules,
*forbid* reads them, and programs that load today are refused.

## What today's engine shows

The gate build of master at `ca98dec2`, on the two programs of [`programs/`](programs/); `tools/run.sh` re-takes
them into `runs/` (both ignored). A rule serves soup to a guest not known to be allergic; `allergic` is what a
commitment may add.

| program | `ein solve` |
|---|---|
| [`soup-no-rules.ein`](programs/soup-no-rules.ein), the kernel's check alone | loads: 4 states, `lattice not monotone: 1 site (the static check)`, the site `soup-unless-allergic` |
| [`soup.ein`](programs/soup.ein), with `(clash-of-declared)` | **refused**: *rule 'std.contradiction.clash-0' [activator meal]: concludes (false) from meal, which rule 'soup-unless-allergic' derives resting on (absent …) over allergic, a relation a commitment can still extend*, and the same for `clash-2` |

Over the corpus, with the rules on for every declared relation, seven programs that load today are refused, the
same seven by the survey's sweep and by S1f.21.8's: `examples/features/07_unstratifiable.ein`,
`examples/lattice/08_absent_witness_hole.ein`, `10_fifo_gap.ein`, and under `examples/ein-bugs/wrong-answers/`
`f07-fpoff`, `f07-rules-swapped`, `kw-neq-in-absent` and its twin. Each is a program the static check already
flags: one the engine answers and claims nothing of.

## The options

- **(a) *Forbid* reads the module's contradiction as it reads the kernel's today**: not as a refutation that rests
  on an absence. The seven load and are answered as now, flagged. **Proposed default**: it changes no program, and
  what *forbid* should read of a flagged program is M1n's question, not this phase's.
- **(b) *Forbid* reads the rules as written.** The seven are refused. It is the stricter and the simpler reading:
  no rule is special. Four of the seven are the witnesses of three open rows of the wrong-answer register (`W8/features-07`,
  `W8/f07-rules-swapped`, `W8/fifo-gap`), which would then be refusals and no longer answers.
- **(c) Left until M1n rules what *forbid* covers**, and the contradiction stays the kernel's until then: S1f.21.11's
  task 2.1 waits.
