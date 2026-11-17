# P1f.21 — findings outside the stages' scope

**Phase:** [P1f.21](README.md)

What a stage of this phase met that is not its own to settle: something the engine does that a page or a
decision says otherwise, or says nothing of. Each has the program and the command that shows it. None is a wrong
answer about $R$, and none is filed anywhere else; a stage that fixes one says so here.

`ein` is the gate build, `ein.rs/target/gate/ein`, of the main tree as it stood on 2026-11-17. A program named
without a path is in [`design.md`](design.md), whole.

| id | what | found by | state |
|---|---|---|---|
| [V-1](#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none) | *forbid* reads rules, and the kernel's contradiction is none: seven corpus programs that load are refused once the contradiction is written as a rule | S1f.21.1 | open |
| [V-2](#v-2--the-surfaces-say-derives-false-of-a-state-that-holds-none) | the surfaces say *derives (false)* of a state that holds none | S1f.21.1 | open |
| [V-3](#v-3---p-prints-a-fact-of-no-argument-with-a-space-inside) | `-p` prints a fact of no argument with a space inside, `(alive )` | S1f.21.1 | open |
| [V-4](#v-4--the-saturated-dump-lists-no-obligation-rule) | the saturated dump lists no obligation rule | S1f.21.1 | open |
| [V-5](#v-5--the-users-notes-from-p1f22-questions-as-facts-the-two-negation-rules-and-the-names) | the user's notes from P1f.22 that are this phase's: `(open q)` as a fact that is a question; the elimination of a double negation as a rule beside the contradiction's; the names of `(false)` and `(open)` | the user, 2026-11-17, at P1f.22 | open: for the ruling of Q-M1f.390 and for Q-M1f.399; taken further by the user's second instruction, [`open_facts.md`](open_facts.md), for S1f.21.4 |

## V-1 — *forbid* reads rules, and the kernel's contradiction is none

**What.** [Q-M1e2.5](../../../docs/decisions/language/Q-M1e2.5-a-refutation-may-not-rest-on-an-absent.md) refuses
a rule that concludes `(false)` or a stored negation where it rests on an `(absent …)` over a relation a
commitment can still extend, *directly, through a fact a rule derives resting on such an `absent`, or through its
activator*. The kernel's contradiction, a fact beside its stored negation, is the same refutation and is no rule,
so *forbid* does not read it: a program in which a fact derived under such an absence can stand beside its
negation loads.

**Shown by.** The contradiction written as a rule, one rule a count of arguments with its activators derived
from the declarations ([`design.md` Appendix A.1](design.md#a1-the-contradiction-rule-set-contrainc)), appended to
every corpus program `ein solve` answers ([`sweep_clash.py`](design.md#b1-sweep_clashpy)): 560 answer as they
did, and seven are refused, each by *forbid*, each naming the written rule. For
`examples/lattice/08_absent_witness_hole.ein` with the set appended, `ein solve` prints, exit 1:

```text
rule 'clash-0' [activator meal]: concludes (false) from meal, which rule 'soup-unless-allergic' derives resting on (absent …) over allergic, a relation a commitment can still extend — so it calls a state that has not yet decided allergic a contradiction, and a commitment would lift it (dead is not upward-closed under absent; Q-M1e2.5). …
```

The seven: `examples/features/07_unstratifiable.ein`, `examples/lattice/08_absent_witness_hole.ein`,
`examples/lattice/10_fifo_gap.ein`, and `f07-fpoff.ein`, `f07-rules-swapped.ein`, `kw-neq-in-absent.ein`,
`kw-neq-in-absent-twin.ein` under `examples/ein-bugs/wrong-answers/`.

**What it is not.** No wrong answer. Each of the seven is a program the static check flags as it stands
(`monotonicity.monotone` is `false` in its `--json-summary`), since the fact derived under the absence is a
positive conclusion from an absence a commitment can fill, and of a flagged program the engine claims nothing.
So the two readings agree that the program is outside what is claimed; they differ in that one refuses it and
the other answers it. The refusal is also at relation grain: it is issued for `clash-0` under an activator
whether or not any rule can conclude the negation of that relation's fact.

**Whose.** The reading of *forbid* is [F21](../../followups/f21_non_monotone_programs_decided.md)'s ground. It
bears on S1f.21.2 only as one line of the contradiction rule's account: what the kernel does differs from firing
the rule in that *forbid* does not read it.

## V-2 — the surfaces say *derives (false)* of a state that holds none

**What.** A state dead by a fact beside its negation holds no `(false)`: the contradiction is read, and nothing
is derived. The read-out, the answer dump's reference, the region drawing's legend and the code's own comment
all name *dead* as *derives `(false)`*.

**Shown by.** `c1.ein` ([`design.md` § 1.1](design.md#11-contradiction)). `ein saturate --dump c1.ein` ends

```text
;; ── DERIVED (2 facts) ──
  (not (p one) :rule r-denies-p)
  (p one :rule q-gives-p)
```

and `ein solve c1.ein` prints

```text
  empty             every node is dead — root's own fixpoint derives (false)
  …
  (false)
```

The same words: [`10-answer-dump.md`](../../../docs/ein/user/04-reference/10-answer-dump.md)'s `root.dead` row
(*root's own fixpoint derives `(false)`*); the legend of `ein render region` (*dead: derives (false)*);
`ein-infer/src/region.rs`' `DEAD` (*the status bit of a node that derives `(false)`*). The reasoning pages define
*dead* with both clauses and are right.

**What it is not.** No answer moves. The words are true of every dead state once the contradiction rule is read
as a rule, which is what the phase's second stage states; whether the surfaces keep the words on that ground or
say *holds a contradiction* is a wording to settle with those pages.

## V-3 — `-p` prints a fact of no argument with a space inside

**What.** The text of a fact of no argument is `(r)`
([the knowledge base § The text of an object](../../../docs/ein/reasoning/01-static-ir/01-knowledge-base.md#the-text-of-an-object)).
`ein solve -p` and `-P` print it `(r )`.

**Shown by.** `v1.ein` ([`design.md` § 2.2](design.md#22-dead-from-false-alive-from-its-absence)).
`ein solve v1.ein -p`:

```text
── state 1 of 1 ──
final-state facts (beyond the program as loaded: root's derivations, the commitment and what it derives; 1 fact):
  (alive )
```

The same run's `stale` row prints `(alive)`, as `ein saturate --dump` and `ein oracle --dump DIR --states` do.

## V-4 — the saturated dump lists no obligation rule

**What.** `ein saturate --dump FILE` lists the program's rules under `;; Rules (N)`, each with its parameters
and its stratum. An obligation rule is in none of its sections: the dump shows its activator's relation under
`;; Relations — auto-vivified` and the activator among the facts, and nothing says the program has an
obligation.

**Shown by.** `ein saturate --dump examples/lattice/05_owed_seats_bounded.ein`: `;; Rules (38)`, the rules of
`std.algebra` that are in the agenda, among which neither `total-owed` nor `surjective-owed` stands;

```text
;; Relations — auto-vivified (4)
;;   total-owed
;;   surjective-owed
;;   functional
;;   injective
```

`ein saturate --dump examples/zebra2.ein` lists `;; Rules (60)` and no hypothesis rule either, so the section is
of the agenda's rules; what the dump leaves unsaid is that it is.

## V-5 — The user's notes from P1f.22: questions as facts, the two negation rules, and the names

**What.** Three things the user said while ruling P1f.22's questions (2026-11-17, verbatim in
[P1f.22's `contexts.md` § 10](../p1f.22_nested_facts/contexts.md#10-the-users-second-notes-verbatim-and-what-they-rule))
that are this phase's to take up, none of them built or ruled here.

1. **`(open q)` as a fact that is a question.** *"if we will emit ordinary facts (open (lives-in Alice House-1))
   they naturally are hypotheses, what rules can do with them? And, (open) context contains "open" questions, like
   "Does Alice live in House-1?" - so there is a meaning of those facts, so we can write rules to infer something
   from them directly"*. Run with a relation `question` in the place of `open`, which the parse refuses under an
   arrow: an hrule over the questions proposes exactly the asked facts, and ordinary rules derive a further
   question, an answer where asked, a contradiction where every answer is denied
   ([P1f.22's `contexts.md` § 13](../p1f.22_nested_facts/contexts.md#13-questions-as-facts-what-rules-can-do-with-open-q),
   `q1.ein` and `q2.ein`). What a held fact cannot be is *still open*: it stays held once answered. That is
   [`design.md` § 3.2](design.md#32-it-can-be-removed) again, and it lines the note up with Q-M1f.390's option
   (c), the question and its answer as facts and *owed* their difference.
2. **The elimination of a double negation is a rule**, beside the contradiction's. Ruled for P1f.22 as its
   Q-M1f.391: `(not (not g))` becomes a fact a file may write, the kernel is not changed, and the rule that
   concludes `g` from it is a theory's. Its home, the module and whether a program imports it, is this phase's to
   say, with [Appendix A.1](design.md#a1-the-contradiction-rule-set-contrainc)'s rule set, which is of the same
   shape: one rule a count of arguments, the relation a parameter.
3. **The names.** *"things like (false) and (open) are effectively controlling the search. Maybe word `false` is
   too heavy loaded with classical binary logic and not expose its actual function in ein? Maybe (abort) or (dead)
   is more clear? Or some common search control interface accessible from ein? Like (ein.search.state.refute)?"*
   Put as [Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open), with
   the options in [P1f.22's `contexts.md` § 12](../p1f.22_nested_facts/contexts.md#12-the-kernels-surface-false-and-open-control-the-search).
   One fact bears on it from P1f.22: `(false)` also stands under an arrow, `(think Ann (false))`, where it is a
   proposition and controls nothing, so a name for the search's act reads badly there.

**The user's direction on 3, 2026-11-17**, verbatim: *"Renaming (false): rules as before can infer (false) in
contexts, also can infer (false) in global context, but the state considered dead only if there is a rule that
infer (ein.contradiction) from global (false), or from global (whatever); both (false) renaming and (open) facts
belongs to p1f.21"*. So, as read for this phase's design: `(false)` stays the word and becomes an ordinary fact,
inferable under an arrow and globally, with no effect of its own; the kernel reads one fact,
`(ein.contradiction)`, and a state is dead where it holds that; and what leads there is a rule a theory states,
`(false)` ⟹ `(ein.contradiction)` the plain one. That is [`design.md` § 2.2](design.md#22-dead-from-false-alive-from-its-absence)'s
*dead from `(false)`* made a rule a program can see and replace, and the prefix is option (d) of Q-M1f.399.

**Taken further on 2026-11-18** by the user's second instruction to the phase ([the README](README.md#the-second-instruction)):
the kernel's fact is a verb, `(ein.kill)` among the names offered, and an obligation emits the hypotheses
themselves under `open`, which rules read and which leave the open context as a state decides them. Point 1 of
this entry is § 6's row *ask* there. Worked out in [`open_facts.md`](open_facts.md), for S1f.21.4.
