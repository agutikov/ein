# 33 — Q-M1f.399: the names of what controls the search, `false` and `open`

## 1. The question

Two words of Ein are read by the kernel, not by rules: a state that holds `(false)` (or a fact beside its stored
negation) is dead, and `open` is what an obligation concludes, the debt the kernel counts. You asked whether
`false` is the right word for what it does (*"too heavy loaded with classical binary logic … Maybe (abort) or (dead)
… Or some common search control interface accessible from ein? Like (ein.search.state.refute)?"*), and then gave two
directions: on 2026-11-17, `(false)` stays the word and becomes an ordinary fact, and a state dies only where a rule
concludes a fact of the kernel's own, `(ein.contradiction)`, from a global `(false)`; on 2026-11-18, *"(false)
controls not search but saturation, right? … somebody has to kill it, maybe (ein.kill) or (ein.die) or
(ein.saturation.kill)?"*, and `open` becomes the hypotheses themselves, `(open (?R ?a ?b))`. With that came a third
note: *what reserved names will be left, and what does the kernel read to decide about saturation or search?*

Recorded in [`open_questions.md` § Q-M1f.399](../../../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open)
(**not ruled, and no default taken**); the options are
[`open_facts.md` § 10.7](../../../p1f.21_verdicts_as_rules/open_facts.md#107-the-kernels-surface-false-and-open-control-the-search),
the second direction is [§ 2](../../../p1f.21_verdicts_as_rules/open_facts.md#2-false-acts-on-a-saturation-the-search-reads-the-verdict)
and [§ 3](../../../p1f.21_verdicts_as_rules/open_facts.md#3-who-kills-a-dead-node-einkill), the reserved names
[§ 8](../../../p1f.21_verdicts_as_rules/open_facts.md#8-the-reserved-names-that-are-left-and-what-the-kernel-reads); the
phase's finding is [V-5](../../../p1f.21_verdicts_as_rules/findings.md#v-5--the-users-notes-from-p1f22-questions-as-facts-the-two-negation-rules-and-the-names).
What `open` would hold is item [32](../32-Q-M1f.390/README.md).

Every run is today's binary, `build/review-bin/ein-head-17085b27`, every P1f.23 stage merged
(`runs/ein-version.txt`), the census on the tree at `fd8da416`, whose code is the binary's;
`tools/run.sh` makes every file.

## 2. What is so today

### 2.1 `(false)` acts within one saturation; the search reads only the verdict

[`programs/kill.ein`](programs/kill.ein), written for this review: one key, two doors, each door the key locks a
guess (an hrule). A locked door's alarm is on (`alarm-when-locked`), every alarm on is logged (`alarm-logged`), the
front door's alarm is never on (`front-alarm-broken` concludes `(false)`), and `dead-from-false` reads a held
`(false)`. `alarm-logged` is written before `front-alarm-broken`.

`ein solve` (`runs/kill.solve.txt`): R is ∅ and {(locks k1 back)}; `N  1 minimal dead set`. Its
events, the node {(locks k1 front)} (`runs/kill.events.md`, side by side in
`runs/kill.front-node.vs-falsum.emu.md`):

<!-- output -->
```text
| 19 | fire    | `alarm-when-locked` ?k=k1 · ?d=front · (locks k1 front) ⟹ **(alarmed front)** |
| 20 | enqueue | `alarm-logged` ?d=front · tiebreaker 2 |
| 21 | enqueue | `front-alarm-broken`  · tiebreaker 3 |
| 22 | fire    | `front-alarm-broken`  · (alarmed front) ⟹ **(false)** |
| 23 | enter   | layer 1 · {(locks k1 front)} · **dead** · owes 0 · core (locks k1 front) |
| 25 | layer   | layer 2 · visited 0 · dead 0 · owing 0 · in R 0 · facts 0 |
```

Three things, each in that stream:
- **`(false)` reorders the saturation**: `front-alarm-broken`, queued after `alarm-logged`, fires first, since the
  agenda fires first what leads to `(false)` (Q-M1f.313).
- **`(false)` stops the saturation**: the node's saturation ends at that firing; `alarm-logged` stays queued and
  `dead-from-false` is never queued. The node holds no `(logged front)` and no `(dead)`.
- **The search reads the verdict, not the fact**: the `enter` line's `kind: dead` and its core are all it takes
  from the node, and the node above it, {(locks k1 back), (locks k1 front)}, is never built (layer 2 visited 0;
  licence L1).

Drawn: `renders/kill.region.svg`, the cube with the run's visits (the top node an ellipse,
*above a dead node*, not visited); `renders/kill.lattice.svg`, the three nodes visited;
the visit order `runs/kill.dump-lattice/nodes.md`. The region trace
(`runs/kill.trace.md`) narrates the states of R and gives the dead one as N, its core and the rule
that refuted it:

<!-- output -->
```text
<summary>{(locks k1 front)} — dead</summary>
Core: `(locks k1 front)`
Refuted by `front-alarm-broken`.
```

**Where nothing stops the saturation**, the same node holds everything: `ein oracle --dump --states` runs every
node to quiescence (`runs/kill.oracle.txt`, `runs/kill.oracle/region.json`),
and {(locks k1 front)} holds `(alarmed front)`, `(dead)`, `(false)`, `(locks k1 front)`, `(logged front)`; the top
node is saturated too, and dead. Root's own saturation is not stopped either:
[`programs/kill-root.ein`](programs/kill-root.ein) writes `(locks k1 front)`, and its trace
(`runs/kill-root.trace.md`, the steps drawn as
`renders/kill-root.trace.step1.svg`, `renders/kill-root.trace.step2.svg`,
`renders/kill-root.trace.step3.svg`, `renders/kill-root.trace.step4.svg`) narrates root dying in four
steps: `alarm-when-locked`, `front-alarm-broken` (*Derives `false`*), then `alarm-logged` and `dead-from-false`. The
read-out (`runs/kill-root.solve.txt`):

<!-- output -->
```text
  empty             every node is dead — root's own fixpoint derives (false)
  unsat core        (locks k1 front)
  refuted by        front-alarm-broken
  (false)
```

and the KB, `(false)` a held fact, an octagon whose arrow `1` is at the box `false`, dashed with `by
front-alarm-broken` beside it, as `(dead)` is with `by dead-from-false`: `renders/kill-root.kb.svg`
(`runs/kill-root.saturate-dump.txt`: `DERIVED (4 facts)`).

**Under an arrow, `(false)` controls nothing.** [`programs/ctx-false.ein`](programs/ctx-false.ein) (open_facts.md
§ 10.4's `x1.ein`): Ann thinks `(= 5 (+ 2 2))` and its negation, a rule concludes `(think Ann (false))`, and root is
alive, ∅ in R, the fact held as any other (`runs/ctx-false.solve.txt`;
`renders/ctx-false.kb.svg`: `(false)` an octagon in the frame `think ctx`, held by
nothing). A name for the search's act (`dead`, `abort`, `refute`)
would read there as a command where a proposition stands.

### 2.2 The census: the kernel's names, and the code that reads each

`runs/kernel-census.md`, taken by `tools/census.py` as open_facts.md § 8
says: the names are `ein-core/src/terms.rs`' `Kernel`; a reader is a line of `ein.rs/crates/*/src` that reads the
field (`kernel.NAME`, or `k.NAME` with `k` bound to `&terms.kernel`), each listed with **file:line** and its function
in the census's second table (raw: `runs/kernel-census.field-readers.txt`). The column *becomes* is § 8.2's
(**read, not run**: nothing is built).

| name | becomes, by § 8.2 | code that reads it through the struct (files) |
|---|---|---|
| `false` | **goes** from the kernel: a proposition; `(false)` ⟹ `(ein.kill)` a rule every program gets | 15 lines in 9 files: `contradiction.rs` (`detect`, `has_contradiction`, `contradicts`), `saturator.rs` (`refresh_first`, the agenda), `forbid.rs` ×5, `monotone.rs`, `naf_deps.rs`, `detector.rs`, `firing.rs`, `hypgen.rs`, `unheld.rs` |
| `not` | **a convention**: a relation whose meaning is two rules (the contradiction, the double negation's elimination) | 29 lines in 13 files, `kb.rs`, `contradiction.rs`, `forbid.rs`, `monotone.rs` (9) among them; the KB view's `kb_dot.rs` no longer, since it draws a stored negation as any fact (Q-M1f.432) |
| `open` | **stays, changed**: the open facts the kernel writes and moves, `(open F J)` | `obligations.rs:276` (`open_argument`), `unheld.rs:167` |
| `relation` | **a convention**; the declaration keeps the count, the `:why`, the blind rung's guessing | `program.rs:159`, `structure.rs:1693`, `from_ir.rs:1297`, `dump/serialise.rs:66` |
| `eq`, `neq` | **stay** (eq on your word) | `firing.rs:199,201`, `predicates.rs:43,44`, `hypgen.rs:640` |
| `absent`, `and`, `or` | stay: syntax; `absent` the kernel's boundary | `hypgen.rs:640` (left out of the blind rung's objects); the rest is the grammar |
| `rule` | stays | `program.rs:159` |
| `hrule` | **goes**: a rule that concludes `(open F)` | none through the struct: the grammar (`ein.l:55`, `ein.y`) and the loader's strings |
| `macro`, `query`, `config`, `trace`, `import` | stay: declarators | none through the struct: the grammar |
| `__closed__` | **goes** (derived once proposing is rules) or `ein.closed` | `hypgen.rs:643`, `mechanisms.rs:239,595`, `unheld.rs:167` |
| `__symmetric__` (and the mirror's `a`, `b`) | **gone** since the first look: the native mirror is removed and the name an ordinary one (Q-M1f.436); a symmetric relation is `std.algebra`'s rule | no field of the struct, no reader (24 fields where there were 27) |
| `#query`, `<forced-positive>` and its two siblings | stay the engine's | `compile.rs:415`, `verdict.rs:85,153`; the three `<…>` none |
| `=`, `#empty`, `#params` | not in § 8.2: a grammar terminal and the parser's two synthetic heads | none through the struct |

Not in the struct, and in § 8.1: **a rule's activator** (no name reserved; `compile.rs`), **`is-a`** (no kernel name;
at the first look spelt as a string in `from_ir.rs:717`, `kb_dot.rs:112,346`, `ir_dot.rs:357` and `grid.rs:317`;
today **spelt nowhere** outside the tests, the ruling that the kernel reads no theory's name having taken out
`program_relations`' `is-a`, the drawings' and `--models grid` with `grid.rs`, Q-M1f.436), and **`ein.kill`**,
spelt nowhere. The census's third table lists every file that spells
each name as a string, outside the tests (raw: `runs/kernel-census.literal-readers.txt`).

**What § 8.3 says is left for the kernel to read**, read, not run: one verb in a saturation, `ein.kill`; `absent`'s
boundary; `eq` and `neq`; a rule's activator; its open facts, which it writes and moves; the split in the search.

**The names in programs** (`runs/name-counts.txt`, § 8's command from the repo root): `eq` 146
in 40 files (30 under `examples/ein-bugs/wrong-answers/`), `neq` 345 in 119, `__closed__` 685 in 91, as § 8.2 has
them; `__symmetric__` 0 (§ 8.2: 12 in 5; the six programs that wrote it were retired with the mirror, Q-M1f.438);
`hrule` 143 in 117 (§ 8.2: 141 in 115); `(false)` 674 lines in 278 files by § 10.7's `grep -c` (§ 8.2 and § 10.7:
658 in 278).

### 2.3 `eq` and `neq` on nested facts (open_facts.md § 8.4), re-taken

[`programs/eqn.ein`](programs/eqn.ein), as § 8.4 writes it; `ein saturate --dump`
(`runs/eqn.saturate-dump.txt`), the `hit` facts by case in `runs/eqn.hits.md`:

| case | what | § 8.4 says | today |
|---|---|---|---|
| c1 | `eq`, both sides bound: depth 1, depth 2, a negation | 3 hits | **3**: `(f a)`, `(g (f a))`, `(not (f a))` |
| c2 | `neq`, both sides bound | 12 | **12** |
| c3 | a fact a rule built against the same one written | `(hit c3-derived-eq (f a) (f a))` | **the same** |
| c4 | a free side bound to a written fact | `(hit c4-bind (g (f a)) (g (f a)))` | **the same** |
| c6 | `neq` with a free side | 18 hits, each an atom, no fact | **16** hits, each an atom (`C X Y a b f g hit p q r` and the five case names), no fact |
| c7 | `eq` as a conclusion, true | no `(false)`, ∅ in R | **the same** (`runs/eqn.solve.txt`) |

- [`programs/eqn8.ein`](programs/eqn8.ein): a false `eq` concluded on two nested facts kills;
  `runs/eqn8.solve.txt`: *every node is dead*, `unsat core (p (f a))`, `(q (f b))`, `refuted by c8`
  (its events `runs/eqn8.events.md`, trace `runs/eqn8.trace.md`).
- [`programs/eqn9.ein`](programs/eqn9.ein): `(eq ?x (f ?w))` refused at load, exit 1
  (`runs/eqn9.solve.txt`: *a guard compares two values, each a variable, a name, or a fact that
  holds no variable*).
- The corpus's case, [`programs/form-in-guard.ein`](programs/form-in-guard.ein)
  (`examples/ein-bugs/wrong-answers/form-in-guard.ein`): root holds `(hit-eq (f a))` and `(hit-neq 1)`, `a`, `b`
  (`runs/form-in-guard.no-search.txt`), as its row in `corpus/regressions.toml` says
  (`runs/form-in-guard.regressions-row.txt`).
- The KB of `eqn.ein`, the nested facts drawn: `renders/eqn.kb.svg`.

## 3. The options

Q-M1f.399's four (open_facts.md § 10.7, **read, not run**), and how each reads held and under an arrow:

| | what | held | in a context | costs |
|---|---|---|---|---|
| **(a)** keep `false` | the pages say once that it is no truth value and what the kernel does with it | `(false)` | `(think Ann (false))` | nothing |
| **(b)** a word for the proposition | `(contradiction)`, `(impossible)`, `(absurd)` | *this state holds a contradiction* | `(think Ann (contradiction))` | a rename: 676 lines in 280 programs, the pages, the diagnostics, the goldens |
| **(c)** a word for the search's act | `(dead)`, `(abort)`, `(refute)` | *drop this state* | `(think Ann (dead))`: a command where a proposition goes | the same rename |
| **(d)** the kernel's surface as a named interface | every relation the kernel reads or states under one prefix, `ein.…` | `(ein.refute)`, or `(false)` by a short name | as written | a rename, and how a program gets the short names |

**Your directions** take (a) for the word and (d) for the kernel's: `(false)` stays, becomes an ordinary
proposition, and the kernel reads one verb under its prefix, `(ein.kill)` the leaning (open_facts.md § 3). Shown
three ways:

**The program as you would write it with `(ein.kill)` in place of `(false)`** —
[`programs/kill.ein-kill.MOCK.ein`](programs/kill.ein-kill.MOCK.ein), a mock beside the real
[`kill.ein`](programs/kill.ein), the diff `runs/kill.ein-kill.MOCK.diff`:
`front-alarm-broken` concludes `(ein.kill)`; `dead-from-false` has nothing left to read, since a rule may not read
the kernel's verb. Today's binary loads it, and `(ein.kill)` is an ordinary fact of an undeclared relation: nothing
dies, R is all 4 states (`runs/kill.ein-kill.MOCK.solve.txt`). Under the direction a
program need not write it at all: `(false)` ⟹ `(ein.kill)` is a rule of a module every program gets, mocked as
[`programs/ein.core.MOCK.ein`](programs/ein.core.MOCK.ein) (`kill`, the contradiction rules `clash-0`…`clash-2` of
design.md A.1, and `dne-2`, the double negation's elimination); it loads today as rules
(`runs/ein.core.MOCK.saturate-dump.txt`: `;; Rules (9)`).

**The direction emulated on today's engine** — [`programs/kill-falsum.emu.ein`](programs/kill-falsum.emu.ein):
the program concludes `(falsum)`, an ordinary fact standing in for the proposition `(false)`, and a rule `kill`
concludes today's `(false)`, standing in for `(ein.kill)`. The answer is the same R
(`runs/kill.solve.vs-falsum.emu.diff`: 2 more facts beyond root, nothing else), and
two things move:
- **the saturation's order**: `front-alarm-broken` no longer jumps the queue, since it concludes no `(false)`, so
  `alarm-logged` fires first and only `kill` jumps
  (`runs/kill.front-node.vs-falsum.emu.md`). The agenda (Q-M1f.313) reads
  `(false)` by its name; under the direction it would have to follow the rules that lead to `ein.kill`;
- **the explanation**: `refuted by` names `kill`, the module's rule, and not the program's
  (`runs/kill-root.solve.vs-falsum.emu.diff`; the trace's N too,
  `runs/kill-falsum.emu.trace.md`).

**The hypothesis as an open fact, `(open (R a b))`** — [`programs/kill.open.MOCK.ein`](programs/kill.open.MOCK.ein),
the diff `runs/kill.open.MOCK.diff`: the hrule `guess` becomes a rule that concludes
`(open (locks ?k ?d))`, and a rule over open facts concludes another open fact (a report, open_facts.md § 6). Today's
parse refuses it (`runs/kill.open.MOCK.solve.txt`: *open is reserved: it is the verdict a
rule's :assert states … and nowhere else*). Emulated with a stand-in, [`programs/kill-question.emu.ein`](programs/kill-question.emu.ein):
a rule concludes `(question (locks ?k ?d))` and the hrule supposes what is asked. H is the same 2 candidates
(`runs/kill.show-hyps.vs-question.emu.diff`), R the same; what the stand-in
cannot do is the move: both questions stay held in the state that answered one
(`runs/kill.solve-p.vs-question.emu.diff`).

## 4. The default

**None taken**: the ledger puts the question with no default. The leanings on record: § 10.7 would *not rename in
this phase*, and if renamed, take a word for the **proposition** ((b)), since the same fact must read well under an
arrow; your 2026-11-17 direction keeps `false` and puts the kernel's verdict under the kernel's prefix ((a) with
(d)); your 2026-11-18 direction makes it a verb, `(ein.kill)` the leaning of open_facts.md § 3, among `(ein.die)`,
`(ein.saturation.kill)` and V-5's `(ein.contradiction)`. S1f.21.4 is to put it again, with the reserved names that
are left (§ 9, items 1 and 6).

## 5. Files

**`programs/`**
- [`kill.ein`](programs/kill.ein) — written for this review: `(false)` within one saturation.
- [`kill-root.ein`](programs/kill-root.ein) — the same with `(locks k1 front)` written: root dies.
- [`kill-falsum.emu.ein`](programs/kill-falsum.emu.ein) — the direction emulated: `(falsum)` the proposition, `kill` the verb.
- [`kill-root-falsum.emu.ein`](programs/kill-root-falsum.emu.ein) — the same, root dying.
- [`kill-question.emu.ein`](programs/kill-question.emu.ein) — the hypothesis as a held `(question F)`.
- [`kill.ein-kill.MOCK.ein`](programs/kill.ein-kill.MOCK.ein) — **mock**: `(ein.kill)` in place of `(false)`.
- [`kill.open.MOCK.ein`](programs/kill.open.MOCK.ein) — **mock**: the hrule as a rule concluding `(open (locks ?k ?d))`.
- [`ein.core.MOCK.ein`](programs/ein.core.MOCK.ein) — **mock**: the module every program would get.
- [`ctx-false.ein`](programs/ctx-false.ein) — open_facts.md § 10.4's `x1.ein`: `(false)` under an arrow.
- [`eqn.ein`](programs/eqn.ein), [`eqn8.ein`](programs/eqn8.ein), [`eqn9.ein`](programs/eqn9.ein) — § 8.4's runs (eqn8 and eqn9 written from its words).
- [`form-in-guard.ein`](programs/form-in-guard.ein) — copy of `examples/ein-bugs/wrong-answers/form-in-guard.ein`.

**`runs/`** (`.txt` is stdout and stderr with `[exit N]` last; `.md` a table rendered from the file it names)
- `runs/ein-version.txt` — the binary.
- kill: `runs/kill.solve.txt`, `runs/kill.solve-p.txt` (`solve -p`), `runs/kill.show-hyps.txt`, `runs/kill.events.jsonl` / `runs/kill.events.md`, `runs/kill.summary.json`, `runs/kill.trace.md`, `runs/kill.dump-lattice/nodes.md` (`nodes.json`, `nodes.md`), `runs/kill.oracle.txt` and `runs/kill.oracle/region.md` (`region.json`, `region.md`), `runs/kill.saturate-dump.txt`, `runs/kill.render-rules.txt`.
- kill-root: `runs/kill-root.solve.txt`, `runs/kill-root.events.jsonl` / `runs/kill-root.events.md`, `runs/kill-root.summary.json`, `runs/kill-root.trace.md`, `runs/kill-root.saturate-dump.txt`.
- the direction emulated: `runs/kill-falsum.emu.solve.txt`, `runs/kill-falsum.emu.events.jsonl` / `runs/kill-falsum.emu.events.md`, `runs/kill-falsum.emu.trace.md`, `runs/kill-falsum.emu.oracle.txt` and `runs/kill-falsum.emu.oracle/region.md` (`region.json`, `region.md`), `runs/kill.solve.vs-falsum.emu.diff`, `runs/kill-root-falsum.emu.solve.txt`, `runs/kill-root.solve.vs-falsum.emu.diff`, `runs/kill.front-node.vs-falsum.emu.md`.
- the hypothesis as a fact, emulated: `runs/kill-question.emu.solve.txt` (`-p`), `runs/kill-question.emu.show-hyps.txt`, `runs/kill-question.emu.events.jsonl` / `runs/kill-question.emu.events.md`, `runs/kill.solve-p.vs-question.emu.diff`, `runs/kill.show-hyps.vs-question.emu.diff`.
- the mocks through today's binary: `runs/kill.ein-kill.MOCK.diff`, `runs/kill.ein-kill.MOCK.solve.txt`, `runs/kill.open.MOCK.diff`, `runs/kill.open.MOCK.solve.txt`, `runs/ein.core.MOCK.saturate-dump.txt`.
- ctx-false: `runs/ctx-false.solve.txt` (`solve -p`), `runs/ctx-false.saturate-dump.txt`.
- eq and neq: `runs/eqn.saturate-dump.txt`, `runs/eqn.solve.txt` (`eqn.solve -p`), `runs/eqn.hits.md`, `runs/eqn8.solve.txt`, `runs/eqn8.events.jsonl` / `runs/eqn8.events.md`, `runs/eqn8.trace.md`, `runs/eqn9.solve.txt`, `runs/form-in-guard.no-search.txt`, `runs/form-in-guard.regressions-row.txt`.
- the census: `runs/kernel-census.md`, `runs/kernel-census.field-readers.txt`, `runs/kernel-census.literal-readers.txt`, `runs/name-counts.txt`.

**`renders/`** (each `.dot` the engine's, drawn as the `.svg` beside it; KB views by `fdp`, the rest by `dot`)
- `renders/kill.region.svg` (`renders/kill.region.dot`) — the cube, the run's visits marked.
- `renders/kill.lattice.svg` (`renders/kill.lattice.dot`) — the nodes the run visited.
- `kill.rule.*` — `ein render rules`, one graph a rule, split from `runs/kill.render-rules.txt`: `renders/kill.rule.rule_alarm_when_locked_lhs_rhs.svg` (`renders/kill.rule.rule_alarm_when_locked_lhs_rhs.dot`), `renders/kill.rule.rule_alarm_logged_lhs_rhs.svg` (`renders/kill.rule.rule_alarm_logged_lhs_rhs.dot`), `renders/kill.rule.rule_front_alarm_broken_lhs_rhs.svg` (`renders/kill.rule.rule_front_alarm_broken_lhs_rhs.dot`), `renders/kill.rule.rule_dead_from_false_lhs_rhs.svg` (`renders/kill.rule.rule_dead_from_false_lhs_rhs.dot`), `renders/kill.rule.rule_guess_lhs_rhs.svg` (`renders/kill.rule.rule_guess_lhs_rhs.dot`).
- `kill.trace.step*` — the trace's inline drawings, state {(locks k1 back)}: `renders/kill.trace.step1.svg` (`renders/kill.trace.step1.dot`), `renders/kill.trace.step2.svg` (`renders/kill.trace.step2.dot`).
- `kill-root.trace.step*` — root dying: `renders/kill-root.trace.step1.svg` (`renders/kill-root.trace.step1.dot`), `renders/kill-root.trace.step2.svg` (`renders/kill-root.trace.step2.dot`), `renders/kill-root.trace.step3.svg` (`renders/kill-root.trace.step3.dot`), `renders/kill-root.trace.step4.svg` (`renders/kill-root.trace.step4.dot`).
- `kill-falsum.emu.trace.step*` — the emulation's trace: `renders/kill-falsum.emu.trace.step1.svg` (`renders/kill-falsum.emu.trace.step1.dot`), `renders/kill-falsum.emu.trace.step2.svg` (`renders/kill-falsum.emu.trace.step2.dot`).
- `renders/kill-root.kb.svg` (`renders/kill-root.kb.dot`) — the dead root's KB, `(false)` held.
- `renders/ctx-false.kb.svg` (`renders/ctx-false.kb.dot`) — `(false)` under an arrow.
- `renders/eqn.kb.svg` (`renders/eqn.kb.dot`) — eqn.ein's nested facts.

**`tools/`** — `tools/run.sh` (every command), `tools/census.py` (the census), `tools/tables.py` (the tables, the dot blocks split out).

## 6. Re-run

From the repo root (the scripts name the binary by its absolute path, `build/review-bin/ein-head-17085b27`):

```sh
bash plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/review/33-Q-M1f.399/tools/run.sh
```

The engine commands it runs, from the item's folder, `E=build/review-bin/ein-head-17085b27`:

```sh
$E solve programs/kill.ein [-p]
$E solve programs/kill.ein --events runs/kill.events.jsonl --json-summary runs/kill.summary.json --trace runs/kill.trace.md --dump-lattice runs/kill.dump-lattice
$E show hyps programs/kill.ein
$E oracle programs/kill.ein --dump runs/kill.oracle --states
$E saturate programs/kill.ein --dump                         # and kill-root.ein, ctx-false.ein, eqn.ein, ein.core.MOCK.ein
$E solve programs/kill-root.ein [--events … --trace … --json-summary …]
$E solve programs/kill-falsum.emu.ein [--events … --trace …]  # and its oracle; kill-root-falsum.emu.ein
$E solve programs/kill-question.emu.ein -p; $E show hyps programs/kill-question.emu.ein
$E solve programs/kill.ein-kill.MOCK.ein; $E solve programs/kill.open.MOCK.ein
$E solve programs/ctx-false.ein -p
$E solve programs/eqn.ein -p; $E solve programs/eqn8.ein [--events … --trace …]; $E solve programs/eqn9.ein
$E solve programs/form-in-guard.ein --no-search -p
$E render kb programs/kill-root.ein                          # ctx-false.ein, eqn.ein; fdp -Tsvg
$E render region runs/kill.oracle --visited runs/kill.dump-lattice; $E render lattice programs/kill.ein --view full
$E render rules programs/kill.ein
python3 tools/census.py runs                                 # from the repo root, with the item's runs/ path
```

## 7. Found while building

- **Re-taken with `ein-head-17085b27`** against the first look's `ein-head-b5fb8d70`: no run moved (read-outs,
  oracles, events, traces' text, show hyps, dumps, timings aside). What moved is the census, by S1f.23.4: the
  struct lost `__symmetric__` and the mirror's `a` and `b`, `is-a` is spelt nowhere outside the tests, and `not`
  lost the KB view's reader; the programs lost every `__symmetric__` (and two `(false)` lines with the six retired
  programs). The drawings are in the canonical form; the census's link to `open_facts.md` is one level deeper, as
  `runs/` is.

- **§ 8.4's c6 gives 16 hits, not 18.** `eqn.ein` as § 8.4 writes it, with today's binary and with
  `build/review-bin/ein-a8d595eb` (a build after the commit that wrote § 8.4) and `ein-0a4a97a3`: 16 `(hit
  c6-free-neq …)` facts, each an atom, no fact (`runs/eqn.hits.md`). The claim stands (a free `neq`
  ranges over the program's names, never a fact); the count does not. The program as written has no case c5, so
  the run behind 18 may have had one more case.
- **The counts of names in programs moved**: `(false)` 658 lines in 278 programs → 676 in 280 (§ 10.7's command);
  `hrule` 141 in 115 → 143 in 117. § 8's own command counts `false` 760 times in 284 files (it counts occurrences,
  and `(false ` too), so § 8.2's "658 in 278" is § 10.7's line count, not § 8's command
  (`runs/name-counts.txt`).
- **The agenda and the explanation read `(false)` by its name**, and the direction moves both, on an emulation
  with the same R: the refuting rule loses its place at the head of the queue, and `refuted by` names the module's
  `kill` instead of the program's rule (§ 3 above). Neither is in open_facts.md § 3's list of surfaces (the read-out's
  *derives (false)*, `:expect (false)`, the events' `kind: dead`).
- **Nothing reserves `ein.` today**: `(ein.kill)` loads as a fact of an undeclared relation and kills nothing
  (`runs/kill.ein-kill.MOCK.solve.txt`).
- The census finds readers beyond § 8.1's *where* column, none that changes its account: `(false)` is also read
  by `detector.rs`, `firing.rs`, `hypgen.rs` (left out of the guessable objects) and `unheld.rs`; `open` also by
  `unheld.rs`; `__closed__` also by `unheld.rs` and `hypgen.rs`.

## 8. Your call

- **(a)** keep `false`; the pages say once that it is no truth value and what the kernel does with it.
- **(b)** a word for the proposition: `(contradiction)`, `(impossible)`, `(absurd)`.
- **(c)** a word for the search's act: `(dead)`, `(abort)`, `(refute)`.
- **(d)** the kernel's surface under one prefix, `ein.…`.
- **Your directions**: `false` stays and becomes a proposition; the kernel reads one verb, **`(ein.kill)`** (or
  `(ein.die)`, `(ein.saturation.kill)`, `(ein.contradiction)`), concluded by a rule of a module every program gets;
  `open` becomes the hypotheses, `(open F J)`.
- **No default taken**; the leanings are (b) if renamed at all (§ 10.7), and `(ein.kill)` under the direction (§ 3).
