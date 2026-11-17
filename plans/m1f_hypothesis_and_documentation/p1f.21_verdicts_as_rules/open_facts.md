# P1f.21 — `(ein.kill)`, open facts, and the search as rules: the user's notes of 2026-11-18, worked out

**Phase:** [P1f.21](README.md). Written on the user's word (*"update p1f.21"*). Nothing here is built, and no
stage is run: the phase is not started without the user's word. [S1f.21.4](README.md#stages) takes what is below
onto real programs and puts its questions. The runs are the gate build of the main tree on 2026-11-18; the
programs they read do not touch what S1f.22.6 changed that day.

## 1. The notes, verbatim

> meanwhile - update p1f.21: (false) controls not search but saturation, right? if we have dead node then
> somebody has to kill it, maybe (ein.kill) or (ein.die) or (ein.saturation.kill) ? Also - instead of emitting
> (open ?R) emit (open (?R ?a ?b)) - the actual hypothesis. There is no problem with kernel read them, count and
> use. Then program with obligations also would have all hypotheses in the KB also (but under open). What kind of
> rules then could work with open facts? Can we completely replace/reimplement search with rules? But they have
> to be removed from KB during discharging. How removing of open facts can look like? Can it be flexible or
> transactional move from open into global true context or into not context?

They continue [V-5](findings.md#v-5--the-users-notes-from-p1f22-questions-as-facts-the-two-negation-rules-and-the-names):
its direction of 2026-11-17 (the state is dead only where a rule infers the kernel's fact from a global `(false)`)
and its note on `(open q)` as a question.

## 2. `(false)` acts on a saturation; the search reads the verdict

**Right, with one precision.** Everything a held `(false)`, or a fact beside its stored negation, does today
happens inside the saturation of one node:

| what it does | where |
|---|---|
| the agenda fires first what leads to it | `saturator::AGENDA` ([Q-M1f.313](../../../docs/decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)) |
| a route's node, on a monotone program, stops saturating at the first contradiction | `Saturator::stop_at_first_contradiction`; [`region.rs`](../../../ein.rs/crates/ein-infer/src/region.rs)' `Axes::stopping_at_first_contradiction` |
| the node gets the bit `DEAD` | `contradiction::detect`, read once a node in `node_in_region` ([`design.md` § 1.2](design.md#12-alive-and-dead)) |

The search never reads `(false)`; it reads the bit. A dead node is not in $R$, and on a monotone program every
node above it is dead without being saturated (licence L1). So `(false)` controls a saturation, and the
**verdict** on the state controls the search. The fact the kernel reads should therefore name what happens to
the state, not to the search.

## 3. Who kills a dead node: `(ein.kill)`

V-5's direction had the kernel read `(ein.contradiction)`, derived by a rule from a global `(false)`. The note
makes it a verb. The three names, beside V-5's:

| | in a rule's `:assert` | what it names |
|---|---|---|
| `(ein.kill)` | *kill this state* | the act, by the rule that concludes it |
| `(ein.die)` | *this state dies* | the same act told of the state; who acts is not said |
| `(ein.saturation.kill)` | *kill the saturation* | the process; but what dies is the state: its saturation stops **and** it is dead, and the second is what the search reads. A longer path would be `ein.state.kill` |
| `(ein.contradiction)` | *this state is contradictory* | a proposition, which `(false)` already is |

**Leaning: `(ein.kill)`.** A conclusion is what a rule asks of the kernel, and the imperative says so.

What follows, for S1f.21.4 to show on programs:

- **`(false)` becomes an ordinary fact**, a proposition, held or under an arrow (`(think Ann (false))`), which
  nothing in the kernel reads. The objection of P1f.22's
  [`contexts.md` § 12](../p1f.22_nested_facts/contexts.md#12-the-kernels-surface-false-and-open-control-the-search)
  to a control word under an arrow goes: `ein.kill` is read only held, and refused under an arrow as `open` is
  today.
- **Who states `(false)` ⟹ `(ein.kill)`.** If it is a module a program imports, every program that writes
  `(false)` and does not import it stops refuting anything: `(false)` is written 658 times in 278 programs (the
  count in that § 12). So it is in a module every program gets with no import line, one `ein saturate --dump`
  shows like any rule. Whether a program may leave it out or replace it (*"or from global (whatever)"*) is a
  question.
- **The kernel's own contradiction**, a fact beside its negation, becomes
  [`design.md` A.1](design.md#a1-the-contradiction-rule-set-contrainc)'s rule set (one rule a count of arguments)
  concluding `(false)`, in the same module. [V-1](findings.md#v-1--forbid-reads-rules-and-the-kernels-contradiction-is-none)
  is settled first: with that set written as rules, *forbid* refuses seven corpus programs.
- **The surfaces' words**: the read-out's *root's own fixpoint derives (false)* (V-2), `:expect (false)`, the
  events' `kind: dead`.

## 4. `(open F)`: the hypothesis itself

The note: in place of `(open ?R)`, an obligation emits the fact that would pay it, `(open (?R ?a ?b))`, and a
program with obligations then holds all its hypotheses, under `open`.

**What the runs show.** `examples/lattice/05_owed_seats_bounded.ein`. Root owes 4 debts (`ein solve … --events`,
its `owe` lines: *seats owes Ann a Chair*, *… Bob a Chair*, *… C1 a preimage in Guest*, *… C2 a preimage in
Guest*). $H$ is 4 candidates (`ein show hyps`: `(seats Ann C1)`, `(seats Ann C2)`, `(seats Bob C1)`,
`(seats Bob C2)`), the union of the debts' payments, and each candidate pays two debts: `(seats Ann C1)` pays Ann's
and C1's. `examples/zebra2-obligations.ein`: root owes 36 debts (72 `owe` lines before the first `enter`, each of
root's twice) and $H$ is 56 candidates.

So the open facts are $H$, as the note says, and they are not the debts:

- **A debt is a disjunction of open facts**, *at least one of* `(seats Ann C1)`, `(seats Ann C2)`. A set of
  `(open F)` says what may be supposed, and not what must be paid.
- **The counts differ**: 4 and 4 on the first program by chance, 56 and 36 at the root of the second. At the
  state `{(seats Ann C1)}`, run as the program with that fact appended (`ein solve seats1.ein --no-search`:
  *root owes 2 instances*, Bob's and C2's), three candidates are neither held nor denied, `(seats Ann C2)`,
  `(seats Bob C1)` and `(seats Bob C2)`. `std.algebra`'s `functional` concludes `(false)` and no negation, so
  nothing denies `(seats Ann C2)`, though Ann's debt is paid.

**What keeps both: the debt in the fact.** `(open F J)`, with F a candidate and J the debt it would pay: the
instance the kernel computes already (the rule, its activator's arguments, its bindings;
[`design.md` § 4.1](design.md#41-what-the-justification-is)), written as a nested fact, for instance
`(open (seats Ann C1) (total-owed seats is-a Ann))`. That is one fact per candidate and debt, 8 at seats' root.
Then:

- the debts S owes are the J of S's open facts that S has not paid, and the kernel counts them as it does today;
- a hypothesis of an hrule or of the blind rung pays no debt: `(open F)` alone, or J the hrule's instance. The
  ladder's three rungs then emit one form, and an hrule is a rule that concludes `(open F)`.

## 5. The move: open, held, not

P1f.22 ruled that a negation is a context (Q-M1f.394). With `(open F)`, a candidate has three places, and a
state puts it in exactly one, or in two where it holds F beside `(not F)` and is killed:

| F is | S holds | how it got there |
|---|---|---|
| open | `(open F …)`: neither F nor `(not F)` | the generator, at root |
| held | F | a commitment (the search's move), or a rule derived it |
| denied | `(not F)` | a rule derived it. The search never commits a negation: a commitment is a set of candidates held, and a candidate it leaves out stays open, for the states above it to take |

So **the move is the definition read on each state**: `(open F J)` is in S where F is a candidate, S holds
neither F nor `(not F)`, and S has not paid J. That answers the note's three words:

- **Transactional: yes, per node.** A child's open facts are its parent's minus what the child decides, taken in
  one step once the child's held facts reach their fixpoint, as its debts are counted today. A move is never
  made in the middle of a saturation. A rule that fired on `(open F)` before F was decided would leave its
  conclusion held after the move: [`design.md` § 2.3](design.md#23-owed-and-discharged), the debt that outlived
  its payment in 15 of 16 nodes.
- **Into held or into not: both**, by whatever decides F in the child: the commitment or a rule for held, only a
  rule for not. A third way out is J paid by another candidate: `(open F J)` goes, and F stays open for its other
  debts.
- **Flexible: in what decides, not in the move.** Any rule that derives F or `(not F)` decides it, and that is the
  program's. The move is the kernel's, and no rule moves an open fact without deciding it. A rule that dropped a
  candidate unproven would take states out of the lattice and change $R$: the search would skip states that are
  alive and discharged.

**Where it lives.** The knowledge base never retracts
([`kb.rs`](../../../ein.rs/crates/ein-core/src/kb.rs): *"push_fact appends, nothing retracts"*), and a fork
inherits its parent's facts. So the open facts are
[Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact)'s
option (d): a part of the node that no fork inherits, written by the pass after the fixpoint. The held facts stay
monotone in the commitment, which every licence is stated on. The note adds to (d) that rules read them (§ 6).

## 6. What rules can do with open facts

A rule that reads an open fact runs after the held fixpoint, in a stratum of its own, and concludes only into
that stratum or to the kernel: another open fact or a kernel verb, **never a held fact**. A held fact concluded
from an open one would stay held in the states above, where the open fact is gone; that is the stale debt again.

| | the rule | from | concludes | what does it today |
|---|---|---|---|---|
| 1 | **propose** | held facts | `(open F)` | an hrule; the obligations rung, read off a guard |
| 2 | **hopeless** | a debt that is unpaid and none of whose candidates is open | a verdict like `(ein.kill)`: no state above pays it, so none is in $R$, though the state is alive | nothing |
| 3 | **forced** | a debt with one open candidate | the next split is that candidate, with one child | the tree route, its alternatives one |
| 4 | **order** | the open debts and candidates | which to split on: fewest candidates first | the tree route, in rule order |
| 5 | **ask** | a question | a further question; an answer, only where asked | the calculator's requests; V-5's point 1 and [`contexts.md` § 13](../p1f.22_nested_facts/contexts.md#13-questions-as-facts-what-rules-can-do-with-open-q) |
| 6 | **report** | a state of $R$ | which candidates stay open in it | the read-out, in part |

Rows 2 and 3 read a count, an absence over open facts, so inside their stratum they are boundary rules, read on its
fixpoint. Row 2 is sound on a monotone program: a denial and a held fact persist upward, so a debt whose every
candidate is decided and none held is paid by no state above. Row 4 changes no $R$, only the cost
(`ein-infer/tests/explore.rs`' `every_mode_is_the_oracle` holds every order to the oracle). Rows 2 and 3 change
none where they are right; the oracle holds them as it holds `(false)`.

## 7. The search as rules

**Every decision of a search can be a rule; the split cannot.** A search does four things:

1. it proposes candidates;
2. it chooses one;
3. it splits the state into the states that take each choice (here the children holding each candidate of the
   debt chosen);
4. it judges each state: dead, owing, in $R$.

Proposing, choosing and judging are the rules of § 3 and § 6. Splitting is not: a rule adds to the one state it
runs in, and a split needs states that disagree, one holding F and one not. There are two ways to have it anyway:

- **(A) The split stays the kernel's one primitive.** A rule concludes what to split on (§ 6, row 4); the kernel
  forks, saturates each child, moves its open facts (§ 5) and reads its verdicts. The search is then rules plus
  one verb. The routes keep what makes them cheap: the copy-on-write fork, a child resuming its parent's
  saturation, the licences.
- **(B) Each state is a context.** P1f.22's nested facts make a state an object, `(in W F)` with W a fact naming
  the commitment. Every rule of the program is written again under `(in ?w …)`, which a macro can do
  ([`contexts.md` § 4](../p1f.22_nested_facts/contexts.md#4-macros-as-the-wrapping)), and a rule makes the child
  worlds from the open facts. The whole lattice is then one saturation, and $R$ is the worlds that hold no
  `ein.kill` and owe nothing. This is an assumption-based truth maintenance system: the oracle written in rules.
  It costs what the kernel saves:
  - every world derives its facts again, since no fork shares them;
  - a world's absences can be read only after that world's saturation, which one saturation of all worlds at once
    does not give without strata per world;
  - it is the whole cube, $2^{|H|}$.

**Leaning: (A) for the engine, and (B) as the reasoning pages' statement of what $R$ is.** (B) defines $R$ with
no search at all, and the oracle is its brute-force reading. Neither is built here.

## 8. The reserved names that are left, and what the kernel reads

The user's third note (verbatim in [the README](README.md#the-third-instruction)): *false* no longer the kernel's,
*rule* stays, *relation* already a convention, *neq* needed and *eq* hardly, *not* a convention once the
contradiction is a rule; *what else? what is read by kernel from KB to make decisions about saturation or
search?*

**The census.**
- The names: `ein-core/src/terms.rs`' `Kernel`, the names interned before any program.
- Who reads each: `grep -rlE 'kernel(\(\))?\.NAME\b'` over the crates' `src/`.
- The counts in programs: `grep -rhoE '(\(|:)NAME( |\))' --include='*.ein' stdlib examples tests`, with the files
  beside them.

### 8.1 What the kernel reads, and what it decides with it

**From a state's facts:**

| it reads | it decides | where |
|---|---|---|
| `(false)` held | the node stops saturating and is dead | `contradiction.rs`, `saturator.rs` |
| a fact beside its `(not F)` | the same | `contradiction.rs` |
| a rule whose conclusion is `(false)`, a `(not …)`, or a fact whose negation some rule concludes | the agenda's first half; *forbid*; the static check, whose verdict grants the routes' licences | `saturator.rs`, `forbid.rs`, `monotone.rs`, `naf_deps.rs`, `structure.rs` |
| `(__symmetric__ R)` | the extent of R closed under swapping, in the saturation, with no rule firing a mirror | `saturator.rs` |
| `(__closed__ R)` | no candidate of R in $H$ (rungs 2 and 3), so a commitment cannot extend R and *forbid* lets a refutation rest on an `absent` over it; the mechanisms' conflicts | `hypgen.rs`, `mechanisms.rs` |
| a fact headed by a rule's name, `(symmetric R)`, `(total-owed seats is-a)` | the rule's instances: the **activator** binds its parameters. No name is reserved for it, and a module's rules are switched on so | `compile.rs` |
| an obligation's guard, on the fixpoint | what the state owes, and rung 2's candidates | `obligations.rs`, `oblgen.rs` |
| the program's names | `neq`'s free side; the blind rung's candidate objects | `predicates.rs`, `hypgen.rs` |
| `is-a`, by its name | nothing: one line of the read-out, the types the blind rung bound as objects; **dropped** on the user's ruling of [V-6](findings.md#v-6--the-read-out-reads-is-a-by-its-name) (a) | `types_as_objects.rs` |

**From the program, not from a state:**
- the rules' plans (the third row);
- the hrules and `(solve :hrules …)`;
- the declarations: the loader holds every fact to its declaration's count of arguments, and the blind rung
  guesses facts only of a relation with labels, never with a label as an object;
- `:hypothesis-relations`, `:no-hypothesis` and `:no-search`;
- `(config …)`, which changes no answer.

### 8.2 Each name, under this phase's direction

| name | today | under the direction | counted |
|---|---|---|---|
| `false` | the kernel's: a held one kills | a proposition; `(false)` ⟹ `(ein.kill)` a rule every program gets (§ 3) | 658 in 278 programs |
| `not` | the kernel's: beside its fact it kills; the agenda, *forbid* and the static check read it; a candidate root denies is not in $H$ ([`design.md` § 3.4](design.md#34-the-extent-an-owed-instance-a-hypothesis-and-openr)) | a relation of one argument whose meaning is two rules: the contradiction ([A.1](design.md#a1-the-contradiction-rule-set-contrainc)) and the elimination of a double negation (Q-M1f.391). Whatever read it for a decision then reads the rules that lead to `ein.kill`, which is V-1's fix. Leaving a denied candidate out of $H$, and § 5's move into *not*, become economies: committing a denied candidate gives a dead child | |
| `relation` | a position word: the declaration, the fact `(relation R A B)` rules match, the count of arguments, the `:why` | a convention for what rules read (Q-M1f.381: any head is a relation). The declaration keeps three uses: the count the loader holds a fact to, the `:why` a fact is read in, and the blind rung's guessing (which relations, which objects) until proposing is rules (§ 6, row 1) | |
| `eq` | computed; binds a free side | **stays as it is**, on the user's word (*"Ok, so eq still required, leave as it is"*). Substitution at load would say most uses with one variable, but not `std.math`'s two `(neq ?v z) (eq ?v ?x)` (`decimal-arith.ein`'s `operand-undeclared`, binary's twin): *?x is a name, and not a fact*. `neq`'s free side ranges over names, and substituting `?x` for `?v` loses that. On nested facts both compare as they should (§ 8.4) | 146 in 40 files, 30 of them under `examples/ein-bugs/wrong-answers/` |
| `neq` | computed; a free side ranges over the program's names | stays: no rule says *differ* without it | 345 in 119 files |
| `and`, `or`, `absent` | the `:match`'s syntax | stay. `absent` is the one premise the kernel orders (the boundary). `or` is one rule a disjunct, which the loader writes | |
| `open` | the verdict, only as an obligation's whole `:assert` | the open facts the kernel writes and moves, `(open F J)` (§ 4, § 5), named beside `ein.kill` (§ 3) | |
| `__closed__` | read by $H$ and the mechanisms' check | derived, once proposing is rules: a relation no rule proposes a fact of is closed, and *forbid* reads that off the rules as it reads the rest. Or the kernel's, `ein.closed` | 685 in 91 files |
| `__symmetric__` | the saturation's native mirror | an economy of the kernel's (`ein.symmetric`), or `std.algebra`'s `symmetric` rule alone | 12 in 5 files |
| `hrule` | a declarator, rung 1 | a rule that concludes `(open F)` (§ 4, § 6 row 1), still activated by `(solve :hrules …)` | 141 in 115 files |
| `rule`, `macro`, `import` | declarators | stay: the program's structure | |
| `query`, `solve`, `config`, `trace` | declarators: questions about $R$, flags, a log | stay: none is a fact of a state. Of `solve`'s lists, `:hypothesis-relations` and `:no-hypothesis` filter what is proposed, and become rules if proposing does | |
| `model`, `region`, `merged`, `owing` | position words of a claim | stay | |
| `<forced-positive>` and its two siblings, the `#` names | the engine's own records | stay the engine's | |

`written`, the loader's until S1f.22.6, is erased by it (Q-M1f.404).

### 8.3 What is left

**The kernel then reads:**
- one verb in a saturation, `ein.kill`;
- `absent`'s boundary;
- `eq` and `neq`;
- a rule's activator;
- its open facts, which it writes and moves (§ 5);
- the split in the search, which it makes (§ 7 (A)).

**The rest becomes a theory's**: `false`, `not`, `relation` as rules read it, `hrule`, `__closed__`, and
`is-a`, which already is one but for the read-out's row (V-6). The grammar's reserved words shrink to the
declarators and the connectives. What a declarator may not bind becomes the `ein.` prefix, `eq`, `neq` and the
connectives.

### 8.4 `eq` and `neq` on nested facts

The user: *"Do eq/neq correctly compare nested facts? they should I guess."* **They do**, by the object: a fact's
identity is its tuple (Q-M1f.380), interned once, so two facts with one tuple are one object wherever they came
from. `corpus/regressions.toml`'s `form-in-guard` holds the simple case (`examples/ein-bugs/wrong-answers/form-in-guard.ein`).
The run below adds depth, a negation, a derived fact against a written one, a bound free side, a free `neq` and a
conclusion. `eqn.ein`:

```lisp
(relation p X :why "p {?1}")
(relation q X :why "q {?1}")
(relation r X :why "r {?1}")
(relation f X :why "f {?1}")
(relation g X :why "g {?1}")
(relation hit C X Y :why "case {?1}: {?2} {?3}")
(p (f a)) (p (f b)) (p (g (f a))) (p a) (p (not (f a)))
(q (f a)) (q (g (f a))) (q (not (f a)))
(rule mk () :match (p a) :assert (r (f a)) :why "a rule builds (f a)")
(rule c1 () :match (and (p ?x) (q ?y) (eq ?x ?y)) :assert (hit c1-eq ?x ?y) :why "eq")
(rule c2 () :match (and (p ?x) (q ?y) (neq ?x ?y)) :assert (hit c2-neq ?x ?y) :why "neq")
(rule c3 () :match (and (r ?x) (q ?y) (eq ?x ?y)) :assert (hit c3-derived-eq ?x ?y) :why "derived")
(rule c4 () :match (and (p a) (eq ?z (g (f a)))) :assert (hit c4-bind ?z ?z) :why "bind")
(rule c6 () :match (and (p (g ?x)) (neq ?v ?x)) :assert (hit c6-free-neq ?x ?v) :why "free neq")
(rule c7 () :match (and (p ?x) (q ?y) (eq ?x ?y)) :assert (eq ?x ?y) :why "check holds")
(solve :no-search)
```

`ein saturate --dump eqn.ein`, the `hit` facts:

| case | what it shows | result |
|---|---|---|
| c1 | `eq`, both sides bound: depth 1, depth 2, a negation | 3 hits, the three equal pairs: `(f a)`, `(g (f a))`, `(not (f a))` |
| c2 | `neq`, both sides bound | 12 hits: the 15 pairs less those 3 |
| c3 | a fact a rule built against the same one written | `(hit c3-derived-eq (f a) (f a))` |
| c4 | a free side bound to a written fact | `(hit c4-bind (g (f a)) (g (f a)))` |
| c6 | `neq` with a free side, the other `(f a)` | 18 hits, each an **atom** of the program: `a`, `b`, `f`, `g`, `p`, the labels `X`, `C`, `Y`, the cases' names… and no fact. The free side ranges over the program's names, as the reference says (*"`neq` over the program's names"*), so it never binds a nested fact. `std.math`'s test of a name rests on this |
| c7 | `eq` as a conclusion, true | no `(false)`; `ein solve`: ∅ in $R$ |

Two more:
- **`eqn8.ein`**, `(p (f a)) (q (f b))` with `:assert (eq ?x ?y)` over `(p ?x) (q ?y)`: *every node is dead*, `unsat
  core (p (f a))`, `(q (f b))`, `refuted by c8`. A false `eq` concluded on two nested facts kills.
- **`(eq ?x (f ?w))`**, a side that is a fact with a variable inside: refused at load. *"a guard compares two
  values, each a variable, a name, or a fact that holds no variable"*. `eq` does not take a fact apart; a premise
  `(p (f ?w))` does.

## 9. What S1f.21.4 answers

On real corpus programs, at least `examples/lattice/05_owed_seats_bounded.ein`, `examples/zebra2-obligations.ein`
and a program with an hrule, with their answers computed (emulated in Ein with a stand-in relation where the
parse refuses `open` or `ein.`, as [`contexts.md` § 13](../p1f.22_nested_facts/contexts.md#13-questions-as-facts-what-rules-can-do-with-open-q)
did). Each is put as a question with a default:

1. The verdict's name, and where `(false)` ⟹ `(ein.kill)` and the contradiction rules live (§ 3); with V-1.
2. The form of an open fact, `(open F)` or `(open F J)`, and J for each rung (§ 4).
3. The move and the stratum: what each node computes, in what order, and what may read it (§ 5, § 6).
4. Which of § 6's rules the standard library states and what the routes read of them, each shown not to change
   $R$, or to be a verdict the oracle holds.
5. The search as rules: (A), (B), or (A) with (B) on the reasoning pages (§ 7).
6. The kernel's surface (§ 8): the census of what it reads re-taken; for each name of § 8.2, its place under the
   rulings (the kernel's, a theory's, syntax), with what reads it moved and the corpus's answers unchanged; `eq`'s
   test of a name; the module every program gets.
7. What becomes of [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact),
   whose four options this design stands beside, and of
   [Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open), the name.
