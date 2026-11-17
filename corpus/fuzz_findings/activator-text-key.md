# activator-text-key

Two activators of one rule whose text is one, `1` and `"1"`, in two orders: this file and
[`activator-text-key-swapped.ein`](activator-text-key-swapped.ein). A string is natural-language text, written only
after `:why`, `:source` or `:goal-text`, and never a value
([Q-M1e2.23](../../docs/decisions/language/Q-M1e2.23-a-string-is-natural-language-text-never.md)), so
`(holds "1")` is refused at the parse, and both files are. A compile plan is keyed by its activator's values, never
by their text, so two activators that print alike key two plans.

The two files are frontend-fuzzer seeds whose recorded outcome is the parse refusal, and the witnesses of
regressions `int-and-string` and `string-then-int` ([`corpus/regressions.toml`](../regressions.toml)). Their
fixtures,
[`examples/ein-bugs/activator-text-int-and-string.ein`](../../examples/ein-bugs/activator-text-int-and-string.ein)
and [`activator-text-string-then-int.ein`](../../examples/ein-bugs/activator-text-string-then-int.ein), are the
refusal's witnesses.

- property: **no-crash** in a debug build; a **lost derivation** in release

```
(rule note (?R ?f)
  :match  (and (?R ?a ?b) (holds ?f))
  :assert (noted ?a ?f))
(edge 1 2) (edge 2 3)
(holds 1)  (holds "1")
(note edge 1)
(note edge "1")
```

## What it guards

A plan keyed by its activator's arguments as text would give one key to an activator `1` and an activator whose
text is `1`, and the second would compile no plan: whichever is written first would keep its conclusions and the
other's would be lost, with nothing said, and a debug build's check that the compile shortcut skipped no activator
would abort. The fact printer drops a string's quotes, so the dump, `--events` and a claim could not tell the two
apart; the count of `noted` facts could.

The panic below is this find's signature, which a later find of the same site is compared against:

```
thread 'main' panicked at crates/ein-infer/src/engine.rs:134:13:
the rule-application counter said nothing changed and a full walk disagreed
```
