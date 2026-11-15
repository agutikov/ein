# P1f.10 — what the phase's stages found outside their scope

**Phase:** [P1f.10](README.md). Opened 2026-11-15.

A stage keeps what it meets outside its own scope here, under a heading of its own, and fixes none of it. Each
row is shown to the user at the phase's stop or its close, and is then fixed by a stage, ruled as it stands, or
given an owner. A wrong answer among them is a row of `corpus/wrong_answers.toml` before anything fixes it.

## From S1f.10.9

Met by [S1f.10.9](s1f.10.9_what_the_blind_enumerator_takes_for_an_object.md#record-2026-11-15-tasks-1-to-3)'s
tasks 1 to 3, 2026-11-15, on the release binary of `d7050ccf`. The witnesses are under the untracked
`build/s1f.10.9/h9/` and verbatim below; `build/s1f.10.9/run.sh`, which the stage's record carries, runs them.
None is fixed, and no register row is filed: the stage was told to file none.

### H9-1 — A reserved word a program writes as a value is a name of the program and never an object

The reverse of E-1: a name the blind enumerator does **not** take. `hypgen::candidate_objects` drops seven
reserved words, `not false and or absent eq neq`, and every name `Program::categorise` calls a relation, the
kernel's `relation`, `rule` and `written` among them, whether or not the program writes the word as a value.
Five of them a program can write as a value today, and each loads: `eq`, `absent`, `false`, `relation`,
`written` (`(order eq)` and its four twins, probed; `neq`, `not`, `and`, `or` and `open` are refused at the
parse). The shipped stdlib writes one: `std.math.digits`'s table `cl` holds the orders `lt`, `eq` and `gt` as
values, `(cl eq 0 0 eq)`.

`h9/eq_as_value.ein`:

```
;; S1f.10.9, H9-1: a reserved word a program writes as a value is a name of
;; the program and never an object. std.math.digits writes `eq` so, in the
;; rows of its table `cl`.
(relation order Order :why "{?1} is an order")
(order lt) (order eq) (order gt)
(relation pick Thing :why "{?1} is picked")
(solve :no-hypothesis (order))
(query :goal (pick ?x))
```

`h9/eq_as_value_written.ein`:

```
;; S1f.10.9, H9-1's twin: eq_as_value.ein with the fact the enumerator does
;; not propose written by the program. It loads and root is alive.
(relation order Order :why "{?1} is an order")
(order lt) (order eq) (order gt)
(relation pick Thing :why "{?1} is picked")
(pick eq)
(solve :no-hypothesis (order))
(query :goal (pick ?x))
```

`ein oracle` on the first: H is `(pick gt)` and `(pick lt)`, 2 candidates, R 4 states, none holding `(pick
eq)`. On the twin: the same 2 candidates, R 4 states, every one holding `(pick eq)`, so the state the first
program never reaches is alive and owes nothing. The same on the shipped stdlib: the stage's witness
`e1/shipped_decimal_arith.ein` proposes `(total lt)` and `(total gt)` and not `(total eq)`.

**How it reads against the rulings.** `eq` in `(order eq)` is a constant in an argument position of a fact, so
it is a name of the program by Q-M1e2.9 (a), and `invariant::program_names` lists it.
[`solution_semantics.md` § 1.2](../../../docs/kernel/inference/solution_semantics.md) says *every name is in
the candidate list* and *a list that left one out was an engine bug* (W4, W15). Read so, this is a wrong
answer of that family. Read the other way, a reserved word is never an object, as a type a signature declares
is not, and § 1.2 has an exception it does not state. **Not filed as a register row**: which reading holds is
the user's, and it is the same question as Q-M1f.306's option (b) seen from the other side, whether a kernel
name a program writes as a value is an object. Not counted over the corpus.

**Today:** open, to be shown with Q-M1f.306. No owner.

### H9-2 — A written dunder fact registers the dunder as an auto-vivified relation of the program

The loader makes a registry entry for the head of every fact a file writes that no `(relation …)` declares
(`ein-ir/src/from_ir.rs`, *Undeclared relations are auto-created open-world*), and leaves a built-in predicate
out, *so vivifying one would put a phantom entry in the registry*. It does not leave the kernel's dunders out:
`ein saturate FILE --dump` on a program that writes `(__closed__ digit)` prints *Relations — auto-vivified
(1)*, `__closed__`, and on the same program with the closure derived by a rule prints none (the stage's
`e1/closed_by_fact.ein` and `e1/closed_by_rule.ein`). So whether the registry holds `__closed__` or
`__symmetric__` depends on how a closure or a symmetry came to hold, and every reader of the registry sees a
kernel name as a relation of the program in one case and not in the other.

The blind enumerator is one such reader, and what it does with the difference is E-1 and Q-M1f.306. **The
other readers were not examined**: the `relation` facts the loader states, `scope.rs`'s *outside* list,
`invariant::Universe`, the render views and the `--dump` listing itself. No wrong answer is known from them.

**Today:** open. It bears on where a fix under Q-M1f.306 (b) is made: one in the enumerator leaves this
difference in place.

### H9-3 — E-1 and E-7 are absent from the corpus's answers by the rung twelve programs take

The census of S1f.10.9 finds no corpus program with a dunder among its candidates and one with a relation's
name. On the 174 programs whose H an hrule, the obligations rung or `(solve :no-search)` builds, the blind
enumerator is not asked; were it (`hypgen::generate_blind` at root, which is not H and decides nothing), it
would name `__closed__` on 8 programs, 458 candidates, and a derived relation's name on 4, 200 candidates
(`cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example object_census`, the `alone`
columns, 2026-11-15):

| programs | rung today | the blind enumerator alone would name |
|---|---|---|
| `examples/binary_logic/{equivalent,knights,not_equivalent,satisfiable,unsatisfiable}.ein`, `stdlib/bool/inputs.ein`, `tests/stdlib/bool/inputs/{01_in_drives,02_required}.ein` | obligations | `__closed__` |
| `examples/domain_elim/{ab,b_branch,b_only}.ein` | an hrule | `surjective`, `total` |
| `examples/features/09_adjacent_via_same_house.ein` | obligations | `adjacent-via-bwd`, `adjacent-via-fwd` |

**Today:** not a wrong answer and nothing to fix. It is a dependency between two of the phase's questions:
[S1f.10.15](s1f.10.15_the_rung_a_loaded_rule_chooses.md) asks which rung a program takes, and a ruling there
that sends a program to the blind enumerator makes Q-M1f.306's (a) to (c) bear on programs they do not bear on
today. The census is re-taken by the same command after such a ruling.
