# M1a — divergence ledger

Differences between `ein` (Python, the oracle) and `ein.rs` that are
**accepted** rather than fixed. Empty is the goal at the
[P1a.5](p1a.5_presentation/README.md) byte gate; a non-empty ledger is
allowed only with a written reason per entry.

The precedent for this shape is
[`docs/kernel/inference/parity_baselines.md`](../../docs/kernel/inference/parity_baselines.md),
which recorded the tree-vs-monotonic divergences explicitly rather than
treating them as failures or hiding them behind `xfail`. The difference
is the standard: that comparison was between two *different engines*;
this one is between an engine and its port, so the bar for an entry is
much higher.

## Rules

1. An entry needs **"what would make this unacceptable"** — a stated
   condition under which it becomes a bug. An entry without one is not a
   decision, it is a shrug.
2. An entry needs a **fixture** in the corpus that demonstrates it, so it
   cannot silently widen.
3. Anything on the [normalisation list](design/01_parity_contract.md) §5
   is *not* a divergence — that list is closed and lives in the design
   doc. Adding to it requires an [open question](open_questions.md).
4. When an entry is fixed, keep it with `**Status:** fixed in <stage>`.
   The trail is the memory.

## Template

```markdown
### D<n> — <one-line title>

**Found:** <date>, <phase/stage>
**Tier:** T<k>
**Status:** accepted | fixed in S1a.<p>.<s>
**Fixture:** <corpus entry>

**What.** <the observable difference, both sides quoted>

**Why it is acceptable.** <argument>

**What would make it unacceptable.** <the condition>
```

## Entries

### D1 — a rule may not bind more than 256 variables

**Found:** 2026-08-18, [S1a.3.1](p1a.3_deductive_core/s1a.3.1_compiler.md)
**Tier:** T2 (it would surface as a compile refusal, hence a firing difference)
**Status:** accepted
**Fixture:** `ein.rs/crates/ein-infer/tests/compile_limits.rs` — built in the
test, **not** checked into `examples/`: a corpus file ein.py compiles and
ein.rs refuses would fail the corpus parity test, which is the alarm this
ledger wants left armed for divergences nobody chose.

**What.** A `:match` binding more than `MAX_REGS` = 256 distinct variables
compiles in ein.py and is a `CompileError` in ein.rs:

```
more than 256 distinct variables in one `:match` — ein.rs numbers a rule's
variables into a fixed register file (256 slots) so the matcher's inner loop
allocates nothing. Split the rule.
```

**Why it is acceptable.** ein.py's bindings are a `dict`, so it has no bound to
port; ein.rs resolves every variable to a register in a fixed-size file,
because that is what makes the inner loop allocation-free
([design/05](design/05_matcher.md) §3 — the change the whole matcher rewrite is
for). A ceiling therefore exists by construction, and the only question is
where. 256 is **42×** the widest rule anything in the corpus compiles
(`domain-elimination`, 6 registers), and the overflow is a typed error with a
remedy rather than a panic or a silent truncation.

**What would make this unacceptable.** A rule anyone actually writes coming
within 8× of the ceiling. `compile_limits.rs::the_corpus_is_nowhere_near_the_ceiling`
measures that distance on every corpus file and fails when it closes, so the
condition is checked rather than remembered.

### D2 — `sorted(alive)` raises in ein.py where ein.rs answers

**Found:** 2026-08-18, [S1a.4.3](p1a.4_search_layer/s1a.4.3_apriori_and_nogoods.md)
— predicted at S1a.0.1 as
[design/02](design/02_determinism_and_order.md) § H2 and
[Q-M1a.4](open_questions.md#q-m1a4--sorted-over-mixed-type-fact-args), and
reached by the first op that runs the layer arithmetic.
**Tier:** T1 (a search-layer counter; T0 as a crash)
**Status:** accepted
**Fixture:** [`examples/ein-bugs/mixed-type-hypothesis.ein`](../../examples/ein-bugs/mixed-type-hypothesis.ein),
in the `crash-parity` group, pinned on the Python side by
`ein.py/tests/inference/test_mixed_type_hypothesis.py` and on the Rust side by
`hypgen_parity.rs`'s `divergent` list — which **asserts** the divergence, so a
file that stopped diverging would fail as loudly as one that started.

**What.** `apriori.layer_1` opens the search with `sorted(alive)` over
`(relation_name, args)` tuples. Two candidates of one relation whose slot *i*
holds a `str` in one and an `int` in the other are incomparable:

```
TypeError: '<' not supported between instances of 'str' and 'int'
```

ein.rs orders `Int < Sym < Fact` by tag and answers:

```
LAYER1 [{(seat Ann 1)}, {(seat Ann left)}]
```

**Why it is acceptable.** Three reasons, in order of weight.

1. **Nothing can reach it without an `hrule`.** Blind hypgen builds candidates
   out of `kb.names`, and `rebuild_indexes` only enters an argument there
   `if isinstance(a, str)`, so every blind candidate is all-strings. Only an
   hrule carries a non-string through, because its `:assert` args come from
   bindings. That scope claim is itself a test
   (`test_blind_hypgen_cannot_produce_a_non_string_arg`), so a change that
   widened it would re-open this rather than quietly extend it.
2. **A crash is not semantics anyone wants preserved.** Reproducing it would
   mean making the port fail on an input it can answer, and answering is the
   behaviour a user would ask for if asked.
3. **The alternative costs the whole corpus.** Fixing ein.py to sort by `repr`
   here — as `canon.state_key` already does — changes the candidate order of
   every puzzle and re-baselines every T2 golden, to buy an input nobody has
   written.

The order ein.rs picks is not arbitrary: `Terms::cmp_semantic` agrees with
Python's `sorted` on every pair Python *can* compare, and the cross-tag order
is only consulted where Python raises.

**What would make this unacceptable.** A real puzzle wanting mixed-type slots —
at which point option (b) of Q-M1a.4 becomes right and ein.py is fixed first,
both ports moving together. The trigger is visible: this is the only entry in
`crash-parity` that is a *search-layer* crash, and a second one would mean the
scope claim above is wrong.

