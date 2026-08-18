# Open Questions — M1a (Rust port)

Milestone-scoped questions. Ids are **sticky** — `Q-M1a.<n>`, following
the `Q-S1.5a.6.B` style used inside M1 stages rather than the global
`Q<n>` sequence in [`plans/open_questions.md`](../open_questions.md), so
the two namespaces cannot collide. A closed id is never reused.

## Index

| Q | title | status |
|---|---|---|
| [Q-M1a.1](#q-m1a1--port-boundary-a-full-vs-b-hot-loop) | Port boundary — A (full) vs B (hot loop behind PyO3) | **resolved 2026-08-17 — A** |
| [Q-M1a.2](#q-m1a2--does-einpy-have-a-sunset) | Does ein.py have a sunset? | open — recommendation: no |
| [Q-M1a.3](#q-m1a3--parse-error-message-parity) | Parse-error message parity, including `-1:-1` at EOF | **resolved 2026-08-18 — (a)** |
| [Q-M1a.4](#q-m1a4--sorted-over-mixed-type-fact-args) | `sorted()` over mixed-type fact args raises in ein.py | **resolved 2026-08-18 — (a), [D2](divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)** |
| [Q-M1a.5](#q-m1a5--reproducing-cpythons-shuffle) | Reproducing CPython's `random.shuffle` for `--shuffle` | **resolved 2026-08-18 — (a), ported** |
| [Q-M1a.6](#q-m1a6--at-none-in-loader-messages) | `at None` in loader messages (top-level forms carry no `loc`) | open — post-parity fix; reproduced at P1a.1 |
| [Q-M1a.7](#q-m1a7--may---jobs--1-move-counters) | May `--jobs > 1` move counters? | open — recommendation: no, plus an opt-in escape |
| [Q-M1a.8](#q-m1a8--_binding_key-drops-non-string-activator-args) | `_binding_key` drops non-string activator args | open — port as-is, flag upstream |
| [Q-M1a.9](#q-m1a9--where-do-goldens-live) | Where do goldens live? | open — decide at the P1a.5 gate |
| [Q-M1a.10](#q-m1a10--does-f11-d1-beta-memories-land-inside-m1a) | Does F11 D1 (beta-memories) land inside M1a? | open — gated on measurement |
| [Q-M1a.11](#q-m1a11--server-wire-protocol) | Server wire protocol — JSON-RPC vs gRPC vs bespoke | **closed moot 2026-08-18 — no server** |
| [Q-M1a.12](#q-m1a12--remote-access-and-auth) | Remote access and auth for `ein serve` | **closed moot 2026-08-18 — no server** |
| [Q-M1a.13](#q-m1a13--argparse-surface-parity) | Reproducing `argparse` `--help` and error text | **resolved 2026-08-18 — (b): behaviour exact, presentation normalised** |
| [Q-M1a.14](#q-m1a14--crash-parity) | Crash parity — inputs where ein.py raises an unhandled exception | **mostly resolved 2026-08-18 — ein.rs names the class** |
| [Q-M1a.15](#q-m1a15--float-formatting-parity) | Float formatting parity in reported numbers | **resolved 2026-08-18 — `pyfmt` landed** |
| [Q-M1a.16](#q-m1a16--how-does-the-harness-drive-the-lever-matrix) | How does the harness drive the `SolverConfig` lever matrix? | open — found at S1a.0.1 |
| [Q-M1a.17](#q-m1a17--win-bs-80--assumed-monotone-guards-dominate) | Win B's ≥ 80 % assumed monotone guards dominate — at root scale they are 11–30 % | open — found at S1a.3.4, measured |
| [Q-M1a.18](#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint) | May a fork stop re-narrating the root's fixpoint? | open — found at S1a.6.9, measured |

---

## Q-M1a.1 — Port boundary: A (full) vs B (hot loop)

**Resolved 2026-08-17: A.** The placeholder deferred this; the milestone
brief settles it — ein.rs re-implements the whole stack with a 1:1
surface, and PyO3 becomes an *output* ([P1a.9](p1a.9_bindings_release/README.md))
rather than the boundary. Boundary B's advantage was preserving M1's
tooling without re-implementation; the parity harness
([design/01](design/01_parity_contract.md)) buys that back more cheaply
than an FFI seam through the hottest loop in the engine would have.

## Q-M1a.2 — Does ein.py have a sunset?

Once ein.rs is the shipping engine, ein.py is (a) the parity oracle,
(b) the reference implementation for M2 experiments, and (c) the
"Python users get a working solver" fallback. Keeping it green costs CI
time and every semantic change has to land twice.

**Recommendation: no sunset.** The oracle is what makes the port
falsifiable, and a second implementation of a research kernel is a
feature, not debt. Revisit only if double-landing becomes the dominant
cost of a semantic change — and note that M1 is *shipped*, so semantic
changes should be rare.

## Q-M1a.3 — Parse-error message parity

ein.py wraps Lark's `UnexpectedInput` as
`{file}:{line}:{col}: unexpected input\n{context}` where `context` is
`e.get_context(text)`. Observed quirks: EOF errors report `-1:-1`, and
the caret lands one past the last token
([design/04](design/04_ir_frontend.md) §4).

Options: (a) reproduce exactly, quirks included; (b) reproduce for the
non-EOF cases and accept a ledger entry for EOF; (c) improve both
implementations together, re-baselining the four `examples/broken/`
fixtures.

**Recommendation: (a) for the port, then (c) as a separate, deliberate
change once T3 is green** — improving diagnostics while the harness is
still finding bugs would hide regressions in noise.

**Resolved 2026-08-18 at [S1a.1.1](p1a.1_ir_frontend/s1a.1.1_lexer_and_parser.md):
(a), and it needed more than the EOF case.** Four behaviours had to be
reproduced, and only the first was known when this question was written:

1. **`-1:-1` at EOF**, with `get_context` rendering the last line and a
   caret one past its end — Lark's `UnexpectedEOF` sets
   `pos_in_stream = -1` and Python's negative slicing does the rest.
2. **A ±40-character context window**, applied *before* the line is
   trimmed, so an error past column 40 renders a **truncated** source
   line.
3. **The `%ignore` delayed-match quirk**: `xearley.py` writes a
   `delayed_matches[m.end()]` key at every position where whitespace or a
   comment matches — including inside a string literal, and including
   when `to_scan` is empty, which still creates the key in a
   `defaultdict`. A dict holding one empty list is truthy, so the error
   is held back until the scanner walks past it. `(y";"{` reports the
   `{`; `(y";"{?` reports the `?`. Found by the differential fuzzer, not
   by reading, and simulated in `parse::death_position`.
4. **Ambiguity resolution prefers the earlier alternative**, which is
   what makes `(rulex …)` a rule named `x` rather than a fact named
   `rulex`.

All four are pinned by `parse_parity.rs` and by 2.2 M fuzzer mutations.
The (c) half — improving the diagnostics in both implementations
together — stays open and belongs after the P1a.5 byte gate.

## Q-M1a.4 — `sorted()` over mixed-type fact args

`apriori.layer_1` does `sorted(alive)` over `(relation, args)` tuples;
if two facts of the same relation have `str` in a slot for one and `int`
for the other, CPython raises `TypeError`. `canon.state_key` deliberately
avoids this with `key=repr`; `apriori` does not.
([design/02](design/02_determinism_and_order.md) §5 H2.)

ein.rs's `Value` is totally ordered and cannot raise. So on such an
input the two implementations *must* differ: one crashes, one answers.

Options: (a) accept the divergence with a fixture pinning both
behaviours; (b) fix ein.py to sort by `repr` here, re-baselining every
affected candidate order; (c) reject such inputs at load time in both.

**Recommendation: (a)**, unless a real puzzle needs mixed slot types —
then (b), because a crash is not a semantics anyone wants to preserve.

**S1a.0.1 — reproduced, and the scope is narrower than it looked.** Blind
hypothesis generation *cannot* reach it: `hypgen._raw_candidates` builds
candidates out of `kb.names`, and `store.rebuild_indexes` only enters an
arg into that index `if isinstance(a, str)` — so every blind candidate is
all-strings. Only an `hrule` can carry a non-string through, because its
`:assert` args come from bindings. The reproducer is therefore one hrule,
one variable, and two facts binding it to `1` and to `left`:
[`examples/ein-bugs/mixed-type-hypothesis.ein`](../../examples/ein-bugs/mixed-type-hypothesis.ein),
pinned by `ein.py/tests/inference/test_mixed_type_hypothesis.py` (which
also pins the scope claim, so a future change that lets blind hypgen emit
a non-string arg re-opens this question by failing).

That strengthens (a): no puzzle without an hrule can hit this, and (b)
would re-baseline every candidate order in the corpus to fix an input
nobody has written.

**The comparator (a) needs landed at
[S1a.2.1](p1a.2_kb_core/s1a.2.1_interner_and_values.md)**:
`Terms::cmp_semantic` orders `Int < Sym < Fact` across tags, as H2
recommends, and within a tag by the interner's rank table or by numeric
value at any width. `Value` deliberately has no `Ord`, so the identity
order cannot reach a sort site by accident.

**Resolved 2026-08-18 at [S1a.4.3](p1a.4_search_layer/s1a.4.3_apriori_and_nogoods.md)
— (a), and the behaviour is now reachable rather than argued.** The
`lattice-shape` diff runs `layer_1` over every corpus file's alive set;
exactly one file diverges, exactly the predicted one, and the port
answers `[{(seat Ann 1)}, {(seat Ann left)}]` where ein.py raises. The
ledger entry is [D2](divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers),
and the parity sweep **asserts** the divergence rather than tolerating
it, so a file that stopped diverging fails as loudly as one that
started.

## Q-M1a.5 — Reproducing CPython's `shuffle`

`--shuffle` seeds `random.Random(seed)` and shuffles each layer's
candidates, carrying RNG state across layers.

Options: (a) port MT19937 seeding + `random.shuffle` +
`_randbelow_with_getrandbits` (~60 lines, table-tested against
CPython output) and keep T3 everywhere; (b) declare shuffled runs
T0-only, on the grounds that shuffle-invariance is the point.

**Recommendation: (a).** It is cheap, it is testable, and `--shuffle`
runs are exactly the ones where a silent ordering difference would be
easiest to dismiss.

**Resolved 2026-08-18 at [S1a.4.5](p1a.4_search_layer/s1a.4.5_solve_loop.md)
— (a), and it took about the size the recommendation guessed.**
`ein-infer/src/mt19937.rs` is CPython's `_randommodule.c`: the twister,
`init_by_array` seeding (absolute value, split into 32-bit words —
`Random(-7)` and `Random(7)` are the same generator), `getrandbits`
including its multi-word path, `_randbelow`'s rejection loop, and
`shuffle`'s downward Fisher–Yates.

It is checked twice. A **table** against CPython 3.14 — the first three
words for four seeds, one of them wider than a word and one negative,
plus a two-shuffle sequence that pins the state carrying across calls.
And on **real data**: `solve-shape`'s third regime runs every corpus
entry with `lattice_order_seed = 7` and compares the whole `enter`
sequence — **65 files, 5 207 enterings, 0 differences** — where the
traversal differs from the unshuffled one on 9 of the 14
`examples/branching` files, so the generator is doing something rather
than agreeing by inertia.

## Q-M1a.6 — `at None` in loader messages

Top-level `SForm`s are constructed without a `loc`
([design/04](design/04_ir_frontend.md) §3), so loader errors that
interpolate `at {form.loc}` print `at None`. ein.rs has the position and
would naturally print it.

**Recommendation: print `at None` during the port (T3), then fix both
implementations together** in a post-parity stage. Tracked here so the
fix is not forgotten; it is a genuine usability bug.

**Reproduced at [S1a.1.3](p1a.1_ir_frontend/s1a.1.3_macros_and_imports.md)**:
`ast::loc_repr` renders `None` for a top-level form and Python's
dataclass `repr` (`Loc(file='…', line=6, col=20)`) otherwise, which is
what makes the eleven `examples/broken/load/import_*.expected` messages
byte-identical — every one of them ends `at None`. The fix, when it
comes, re-baselines all of them in both implementations at once.

**Confirmed across the whole loader at
[P1a.2](p1a.2_kb_core/README.md)**: all eighteen remaining
`examples/broken/load/` messages end `at None` too, and the one that does
*not* — `macro_arity_mismatch`, whose error is raised inside macro
expansion on a nested node — carries a real `Loc`. So the re-baseline is
exactly "every loader message except one".

## Q-M1a.7 — May `--jobs > 1` move counters?

[design/08](design/08_parallelism.md) commits to deterministic parallel
execution (same counters, same output) via speculate-and-validate, with
`--unordered` as an opt-in that relaxes to T0.

The open part is whether the validation cost is acceptable in the regimes
that matter (a large no-good store with frequent singleton writebacks).
Measure the re-validation rate in [P1a.7](p1a.7_parallelism/README.md); if
it is high, the fallback is to make `--unordered` the documented
recommendation for large searches rather than to weaken the default.

## Q-M1a.8 — `_binding_key` drops non-string activator args

`Saturator._binding_key` uses `plan.activator_args`, which
`compile_rule` builds as `tuple(a for a in activator.args if
isinstance(a, str))` — while the *plan cache* key stringifies **all**
args. Two activators differing only in an `int` arg therefore share a
binding key and can suppress each other's firings.

Almost certainly unintended. **Port as-is** (it is current behaviour and
T2 would flag any change), and open an ein.py issue with a fixture that
demonstrates it. Fix both together, after parity.

## Q-M1a.9 — Where do goldens live?

`ein.py/tests/golden/**` holds cross-implementation artefacts inside a
Python-specific tree ([design/11](design/11_shared_assets.md) §5). Read
in place, or promote to repo-root `testdata/golden/`?

**Recommendation: read in place until the [P1a.5](p1a.5_presentation/README.md)
gate; promote when ein.rs starts producing goldens too.**

## Q-M1a.10 — Does F11 D1 (beta-memories) land inside M1a?

[F11](../followups/f11_deductive_layer_perf.md) parks RETE beta-memories
on a fork-state design problem that [design/03](design/03_data_model.md)
§5 dissolves. [design/05](design/05_matcher.md) §7 sketches the answer,
and [P1a.6](p1a.6_performance/README.md) schedules it.

Open: whether it is still the largest lever *after* the register matcher
and the semi-naive boundary land. It may not be — those two remove the
costs that made partial-join recomputation expensive. **Decide by
profile, not by plan**; if it is a wash, revert it and leave F11 open,
exactly as P1.8a's D3 was handled.

## Q-M1a.11 — Server wire protocol

**Closed moot 2026-08-18: there is no server.** The question was to be
decided at P1a.8 kickoff "informed by what M1b picks for its stack" —
and that is exactly what dissolved it. M1b picked Tauri
([M1b § Stack](../m1b_gui/README.md#stack)), whose backend is a Rust
process linking `ein-core`/`ein-ir`/`ein-infer` directly; a wire protocol
between the GUI and the engine would have been a serialisation boundary
inside one process. With M2 crossing into CPython through PyO3
([P1a.9](p1a.9_bindings_release/README.md)) and the CLI running in-process,
no consumer was left. The JSON-RPC recommendation and the rest of
`design/09` are in git history if a hosted use case ever revives them.

## Q-M1a.12 — Remote access and auth

**Closed moot 2026-08-18 with [Q-M1a.11](#q-m1a11--server-wire-protocol).**
There is nothing to expose: the engine is a library and a CLI. If hosted
use is ever wanted, the posture recorded here still holds as a starting
point — a reverse proxy plus a token in front of a purpose-built service,
never an auth system inside the engine.

## Q-M1a.13 — `argparse` surface parity

**Resolved 2026-08-18: (b), with (c)'s content half made binding.** ein.rs
uses `clap`; `--help` layout *and* usage-error text go on the
[normalisation list](design/01_parity_contract.md) §5. Everything a script
or a habit can depend on stays exact — the difference is presentation, and
only presentation.

T3 includes `--help` output and CLI error messages. `argparse` has a very
specific layout (usage line wrapping, `options:` heading, metavar
rendering, two-space indent) and its own error text
(`argument -n/--solutions: invalid int value: 'x'`). `clap` does not
match it and cannot be configured to.

The options were: (a) hand-roll the argument parser and the help renderer
to match `argparse` byte-for-byte; (b) use `clap` and put
`--help`/CLI-error text on the normalisation list; (c) match the
*semantics* (flags, defaults, mutual exclusion, exit codes) exactly and
accept different help text.

### What stays exact

- The three subcommands, the four `render` sub-subcommands, and the
  delegated dispatch — `ein saturate --help` prints `saturate`'s own help
  under `prog="ein saturate"`, and `saturate` still appears in
  `ein --help` though the top parser never parses it.
- Every option at every level: long name, short key, metavar, arity,
  default, `choices`, mutually-exclusive group — and its help *string*,
  which is content, not layout.
- The accept/reject verdict on every invocation, and the exit code.
- Which stream each byte goes to.

Free: wrapping, indentation, headings, ordering within a section, and the
wording of a diagnosis.

### Why not (a)

The two halves are not separable. `argparse` welds its wrapped `usage:`
block onto *every* error, so exempting the layout exempts the message —
measured 2026-08-18:

    $ ein solve examples/zebra.ein -n x
    usage: ein solve [-h] [-n N | -e] [-m MAX_SET_SIZE] [-T MAX_TIME]
                     [-E MAX_ENTERINGS] [-L] [-K] [-o {lex,score-sum}] [-y] [-z]
                     [-d SEED] [-v] [-g PROGRESS_EVERY] [-D DIR] [-c] [-H] [-t]
                     [-s] [-p] [-P] [-f] [--events FILE.jsonl]
                     [--events-level {normal,verbose}] [--json-summary FILE.json]
                     [-r FILE.md] [-G] [-F] [-R] [-l]
                     file
    ein solve: error: argument -n/--solutions: invalid int value: 'x'
    → exit 2

A byte-exact error therefore needs argparse's usage formatter, which is
most of what (a) was priced at. The middle option — reproduce the
`ein solve: error: …` diagnosis line and drop the usage block — was
offered and declined: half a formatter for a line nothing reads
mechanically.

### What replaces the byte diff

The byte comparison of `--help` was the only thing checking that ein.rs
had not silently *lost* an option, so it is replaced rather than dropped.
Both engines' help is parsed into a structure —
`{subcommand → {option → short, metavar, arity, default, choices, group,
help}}` — and the structures are diffed. On the property that matters
this is *stronger* than the byte diff: a renamed short key or a changed
default fails on its own line, instead of somewhere inside an 89-line
text blob. Same instrument shape as
[S1a.5.3](p1a.5_presentation/s1a.5.3_state_dumps.md)'s `dump-shape` —
when there is no line protocol to diff over, render one.

### What would make this unacceptable

A consumer that reads `ein --help` or matches on ein's stderr text. There
is none as of 2026-08-18: no script under `utils/` parses either, and
`feature_matrix.py` only *echoes* a failing child's stderr into a report
field. The day one is written, this is the entry to revisit.

## Q-M1a.14 — Crash parity

Some inputs make ein.py raise an unhandled exception (Q-M1a.4's
`TypeError`; a `KeyError` from an unbound `:assert` var is *caught*
nowhere and surfaces as a traceback). ein.rs will not have Python
tracebacks.

Proposal: the harness compares **exit code + the first line of stderr**
for crash cases and records them as a distinct corpus group
(`crash-parity`), with the traceback body normalised away. Any input in
that group is also a candidate ein.py bug report.

**S1a.0.1 — the first-stderr-line half is wrong; implemented as exit code
+ exception class.** The first `crash-parity` fixture (Q-M1a.4's
`mixed-type-hypothesis.ein`) raises `TypeError: '<' not supported between
instances of 'int' and 'str'` — and *which operand is named first*
depends on the `frozenset` iteration order inside `sorted`, so ein.py
alternates between two messages across `PYTHONHASHSEED` values. A rule
that compares that line makes the determinism sweep fail on a difference
that is not one. `tier::compare_crash` therefore takes the exception
class off the last traceback line and drops the message body.

**A second fixture, from the CLI surface itself (found 2026-08-18, while
resolving Q-M1a.13).** A missing input file is not an argument error:
`cli/_common._parse_or_exit` and `cli/solve._timed_load` both call
`Path.read_text` unguarded, so `ein solve /nope.ein` is a
`FileNotFoundError` traceback and exit 1 — not the clean message
[S1a.5.4](p1a.5_presentation/s1a.5.4_cli.md) originally listed among its
argument errors. It belongs to this group instead, and it sharpens the
open half below: the first fixture needs a mixed-type puzzle, this one
needs a typo.

**S1a.5.4 — the open half, answered for every path the CLI reaches: name the
class.** ein.rs now prints CPython's own last line, so the comparison passes
on the whole line rather than only on the class it extracts:

- a missing input file → `FileNotFoundError: [Errno 2] No such file or
  directory: '<path>'`, exit 1;
- a `CompileError` out of `solve` or `saturate` →
  `ein.inference.compile.CompileError: <message>`, exit 1 — the *message*
  was already at parity from P1a.3, so naming the class was the whole gap.

That is 6 of the 7 `crash-parity` cells; the seventh is D2. Naming a Python
class from Rust is not a category error here: the class is the *oracle's*
observable, and reproducing it is what I1 asks for. What stays open is the
narrower question the relaxation would answer — whether a future ein.rs-only
error, with no Python counterpart to name, joins this group or a new one.
Nothing in the corpus reaches one.

## Q-M1a.15 — Float formatting parity

Several reported numbers are formatted floats — `--hyp-stats`'s
`{100.0 * n / total:>5.1f}` percentages, `--timing`'s `{ms:9.2f}` (whose
*values* are normalised away, but whose *widths* are not), and
`--stats`' `{elapsed_ms:.1f}`. Rust's `{:.1}` and Python's `%.1f` agree
on round-half-to-even for `f64`, but the two differ on `-0.0`, on `inf`
/ `nan` spellings, and on very large magnitudes.

Proposal: a `pyfmt` helper beside `pyrepr`
([design/02](design/02_determinism_and_order.md) §7) covering `f`-format
with width/precision, differentially tested over a wide float corpus.
Small, and it removes a whole class of one-character T3 diffs.

**Resolved 2026-08-18 at [S1a.1.2](p1a.1_ir_frontend/s1a.1.2_ast_and_dumper.md):
`ein-core::pyfmt`**, covering `[[fill]align][sign][0][width][.precision]f`
and rejecting anything outside that subset rather than guessing at it.
230 values × 19 specs against CPython, 0 differences. Three findings
beyond the proposal: Rust spells NaN `NaN`; a NaN never carries a sign in
CPython while an infinity does; and an **empty** spec is `str(x)`, not
`.6f` — so the `f` is required, not assumed. The digits themselves come
from Rust's `{:.*}`, which agrees with Python on round-half-even over the
exact binary value.

`pyrepr` landed with it, and needed a **generated Unicode table**:
`repr()` escapes by general category and `rustc` exposes only
`is_control()`, so `printable.rs` is generated from CPython's own tables
by `utils/gen_unicode_printable.py` (737 ranges, Unicode 16.0.0). A
CPython upgrade that moves a category surfaces as a named code point in
the differential test rather than as a mystery diff at P1a.5.

## Q-M1a.16 — How does the harness drive the lever matrix?

[design/01](design/01_parity_contract.md) §4 puts "each `SolverConfig` lever
flipped off (the same matrix [`utils/feature_matrix.py`](../../utils/feature_matrix.py)
already drives)" in the corpus run matrix. Building the manifest at
[S1a.0.1](p1a.0_conformance_harness/s1a.0.1_parity_contract_and_corpus.md)
found that only **four of the ten** are reachable from the CLI — `-L`
(`enable_pre_branch_lookahead`), `-K` (`enable_lookahead_kill_cache`), `-y`
(`lattice_sanity_check`) and `-o score-sum` (`lattice_order`). The other six —
`enable_path_nogoods`, `enable_symmetric_mirror`, `enable_singleton_writeback`,
`enable_forced_positive`, `enable_fail_fast_fork`, `hypgen_scoring` — exist only
as a Python kwarg or a puzzle's own `(config …)` block. `feature_matrix.py`
reaches them because it imports the engine; the harness shells out, so it
cannot.

That matters more than it looks: those six gate exactly the optimisations
[P1a.6](p1a.6_performance/README.md) will re-implement, and "lever off" is the
cheapest way to isolate a parity failure to one of them.

Options:

- **(a) add `--config KEY=VALUE` to both CLIs** (repeatable, kebab-cased,
  parsed by the same coercer `(config …)` uses). Additive, ~20 lines, makes
  `levers = "all"` real. Costs: one more flag on the T3 surface both
  implementations must match, and a way to set a lever that a puzzle file
  cannot audit.
- **(b) generate per-lever puzzle variants** — copy the fixture with a
  `(config …)` block appended into a temp dir. No CLI change, but the corpus
  entry is then not the file in `examples/`, which weakens the "both
  implementations read the same bytes" guarantee for those runs.
- **(c) leave the six unexercised** and note the gap. The corpus keeps four
  levers; the other six are covered only by ein.py's own test suite.

**Recommendation: (a)**, decided before [P1a.6](p1a.6_performance/README.md)
rather than at it — the flag has to exist in *both* implementations, so it is
cheapest to add while the Rust CLI is still a stub. Until then the manifest's
`levers` lists the four, and `conformance/README.md` says why.

## Q-M1a.17 — Win B's ≥ 80 % assumed monotone guards dominate

**Found 2026-08-18 at [S1a.3.4](p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md),
by measurement rather than by argument.**

[design/06](design/06_saturation.md) § Win B projects that guard sub-plan
**evaluations** drop by ≥ 80 %, and names the mechanism: a *monotone*
guard's query is purely positive, so if it found nothing at round *r* it
can only start finding something through a fact added since — which is
`run_seeded` on the guard's sub-plan, restricted to Δ ∩ watched.

The port instruments that split (`Saturator::guard_evals` /
`guard_evals_monotone`), and at **root scale the mix is the other way
round**:

| root | rounds | guard evaluations | of which monotone |
|---|---:|---:|---:|
| `zebra2` | 40 | 958 | **109 (11 %)** |
| `zebra` | 119 | 945 | **280 (30 %)** |

The reason is structural rather than incidental. A candidate that is
*still parked* has a guard that **failed**, and a failing **monotone**
guard retires its candidate on the spot — so every re-judged candidate is
one whose failing guard is *non-monotone*, i.e. a `forall`'s
`(absent (and G (absent B)))`, which design/06 excludes from the
mechanism by name. What is left for the semi-naive path is the monotone
guards that *passed* earlier in the same `first_failing` scan.

The boundary is still where the time is — 80 % of a `zebra` root
saturation and 34 % of a `zebra2` one, and essentially all of it inside
the queries themselves (945 evaluations × 6.2 µs ≈ the whole 5.8 ms).
The two refinements that do not depend on monotonicity **landed** and are
T2-green: the per-round `(guard, projected env) → verdict` memo, and an
allocation-free watch stamp on an ordered parked set instead of a
pop-and-re-push heap. Together they moved the boundary by ~2 % at root
scale, which is the honest number.

**Open:** does the *exhaustive* mix differ? design/06's figure comes from
an exhaustive `zebra2` (3 178 rounds, 33 113 queries), which
[P1a.4](p1a.4_search_layer/README.md) is the first phase able to run. The
instrument is already in place, so the question is answered by running it
rather than by re-arguing it.

**Recommendation:** carry the semi-naive guard re-evaluation
(T1a.3.4.5) into [P1a.6](p1a.6_performance/README.md) as a *measured*
optimisation with this as its trigger condition, rather than landing a
mechanism here whose measured reach is a tenth of its stated one. If the
exhaustive mix is monotone-dominated, it lands there with a number; if it
is not, the boundary needs a different idea and this question is where
that gets decided.

---

## Q-M1a.18 — May a fork stop re-narrating the root's fixpoint?

**Found 2026-08-18 at [S1a.6.9](p1a.6_performance/s1a.6.9_fork_entry_delta.md),
by measurement.** The numbers are in
[baseline.md §9](p1a.6_performance/baseline.md#9-the-fork-entry-re-derivation).

Every entering builds a fresh `Saturator` over the forked root, so its
first enqueue pass is a FULL pass and the root's whole deductive closure
is re-derived inside the fork. Measured on `-e` runs: **95.6 %**
(`zebra2`, 36 442 / 38 136) and **94.6 %** (`zebra`, 107 610 / 113 746) of
a fork's firings are redundant re-derivations, and `try_commitment_set` is
**95.0 %** of `zebra -e` cumulatively — the one workload that misses its
milestone target.

Resuming the saturator from the root's state (`engine`, `seen`, `fired`,
`parked`, tiebreaker) with `delta = the commitment facts` removes them.
The fixpoint, the alternative justifications, the verdict, `k`, the models
and the unsat core are all argued to be unchanged
([S1a.6.9](p1a.6_performance/s1a.6.9_fork_entry_delta.md) § What is *not*
at risk, with the `alt` split measured). What changes is the
**narration**:

- **T2** at `verbose` loses ~108 k `fire` lines on `zebra -e` — and
  `EVENTS.md` § Levels says the tier runs at verbose precisely to catch a
  dropped redundant firing;
- **T3** moves `n_firings` in `--trace`, the `("firings", len)` counts in
  `--dump-states`, and the *first five firings* `render/shape.rs` prints
  per solution;
- **T0/T1** do not move: `BaseStats` never counts a firing.

**The options.**

- **(a) No.** I1 is the milestone's spine and the trace is an observable.
  The win is then taken only where it is invisible:
  [S1a.6.8](p1a.6_performance/s1a.6.8_compile_cache_and_extents.md) for the
  compile share and [S1a.6.3](p1a.6_performance/s1a.6.3_beta_memories.md)'s
  *root* beta-memories for the match share — same firings, same order,
  discovered by lookup instead of by rescanning.
- **(b) Yes, in both engines.** ein.py changes first, ein.rs follows, T2/T3
  goldens are regenerated once, and the change is recorded in
  [divergences.md](divergences.md) as a *joint* change rather than a
  divergence. This is a change to the M1 engine, so it is a followup and
  a new stage, never a retrofit into a shipped phase.
- **(c) Yes, behind a flag** that is off in the parity build. Keeps I1 and
  gets the speed for the M1b/M2 consumers — at the cost of a second code
  path through the saturator's most delicate ordering, which is exactly
  what P1a.6 Rule 3 (a wash is a revert) exists to discourage.

**Recommendation: (b), and the argument is not primarily speed.** A fork's
firing list is what
[`08-human-style-deductive-trace`](../ideas/08-human-style-deductive-trace.md)
renders, and 960 re-derivations of what was already true before the
hypothesis is noise in it — the human walkthrough in
[`zebra_walkthrough.md`](../../docs/kernel/inference/zebra_walkthrough.md)
narrates what a hypothesis *adds*. If a shorter trace is the better trace,
the shorter trace is the one both engines should produce, and the speed is
a consequence. **Decide it against a rendered before/after**
(T1a.6.9.3), not against a line count.

**Blocked on:** nothing — the flag in T1a.6.9.2 produces the evidence.
