# fuzz_findings/

Minimised inputs a fuzzer found something on, and the few written by hand beside them. Two fuzzers write here: the
frontend's, `cargo test -p ein-ir --test fuzz_properties`, and an engine fuzzer of whole programs. "Something" is
**a violated property**, named in the finding: `no-crash`, `id-order` (the same program under a permuted id space
answers the same), `jobs` (the same program at `--jobs 8` answers as at `--jobs 1`).

Every file here of 2 KB or less is also one of the frontend fuzzer's seeds (all of them but `fork-delta-order.ein`):
`every_seed_and_finding_parses_the_way_it_was_recorded` reads this directory, so adding one changes
`ein-ir/tests/golden/fuzz_seeds.txt`, and that is deliberate: a find that nothing re-parses is a souvenir.

**Empty is the steady state.** A find is either fixed, and promoted to a corpus fixture under `examples/` in the same
commit, or accepted as defined behaviour. What stays here is each find's text, which keys the seeds golden, and its
note, the dedup signature a later find is compared against. A row of
[`corpus/wrong_answers.toml`](../wrong_answers.toml) may point here when its witness violates a property the corpus
sweeps assert.

| file | property | what it is now |
|---|---|---|
| [`hrule-reads-not.ein`](hrule-reads-not.md) | `no-crash` | an `(hrule …)` whose `:match` reads the `not` relation. Legal ein-lang; every command answers it as `ein oracle` does, and where an hrule reads `not`, every hrule is matched before any candidate is emitted |
| [`unsat-core-id-space.ein`](unsat-core-id-space.md) | `id-order` | a program whose core could move under a permuted id space. No core it reports moves; its corpus fixture is [`examples/ein-bugs/unsat-core-id-space.ein`](../../examples/ein-bugs/unsat-core-id-space.ein), which the id-order sweep permutes |
| [`d3-unsat-core.ein`](d3-unsat-core.md) | `id-order` | the same, from another seed. Refused at load by *forbid*: `fire-0` concludes `(not …)` from `r1`, which `fire-1` derives under an `absent` over a relation a commitment can extend; the refusal is the same under every permutation. Its fixture is [`examples/ein-bugs/d3-unsat-core.ein`](../../examples/ein-bugs/d3-unsat-core.ein) |
| [`d3-goal-row-order.ein`](d3-goal-row-order.md) | `id-order` | one model that satisfies the goal twice. The read-out prints every binding row, sorted, so no row moves; its fixture is [`examples/ein-bugs/d3-goal-row-order.ein`](../../examples/ein-bugs/d3-goal-row-order.ein) |
| [`fork-delta-order.ein`](fork-delta-order.md) | `id-order` | **written by hand**, for the mechanism by which a dead node's core could move with the interner: a fork's delta is in the order its commitment was written, so one core is reported under every permutation; its fixture is [`examples/ein-bugs/fork-delta-order.ein`](../../examples/ein-bugs/fork-delta-order.ein) |
| [`binding-key-layout.ein`](binding-key-layout.md) and `-swapped` | `no-crash` | **written by hand**: two activators of one rule, a name of digits and a nested fact in one position, in either order. A rule application's identity is its activator's every argument, so both orders derive every `noted` fact; the fixtures are [`examples/ein-bugs/binding-key-both.ein`](../../examples/ein-bugs/binding-key-both.ein) and `binding-key-swapped.ein` |
| [`activator-text-key.ein`](activator-text-key.md) and `-swapped` | `no-crash` | **written by hand**: two activators whose text is one, `1` and `"1"`. A compile plan is keyed by the activator's values; a string is no value, so both files are refused at the parse, and their fixtures, [`examples/ein-bugs/activator-text-int-and-string.ein`](../../examples/ein-bugs/activator-text-int-and-string.ein) and `activator-text-string-then-int.ein`, are the refusal's witnesses |
| [`kwpair-below-the-filter.ein`](kwpair-below-the-filter.md) | `no-crash` | a keyword pair *below* the top level of an undeclared form. Refused at the parse, a keyword pair inside a term being no value; its fixture is [`examples/broken/kwpair-below-the-filter.ein`](../../examples/broken/kwpair-below-the-filter.ein) |

The id-order half of a register row that points here is held by
`id_order_invariance::the_register_s_id_order_rows_answer_what_they_record`, and the plain half, with every other
row, by `ein-cli/tests/wrong_answers.rs`.

```sh
ein solve corpus/fuzz_findings/d3-unsat-core.ein   # exits 1 with forbid's refusal
```
