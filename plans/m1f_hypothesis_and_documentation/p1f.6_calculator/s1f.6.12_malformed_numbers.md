# S1f.6.12 — Malformed numbers, and the names the theory occupies

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's ruling of that day. Its first task is an investigation on the
prototype and **stops for the user** with what it found.
**Depends on:** the investigation needs only the prototype (`build/s1f.6/prototype/r3/k.ein`); the build needs
[S1f.6.6](s1f.6.6_division.md), so every operation flags.

## The ruling

The user, 2026-11-11: *"with theory loaded names `n`, `d`, `z`, `sum-of` etc became occupated and must not be
used with other meaning, so if theory can detect malformed numbers - do it, what cons/objections?"*

Today a request the theory cannot read derives nothing: exit 0 and no word
([design.md, round 1's finding 3 and round 2 § A](design.md)). The ruled direction: the theory detects what it
can, a malformed request derives `(malformed <request>)`, and a second rule derives `(false)` from the flag, as
division by zero does.

## T1, the investigation: what can be detected, and what it costs

The answer to *what cons/objections* is taken by experiment, on the prototype, before anything is planned
further. A detection must be monotone: no `absent`, no `not`.

| the malformation | the candidate detection | to find out |
|---|---|---|
| a name that is no digit where a digit goes, `(n z x)` | ten `neq` guards on the digit's slot, `(neq ?a 0)` … `(neq ?a 9)`; `neq` is computed and monotone | does it load and fire only on a non-digit; what a guard that ranges a variable over the program's names costs when the slot is bound; whether one table of the ten digits with a guard is cheaper |
| a nested fact where a digit goes, `(n z (n z 1))` | a pattern, `(n ?p (n ?q ?a))`, and one for each other head a value can have | whether *any fact that is no digit* can be said without listing heads |
| a number that does not end in `z`, `(n 1 2)`, `(n end 1)` | the innermost prefix is a name that is not `z`: a `neq` guard against `z`, and against being a fact | the same questions |
| a name where a number goes, `(add foo z)` | as the row above, at the operand | whether it collides with named constants, `(let foo N)`: a name is a number exactly when a `let` binds it, and *no `let` binds it* is a negation |
| a nested fact of another relation where a number goes, `(add (pair 1 2) z)` | none known without listing every other relation | stated as not detectable, or found otherwise |
| `n` with the wrong count of arguments, `(n 1)`, `(n z 1 2)` | none in the theory: no pattern of arity two matches it. It is E-2 of [`findings.md`](findings.md), and the fix is the loader holding a nested fact to its relation's declared arity | what the loader does today with a top-level fact of the wrong arity; what a check on nested facts would refuse in the corpus (a census); whether it is refused at load, which is better than a flag |
| a request with the wrong count of operands, `(add X)` | the same loader check | the same |

For each row T1 reports: detected or not, by which rule, the exact output, and **what the detection costs a
well-formed request** in firings and in the measure of S1f.6.3's test, since a check that walks every operand
before every operation doubles the walks. It also says where the check runs: on the way through the circuit,
where a walk that meets a non-digit flags and costs nothing extra, or as a pass of its own.

The objections T1 is expected to weigh, each to be confirmed or dropped by the experiment:
- **Cost,** if detection is a separate pass.
- **Completeness.** Some malformations cannot be detected without negation; a program then cannot read the
  absence of a flag as *well-formed*.
- **`(false)` for a typing mistake** empties R for the whole program, as ruled for division by zero. That is the
  ruled meaning; T1 only shows it on a witness.
- **Rules that fire on other people's facts.** A detection rule whose pattern is loose can flag a fact of the
  program's own that happens to use `n`. This is the occupied-names question below.

T1 ends with the list, a recommendation per row, and **stops for the user**.

## T2, the names the theory occupies

The user's point: once the theory is imported, `n`, `z`, `sum-of` and the rest mean what the theory says. T2
establishes what a clash does today, by probes, and what should be said:

- a program that declares its own `(relation n …)` beside a flat import of the theory (the loader's
  duplicate-name guard, or two declarations merged);
- a program that states facts of `n` with another meaning and never declares it;
- a program that uses `z` as an unrelated name, in a fact of its own, and in an operand's place;
- a program that declares a relation named as one of the theory's internal ones (`done`, `b`, a tag);
- the same four under a qualified import, where the theory's names carry a prefix and nothing clashes.

Then the proposal: the module's header lists every name it occupies, split into its interface and its internals;
the internals get names unlikely to clash (a common prefix is the default); and where the loader can refuse a
clash it already refuses, the page says so. Whether a module may **reserve** names, so that a clash is refused
at load, is a language question the stage puts to the user with the probes and does not decide.

## T3 onward, the build, after the user's ruling on T1

1. The detection rules T1's ruling keeps, in `std.arith`, under the rule of the theory.
2. `(malformed <request>)`, and the rule from it to `(false)`.
3. The loader's arity check on nested facts, if ruled: an engine task with its register rows filed `open` first
   (a nested fact of the wrong arity answered by silence is the witness), its census, its sweep and its pages,
   as S1f.6.11's.
4. One program per rule under `tests/stdlib/arith/`, a harness test of malformed requests, and the search-free
   test on well-formed ones unchanged.
5. The same for `std.num9`, where a digit slot and the fixed count of digits are the cases.

## Acceptance

- T1's table, every row with its output, and the user's ruling recorded.
- Every malformation ruled detectable derives the flag and then `(false)`; every one that is not is named on the
  module's page as answered by silence.
- A well-formed request costs what T1 said it would.
- E-2 is fixed by this stage or ruled.
