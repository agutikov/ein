# Q-M1f.133 — A macro's call inside a claim's fact or a `:using` is refused

A macro's call inside a fact a claim lists, a query's `:expect` or a `(solve … :expect …)`'s, is refused at load:
*query: its :expect lists (settled Ann), a call of the macro settled, (macro settled (?g) (paid ?g)) — a claim
lists facts, and …*. A call in a fact's `:using` is refused as a written fact's is. A claim headed by the call
names no relation of the program and is refused by the line
[Q-M1f.118](Q-M1f.118-a-claim-is-not-expanded-so-a-claim-headed.md) gives it; the line above is printed where
nothing else refuses the program.

**Why.** A claim lists facts and a `:using` names facts, and a fact is not expanded, so a call there is a fact no
state can hold: the claim could never be held. A claim that cannot hold is refused at load, where its author reads
it, rather than failing at the run.

**Held by.** `ein-cli/tests/macro_facts.rs`: `a_macros_call_written_as_a_fact_is_refused_on_every_command`, the
rows `inside-a-claims-fact`, `inside-a-region-claims-fact` and `in-using`.
