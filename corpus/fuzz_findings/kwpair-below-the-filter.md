# kwpair-below-the-filter

A keyword pair *below* the top level of an undeclared form: a generated program, mutated from
`examples/syntax/trace-steps.ein` (`trace` → `tracex`). No keyword pair parses inside a term or a nested fact, a
keyword being a form's own and closed per form
([Q-M1e2.28](../../docs/decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)), so the program
is refused at the parse, at its first `:rule`, on every command, and its fixture is the `parse-negative` entry
[`examples/broken/kwpair-below-the-filter.ein`](../../examples/broken/kwpair-below-the-filter.ein), whose
`.expected` is the report. No value holds a keyword pair, so a render view's value labeller has no keyword-pair case
to reach. The `.ein` beside this note stays a frontend-fuzzer seed, its recorded outcome the parse refusal
(`ein-ir/tests/golden/fuzz_seeds.txt`).

- property: **no-crash** — a generated program exits 0, 1 or 2, never a panic
- minimised: 2 → 1 form

```
(tracex (step s1 :rule from-condition :using (c10) :derives (lives-in N H1))
  (step s2 :rule adjacent :using (and s1 c15) :derives (color-loc Blue H2))
  (step s3 :using (s2))
  (step s4 :derives (final X)))
```

## What it guards

`tracex` is neither a kernel head nor a declared relation, so a loader that accepted the form would read it as an
ontology fact, and two render views label such a fact's arguments: `ein render constraints` and `ein render
program`. A labeller that recursed into each `(step …)` and reached its keyword pairs would abort on a program the
loader accepts. The parse refusing a keyword pair below a form's top level is what makes that unreachable.

The panic below, its site and its message verbatim, is this find's signature, which a later find is deduplicated
against:

```
thread 'main' (3482912) panicked at crates/ein-render/src/dot_util.rs:152:32:
not a value node: KwPair
```

```sh
ein render constraints corpus/fuzz_findings/kwpair-below-the-filter.ein   # the parse refusal, exit 1
```
