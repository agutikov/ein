# Q-M1f.576 — The blind enumerator writes only where root holds no `open` fact and no obligation is activated

The blind enumerator writes its guesses only where root's fixpoint holds no `open` fact and no obligation is
activated: never beside hypotheses a rule or the program writes, nor beside an obligation, which writes what would
pay what is owed. A rule that concludes nothing at root keeps it off no more than one the program lacks.

**Why.** A program that says what to suppose, or what it owes, is not second-guessed by a guess about everything.

**Held by.** `ein_infer::hypgen`; `ein-infer/tests/open_facts.rs`: `the_blind_enumerator_writes_only_where_nothing_else_does`.
