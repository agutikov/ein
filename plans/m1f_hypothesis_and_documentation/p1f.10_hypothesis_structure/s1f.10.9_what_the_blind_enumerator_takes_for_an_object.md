# S1f.10.9 — What the blind enumerator takes for an object

**Phase:** [P1f.10](README.md), track H (what H is)
**Status:** planned 2026-11-12; at the re-plan of 2026-11-15, E-54 added and its stop made the phase's one
stop. **Tasks 1 to 3 ran on 2026-11-15** ([§ Record](#record-2026-11-15-tasks-1-to-3)): the witnesses rebuilt
and banked, the census taken, the question written as
[Q-M1f.306](../open_questions.md#q-m1f306--what-the-blind-enumerator-takes-for-an-object). **Stopped at the
question**, which is put at the phase's one stop ([S1f.10.12](s1f.10.12_the_structure_shown.md)); task 4 waits
for the ruling, and nothing changes before the user replies.
**Depends on:** nothing in the phase. [S1f.10.1](s1f.10.1_exclusion_census.md)'s census reads the same
candidate sets, and the bundle marks the candidates this stage counts.
**Source:** row 56 of [section J](../p1f.6_calculator/questions.md#j-findings-that-need-an-owner) of the
calculator's questions, E-1 and E-7 of its [`findings.md`](../p1f.6_calculator/findings.md). The row's
suggestion was *(b) P1f.10, which plans how the candidates are built*; the user left the line empty on
2026-11-12, and an empty line takes the recommendation
([round 2, § Left empty](../p1f.6_calculator/questions_round2.md#empty)). So this stage stands on an empty
line and is dropped if the user names the row.

## The three findings

- **E-1. `__closed__` is proposed as an object.** Where the blind enumerator builds H for a declared
  relation and the program writes a `(__closed__ …)` fact of another, the dunder is among the candidate
  objects, so `(total __closed__)` is a candidate beside `(total 0)` to `(total 9)`. The witness: a program
  that imports the calculator's prototype, which closes its own relations, and declares `(relation total
  Number)` without closing it: 11 candidates, k_s 2 048 (2026-11-11).
- **E-7. The blind enumerator proposes as objects the relation names a rule asserts as values.** The
  witnesses are the import probes' `liquid` and `thing`; E-1's neighbour.

- **E-54. The blind enumerator's universe is every relation and every name the program holds**, loaded from
  wherever: a declared relation no rule and no closure touches is proposed about, over objects that include
  every name a module brings. So what a program imports, and not only what it writes, is what is guessed. The
  witness: a library declaring `colour` and `shape`, closing neither; a program that imports it for `colour`
  and declares `near`: 4 candidates, `shape` and `square` among them, 16 states (2026-11-14). No tracked
  program reaches it. Given to this stage on 2026-11-14.

All three load. The user ruled the case behind them (a
program's own open relations beside the theory) *left as it is*; these rows are the roughness inside it.

## Why it is not simply a fix

**Removing a candidate changes H, and H is part of the answer.** k_s 2 048 on E-1's witness counts states
that hold `(total __closed__)`; without the candidate the count is another number. Tracks S and R of this
phase may not change an answer. So this stage is one of the three places in the phase where an answer would
move, with [S1f.10.6](s1f.10.6_obligations_under_hypothesis.md) and
[S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md), and it moves only if the user rules that today's is
wrong:

- The candidates are the program's names by ruling: *a constant a fact names at any depth, or a rule's
  `:match` or `:assert`, is a candidate* (Q-M1e2.9 (a)), and a rule's constant is a name of the program
  (Q-M1e.21 (b)). A relation name a rule asserts as a value is, read literally, such a constant (E-7).
- `__closed__` is a kernel name a program may not bind (Q-M1f.7 (b), *"Never bound"*) and *uses … as a
  relation or a value*. A written `(__closed__ R)` fact names `R`, not the dunder, as a constant; whether
  the dunder itself is a *name of the program* in the sense of Q-M1e2.9 is what E-1 asks.
- Where the blind enumerator binds a **type** as an object, every read-out already says so (`types as
  objects`, Q-M1e.12 (a): *the fixture, with the warning*). That is a precedent for the other reading:
  keep the candidate, and say it.

## Tasks

1. **T1f.10.9.1 — the three witnesses rebuilt and banked** under `build/` first, on the release binary and the shipped stdlib as they
   stand (both were found on the calculator's prototype): H listed, the candidates that
   are a dunder or a relation name marked, R with and without them computed by `ein oracle` on a copy of
   the program that closes the relation.
2. **T1f.10.9.2 — the census.** Over every corpus program: how many candidates of H are `__closed__`,
   `__symmetric__`, or a name that the program uses *only* as a relation or only as a relation name in
   value position, or a name or a relation that only an imported module writes (E-54); and how many programs' R
   would differ without them. Programs that state their lattice
   or close their relations have no such candidate; the census says how many are left.
3. **T1f.10.9.3 — the question, and the stop.** Under the next free id in
   [`open_questions.md`](../open_questions.md), with the census: (a) leave it, and the read-out names such
   candidates as it names types bound as objects; (b) a kernel dunder is never an object, which is a
   wrong answer today and a register row before its fix; (c) (b), and a relation name asserted as a value
   is not an object either, which narrows Q-M1e2.9 and needs the user to say so; and for E-54, (d) leave it,
   the universe is the program's names wherever they were written, or (e) a name or a relation only an import
   brings is not guessed about unless the program writes it, which makes what is guessed depend on what the
   main file spells and needs the user to say so. **Default: (a) for E-7, since it is the ruling as written;
   (b) for E-1, since no program means a dunder as an object; (d) for E-54, since every import brings its
   module whole and the answer does not depend on the import form.** The question is put at the phase's one
   stop, and no default is acted on before the user replies.
4. **T1f.10.9.4 — as ruled.** Under (b) or (c): the register rows filed `open` first, with the witnesses
   under `examples/ein-bugs/wrong-answers/`; the fix in `invariant::program_names` or the enumerator; the
   sweep, old binary against new over the corpus, with every moved R listed; the rows flipped in the fixing
   commit; `solution_semantics.md` § 1.2 and `defined_behaviour.md` restated. Under (a): the read-out's
   `universe` row and `root.hypgen` gain the count, with [`read_out.md`](../../../docs/kernel/inference/read_out.md)
   and its reference test in the same commit.

## Acceptance

- The three witnesses and the census are on the stage's record, each with its command.
- The user has ruled; E-1, E-7 and E-54 are closed in the calculator's `findings.md` with what was done.
- If an answer moved, it moved by a register row, and the phase's README names this stage as an exception to
  *no answer changes*.

## What it must not do

- **Not remove a candidate before the ruling.**
- **Not reopen the ruling on a program's own open relations beside a theory:** left as it is.
- **Not touch the obligations rung or an hrule's candidates:** the findings are the blind enumerator's.

## The standing rules

A wrong answer is a row in `corpus/wrong_answers.toml`, filed `open` before its fix. Config never changes an
answer. The gate is `./run_tests.sh`, once on the last commit of the batch; `utils/ci_container.sh` is not
needed.

---

## Record, 2026-11-15: tasks 1 to 3

**No engine behaviour changed, no candidate was removed, no register row was filed, no expectation was
edited.** What was added: one measurement example, `ein-infer/examples/object_census.rs`; this record;
[Q-M1f.306](../open_questions.md#q-m1f306--what-the-blind-enumerator-takes-for-an-object); three findings in
the phase's [`findings.md`](findings.md#from-s1f109) (H9-1 to H9-3).

**The binary.** The release build of `d7050ccf` (`cargo build --release -p ein-cli`, in a worktree of that
commit), on AC. `ein --version`:

```
ein 0.1.0
protocol   ein-events/1
container  einb/2.0
features   einb, parallel, snmalloc
stdlib     sha256:3ed7f9d0f377124b7b8d8a723deaa85931bce8a41c8f597d9e957abe5aa3de8e  checkout …/stdlib
```

**The artefacts** are under the untracked `build/s1f.10.9/`: each witness, beside it its `ein solve` read-out
(`X.solve.txt`), its `ein oracle` read-out (`X.oracle.txt`) and dump (`X.oracle/region.json`, `region.md`),
and `X.without.txt`; `run.sh` and `run.txt`, the script that writes them and what it printed; `without.py`;
`census/rows.tsv`, the census's row for every corpus program, `census/witnesses.tsv`, the same example over the
witnesses, and `census/features01/`, one program's `--dump-lattice`. `build/` is not tracked, so every program
and command is below.

### What the code does, which is what the three findings share

[`hypgen::candidate_objects`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) is the KB's names index at
root's fixpoint, **which holds every fact's head beside its top-level arguments**, and
[`invariant::program_names`](../../../ein.rs/crates/ein-infer/src/invariant.rs), every constant in an argument
position of a fact at any depth or of a rule; less the names a relation's signature declares, less seven
reserved words (`not false and or absent eq neq`), less every name the registry calls a relation or a rule
(`Program::categorise`). **The registry holds a relation a `(relation …)` declares and the head of every fact a
file writes** (`ein-ir/src/from_ir.rs`: *Undeclared relations are auto-created open-world*), kernel dunders
included. It does not hold the head of a fact only a rule derives. So:

- **a head only a rule derives is an object.** E-1 is that head being `__closed__` or `__symmetric__`; E-7 is
  it being an ordinary undeclared relation. They are one mechanism;
- **a head a file writes a fact of, or declares, is not**, whether or not the program also writes it as a
  value (`e7/registered_as_value.ein`);
- every relation in the registry with a signature of one or two types and no scoping is proposed about, over
  every such object, wherever either was loaded from (E-54).

### T1f.10.9.1: the witnesses

`build/s1f.10.9/run.sh` runs every witness (`EIN_REPO=<checkout> EIN_BIN=<binary> build/s1f.10.9/run.sh`):

```sh
#!/bin/sh
# S1f.10.9, T1: every witness under `ein solve` and `ein oracle --dump`, and
# what is left of R without the marked candidates. It writes beside the
# witnesses; $EIN_REPO is the checkout (default: two directories up), and
# $EIN_BIN the binary (default: the checkout's release build).
cd "$(dirname "$0")" || exit 1
REPO=${EIN_REPO:-$(cd ../.. && pwd)}
EIN=${EIN_BIN:-$REPO/ein.rs/target/release/ein}
$EIN --version > version.txt
one() {  # FILE [without.py's arguments]
    f=$1; shift
    b=${f%.ein}
    $EIN solve "$f" > "$b.solve.txt" 2>&1; echo "exit $?" >> "$b.solve.txt"
    rm -rf "$b.oracle"
    $EIN oracle "$f" --dump "$b.oracle" > "$b.oracle.txt" 2>&1; echo "exit $?" >> "$b.oracle.txt"
    python3 without.py "$b.oracle" "$@" > "$b.without.txt" 2>&1
    printf '== %s\n' "$f"; grep -E '^  (states|commitments|universe)' "$b.oracle.txt"; cat "$b.without.txt"
}
one e1/closed_by_rule.ein        --name __closed__
one e1/closed_by_fact.ein        --name __closed__
one e1/shipped_arith.ein         --name __closed__
one e1/shipped_decimal_arith.ein --name __closed__
one e1/dunder_as_value.ein       --name __closed__
one e1/symmetric_by_rule.ein     --name __symmetric__
one e7/names.ein                 --name thing --name liquid
one e7/names_tasty.ein           --name thing --name liquid
one e7_declared/names_tasty.ein  --name thing --name liquid
one e7/self.ein                  --name liquid
one e7/self_declared.ein         --name liquid
one e54/names_s.ein              --name red --name square --relation lib.shape
one e54/names_w.ein              --name red --name square --relation lib.shape --relation lib.colour
one e54/names_s_no_shape.ein     --name red --name square --relation lib.shape
one e7/registered_as_value.ein   --name near --name likes
one h9/eq_as_value.ein           --name eq
one h9/eq_as_value_written.ein   --name eq
# The registry, where the closure is written and where a rule derives it.
for f in e1/closed_by_fact.ein e1/closed_by_rule.ein; do
    printf '== %s, the relations the loader made up\n' "$f"
    $EIN saturate "$f" --dump | grep -A1 'auto-vivified' || echo '(none)'
done
# E-7 on the corpus's one program, past the oracle's limit: the nodes the
# default-budget describe computed, and those of R that hold a marked candidate.
mkdir -p census/features01
( cd "$REPO" && "$EIN" solve examples/features/01_not_and_absent.ein \
    --dump-lattice "$OLDPWD/census/features01/lattice" > "$OLDPWD/census/features01/solve.txt" 2>&1 )
python3 - <<'PY'
import json
d = json.load(open('census/features01/lattice/nodes.json'))
marked = [c for c in d['candidates'] if 'explicitly-dislikes' in c]
r = [n for n in d['nodes'] if n['kind'] == 'in-r']
held = [n for n in r if any('explicitly-dislikes' in c for c in n['commitment'])]
print('== examples/features/01_not_and_absent.ein, by the nodes describe computed')
print('H: %d candidates, %d marked' % (len(d['candidates']), len(marked)))
print('%d nodes computed, %d in R, %d of those hold a marked candidate; the smallest: %s' % (
    len(d['nodes']), len(r), len(held), min(held, key=lambda n: len(n['commitment']))['commitment']))
PY
```

`build/s1f.10.9/without.py` reads an oracle dump:

```python
#!/usr/bin/env python3
"""R of an `ein oracle --dump DIR`, and what is left of it without the
candidates that name one of the given names or are of one of the given
relations: the commitments of R that hold none of them.

    python3 without.py DIR [--name NAME]... [--relation REL]...
"""
import json, sys

def main(argv):
    names, rels, dump = [], [], argv[1]
    it = iter(argv[2:])
    for a in it:
        (names if a == "--name" else rels).append(next(it))
    d = json.load(open(dump + "/region.json"))
    cands = d["candidates"]
    if d["region"] is None:
        print("H: %d candidates. No cube: past the oracle's limit of %d" % (len(cands), d["limit"]))
        return
    def marked(c):
        head, *args = c.strip("()").replace("(", " ").replace(")", " ").split()
        return head in rels or any(a in names for a in args)
    mask = sum(1 << i for i, c in enumerate(cands) if marked(c))
    r = [n for n in d["region"]["nodes"] if n["status"] == "R"]
    left = [n for n in r if n["mask"] & mask == 0]
    print("H: %d candidates, %d marked: %s" % (
        len(cands), bin(mask).count("1"),
        " ".join(c for c in cands if marked(c)) or "-"))
    print("R: %d commitments, %d states" % (len(r), len({n["class"] for n in r})))
    print("R without the marked: %d commitments, %d states" % (
        len(left), len({n["class"] for n in left})))

main(sys.argv)
```

**How *R without them* is computed, three ways that agree.** A node is root's fixpoint with a commitment
written in and saturated, whatever else H holds, so R over H less a set J of candidates is exactly the
commitments of R that hold no member of J. (1) `without.py` counts those in `ein oracle --dump`'s cube. (2) `ein
oracle` on a **copy of the program** in which the marked candidates do not arise, by a means the language has:
the closure written as a fact (E-1), the derived relation declared and scoped out (E-7), the import's relation
scoped out (E-54's relation half). (3) The census example's own count on the same file
(`census/witnesses.tsv`). On every witness the three give one number. The stage's wording, *a copy of the
program that closes the relation*, would close `total` itself and leave no candidate at all; the copies below
remove the marked candidates and no other.

In every table `k_c = k_s`: no witness merges two commitments into one state.

#### E-1, `__closed__` as an object

`e1/closed_by_rule.ein`, the prototype's shape rebuilt, since both were found on the prototype:

```
;; S1f.10.9, E-1: a kernel dunder among the blind enumerator's objects.
;; The shape of the calculator's prototype: a table closed by its own rule,
;; beside a relation the program declares and does not close.
(relation digit Digit :why "{?1} is a digit")
(digit 0) (digit 1) (digit 2) (digit 3) (digit 4)
(digit 5) (digit 6) (digit 7) (digit 8) (digit 9)
(rule digits-closed () :match () :assert (__closed__ digit)
  :why "the digits are all written: none is ever guessed")
(relation total Number :why "the total is {?1}")
(query :goal (total ?x))
```

`e1/closed_by_fact.ein`, its copy:

```
;; S1f.10.9, E-1's copy: closed_by_rule.ein with the closure written as a
;; fact and not derived by a rule. Nothing else differs.
(relation digit Digit :why "{?1} is a digit")
(digit 0) (digit 1) (digit 2) (digit 3) (digit 4)
(digit 5) (digit 6) (digit 7) (digit 8) (digit 9)
(__closed__ digit)
(relation total Number :why "the total is {?1}")
(query :goal (total ?x))
```

`e1/shipped_arith.ein` and `e1/shipped_decimal_arith.ein`, the shipped stdlib:

```
;; S1f.10.9, E-1 on the shipped stdlib: std.math.arith closes its flag by its
;; closing rule, and writes no (__closed__ R) fact.
(import std.math.arith)
(relation total Number :why "the total is {?1}")
(query :goal (total ?x))
```

```
;; S1f.10.9, E-1 as the finding words it, on the shipped stdlib: the
;; calculator imported beside a relation the program does not close.
;; std.math.digits, which it brings, writes (__closed__ R) facts.
(import std.math.decimal-arith)
(relation total Number :why "the total is {?1}")
(query :goal (total ?x))
```

`e1/dunder_as_value.ein`, the dunder written as a value:

```
;; S1f.10.9, E-1's neighbour: a dunder written as a value, which Q-M1f.7 (b)
;; keeps legal, as std.algebra's (imply1 functional __closed__) is.
(relation thing Thing :why "{?1} is a thing")
(thing a)
(rule lift (?P ?Q) :match (?P ?r) :assert (?Q ?r)
  :why "{?r} is {?P}, so it is {?Q}")
(lift complete __closed__)
(complete thing)
(relation total Number :why "the total is {?1}")
(query :goal (total ?x))
```

`e1/symmetric_by_rule.ein`, the other dunder:

```
;; S1f.10.9, E-1 for the other dunder: the only (__symmetric__ R) fact is
;; one a rule derives.
(relation near Thing Thing :why "{?1} is near {?2}")
(near a b)
(rule near-symmetric () :match () :assert (__symmetric__ near)
  :why "near holds both ways round")
(relation total Number :why "the total is {?1}")
(solve :no-hypothesis (near))
(query :goal (total ?x))
```

| witness | H today | marked | R, `ein oracle` | R without the marked | the copy's R, `ein oracle` |
|---|---|---|---:|---:|---:|
| `closed_by_rule` | 11: `(total 0)` to `(total 9)`, `(total __closed__)` | `(total __closed__)` | 2 048 | 1 024 | `closed_by_fact`: 10 candidates, 1 024 |
| `shipped_arith` | 1: `(total __closed__)` | `(total __closed__)` | 2 | 1 | — |
| `shipped_decimal_arith` | 13: `(total 0)` to `(total 9)`, `(total 10)`, `(total gt)`, `(total lt)` | none | 8 192 | 8 192 | — |
| `dunder_as_value` | 2: `(total __closed__)`, `(total a)` | `(total __closed__)` | 4 | 2 | — |
| `symmetric_by_rule` | 3: `(total __symmetric__)`, `(total a)`, `(total b)` | `(total __symmetric__)` | 8 | 4 | — |

- **E-1 reproduces, 11 candidates and k_s 2 048, on the prototype's shape.** The marked candidate doubles R.
- **It does not reproduce as the finding words it.** *Where … the program writes a `(__closed__ …)` fact of
  another* is not the condition: a written `(__closed__ R)` registers `__closed__` as an auto-vivified relation
  (`ein saturate e1/closed_by_fact.ein --dump` prints *Relations — auto-vivified (1)*, `__closed__`; on
  `closed_by_rule.ein` it prints none), and a registered relation is no object. The dunder is an object only
  where **every** `(__closed__ …)` fact of the program is derived by a rule, or where a program writes the
  dunder as a value. The prototype closed its tables by a rule. The shipped `std.math.digits` writes its
  closures as facts, so **the shipped calculator beside an open relation proposes no dunder**
  (`shipped_decimal_arith`); `std.math.arith` alone, whose closing rule derives its closures, does.
- **So one closure stated two ways gives two answers today**: `closed_by_rule` and `closed_by_fact` differ in
  nothing but how `(__closed__ digit)` comes to hold, and R is 2 048 states against 1 024.
- `__symmetric__` behaves the same (`symmetric_by_rule`).

#### E-7, a relation's name as an object

`e7/onto.ein` and `e7/names.ein` are the import probes of the finding, unchanged:

```
;;; an ontology library: facts, with one relation declared and one not, two rules and a macro
(relation is-a Thing Type :why "{?1} is a {?2}")
(relation unused Thing :why "{?1} is never mentioned")
(is-a Tea Drink)
(is-a Milk Drink :source "the ontology")
(likes Tea Milk)
(rule things () :match (is-a ?x Drink) :assert (thing ?x) :why "{?x} is a drink, so a thing")
(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why "{?x} is a drink, so a liquid")
(macro both (?a ?b) (and ?a ?b) :why "{?a} and {?b}")
```

```
;;; probe: do an imported fact's constants become names of the program?
(import onto :as o)
(relation tasty Thing :why "{?1} is tasty")
(tasty Coffee)
```

`e7/names_tasty.ein`, the same within the oracle's limit:

```
;; S1f.10.9, E-7 within the oracle's limit: names.ein with the library's two
;; declared relations scoped out, so H is over `tasty` alone.
(import onto :as o)
(relation tasty Thing :why "{?1} is tasty")
(tasty Coffee)
(solve :no-hypothesis (o.is-a o.unused))
```

`e7_declared/onto.ein` and `e7_declared/names_tasty.ein`, its copy:

```
;;; e7/onto.ein with `thing` and `liquid` declared. Nothing else differs.
(relation is-a Thing Type :why "{?1} is a {?2}")
(relation unused Thing :why "{?1} is never mentioned")
(relation thing Thing :why "{?1} is a thing")
(relation liquid Thing :why "{?1} is a liquid")
(is-a Tea Drink)
(is-a Milk Drink :source "the ontology")
(likes Tea Milk)
(rule things () :match (is-a ?x Drink) :assert (thing ?x) :why "{?x} is a drink, so a thing")
(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why "{?x} is a drink, so a liquid")
(macro both (?a ?b) (and ?a ?b) :why "{?a} and {?b}")
```

```
;; S1f.10.9, E-7's copy: e7/names_tasty.ein over the library that declares
;; `thing` and `liquid`, both scoped out with the other two.
(import onto :as o)
(relation tasty Thing :why "{?1} is tasty")
(tasty Coffee)
(solve :no-hypothesis (o.is-a o.unused o.thing o.liquid))
```

`e7/self.ein` and `e7/self_declared.ein`, one file and its copy:

```
;; S1f.10.9, E-7 in one file: `liquid` is the head of a fact only a rule
;; derives. No file writes a fact of it and no (relation …) declares it.
(relation is-a Thing Type :why "{?1} is a {?2}")
(is-a Tea Drink)
(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why "{?x} is a drink, so a liquid")
(relation tasty Thing :why "{?1} is tasty")
(solve :no-hypothesis (is-a))
(query :goal (tasty ?x))
```

```
;; S1f.10.9, E-7's copy: self.ein with `liquid` declared, and scoped out as
;; `is-a` is. Nothing else differs.
(relation is-a Thing Type :why "{?1} is a {?2}")
(is-a Tea Drink)
(relation liquid Thing :why "{?1} is a liquid")
(rule liquids () :match (is-a ?x Drink) :assert (liquid ?x) :why "{?x} is a drink, so a liquid")
(relation tasty Thing :why "{?1} is tasty")
(solve :no-hypothesis (is-a liquid))
(query :goal (tasty ?x))
```

`e7/registered_as_value.ein`, the control:

```
;; S1f.10.9, E-7's control: a relation's name written as a value is no
;; object where the registry holds the relation, declared (`near`) or the
;; head of a written fact (`likes`).
(relation tasty Thing :why "{?1} is tasty")
(relation near Thing Thing :why "{?1} is near {?2}")
(marker near)
(likes Tea Milk)
(marker likes)
(solve :no-hypothesis (near))
(query :goal (tasty ?x))
```

| witness | H today | marked | R, `ein oracle` | R without the marked | the copy's R, `ein oracle` |
|---|---|---|---:|---:|---:|
| `names` | 39, over `o.is-a`, `o.unused` and `tasty` | 22, each naming `liquid` or `thing` | not reached, 39 over the limit of 16 | — | — |
| `names_tasty` | 5: `(tasty Drink)`, `(tasty Milk)`, `(tasty Tea)`, `(tasty liquid)`, `(tasty thing)` | `(tasty liquid)`, `(tasty thing)` | 32 | 8 | `e7_declared/names_tasty`: 3 candidates, 8 |
| `self` | 3: `(tasty Drink)`, `(tasty Tea)`, `(tasty liquid)` | `(tasty liquid)` | 8 | 4 | `self_declared`: 2 candidates, 4 |
| `registered_as_value` | 2: `(tasty Milk)`, `(tasty Tea)` | none: neither `near` nor `likes` is an object | 4 | 4 | — |

- **E-7 reproduces**: `liquid` and `thing` are objects. On `names`, `ein solve` reads *R is decided: ∅ and ⊤
  are both in it, so it is the whole cube (L1, L2)*, k_c 549 755 813 888, which is 2³⁹; the 17 unmarked
  candidates span 2¹⁷ = 131 072 of those commitments. That figure is arithmetic on the read-out's, not a run.
- **The finding's title does not describe its witness.** `liquid` and `thing` are never written in an argument
  position: no rule *asserts them as values*. They are heads, of facts `things` and `liquids` derive, and they
  are objects because the names index holds heads and the registry does not hold these two. **So Q-M1e2.9 (a)
  does not make them names**: `invariant::program_names`, which built that ruling, lists argument positions
  and says of a head that it *names a relation*; neither is on it. [§ Why it is not simply a
  fix](#why-it-is-not-simply-a-fix)'s first bullet, *a relation name a rule asserts as a value is, read
  literally, such a constant (E-7)*, is true of a shape the witness does not have.
- **A relation's name that the program does write as a value is not an object**, where a file declares the
  relation or writes a fact of it (`registered_as_value`). So today the rule is not *a constant a fact names is
  a candidate* either: it is *unless the registry calls it a relation*.
- **Declaring a derived relation changes H twice**: its name stops being an object, and the relation becomes
  one the blind enumerator proposes about, until it is scoped out (the two copies scope it out).

#### E-54, what an import brings

`e54/lib.ein`, `e54/names_s.ein` and `e54/names_w.ein` are the finding's probes, unchanged:

```
;; A library: two relations it declares and does not close, and facts of each.
(relation colour Thing :why "{?1} is a colour")
(relation shape Thing :why "{?1} is a shape")
(colour red)
(shape square)
```

```
;; Import invariance probe: the blind enumerator over the program's declared relations and its names.
(import lib :symbols (colour))
(relation near Thing Thing :why "{?1} is near {?2}")
(query :goal (near ?x ?y))
```

```
;; Import invariance probe: the blind enumerator over the program's declared relations and its names.
(import lib)
(relation near Thing Thing :why "{?1} is near {?2}")
(query :goal (near ?x ?y))
```

`e54/names_s_no_shape.ein`, the copy for the relation half:

```
;; S1f.10.9, E-54's copy for the relation half: names_s.ein with the relation
;; the main file does not spell scoped out. Nothing else differs.
(import lib :symbols (colour))
(relation near Thing Thing :why "{?1} is near {?2}")
(solve :no-hypothesis (lib.shape))
(query :goal (near ?x ?y))
```

| witness | H today | marked | R, `ein oracle` | R without the marked | the copy's R, `ein oracle` |
|---|---|---|---:|---:|---:|
| `names_s` | 4: `(colour square)`, `(lib.shape red)`, `(near red square)`, `(near square red)` | all 4: each names `red` or `square`, which only `lib` writes; `(lib.shape red)` is also of a relation the main file does not spell | 16 | 1, R = {∅}; without `(lib.shape red)` alone, 8 | `names_s_no_shape`: 3 candidates, 8 |
| `names_w` | 4: `(lib.colour square)`, `(lib.shape red)`, `(near red square)`, `(near square red)` | all 4; two are of a relation the main file does not spell | 16 | 1; without the two of `lib.colour` and `lib.shape`, 4 | — |

- **E-54 reproduces: 4 candidates, 16 states, under both import forms.** So does its reach beyond its own
  witness: `e1/shipped_decimal_arith`'s 13 candidates each name a digit or an order that only
  `std.math.digits` writes, and without them its R is {∅} (`census/witnesses.tsv`).
- **Every candidate is marked**, because the main file writes no name at all: `red` and `square` are the
  library's. Without them R is {∅}. No copy shows that by a means the language has short of scoping every
  relation out, so the 1 is the cube's count (`without.py`, and the census example).
- **The relation half depends on what *the main file writes it* means.** If an import's `:symbols` list counts,
  as the census counts it, `names_s` spells `colour` and `names_w` does not, and the two forms of one program
  would be left with 8 states and 4: the answer would depend on the import form, which
  `ein-cli/tests/import_invariance.rs` holds it does not. If a listing does not count, both are 4.

### T1f.10.9.2: the census

`ein.rs/crates/ein-infer/examples/object_census.rs`, a measurement example beside `mechanism_census`: it loads
each corpus program, saturates root and takes H as `region::prepare` does (`shared/census.rs`'s `root_of`),
marks every candidate, and where the blind enumerator built H, a candidate is marked and `n` is within the
oracle's limit, runs the oracle's own cube (`region::compute`) on a second load and counts the commitments and
states of R that hold no marked candidate. Its header states each set.

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example object_census > build/s1f.10.9/census/rows.tsv
grep '^#' build/s1f.10.9/census/rows.tsv        # the summary
```

2026-11-15, at `d7050ccf` with the example added, the oracle's default limit of 16.

**What was read.** 867 corpus files. 223 do not load, on any command: the 65 negative fixtures (35
`load-negative`, 30 `parse-negative`) and 158 register witnesses whose answer is a refusal (`ein saturate` exits
non-zero on each). 644 are read:

| the rung that built H | programs | with a candidate | with a marked candidate |
|---|---:|---:|---:|
| blind | 461 | 106 | 3 |
| declined, so blind | 9 | 9 | 0 |
| an hrule | 88 | 83 | 0 (not the blind enumerator's) |
| the obligations | 62 | 20 | 5 (not the blind enumerator's) |
| `(solve :no-search)` | 21 | 0 | 0 |
| stuck | 3 | 0 | 0 |

**Where the blind enumerator built H: 470 programs, 115 with a candidate, 2 190 candidates** (26 of the 115 hold
more than 16). The 355 others have no candidate at all: nothing in them is left open for the enumerator to
propose about.

| the candidate names | programs | candidates | cube within the limit | R differs without them | not reached |
|---|---:|---:|---:|---:|---:|
| **E-1**: a kernel dunder, never written as a value (`K1`) | 0 | 0 | — | — | — |
| E-1's neighbour: a kernel dunder also written as a value (`K2`) | 0 | 0 | — | — | — |
| **E-7**: a relation's name, never written as a value (`R1`) | 1 | 16 | 0 | — | 1 |
| a relation's name the program also writes as a value (`R2`) | 0 | 0 | — | — | — |
| **E-54**: a name only an imported module writes (`IN`) | 2 | 4 | 2 | **2** | 0 |
| **E-54**: a candidate of a relation the main file does not spell (`IR`) | 0 | 0 | — | — | — |
| any of them | 3 | 20 | 2 | 2 | 1 |

The three programs:

| program | H | marked | R | R without the marked |
|---|---:|---|---:|---:|
| `examples/ein-bugs/wrong-answers/import-directives-as.ein` | 2 | both, `(chosen Drink)` and `(chosen Tea)`: the module's names | 4 commitments, 4 states | 1, 1 |
| `examples/ein-bugs/wrong-answers/import-directives-twice.ein` | 2 | both, the same | 4, 4 | 1, 1 |
| `examples/features/01_not_and_absent.ein` | 35 | 16, each naming `explicitly-dislikes`, the head of what its rule `see-stored-neg` derives | **not reached by the oracle**: 2³⁵, past the limit; the program is flagged *not monotone* | not reached |

- **E-1: no corpus program has a dunder among its candidates.** No corpus answer rests on one.
- **E-7: one program, and the oracle does not reach it.** What is computed about it is by another instrument
  and is named as such: the default-budget `ein solve --dump-lattice` computes 64 655 nodes before it stops,
  every one in R, and **58 966 of them hold a marked candidate**, the smallest `{(is-a Alice
  explicitly-dislikes)}` (`run.sh`'s last step). Each is a node by the function the oracle shares, so **R
  holds commitments that a list without the 16 would not have**: its R would differ. How many, nobody has
  computed.
- **E-54: two programs, both register witnesses of W90, and R differs on both**, 4 states to 1. Their `fixed`
  rows state the right answer by hand over the module's names: *its hypotheses are the facts of its open
  declared relation over its names: `(chosen Drink)` and `(chosen Tea)`* (the witness's header). So the
  register today holds the reading E-54 questions as **right**, twice.
- **The self-check held**: no program without an `(import …)` has an `IN` or `IR` mark.

**Where another rung built H**, its candidates carry the same marks on 5 programs, all under
`examples/binary_logic/`, on the obligations rung: `IN` on 28 candidates (`0`, `1` and the gate functions'
names, which the stdlib's Boolean modules write), and `R2` on 16 of `synthesis.ein`'s (the gate functions,
which `std.bool.gates` declares *as a relation and as a value*).
These are what an obligation owes, not what the blind enumerator guesses, and no option of the question
touches them.

**What the blind enumerator would propose there, were it asked** (`hypgen::generate_blind` at root; not H,
decides nothing): a candidate on 130 of those programs, 86 943 candidates in all, of which a dunder (`K1`) on
**8 programs, 458 candidates**, and a derived relation's name (`R1`) on **4 programs, 200 candidates**:

| programs | rung today | the blind enumerator alone would name |
|---|---|---|
| `examples/binary_logic/{equivalent,knights,not_equivalent,satisfiable,unsatisfiable}.ein`, `stdlib/bool/inputs.ein`, `tests/stdlib/bool/inputs/{01_in_drives,02_required}.ein` | obligations | `__closed__`, 36 to 78 candidates each |
| `examples/domain_elim/{ab,b_branch,b_only}.ein` | an hrule | `surjective`, `total`: 52 each |
| `examples/features/09_adjacent_via_same_house.ein` | obligations | `adjacent-via-bwd`, `adjacent-via-fwd`: 44 |

So E-1 and E-7 are absent from the corpus's answers because of the rung those twelve programs take, not
because nothing derives a dunder or an undeclared head in them. [S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md)'s
question is which rung a program takes; a ruling there that sends a program to the blind enumerator re-takes
this census (H9-3).

### T1f.10.9.3: the question, and the stop

[Q-M1f.306](../open_questions.md#q-m1f306--what-the-blind-enumerator-takes-for-an-object), options (a) to (e)
as [§ Tasks](#tasks) states them, each with what it would change on the witnesses and on the corpus; the
default as this file names it; **not acted on before the user replies**.

### What contradicts the plan

1. **E-1's condition.** Not *where the program writes a `(__closed__ …)` fact*: where every such fact is
   derived by a rule, or the dunder is written as a value. The shipped calculator beside an open relation does
   not reproduce it; the rebuilt prototype shape and `std.math.arith` alone do.
2. **E-7's reading.** The witness's names are heads, never values, and Q-M1e2.9 (a) as built does not list
   them. So the reason this file gives for defaulting (a) on E-7, *since it is the ruling as written*, does not
   hold of the witness, and option (c)'s *which narrows Q-M1e2.9* does not either: (c) would bring a head only
   a rule derives into line with every head the registry already holds, and take nothing from
   `program_names`. **The default is left as this file names it and the question says this beside it.**
3. **E-1 and E-7 are one mechanism**, so (b) without (c) is a rule about two names inside a behaviour that
   stays.
4. **E-54's *no tracked program reaches it***, which was said of the import forms agreeing: two tracked
   programs' R is over names only a module writes, and the register holds that R as right.
5. **The census's *R would differ* is by the oracle on two programs of three.** The third is past its limit
   and is decided by nodes a describe computed, named so above.

### Not done

- Task 4, by instruction: nothing is removed, filed, fixed or restated.
- E-1, E-7 and E-54 are not closed in the calculator's `findings.md`: the acceptance closes them on the
  ruling. Each row there has a line pointing here.
- The phase README's status line and stage table are not edited: another stage is running on them.
- The main tree's binary was not used, and nothing was run twice on another build.
- `./run_tests.sh` was not run, by instruction. Run: `cargo fmt --all --check`, `cargo clippy --workspace
  --all-targets -- -D warnings` (both as `run_tests.sh` runs them, clean), and `python3 utils/doc_audit.py
  --links --check`.
