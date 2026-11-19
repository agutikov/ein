# The rule every program gets: `std.core`

`std.core` is the one module of the standard library that no program imports. The load brings it into every
program, whole, after the program's own forms and those of the modules it imports, and its one rule is then a rule
of the program under its full name, `std.core.inconsistent`
([Q-M1f.551](../../../../decisions/stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)).

## What it says

```lisp
(rule inconsistent ()
  :match (false)
  :ein   inconsistent
  :why   "a state that derives a contradiction is inconsistent")
```

With $S$ a state, and $\bot$ the fact `(false)`:

$$
\bot \in S \implies S \text{ is inconsistent}
$$

`(false)` is a fact, the library's word for a contradiction: a rule concludes it, a program states it, and another
fact may hold it under an arrow. The rule states no `:assert`. `:ein inconsistent` is a word to the engine and no
fact ([`02-syntax.md` § Conclusion forms in `:assert`](../02-syntax.md#conclusion-forms-in-assert)): where a rule
that states it fires, the saturation of the state ends and the state is dead, and the firing writes nothing. So a
state that holds `(false)` is dead by this rule, and the engine reads `(false)` by its name nowhere to judge one.

| a state holds | the rule |
|---|---|
| `(false)`, concluded by a rule, stated by the program or committed | fires once, on that fact, and the state is dead |
| no `(false)` | does not fire; the state is alive, unless it holds a fact beside its stored negation, which the engine reads itself, with no rule |
| `(false)` only under an arrow, `(think Ann (false))` | does not fire: the rule matches a held `(false)` |

A program's own rule may state the same word over other premises, with no `(false)` between
([`examples/features/18_ein_inconsistent.ein`](../../../../../examples/features/18_ein_inconsistent.ein)).

## In a program

[`tests/stdlib/core/01_false_ends_the_state.ein`](../../../../../tests/stdlib/core/01_false_ends_the_state.ein)
imports nothing. A switch is on and off at once, which its rule `on-and-off` concludes `(false)` from:

```lisp
(on  s1)
(off s1)

(rule on-and-off ()
  :match  (and (on ?s) (off ?s))
  :assert (false)
  :why    "{?s} is on and off at once")

(solve :expect (false))
```

`ein saturate --dump tests/stdlib/core/01_false_ends_the_state.ein` lists the module's rule beside the program's,
and the one fact derived:

<!-- output -->
```text
;; Rules — the saturation agenda's (2)
;;   on-and-off  :params ()  stratum 0
;;   std.core.inconsistent  :params ()  :ein inconsistent
```

<!-- output -->
```text
;; ── DERIVED (1 facts) ──
  (false :rule on-and-off)
```

`ein solve tests/stdlib/core/01_false_ends_the_state.ein --trace t.md` tells root's two steps, the second the
module's, which derives nothing:

<!-- output -->
```text
### Step 1 — `on-and-off`
> s1 is on and off at once
Premises: `on(s1)`, `off(s1)`
Derives `false`.

### Step 2 — `std.core.inconsistent`
> a state that derives a contradiction is inconsistent
Premises: `false`
States `:ein inconsistent`: the state is inconsistent, and the step derives no fact.
```

**An explanation reads through the rule.** A dead state's core is the frontier of the `(false)` the rule matched,
and `refuted by` names the rule that concluded that `(false)`, since an explanation names the rules a program wrote
or imported:

<!-- output -->
```text
  unsat core        (off s1)
  unsat core        (on s1)
  refuted by        on-and-off
```

[`02_loaded_and_alive.ein`](../../../../../tests/stdlib/core/02_loaded_and_alive.ein) is the same program with the
switch on and not off: the rule is loaded, no state holds `(false)`, and root is alive.

## What a program cannot do with it

- **Import it.** `(import std.core)`, in a program or in a module, is refused
  ([`07-diagnostics.md` § 2.32](../07-diagnostics.md#232-no-import-names-the-module-every-program-gets)).
- **Leave it out, replace it or add to it.** Every load brings the module as the standard library holds it. A saved
  file carries it beside the program's other modules
  ([`10-answer-dump.md`](../10-answer-dump.md#a-state-file-is-its-program-and-the-choices-made)), and a `.einb`
  stores the program without it.

## The rule

### `std.core`

#### `inconsistent`

A held `(false)`: the state is inconsistent.

<!-- rule std.core inconsistent -->
```lisp
(rule inconsistent ()
  :match (false)
  :ein   inconsistent
  :why   "a state that derives a contradiction is inconsistent")
```

![The rule inconsistent of std.core, as the engine draws it](diagrams/21-std.core.inconsistent.svg)
<!-- /rule -->
