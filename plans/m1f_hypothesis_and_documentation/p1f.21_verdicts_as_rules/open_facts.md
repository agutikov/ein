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
  `contexts.md` § 12, [§ 10.7](#107-the-kernels-surface-false-and-open-control-the-search) here,
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
| 5 | **ask** | a question | a further question; an answer, only where asked | the calculator's requests; V-5's point 1 and [§ 10.8](#108-questions-as-facts-what-rules-can-do-with-open-q) |
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
  ([§ 10.3](#103-macros-as-the-wrapping)), and a rule makes the child
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
parse refuses `open` or `ein.`, as P1f.22's `contexts.md` § 13
did, [§ 10.8](#108-questions-as-facts-what-rules-can-do-with-open-q)). Each is put as a question with a default:

1. The verdict's name, and where `(false)` ⟹ `(ein.kill)` and the contradiction rules live (§ 3); with V-1.
2. The form of an open fact, `(open F)` or `(open F J)`, and J for each rung (§ 4).
3. The move and the stratum: what each node computes, in what order, and what may read it (§ 5, § 6).
4. Which of § 6's rules the standard library states and what the routes read of them, each shown not to change
   $R$, or to be a verdict the oracle holds.
5. The search as rules: (A), (B), or (A) with (B) on the reasoning pages (§ 7).
6. The kernel's surface (§ 8): the census of what it reads re-taken; for each name of § 8.2, its place under the
   rulings (the kernel's, a theory's, syntax), with what reads it moved and the corpus's answers unchanged; `eq`'s
   test of a name; the module every program gets; with [V-7](findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count),
   a relation that only an activator names.
7. What becomes of [Q-M1f.390](../open_questions.md#q-m1f390--what-an-obligation-emits-the-verdict-with-its-justification-or-as-a-fact),
   whose four options this design stands beside, and of
   [Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open), the name.

## 10. What P1f.22 worked out that this phase reads

Copied at P1f.22's close (2026-11-18) from that phase's page on the user's notes, `contexts.md`, which went with
its folder (`git log --diff-filter=D -- plans/m1f_hypothesis_and_documentation/p1f.22_nested_facts`). The sections
below are that page's § 1 and § 10 (the user's words), § 3 to § 6 and § 11 to § 13, as written on 2026-11-17, every
run on the gate build of that day; a note under a section says where a later stage of P1f.22 changed what it says of
the engine. What this phase reads in them: the user's words (§ 10.1); a contradiction in a context, and
*everybody's thoughts are false* (§ 10.4, § 10.6); the elimination of a double negation as a rule (§ 10.5); the
names of `(false)` and `(open)` (§ 10.7); and questions as facts (§ 10.8). What the page worked out of the drawings,
the words and the reserved words in the three perspectives is decisions now (Q-M1f.394, .395, .397, .398).

### 10.1 The user's notes, verbatim

The first, 2026-11-17, on the six questions of P1f.22's survey:

> docs/ein/reasoning/ - canonical graph representation has to somehow show that nexted fact is in global context
> or/and is in local/fact context. Now it is not explicitely presented. Maybe consider approach similar to `absent`
> drawing in rules. The implementation (ein.rs) renders nested facts as atoms with names made from entire tuple. It
> is not valid because if fact present in multiple contexts it will be drawn multiple times, while canonical
> (semi-Levi-bipartite) graph has every object drawn once. I want to have: all rules are global, so no rule binds
> nested-only fact as global fact, but can bind it as nested fact. So there could be separate global rules for this
> fact and another rules for same facts in the context - the straightforward way to write pattern as is wrapping
> every fact with context, and maybe macros could be useful here to simplify this wrappings? Q-M1f.391 Rule for
> (not (not g)) -> g is a simple rule, essential negation elimination, already trivial in ein. That's why I also
> raised a question about (and g (not g)) -> (false) could be a rule in p1f.21; Another question about rules that
> infer (false) under context - does it reach global context? If not then how is it stored or reported or affect
> further inference, maybe just store it in context and that's it, like (think Ann (false))? Then it has to be
> bindable to checks - for example to infer false if everybody's thought are false (just an example). In the
> docs/ein/reasoning all those reserved words are conceptual mathematical objects. But in docs/ein/user and
> docs/ein/ein.rs they are surface between ein reasoning kernel and pure ein theory.  Q-M1f.392 - I don't
> udnerstant - write with details into separate file in plans. Q-M1f.392 (a) for now, I don't yet know why we would
> need to change it. The only strictly required for now is to know fact's arity, for all other you can use
> variables. Q-M1f.394 explain with examples in separate file. Q-M1f.396 sections.

The second, the same day, on that page's readings of the first (what each ruled is the ledger's Q-M1f.391 to
.399):

> Q-M1f.393 yes (a); Q-M1f.391 OK; Q-M1f.398 lets try (a) and see how it looks; Q-M1f.395 - yes, 3-arity fact can
> an arrow only when nothing points on it; Q-M1f.392 I still do not understand, show an issue, how to reproduce it,
> explain for dummies. Q-M1f.394 held = in the global context; in a context = in a fact's (local) context;
> Question 2: `not` obviously holds something like logically mirrored world where all true facts are false and all
> false are true, right? it looks quite consistent with context idea here, isn't it? And it already meets some
> criterias, e.g. denied facts not bind to rules without context, right? E.g. this is a way to define tri-state,
> multi-state logic and other kinds of logic. This also leads me to the surface between ein theory and reasoning
> kernel - things like (false) and (open) are effectively controlling the search. Maybe word `false` is too heavey
> loaded with classical binary logic and not expose it's actual function in ein? Maybe (abort) or (dead) is more
> clear? Or some common search control interface accassible from ein? Like (ein.search.state.refute)? Also for
> (open) (in p1f.21) if we will emit ordinary facts (open (lives-in Alice House-1)) they naturally are hypotheses,
> what rules can do with them? And, (open) context contains "open" questions, like "Does Alice live in House-1?" -
> so there is a menaning of those facst, so we can write rules to infer something from them directly, right? What,
> for instance? Question 3 — value, term, object, fact - is't simple - they are in different contexts: object - in
> core ein theory explained on graphs, we have to say what are all those nodes - they are objects; value - wording
> "(knows Carol Dan) inside the claims fact is a value, not a fact of the knowledge base." is not correct. Use word
> `value` only when talk about variables in rules, it's obvious that {var 1-n values} and here value is an object,
> but the object is a value only when binded to variable; `relational node` - don't use it, it is too SQL-ish;
> graph node - ok; In docs/ein/ein.rs - use whatever naming convention you want, just make software documentation
> good; term - is mostly about syntax; I found only one place where now "ground term" exists, maybe survey showed
> false positive here.

### 10.2 Rules are global: what a premise reads, and what a wrapped premise reads

**A premise is matched among held facts and nowhere else; a pattern under an arrow of a premise is matched against
the fact that stands there, held or not.** So a rule written over `(= ?x ?y)` is a rule of the global context, and
the same rule for a context is the same patterns, each wrapped. `x6.ein`:

```lisp
(= a b)
(think Ann (= c d))
(rule sym ()
  :match  (= ?x ?y)
  :assert (= ?y ?x)
  :why    "{?x} is {?y}")
(rule sym-in ()
  :match  (think ?c (= ?x ?y))
  :assert (think ?c (= ?y ?x))
  :why    "{?c} thinks {?x} is {?y}")
(query :goal (= ?x ?y))
(query :goal (think ?c (= ?x ?y)))
```

`ein solve x6.ein`:

```text
  query 1 of 2      (= ?x ?y)
                      ?x = a · ?y = b
                      ?x = b · ?y = a
  query 2 of 2      (think ?c (= ?x ?y))
                      ?c = Ann · ?x = c · ?y = d
                      ?c = Ann · ?x = d · ?y = c
```

`sym` fired on `(= a b)` and not on `(= c d)`, which only Ann thinks; `sym-in` fired on what Ann thinks and
concluded a fact Ann thinks, `(think Ann (= d c))`, and `(= d c)` is not held. The two rules do not see one
another's facts. This is the engine today, and P1f.22's analysis's Propositions 1 to 3 are this said in mathematics.

**The context can be a parameter**, so one rule serves every context relation that names it. `x4.ein`:

```lisp
(think Ann (= 5 (+ 2 2)))
(think Ann (not (= 5 (+ 2 2))))
(says Bob (likes Bob Carol))
(says Bob (not (likes Bob Carol)))
(rule ctx-contradiction (?C ?R)
  :match  (and (?C ?p (?R ?a ?b)) (?C ?p (not (?R ?a ?b))))
  :assert (?C ?p (false))
  :why    "{?p}: {?R} of {?a} and {?b}, and not")
(ctx-contradiction think =)
(ctx-contradiction says likes)
(query :goal (think ?p (false)))
(query :goal (says ?p (false)))
```

`ein solve x4.ein`: alive, one state, `?p = Ann` for the first query and `?p = Bob` for the second. What is fixed
in the text is the count of arguments, of the context (`?C` of two) and of the fact in it (`?R` of two); the
relations are parameters, each bound by an activator. That is the note's *"the only strictly required … is to
know fact's arity"*.

### 10.3 Macros as the wrapping

**A macro wraps.** A macro's parameter may stand for a whole pattern, and its call is replaced by its body in a
`:match`, an `:assert` and a `:goal`. `m2.ein`:

```lisp
(macro in (?c ?F) (think ?c ?F) :why "{?c} thinks {?F}")
(think Ann (= a b))
(think Ann (not (= a b)))
(rule contradiction-in ()
  :match  (and (in ?c (= ?x ?y)) (in ?c (not (= ?x ?y))))
  :assert (in ?c (false))
  :why    "{?c} thinks {?x} is {?y} and not")
(query :goal (in ?c (false)))
```

`ein solve m2.ein`: `?c = Ann`, `as :expect (model (think Ann (false)))`. With the macro the rule reads as the
global rule does, each fact under `in ?c`. A macro for one fixed context is the same with one parameter, `(macro
ann (?F) (think Ann ?F) …)` (`m1.ein`, a transitivity inside what Ann thinks: `(ann (= a ?w))` answers `b` and
`c`). And a macro's call expands where it stands **inside** a pattern too: with `(macro eqp (?x ?y) (= ?x ?y) …)`,
the premise `(think ?p (eqp ?x ?y))` matches `(think Ann (= a b))` (`m3.ein`).

**A macro does not write a rule.** Its body is a premise; `(macro lifted (?c) (rule …) …)` is refused at the parse
(`m4.ein`: ``unexpected `rule` ``). So there is no form today that takes a rule written for the global context and
gives its twin for a context: **each rule is written twice**, once bare and once wrapped, or once with the context
a parameter as in § 10.2. The gap, if it is one:

| a program wants | today | what would close it |
|---|---|---|
| a rule for a context, written once | the rule with every fact under `(in ?c …)`, by a macro or by hand | nothing |
| one rule for several context relations | the context relation a parameter, an activator each (§ 10.2) | nothing |
| **a rule set written once, applied both globally and inside a context** | written twice | a form that lifts a rule, e.g. a rule-level macro; nothing has asked for it, and it is not proposed here |

### 10.4 `(false)` in a context

**It does not reach the global context.** `(false)` is a fact, the one of the relation `false` with no argument,
and it may stand under an arrow (Q-M1f.123). `(think Ann (false))` is then a held fact of `think` that points at
the node $\bot$; $\bot$ itself is not held, and a knowledge base is dead exactly when it *holds* $\bot$ or a fact
with its negation. `x1.ein`, a rule that infers it:

```lisp
(think Ann (= 5 (+ 2 2)))
(think Ann (not (= 5 (+ 2 2))))
(think Bob (= 5 (+ 2 2)))
(rule ctx-contradiction ()
  :match  (and (think ?p (= ?a ?b)) (think ?p (not (= ?a ?b))))
  :assert (think ?p (false))
  :why    "{?p} thinks {?a} is {?b} and thinks it is not")
(query :goal (think ?p (false)))
```

`ein solve x1.ein`: `states (k_s) 1`, `∅ in R — root is alive and owes nothing`, `?p = Ann`,
`as :expect (model (think Ann (false)))`. So:

- **stored**: as the fact `(think Ann (false))`, in the context and nowhere else, as the note proposes;
- **reported**: as any fact is, in the state, in a query's rows and in a claim; no row of the read-out treats it
  apart;
- **further inference**: it is read by the rules that match it, and by nothing else. The kernel reads $\bot$ only
  where it is held.

**It is bindable**, in a premise and under a quantifier. *Everybody's thoughts are false* is one `forall`.
`x3.ein`:

```lisp
(import std.macro :symbols (forall))
(person Ann)
(person Bob)
(think Ann (false))
(think Bob (false))
(rule all-confused ()
  :match  (forall ?p (person ?p) (think ?p (false)))
  :assert (false)
  :why    "everybody's thoughts are false")
```

`ein solve x3.ein`: `states (k_s) 0`, `unsat core (think Ann (false))`, `unsat core (think Bob (false))`,
`refuted by all-confused`. Without the line `(think Bob (false))` (`x3b.ein`): `states (k_s) 1`, root alive. So the
global `(false)` is reached from the contexts' by a rule that says so, and only so.

**What the kernel does not do in a context.** Globally, a fact held with its negation is dead with no rule
written. In a context nothing is: `(think Ann g)` beside `(think Ann (not g))` is alive until a rule such as
`ctx-contradiction` concludes `(think Ann (false))`. That is the same asymmetry P1f.21's question is about from
the other side: if $g \land \lnot g \Rightarrow \bot$ becomes a rule of Ein, its twin for a context is that rule
wrapped (§ 10.2), and the two read alike. A check, `(eq …)` or `(neq …)`, cannot be wrapped: it is computed, never
stored, and is refused under an arrow.

**A finding on the way** (P1f.22's N-15, now [F21's](../../followups/f21_non_monotone_programs_decided.md#met-again-by-p1f22-everybodys-thoughts-are-false-at-a-dead-root)). The same *everybody* written as two rules over `absent`, a rule
that derives `(some-sane)` where a person has no `(think ?p (false))` and a rule that concludes `(false)` where
`(some-sane)` is absent, answers **dead with Bob not confused** (`x2b.ein`), under `lattice monotone (the static
check)`. It is the open wrong answer `W8/fifo-gap-root` met again: two rules that wait on an `absent` at one
boundary, admitted in the agenda's order, the one that concludes `(false)` first. The `forall` form asks its
nested absence as one question and is right.

### 10.5 The negation of a negation, and its elimination as a rule

**Read of the note**: `(not (not g))` is to be an ordinary fact, and $\lnot\lnot g \Rightarrow g$ a rule a theory
states, as $g \land \lnot g \Rightarrow \bot$ would be in P1f.21. That is Q-M1f.391's option (c) with the
elimination moved out of the kernel into a rule. What is so today:

| | today |
|---|---|
| a rule's `(not ?x)` over a stored negation | stores `(not (not g))` (`e11.ein`); the kernel reads it as the negation of the fact `(not g)`: dead beside `(not g)`, alive beside `g` |
| a file writing `(not (not g))`, as a fact, a premise or a conclusion | refused at the parse: ``unexpected `not` `` (`d1.ein`, the rule below) |
| the state file of a state that holds one | does not load, for that reason |

So the rule is **not** writable today, though the engine already holds the fact it would read. `d1.ein`:

```lisp
(rule dne (?R)
  :match  (not (not (?R ?a ?b)))
  :assert (?R ?a ?b)
  :why    "not not")
(dne =)
```

``d1.ein:2:17: unexpected `not` ``. Under the reading above, P1f.22 would:

1. **admit `(not (not …))`** at any depth in the grammar, as a fact, a pattern and a conclusion; the knowledge-base
   page's shape of $\mathsf{not}$ loses its last clause, and Q-M1f.113's *"a fact of an ordinary relation, or its
   negation"* becomes *"a fact, or the negation of one"*;
2. change **nothing in the kernel**: a held $\lnot f$ beside a held $f$ is dead for every fact $f$, a negation
   among them, as it is now; `e11`, `f1` and `f2` answer as today, and `f2`'s state file loads;
3. leave **the elimination to a rule**, one for each count of arguments with the relation a parameter, as `dne`
   above. Where that rule lives (a module of the standard library a program imports, beside the contradiction
   rule) is P1f.21's to say, and is put there.

**One thing this reading leaves out, to know of.** A variable for a whole fact may be denied at the top of a
conclusion, `:assert (not ?f)`, and may not stand under `not` anywhere else. `x5.ein` and `x5b.ein`:

```lisp
(rule ctx-contradiction ()                     ; refused: unexpected `?f`
  :match  (and (think ?p ?f) (think ?p (not ?f)))
  :assert (think ?p (false))
  :why    "{?p} thinks {?f} and thinks not")

(rule flip ()                                  ; refused: unexpected `?f`
  :match  (and (liar ?p) (think ?p ?f))
  :assert (think ?p (not ?f))
  :why    "{?p} lies")
```

So the global rule *deny whatever a liar thinks*, which loads, has no wrapped twin: inside a context the denial
must spell the fact's relation and count, `(think ?p (not (?R ?a ?b)))`. That is consistent with *the arity is
required*, and with Q-M1f.393 (a) as P1f.22 read the user's note; it is said here because *wrap every fact with
the context* meets it. Admitting `(not ?x)` under an argument arrow would touch none of the analyses that Q-M1f.393's (b) touches (the
premise's and the conclusion's relation stay fixed), and is not proposed unless asked for.

**What holds now** (re-run at P1f.22's close, 2026-11-18). S1f.22.3 built this reading, Q-M1f.391 (c): the
grammar admits `(not (not …))` at any depth, as a fact, a pattern and a conclusion; the kernel is not changed; and the
state file of a state that holds one loads. So the table's second and third rows no longer hold, and `d1.ein` loads.
`examples/beliefs/06_double_negation.ein` writes the elimination as a rule, `eliminate`, one rule for facts of two
arguments with the relation a parameter, on for `=` by its activator, and its twin `eliminate-in` wrapped in
`(think ?p …)`; it declares `=`, for the reason [V-7](findings.md#v-7--a-rule-can-conclude-a-fact-of-a-relation-the-program-does-not-count)
gives. No rule of the elimination is in `stdlib/`: where it lives is this phase's (V-5, point 2). The rest of the
section holds as written: `x5.ein` is still refused, ``x5.ein:2:45: unexpected `?f`: a variable``.

### 10.6 `not` as a context

**Right: `not` is a context, and it already behaves as one.** The fact a negation denies is under the negation's
arrow and is not held, so no bare premise binds it; a rule about denied facts writes its patterns under `not`, as
a rule about what Ann thinks writes them under `think`. `n1.ein`:

```lisp
(not (= 5 (+ 2 2)))
(rule seen ()
  :match  (= ?a ?b)
  :assert (seen-held ?a)
  :why    "an equality is held")
(rule seen-denied ()
  :match  (not (= ?a ?b))
  :assert (seen-denied ?a)
  :why    "an equality is denied")
(query :goal (seen-held ?x))
(query :goal (seen-denied ?x))
```

`ein solve n1.ein`: `(seen-held ?x)` has no row; `(seen-denied ?x)` answers `?x = 5`.

**Not quite: it is no mirror of the global context.** A mirror would hold every fact the global context lacks,
and then a fact would be held or denied and nothing else. The `not` context holds exactly the facts that something
stated or concluded as denied, and grows the way the global one does, by what is written and what rules conclude.
A fact in neither is **open**. So there are two contexts that grow independently, and four things they can say of
one fact:

| | in the global context | in the `not` context |
|---|---|---|
| asserted | yes | no |
| negated | no | yes |
| open | no | no |
| contradicted | yes | yes |

The three-valued reading is already there (asserted, negated, open), and it comes from `not` *not* being the
complement. Where it is a mirror is one step further out: with the elimination of § 10.5 as a rule, the `not` context
of the `not` context is the global one again.

**What makes `not` different from `think` is one reading by the kernel**: a fact in both contexts makes the state
dead. That reading is the rule $g \land \lnot g \Rightarrow \bot$ that P1f.21 is about; once it is a rule a theory
states, the kernel reads **no** context, and `not` is a context a module gives its meaning to, as a program gives
`think` its.

**Other logics, as contexts.** A program adds a truth value by adding a context relation and the rules between
the contexts. `n2.ein`, four verdicts of one fact, the third value the program's own `maybe`:

```lisp
(import std.macro :symbols (unknown))
(= a b)
(not (= a c))
(maybe (= a d))
(pair a b) (pair a c) (pair a d) (pair a e)
(rule v-true    () :match (and (pair ?x ?y) (= ?x ?y))         :assert (verdict ?x ?y yes)   :why "held")
(rule v-false   () :match (and (pair ?x ?y) (not (= ?x ?y)))   :assert (verdict ?x ?y no)    :why "denied")
(rule v-maybe   () :match (and (pair ?x ?y) (maybe (= ?x ?y))) :assert (verdict ?x ?y maybe) :why "maybe")
(rule v-unknown () :match (and (pair ?x ?y) (unknown (= ?x ?y)) (absent (maybe (= ?x ?y))))
                   :assert (verdict ?x ?y unknown) :why "nothing said")
(query :goal (verdict a ?y ?v))
```

`ein solve n2.ein`: `?v = yes · ?y = b`, `?v = no · ?y = c`, `?v = maybe · ?y = d`, `?v = unknown · ?y = e`, under
`lattice monotone (the static check)`. What such a logic *means* (what follows from `maybe g` beside `not g`) is
the rules the program writes between the contexts; the kernel supplies it only for `not`.

### 10.7 The kernel's surface: `(false)` and `(open)` control the search

**The observation is right, and it is half of what `(false)` is.** It has two parts to play, and the name fits
neither well:

| where `(false)` stands | what it does | run |
|---|---|---|
| **held**, in the global context | the kernel's verdict *this state is impossible*: the node is dropped, its core recorded, the search goes on elsewhere. This is control | every dead state |
| **under an arrow**, in a context | a proposition, *what this context says is contradictory*: `(think Ann (false))`. It controls nothing, and rules read it | `x1.ein`, § 10.4 |

It is not a truth value in either. No fact of Ein *is false*: a denied fact is `(not g)`, and `(false)` is what
logic writes $\bot$, the absurd. So the worry is sound: a reader who knows two-valued logic reads `(false)` as
*the value false* and `(not g)` as *g is false*, and only the second is near the truth.

**Three ways to go**, none of them this phase's to build:

| | what | reads, held | reads, in a context | costs |
|---|---|---|---|---|
| (a) keep `false` | the pages say once that it is no truth value and what the kernel does with it (Q-M1f.397's surface) | `(false)` | `(think Ann (false))` | nothing |
| (b) a word for the proposition | `(contradiction)`, `(impossible)` or `(absurd)` | *this state holds a contradiction* | `(think Ann (contradiction))` | a rename |
| (c) a word for the search's act | `(dead)`, `(abort)`, `(refute)` | *drop this state* | `(think Ann (dead))`, `(think Ann (abort))`: a command where a proposition goes | a rename |
| (d) the surface as a named interface | every relation the kernel reads or states under one prefix, `ein.…`, as a module's names are under `std.…`; a program writes the short names | `(ein.refute)` or `(false)` by a short name | as written | a rename, and a decision on how a program gets the short names |

`(abort)` says more than happens: one state is dropped and the search continues. `(dead)` is the word the pages
already use for the *verdict* on a state, which P1f.21's design keeps apart from the fact that causes it (*dead
from `(false)`*). The rename itself is mechanical and wide: `(false)` is written 658 times in 278 programs under
`examples/`, `tests/` and `stdlib/` (`grep -c '(false)'` over the three, summed), and in the pages, the
diagnostics and the goldens.

**What I would do.** Not rename in this phase. It is the same question as P1f.21's, what the kernel's verdicts
are and how a program sees them, so it is put there as
[Q-M1f.399](../open_questions.md#q-m1f399--the-names-of-what-controls-the-search-false-and-open), with the options
above. If it is renamed, I would take a word for the **proposition** ((b)), because the same fact has to read well
under an arrow, where it controls nothing; the control is then one sentence of the kernel's surface: *a state that
holds it is dropped*. And (d) is worth its own look there: it would make Q-M1f.397's line between kernel and
theory something a reader sees in every name.

### 10.8 Questions as facts: what rules can do with `(open q)`

Today `open` is the verdict and nothing else: `(think Alice (open p))` is refused at the parse (*"open is
reserved: it is the verdict a rule's :assert states … and names no relation and no value"*). So the runs below
write the relation `question` where the note writes `open`; nothing else differs.

**They are naturally hypotheses: yes.** An hrule over the question facts proposes exactly what is asked.
`q2.ein`:

```lisp
(relation lives-in Person House :why "{?1} lives in {?2}")
(question (lives-in Alice House-1))
(question (lives-in Bob House-2))
(lives-in Carol House-3)
(hrule ask ()
  :match  (question (lives-in ?p ?h))
  :assert (lives-in ?p ?h)
  :why    "suppose {?p} lives in {?h}: it is asked")
```

`ein show hyps q2.ein`: `(lives-in Alice House-1)`, `(lives-in Bob House-2)`, ``H, built by the rung `hrules`: 2
candidates``. That is the second rung of the ladder, *the facts that would discharge what root owes*, written as a
rule of Ein instead of read off the obligations by the kernel.

**What rules can infer from a question directly.** `q1.ein`:

```lisp
(relation lives-in Person House :why "{?1} lives in {?2}")
(relation has Person Key        :why "{?1} has {?2}")
(question (lives-in Alice House-1))
(key-of House-1 Key-1)
(has Alice Key-1)
(rule sub-question ()
  :match  (and (question (lives-in ?p ?h)) (key-of ?h ?k))
  :assert (question (has ?p ?k))
  :why    "to know whether {?p} lives in {?h}, ask whether {?p} has {?k}")
(rule answer ()
  :match  (and (question (lives-in ?p ?h)) (key-of ?h ?k) (has ?p ?k))
  :assert (lives-in ?p ?h)
  :why    "{?p} has {?k}, the key of {?h}")
(query :goal (question ?q))
(query :goal (lives-in ?p ?h))
(solve :no-search)
```

`ein solve q1.ein`: the questions are `(lives-in Alice House-1)` and `(has Alice Key-1)`; `(lives-in Alice
House-1)` is held. So, each a thing a rule can do with a question:

| | from a question | the example |
|---|---|---|
| 1 | **a further question**: what would have to be known to answer it. Reasoning from the goal backwards, in an engine that only goes forwards | `sub-question` |
| 2 | **an answer, computed only where asked** | `answer`. The calculator already works so: `(add X Y)` is a question and `sum-of` its answer, which is why it computes only what a program asks |
| 3 | **a hypothesis** | `q2.ein` above |
| 4 | **a contradiction**: every answer to a question is denied | what domain elimination concludes |
| 5 | **a report**: which questions nothing answers | `(question q)` with `(unknown q)`; it reads an absence, so it is a boundary rule and not a monotone one |

**The one thing an ordinary fact cannot be is *still open*.** A held fact is never removed, so `(question q)`,
held, means *q was asked*, in this state and in every larger one, and it stays held after `q` is answered. *Still
unanswered* is the question together with the absence of `q` and of `(not q)`, which is what the kernel's pass
computes today and what P1f.21's survey found of `(open R)` (*"it can be removed"*). That lines the note up with
Q-M1f.390's option (c): the question (there, the *requirement*) is a fact, the answer (the *payment*) is a fact,
and *owed* is their difference, never stored. It is entered there as [V-5](findings.md#v-5--the-users-notes-from-p1f22-questions-as-facts-the-two-negation-rules-and-the-names).
