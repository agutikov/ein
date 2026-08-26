# Hypotheses from obligations — what the ladder cost, measured

**Stage:** [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md) · **Phase:** [P1d.2](README.md)
**Taken:** 2026-08-25, against the corpus at `5b6feb8` + this stage.
**Machine:** `utils/bench_env.sh` — i9-14900HX, pinned to a P-core, governor
`powersave`/`balance_performance`, turbo on. Release build, `snmalloc`.
**Re-take:** every number below comes from `ein solve … --events` /
`--json-summary`; the control arm is `EIN_OBLIGATION_CHOICE=off`.

The milestone's claim is that **a requirement is a choice point**. This is the
first stage where that is a number rather than a sentence, and the number has
two halves: how much narrower the branch is than the blind enumerator's
(§3), and whether the search it drives still finds every model (§2).

---

## 1. What shipped, and the one place it is not what the plan drew

The ladder, as [S1d.2.5](s1d.2.5_hypotheses_from_obligations.md) specified it:

| the program has | hypotheses come from | narrates |
|---|---|---|
| any `(hrule …)` | the user's hrules — an override, exactly as before | nothing |
| no hrule, undischarged obligations | **the facts that would discharge them** | `rung` |
| no hrule, no obligation rule | the blind combinatorial enumerator | nothing |

The candidate set of one obligation instance is read by running that
obligation's own `absent` guard **with the witness step skipped**
(`Matcher::scan_without`), so the branch is the guard rather than a
restatement of it — the [domain contract](domain_contract.md)'s C1, made
structural by [S1d.2.3](s1d.2.3_the_form.md)'s `(open ?R)`.

**The deviation: the rung proposes the union of every accepted obligation's
candidates, where the plan said "one chosen obligation's".** Not a
simplification — the two differ, and the reason the union is the only one this
engine can take is structural:

> The search is a **breadth-first lattice over root's `alive` set**: layer *k*
> enters the *k*-subsets of one fixed set, and `alive` is recomputed only at a
> layer barrier, where any commitment no longer wholly inside it is dropped.
> Branch on obligation *O*'s candidates alone and layer 1 is *O*'s alternatives
> — correct, mutually exclusive, jointly exhaustive. Layer 2 is then **pairs of
> them**, every one of which is two witnesses for a slot that needs one. A
> model needing an arrow some *other* obligation owes is not at any depth,
> because it was never in `alive`. `zebra2-minus-15` needs three.

"Choose one obligation, branch, recurse *at that node*" is a **depth-first**
move, and the traversal that could take it is
[P1d.10](../p1d.10_exhaustive_search/README.md)'s subject, not this stage's.
What survives of the per-instance structure is everything except the
partition: the walk order (§4), the decline rule, the `owed` / `branches` /
`declined` split the `rung` event reports, and the stuck report. Any single
obligation's candidate set is a subset of what the rung proposes, so nothing
is *lost* by taking the union — what is lost is the pruning a DFS would get
from committing to one requirement at a time, and §2 is the measurement of how
much that would have been worth here: **nothing at all**.

**The second deviation, and it is a tightening.** The contract's C4 says a
declined obligation makes the rung "fall through — to another obligation, or
to the blind generator". Per-obligation fall-through loses completeness
silently: the declined obligation's witnesses are then proposed by nobody, and
a model needing one is unreachable with no line in any stream saying so. The
rung therefore declines **the whole call** — one obligation that cannot be
branched on safely sends the entire generation to the blind enumerator, which
is what makes the rung's exhaustiveness claim unconditional rather than
per-instance. It is narrated (`mode=declined`, with the relation named), and
`tests/stdlib/bijection/06_blind_enumeration.ein` is the fixture.

---

## 2. The zebra family: every counter, both paths

The fixture the stage exists for is `examples/zebra2-obligations.ein` —
`zebra2.ein` with the `(hrule guess …)` and the `(query … :hrules …)` clause
deleted and **nothing else**, which is a claim `examples/gen_zebra2_variants.py
--check` makes rather than a reader. Its twin
`zebra2-minus-15-obligations.ein` is the same deletion applied to the
under-determined variant.

Both files owe exactly what their originals owe, which is the first thing to
check — a variant that owed differently would be a different puzzle:

| | owes at root | split by relation |
|---|---:|---|
| `zebra2.ein` / `zebra2-obligations.ein` | **36** | color 6 · nation 8 · drink 6 · smoke 8 · pet 8 |
| `zebra2-minus-15.ein` / …`-obligations.ein` | **46** | color 10 · nation 8 · drink 8 · smoke 10 · pet 10 |

### 2.1 `zebra2` — determinate, exhausted

`solve -e`, every counter the JSON summary reports:

| counter | `zebra2` (hrules) | `zebra2-obligations` |
|---|---:|---:|
| `enterings_total` | 101 | **101** |
| `enterings_alive` | 34 | **34** |
| `enterings_dead_pre` / `_post` | 0 / 67 | **0 / 67** |
| `layers_explored` | 2 | **2** |
| `nogoods_emitted` / `_subsumed` | 67 / 0 | **67 / 0** |
| `forced_positives` | 0 | **0** |
| `solution_nodes` | 1 | **1** |
| layer 1 `alive` / `joined` / `candidates` | 56 / 56 / 56 | **56 / 56 / 56** |
| layer 2 `alive` / `frontier` / `joined` | 23 / 10 / 45 | **23 / 10 / 45** |
| the model | the Zebra answer | **the same fact set, exactly** |

**Not one counter moves.** The re-baseline this stage was scheduled to argue
for is empty, and Q-M1d.4 — *may an obligation-driven generator change the
traversal?* — closes with the answer *it may, and on this corpus it does not*.

Why the two coincide is worth stating, because it is not luck. The hrule
proposes every (value, house) pair for each `*-loc`; the obligation rung
proposes, per unwitnessed slot, that slot's scan. A pair whose value is already
located and whose house is already filled is `fact_already_exists` or
`negated_fact` for the hrule — `functional-negative` and `injective-negative`
have written the row and the column — and is not owed at all for the rung. The
two sets are the same set, arrived at from opposite directions: the hrule
enumerates and the negatives subtract; the obligations never enumerate what
the negatives would have removed. The *raw* counts differ and say so — **125
candidates for the hrule against 180 for the rung** at root, the rung's
surplus being the `total-owed` / `surjective-owed` duals proposing the same
arrow from both ends, which `seen_in_call` folds.

### 2.2 `zebra2-minus-15` — under-determined

The interesting case, because 32 models can disagree where one cannot.

| | `-m 2` | `-m 3` |
|---|---|---|
| `enterings_total`, both paths | 4 656 | 48 745 |
| models found, both paths | 28 | 32 |
| model sets | **equal** | **equal** |
| every other counter | identical | identical |

`-m 3` is where all 32 models are ([layer census
§4](../p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers));
depths 4 and 5 exist only to prove there are no more. **The full-depth
re-take is §2.3.**

The comparison is pinned as a test —
`ein-infer/tests/obligation_rung.rs::the_theory_finds_what_the_hrule_finds` —
at `-m 2` in the default gate and `-m 3` under `EIN_CORPUS_SLOW=1`, which is
where the corpus sweep draws the same line. It compares **model sets**, not
counters: counters were licensed to move and answers were not, and a test that
pinned the counters would be testing the fixtures' arithmetic.

### 2.3 The full-depth re-take

T1d.2.5.5 asked for the twin against [the 618 076 / 416 s
baseline](../p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers).
Both arms were run to the depth cap, back to back, on the same machine:

| `solve -e` (depth 5) | `zebra2-minus-15` | `zebra2-minus-15-obligations` |
|---|---:|---:|
| `enterings_total` | 618 076 | **618 076** |
| `enterings_alive` | 598 955 | **598 955** |
| `enterings_dead_post` | 19 121 | **19 121** |
| `layers_explored` | 5 | **5** |
| `nogoods_emitted` | 19 121 | **19 121** |
| `solution_nodes` | 32 | **32** |
| model sets | — | **equal** |
| `exhausted` | `false` | **`false`** |
| wall | 422.2 s | 429.9 s |

The census's 618 076 reproduces exactly, which is the other half of the
claim — the baseline is a *baseline* and not a number that drifted. `exhausted
= false` on both arms is the depth cap with a non-empty frontier, unchanged by
this stage and still
[Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)'s
and [S1d.10](../p1d.10_exhaustive_search/README.md)'s to answer. The 1.8 %
wall difference is the two runs' background load — the first had the second's
compile behind it — and is not a claim.

**Neither arm's models owe anything**: 0 of the 32 recorded nodes on either
side has a non-empty tally, so the closed-and-owing corner does not arise here
and the model-set equality is between two sets of fully discharged states.

---

## 3. The control arm: what the theory is worth

`EIN_OBLIGATION_CHOICE=off` declines every call, collapsing the ladder to its
pre-S1d.2.5 shape. Same file, same theory, one rung apart:

| `examples/…` | rung 2 — layer 1 `alive` | blind — layer 1 `alive` | ratio |
|---|---:|---:|---:|
| `zebra2-obligations.ein` | **56** | 3 734 | **66.7×** |
| `zebra2-minus-15-obligations.ein` | **96** | 3 774 | **39.3×** |

Layer 1 rather than a wall clock, because **the blind arm does not finish**:
layer 2 joins C(3 734, 2) ≈ 6.97 M candidate sets where the rung joins 45, and
the run has no end anyone has waited for. That is the whole comparison — the
same puzzle, the same rules, and the difference between a search of 101
enterings and one nobody can run is which of the two facts about `color-loc`
the generator was told: *guess a (Color, House) pair*, in a keyword beside the
question, or *`(bijective color-loc)`*, in the theory.

The blind arm's 3 734 is also where the domain contract's C4 bites in the
other direction: 962 of those candidates are `is-a*` arrows and 984 are
`next-to`, relations the puzzle never intended anyone to guess about. The rung
proposes none of them — not because it filters them, but because nothing owes
them.

---

## 4. The choice heuristic, measured — and it is inert

T1d.2.5.2 asked which obligation to branch on: **fail-first** (smallest
candidate set) against **rule order** (report order) as the control.
`EIN_OBLIGATION_CHOICE` selects, and both arms were run on both fixtures:

| | `rule-order` | `fail-first` |
|---|---:|---:|
| `zebra2-obligations -e` — enterings / k | 101 / 1 | **101 / 1** |
| …raw `hyp` candidates / emitted | 397 / 90 | **397 / 90** |
| …best-of-9 wall | 30.4 ms | 31.8 ms |
| `z2-minus-15-obligations -e -m 2` — enterings / k | 4 656 / 28 | **4 656 / 28** |
| …raw / emitted | 7 686 / 3 295 | **7 686 / 3 295** |
| …best-of-9 wall | 1 597 ms | 1 593 ms |

**Nothing moves, and the reason is structural rather than a property of these
two files.** Two mechanisms erase the order between the heuristic and the
traversal:

1. **The emitted set is order-free.** The rung proposes the union over
   instances and `seen_in_call` dedups it, so which instance offers an arrow
   first changes which instance gets credited for it and nothing else.
2. **The layer re-sorts.** `alive` is an `FxHashSet` and
   `apriori::order_candidates` sorts every layer canonically (`lattice_order =
   "lex"`, the default and what every baseline was taken under), so no order
   the generator could impose survives to become a traversal.

So the heuristic is recorded as **inert, with the number: 0 difference on
every counter, on both fixtures**, per F9's rule. It is kept rather than
deleted because it is the *interface* a depth-first traversal would need on
day one, and because it costs one `sort_by_key` on a list whose length is the
instance count. What would make it live is the deviation in §1 being closed —
a per-node branch on one obligation — at which point "which one" is the whole
question. That is [P1d.10](../p1d.10_exhaustive_search/README.md)'s to answer,
and this row is the note it inherits.

---

## 5. The corpus: what moved

**No verdict moved, on any entry.** `corpus_exits.txt` gained 21 cells — the
two new fixtures — and modified exactly one: `render rules` on
`tests/stdlib/bijection/06_blind_enumeration.ein`, which went 1 → 0 because
the file gained a rule of its own and that subcommand exits 1 when a program
has none.

**No counter moved, on any entry.** Of the 8 081 corpus shape digests, 168
lines moved and the split is the whole claim:

| | renderings | what they are |
|---|---:|---|
| **new** | 90 | the two new fixtures, 45 apiece |
| **changed** | 42 | the `hyp` and `hyp+closed` previews of the **21** programs that reach the new rung, which now print which one they took |
| **changed** | 36 | `06_blind_enumeration.ein`, whose *text* this stage changed |
| **removed** | 0 | |

Every `dot`, `dump`, `explain`, `trace`, `commit`, `solve` and `saturate`
digest of every pre-existing entry is byte-identical. The 42 are `--hyp-stats`
text and not a counter: a draft that printed the rung line for the hrule path
too moved **206** digests, and restricting it to the rung this stage added is
what took that back to 42.

**And those 42 describe a different fork from the search — which is worth
knowing before reading one against §2.** `--hyp-stats` and `--json-summary`
run `emit_closed` on a fork of their own before saturating, a surface contract
that predates this stage; a real solve carries only the `(__closed__ R)` a
program authored or `infer-closure` derived, and on the zebra family that is
none. The ladder made the difference *visible* for the first time, because the
rung reads the same closed set the blind enumerator does. So on
`zebra2-obligations.ein` the preview closes `nation-loc` — no rule positively
concludes it — scopes its 8 obligations out under `pre.closed_relation`, and
prints `branches 28 · declined 8 · uncovered 2` over 43 candidates, where the
search that actually runs prints `branches 36 · declined 0 · uncovered 4` over
56. Both are true of the world they describe, and **§2's are the search's**,
which is where the models come from. The hrule previews did not move for the
same reason rung 1 narrates nothing: it never consults the closed set, so
`zebra2.ein` reads 125 raw / 56 emitted on both surfaces and the rung fixtures
read 180 / 56 on one and 140 / 43 on the other.

The rung census, over the 156 files under `examples/`, `tests/` and `stdlib/`
that load:

| rung | files | |
|---|---:|---|
| blind — no obligation rule declared | 114 | the ladder is not consulted, and narrates nothing |
| hrules — the override | 19 | 19 of them owe something and are branched by their hrule anyway |
| obligations | 11 | of which **2 branch**: the two new fixtures. The other 9 owe nothing at root — 3 are the stdlib modules loaded standalone, 6 are the *satisfied* halves of the S1d.2.4 conformance pairs |
| stuck | 11 | owed, and `:no-hypothesis` names the relation owed |
| declined | 1 | `06_blind_enumeration.ein`, by construction |

**That the pre-existing corpus reaches the new rung's *generating* mode
nowhere is not an accident of this stage — it is what T1d.2.5.4 predicted.**
Every entry that searches carried `:hrules`; every entry that owes scoped the
owed relation out. There was no third case, which is exactly why the rung
needed a fixture of its own.

### 5.1 The eleven stuck programs

A state that owes something it may not branch on proposes nothing, so
`complete()` says yes, so the node is recorded as a model — while the tally
beside it says the requirement is unmet. All eleven are in that state today
and all eleven were before this stage; what is new is that they **say so**, in
a `rung` line and in `--hyp-stats`. Ten owe because `:no-hypothesis` names the
relation ([`23_total_owed.ein`](../../../tests/stdlib/algebra/23_total_owed.ein)
and its nine relatives); the eleventh regime — closed *and* owing — is
[`03_closed_and_owing.ein`](../../../tests/stdlib/closure/03_closed_and_owing.ein),
whose relation is `__closed__` and which the domain contract §3 already banked.

**No verdict word moves on any of them here.**
[S1d.2.6](s1d.2.6_verdicts_counters_corpus.md) is where that is decided, and
this is the evidence it decides on.

---

## 6. The completeness condition

The ladder is exhaustive **iff obligations and saturation between them
determine every remaining open fact**. The rung reports the structural half of
that as a number — `uncovered`, the count of hypothesis-eligible relations no
obligation names, under the same eligibility test the blind enumerator
applies:

- `uncovered = 0` ⇒ exhaustive **by construction**: every relation a
  hypothesis could be about is one some obligation owes, so a branch set that
  discharges every debt has left nothing undecided.
- `uncovered > 0` ⇒ the claim now rests on saturation determining those
  relations, which only a model-set comparison settles.

On both zebra fixtures `uncovered = 4` — `is-a`, `is-a*`, `right-of`,
`next-to` — and §2 is where the comparison settled it: the four are
saturation-determined (`is-a` is authored and no rule asserts it; `is-a*`,
`right-of` and `next-to` are closed by the puzzle's own rules from what is),
so the model sets agree exactly.

**What this stage does not build**, and the reason is worth recording rather
than leaving as a gap: the *per-state* version of the same question — "how
many facts would the blind enumerator still propose at a node the rung called
complete" — needs a blind pass over that node's KB, and the blind pass is not
a read. `enable_lookahead_kill_cache` writes `(not h)` into the KB it walks,
which would change the node's `state_key` and therefore the model dedup. A
probe that had to disable a config flag to avoid changing the answer is a
probe that is measuring a different engine. The number belongs to
[P1d.3](../p1d.3_model_sets/README.md)'s compact-model-set work, where
the closed-world completion that makes it meaningful also lives.

---

## 7. Cost

The rung is off for every program that declares no obligation rule — one
`obligations.is_empty()` on the first line — and off for every program that
declares an `(hrule …)`. So the phase's cost guard is a claim about programs
that pay *nothing*, and an A/B against a build of `5b6feb8` says they pay it:

| | `5b6feb8` | + S1d.2.5 | |
|---|---:|---:|---|
| `zebra -e` | 46.2 ms | 45.8 ms | −0.8 % |
| `zebra2 -e` | 30.6 ms | 30.4 ms | −0.9 % |
| `zebra2-minus-15` (solve) | 32.4 ms | 31.5 ms | −2.7 % |
| `branching/06 -e` | 198.4 ms | 192.9 ms | −2.8 % |

Best-of-11, same machine, same minute; the differences are the run-to-run
spread and the sign is noise. `zebra -e` is inside
[P1a.6](../../../docs/history/m1a_rust/README.md)'s 47.5 ms baseline.

Where the rung *does* run it costs one matcher pass per `(obligation rule,
activator)` per generation call — the same shape `obligations::tally` already
pays per quiescence, and paid twice at a node that both tallies and generates.
`zebra2-obligations -e` at 31.0 ms against `zebra2 -e`'s 30.4 ms is what that
doubling is worth on the file that pays it most: **+0.6 ms over 101
enterings**, inside the spread.

---

## 8. What this closes and what it leaves

| | |
|---|---|
| **[Q-M1d.4](../open_questions.md)** | **closed.** The generator may change the traversal; on this corpus it does not — 101 enterings and 48 745 enterings, both paths, counter for counter |
| the idea note's `:hrules` complaint | **closed as a fixture.** `zebra2-obligations.ein` solves with no hypothesis rule in the file, and the thing that replaced it is `(bijective color-loc)` |
| T1d.2.5.3's completeness test | **passing**, on one determinate puzzle and one 32-model one, pinned in `cargo test` |
| the choice heuristic | **inert, with the number** — and the reason is the traversal, not the fixtures |
| "one chosen obligation" | **not built, and §1 says why.** It is a depth-first move; the engine's search is a breadth-first lattice over a fixed `alive` |
| the per-state leftover-open count | **not built, and §6 says why.** It belongs with the closed-world completion in P1d.3 |
| the stuck states | **reported, not judged.** Eleven of them, and S1d.2.6 is where a verdict word may move |
