# S1f.21.8 — rules the standard library states

**Phase:** [P1f.21](README.md)
**Status:** planned 2026-11-19 on the user's word (*"plan sections from this list: A, B, C, ... as stages"*);
**not started**.
**Group:** [D](modifications.md#d-rules-the-standard-library-could-state-today), items 17 to 19.
**Waits on:** where each rule lives, which is this stage's first reading. Nothing in the engine.
**Writes:** `stdlib/` and its manifest; `tests/stdlib/`; `corpus/corpus.toml`; `stdlib/README.md`; the rules pages
under `docs/ein/user/04-reference/04-stdlib/`, written by their generator.

## What it does

Three sets of rules that load and run on today's engine, each a module a program imports. **Additive**: a
program that does not import one answers as it did. They are what
[S1f.21.11](s1f.21.11_kill_and_three_conventions.md) later gives every program with no import line.

1. **The elimination of a double negation** (item 17): one rule a count of arguments, the relation a parameter,
   as `examples/beliefs/06_double_negation.ein` writes it for two arguments.
2. **The contradiction** (item 18): `clash-0` to `clash-6`, one rule a count, and `clash-of-0` to `clash-of-6`,
   which derive each activator from a declaration ([`design.md` A.1](design.md#a1-the-contradiction-rule-set-contrainc)).
3. **The requirement, the payment and `discharged`** (item 19): `due` from an obligation's trigger, `paid` where a
   witness is held, `discharged` where no requirement is unpaid
   ([`design.md` A.2](design.md#a2-the-requirement-and-the-payment-emuinc), [A.3](design.md#a3-stdslots-two-obligations-slots-emuinc)).

**Reading 1, where they live.** Default: two modules, `std.negation` for 1 and 2 and `std.debts` for 3, each
rule under an activator of its own, so that importing a module beside `std.algebra` moves no fact of a program
that does not activate it. `std.logic` stays free for M1g. The other readings: 3 inside `std.algebra` and
`std.slots`, beside the obligations it restates; 1 and 2 as two modules.

**Reading 2, which counts.** Default: 0 to 6 for the contradiction, as the design has it, and the same for the
elimination; the count the corpus needs is taken by a census before the rules are written.

## What must hold

- **Each rule has its program** under `tests/stdlib/` that activates it, with an `:expect`
  (`ein-infer/tests/stdlib_coverage.rs`); a refutation rule has two, one where it fires and one where it is
  loaded, activated and satisfied; each file has its `corpus.toml` entry.
- Each rule has its region on its family's rules page and its drawing (`utils/gen_stdlib_rule_pages.py`, then
  `utils/render_diagrams.py`), and the manifest is written again (`utils/stdlib_manifest.py --write`).
- **The two invariances of the survey, re-taken with the modules imported and not appended**:
  [`design.md` § 2.1](design.md#21-from-a-fact-and-its-negation-false)'s, every corpus program `ein solve` answers
  with the contradiction module imported answers the same, but the seven *forbid* refuses
  ([V-1](findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none)); and
  [§ 2.4](design.md#24-the-verdicts-together-which-may-be-a-fact)'s, the unpaid requirements are the kernel's
  `owes` at every node of the seats program and of `examples/zebra2-obligations.ein`.
- The module's page says what V-1 found: a program that imports the contradiction rules and rests a refutation on
  an `absent` of that shape is refused. No corpus program imports the module but its tests.
- `utils/stdlib_mutants.py`'s survivors for the new rules are banked.

## Acceptance

`ein test tests/` holds every claim; `stdlib_coverage.rs` and `utils/check_generated.py` pass;
`examples/beliefs/06_double_negation.ein` imports the rule it wrote itself, and answers as before; one gate.

## Record

Nothing yet.
