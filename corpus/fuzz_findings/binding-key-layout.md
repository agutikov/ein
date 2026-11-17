# binding-key-layout

Two activators of one rule, a name of digits and a nested fact in one position, `(note edge 1)` and
`(note edge (src Y))`, in two orders: this file and [`binding-key-layout-swapped.ein`](binding-key-layout-swapped.ein).
Each argument of an activator binds its parameter, a nested fact included
([Q-M1f.43](../../docs/decisions/engine/Q-M1f.43-a-plans-activator-is-every-argument.md)), and a rule
application's identity interns its activator's every argument, as the compile cache's key does
([Q-M1e.16](../../docs/decisions/engine/Q-M1e.16-a-firings-identity-keys-on-every-activator.md)). So both orders
derive the same two `noted` facts, `(noted 1 1)` and `(noted 2 1)`, in a release build and a debug one: the nested
fact's `?f` is `(src Y)`, which nothing `holds`.

The two files are the witnesses of regression `binding-key-both/binding-key-layout`
([`corpus/regressions.toml`](../regressions.toml)) and frontend-fuzzer seeds. The fixtures are
[`examples/ein-bugs/binding-key-both.ein`](../../examples/ein-bugs/binding-key-both.ein) and
[`binding-key-swapped.ein`](../../examples/ein-bugs/binding-key-swapped.ein), with the single-activator controls
beside them, and `ein-infer/tests/rule_semantics.rs`'s
`an_int_beside_a_nested_fact_in_one_position_loses_no_derivation` states the same program in-process.

- property: **no-crash** in a debug build; a **lost derivation** in release

```
(rule note (?R ?f)
  :match  (and (?R ?a ?b) (holds ?f))
  :assert (noted ?a ?f))
(edge 1 2) (edge 2 3)
(holds 1)  (holds 3)
(note edge 1)
(note edge (src Y))
```

## What it guards

Two activators of one rule whose arguments are of different kinds compile two plans. An identity that compared
two plans' bindings position by position under one key, with a nested fact binding no parameter, would compare
different variables, and the order the two activators are written in could decide what is derived, with nothing
said; a debug build's layout check would abort on both files. Both files answer alike:

```text
$ ein saturate --dump corpus/fuzz_findings/binding-key-layout.ein
;;   noted (2)
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
```

The panic below is this find's signature, which a later find of the same site is compared against:

```
panicked at crates/ein-infer/src/engine.rs:254:21:
assertion `left == right` failed: two plans share (rule, activator) and disagree on their register layout — `BindingKey` compares their value vectors and would be comparing different variables
```
