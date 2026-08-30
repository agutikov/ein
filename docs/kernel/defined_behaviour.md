# Ein — defined behaviour

Thirteen behaviours whose only statement, until 2026-08-21, was a Python source
file. This page is where they are *stated* — and, since M1c S1c.1.2, where the
first diagnostics that never had a Python counterpart are stated too
([§4.1](#41-the-first-errors-with-no-python-counterpart--m1c-s1c12)).

> **Audience: both.** §1 and §4 are what you meet when a program is wrong;
> §2 and §3 are what a reimplementer needs and a puzzle author can skip.

## Why this page exists

For five phases of [M1a](../history/m1a_rust/README.md) the port had an
**oracle**: `ein.py` ran beside `ein.rs` over the whole corpus and the two were
compared byte for byte. That made a whole class of question cheap to answer —
"what *should* the engine print here?" was "whatever the other one prints" —
and it let the specification stay silent about things the implementation had
already decided.

[P1a.10](../history/m1a_rust/README.md#p1a10--one-implementation) removed
the second engine, and with it that answer. A behaviour defined as *whatever
ein.py did* became **undefined** the moment ein.py left the tree, and
undefined behaviour in a specification repo is worse than a quirk: nobody can
tell a regression from a fix.

So each one is written down here, as ein.rs's own defined behaviour, with the
Python provenance as an aside. Three consequences of that framing are worth
being explicit about:

- **These are normative.** A change to any of them is a change to the kernel,
  and needs the same treatment as a change to `(absent …)`: a reason, a
  fixture, and an entry in whatever ledger is current.
- **"Because CPython did" is not a reason any more — but it was a good one.**
  Several of these are strange in isolation and were kept because reproducing
  them exactly is what made the port *measurable*
  ([design/01](../history/m1a_rust/design/01_parity_contract.md)). The
  argument for keeping them now is different and weaker: they are what every
  checked-in fixture, golden and example output in the repo was baselined
  against. Where a better behaviour is known, it is named below.
- **This is not the divergence ledger.**
  [`divergences.md`](../history/m1a_rust/divergences.md) records where the
  two engines *differed*; this page records where they agreed and only one
  of them said why.

## 1. Parse and load diagnostics

A parse error is `{file}:{line}:{col}: unexpected input`, followed by a
context block: the source line, then a caret under the offending column.

### 1.1 EOF reports `-1:-1`

An input that ends while the parser still wants more reports line `-1`,
column `-1`, and its context block renders the **last** line of the file with
the caret one character past its end.

```text
$ ein solve /tmp/t.ein          # file is: (relation r T
/tmp/t.ein:-1:-1: unexpected input
(relation r T
             ^
```

*Why.* Lark's `UnexpectedEOF` carries `pos_in_stream = -1`, and CPython's
negative slicing turns that into "everything but the last character" and "the
last character onward" — which renders as the tail of the file. It is a bad
message and it is the one every `examples/broken/` fixture is baselined
against. [Q-M1a.3](../history/m1a_rust/open_questions.md#q-m1a3--parse-error-message-parity)
option (c) — improve it — was deferred past the byte gate and is still open.

### 1.2 The context window is ±40 characters, applied before the line is trimmed

The block is built by taking 40 characters either side of the error position
and *then* cutting to the current line. An error past column 40 therefore
renders a **truncated** source line, with no ellipsis marking the cut:

```text
$ ein solve long.ein             # (relation aaa…60 a's… T){
long.ein:1:74: unexpected input
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa T){
                                        ^
```

The line begins `(relation`, and the block does not.

### 1.3 The reported column is the first position with no pending `%ignore` match

This is the strangest one, and it was found by a fuzzer rather than by
reading. The reported position is not where the grammar first fails; it is
the first position at or after that point where the scanner has no *pending*
whitespace-or-comment match.

```text
(y";"{      → 1:6   ·  the `{`
(y";"{?     → 1:7   ·  the `?`
```

The two inputs fail at the same place. They report different ones because the
`;` **inside the string literal** matched the comment rule `;[^\n]*`, whose
end is one character further along, and a pending match holds the error back
until the scanner walks past it.

*Why.* Lark's Earley scanner keys pending matches in a `defaultdict(list)` by
end position, and the `%ignore` pass writes a key at every position where
whitespace or a comment matches — including inside a string, and including
when the scan set is empty, which still *creates* the key. A dict holding one
empty list is truthy. `ein-ir`'s `parse::death_position` simulates exactly
this rather than reporting the true failure point.

### 1.4 Ambiguity resolves to the earlier alternative

Where the grammar admits two parses, the one written first in the grammar
wins — and `rule` is written before the generic-fact alternative. So the same
six letters split two ways depending on what follows them:

```lisp
(rulex (?a) :match (q ?a) :assert (q ?a))   ; a RULE named x
(rulex A)                                   ; a FACT named rulex
```

The second is not the ambiguity resolving the other way; it is the only parse,
because a rule needs `:match` / `:assert`. `SYMBOL`'s reserved-word exclusion
is a negative lookahead with a **word boundary**, which is why `rulex` is a
legal symbol at all where `rule-x` is a parse error
([`00_ebnf.md` §1](ir/03-ein-lang/00_ebnf.md)).

### 1.5 Loader messages about a top-level form end in `at None`

Every error the loader raises interpolates the form's source location, and a
*top-level* form is built without one:

```text
unknown relation `foo` at None
```

**23 of the 30 `examples/broken/load/` fixtures** have a message that ends
this way. The seven that do not are the ones whose error is raised somewhere
that *has* a location, or nowhere that has one at all:
`macro_arity_mismatch` (inside macro expansion, on a nested node — it prints a
real `Loc(file=…, line=6, col=20)`), the two `config_*`, the two `import_*`,
`unimported_std_macro`, and `derivation_cycle`.

*Why.* [Q-M1a.6](../history/m1a_rust/open_questions.md#q-m1a6--at-none-in-loader-messages)
— a genuine usability bug, reproduced deliberately so that fixing it would be
one visible re-baseline rather than noise during the port. It is the clearest
candidate on this page for being **fixed** rather than documented; what it
costs is re-blessing every message in that directory at once.

## 2. Values and order

### 2.1 Fact arguments are totally ordered: `Int < Sym < Fact`

Within a tag, integers order by numeric value at any width, symbols by
**lexicographic rank** — position in the sorted symbol list, never the order
the interner assigned ids in — and facts by relation name, then arguments
element-wise, then arity, so a shorter argument tuple that is a prefix of a
longer one sorts first.

*Why it is stated rather than inherited.* The cross-tag half of this order has
no Python counterpart at all: `sorted()` **raises** on a `str`/`int` pair and
on any pair of facts. The search layer sorts its alive set, so a puzzle with
mixed-type arguments in one relation slot, or with an `(hrule … :assert (not
…))` head, used to crash. `Terms::cmp_semantic` agrees with `sorted()` on
every pair Python could compare, and the cross-tag order is consulted exactly
where Python raised —
[D2](../history/m1a_rust/divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers).

### 2.2 Where a value is *printed* or sorted for display, it is printed as CPython's `repr()`

State-dump node order, explanation tie-breaks and DOT labels all go through a
faithful `repr()` for the four shapes a fact argument can take — `str`, `int`,
`tuple`, and a nested fact. String escaping follows CPython's rule, which is
by Unicode **general category**, not by `is_control()`:
`ein-core/src/printable.rs` is generated from CPython's own tables (737
ranges, Unicode 16.0.0) by
[`utils/gen_unicode_printable.py`](../../utils/gen_unicode_printable.py). Run
it after a CPython upgrade; a category that moves surfaces as a named code
point in the differential test.

Integers are carried as canonical decimal **text**, not as an `i64`: the
grammar accepts `-?[0-9]+` at any width, so a fixed-width parse would reject
values the engine can otherwise handle.

### 2.3 Formatted floats follow Python's `format(x, spec)` for the `f` type

`--hyp-stats` percentages, `--timing` columns and `--stats` elapsed times are
`[[fill]align][sign][0][width][.precision]f`. Three points where the obvious
Rust spelling is wrong: NaN prints `nan` (not `NaN`), a NaN never carries a
sign while an infinity does, and an **empty** spec is `str(x)` rather than
`.6f`. Anything outside that subset is rejected rather than guessed at.

### 2.4 Canonical state identity is the sorted fact list itself, never a hash

Two commitment-set branches that saturate to the same closed KB collapse to
one lattice node, and the key that decides this is the **representation**: a
collision costs a comparison, not a wrong answer. `state_digest` exists only
for display and is never identity.

Any total order over facts serves for identity, so the key sorts by interned
id. Where the key is *displayed* — `--dump-states` orders its nodes — the
`repr()` order of §2.2 is used instead, because that is the order every
checked-in dump was baselined against.

## 3. Search

### 3.1 `--shuffle` is CPython's Mersenne Twister, exactly

`--shuffle` (`-z`), with `--seed` (`-d`) pinning the permutation, seeds
MT19937 the way `random.Random(seed)` does — absolute value, split into 32-bit
words — and shuffles each layer with `random.shuffle`'s downward Fisher–Yates
via `_randbelow`'s rejection loop, carrying generator state across layers.

The absolute value is observable: `--seed=-7` and `--seed 7` are **the same
generator**, and two exhaustive runs of one puzzle under them produce
`summary.json` files that differ in exactly one field — the seed the run
recorded.

*Why reproduce it at all.* Shuffle-invariance is a property the engine
claims: a `--shuffle` run must reach the same verdict, `k` and models as an
unshuffled one. Reproducing the exact generator is what lets a shuffled run be
compared against a recorded one at all, and `--shuffle` runs are precisely the
ones where a silent ordering difference would be easiest to dismiss.

### 3.2 A rule application's identity ignores nested-`Fact` activator arguments

> **Corrected 2026-08-29** — M1e
> [S1e.1.4](../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.4_defined_behaviour_q_m1a8.md),
> the review's `Q3`. Until then this section said *integer*, and the engine has
> never done what it described. The claim was written from one of the key's
> three components; the probe that refuted it is
> `ein-infer/tests/rule_semantics.rs::activators_differing_only_by_an_int_argument_both_fire`.

The identity of a firing is `(rule, activator, bindings)`, and there are
**three** keys over one activator, each keeping a different part of it:

| key | what it keeps of the activator |
|---|---|
| the compile cache key | **every** argument, stringified |
| the identity's activator half (`plan.activator_args`) | the **symbol** arguments |
| the identity's bindings half (the plan's register file) | every argument that **binds a parameter** — symbols *and* integers |

An **integer** argument binds its parameter, so it is absent from the second
key and present in the third: two activators differing only in an integer
argument have **different** identities and both fire. A nested **`Fact`** binds
nothing, so it reaches neither: two activators differing only there share one
identity, and the second application is dropped before it is enqueued.

That collision is by itself harmless, and the reason is worth stating because
it is what makes the *next* paragraph the whole of the defect: the activator
reaches the compiler at one site, which skips a `Fact` argument outright, so
two activators differing only in a nested `Fact` compile to plans that are
equal in **every** field. The dropped application is a duplicate.

**Where it is not harmless is a mixed pair.** An integer in the position
another activator gives a nested `Fact` puts two plans in one identity space
with two *different register layouts* — `?f` is register 1 in one and register
3 in the other — so the identity compares `(?R ?f ?a ?b)` against `(?R ?a ?b
?f)` position by position. A vector that is a legitimate match of both then
suppresses a firing whose conclusion nothing else derives:

```lisp
(relation edge  Node Node)
(relation holds Node)
(relation noted Node Node)

(rule note (?R ?f)
  :match  (and (?R ?a ?b) (holds ?f))
  :assert (noted ?a ?f))

(edge 1 2) (edge 2 3)
(holds 1)  (holds 3)

(note edge 1)                     ; an int  in the second position
(note edge (src Y))               ; a Fact  in the second position
```

```text
$ ein saturate q_m1a8.ein --dump          # both activators
;; ── DERIVED (3 facts) ──
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
  (noted 2 3 :rule note)

$ ein saturate q_m1a8.ein --dump          # with the `(note edge 1)` line deleted
;; ── DERIVED (4 facts) ──
  (noted 1 1 :rule note)
  (noted 1 3 :rule note)          ← lost above, silently
  (noted 2 1 :rule note)
  (noted 2 3 :rule note)
```

Adding an activator **removed** a conclusion, and swapping the two lines puts
it back. `Engine::check_layout` asserts against exactly this shape — under
`debug_assertions` only, so a release build is where the wrong answer lives.
Its doc comment called the shape *"a shape no rule application has"*; the
program above is thirteen lines.

*Why it is here rather than fixed.* This is
[Q-M1a.8](../history/m1a_rust/open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args),
closed 2026-08-29 as stated and refiled as its real self:
[Q-M1e.16](../../plans/m1e_review_processing/open_questions.md#q-m1e16--the-binding-key-compares-two-register-layouts-as-one).
It was reproduced from ein.py because it was current behaviour and the byte
gate would have flagged any change — and the misreading is ein.py's too:
`_binding_key`'s third component is `frozenset(bindings.items())`, and
`bindings` held the integer. It stays because **no corpus program can reach
it**: of the **153 143** activator arguments every plan compiled by `ein
solve -m 2` over all 204 `.ein` files under `examples/`, `stdlib/` and
`tests/` binds against — forks' plans included — **every one is a symbol**,
not one integer and not one nested `Fact`, and **no** `(rule, activator)`
space holds more than one plan (measured 2026-08-29). **It is the one item on this page that
is a latent bug rather than a quirk** — a program that puts an integer and a
nested `Fact` in one activator position loses a derivation, with no
diagnostic.

The program above is deliberately **not** a corpus fixture: `cargo test` builds
with `debug_assertions`, where it trips the assertion instead of answering.
Both halves are banked in `ein-infer/tests/rule_semantics.rs` instead, as
`an_int_beside_a_nested_fact_in_one_position_loses_a_derivation` — which
expects the panic in a debug build and the missing fact in a release one.

## 4. Errors and exit codes

The engine reports a failure it cannot diagnose by printing a Python
exception's last line and exiting non-zero:

| input | last line | exit |
|---|---|---|
| missing input file | `FileNotFoundError: [Errno 2] No such file or directory: '<path>'` | 1 |
| a rule that will not compile | `ein.inference.compile.CompileError: <message>` | 1 |
| unbound variable in `:assert` | `KeyError: "unbound var ?v1 in :assert — bindings: {…}"` | 1 |
| `:assert` head that is not a fact | `TypeError: <message>` | 1 |
| `--max-firings` budget exhausted | `ein.inference.saturator.SaturatorStepLimitError: <message>` | 1 |
| an argument the CLI rejects | `error: invalid value 'x' for '--solutions <N>': invalid int value: 'x'` | 2 |

`KeyError` quotes its message because CPython's `KeyError.__str__` is the
**repr** of the key, quotes and all.

The last row is the odd one out and belongs to §5 rather than here: it is
`clap`'s wording, not `argparse`'s, and it is the one diagnostic on this page
that was *deliberately not* reproduced. What is fixed about it is the **exit
code and the stream**, which is what a script can depend on.

*Why a Rust binary names Python classes.* Because these strings were the
oracle's observable and reproducing them is what
[Q-M1a.14](../history/m1a_rust/open_questions.md#q-m1a14--crash-parity) asked
for; each is pinned by a `crash-parity` corpus entry. **Now that there is no
Python, they are a name without a referent** — the strongest candidate on this
page for deliberate change, and the reason nothing has changed yet is that
every one of them is a checked-in expected output.

### 4.1 The first errors with no Python counterpart — M1c S1c.1.2

The paragraph above used to end *"a future ein.rs-only error with no Python
counterpart has no rule to follow and would decide this question by arriving;
none exists in the corpus."* Five arrived on 2026-08-23 with
[`:expect`](ir/03-ein-lang/01_grammar.md#query), and what they decided is:
**a diagnostic that never had a Python counterpart names no exception class.**
It is a `kb load error:` like any other loader message, in ein's own words.

| input | message | exit |
|---|---|---|
| a `(query …)` keyword outside the allow-list | `(query …): unknown keyword :<k> — one of :goal :goal-text :hrules :hypothesis-relations :no-hypothesis :expect :mode` | 1 |
| `:expect` that is not `(false)` / `(model …)` / `(or (model …) …)` | ``:expect <what> — expected `(false)`, `(model …)` or `(or (model …) …)` `` | 1 |
| `:expect` naming a relation the program does not have | `:expect names <r>, which no declaration or fact makes a relation` | 1 |
| `:expect` omitting a relation the `:goal` asks about | `:expect does not name <r>, which the query's :goal asks about` | 1 |
| a `?var` or `_` inside an `:expect` | ``:expect — `?v` is a variable; an expectation is an answer, not a pattern`` | 1 |

They accumulate and `; `-join with every other loader error, as §1.5's do, and
each is pinned by a fixture in
[`examples/broken/load/`](../../examples/broken/load/).

**And one non-error that is also new**: a query whose `:expect` is *false*
prints `:expect FAILED` with the disagreement and exits **1** — a result, not a
usage error, so it takes §4's code rather than §5's. `ein solve` on a file with
several `(query …)` blocks runs each and exits non-zero if any expectation
fails; the flags that name a single output path are refused on such a file with
exit **2**, which is §5's code because that one *is* a usage error.

A third label joins `holds` and `FAILED`: **`NOT CHECKED`**, for a claim a
stopped search neither confirmed nor refuted. It takes **1**, the same code a
false claim does, because a green line for a claim nobody checked is what the
whole form exists to prevent. Two things reach it — a run that stopped at `-n`,
and a lattice frontier still alive at `--max-set-size`, where `k = 0` means "no
model within the cap" rather than "no model".

### 4.2 The verdict atom's refusals — M1d S1d.2.3

Nine more with no Python counterpart, and by §4.1's rule they name no
exception class either. They all concern `open`, the reserved verdict atom
([`06_reserved_names.md` § the verdict atom](ir/03-ein-lang/06_reserved_names.md)),
and every one of them is a place the engine would otherwise have had to guess.
`<k>` is `rule` or `hrule`, `<n>` the rule's name.

| input | message | exit |
|---|---|---|
| `(open …)` in a `:match` | ``<k> '<n>': `(open …)` is a verdict about the KB and is legal only in :assert — the third-state probe for a fact is `(unknown …)` at <loc>`` | 1 |
| `(open …)` with two or more arguments | ``<k> '<n>': `open` takes the incomplete relation and nothing else — `(open)` or `(open ?R)`, not <n> arguments at <loc>`` | 1 |
| an `:assert` concluding `open` *and* something else | ``<k> '<n>': a rule asserting `open` may assert nothing else — it is read after the fixpoint, where a derivation would be too late at <loc>`` | 1 |
| `(open …)` nested inside a conclusion | ``<k> '<n>': `(open …)` is a whole conclusion, not a term inside one at <loc>`` | 1 |
| `open`'s argument is neither a variable nor a name | ``<k> '<n>': `open`'s argument names a relation — a rule parameter or a relation name at <loc>`` | 1 |
| `(open ?R)` where `?R` is not a parameter | ``<k> '<n>': `(open ?R)` names a relation the activator does not bind — `?R` is not in the parameter list at <loc>`` | 1 |
| no `(absent …)` holds a positive `?R` premise | ``<k> '<n>': `(open ?R)` needs an `(absent …)` in :match holding a positive `?R` premise — that premise is the witness the obligation owes at <loc>`` | 1 |
| two or more `(absent …)` do | ``<k> '<n>': <m> `(absent …)` guards hold a positive `?R` premise — which one states the obligation is not decidable at <loc>`` | 1 |
| the `?R` premise binds no variable of its own | ``<k> '<n>': `(open ?R)`'s `?R` premise binds no variable of its own, so the obligation is ground — that is a plain `absent` check and not something a witness could discharge at <loc>`` | 1 |
| two positive `?R` premises each bind one | ``<k> '<n>': <m> positive `?R` premises each bind a witness variable — a compound witness has no single slot to branch on at <loc>`` | 1 |

Four are pinned by fixtures in
[`examples/broken/load/`](../../examples/broken/load/) — `open_in_match`,
`open_arity`, `open_mixed_assert`, `open_compound_witness` — and all ten by
`ein-ir`'s `the_verdict_atom_refuses_every_shape_it_cannot_resolve`.
Binding the name is the eleventh and is not new: `(relation open …)`,
`(macro open …)` and `(rule open …)` take §1's existing *shadows a reserved
kernel name* message, because `open` joined `RESERVED`.

### 4.3 Kernel meta-primitive arity — M1e S1e.2.1

Two more, and they are the first on this page whose *predecessor was not a
message at all*. `(eq ?x)` used to reach the matcher and take the process down
with a `panicked at … assertion failed` and exit **101** — a refusal carrying a
stack trace, which is the one thing
`corpus_cli::every_refusal_carries_a_diagnostic` forbids — and `(eq ?x A B)` /
`(absent A B)` used to *fire*, having dropped every argument past the ones they
read. Both are `CompileError`s now, refused before a `Step::Guard` exists.

| input | message | exit |
|---|---|---|
| `eq` or `neq` at any arity but 2 | ``` `(eq …)` takes exactly 2 arguments and was given <n>: <repr> at <loc> — a built-in predicate compares exactly two values, so write one guard per pair. ``` | 1 |
| `absent` at any arity but 1 | ``` `(absent …)` takes exactly 1 argument and was given <n>: <repr> at <loc> — negation-as-failure is over a single premise, so wrap several in an `(and …)`. ``` | 1 |

They travel as `ein.inference.compile.CompileError: <message>` — §4's second
row — because that is the layer they are raised at, and by §4.1's rule the
messages themselves name no exception class. **`<loc>` is real**: unlike the
loader messages of §1.5, which end in `at None` because a top-level form
carries no position, a premise is a `generic_list` and the parser hands one a
`Loc`. So these two read `at Loc(file='…', line=…, col=…)`.

**Why only two rows for seven wrong cells.** The
[S1e.1.6 sweep](../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.6_coverage_gaps.md) found
the rule behind them:
[`00_ebnf.md` §2](ir/03-ein-lang/00_ebnf.md)'s *Kernel meta-primitives
(shape-pinned)* block names **four** productions and the engine has **seven**
primitives, so `eq`, `absent` and `false` are ordinary `GenericList`s whose
arity is checked by whatever reads them. `neq` is pinned and its wrong arities
have always been a positioned **parse** error; `false` is unpinned and needs no
row, being silent in a `:match` at every arity, which is what a reader expects.
That leaves `eq` and `absent`, one row each, at every arity but their own.

Four fixtures pin them in
[`examples/broken/compile/`](../../examples/broken/compile/README.md) —
`eq_arity_low`, `eq_arity_high`, `absent_arity_zero`, `absent_arity_high` —
and all 21 cells of the sweep, the fourteen that were right included, are
`ein-cli/tests/primitive_arity.rs`.

## 5. The CLI surface

Everything a script or a habit can depend on is fixed: the four subcommands
and four `render` sub-subcommands, the delegated dispatch (`ein saturate
--help` prints `saturate`'s own help; `saturate` appears in `ein --help`
though the top parser never parses it), every option's long name, short key,
metavar, arity, default, choices and mutually-exclusive group, its help
*string*, the accept/reject verdict on every invocation, the exit code, and
which stream each byte goes to.

**`ein test` is the fourth**, added by M1c
[S1c.1.3](../history/m1c_external_validation/README.md#s1c13--ein-test),
and it is where the exit codes of §4 are **deliberately not** ein.py's:

| code | `solve` / `render` / `saturate` | `test` |
|---|---|---|
| 0 | success | every expectation held, and at least one was checked |
| 1 | a load error — or a false `:expect` | an expectation is **false**, or was not checked |
| 2 | a usage error, or a budget abort | a load error, a usage error, a budget abort — or nothing to check |

1 is taken there, and a runner that cannot tell a broken file from a false
claim is the one failure mode a test runner must not have. 2 dominates 1 in a
multi-file run for the same reason: with one file unloaded, "every expectation
was checked and some are false" would be a false description of the run.
`test` has no ein.py counterpart to diverge from — it is the second such
subcommand, after `ein kb`.

**M1d [S1d.2.6](../../docs/history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus)
added a fourth verdict word and no exit code.** `Open` — a consistent,
quiescent state with an obligation the program stated still unwitnessed —
exits **0**, exactly as `Solution`, `Ambiguity` and `Contradiction` do: the
verdict channel and the *claim* channel are separate, and the claim channel is
`:expect`. A program whose expectation holds against an open state exits 0 and
one whose expectation is false exits 1, both for the reasons they already did.
`:expect` did not grow a word for the verdict, so no claim can assert openness;
all three of its forms are assertions about **facts**, and an `open` conclusion
is by construction never a fact. Twelve corpus entries changed word and **zero**
changed exit code.

**M1d [S1d.3.3](../../docs/history/m1d_satisfiability/README.md#s1d33--the-verdict)
added the 49th option and one rendering rule.** `solve --models {list,key}`
chooses the projection of a model **set** that goes to stdout — the blocks, or
the smallest set of slots that tells the models apart and the table of
combinations that occur. It is read by the `Ambiguity` arm alone, defaults to
`list`, and reaches nothing recorded: the verdict, every counter, the JSON
summary, the event stream, `-p` and `:expect` are byte-identical under either
value.

The rule beside it is **normative and about what a count may claim**:

| the search | what a report of the model set may say |
|---|---|
| `exhausted = true` | *these are the models* — `solutions (k) 9`, `Ambiguous — distinct complete models` |
| `exhausted = false` | *these are models **found*** — `solutions (k) 5   (a lower bound — the search did not exhaust)`, `Ambiguous — distinct complete models found` |

`Solution` has carried the same distinction since ein.py, as *"(not certified —
pass --exhaustive)"*. `Ambiguity` did not, and the difference matters more
there: an unqualified `k = 1` is a guess about uniqueness, where an unqualified
`k = 5` on a file with nine models is **wrong**.

**M1d [T1d.10.5.2b](../../docs/history/m1d_satisfiability/README.md#s1d105--what-exhausted-means)
extended the rule to the empty model set, which is where it was worth most.**
A `k = 0` is a claim like any other and `exhausted` is what licenses it, so the
table above now has four rows rather than two:

| verdict | `exhausted = true` | `exhausted = false` |
|---|---|---|
| `Solution` | `Solution` | `(not certified — pass --exhaustive)` |
| `Ambiguity` | *these are the models* | *models **found***, `(a lower bound …)` |
| `Contradiction` | *No solution — the constraints are contradictory* | *No model found — the search did not exhaust the lattice*, `(none found …)` |
| `Open` | *the requirement is unmet, not refuted* | *…and the search did not exhaust* |

The fixture is `examples/saturation/type-exclusivity/pets.ein`, and it is why
this is a defect and not a nicety: it said **the constraints are
contradictory** at `-m 5` and `-m 8`, and it has **35 models** at `-m 10`. The
claim channel had refused to be fooled by the same run since M1c —
`:expect (false)` there comes back `NOT CHECKED`, not `FAILED`, because
`expect.rs` will not settle a claim on a stopped search — so the two channels
were saying *refuted* and *nobody knows* about one solve. They agree now.

**The unsat core is renamed with it**, for the same reason and in the same
breath: an unsat core explains why a program has **no model**, which a
truncated run has not shown. Its block header is `refuted so far (n facts)`
when `exhausted = false` — which is also what makes the empty one legible,
where `unsat core (0 facts)` read as *the empty set is contradictory* rather
than *nothing died*.

**Words moved; nothing else did.** 26 corpus cells across 13 files change what
they print, all of them `Contradiction` at `exhausted = false`; the 48
exhausted `Contradiction` cells keep every word; **no exit code, no counter, no
`--json-summary` field and no `:expect` outcome moves**, and `verdict.type`
stays `Contradiction`, which is the same shape S1d.3.3 held to. `Open` at
`exhausted = false` is unreachable on today's corpus — every `Open` in it is
exhausted — so that row is written to be right rather than because a cell moved.

What remains of [Q-M1d.1](../../docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)
after this is the *stopping* half, not the vocabulary half: whether a search
may stop early on purpose, and what would license it. The word it used to be
about is settled: a refutation said under a depth cap is not a refutation, and
it no longer says it is.

**M1d [S1d.4.1](../../docs/history/m1d_satisfiability/README.md#s1d41--what-closure-costs)
added the 50th option and no behaviour at all.** `ein test --json-report
FILE.json` writes **one row per `(query …)` of the whole selection** — the
claim's shape (`model` / `or` / `false`), how many models it lists, the
relations its `:goal` closes, the outcome, and what the run found. It is
additive in the strict sense the other artefact flags are — stdout, stderr, the
exit code and *what is solved* are identical with it and without it, so a query
stating no `:expect` is still never solved and its row says so.

Two things about it are contract rather than convenience:

- **It takes any selection**, where `--events` and `--json-summary` are refused
  over more than one run. Those name one *path* for one *run*; a report has no
  run to be more than one of, and one invocation over the three corpus roots is
  what the census reads.
- **A file that did not load carries no claim.** Its row is `queries = 0`,
  `outcome = "error"`, `expect = null`. Three `examples/broken/load/` fixtures
  contain the token `:expect` and exist to be refused; a claim is a property of
  a *program*, so they are not in the numerator of "what fraction of the corpus
  claims a model set".

The schema is `ein-test-report/1`, deliberately not `ein-summary/1`: a summary
is one run's counters, a report is one row per query, and a consumer reading
the same version marker on both would be right to expect the same fields.

**M1d [S1d.4.3](../../docs/history/m1d_satisfiability/the_vocabulary.md)
moved no byte of stdout and added one line to stderr.** A `:expect` that does
not hold under `ein solve` now writes

```text
<file>: :expect NOT CHECKED — expected Ambiguity with k = 2, got Solution with k = 1
```

to **stderr**, beside the unchanged report under the solution table. The rule
it settles is which stream carries what, and both halves are the answer:

| | stream | why |
|---|---|---|
| the `:expect` block — label, disagreements, the derivation that put a surplus fact there, the models projected through the `:goal` | **stdout** | it is what the run *found*. A false claim is a **result**, not a refusal of the input, and its report belongs under the table it is about |
| one line naming the file, the label and the first disagreement | **stderr** | an exit 1 with an empty stderr is a run nobody can diagnose from a pipeline |

Before it, `solve` produced exactly that undiagnosable shape, which is why
`examples/features/11_expect_ambiguity.ein` — then the corpus's only fixture
that reached `Outcome::NotChecked` under a declared run, joined at M1e S1e.3.1
by `examples/ein-bugs/complete-records-stale.ein`, whose claim is a `Solution`
and so is checkable only when the search exhausts — **could not declare plain
`solve`**: `corpus_cli::every_refusal_carries_a_diagnostic` forbids it. Both
declare it now, and `corpus_exits.txt` banks the 1. `ein test` is unchanged:
it prints its own per-file report and has always been readable on stdout alone.

**M1d [T1d.10.5.0](../../docs/history/m1d_satisfiability/README.md#s1d105--what-exhausted-means)
closed the one door in that rule with no guard on it.** `--max-set-size 0` is a
**truncation**: a run that explores no layer over a non-empty frontier reports
`exhausted = false`, and the table above then applies to it unchanged — its
`k = 0` means *no model within the cap*, never *no model*. Before it the layer
loop `1..=max_set_size` never ran at zero, so `truncated` was never set and
`exhausted` kept its `true` default; **51 of the 150 corpus entries that load —
every one that reaches the search** — stated a refutation with an empty unsat
core, a certified exhaustion claim and a success exit code.

Two things the fix decides rather than assumes:

- **A cap of zero answers; it does not refuse.** A program whose root is
  already complete has no lattice to exhaust and reports its verdict exactly at
  a cap of zero — `Solution` on `examples/branching/01_saturate_only.ein`,
  `Open — owes 1` on `tests/stdlib/algebra/23_total_owed.ein`, both
  `exhausted = true`. The other **99** entries are that class and not one of
  them moved. The alternative — the `Aborted` shape `--max-enterings 0` uses —
  would decline a question the engine answers exactly, and P1d.10's own
  reconnaissance asks it once per node.
- **An empty `unsat_core` on a `Contradiction` stays constructible.** A search
  that entered nothing has no dead commitment to cite, and the emptiness is not
  this cap's shape to begin with: **12** corpus entries report `Contradiction`
  with a 0-fact core under their ordinary `solve` run, every one of them at
  `exhausted = false`. Whether that pair may keep the *word* is
  [Q-M1d.1](../../docs/history/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
  and is untouched here — no exit code moved, and `corpus_exits.txt` is
  unchanged.

**M1e [S1e.1.5](../../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.5_cli_semantics.md)
answered the same question for the other budget flag, and answered it the other
way.** `--solutions 0` is **refused**, exit 2:

```text
$ ein solve x.ein -n=0
error: invalid value '0' for '--solutions <N>': invalid solution count: '0' (expected 1 or more, or --exhaustive)
```

The asymmetry with `--max-set-size 0` above is the point, and it is not a
preference. A cap of zero *has* an answer the engine gives exactly — explore no
layer, and say so with `exhausted = false`. A stop-after of zero has none:
`stop_after` is compared with `>=` **after** a model is recorded, so `Some(0)`
cuts at the **first** model and `-n 0` was `-n 1` under another name, measured
byte-identical on stdout and on every `--json-summary` field across a unique
model, a nine-model ambiguity and a contradiction. Neither reading a user could
mean was available — *record nothing* is not what it did, and *no limit* is
already spelled `--exhaustive` — which is the argument `--jobs 0` had been
refused with since [S1a.7.5](../history/m1a_rust/README.md#s1a75--the---jobs-contract).

Three things it settles that are easy to state loosely:

- **Every negative goes with it.** The CLI clamped `-n` to zero with `max(0)`,
  so `--solutions=-7` was `-n 1` too. `-n -7` with a space was already `clap`'s
  *unexpected argument*; the `=` spelling was the open door.
- **A non-integer keeps its message.** `error: invalid value 'x' for
  '--solutions <N>': invalid int value: 'x'` is §4's own example row, so the
  validator refuses the *range* and leaves the *type* alone.
- **It is a deliberate divergence from ein.py, not a repair of one.** ein.py
  declared `-n` as `type=int` with no bound and compared
  `len(lstate.solution_nodes) >= stop_after`, so it accepted zero and behaved
  the same way. Nothing pins that: no golden under
  `tests/golden/from_ein_py/` is a `solve` invocation — the nineteen are two
  canonical parse dumps and seventeen renderings — and every `-n` in the corpus
  is `-n 3`. Held by
  `ein-cli/tests/cli_semantics.rs::solutions_takes_a_count_of_one_or_more_and_nothing_else`.

**M1d P1d.10 added the 51st option, and it moves no byte of stdout.** `ein
solve --layer-progress` streams the **per-layer census** to stderr and nothing
per entering — three lines a layer, where `--verbose` had one and a half:

```text
  layer 3: alive=96 root_facts=339
  layer 3 gen:  frontier=2911 joined=60260 −dead=0 −clause=16171 (26.8%) cand=44089
  layer 3 test: entered=44089 alive=33940 complete=7256 models=4 dead=10149 dead_pre=0 …
  layer 3 done: survivors=26684 enterings=48745 solution-nodes=32
```

The numbers are the sixteen the `layer` event already carried; what is new is
that they reach a reader who is *watching a run* rather than parsing a JSONL
file afterwards. Three identities hold per layer and are a test
(`cli_semantics::the_layer_progress_rows_add_up`):

| identity | which half of the loop it accounts for |
|---|---|
| `joined − dropped_dead − dropped_clause = cand` | generation: what the prefix join proposed and what the learned clauses took off it |
| `entered = alive + dead` | testing |
| `alive = complete + survivors` | where the consistent forks went |

The third is the one no counter stated before. `alive_enterings` counts every
consistent fork, and only the **incomplete** ones reach the next frontier — so
`complete` is this layer's solution-outcome enterings and `models` is what they
collapse to under `state_key`. On `examples/zebra2.ein` layer 1 that is
**13 → 1**: thirteen commitments each complete the puzzle, and all thirteen are
the same model.

`--verbose` still prints the per-entering line and now prints these rows too;
`--layer-progress` is the same dumper with the entering firehose silenced,
which is what makes it usable on a run that enters 618 076 times (6 180 lines
at the default `--progress-every`). Both cost what `-v` costs — the dumper
reads each fork for the `state_key` dedup, so neither is free — and both are
additive: stdout, the exit code and every counter are identical with them and
without.

**Free, and different from the Python CLI's:** wrapping, indentation,
headings, ordering within a section, and the wording of a usage diagnosis.
`clap` cannot be configured into `argparse`'s layout and hand-rolling one was
priced and declined
([Q-M1a.13](../history/m1a_rust/open_questions.md#q-m1a13--argparse-surface-parity)).
The structure — `{subcommand → {option → short, metavar, arity, default,
choices, group, help}}` — is what the gate checks, which catches a lost option
on its own line rather than inside an 89-line text blob.

## 6. What is *not* defined, and is filed

One rendering on this boundary is genuinely **under-determined**, and saying
so is the point of listing it here:

- **The goal row a solve table prints.** If a model satisfies `(query :goal
  …)` more than once, the table prints the first row of an *unsorted* match,
  and which row that is depends on the order facts entered the KB. It moves
  under a permuted id space in a single engine, one build — so it is a
  [design/02](../history/m1a_rust/design/02_determinism_and_order.md)
  violation in its own right and not a consequence of anything else.
  `summary.json`'s `goal_bindings` carries **all** rows and sorts them, which
  is why no counter and no JSON field can see this. The fix is available and
  is a decision, not a repair — print the lex-smallest row, at the cost of one
  visible change to a checked-in fixture. Reproducer and notes:
  [`corpus/fuzz_findings/`](../../corpus/fuzz_findings/README.md).

## See also

- [`../../docs/history/m1a_rust/divergences.md`](../history/m1a_rust/divergences.md)
  — where the two engines differed, and why each difference was accepted.
- [`../../docs/history/m1a_rust/open_questions.md`](../history/m1a_rust/open_questions.md)
  — Q-M1a.3 / .4 / .5 / .6 / .8 / .13 / .14 / .15, the resolutions this page
  restates.
- [`inference/reserved_engine_strings.md`](inference/reserved_engine_strings.md)
  — the other kind of thing that is defined only because the engine does it.
- [`inference/implementation.md`](inference/implementation.md) — where each
  behaviour above is implemented.
- [`standard_of_proof.md`](standard_of_proof.md) — its opposite number: this
  page states what the engine **does**, that one states what it takes to know
  it, and what an argument for leaving a behaviour alone has to rest on.
