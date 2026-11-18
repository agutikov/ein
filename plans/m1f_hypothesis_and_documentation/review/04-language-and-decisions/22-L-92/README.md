# L-92 — Q-M1f.362's sentence on the fact index is behind the engine's pages

## 1. The question

The decision that no rule of the calculator joins two facts on a number, Q-M1f.362, gives as its reason *"The
engine's fact index does not key on a nested fact"*. The engine's pages say the index **keys one level inside** a
nested fact: each name at a position of the nested fact is a key. Both are true of different things — no key has a
fact as its *value*, and the names inside one *are* keys — so the decision's sentence reads as "nested facts are not
indexed", which the pages contradict. The question is whether the decision's sentence is re-worded, or kept.

Recorded at the milestone's [`findings.md`](../../../findings.md), row **L-92** (§ Decisions), from P1f.22's N-9.

## 2. What is so today

### 2.1 The sentence and the pages

[`runs/sentence-pages-code.txt`](runs/sentence-pages-code.txt) has each grep and its output. The decision,
[`Q-M1f.362`](../../../../../docs/decisions/stdlib/math/Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md),
line 10:

> **Why.** The engine's fact index does not key on a nested fact, so a join of two facts on a number would scan a
> relation for every fact of the other, and a batch of requests would cost the square of its count. Written this way,
> a batch costs in proportion to its count.

[`03-data-model.md` § 6.1 The participation key](../../../../../docs/ein/ein.rs/03-design/03-data-model.md#61-the-participation-key):

> A name in argument `slot` is keyed `(rel, slot, DIRECT, value)`. A nested fact in argument `slot` has no single
> name to key on, so the key reaches **one level inside** it instead: each name at position `inner` of the nested
> fact is keyed `(rel, slot, inner, value)`. […] One level only. That serves a premise over a stored negation and one
> over a context whose fact holds names, `(says ?w (owns ?p ?x))` with `?p` bound. What is nested deeper is not
> keyed: a numeral of the standard library and a tag nest to any depth, and a premise whose only bound slots hold
> facts walks the held extent of its relation.

[`05-matcher.md` § 5.2 A probe into the participation index](../../../../../docs/ein/ein.rs/03-design/05-matcher.md#52-a-probe-into-the-participation-index):

> `Kb::index_fact` keys every argument that is a name directly, and for an argument that is a fact, each of that
> fact's name arguments one level in. No key has a fact as its `value` and none reaches two levels in, so
> `(not (R ?b ?i))` with `?b` bound probes `(not, slot 0, inner 0) = ?b` instead of walking the held extent of `not`,
> and a premise whose only bound slots hold facts walks the held extent unless every slot is bound (§ 5.1).

No other page or decision says what the index keys (the grep in the file finds Q-M1f.362's line 5, *"join on a
**name**, which the fact index keys"*, and Q-M1f.149's same phrase).

### 2.2 The code

(Read, not run; line numbers in [`runs/sentence-pages-code.txt`](runs/sentence-pages-code.txt).)

- `ein-core/src/kb.rs:92` `SlotKey { rel, slot, inner, value }`, `inner = DIRECT` for a key on the argument itself.
- `ein-core/src/kb.rs:1139` `Kb::index_fact`: a name argument is keyed directly; for a fact argument, each of its
  arguments **that is not a fact** is keyed `(rel, slot, inner, value)`; a fact argument itself gets no key.
  `rebuild_layer` (line 1236) is the batch half and does the same.
- `ein-infer/src/match_.rs:919` `Matcher::probe_key`: a register that holds a fact is skipped (*"A nested fact: the
  index does not hold one, so the scan moves on"*), so a premise whose bound slot holds a fact walks the extent.
- Held by `kb.rs`' unit test `the_participation_index_keys_the_join_types_and_one_level_in`.

### 2.3 Both halves, measured

Three programs of N facts a side, written by [`programs/gen_joins.py`](programs/gen_joins.py), timed by `ein
saturate` of today's binary, `build/review-bin/ein-head-17085b27` ([`runs/timings.md`](runs/timings.md); on AC,
pinned to a P-core thread, the least of three runs, **on a machine another agent's test runs were loading**, load
average 23 at the start, see the file: every time is some 2.5 times the first look's, and the ratio across N is the point):

| N | [`join-on-a-name`](programs/join-on-a-name-2000.ein): `(p ?x) (q ?x ?y)`, `?x` a name | [`join-on-a-fact`](programs/join-on-a-fact-2000.ein): the same, `?x` the fact `(f aI)` | [`join-one-level-in`](programs/join-one-level-in-2000.ein): `(p (f ?x)) (q (f ?x) ?y)` |
|---|---|---|---|
| 2000 | 3.26 ms | 66.25 ms | 3.51 ms |
| 4000 | 6.81 ms | 242.50 ms | 7.24 ms |
| 8000 | 14.47 ms | 976.44 ms | 13.76 ms |

A join on a fact is quadratic (×3.7 and ×4 a doubling; at the first look, unloaded, 25.86, 98.65 and 389.40 ms): Q-M1f.362's reason holds of a join **on** a nested fact. The same
facts joined on the name **inside** them are linear, like a join on a name: the pages' one level in. Each run's
whole output is `runs/PROGRAM-N.saturate.txt`, the least of three (`.1.txt` to `.3.txt`), 2000 firings, 4000, 8000
each, the same derivations.

(A join whose second premise has every slot bound, `(p ?x) (q ?x)`, is a ground lookup and linear for a fact too,
[§ 5.1](../../../../../docs/ein/ein.rs/03-design/05-matcher.md#51-a-ground-premise-one-lookup); the programs give `q` a
free second slot so that the probe is what is measured.)

## 3. The options

- **(a) Re-word the decision's sentence** to say both halves. Proposed:

  > **Why.** The engine's fact index keys a name, and the names one level inside a nested fact, never a fact itself:
  > a premise whose bound argument holds a fact walks the held extent of its relation. A number is a fact, so a join
  > of two facts on a number would scan a relation for every fact of the other, and a batch of requests would cost the
  > square of its count. Written this way, a batch costs in proportion to its count.

  It changes no claim of the decision and no test; the link check and the release voice are unaffected (no id, no
  date).
- **(b) Keep it.** Read as *"no key has a nested fact as its value"* it is true, and it is all the decision needs:
  a number is a fact, and joining on it is joining on a fact. The pages carry the detail.

## 4. The default

**None taken** in the ledger: *"The user's call: the decision's sentence re-worded, or kept."*

## 5. Files

- [`programs/gen_joins.py`](programs/gen_joins.py) — writes the three programs at any N; the N = 2000 three are kept:
  [`join-on-a-name-2000.ein`](programs/join-on-a-name-2000.ein), [`join-on-a-fact-2000.ein`](programs/join-on-a-fact-2000.ein),
  [`join-one-level-in-2000.ein`](programs/join-one-level-in-2000.ein).
- [`runs/sentence-pages-code.txt`](runs/sentence-pages-code.txt) — the decision's line, the pages' lines, the other
  statements, and the code that keys the index, each with its command.
- [`runs/timings.md`](runs/timings.md) — the table above, with the machine state.
- [`rerun.sh`](rerun.sh) — writes everything under `runs/`: the code file, the 4000 and 8000 programs (in
  `runs/gen/`) and the timings.
- `runs/PROGRAM-N.saturate.txt`, nine files — `ein saturate` of each program at N = 2000, 4000, 8000, the least of
  its three runs (each run beside it, `.K.txt`):
  [`join-on-a-name-2000`](runs/join-on-a-name-2000.saturate.txt), [`-4000`](runs/join-on-a-name-4000.saturate.txt), [`-8000`](runs/join-on-a-name-8000.saturate.txt);
  [`join-on-a-fact-2000`](runs/join-on-a-fact-2000.saturate.txt), [`-4000`](runs/join-on-a-fact-4000.saturate.txt), [`-8000`](runs/join-on-a-fact-8000.saturate.txt);
  [`join-one-level-in-2000`](runs/join-one-level-in-2000.saturate.txt), [`-4000`](runs/join-one-level-in-4000.saturate.txt), [`-8000`](runs/join-one-level-in-8000.saturate.txt).

## 6. Re-run

```sh
bash plans/m1f_hypothesis_and_documentation/review/04-language-and-decisions/22-L-92/rerun.sh
```

It writes [`runs/sentence-pages-code.txt`](runs/sentence-pages-code.txt) from the checkout (each `grep` and each
excerpt with its line numbers), writes the 4000 and 8000 programs with `gen_joins.py`, and times
`taskset -c 0 build/review-bin/ein-head-17085b27 saturate` three times on each of the nine programs.

## 7. Found while building

- **Re-taken with `ein-head-17085b27`**: the sentence, the two pages and the code are as they were at the first look
  (the code's line numbers too); the timings keep their ratios, the times some 2.5 times the first look's under
  another agent's load. Nothing in the phase's merged work touched the index.

- **`kb.rs`' comment in `Kb::index_fact` (line 1158) still carries the sentence the data-model page lost**: *"Only one
  level: the corpus's nesting is `(not (R …))` and a second would cost entries nothing asks for."* The page now says
  the one level also serves a context, `(says ?w (owns ?p ?x))`.
- **`SlotKey`'s doc comment (kb.rs:82–85) links `03-data-model.md` § 5, *The layered KB*,** for why the second level
  exists; the participation key is § 6.1. The anchor exists, so the link check passes.

## 8. Your call

- **(a)** re-word Q-M1f.362's *Why* to *"keys a name, and the names one level inside a nested fact, never a fact
  itself"* (the text in § 3).
- **(b)** keep it: *"does not key on a nested fact"* read as *no key's value is a fact*.
- **Default**: none taken.
