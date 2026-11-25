# Q-M1f.618 — `absent` and `open` heading a fact that is a value are names

A fact that is a value may be headed by `absent` or by `open`, and there each is a name, the relation of that fact,
of which the kernel reads nothing: under an ordinary relation
([Q-M1f.571](Q-M1f.571-absent-takes-a-premise-and-open-a-name-or-a-fact.md)), and in the three other places a value
stands, the fact a stored negation is of, an argument of `eq` or `neq` in a guard, and a premise a fact's `:using`
lists. Beside declarations of `p` and `q` and the fact `(p k)`, each of these loads:

```lisp
(not (absent k))
(not (open k))
(q a :rule r :using ((absent k) (open k)))
(rule r1 () :match (and (p ?x) (neq ?x (absent k))) :assert (q (absent ?x)) :why "r1 {?x}")
(rule r2 () :match (and (p ?x) (not (open ?x))) :assert (q (open ?x)) :why "r2 {?x}")
```

No shape is checked of such a fact: `(not (open k))` loads with `k` neither a fact nor a relation of the program,
which a written `(open k)` is refused for. The kernel reads the two words at the head of a premise, a written fact or
a conclusion alone ([Q-M1f.66](Q-M1f.66-open-is-a-name-and-the-kernel-reads.md)), and there what it refuses stays
refused: `(open (open P))`, `(open (absent P))` and `(absent (absent P))`
([Q-M1f.568](Q-M1f.568-directly-under-absent-or-open-the-other.md)), and a written or concluded `(absent P)`.

**Why.** A value is a fact a state could hold, read as written, and the kernel reads no form inside it; a word there
is what any word is, as it is under an ordinary relation. Refused in some of these places and not in others, the two
words would mean something where nothing reads them.

**Held by.** `ein-ir/src/ein.y`'s `NestedFact`, a value, whose head is a `ValueHead`, `absent` and `open` among
them, and which `NegFact`, `FactArg` and a fact's `:using` take; `ein-cli/tests/value_rule.rs`:
`what_a_state_could_hold_loads`, its shapes `absent-negated`, `absent-using`, `absent-concluded-value` and
`absent-premise-negated` on five commands; `ein-cli/tests/open_reserved.rs`:
`open_is_a_name_where_the_kernel_does_not_read_it`, its places a stored negation's relation, a fact's `:using`, a
guard's value and a conclusion's argument;
`examples/ein-bugs/wrong-answers/absent-fact-nested.ein`, which loads.
