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

### 3.2 A rule application's identity ignores non-string activator arguments

The identity of a firing is `(rule, activator, bindings)`, and the activator
half keeps only the activator's **string** arguments — while the *compile*
cache key stringifies all of them. Two activators differing only in an integer
argument therefore share an identity, and can suppress each other's firings.

*Why it is here rather than fixed.* This is
[Q-M1a.8](../history/m1a_rust/open_questions.md#q-m1a8--_binding_key-drops-non-string-activator-args),
and it is almost certainly unintended. It was reproduced because it was
current behaviour and the byte gate would have flagged any change; it stays
because nothing in the corpus reaches it and changing it moves firing counts
everywhere. **It is the one item on this page that is a latent bug rather than
a quirk**: a puzzle whose rule parameters are integers can lose a firing, with
no diagnostic.

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

**M1d [S1d.2.6](../../plans/m1d_satisfiability/p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md)
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

**M1d [S1d.3.3](../../plans/m1d_satisfiability/p1d.3_model_sets/s1d.3.3_the_verdict.md)
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
`k = 5` on a file with nine models is **wrong**. `Contradiction` is
deliberately **not** covered — a refutation said under a depth cap is
[Q-M1d.1](../../plans/m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
question about a *word*, not this one about a *count*.

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
