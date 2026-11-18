# M1n — non-monotone programs

**Status:** created 2026-11-19 on the user's instruction (below); **not started**. A phase runs only on the user's word.
**Where it sits** against M1g, M1h, M1k and M12 is not ruled; the user has said the work stops after M1f.

## The instruction

The user, 2026-11-19, verbatim: *"create new milestone M1n for non-monotone programs"*, and *"pull F21 and related
followups and ideas"*.

## What it is

A program is **non-monotone** when an `absent` reads what a rule, a commitment or another `absent` can still change,
so that a state's facts, R's shape or the answer depend on the order things happen in. Ein's standing rule since
2026-11-02 (the user, kept verbatim in F21's head): **detect, document, warn, and decide nothing** — *"Currently my
goal is not to build complete theory os solving the non-monotonic problems; But I want to have a robust ways to
detect non-monotonocity, understand consequences of solving it either way, be able to at least show warnings"*. The
engine claims no answer of a flagged program (AGENTS.md § The language). M1n is where the decisions that rule
deferred are taken, each on the user's word.

## What it pulls

**Moved here, whole** (from `plans/followups/`, 2026-11-19):

- [`f21_non_monotone_programs_decided.md`](f21_non_monotone_programs_decided.md), **F21**: the analysis of
  2026-11-02 (the state, the status, the program, path dependence, *forbid*), its seven decisions **D1 to D7** with
  their options, evidence and what holds until each is taken, and everything handed over to it since: M1e2's
  inventory, M1f's findings (rewriting a non-monotone program; P1f.10's H6-1, H6-3, H6-4 and K-41; the FIFO gap at
  root, `W8/fifo-gap-root`, with the user's three calls; P1f.22's N-15, *everybody's thoughts are false*, with the
  user's answer of 2026-11-19 and the idea it carries). The name F21 is kept for the document.
- [`f21_probes/`](f21_probes/): its fourteen probe programs, which the hole, node and taint censuses
  (`utils/censuses/`) run.
- **The register's rows**: the five rows of `W8` that F21 owned are owned by **S1n.2.1**
  ([`register.toml`](../wrong_answers/register.toml), `owner = "S1n.2.1"`; `python3 utils/wrong_answers.py` holds
  each to what the binary answers today).

**Read here, owned elsewhere** (each stays where it is; M1n reads the named part):

- [F5](../followups/f5_rules_as_data.md)'s **PF5.4**, stratification and termination for rules as data (M12's).
- [F26](../followups/f26_consistency_and_completeness.md)'s closed-world completeness and default reasoning
  (Reiter, Clark's completion, circumscription, the stable models behind `absent`) (M12's).
- [Idea 11](../ideas/11-saturation-and-satisfiability.md), *saturation and satisfiability*: its § *Семантика
  отсутствия* (the semantics of absence) and the closed-world completion of an incomplete fixed point.
- [Idea 10](../ideas/10-generic-self-modification.md)'s open question *semantic firewall / stratification*.
- [F12](../followups/f12_rules_and_relations/ideas2.md)'s note that `absent` makes a rule's operator valid but not
  monotone.
- [M1h's S1h.3.5](../m1h_beyond_word_problems/p1h.3_gaps_assumptions_and_readings/s1h.3.5_non_monotone_or_not.md),
  *non-monotone or not*, held to the word problems' programs.
- [M1g's P1g.7](../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md), *unsatisfiable against owing*, which
  reads F21's dead/owing status.
- [`08-monotonicity.md`](../../docs/ein/reasoning/02-kb-evolution/08-monotonicity.md): the documentation of what
  holds today, each claim labelled by its standing; § 7 the analysis, § 8 what is open, § 9 the causes.

## The phases

Drafted from F21's decisions and the calls handed to it; none is planned in stages, and none starts without the
user's word.

| phase | what | from |
|---|---|---|
| P1n.1 | **Detection, complete.** Every way a program is non-monotone is flagged, and only those: the race at root between two rules each waiting on an `absent` (the FIFO gap's call 2, N-15's question *"Does ein detect non-monotone program here???"*, answered *no* on 2026-11-19), an hrule that reads what a commitment can change (H6-1), the mechanisms check reading root's activators (H6-3, H6-4); the detector's verdicts against the oracle on the corpus | F21 § Handed over; the user's standing rule |
| P1n.2 | **What a node's state is, and the wrong answers it decides.** D2 (Q-M1e2.2: a fork of root, a state with its guards as premises, stratum-order admission) and D3 (path dependence as a constraint on incremental search); the register's five `W8` rows, S1n.2.1 their owner; N-15's twin-or-row call; the three statements that say more than holds (the FIFO gap's call 3) | F21 D2, D3; the register |
| P1n.3 | **What a flagged program's answer is.** D1 (*forbid* extended to every `absent` site), D4 (what is reported when R has holes), D5 (refuse, qualify, or search differently), D7 (the search past the oracle's limit) | F21 D1, D4, D5, D7 |
| P1n.4 | **Other ways to compute.** The user's *optimistic saturation* (*"at first step skip non-monotone rules and get a fixedpoint without them, and then check"*, 2026-11-19, recorded and not investigated); stratified evaluation; the well-founded and the stable-model semantics; truth maintenance; rewriting a non-monotone program into a monotone one (F21 § Handed over 2026-11-16) — each shown on the corpus's flagged programs before anything is built | the user's idea; F21's prior art |

D6, learning from a refutation that rests on an `absent`, is closed (2026-11-04, F21 § D6).

## Rules of the milestone

The standing ones ([`AGENTS.md`](../../AGENTS.md)): a phase per subject, a stage per subagent; a claim about the
engine is a run, held against `ein oracle`; a decision that moves an answer names every program it moves, with
today's answer and the new one; what a flagged program answers stays unclaimed until the decision that rules it;
no estimate is written.
