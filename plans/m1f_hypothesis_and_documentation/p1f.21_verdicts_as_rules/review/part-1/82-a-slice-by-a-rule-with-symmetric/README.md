# 82 · A slice by a rule, activated with `symmetric`, loads now

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-34--the-milestones-l-109-program-loads)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it.

> **The pictures are files under `runs/`, which git ignores**, so they are embedded as `<img>` and not as links (a
> link into `runs/` would be a missing file to the gate's link check on a clean checkout). If one is missing,
> `bash tools/run.sh` (about 5 s) writes every run and drawing of this page, on the binary [§ 5](#5-on-the-built-engine)
> names.

## In one paragraph

Your rule `about-1` collects the **facts about one relation**: activated with `(about-1 symmetric)`, it reads every
`(symmetric ?r)` whose `?r` is a declared relation and concludes `(in-slice-1 symmetric ?r)`. Appended to
`examples/zebra2.ein` it yields one fact, `(in-slice-1 symmetric next-to)`. Until S1f.21.17 the loader **refused**
that program (the milestone's L-109); now it **loads under one warning**, and its H and $R$ are `zebra2`'s.
The warning is not about `about-1`. Writing the **rule name** `symmetric` as an **argument** lets the load check read
`std.algebra`'s generic rule `imply1` as able to derive `(symmetric <a fact>)`, an activator that would bind a
relation's name to a fact, which activates nothing. Nothing in `zebra2` activates `imply1`, so no run derives one:
the warning is read off the program's text, as Q-M1f.1 says it is. The six small programs of [§ 2](#2-six-small-programs-one-step-each)
take it apart one step at a time.

## The question

The program is valid, and it loads and answers as `zebra2` does, so L-109 is closed whatever you pick. What is left
is **the warning**. It names a rule nothing activates, and it fires on `imply1`'s own intended use too:
[`imply1-into-a-rule.ein`](programs/imply1-into-a-rule.ein), `(imply1 equivalence symmetric)` with `equivalence` an
undeclared marker, derives `(symmetric next-to)` and the mirror `(next-to H2 H1)`, and still warns. So does
[`imply1-into-a-rule-declared.ein`](programs/imply1-into-a-rule-declared.ein), the marker declared
(`runs/imply1-into-a-rule*.solve.txt`). Item 81's eighteen lines are the same case: each names a rule with no plan.

The options, from as built to none, each with what it prints on these programs, are in
[`ANSWERS.md`](../ANSWERS.md) under item 82:

- **(a)** as built: a rule that *could* derive such an activator is warned of, read off the text. **Default.**
- **(b)** only a rule something can activate;
- **(c)** (b), and a premise `(?R1 ?a)` read by the relations its written activators name;
- **(d)** no load line, only the run's, where a state holds such an activator;
- **(e)** neither line.

## 1. What a slice is, and where `about-1` comes from

- **Your words**, on Q-M1f.426 (a), which started it: *"would be nice to formalize this KB slice with something, e.g.
  ein query or rule that selects all nodes where relation is argument (is it correct definition?), and together with
  `constraints` allow to render arbitrary query."*
- **A fact about a relation** is a fact with a relation's atom under one of its argument arrows: `(symmetric next-to)`
  says something of `next-to`
  ([Q-M1f.381](../../../../../../docs/decisions/language/Q-M1f.381-a-relation-is-the-atom-under-a-facts-first-arrow.md)).
- **The engine has that slice as a view**, built by P1f.23: `ein render constraints` draws every fact root holds with
  a relation of the program directly under an argument arrow
  ([Q-M1f.523](../../../../../../docs/decisions/cli/Q-M1f.523-the-facts-about-relations-are-every-fact.md),
  [Q-M1f.530](../../../../../../docs/decisions/cli/Q-M1f.530-ein-render-constraints-draws-roots-facts-about.md)),
  and `ein render kb --query N` draws the facts that answer a goal
  ([Q-M1f.529](../../../../../../docs/decisions/cli/Q-M1f.529-ein-render-kb-draws-the-slice-of-a-goal.md)).
- **In the language, no goal can say it.** A goal cannot bind a fact's relation by matching: `(?f ?r)` is refused,
  since a relation variable is a rule's parameter. A pattern also sees one count, and nothing says *?r is a relation*
  of an undeclared atom. What runs is a **rule whose parameter is the fact's relation**: `about-1`, one rule per
  count and one activator per relation asked of. That is S1f.23.7's examples, § 4.2 to 4.6; the file is deleted with
  P1f.23 and read by `git show bdf57cb6^:plans/m1f_hypothesis_and_documentation/p1f.23_review_processing/s1f.23.7_examples.md`.
  Its § 4.5 met the refusal, and the milestone's [L-109](../../../../findings.md) kept it as your call.

The rule, as L-109's row writes it:

```lisp
(rule about-1 (?F)                        ; ?F is a parameter, so it may stand in a relation's place
  :match  (and (?F ?r) (relation ?r))     ; a fact of relation ?F whose one argument is a declared relation
  :assert (in-slice-1 ?F ?r)              ; restated as a fact of in-slice-1
  :why    "{?F} says something of {?r}")
(about-1 symmetric)                       ; its activator: ?F = symmetric
(query :goal (in-slice-1 ?f ?r))
```

A rule is activated by each fact of its name with as many arguments as it has parameters
([Q-M1f.115](../../../../../../docs/decisions/language/Q-M1f.115-an-activator-is-a-fact-of-its-rules-name-and-one.md)),
and a plan is compiled for each such activator: `(about-1 symmetric)` gives one plan,
`:match (and (symmetric ?r) (relation ?r))`.

<img src="runs/steps/rule.about-1.svg" alt="about-1 drawn by ein render rule: in its match, a fact whose arrow 1 points at the diamond ?F and arrow 2 at ?r, and the fact (relation ?r); in its assert, a dashed fact of in-slice-1 with arrows 2 and 3 at ?F and ?r">

*`ein render rule --name about-1` (`runs/steps/rule.about-1.svg`): `?F` is a diamond at arrow 1, the relation's
place, in the match, and at arrow 2, an argument's place, in the assert.*

**How to read the knowledge-base drawings below.** A box is an atom; an octagon is a fact, its arrow `1` drawn up to
its relation and `2`, `3`, … down to its arguments; a dashed fact is derived, labelled with its rule; the colour is
the fact's relation's.

## 2. Six small programs, one step each

Each is a few forms in [`programs/`](programs/). Each adds one thing to the one before, and ends with
`(solve :no-search)`: the question is about root, so $R$ is {∅} and no search enters the read-out. Every run exits 0.

| step | what it adds | `warning:` lines | the slice, query 1 | the program |
|---|---|---|---|---|
| 1 | the slice alone; `symmetric` a marker no rule is named | 0 | `(in-slice-1 symmetric next-to)` | [`step-1-the-slice-alone.ein`](programs/step-1-the-slice-alone.ein) |
| 2 | a rule named `symmetric`, as `std.algebra`'s | 0 | the same | [`step-2-a-rule-named-symmetric.ein`](programs/step-2-a-rule-named-symmetric.ein) |
| 3 | a rule shaped as `std.algebra`'s `imply1`, **never activated** | **1**, the load's | the same | [`step-3-and-imply1.ein`](programs/step-3-and-imply1.ein) |
| 4 | step 3 without `(about-1 symmetric)` | 0 | none | [`step-4-imply1-without-the-slice.ein`](programs/step-4-imply1-without-the-slice.ein) |
| 5 | step 3 with `(about-1 bijective)` in its place | 0 | `(in-slice-1 bijective next-to)` | [`step-5-bijective-in-its-place.ein`](programs/step-5-bijective-in-its-place.ein) |
| 6 | step 3 with `imply1` activated on a fact | **2**, the load's and the run's | `(in-slice-1 symmetric next-to)` | [`step-6-a-run-derives-one.ein`](programs/step-6-a-run-derives-one.ein) |

Step 3 is `zebra2-about-1.ein` in nine forms; step 4 is `zebra2` as it is; step 5 is the bijective twin; step 6 is
what the warning is about, made to happen. Every step's runs are in `runs/steps/`: `<program>.solve.txt`, stderr
alone `.stderr-only.txt`, root `.saturate-dump.txt`, H `.show-hyps.txt`, and three drawings, `.kb.svg` (root),
`.slice.svg` (query 1's slice, `ein render kb --query 1`) and `.constraints.svg` (the engine's facts about
relations).

Run any of them from the repository's root:

```sh
P=plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/review/part-1/82-a-slice-by-a-rule-with-symmetric/programs
ein.rs/target/release/ein solve $P/step-3-and-imply1.ein                 # the read-out, the warning last
ein.rs/target/release/ein solve $P/step-3-and-imply1.ein 2>&1 >/dev/null # the warning alone
```

### Step 1 — the slice alone

```lisp
(relation next-to House House :why "{?3} is next to {?4}")
(next-to H1 H2)
(symmetric next-to)                       ; a fact about next-to; here no rule is named symmetric
(rule about-1 (?F) …)                     ; as above
(about-1 symmetric)
```

The read-out's query, `runs/steps/step-1-the-slice-alone.solve.txt`; stderr is empty:

<!-- output -->
```text
  query             (in-slice-1 ?f ?r)
                      ?f = symmetric · ?r = next-to
  as :expect        (model (in-slice-1 symmetric next-to))
```

| root, `ein render kb` | query 1's slice, `--query 1` |
|---|---|
| <img src="runs/steps/step-1-the-slice-alone.kb.svg" alt="root of step 1: the declaration&#x27;s two facts, (next-to H1 H2), (symmetric next-to), (about-1 symmetric) and the dashed (in-slice-1 symmetric next-to) by about-1"> | <img src="runs/steps/step-1-the-slice-alone.slice.svg" alt="the slice of query 1: the one fact (in-slice-1 symmetric next-to), by about-1, its arrows at in-slice-1, symmetric and next-to"> |

The atom `symmetric` is pointed at twice over. Arrow 1 of `(symmetric next-to)` points at it as a **relation**.
Arrow 2 of `(about-1 symmetric)` and of `(in-slice-1 symmetric next-to)` points at it as an **argument**. That is
what makes `about-1` work, and it is what the check reads in step 3.

**The engine's own slice is another thing.** `ein render constraints` on the same program keeps five facts: the two
the declaration stores, `(symmetric next-to)`, and the slice's own two, `(about-1 symmetric)` and
`(in-slice-1 symmetric next-to)`, since `symmetric` is a relation of the program and stands under their arrow 2.
`(next-to H1 H2)` is left out. `about-1` *restates* the facts of the one relation it is given as new facts; the view
*selects* what root holds.

<img src="runs/steps/step-1-the-slice-alone.constraints.svg" alt="the constraints view of step 1: the declaration&#x27;s facts, (symmetric next-to), (about-1 symmetric) and (in-slice-1 symmetric next-to); no (next-to H1 H2)">

### Step 2 — a rule named `symmetric`

Step 1, and the rule `std.algebra` has (`stdlib/algebra.ein`, its `symmetric`):

```lisp
(rule symmetric (?rel)
  :match  (?rel ?a ?b)
  :assert (?rel ?b ?a)
  :why    "{?rel} is symmetric: {?a} ↔ {?b}")
```

Now `(symmetric next-to)` is two things at once: the fact about `next-to` that the slice reads, and the activator of
the rule `symmetric`, binding `?rel` to the **name** `next-to`. Root derives `(next-to H2 H1)` by `symmetric`. The
slice is unchanged, and there is still **no warning**.

<img src="runs/steps/step-2-a-rule-named-symmetric.kb.svg" alt="root of step 2: as step 1, and the dashed (next-to H2 H1) by symmetric">

### Step 3 — and `imply1`: the warning

Step 2, and a rule shaped as `std.algebra`'s `imply1` (`stdlib/algebra.ein`), which **nothing activates**:

```lisp
(rule imply1 (?R1 ?R2)
  :match  (?R1 ?a)
  :assert (?R2 ?a)
  :why    "(imply1 {?R1} {?R2}): {?R1}({?a}) → {?R2}({?a})")
```

No `(imply1 … …)` fact is written or derived, so the saturation compiles no plan of it
(`runs/steps/step-3.plans.txt`: `imply1  :params (?R1 ?R2)  no plan compiled`), and root is step 2's. Yet stderr is:

<!-- output -->
```text
warning: step-3-and-imply1.ein: rule 'imply1': its :assert writes (?R2 ?a), whose ?a is rule 'symmetric''s parameter ?rel, which names a relation, and (?R1 ?a) can bind ?a to a fact, as an argument of the relation ?R1 names — a relation is named by a symbol, not a fact, so such an activator activates nothing
```

That is `zebra2-about-1.ein`'s line word for word, `std.algebra.imply1` there for `imply1`. It is read off the
program's text, by three readings that meet:

<img src="runs/the-check.svg" alt="the check&#x27;s chain: (about-1 symmetric) writes the rule name symmetric as an argument; with imply1&#x27;s conclusion (?R2 ?a), Q-M1f.42 reads it as writing into the rule symmetric; symmetric&#x27;s ?rel names a relation; Q-M1f.41 reads ?a as able to be a fact; so imply1 could derive (symmetric &lt;a fact&gt;), and the load warns; a second warning only where a state holds one. Steps 4 and 5 break the chain at its first two links">

*A drawing made by hand for this page, `tools/the-check.dot` → `runs/the-check.svg`; it explains the check and is
no render of a knowledge base.*

1. **Which rules `(?R2 ?a)` can write into**
   ([Q-M1f.42](../../../../../../docs/decisions/language/Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)):
   a conclusion whose relation is a parameter is read as writing into every rule of its count whose name the program
   writes **as an argument**, in a fact at any depth, a rule's clause or the `(solve …)` form. A rule's name reaches
   a parameter only from such a place. Here the one such name of count 1 is `symmetric`, from `(about-1 symmetric)`.
2. **That rule's parameter names a relation**: in `(?rel ?a ?b)`, `?rel` stands at arrow 1.
3. **What `?a` can be**
   ([Q-M1f.41](../../../../../../docs/decisions/language/Q-M1f.41-a-position-holds-a-fact-unless-the-program.md)):
   `?a` is bound by `(?R1 ?a)`, an argument of a relation a parameter names, and such a position holds any object,
   a fact among them.

So `imply1` *could* derive `(symmetric <a fact>)`, an activator binding `?rel` to a fact. Such an activator
activates nothing: no plan of the rule is compiled for it, since its pattern would be `(<a fact> ?a ?b)`, a fact in a
relation's place. The load says so whether or not a run derives one
([Q-M1f.1](../../../../../../docs/decisions/language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md):
*the load's warning is read off the program, so it is the same on every command and every run*). The check does not
read whether `imply1` has an activator. No answer and no exit code moves for it.

| `imply1`, `ein render rule --name imply1` | `symmetric`, `--name symmetric` |
|---|---|
| <img src="runs/steps/rule.imply1.svg" alt="imply1: in the match a fact whose arrow 1 is at the diamond ?R1 and arrow 2 at ?a; in the assert a dashed fact whose arrow 1 is at ?R2 and arrow 2 at ?a"> | <img src="runs/steps/rule.symmetric.svg" alt="symmetric: in the match a fact whose arrow 1 is at ?rel, arrows 2 and 3 at ?a and ?b; in the assert the same with ?b and ?a swapped"> |

*`?R2` stands at arrow 1 of `imply1`'s conclusion; `?rel` at arrow 1 of `symmetric`'s pattern. If `?R2 = symmetric`,
`imply1`'s conclusion is `symmetric`'s activator, with `?a` in `?rel`'s place.*

### Step 4 — without the activator: no warning

Step 3 with `(about-1 symmetric)` deleted, the rule `about-1` kept. Stderr is empty. In `(symmetric next-to)` the
name `symmetric` stands at arrow 1, as the relation, so the program writes no rule's name as an argument, and link 1
of the chain finds nothing. **This is `examples/zebra2.ein` as it is**: it imports `std.algebra`, `imply1` among its
rules (an import brings its module whole,
[Q-M1f.247](../../../../../../docs/decisions/imports/Q-M1f.247-every-import-brings-its-module-whole.md)), writes
`(symmetric next-to)`, and loads with no warning. Query 1 answers no row; its slice keeps no fact
(`runs/steps/step-4-imply1-without-the-slice.slice.dot`: `// the slice keeps no fact`).

### Step 5 — `bijective` in its place: no warning

Step 3 with `(bijective next-to)` written and `(about-1 bijective)` for `(about-1 symmetric)`. The program does
write `bijective` as an argument, but no rule is named `bijective`, so link 1 finds no rule. Stderr is empty, and the
slice is `(in-slice-1 bijective next-to)`. **This is the twin** `zebra2-about-1-bijective.ein`, whose `bijective`
is a marker `std.bijection` reads; its rule is `bijective-setup`.

<img src="runs/steps/step-5-bijective-in-its-place.kb.svg" alt="root of step 5: (bijective next-to) and (symmetric next-to) beside the declaration, (about-1 bijective), the dashed (in-slice-1 bijective next-to) by about-1 and (next-to H2 H1) by symmetric">

### Step 6 — a run that derives one: the second warning

Step 3, and `imply1` activated on a relation whose argument is a fact:

```lisp
(relation item Thing :why "{?3} is an item")
(item (H1 H2))                            ; an item that is a fact
(imply1 item symmetric)                   ; ?R1 = item, ?R2 = symmetric: (item ?a) ⇒ (symmetric ?a)
```

Root derives `(symmetric (H1 H2) :rule imply1)` (`runs/steps/step-6-a-run-derives-one.saturate-dump.txt`). It is an
activator of the rule `symmetric` with `?rel` bound to the fact `(H1 H2)`, and it **activates nothing**. The state
holds it as a fact and no more: `symmetric` mirrors `next-to` as before and nothing else. A **second** line names it,
placed at `symmetric`'s pattern `(?rel ?a ?b)`, line 20; both lines come after the read-out, when the run ends
([Q-M1f.580](../../../../../../docs/decisions/diagnostics/Q-M1f.580-a-loads-and-a-runs-warnings-are-printed-when.md)):

<!-- output -->
```text
warning: step-6-a-run-derives-one.ein: rule 'imply1': its :assert writes (?R2 ?a), whose ?a is rule 'symmetric''s parameter ?rel, which names a relation, and (?R1 ?a) can bind ?a to a fact, as an argument of the relation ?R1 names — a relation is named by a symbol, not a fact, so such an activator activates nothing
warning: step-6-a-run-derives-one.ein:20:12: rule 'symmetric': its activator (symmetric (H1 H2)) binds its parameter ?rel, which names a relation, to the fact (H1 H2), and activates nothing — a relation is named by a symbol, not a fact
```

The slice is unchanged. `(symmetric (H1 H2))` is of the relation `symmetric`, but `(relation (H1 H2))` is not held,
so `about-1` does not take it.

<img src="runs/steps/step-6-a-run-derives-one.kb.svg" alt="root of step 6: (item (H1 H2)) and the dashed (symmetric (H1 H2)) by imply1 both point their arrow 2 at the fact (H1 H2), drawn in a frame; (imply1 item symmetric); the slice&#x27;s fact by about-1; (next-to H2 H1) by symmetric">

*The fact `(H1 H2)` is the octagon in the frame `item, symmetric ctx`, pointed at by arrow 2 of `(item (H1 H2))`
and of the dashed `(symmetric (H1 H2))` by `imply1`.*

## 3. Back on `zebra2`

[`zebra2-about-1.ein`](programs/zebra2-about-1.ein) is step 3 at full size: `examples/zebra2.ein` whole, a header
before it, and L-109's three forms after it.

- **The chain is step 3's.** `(about-1 symmetric)` is the one place the program writes the rule name `symmetric` as an
  argument; `std.algebra.imply1` is imported and **never activated**. The saturation lists it
  `std.algebra.imply1  :params (?R1 ?R2)  no plan compiled`, one of the 26 rules with no plan
  (`runs/about-1.plans-imply1-symmetric.txt`, from `runs/about-1.saturate-dump.txt`). So no state holds
  `(symmetric <a fact>)`: one warning line, the load's, and no second.
- **Not flagged**: the static check finds no site, `monotone  yes — no site` (`runs/about-1.monotone.txt`), and the
  read-out has no `WARNING` row. Item 66's fixtures are flagged because a commitment there can derive such an
  activator; here nothing can.
- **What the three forms add is one fact at root**, `(in-slice-1 symmetric next-to)`, and the line. H and $R$ are
  zebra2's, row for row ([§ 5](#5-on-the-built-engine)).

<img src="runs/about-1.query-2.svg" alt="query 2&#x27;s slice on zebra2-about-1: the one fact (in-slice-1 symmetric next-to), by about-1">

*`ein render kb programs/zebra2-about-1.ein --query 2` (`runs/about-1.query-2.svg`; folded,
`runs/about-1.query-2-fold.svg`). The bijective twin's is five facts, `runs/bijective.query-2.svg`.*

**Beside the engine's slice.** `ein render constraints` keeps 197 facts of `zebra2`'s root and 199 of
`zebra2-about-1`'s. The two more are `(about-1 symmetric)` and `(in-slice-1 symmetric next-to)`: the slice's own
machinery enters the slice. `about-1`'s query keeps one fact, because it asks of one relation, `symmetric`, at one
count. The counts by relation are `runs/about-1.constraints-by-relation.txt`, whose first lines are:

<!-- output -->
```text
     18 relation
     16 co-located
     16 co-located-negative
     10 std.algebra.surjective
     10 std.algebra.total
      9 std.contradiction.clash-0
      9 std.contradiction.clash-2
      5 bijective
      5 guess
      5 open
…
199 facts in all
```

The drawings are `runs/about-1.constraints.svg` and `runs/zebra2.constraints.svg`.

## 4. The finding's figures, and today's

| | L-109's figure (as it was), then V-34's | **the built engine** | the run |
|---|---|---|---|
| loads? | L-109: refused, `ein solve` exit 1, *kb load error: rule 'std.algebra.imply1': its :assert writes (?R2 ?a) …*; V-34: `ein saturate` exits 0 | **loads**: every command run exits 0 but the oracle (below) | `runs/about-1.solve.txt`, `.saturate-dump.txt`, `.show-hyps.txt` |
| the warning | V-34: one, naming `std.algebra.imply1` | **one line**, on every command: *rule 'std.algebra.imply1': its :assert writes (?R2 ?a), whose ?a is rule 'symmetric''s parameter ?rel, which names a relation, and (?R1 ?a) can bind ?a to a fact, … so such an activator activates nothing* | `runs/about-1.solve.txt` (its last line before the exit), `runs/about-1.kb.stderr.txt` |
| what root derives of the slice | V-34: `(in-slice-1 symmetric next-to)` | **`(in-slice-1 symmetric next-to :rule about-1)`**, the one fact of `in-slice-1` (0 before saturation, 1 after) | `runs/about-1.saturate-dump.txt`, `runs/about-1.in-slice-1.txt` |
| flagged? | — | **no**: `lattice monotone (the static check) — R is convex: no hole`, no `WARNING` row; `--monotone`: `yes — no site` | `runs/about-1.solve.txt`, `runs/about-1.monotone.txt` |
| the second query, `(in-slice-1 ?f ?r)` | — | **one row**, `?f = symmetric · ?r = next-to`; `as :expect (model (in-slice-1 symmetric next-to))` | `runs/about-1.solve.txt` |
| the puzzle's query | — | as zebra2's: *The Norwegian drinks water in House-1; the Japanese owns zebra in House-5* | `runs/about-1.solve.txt` |
| the bijective twin | L-109: exit 0, query 2 a row for each of zebra2's bijections, `?f = bijective · ?r = color-loc` the first | **exit 0, no warning**; 5 rows, `color-loc`, `drink-loc`, `nation-loc`, `pet-loc`, `smoke-loc`, each `?f = bijective` | `runs/bijective.solve.txt`, `runs/bijective.in-slice-1.txt` |

**Beside zebra2's own:**

| | `examples/zebra2.ein` | **with L-109's forms** | the runs |
|---|---|---|---|
| H | 56 candidates, ``note: …: H, from `union`: 56 candidates`` | **the same 56**, the same note; sorted, the two lists are the same (`runs/hyps-diff.txt`, exit 0, no line) | `runs/zebra2.show-hyps.txt`, `runs/about-1.show-hyps.txt` |
| R | `states (k_s) 1`, `commitments (k_c) 261 982`, `∅ not in R — root owes 36 instances`, `universe 56 candidates, 21 dead alone` | **the same rows**, from `asked` to `work`, line for line | `runs/zebra2.solve.txt`, `runs/about-1.solve.txt` |
| the queries | one | two: the puzzle's and the slice's | |
| the facts about relations | 197 | 199: `(about-1 symmetric)` and `(in-slice-1 symmetric next-to)` more | `runs/zebra2.constraints.svg`, `runs/about-1.constraints.svg` |

**No oracle and no region.** H is 56 candidates, so the cube is 2^56 saturations: `ein oracle` refuses it, *56 root
candidates, over --region-limit 16: the region is 2^56 saturations and was not computed*, exit 2
(`runs/about-1.oracle.txt`), and with no cube there is no region to draw. The lattice is the one a describing run
visits, `runs/about-1.lattice.svg`; the whole knowledge base is `runs/about-1.kb.svg`.

## 5. On the built engine

**How it was taken.** The binary is `build/review-bin/ein-final-4aa950bc`, the release build of `4aa950bc`. Its
engine is that of `5e3d80e7`, the last engine commit, after S1f.21.24. It ran with the checkout's `stdlib/`, and
its `--version` is `runs/version.txt`. The item's first runs were on `7b72bd84`, before S1f.21.24; every figure of
§ 3 and § 4 was re-taken on this binary, and none moved. `tools/run.sh` re-takes every run into `runs/`, and
`tools/` and `runs/` are both ignored by git:

- **the program**, `zebra2-about-1.ein`: the read-out (`about-1.solve.txt`), `-p`, `--monotone`, `ein show hyps`,
  `ein saturate --dump`, the events and the summary (`about-1.events.jsonl`, 30 MB, and `about-1.summary.json`),
  the answer and the visited lattice as files (`about-1.dump-answer/`, `about-1.dump-lattice/`), the oracle's
  refusal, and five drawings, each beside its `.dot`: the knowledge base, the lattice, query 2's slice plain and
  folded, and the facts about relations;
- **the twin**, `zebra2-about-1-bijective.ein`: its read-out, `ein saturate --dump`, `ein show hyps` and query 2's
  slice (`bijective.*`), and `twin-diff.txt`, the two programs diffed: the header and that one line;
- **`examples/zebra2.ein` as it is**: the read-out, `ein show hyps`, `ein saturate --dump` and the facts about
  relations, to set beside them; `copy-diff.txt` diffs the program's body against it, exit 0, no line;
- **the six steps**, into `runs/steps/` (see [§ 2](#2-six-small-programs-one-step-each)), and the three rules drawn
  from step 6: `rule.about-1.svg`, `rule.imply1.svg`, `rule.symmetric.svg`;
- **the chain**, `the-check.svg`, drawn by hand from `tools/the-check.dot`.

**The programs.**

- [`zebra2-about-1.ein`](programs/zebra2-about-1.ein): `examples/zebra2.ein` whole, a header before it, and the three
  forms of [the milestone's L-109](../../../../findings.md) after it, as the row writes them:
  `(rule about-1 (?F) :match (and (?F ?r) (relation ?r)) :assert (in-slice-1 ?F ?r) :why "{?F} says something of {?r}")`,
  `(about-1 symmetric)` and `(query :goal (in-slice-1 ?f ?r))`.
- [`zebra2-about-1-bijective.ein`](programs/zebra2-about-1-bijective.ein): the same with `(about-1 bijective)`, the
  run L-109 set beside it, which loaded then.
- `step-1-…` to `step-6-…`: the ladder of [§ 2](#2-six-small-programs-one-step-each), none a corpus file.

**Commands to see it yourself**, from the repository's root, with `E=ein.rs/target/release/ein` and
`I=plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules/review/part-1/82-a-slice-by-a-rule-with-symmetric`:

```sh
$E solve $I/programs/zebra2-about-1.ein 2>&1 | tail -6           # the slice's row, then the one warning
$E solve $I/programs/zebra2-about-1-bijective.ein 2>&1 >/dev/null # the twin: nothing on stderr
$E saturate $I/programs/zebra2-about-1.ein --dump 2>/dev/null | grep -E 'imply1 |in-slice-1 symmetric'
$E solve $I/programs/step-6-a-run-derives-one.ein 2>&1 >/dev/null # both lines: the load's and the run's
$E render kb $I/programs/zebra2-about-1.ein --query 2 | dot -Tsvg > /tmp/82-slice.svg   # the slice drawn
```
