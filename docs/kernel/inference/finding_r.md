# How R is found — the oracle and the routes

The answer to a program is a region `R` of commitments
([`solution_semantics.md`](solution_semantics.md) § 2). Two commands compute it:

- **`ein oracle`** evaluates the definition: it computes every node of the cube.
- **`ein solve`** computes the same `R` in each of its modes, and on a monotone program skips
  every node that a stated licence decides without computing it.

The oracle is the reference: `ein-infer/tests/explore.rs` holds every mode of `ein solve` to it on
every corpus program whose cube is within the oracle's limit.

This page writes both down as formulas, and answers three questions:

- what happens to a candidate that is refuted;
- how a run proves that `R` is one state without visiting the lattice;
- why nothing a node derives is written back into root.

The rows `ein solve` prints about this, `route` and `work`, are
[`read_out.md`](read_out.md)'s.

## 1. A node

**Root** is the program's own knowledge base saturated once: `root = sat(KB₀)`. It is sealed. Every
node is a copy-on-write fork of it, read and thrown away, so nothing a node derives ever reaches root
or another node.

**H**, the universe of candidates, is built once at root by the hypothesis ladder
([`solution_semantics.md`](solution_semantics.md) § 1.4). It is listed by `ein show hyps FILE` and
its structure is described in [`hypothesis_space.md`](hypothesis_space.md).

For a **commitment** `C ⊆ H`:

```text
st(C)     = sat(fork(root) + C)                 the node's state, saturated to quiescence
dead(C)   ⇔ st(C) holds a contradiction          contradiction::detect
owes(C)   = the obligation instances st(C) holds with no witness
cl(C)     = st(C) ∩ H                            its closure, C ⊆ cl(C)

R         = { C ⊆ H : ¬dead(C) ∧ owes(C) = ∅ }   alive and discharged
```

Every route reads a node through one function, `ein_infer::region`'s node function, through the
node cache of `ein_infer::explore` (`Space::eval`), so two routes cannot read one node differently.
A node's **core**, the facts its contradiction rests on (`explain::contradiction_frontier`), is
taken only where a surface prints it.

**What a route computes of a dead node.** `st(C)` above is the definition, and the oracle computes
it whole. On a monotone program a route's node, root's excepted, is saturated **to quiescence, or
to its first contradiction where it is dead**:

```text
st_r(C)   = st(C)                                if st(C) holds no contradiction
          = the fork as it stands after the firing that makes its first contradiction,
            every conclusion of that firing written, or with C written where that
            already makes one                                               otherwise
dead_r(C) ⇔ st_r(C) holds a contradiction  ⇔  dead(C)          the KB only grows
```

So an alive node is computed whole, and a dead one costs only the derivation of its contradiction.
No route reads more of a dead node than `dead(C)`: what it remembers is the set (§ 5), and a dead
node's closure, denials and debts decide nothing. `R`, `N` and every count are therefore the
oracle's (`ein-infer/tests/first_contradiction.rs` holds every question alike with and without the
stop, on every corpus program that searches). What a dead node **reports** is `st_r(C)`: the
`work` row's facts, and its debts and closure on the surfaces that list visited nodes. **Its core**,
where a surface prints one, is taken over `st(C)`, as the oracle's is: the saturation goes on from
the stop, firing what it would have fired, so no core depends on the stop. Not stopped: root's
node, a flagged program's nodes (its detector reads every node's fixpoint), the oracle's, the
structure's readers (`ein render h-space`; its theory check reads a dead node's facts), and the
whole node the tree's order reads, whose reader needs every application.

## 2. The oracle: the cube

```text
for every C ⊆ H:                  2ⁿ nodes, n = |H|
    compute st(C)
    read dead(C), owes(C), cl(C), and its core if dead
R = { C : ¬dead(C) ∧ owes(C) = ∅ }
```

It skips nothing (`L7 the cube`) and assumes nothing about the program, so it is right for every
program, monotone or not. It also reads off what a description of `R` needs:

- `N`, the minimal dead sets;
- the two lines, minimal and maximal;
- the shape, the holes, and the counts `k_s` (states) and `k_c` (commitments).

Its cost is `2ⁿ` saturations, so it refuses a universe past `--region-limit` (default 16):
`ein oracle examples/zebra2.ein` refuses its 56 candidates, *"the region is 2^56 saturations and was
not computed"*. The function is `region::compute`.

## 3. `ein solve` on a program the static check flags

A program is **flagged** when the static check cannot prove it monotone
([`monotonicity.md`](monotonicity.md)). On such a program a dead set can have an alive superset, and
an alive superset of a member of `R` can owe. No skip is sound, and every mode takes the cube, as the
oracle does (route 2). It takes the cube in layer order where its question can be decided before the
cube ends.

The one licence a flagged program keeps is **L0**: a fork only adds, so when root is dead every node
is. So `ein solve` is right on every program; what the licences below buy is speed, on monotone ones.

## 4. A monotone program: three properties and the licences

When no read of a compiled plan can take a conclusion away or add a debt as a commitment grows
(`ein_infer::monotone`), three properties hold:

```text
dead is upward-closed           dead(D) ∧ D ⊆ C            ⇒ dead(C)
discharged is upward-closed     owes(D) = ∅ ∧ D ⊆ C        ⇒ owes(C) = ∅
cl is a closure operator        st(C) = st(cl(C))
```

From these, each route skips what one of the licences decides. The `route` row names the licences a
run used.

| licence | statement | what it saves |
|---|---|---|
| **L0** | root dead ⇒ every node dead | the whole search (every program) |
| **L1** dead is upward-closed | `dead(D) ∧ D ⊆ C ⇒ dead(C)` | every superset of a set known dead |
| **L2** discharged is upward-closed | `C ∈ R ∧ C ⊆ C′ ∧ ¬dead(C′) ⇒ C′ ∈ R` | the owing check above a member of `R` |
| **L3** a commitment's state is its closure's | `st(C) = st(cl(C))` | the routes visit closed **states**, not commitments, and count `k_c` without visiting it |
| **L4** a node of R is maximal when every one-up extension is dead | `s ∈ R ∧ ∀b ∉ s: dead(s ∪ {b}) ⇒ no member of R ⊋ s` | the climb, and the last step of `--single` (§ 6) |
| **L5** an owed instance's alternatives are exhaustive | at an owing state `C` owing a licensed instance `I` with alternatives `A(I)`: `R ∩ ↑C = ⋃_{a ∈ A(I)} R ∩ ↑cl(C ∪ {a})` | branching on one instance's alternatives instead of on every extension |
| **L8** what a state denies is dead above it | `(not b) ∈ st(S) ⇒ dead(S ∪ {b})` | a saturation for each candidate a state's own rules have already denied |
| **L9** a fact nothing proposes and nothing writes is in no state | a witness `w` of `I` with `w ∉ H`, `w ∉ st(∅)`, of a relation no plan writes and the kernel does not mirror, is in no `st(C)`; so where every other witness of `I` is a candidate or denied at root, L5's equation holds with `A(I) = H ∩ witnesses(I)` | branching on every extension at a state owing an instance L5 alone does not license |

**Each licence's premise is checked, not assumed.**

- **L5**'s premise, that every read of the instance's guard beside its witness is static and every
  witness is a candidate or denied at root, is checked at load (`oblgen::alternatives`). It is checked
  again on every node a route computes, and a node that breaks it refuses the run. The argument is
  written in `tree.rs`'s header.
- **L8**'s argument is in `known_dead.rs`: `st(S ∪ {b}) ⊇ st(S)` holds `(not b)`, and its
  commitment holds `b`.
- **L9**'s argument is in `unheld.rs`. The relations no plan writes are read off root's plans, and
  every node a route computes with the table, the whole node the tree's order reads included, refuses
  the run if it holds a fact of one of them. Only the tree takes L9; the structure's readers keep
  L5's table.
- A route that meets a state contradicting what it knows (an alive set it holds dead, a member of `R`
  with an owing extension) refuses the run. A refusal is an engine defect, never an answer.

## 5. What a run knows dead, and what happens to a refuted candidate

Every route on a monotone program asks one memory before it saturates anything: `known_dead`.

```text
known dead:   D            for every set D a saturation found dead
              S ∪ {b}      for every state S met whose own fixpoint stores (not b)     (L8)

before computing C:   if some known-dead set D ⊆ C   ⇒ C is dead, not saturated     (L1)
```

**A refuted candidate.** Suppose `{h1}` is computed and found dead.

- **Nothing is written into root.** The fork that found the contradiction is thrown away, and root
  holds no `(not h1)`.
- **`{h1}` goes into the run's memory.** Every commitment that holds `h1` contains `{h1}`, so it is
  dead by L1 and never saturated, and no state is ever extended by `h1`. For the rest of the run the
  search is over `H ∖ {h1}`.
- **The read-out names it** among the candidates `dead alone`.
- **In general, a dead set `D` removes all of `↑D`**, the sets above it, from the search.

**Why a refutation is a constraint on commitments and not a fact in root:**

1. **It would change the answer.** `R` is defined over `st(C) = sat(root + C)`. A `(not h1)` in root
   would let every rule that reads the negation fire in every other node, so states, and with them
   `R`, could differ from the definition.
2. **It is only sound on a monotone program.** On a flagged program `{h1}` dead says nothing about the
   sets above it.
3. **Every route and the oracle must read the same node.** They can only if every node is a fork of
   one root that nothing changes.

**What the memory keeps of a death** is the set that was computed and found dead, not the smaller
set its contradiction rests on. A denial is the other source: a state's own rules store `(not b)`
(domain elimination, for instance), and L8 reads it with no saturation. It is a refutation of `b`
*relative to that state*.

## 6. The routes, question by question

Each question does the least work it needs ([`read_out.md`](read_out.md) §§ 2–3 has what each one prints).

**Describe** (`ein solve FILE`), and `--count` by the same route: **the state route**
(`states.rs`).

```text
visit the alive states by size, from cl(∅):
    for each state S, for each b ∈ H ∖ S:
        if S ∪ {b} is known dead          skip                                 (L1, L8)
        elif S ∪ {b} is a known state's   read it, no saturation               (L3)
        else                              compute st(S ∪ {b})
k_c(T) = 2^|T| − Σ_{S ⊊ T, S an alive state} k_c(S)                            (L3)
```

`N`, the merged states and the two lines are read off the states it met.

**`--single`, `-n N`, `--minimal`, `--maximal`: the obligation tree** (`tree.rs`).

```text
tree(C):   if C is known dead or dead(C)              → nothing below C
           elif owes(C) = ∅                            → a leaf: C is a state of R
           elif C owes a licensed instance I           → children cl(C ∪ {a}), a ∈ A(I)   (L5, L9)
           else                                        → children cl(C ∪ {b}), b ∉ C
```

By L5, every member of `R` lies above some leaf of the tree once the tree is exhausted.

- **Which instance a state branches on** decides the work and never the answer. The order is
  `order.rs`'s, and the `route` row says in words which order a run took.
- **`-n N`** lists the tree's leaves, then the states of `R` above them (L2).
- **`--minimal`** shrinks each leaf to a minimal commitment that stays discharged, and gets the rest
  of the line by duality: the minimal commitments of `R` are the alive minimal transversals of the
  complements of the maximal owing states.
- **`--maximal`** climbs a leaf to a maximal state (L4), then finds the others by *dualise and
  advance*.

## 7. Proving that R is one state without the lattice

This is `--single`'s question, *is R one state?*

```text
if ∅ ∈ R:   s = cl(∅)                                       root's own state
else:       run the tree; stop at the second distinct leaf  → not one state
            if no leaf                                      → R is empty
            if exactly one leaf state s                     → R ⊆ ↑s          (L5)

then:       for each b ∈ H ∖ s:
                if s ∪ {b} is alive                         → it is in R (L2), another state:
                                                              not one state
            all dead                                        → every proper superset of s is
                                                              dead (L1), so R = { C : cl(C) = s }:
                                                              one state, proved                (L4)
```

**What the proof costs** is the tree's nodes and `s`'s one-up extensions, less every set the memory
of § 5 decides without a saturation. That is the size of the search tree, not of the cube, but it
can still be large. Each figure below is the `work` row of the command beside it:

| program | candidates | `ein solve FILE --single` | the cube |
|---|---:|---|---:|
| `examples/zebra2.ein` | 56 | *Solution — R is one state*, 7 saturations | 2⁵⁶ |
| `examples/zebra.ein` | 56 | *Solution — R is one state*, 9 saturations | 2⁵⁶ |
| `examples/beyond_arith/send_more_money.ein` | 80 | *Solution — R is one state*, 18 031 saturations | 2⁸⁰ |

On a flagged program none of this holds. A dead set can have an alive superset, so the only proof
is the cube.

## 8. What holds it

| claim | test |
|---|---|
| every mode of `ein solve` gives the oracle's `R` on every monotone and flagged cube within the limit | `ein-infer/tests/explore.rs` |
| each claim a program states is decided alike by the question `ein test` asks, by describe and by the oracle | `ein-infer/tests/claim_routes.rs` |
| no lever, option or flag changes `R` | `ein-cli/tests/lever_invariance.rs` |
| a denial kills exactly where a computed node would, on every monotone cube; the run with and without the memory's denials gives one answer | `ein-infer/tests/known_dead.rs` |
| a route's dead node stopped at its first contradiction gives every question the answer the node run to quiescence gives | `ein-infer/tests/first_contradiction.rs` |
| every order the tree can take gives the oracle's answer | `ein-infer/tests/explore.rs` (`every_order_of_the_tree_is_the_oracle`), `ein-infer/tests/order.rs` |

## Cross-references

- [`solution_semantics.md`](solution_semantics.md): the definitions of commitment, state, closure,
  monotone and `R`.
- [`monotonicity.md`](monotonicity.md): what makes a program flagged, and how each kind is solved.
- [`read_out.md`](read_out.md): the `route` and `work` rows.
- [`hypothesis_space.md`](hypothesis_space.md): `H` and its structure, which the tree's order reads.
- `ein.rs/crates/ein-infer/src/tree.rs`, `states.rs`, `known_dead.rs`, `order.rs`: the routes and their arguments,
  each in its module header.
