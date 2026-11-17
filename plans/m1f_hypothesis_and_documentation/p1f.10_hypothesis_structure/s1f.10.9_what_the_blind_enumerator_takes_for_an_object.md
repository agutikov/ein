# S1f.10.9 — What the blind enumerator takes for an object

**Phase:** [P1f.10](README.md), track H (what H is)
**Status:** planned 2026-11-12; at the re-plan of 2026-11-15, E-54 added and its stop made the phase's one
stop. **Tasks 1 to 3 ran on 2026-11-15** ([§ Record](#record-2026-11-15-tasks-1-to-3)): the witnesses rebuilt
and banked, the census taken, the question written as
[Q-M1f.306](../open_questions.md#q-m1f306--what-the-blind-enumerator-takes-for-an-object). Stopped at the
question, which was put at the phase's one stop ([S1f.10.12](s1f.10.12_the_structure_shown.md)). **Ruled
2026-11-16, and task 4 ran the same day** ([§ Record](#record-2026-11-16-task-4)): (b) for E-1, *"dunder never
an object, wide reading"*, so the blind enumerator's objects leave the kernel's two dunders out and the
register's W96, four rows, is filed and fixed; (a) for E-7, left and named by a `relation names` row; (d) for
E-54, left. **Done.** H9-1, H9-4 and H9-5 stay open in [`findings.md`](findings.md#from-s1f109). **At the phase's
close** ([§ Record](#record-2026-11-16-h9-1-and-h9-4-as-ruled-at-the-close)), on the user's *"File and fix them in
this phase"*: H9-4 filed and fixed as the register's W98; H9-1 stopped for the user, two rulings deciding it
opposite ways.
**Depends on:** nothing in the phase. [S1f.10.1](s1f.10.1_exclusion_census.md)'s census reads the same
candidate sets, and the bundle marks the candidates this stage counts.
**Source:** row 56 of section J of the
calculator's questions, E-1 and E-7 of its `findings.md`. The row's
suggestion was *(b) P1f.10, which plans how the candidates are built*; the user left the line empty on
2026-11-12, and an empty line takes the recommendation
(round 2, § Left empty). So this stage stands on an empty
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
   `universe` row and `root.hypgen` gain the count, with [`read-out.md`](../../../docs/ein/user/reference/read-out.md)
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

---

## Record (2026-11-16): task 4

**The rulings.** The user, 2026-11-16, shown the phase's four questions with their defaults at the stop: *"all
defaults ok"*. For [Q-M1f.306](../open_questions.md#q-m1f306--what-the-blind-enumerator-takes-for-an-object)
that is **(b) for E-1**, a kernel dunder is never an object; **(a) for E-7**, left, and the read-out names such
candidates; **(d) for E-54**, left. The entry had left one thing to the user that no default covered, whether
*never* covers a dunder a program writes as a value. It was asked back, and the user ruled it the same day:
*"dunder never an object, wide reading"*. The task was started on the narrow reading and nothing of it was
committed; what is built is the wide one.

**The commits**, on the branch `worktree-agent-a5f864753ca542a7f`, from `3e59b90a`:

| commit | what |
|---|---|
| `200df470` | W96 filed `open`, four rows, each with its witness and corpus entry; finding H9-4 |
| `4ee131e3` | the fix, the rows flipped, `dunder_objects.rs`, the docs of what an object is, 271 golden lines; H9-5 |
| `f475ed2e` | W96's `[case.fixed]` names `4ee131e3` |
| `05bebca9` | E-7's naming: `relation_names.rs`, the `relation names` row, the summary's key, their pages and tests |
| this one | the record, the phase's status, Q-M1f.306's *Built*, the calculator's three rows closed, and `AGENTS.md`'s standing rule on the two dunders, which gains *never an object* in place |

### What an object is, and where it is decided

One place: [`hypgen::candidate_objects`](../../../ein.rs/crates/ein-infer/src/hypgen.rs), the list the blind
enumerator fills every slot from. It was the KB's names index at the state, which holds the head of every fact,
united with [`invariant::program_names`](../../../ein.rs/crates/ein-infer/src/invariant.rs); less a signature's
type roles, seven kernel words, and what the registry calls a relation or a rule. **It now also leaves out
`terms.kernel.closed` and `terms.kernel.symmetric`, by name**: three lines, an array of the two and one clause
of the filter. So a dunder is no object

- where the names index holds it as the head of a fact a rule derives and no registry holds it, the three
  witnesses the narrow reading would have moved; and
- where a program writes it in an argument position, `(lift complete __closed__)`, the fourth.

**`invariant::program_names` is not changed.** It and the enumerator's objects were one list with two readers:
`candidate_objects` unions it in, and `Universe::of` takes it whole as the alive-set invariant's baseline. The
exclusion is made in `candidate_objects`, after the union, so the invariant's list still holds a dunder
wherever a program writes one as a value, a rule that concludes a fact naming it breaches nothing, and nothing
is refused at load: Q-M1f.7 (b), *used as a relation or a value*, stands. `predicates::domain`, the third
reader of that list (what an unbound `eq`/`neq` variable ranges over), is unchanged too.
`ein-infer/tests/dunder_objects.rs`'s last test holds both halves on the fourth witness: the name is on the
list, and the object is not in the pool. `alive_invariant.rs`'s seed check, which holds every name of the
program to be an object but the kernel's words, lists the two dunders among those words; no corpus program
needed it.

**The invariances that hold it**, `ein-infer/tests/dunder_objects.rs`, each by the cube (`region::compute`,
`ein oracle`'s library) and held to a bare describe's counts:

| test | what is equal |
|---|---|
| `a_closure_by_a_rule_answers_what_the_closure_by_a_fact_answers` | four pairs, the rule form against the fact form: the rung, H in its order, R commitment for commitment, the states. The register's witness and its control read from their files; a closure by a rule with an empty `:match`; one by a rule that reads the table; a symmetry by a rule. No H names a dunder |
| `a_dunder_written_as_a_value_is_no_object` | the fourth witness against itself with `(__closed__ thing)` also written, which registers the dunder and changes nothing else: one H, `(total a)`, and one R, two states |
| `the_shipped_closing_rule_leaves_no_dunder_to_propose` | `(import std.math.arith)` beside an open `total` against the same with `total` closed: no candidate, R {∅} |
| `the_programs_names_hold_a_dunder_where_it_is_written_as_a_value` | on every form: whether `program_names` holds a dunder, and whether `candidate_objects` does |

and one unit test in `hypgen.rs`, `candidate_objects_never_hold_a_kernel_dunder`. The answer does not depend on
the import form, the id order or a lever: `import_invariance.rs`, `id_order_invariance.rs` and
`lever_invariance.rs` ran green over the corpus with the five new programs in it, and `explore.rs` holds every
mode to the oracle.

### The register: W96

Four rows of one case, filed `open` in `200df470` with today's answer and the right one as data, flipped in
`4ee131e3`. `[case.wrong]` is `ein solve` on the gate build of `3e59b90a`; `ein oracle` agreed on each.

| row | witness, under `examples/ein-bugs/wrong-answers/` | H, R before | H, R after |
|---|---|---|---|
| `W96/closed-by-rule` | `dunder-object-closed-by-rule.ein`: ten digits closed by a rule, `total` open | 11, 2 048 states | 10, 1 024, its control's |
| its `[case.control]` | `dunder-object-closed-by-fact.ein`: the closure written as a fact | 10, 1 024 | the same |
| `W96/arith-closing-rule` | `dunder-object-arith.ein`: `(import std.math.arith)`, `total` open | 1, 2 | none, 1: R {∅} |
| `W96/symmetric-by-rule` | `dunder-object-symmetric-by-rule.ein`: `(__symmetric__ near)` derived | 3, 8 | 2, 4 |
| `W96/as-value` | `dunder-object-as-value.ein`: `(lift complete __closed__)` | 2, 4 | 1, 2 |

The first row's run is `solve --maximal`, one state, since a `now` lists its states and 2 048 are too many to
write down; the counts are in its `[case.wrong]`, `[case.control]` and `[case.region]`. The other three ask all
of R. None states an `:expect`.

**The symmetric witness is not the entry's.** The entry's has `(near a b)` alone. With it in the corpus two
sweeps failed for a reason that is not E-1's: the saturation that derives a `(__symmetric__ R)` by a rule does
not mirror R, so root is not a fixpoint ([H9-4](findings.md#h9-4--a-__symmetric__-r-a-rule-derives-is-not-mirrored-by-the-saturation-that-derives-it)).
The witness writes `near` both ways round, which leaves the mirror nothing to add; its H and R are the entry's,
3 and 8 before, 2 and 4 after.

### The corpus diff

**Method.** `ein solve FILE -H --json-summary OUT` on every one of the 872 corpus programs, the gate build of
`3e59b90a` against the gate build of the fix, both on the checkout's stdlib (`EIN_STDLIB`): stdout, the first
line of stderr, the exit code and the summary compared, its timing fields aside. The script and what it printed
are untracked, under the session's scratch directory.

**No existing program's H, R, read-out, stderr or exit code moved.** 26 programs differ: the four witnesses, in
their answers, and 22 in one field of the summary and nothing else.

- **`root.hypgen.pre_candidate.closed_relation` drops on 22 programs.** The counter counts one skip per focal
  object and closed relation. On these programs the dunder was an object that proposed nothing, every relation
  it was tried on being closed, so it was walked and counted and never reached H. `tests/imports/lower.ein` 20 →
  16; `stdlib/bool/props.ein` and the seven programs under `tests/stdlib/bool/props/` 1 026 → 972;
  `stdlib/math/arith.ein` 2 → the key absent. The 22: `examples/binary_logic/{full_adder,simulation}.ein`;
  `stdlib/math/{arith,positional}.ein`; `stdlib/bool/{circuit,gates,props}.ein`;
  `tests/stdlib/bool/circuit/{01_propagate,02_one_value}.ein`, `gates/01_a_relation_a_function.ein`, `props/01`
  to `07`; `tests/imports/{lower,upper,upper_without_entry,01_nested_import_whole,02_nested_import_left_behind}.ein`.
  **No test and no page pins a value of the counter** (`grep -rn closed_relation docs/kernel ein.rs/crates
  utils`): `answer_dump.md` and `events.md` define the key, `hypgen_coverage.rs` names it. `closed.rs`'s header
  gave two corpus totals of it with no command behind them, and they are taken out.
- **Under the probe `EIN_LEFTOVER=1`**, which asks the blind enumerator alone at each listed state and is in no
  golden, the same sweep finds six more: `leftover.states` drops on
  `examples/binary_logic/{knights,not_equivalent,satisfiable}.ein`, `stdlib/bool/inputs.ein` and
  `tests/stdlib/bool/inputs/{01_in_drives,02_required}.ein` (`knights` 867 → 809 a state). They are six of the
  eight programs of [H9-3](findings.md#h9-3--e-1-and-e-7-are-absent-from-the-corpuss-answers-by-the-rung-twelve-programs-take),
  whose H the obligations rung builds: the count is of what the blind enumerator would still propose, and it
  proposed about the dunder.

**The goldens**, `ein-render/tests/golden/corpus_shapes.md5`, 271 lines of 189 programs, re-banked per commit
and under `EIN_CORPUS_SLOW=1` in `4ee131e3`, on the main session's decision of 2026-11-16 (the stop in this
task's brief, *an existing program's golden moves*, was reached and reported before anything was re-banked):

| lines | programs | why |
|---|---|---|
| 64 | the four witnesses, 16 each: `hyp`, `hyp+closed`, `read-out[…]` and `cores[…]` for describe, maximal, minimal, single and n2, `oracle`, `region-lattice`, `region-trace`, `states` | their answers moved, by the rows. The control's lines did not |
| 22, `hyp` | the 22 programs above | the enumerator's narration at root: fewer skip lines, the dunder not walked |
| 185, `hyp+closed` | 96 under `examples/ein-bugs/wrong-answers/`, 14 `ein-bugs/`, 12 `features/`, 9 `branching/`, 5 `syntax/`, 18 `saturation/`, 2 `binary_logic/`, 1 `broken/compile/`; 3 `stdlib/bool/`, 2 `stdlib/math/`; 7 `tests/stdlib/bool/props/`, 5 `tests/imports/`, 4 `tests/stdlib/elim/`, 2 `tests/stdlib/macro/`, 2 `tests/stdlib/bool/circuit/`, 1 each of `tests/text/`, `tests/stdlib/bool/gates/`, `tests/stdlib/bijection/` | the op runs `closed::emit_closed`, **a pass no command calls**, before the enumerator. The pass writes `(__closed__ R)` facts itself, no registry holds the dunder, and under the op the enumerator proposed facts about it wherever the pass closed something |

That the re-bank moved no other line of an existing program, the check, run on the diff of `4ee131e3`:

```sh
git diff -U0 200df470 4ee131e3 -- ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5 \
  | grep '^[-+][0-9a-f]' | grep -v 'dunder-object-' | grep -v -E '::hyp(\+closed)?$' | wc -l      # 0
```

So no `cores[…]` line of the 22 holds the counter: the summary is in no digested rendering of theirs.
`ein-cli/tests/golden/corpus_exits.txt` gained the five programs' ten cells in `200df470` and did not move in
`4ee131e3`.

**`hyp+closed` digests a pass no command calls.** It is how the corpus held E-1 in its goldens while no
program's H held a dunder, and why the census of task 2, which read H as the commands build it, counted none.
The op and the pass are as they were; whether they stay is
[S1f.10.16](s1f.10.16_what_the_walk_left.md)'s, and it is filed as
[H9-5](findings.md#h9-5--the-shape-goldens-hypclosed-op-digests-a-pass-no-command-calls).

**The census, re-taken on the final build**:

```sh
cargo run --profile gate --manifest-path ein.rs/Cargo.toml -p ein-infer --example object_census
```

2026-11-16, at `05bebca9`, the oracle's default limit of 16. 872 corpus files, 223 that do not load, 649 read.

| the rung that built H | programs | with a candidate | with a marked candidate |
|---|---:|---:|---:|
| blind | 466 | 110 | 3 |
| declined, so blind | 9 | 9 | 0 |
| an hrule | 88 | 83 | 0 |
| the obligations | 62 | 20 | 5 |
| `(solve :no-search)` | 21 | 0 | 0 |
| stuck | 3 | 0 | 0 |

Where the blind enumerator built H: 475 programs, 119 with a candidate, 2 213 candidates. That is task 2's 470,
115 and 2 190 and the five new files, four of them with a candidate, 10 + 10 + 2 + 1.

| the candidate names | task 2, 2026-11-15 | now |
|---|---|---|
| **a kernel name** (`K1`, `K2`), in H where the blind enumerator built it | 0 programs | 0 programs |
| the same, in what the blind enumerator alone would propose where another rung built H | `K1` on 8 programs, 458 candidates | **0**: 86 485 candidates on those 130 programs where it was 86 943, 458 fewer |
| **a relation's name** (`R1`), in H | 1 program, 16 candidates | 1 program, 16: `features/01`, the count the read-out's row prints |
| the same, alone | 4 programs, 200 | 4 programs, 200 |
| **a name only a module writes** (`IN`) | 2 programs, 4 candidates, R differs on both | the same |

So the example's marks agree with the engine: it finds no dunder among any candidate the blind enumerator
proposes, as H or alone, and its `R1` on the blind rung is the row's count.

### E-7: left, and named

The candidates stay and no answer moves. What was built is the naming, as
[`types_as_objects`](../../../ein.rs/crates/ein-infer/src/types_as_objects.rs) names a type bound as an object:

- [`ein_infer::relation_names`](../../../ein.rs/crates/ein-infer/src/relation_names.rs). Where the blind
  enumerator built H (the rung took `Blind`, or `Declined` and fell through), every argument of a candidate that
  is **the head of a fact root's fixpoint holds, or a relation a rule's `:assert` names as written**: the names,
  the relations of the candidates that name one, and how many candidates do. An argument of a candidate is an
  object, so no registry holds a relation of that name: no file declares it and none writes a fact of it.
- **The read-out**: a `relation names` row under `universe`, after `types`, on every question, `ein oracle`
  too: *explicitly-dislikes — a relation only a rule concludes, and 16 candidates of is-a, likes name it as an
  object: no file declares it or writes a fact of it*.
- **`--json-summary`**: `root.hypgen.relation_names_as_objects`, `{objects, relations, candidates}`, **written
  only where it is not empty**, as `later_debts` is, so every other program's summary is byte for byte what it
  was. `ein oracle --dump`: the same key in `region.json`, written the same way, and `**Relation names as
  objects**` in `region.md`.

**It prints on one corpus program**, `examples/features/01_not_and_absent.ein`, 16 of its 35 candidates, as the
entry said. Its seven lines of `corpus_shapes.md5` are re-banked in `05bebca9`, per commit and under
`EIN_CORPUS_SLOW=1`: `read-out[describe]`, `read-out[maximal]`, `read-out[minimal]`, `read-out[single]`,
`read-out[n2]`, `oracle` and `region-trace`; `git diff -U0` on the file names no other program.
`read_out_examples.md`, written by `utils/read_out_examples.py`, gains the row in X15, that program's own
example, and three sentences of caption; X1 to X14 and X16 to X20 are unchanged, so no example's read-out moved
by the E-1 fix either. `read_out.md`'s `universe` table, `answer_dump.md`'s three keys, `region_dump.md` and
`solution_semantics.md` § 1.4 state it; `read_out_reference.rs`, `answer_dump.rs`, whose key census reads
`features/01` for the key no other program writes, and `region_dump.rs` hold the pages.
`ein-cli/tests/relation_names_row.rs` holds the row on the eight questions (describe, `--maximal`, `--minimal`,
`--single`, `-n 1`, `--count`, `--monotone`, `--no-search`), the summary's key, `ein oracle` and its dump, on a
program written for it and on its copy that declares the relation, which prints no row and writes no key.

### E-54: left

Nothing built: the universe is the program's names wherever they were written. **What the read-out says of it
today is nothing.** The `universe` row counts H's candidates and `-H` lists them by relation, a module's
relation under the name the main file spells, else its full name (`lib.shape`); no row and no key says that a
name or a relation is one only an import brings. The witness prints *universe 4 candidates, 0 dead alone*
under both import forms. The two corpus programs whose R is over a module's names,
`import-directives-as.ein` and `-twice.ein`, keep their four states, which their `fixed` rows hold as right.

### What stays open

- **[H9-1](findings.md#h9-1--a-reserved-word-a-program-writes-as-a-value-is-a-name-of-the-program-and-never-an-object)
  is not in this ruling and is not touched.** A reserved word a program writes as a value, `eq` in `(order
  eq)`, is a name of the program and never an object, and read against *every name is in the candidate list* it
  is a wrong answer with no row. It is the reverse of what was ruled here: the ruling takes a kernel name out of
  the objects, H9-1 asks whether one should be in. `solution_semantics.md` § 1.2 now states the exclusion as
  what the engine does, beside the dunders', and makes no claim that it is ruled.
- **[H9-4](findings.md#h9-4--a-__symmetric__-r-a-rule-derives-is-not-mirrored-by-the-saturation-that-derives-it)**,
  found by this task: a `(__symmetric__ R)` a rule derives is not mirrored by the saturation that derives it.
  Not fixed, no row.
- **[H9-5](findings.md#h9-5--the-shape-goldens-hypclosed-op-digests-a-pass-no-command-calls)**, found by this
  task: the `hyp+closed` op.
- [H9-2](findings.md#h9-2--a-written-dunder-fact-registers-the-dunder-as-an-auto-vivified-relation-of-the-program)
  stands as it was: a written dunder fact still registers the dunder as a relation of the program. The fix is
  in the enumerator, so that difference is in place for the registry's other readers; the enumerator no longer
  reads it for a dunder.

### Tests and verification

Run in the worktree, on AC, with two other agents' suites running on the machine. Each was run on the tree
whose `ein.rs/`, `corpus/`, `examples/`, `docs/`, `stdlib/`, `tests/` and `utils/` are `05bebca9`'s, after the
last formatting edit; this commit changes files under `plans/` and one bullet of `AGENTS.md`, after which the
link audit and the four tests that read `AGENTS.md` or the gate's lists (`embedding`, `cli_semantics`,
`gate_steps`, `nightly_steps`) were run again, green.

| command | result |
|---|---|
| `cargo test --manifest-path ein.rs/Cargo.toml --profile gate --no-fail-fast -p ein-infer -p ein-corpus` | green; `dunder_objects`, `register_region`, `alive_invariant`, `claim_round_trip`, `detector`, `explore`, `hypgen_coverage` among them |
| `cargo test … --profile gate --no-fail-fast -p ein-cli`, the whole crate | green; `wrong_answers`, `import_invariance`, `lever_invariance`, `read_out_reference`, `corpus_cli`, `config_reference`, `answer_dump`, `region_dump`, `relation_names_row` among them |
| `cargo test … --profile gate --no-fail-fast -p ein-render`, the whole crate | green; `id_order_invariance` unblessed, `dot_wellformed`, `jobs_invariance` among them |
| `EIN_CORPUS_SLOW=1 cargo test … --profile gate -p ein-render --test id_order_invariance`, unblessed | green |
| `cargo fmt --all --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | clean |
| `python3 utils/check_hashmap_iteration.py` | *determinism lint ok — 316 file(s), 62 annotated* |
| `python3 utils/stdlib_manifest.py` | *stdlib ok — 24 modules match MANIFEST.sha256* |
| `python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md examples/README.md stdlib/README.md utils/README.md c/README.md` | *── links ── 574 pages, 0 finding(s)*, run with this record in place |

The same suites were also green on each earlier commit's tree but one: on `200df470`, the rows open,
`ein-infer`, `ein-corpus`, the six `ein-cli` tests of the brief and the shape golden both ways; on `4ee131e3`,
the same set.

**`corpus_cli` under `EIN_BLESS=1` fails one test of seven on this machine today, and it is the clock.**
`EIN_BLESS=1` implies the slow entries, and `the_slow_flag_still_describes_the_sweep` then holds each to four
times its recorded cost: `examples/beyond_arith/send_more_money.ein`, recorded 8 404 ms, swept in 49 303 ms, and
`examples/saturation/square-bwd/houses.ein`, 1 584 against 7 204, the test's own message asking *machine load?*.
The load average was 28 with the other agents' suites running. Neither program's answer moved, and alone the
gate build of `3e59b90a` and this one take 24.7 s and 28.8 s on the first and 2.8 s and 2.7 s on the second, so
the change is not what slowed them. The same test binary unblessed, which does not read the slow entries, passed
on every tree, and `corpus_exits.txt` did not move under the bless. **So the slow entries' cost is not re-held by this task**: a quiet
machine's `EIN_CORPUS_SLOW=1 cargo test … --test corpus_cli` is what holds it.

### Defaults taken

1. **The first row's run is the maximal line**, one state, and its counts are in three other blocks: a `now`
   lists its states, and no row of the register lists two thousand.
2. **The `__symmetric__` witness writes `near` both ways round**, so that it does not rest on H9-4. Its H and
   R are the entry's.
3. **The dunders are left out in `candidate_objects`, not by a change to the names index or the registry**: the
   smallest change that makes the ruling true of the blind enumerator, and the one that leaves every other
   reader of those two as it was.
4. **E-7's count is of the candidates that name a relation a rule concludes**, by two clauses: a head root's
   fixpoint holds, and a relation a rule's `:assert` names as written. The second names a relation no rule has
   fired for at root where the program writes its name as a value. Not counted: a relation a rule only *reads*
   (`:match`), which nothing concludes, and a relation a rule writes through a parameter that root holds no fact
   of. `object_census`'s `R1 ∪ R2` also counts a relation only a `:match` names; on the corpus the two agree,
   16 on one program.
5. **The row is a row of its own, `relation names`, after `types`**, and not a number inside the `universe`
   row: the ruling's precedent names the objects and their relations, and a program with none prints the
   `universe` row it printed.
6. **The JSON keys are written only where not empty**, in the summary and in `region.json`: an always-present
   key would have moved every program's summary.
7. **`ein test`'s report does not carry it**, as it does not carry `later_debts`: the ruling names the
   `universe` row and `root.hypgen`.
8. **The example is X15**, `features/01`'s own, and no new example: the row has one corpus program.
9. **`alive_invariant.rs`'s list of the kernel's words gains the two dunders**, though no corpus program needs
   it, since the list says which names of a program are never objects and that is now true of them.
10. **`closed.rs`'s two corpus totals are removed, not re-taken**: no command was behind them.

### Not done

- `./run_tests.sh`, the nightly and `utils/ci_container.sh`: by instruction.
- H9-1, H9-4 and H9-5 are filed and not fixed; none has a register row.
- The `hyp+closed` op and `emit_closed` are not changed or deleted.
- `ein test`'s JSON report and the lines it prints beside a claim do not carry the new naming.
- The untracked artefacts of tasks 1 to 3 under the main tree's `build/s1f.10.9/` were read and not rewritten:
  their read-outs are of the engine before the fix.

---

## Record (2026-11-16): H9-1 and H9-4, as ruled at the close

**The ruling.** The user at P1f.10's close, 2026-11-16, shown H9-1 and H9-4 with *reads as a wrong answer: file
it as an `open` register row* proposed for each: *"File and fix them in this phase"*
([close.md § The user's answers](close.md#the-users-answers-at-the-close-2026-11-16)). The work as set: decide
from what the language already rules what each right answer is, file each wrong answer `open` with its witness,
fix it and flip it; and stop where the pages do not decide a right answer, or where an existing program's
answer would move.

**What came of it.** H9-4 is decided by the pages, filed as the register's **W98**, two rows, and fixed; no
other corpus program's answer moved. **H9-1 is stopped for the user**: two rulings on the pages decide it
opposite ways, and the reading the finding calls a wrong answer restates a `fixed` register row's right answer.
No row is filed for it and nothing of it is changed.

**The commits**, on the branch `worktree-agent-a6306e4b8175b1a3c`, from `cb0fbde8`:

| commit | what |
|---|---|
| `30d12edc` | W98 filed `open`, two rows, their witnesses, controls and corpus entries; the five corpus sweeps the witnesses break name them in a list `W98_OPEN` each |
| `72121213` | the fix in `saturator.rs`, the mark read where it comes to hold and recorded as each mirror's activator; the rows flipped; the five lists deleted; two tests; the three pages; 31 golden lines of the two witnesses |
| `5a7d200d` | W98's `[case.fixed]` names `72121213` |
| this one | this record, the stage's status, H9-1 and H9-4 in [`findings.md`](findings.md#from-s1f109) |

### H9-4: the right answer, and why the pages decide it

A `(__symmetric__ R)` a rule derives marks R as the same fact written does, so each witness's right answer is
its control's. Three things on the pages say so, and none says otherwise:

- [`reserved-engine-strings.md` § `__symmetric__`](../../../docs/ein/ein.rs/reserved-engine-strings.md#__symmetric__--engine-effect):
  the native mirror is *a performance optimization, NOT a capability*, computing *the identical closure as the
  stdlib `symmetric` rule*. That rule is activated by `(symmetric R)` however the fact came to hold, a rule's
  conclusion among them, so the identical closure reads a derived mark too.
- [`solution-semantics.md` § 1.3](../../../docs/ein/reasoning/kb-evolution/solution-semantics.md#13-root): root is the
  program's facts saturated to quiescence. A root whose first saturation leaves a mirror underived is not.
- A program may conclude a dunder's fact (`reserved_engine_strings.md`'s table: *use it … as the relation of …
  a conclusion*), and the user's ruling on Q-M1f.306 held one closure stated two ways to one answer.

### Why the finding's witness broke the sweeps

`Saturator::next_mirror_firing` seeded the mirror once, at the saturation's first step, from the relations
marked then, and on `symmetric-derived-root.ein` the rule derives the mark after it. `symmetric_rels` re-read
the marked relations when the count of marks changed, so `near` facts stored *after* the mark were offered to
the mirror; `(near a b)`, stored before, never was. Root's first saturation was no fixpoint, and every reader
that saturates the same root again wrote `(near b a)` into it. H9-4 named two sweeps; with the witness in the
corpus five failed, each for that reason:

- **`detector.rs`'s fork-against-scratch sweep.** A node's fork resumes root's saturation, whose mirror never
  saw `(near a b)`; scratch saturates root's fixpoint and the commitment with a fresh saturator, whose seed reads
  the mark. On the node witness, `symmetric-derived-node.ein`, root holds no mark, and the departure is a
  split's: a split from `{(close b)}`'s fixpoint, which holds the mark, mirrors `near` where the fork did not
  (*split {(close a), (close b)} from {(close b)}*).
- **`claim_round_trip.rs`'s panic.** `claim::print` and `claim::check` describe R through `claim.rs`'s
  `lattice`, whose route prepares root and seals it, and then read each state through `fixpoints`, which
  prepares the same root again and fans out from it without sealing. That holds wherever a second preparation
  writes nothing. Here it wrote `(near b a)` into root's open top layer, and `Kb::branch`'s assertion, *Kb::branch
  on an unsealed KB*, stopped the run. **Why the binary did not**: `ein solve` and `ein test` check a claim with
  `check_described`, against the description the run already computed, on a root its describe had prepared
  twice and sealed, so the third preparation wrote nothing. `claim.rs` is not changed: its premise, that a
  saturated root is a fixpoint, is the one H9-4 broke, and the assertion that stopped the run is what fails when
  the premise stops holding.
- **`read_out.rs`**: the read-out listed ∅ with `(near b a)`, the oracle's ∅ being the first saturation's,
  without it. **`summary_properties.rs`**: the root block of a run that saturates root again differed from
  `--monotone`'s. **`answer_dump.rs`**: a state file states the mark, so the program it is saturates past the
  state it records: `(near b a)` at root, `(false)` under a commitment.

The open commit left the witnesses out of the five sweeps by name, each list's doc saying why, and the fixing
commit deleted the five lists; each sweep holds both witnesses since.

### The register: W98

Two rows of one case, filed `open` in `30d12edc` with today's answer and the right one as data, flipped in
`72121213`. `[case.wrong]` is `ein solve` on the gate build of `cb0fbde8`; `ein oracle` agreed on each.

| row | witness, under `examples/ein-bugs/wrong-answers/` | before | after, the control's |
|---|---|---|---|
| `W98/derived-at-root` | `symmetric-derived-root.ein`: `(near a b)` and a rule with an empty `:match` that concludes `(__symmetric__ near)`, the finding's own | H `(near b a)`; R two states, ∅ and `{(near b a)}`, each listed holding both facts | no candidate, R {∅} |
| its `[case.control]` | `symmetric-written-root.ein`: the mark written | no candidate, R {∅} | the same |
| `W98/derived-at-node` | `symmetric-derived-node.ein`: `close-symmetric` marks `near` once anything is close, `one-way` calls `(near b a)` a contradiction, `near` scoped out | H `(close a)`, `(close b)`; R the cube, four states | the same H, each dead alone; R {∅} |
| its `[case.control]` | `symmetric-rule-node.ein`: `near` closed by the program's own rule `close-mirror` | R {∅} | the same |

`ein saturate --dump` on the first witness printed `(near a b)` and `(__symmetric__ near :rule near-symmetric)`
and no `(near b a)`; it prints `(near b a :rule __symmetric__)` after them since, as the control does.

### The fix

[`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs), the mirror's functions:

- `symmetric_rels`, where it reads the marked relations again, offers the whole extent of every relation marked
  since its last read to the mirror's queue, in name order and each extent in the store's order, as the seed
  offers a marked relation's. It does so once the mirror is seeded; before, the seed reads every marked
  relation's extent itself.
- `next_mirror_firing` reads the marks before it marks the seed taken, so that the read offers nothing the seed
  is about to offer.
- **Each mirror's derivation records the mark as its activator** (`Prov::activator`), as a rule's firing records
  the fact that activated its plan. A core follows an activator only where a node derived or committed it
  (`ein_core::walks::node_activator`, Q-M1e2.12 (c)), so a mark root holds, written or derived, moves no core.
  The first form of the fix left it out, and `corpus_cores.rs`'s
  `every_core_names_its_commitment_and_derives_its_contradiction` failed on the node witness: its three nodes,
  dead since the fix, reported the core `(near a b)`, which names none of their commitments. With it each names
  its `close` fact. The activator was folded into the fix commit before the branch was handed back, so no commit
  of the branch fails that test; the corpus diff and the goldens below are the fix commit's.

So the saturation in which the mark comes to hold, root's or a node's resumed one, closes R: a mark derived at
root, one a commitment brings a rule to derive in a node, and one a fork's delta holds. `has_pending_mirror`,
which `is_stalled` asks, read the marked relations' extents directly and said *not stalled* on the root witness
where `step` reported the fixpoint; the two agree since.

**The corpus diff.** `ein solve FILE -H --json-summary OUT` on every one of the 888 corpus programs of
`72121213`, the gate build of `cb0fbde8` against the gate build of `72121213`, stdout, stderr's first line, the
exit code and the summary compared: **the two W98 witnesses differ, by their rows, and nothing else does**; their
controls do not (`build/h9/sweep.py`, untracked, in the worktree; 2026-11-16). No corpus program derived a
`(__symmetric__ R)` by a rule over a relation not already closed under it, as H9-4 said.

**The goldens.** `ein-render/tests/golden/corpus_shapes.md5`, re-banked per commit in `72121213`: 31 lines,
all the two witnesses'. `symmetric-derived-root.ein`, 19: `saturate`, `hyp`, `explain`, `explain-noalts`,
`dot[kb-since]`, `read-out[…]` and `cores[…]` for describe, maximal, minimal, single and n2, `region-trace`,
`states`, `region-lattice`, `oracle`. `symmetric-derived-node.ein`, 12: the same but `saturate`, `hyp`, the two
`explain`s, `dot[kb-since]` and the minimal line's two, whose ∅ did not move. The controls' lines did not move,
and `corpus_exits.txt` did not move. The check:

```sh
git diff -U0 30d12edc 72121213 -- ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5 \
  | grep '^[-+][0-9a-f]' | grep -v 'symmetric-derived-' | wc -l      # 0
```

**Tests**, in `ein-infer/tests/stdlib_semantics.rs`, beside the test that holds the native mirror to the stdlib
rule's closure: `the_native_mirror_reads_a_mark_a_rule_derives`, a mark derived by a rule with an empty
`:match` and by one that reads a fact of the relation against the written mark, the closure equal and a second
saturation of each fixpoint writing nothing; and `a_derived_mark_answers_what_its_control_answers`, each W98
witness against its control by the cube, one H and one R. Both fail with the saturator as it was (run on the
fix's tree with `saturator.rs` restored to `30d12edc`'s, then put back).

**Pages**: `reserved_engine_strings.md`'s `__symmetric__` section, *the mark is read in the saturation it comes
to hold in*, and is each mirror's activator; `06_reserved_names.md`'s `__symmetric__` row; `solution_semantics.md` § 1.2's dunder bullet, one
root beside one H and one R for a symmetry. `AGENTS.md` states nothing of the mirror and is unchanged, its
pointer to the register's owners restored.

### H9-1: stopped, three readings

**The words.** `candidate_objects` leaves out seven kernel words by name, `not false and or absent eq neq`.
Four are never a name (`not`, `and`, `or`, `neq`: refused at the parse), so the ones a program can write as a
value are `eq`, `absent` and `false`. [`diagnostics.md` § 2.22](../../../docs/ein/user/reference/diagnostics.md#222-a-value-is-a-name-or-a-fact-a-state-could-hold),
the user's value rule, keeps all three names: *`(h absent)`, `(h false)` and `(mk eq)` are facts*. So **no
reading refuses them at load**: a refusal would undo § 4.22 and refuse `std.math.digits`, whose `cl` rows write
`eq`. `relation` and `written`, the finding's other two, are relations of every program (`Program::categorise`),
and a relation's name written as a value is no object by the rule § 1.2's last bullet states and Q-M1f.306 (a)
left; no reading below moves them.

**Why the pages do not decide it.**

- Q-M1e2.9 (a),
  ruled 2026-11-04, *meant literally*: every constant in an argument position is a name of the program, and
  [`solution-semantics.md` § 1.2](../../../docs/ein/reasoning/kb-evolution/solution-semantics.md#12-names) says every name
  is in the candidate list, a list that left one out an engine bug (W4, W15). Read so, `eq` in `(order eq)` is
  an object, and H9-1 is a wrong answer. The close's line was this reading.
- Q-M1e2.26, ruled 2026-11-07:
  `eq` is *the relation of equality*, in every place a relation can be written. Read with § 1.2's relation
  rule, a value `eq` is a relation's name and no object, as `colour` in `(order colour)` is none where `colour`
  is declared. **The register holds this reading as right**: `W52/relvar-eq-premise`, `fixed`, binds a relation
  parameter by `(r eq)`, and its `right`, H `(hit a b)` and `(hit b a)` and R four states, has no candidate over
  `eq`.
- § 1.2's own bullet, *the kernel's words … whether or not a program writes one as a value*, was written by
  this stage's task 4 as what the engine does, with no ruling behind it; and the user's dunder ruling, *"dunder
  never an object, wide reading"*, is by name, for the two dunders, on the reason that no program means a dunder
  as an object, which does not hold of `eq`: `std.math.digits` means it as an order, `lt`, `eq`, `gt`.

**The readings, each with what it moves.** The corpus column is the sweep of the 884 corpus programs used for
H9-4, `ein solve FILE -H --json-summary`, the gate build of `cb0fbde8` against a build with the reading, for (c)
both builds with H9-4's fix, which moves no corpus program (`build/h9/sweep_h91.txt` and `sweep_h91r3.txt`,
untracked; 2026-11-16); the witnesses by `ein solve -H`.

| | reading | the finding's witness: `(order lt) (order eq) (order gt)`, `order` scoped out, beside an open `pick` | `(word yes) (word absent) (word false)` the same way | the corpus |
|---|---|---|---|---|
| **(a)** | a kernel word a program writes in an argument position is an object: `eq`, `absent`, `false` | H 2 → 3, R 4 → 8 states, `(pick eq)` in H | H 1 → 3, R 2 → 8 | **two `fixed` rows move**: `W52/relvar-eq-premise`, H 2 → 7 and R 4 → 128 states, its `right` restated; `W52/relvar-eq-assert`, H over `eq` too, R still empty. And 244 programs that bring `std.math.digits` (`examples/arith/`, `beyond_arith/`, `binary_arithmetic/`, `word_problems/`, `stdlib/math/`, `tests/stdlib/math/`): `root.hypgen.pre_candidate.closed_relation` rises and the shape golden's `hyp` line moves, no H, R, read-out or exit code |
| **(b)** | never an object, however written (the engine today) | none | none | none: § 1.2's bullet becomes a ruling, and H9-1 closes as no wrong answer |
| **(c)** | `eq` is a relation (Q-M1e2.26), so its name as a value is no object, as a declared relation's is not; `absent` and `false`, neither a relation whose facts a state could hold (`absent` heads no fact, and `(false)` says that a state is impossible), are objects where a program writes them as values | none | H 1 → 3, R 2 → 8 | **none** |

The finding names `eq`, and so does the close's line; the user's *"File and fix them"* was given on that line,
which did not show that (a) restates W52's right answer. Which reading is right is the user's, and no row is
filed: under (a) the rows would be the finding's witness and W52's two restated; under (c) two new rows, the
`absent` and `false` witnesses; under (b) none. Each reading's code is in `candidate_objects` alone: (a) keeps a
kernel word that `invariant::program_names` lists (`build/h9/h91_reading1.patch`, untracked), (c) the same with
`eq` aside. Nothing of it is committed. The witnesses are in the worktree's untracked `build/h9/`
(`eq_as_value.ein`, `eq_as_value_written.ein`, `words_as_values.ein`), and the finding has the first two verbatim.

### Verification

Run in the worktree on the tree of `5a7d200d`, whose `ein.rs/`, `corpus/`, `examples/`, `docs/` and `stdlib/` are
this commit's, which changes files under `plans/` only; the link audit was run again with this record in place.
On AC, with other agents' suites on the machine.

| command | result |
|---|---|
| `cargo test --manifest-path ein.rs/Cargo.toml --profile gate --no-fail-fast -p ein-infer -p ein-render -p ein-corpus -p ein-core -p ein-ir` | green, 928 tests; `stdlib_semantics`, `detector`, `claim_round_trip`, `read_out`, `corpus_cores`, `register_region`, `explore`, `id_order_invariance`, `dot_wellformed`, `jobs_invariance` among them |
| `cargo test … --profile gate --no-fail-fast -p ein-cli -- --skip no_config_flag_moves_a_line_of_any_mode` | green, 407 tests; `wrong_answers`, `answer_dump`, `summary_properties`, `corpus_cli`, `import_invariance`, `lever_invariance`'s other tests among them. The one skipped is by instruction: slow on one cell today, another agent's |
| `EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance`, unblessed | green |
| `EIN_CORPUS_SLOW=1 cargo test … -p ein-infer --test explore`; `… --test detector` | green |
| `EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test import_invariance` | green |
| `cargo fmt --all --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | clean |
| `python3 utils/check_hashmap_iteration.py` | *determinism lint ok — 337 file(s), 70 annotated* |
| `python3 utils/stdlib_manifest.py` | *stdlib ok — 24 modules match MANIFEST.sha256* |
| `python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md corpus/README.md tests/README.md examples/README.md stdlib/README.md utils/README.md c/README.md` | *── links ── 582 pages, 0 finding(s)*, with this record in place |

On `30d12edc`, the rows open: `ein-infer`, `ein-render` and `ein-cli` whole (the same skip) and `ein-corpus`, each
test that failed on a witness run again once its list was in place, green; fmt, and clippy over `ein-infer`'s and
`ein-cli`'s targets, clean.

### Not done

- H9-1: no row and no change; stopped for the user with the three readings above.
- `./run_tests.sh`, the nightly and `utils/ci_container.sh`: by instruction.
- `state.md` and `close.md`: by instruction.
- The shape golden's `hyp` and `hyp+closed` lines of the four W98 programs are banked as every program's are;
  the removal of both ops, which runs beside this, takes them out with the rest.
