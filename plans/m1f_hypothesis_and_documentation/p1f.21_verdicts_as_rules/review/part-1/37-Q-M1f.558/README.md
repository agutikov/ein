# 37 · Q-M1f.558: *forbid*, once the contradiction is rules

[← the topic](../README.md) · [← the phase](../../../README.md) ·
[the ledger](../../../../open_questions.md#q-m1f558--forbid-and-the-contradiction-as-rules) ·
[V-1](../../../findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none)

**Part 1. Not ruled; a default is proposed.** [S1f.21.11](../../../s1f.21.11_kill_and_three_conventions.md)'s task
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

## What is already decided of non-monotone programs

Your question, 2026-11-19: *"What we have decided on the non-monotone programs so far?"* All seven programs are of
that kind, so this is what the item stands on.

**Decided, and standing:**

| what | where |
|---|---|
| **The policy: detect, document, warn, decide nothing.** You, 2026-11-02 and again 2026-11-16: *"non-monotone programs still out of scope, we know about them, can detect them, can solve them somehow, but don't claim about solution soundness, correctness or completeness or anything."* An engine change need not keep anything of a flagged program but what the cube computes, and none is argued or blocked on a flagged program's answer | `AGENTS.md` § The language; [M1n](../../../../../m1n_non_monotone_programs/README.md)'s head |
| **Detection.** The static check flags a program where an `absent` reads a relation a commitment can still extend; the read-out says `lattice not monotone: N sites`, names each site and the stale facts, and every route gives up its licences and classifies the whole cube | [monotonicity](../../../../../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md) |
| **A node's state is a fork of root's fixpoint.** On a flagged program that is an assumption, printed in the read-out's `assumes` row, and nothing is claimed of the program's solutions | [Q-M1e2.2](../../../../../../docs/decisions/hypotheses/Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md) |
| **One refusal, *forbid*.** A refutation, a rule that states the key or concludes `(false)` or a stored negation, may not rest on an `absent` over a relation a commitment can still extend: directly, through a fact derived under such an `absent`, or through its activator. Refused at load. An `absent` that concludes a **positive** fact is not refused: it loads, flagged | [Q-M1e2.5](../../../../../../docs/decisions/language/Q-M1e2.5-a-refutation-may-not-rest-on-an-absent.md) |
| **One answer.** Every command gives the same $R$ of a flagged program, and no option changes it | `AGENTS.md` § The answer |

**Not decided, and parked in M1n**, which is not started: whether *forbid* is extended to every `absent`, the
positive ones too (declined for the build, open); what a node's state is, and whether a stale fact disqualifies it;
path dependence; what is reported when $R$ has holes; refusing, qualifying or searching differently; learning from
a refutation that rests on an `absent`; the search past the oracle's limit. With them the five open rows of the
wrong-answer register (`W8`), and from this phase V-8 and V-21.

**What that gives here.** Each of the seven derives a positive fact under an `absent`, which loads, and then holds
it beside its negation. Today nothing refuses that, for one reason only: the kernel's own contradiction is no rule,
so *forbid* does not read it. That is a gap in what Q-M1e2.5 says, not a decision. Once the contradiction is a rule
that concludes `(false)`, these are *a refutation resting, through a derived fact, on an absence a commitment can
fill*, the very shape that decision refuses. So **(b) is what the standing decisions give with nothing added**, and
(a), which I first proposed because it changes no program, argues from the answers of flagged programs, which your
ruling of 2026-11-16 says not to do, and needs *forbid* to treat one module's rules as special.


- **(a) *Forbid* reads the module's contradiction as it reads the kernel's today**: not as a refutation that rests
  on an absence. The seven load and are answered as now, flagged. First proposed, because it changes no program;
  **withdrawn**: that reason is one the standing ruling sets aside, and it makes one module's rules special.
- **(b) *Forbid* reads the rules as written.** The seven are refused wherever the contradiction is on for the
  relation. It is what Q-M1e2.5 says already; no rule is special. **Proposed now.** Four of the seven are the witnesses of three open rows of the wrong-answer register (`W8/features-07`,
  `W8/f07-rules-swapped`, `W8/fifo-gap`), which would then be refusals and no longer answers.
- **(c) Left until M1n rules what *forbid* covers**, and the contradiction stays the kernel's until then: S1f.21.11's
  task 2.1 waits.
