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
| [Q-M1a.3](#q-m1a3--parse-error-message-parity) | Parse-error message parity, including `-1:-1` at EOF | open — blocking P1a.1 |
| [Q-M1a.4](#q-m1a4--sorted-over-mixed-type-fact-args) | `sorted()` over mixed-type fact args raises in ein.py | open — blocking P1a.4 |
| [Q-M1a.5](#q-m1a5--reproducing-cpythons-shuffle) | Reproducing CPython's `random.shuffle` for `--shuffle` | open — recommendation: port MT19937 |
| [Q-M1a.6](#q-m1a6--at-none-in-loader-messages) | `at None` in loader messages (top-level forms carry no `loc`) | open — post-parity fix |
| [Q-M1a.7](#q-m1a7--may---jobs--1-move-counters) | May `--jobs > 1` move counters? | open — recommendation: no, plus an opt-in escape |
| [Q-M1a.8](#q-m1a8--_binding_key-drops-non-string-activator-args) | `_binding_key` drops non-string activator args | open — port as-is, flag upstream |
| [Q-M1a.9](#q-m1a9--where-do-goldens-live) | Where do goldens live? | open — decide at the P1a.5 gate |
| [Q-M1a.10](#q-m1a10--does-f11-d1-beta-memories-land-inside-m1a) | Does F11 D1 (beta-memories) land inside M1a? | open — gated on measurement |
| [Q-M1a.11](#q-m1a11--server-wire-protocol) | Server wire protocol — JSON-RPC vs gRPC vs bespoke | open — recommendation: JSON-RPC 2.0 |
| [Q-M1a.12](#q-m1a12--remote-access-and-auth) | Remote access and auth for `ein serve` | open — out of v1 scope |
| [Q-M1a.13](#q-m1a13--argparse-surface-parity) | Reproducing `argparse` `--help` and error text | open — blocking P1a.5 |
| [Q-M1a.14](#q-m1a14--crash-parity) | Crash parity — inputs where ein.py raises an unhandled exception | open |
| [Q-M1a.15](#q-m1a15--float-formatting-parity) | Float formatting parity in reported numbers | open |

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
(b) the reference implementation for M2/M3 experiments, and (c) the
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

## Q-M1a.6 — `at None` in loader messages

Top-level `SForm`s are constructed without a `loc`
([design/04](design/04_ir_frontend.md) §3), so loader errors that
interpolate `at {form.loc}` print `at None`. ein.rs has the position and
would naturally print it.

**Recommendation: print `at None` during the port (T3), then fix both
implementations together** in a post-parity stage. Tracked here so the
fix is not forgotten; it is a genuine usability bug.

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

JSON-RPC 2.0 over stdio/unix/tcp is the recommendation
([design/09](design/09_server_mode.md) §2): no codegen, debuggable,
LSP-shaped. gRPC buys streaming ergonomics and typed schemas at the cost
of a build-time dependency and a much heavier client story; a bespoke
binary protocol is premature.

Decide at [P1a.8](p1a.8_server_mode/README.md) kickoff, informed by what
[M1b](../m1b_gui/README.md) picks for its stack.

## Q-M1a.12 — Remote access and auth

`ein serve` is local-only in v1 (unix socket 0600, TCP loopback,
`--allow-remote` required to bind elsewhere, no authentication). If
hosted use is ever wanted, the answer is a reverse proxy plus a token,
not an auth system in the engine. Out of v1 scope; recorded so the v1
posture is a decision rather than an oversight.

## Q-M1a.13 — `argparse` surface parity

T3 includes `--help` output and CLI error messages. `argparse` has a very
specific layout (usage line wrapping, `options:` heading, metavar
rendering, two-space indent) and its own error text
(`argument -n/--solutions: invalid int value: 'x'`). `clap` does not
match it and cannot be configured to.

Options: (a) hand-roll the argument parser and the help renderer to match
`argparse` byte-for-byte; (b) use `clap` and put `--help`/CLI-error text
on the normalisation list; (c) match the *semantics* (flags, defaults,
mutual exclusion, exit codes) exactly and accept different help text.

**Leaning (a) for the ~40 flags across four subcommands** — it is
mechanical, and "drop-in replacement" is weakened noticeably if `--help`
differs. But it is real work and (c) is defensible; decide at
[P1a.5](p1a.5_presentation/README.md) kickoff with a prototype of both.

## Q-M1a.14 — Crash parity

Some inputs make ein.py raise an unhandled exception (Q-M1a.4's
`TypeError`; a `KeyError` from an unbound `:assert` var is *caught*
nowhere and surfaces as a traceback). ein.rs will not have Python
tracebacks.

Proposal: the harness compares **exit code + the first line of stderr**
for crash cases and records them as a distinct corpus group
(`crash-parity`), with the traceback body normalised away. Any input in
that group is also a candidate ein.py bug report.

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
