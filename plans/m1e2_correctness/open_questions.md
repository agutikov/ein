# M1e2 — Open questions

Questions this milestone **raises**, as `Q-M1e2.<n>`. The M1e questions it
carries keep their `Q-M1e.<n>` ids and live in
[M1e's register](../../docs/history/m1e_review_processing/open_questions.md). An id is sticky
and is never reused.

Each of the first five is a **reading** a contested stage needs before it can
choose a fix. They are filed here so that the review
([S1e2.1.7](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)) has one list to
rule on, not five stage files to find.

## Index

| Q | title | needed by | status |
|---|---|---|---|
| [Q-M1e2.1](#q-m1e21--is-a-discharged-superset-of-a-discharged-state-a-new-model) | Is a discharged superset of a discharged state a new model? | [S1e2.2.3](p1e2.2_wrong_answers/s1e2.2.3_obligations_join_superset.md) (W5) | **ruled 2026-11-01**: *literal*, k = \|R\| = 7, as Q-M1e2.7's (a) had said; W5's `right` restated |
| [Q-M1e2.2](#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in) | Does a guard's `absent` premise bind the state the search ends in? | [P1e2.1b](p1e2.1b_non_monotone_programs/README.md)'s [S1e2.1b.2](p1e2.1b_non_monotone_programs/s1e2.1b.2_fork_or_scratch.md) (the evidence, done), then [S1e2.2.6](p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md) (W8, done 2026-11-04 as far as the deferral allows) | **deferred to [F21](../followups/f21_non_monotone_programs_decided.md), 2026-11-02** (D1–D3); W8's tension with the milestone's acceptance **ruled 2026-11-03**: *amend the acceptance*, W8's rows open with F21 as owner ([S1e2.1.7 § The user's rulings](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#the-users-rulings-2026-11-03)); one row since 2026-11-04, when S1e2.2.5's F2 (*via*) refused `stale-naf-fork` |
| [Q-M1e2.3](#q-m1e23--what-does-exhausted-certify-when-the-rung-leaves-relations-uncovered) | What does `exhausted` certify when the rung leaves relations uncovered? | [S1e2.2.8](p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md) (W6) | **ruled 2026-11-03: (a)**, *scoped, and the read-out says so* ([S1e2.1.7 § The user's rulings](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#the-users-rulings-2026-11-03)); **built 2026-11-04** by S1e2.2.8 (`52004a68`): the `outside` row and `root.hypgen.outside`, W6's two rows fixed |
| [Q-M1e2.4](#q-m1e24--is-an-empty-unsat-core-a-core) | Is an empty unsat core a core? | [S1e2.2.13](p1e2.2_wrong_answers/s1e2.2.13_empty_core.md) (W12) | **closed 2026-11-03** at the revisit, by `solve_interface.md` § 0: W12's R is {∅}, so no core; no region run reports a 0-fact core (below) |
| [Q-M1e2.5](#q-m1e25--does-s1f108s-ruling-move-here) | Does S1f.10.8's ruling move here? | [P1e2.4](p1e2.4_what_the_switch_left/README.md)'s S1e2.4.1, then [S1e2.2.5](p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md) (W3) and P1e2.1b's [S1e2.1b.4](p1e2.1b_non_monotone_programs/s1e2.1b.4_refutation_resting_on_absent.md) | **ruled 2026-11-02: *forbid*** ([S1e2.4.1 § 5](p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#5-the-ruling)); F1–F5 are S1e2.2.5's; applied at the revisit (2026-11-03) to `W1/alive-empty-interlayer` too |
| [Q-M1e2.6](#q-m1e26--does-p1e23-become-a-phase) | Does P1e2.3 become a phase? | the milestone's scope; [P1e2.3](p1e2.3_surfaces_that_misreport_the_answer/README.md) | **ruled 2026-11-02**: yes, after P1e2.1c; **specified the same day**, four stages, 5 d; **closed 2026-11-03** at its S1e2.3.4, every surface of the re-take gone, each under its stage's ruling ([§ Closed](#closed-2026-11-03--every-surface-gone-under-its-ruling)) |
| [Q-M1e2.7](#q-m1e27--the-answer-is-a-region-of-the-lattice) | The answer is a region of the lattice | [P1e2.1a](p1e2.1a_the_solution_region/README.md), all of it; [P1e2.1b](p1e2.1b_non_monotone_programs/README.md) for (d); then every case the review has not read | **reading given 2026-10-15**; on 2026-10-16 (a)–(c) and (e)–(i) [ruled for the monotone case](#the-second-answer-2026-10-16), and [(e)'s display and (h)'s names](#the-third-answer-2026-10-16); [the census](#the-census-2026-10-16) contradicts none; **(d) deferred to [F21](../followups/f21_non_monotone_programs_decided.md), 2026-11-02** (D4) |
| [Q-M1e2.8](#q-m1e28--where-does-the-rework-of-ein-solve-go) | Where does the rework of `ein solve` go? | P1e2.1a's S1e2.1a.5 (it must print `k` = \|R\|), and P1e2.2's scope | **ruled 2026-10-16**: [S1e2.1a.9](p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md) now, [P1e2.1c](p1e2.1c_the_region_search/README.md) next |
| [Q-M1e2.9](#q-m1e29--which-constants-are-names-of-the-program) | Which constants are names of the program? | [S1e2.2.14](p1e2.2_wrong_answers/s1e2.2.14_unread_names.md) (W15), and S1e2.2.7 (W7) after it | **ruled 2026-11-04: (a)**, *§ 1.2 literally*: a constant in a fact at any depth or anywhere in a rule; not one only a query names. **Built 2026-11-04** by S1e2.2.14: W15's eight rows fixed |
| [Q-M1e2.10](#q-m1e210--when-the-obligations-rung-cannot-trust-its-domain-scan-whose-candidates-is-r-over) | When the obligations rung cannot trust its domain scan, whose candidates is R over? | [S1e2.2.15](p1e2.2_wrong_answers/s1e2.2.15_obligations_rung_universe.md) (W14) | **ruled 2026-11-04: (a)**, *decline*: a guard over a relation in D declines the rung, and R is the blind enumerator's region; put to the user the same day by T1e2.2.15.1; **built** the same day (`bde8f53b`), W14's four rows fixed |
| [Q-M1e2.11](#q-m1e211--which-derivations-is-the-unsat-core-the-smallest-over-and-may-a-config-flag-change-it) | Which derivations is the unsat core the smallest over, and may a config flag change it? | [S1e2.2.11](p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md) (W10) | **ruled 2026-11-04: (i)(b) and (ii)(b)**, put to the user the same day by T1e2.2.11.1: the core promise is every derivation the program records, (3) built and `W10/alt-cap-core` fixed, `W10/alt-cap-shared` open and deferred to [F22](../followups/f22_exact_core_past_the_cap.md); the core, and nothing else, exempt from the config principle under `:record-alternative-justifications false`. **Built** the same day by S1e2.2.11 |
| [Q-M1e2.12](#q-m1e212--which-activators-does-a-dead-nodes-core-follow) | Which activators does a dead node's core follow? | [S1e2.2.19](p1e2.2_wrong_answers/s1e2.2.19_core_misses_activator.md) (W19) | **ruled 2026-11-04: (c)**, put to the user the same day by T1e2.2.19.1: a dead node's core follows an activator only a node derives or commits, and root's, stated or derived, are the program's. **Built** the same day by S1e2.2.19 (`94c7bcfb`): W19's three rows fixed, no corpus core moved |
| [Q-M1e2.13](#q-m1e213--what-does-a-wildcard-or-a-range-in-a-rules-pattern-mean) | What does a wildcard or a range in a rule's pattern mean? | [S1e2.2.20](p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md) (W20) | **ruled 2026-11-04: (a)**, put to the user at P1e2.2's end: `_` in a rule's pattern matches any value, as the grammar says, head and argument; a range in a rule is refused at load. The head half re-ruled by Q-M1e2.14 (c). **Built 2026-11-05** by S1e2.2.20 (`eddea251`): W20's five rows fixed. The range half superseded by Q-M1e2.24 and built 2026-11-08 by S1f.5.3 (`65d1ad85`): no range token |
| [Q-M1e2.14](#q-m1e214--how-does-a-head-wildcard-match-every-relation) | How does a head wildcard match every relation? | [S1e2.2.20](p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md) (W20's `wild-in-head`) | **ruled 2026-11-05: (c)**, put to the user the same day by T1e2.2.20.1: a head `_` is refused at load, as a range is; the argument `_` matches any value; `01_grammar.md`'s row an *argument wildcard in patterns*. **Built 2026-11-05** by S1e2.2.20 (`eddea251`) |
| [Q-M1e2.15](#q-m1e215--what-does-a-derivation-through-a-forall-rest-on) | What does a derivation through a `forall` rest on? | [S1e2.2.21](p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md) (W21) | **ruled 2026-11-05: (b)**, put to the user the same day by T1e2.2.21.1: a derivation through a `forall` rests on each fact the universal read, each at its own smallest frontier, the residual (13 of `zebra2-hints`' N cores larger than (c)'s) joining F22's; and, ruled with it, a core is taken only where a surface prints one (*minimum work per question*), moving no answer. **Built 2026-11-05** by S1e2.2.21 (`67d07fab`, `3fb9d712`): W21's six rows fixed, no answer moved by the lazy core |
| [Q-M1e2.16](#q-m1e216--what-does-the-close-do-with-the-inventorys-items) | What does the close do with the inventory's items? | [the inventory of 2026-11-05](inventory_2026-11-05.md); S1e2.2.22–.25, S1e2.3.8, S1e2.3.10, F21, F22, F23 | **ruled 2026-11-05: as recommended**, put to the user the same day with the inventory the user asked for before the close: F1 and F2 are W22 and W23, fixed before the close; `_` in a guard, a premise that is not a form and a nested unbound relation or keyword pair refused at load (W24); W-8 and W-10 F22's; a reloaded dump's provenance S1e2.3.8's, `refuted by` and `--json-report`'s rows S1e2.3.10, the detector's labels F21's; the eleven diagnostics F23; the owned wrong answers with no row filed `open`, deferred (S1e2.2.25); the close's re-take the register's tests and the inventory's sweep |
| [Q-M1e2.17](#q-m1e217--where-does-a-keyword-end) | Where does a keyword end? | [S1e2.2.22](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md) (W22) | **ruled 2026-11-05: (i)(a) and (ii)(a), as recommended**, put to the user the same day by T1e2.2.22.1: (i) a keyword ends at the word boundary `00_ebnf.md` § 1 states, so `(neq-1 ?x)` stays `neq` and `-1` (not (b), a parse error, nor (c), `neq-1` a name); (ii) a trace word read whole, which has no other reading, is a parse error (not (b), the six trace words left matching as a prefix). **Built 2026-11-05** by S1e2.2.22 (`ed45a470`): W22's fourteen rows fixed |
| [Q-M1e2.18](#q-m1e218--what-else-in-a-rule-reads-nothing-in-silence) | What else in a rule reads nothing, in silence? | [S1e2.2.24](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md) (W24) | **ruled 2026-11-05: (i)(a), (ii)(a), (iii)(a), (iv)(a), (v)(a), as recommended**, put to the user the same day by T1e2.2.24.1: a form, a variable no earlier premise binds and a keyword pair as a guard's argument are refused at load; a relation parameter its activator binds to a nested fact is refused at load where the activator is written and by the compiler where a rule derives it; each a W24 row, filed and fixed by S1e2.2.24's T2; `empty_absent.ein` and `unbound_head.ein` refused at load and moved to `examples/broken/load/`. **Built 2026-11-05** by S1e2.2.24 (`3ab9a231`): W24's seven rows fixed |
| [Q-M1e2.19](#q-m1e219--what-do---minimal-and---maximal-answer-past-the-limit) | What do `--minimal` and `--maximal` answer past the limit? | [S1e2.2.26](p1e2.2_wrong_answers/s1e2.2.26_line_modes_past_the_limit.md) (W27) | **ruled 2026-11-05: (1) and (2), as recommended**, put to the user the same day by S1e2.2.25's re-take (T1e2.2.25.1): past `--region-limit` on a flagged program the line modes label every member met as on their line (65 214 on `square-unique/corner-house`), where describe certifies one minimal state; S1e2.2.26 files it as W27 and fixes it before the close, `--minimal` certifying exactly what describe certifies and `--maximal` none, the maximal line not known. **Built 2026-11-05** by S1e2.2.26 (`c21f93fc`): W27's four rows fixed |
| [Q-M1e2.20](#q-m1e220--does-a-claim-compare-values-or-their-printed-text) | Does a claim compare values, or their printed text? | [S1e2.3.8](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md) (N5) | **ruled 2026-11-06: (a), as recommended**, put to the user the same day by S1e2.3.8's step 1 (T1e2.3.8.1): a claim and a query's projection classes compare printed text, which printed the int `1` and the name `"1"` alike, so a false claim held and two classes were one on three probes (no corpus program); the one printer moves them, and the case is W28, filed `open` (T1e2.3.8.2) and fixed with the printer. **Built 2026-11-06** (`712f23fd`) |
| [Q-M1e2.21](#q-m1e221--which-rule-does-refuted-by-name-where-the-core-derives-its-contradiction-through-two) | Which rule does `refuted by` name where the core derives its contradiction through two? | [S1e2.3.10](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_refuted_by_and_report_rows.md) (N7) | **ruled 2026-11-06: (a), as recommended**, put to the user the same day by S1e2.3.10's step 1, which paused on it (5 041 distinct printed cores per commit, 6 917 nightly, derive a fact of their contradiction through two rules): `refuted by` names the first recorded derivation the core supports, the alternatives flag's exemption as it stands, the `two-rules` pair the accepted cost; **built** the same day (`a3bb9e70`) |
| [Q-M1e2.22](#q-m1e222--does-ein-need-a-wildcard) | Does Ein need a wildcard? | [S1f.5.1](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md) (W-22) | **ruled 2026-11-06**: a bare `_` is reserved and refused, and `_` is valid anywhere in a name; not built |
| [Q-M1e2.23](#q-m1e223--what-is-a-string-for) | What is a string for? | [S1f.5.2](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md) (F4) | **ruled 2026-11-06**: natural language, only in `:why`, `:source` and `:goal-text`; G-6 put to the user; not built |
| [Q-M1e2.24](#q-m1e224--are-there-numbers-in-ein) | Are there numbers in Ein? | [S1f.5.3](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.3_range_retired.md) (F6), [S1f.5.4](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md) (F7) | **ruled 2026-11-06 and -07**: no numbers and no ranges, now or planned; any run of name characters is a name, digits included, and a digit label is a candidate; a name's characters are an ASCII allow-list, forever, `\` never and quotes and brackets for now; **the range token retired 2026-11-08** by S1f.5.3 (`65d1ad85`), the rest not built |
| [Q-M1e2.25](#q-m1e225--how-strict-is-the-grammar-and-what-holds-the-parser-to-it) | How strict is the grammar, and what holds the parser to it? | [S1f.5.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md) (F9) | **the user's instructions, 2026-11-06 and -07**: revisit it, and generate the parser; **ruled 2026-11-07: grmtools, with recovery on**, made deterministic (F11); not built |
| [Q-M1e2.26](#q-m1e226--are-eq-and-neq-relations) | Are `eq` and `neq` relations? | [S1f.5.7](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md) (S-18, G-21) | **ruled 2026-11-07**: yes, of equality and non-equality, their facts fixed by name identity and computed, in a goal, a premise and a conclusion, a free variable over the program's names; supersedes Q-M1e2.18 (ii)(a); a nested fact as a guard's argument answered; not built |
| [Q-M1e2.27](#q-m1e227--what-do--and-null-mean-in-eins-json) | What do `[]` and `null` mean in Ein's JSON? | [S1f.5.8](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md) (D-14) | **ruled 2026-11-07**: `[]` is the empty set, `null` an object that does not exist; a node that is not dead has `core: null`; not built |
| [Q-M1e2.28](#q-m1e228--which-keywords-does-each-form-take) | Which keywords does each form take? | [S1f.5.11](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md) (D-15, G-15, G-22) | **ruled 2026-11-07**: a fixed list per form, and any other keyword is an error; a term and a nested fact take none, so `:where` is refused wherever written; `:cardinality` and `:mode` retired, a fact keeps `:rule` and `:using`, and `:why` on a fact is refused; supersedes Q32's drop; not built |

C1's reading is [Q-M1e.12](../../docs/history/m1e_review_processing/open_questions.md#q-m1e12--the-blind-rung-is-untyped-and-a-model-binds-a-type-as-an-object)'s
and is not re-filed. The user ruled it (a) on 2026-11-03 ([S1e2.1.7 § The user's rulings](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#the-users-rulings-2026-11-03)).

---

## Q-M1e2.1 — Is a discharged superset of a discharged state a new model?

`examples/ein-bugs/wrong-answers/owed-seats.ein` (two guests, two chairs, `total-owed` +
`surjective-owed`) has two matchings. Is a state that seats Ann on **both**
chairs and Bob on one, owing nothing, a third model? *No* (**minimal**): k = 2,
the lattice's own design, where a complete node leaves the frontier. *Yes*
(**literal**, Q-M1e.6's *owes nothing*): k = 7. Today the lattice answers 3
and the tree 6, so both are wrong under both readings. The recommendation in
S1e2.2.3 is *minimal*. A requirement is discharged by a witness, and a second
witness is `functional`'s question, not `total`'s.

**Ruled 2026-11-01: *literal*.** The user, choosing between keeping the
minimal line as W5's question and following (a) below to the letter, took
the second: k = |R|, and a discharged superset of a discharged state is a
state of R. The register's `W5/owed-seats` row states **7** as `right`, with
the minimal line's 2 as the alternative not taken, and its `now` is a bare
`ein solve`, which answers all of R. The row stays `fixed`: the walk's 3 was
wrong under both readings. The rest of this entry is the record of how the
question got here.

**Reframed by [Q-M1e2.7](#q-m1e27--the-answer-is-a-region-of-the-lattice)
(2026-10-15).** The two readings are two numbers of one region: *minimal*
is the size of R's minimal line (2, the matchings), *literal* is |R| (7),
and R's maximal line is one node, every guest on every chair. The question
becomes Q-M1e2.7's (a), which line `k` counts, and is re-asked when
[P1e2.1a](p1e2.1a_the_solution_region/README.md) has written the definitions.

**Answered by (a), 2026-10-16:** `k` = |R|, so every node of R counts, and a
discharged superset of a discharged state is a solution. On `owed-seats` that
is the literal **7**, the same in commitments and in states, since nothing
there saturates onto anything else. S1e2.2.3 is re-read against it.

## Q-M1e2.2 — Does a guard's `absent` premise bind the state the search ends in?

`features/07_unstratifiable.ein` records `{(p one), (q one)}`, where `(p one)`
was derived under `(absent (q one))`, and its own header says that set *"is not a
model of this program under any reading"*. Under an append-only reading of
Q-M1e.6 it qualifies. Three answers: **guards are premises** (a stale fact
disqualifies a state), **refuse what cannot be stratified** (a load error),
or **append-only is the semantics** (not wrong, and say so). The first two
compose.

**Asked of every node (2026-10-15).** The oracle has to build each node of
the cube, and the question is then not only about the state the search ends
in. Built on root's fixpoint (the engine's fork), `{(q one)}` on
`stale-naf-fork.ein` is dead. Built from scratch (`region.py`), it is in R.
[P1e2.1b](p1e2.1b_non_monotone_programs/README.md)'s
[S1e2.1b.2](p1e2.1b_non_monotone_programs/s1e2.1b.2_fork_or_scratch.md) owns the
question in that form.

**Deferred to [F21](../followups/f21_non_monotone_programs_decided.md),
2026-11-02**, unruled. S1e2.1b.2's evidence is in
([`node_census.md`](p1e2.1b_non_monotone_programs/node_census.md)): fork
and scratch differ on 4 of 256 flagged nodes and on none of 268 213 monotone
ones, and § 9 there computes W8's two rows under every option. The user then
re-planned P1e2.1b to detect, document and warn, verbatim: *"Currently my
goal is not to build complete theory os solving the non-monotonic problems;
But I want to have a robust ways to detect non-monotonocity, understand
consequences of solving it either way, be able to at least show warnings
about the program that is not monotonic, etc. … so prepare everything we are
able to do now, without making final decisions; And then leave further
investigation of non-monotonic lattices as followup."* The question grew on
the way: it is one case of the user's general form, *"sat(P+C1+C2) !=
sat(sat(P+C1)+C2)"*, and what it asks is whether a node's state is a function
of C alone. F21 holds it as D2 (the options (A)–(D), (D)'s scope, admission in
stratum order), with (E), *forbid* extended to every `absent` site, as D1 and
path dependence as D3. Until it is decided a node is built by fork from root,
and P1e2.1b's detector says so wherever R is reported. W8's two rows stay
`open`; their tension with the milestone's acceptance is parked for the
revisit in [S1e2.2.6](p1e2.2_wrong_answers/s1e2.2.6_stale_naf_positive.md#the-deferral-and-the-milestones-acceptance).

**Ruled 2026-11-03**, the user at the revisit ([S1e2.1.7 § The user's rulings](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#the-users-rulings-2026-11-03)): *"(2) Amend
acceptance"*. Q-M1e2.2 stays deferred to F21's D2. W8's two rows stay `open`,
reviewed `deferred` (a verdict added to the register for them), with F21 as
`owner`, and the milestone's acceptance lets such a row stay open, named at the
close. S1e2.2.6 stays as restated: after S1e2.2.5's F2 it re-checks the
reading.

**2026-11-04, S1e2.2.5's F2: *via, per activator*** (the user's ruling of
that day). *Forbid* refuses `stale-naf-fork` and its twin, so
`W8/stale-naf-fork` is fixed by the refusal and the question is one witness's,
`W8/features-07`, still deferred to F21's D2.
S1e2.2.6 re-checked that row the same day and closed as far as the deferral
allows: `features/07` answers what the row records, the row stays `open`,
reviewed `deferred`, owner F21, and the witness's header says so.

## Q-M1e2.3 — What does `exhausted` certify when the rung leaves relations uncovered?

`completeness.md` §3c's fixture is `exhausted = true` with `knows` never
hypothesised, and `arity3.ein` likewise for an arity-3 goal. Relative to
`alive₀` it is right by Q-M1e.6's second clarification, and as a statement
about the program it is not. Three answers: **scoped, and the read-out says
the scope** (recommended), **uncertified unless covered**, or **open-world,
not wrong**.

**Ruled 2026-11-03: (a), *scoped, and the read-out says so*** ([S1e2.1.7 § The user's rulings](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#the-users-rulings-2026-11-03)). The
revisit had found the question narrowed and not decided: ruling (g) of
2026-10-16 makes `exhausted` *"the reported region is all of R"*, partial only
under a budget, and (h) makes the universe the candidate list, which rules out
*uncertified unless covered* as worded; whether `knows` and `r3` belong in the
universe no ruling said. The user's answer: R is over the universe as scoped,
and the read-out must state that `knows` and `r3` are outside it. Written where
the definition lives,
[`solution_semantics.md` § 1.4](../../docs/kernel/inference/solution_semantics.md#14-candidate-and-universe).
W6's two rows are confirmed, and S1e2.2.8 builds the sentence.

**Built 2026-11-04** by [S1e2.2.8](p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md)
(`52004a68`). The read-out names a relation outside the universe when the program
can conclude or ask about it, does not scope it out itself, and the rung that
built H proposes no fact of it, § 1.4's definition: an `outside` row under
`universe` on every question, *knows — no candidate: no obligation owes it. R holds of it only what the program states and saturation derives*, `root.hypgen.outside` in the summary,
a line beside each claim `ein test` prints, and `ein oracle --dump`'s
`region.md` and `region.json`. No verdict, R or exit code moved. It names a
relation on 22 of the 234 sweep programs that reach H, 16 of them under the
hrules; the wording S1e2.1.7's option (a) used, *the relations H holds no
candidate of*, would have named one on 168, `is-a` on `arity3.ein` among them,
which the program scopes out (T1e2.2.8.1, 2026-11-04). W6's two rows are
fixed.

## Q-M1e2.4 — Is an empty unsat core a core?

`alive-empty-phase1.ein`'s exhausted `Contradiction` is right and prints
`unsat core (0 facts)`, because its refutation rests on premise-less engine
writes. M1e's S1e.3.1 called it honest. S1e2.2.13 argues it is false: ∅ with
this program's rules is consistent, and the engine knows which deaths the
writes came from.

**Closed 2026-11-03 by `solve_interface.md` § 0** (approved 2026-10-16), at the
revisit ([S1e2.1.7](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md#revisited-2026-11-03)).
Under R a node is `sat(root ∪ C)` with nothing the search wrote, so the kill
cache's premise-less negatives are no node's. R on `alive-empty-phase1.ein` is
{∅}, root alive and discharged, and the `Contradiction` whose core was empty is
no answer of the program. The 2-fact core of the other reading was Q-M1e.7
ruling A's node, which § 0 replaced. No answer reports a 0-fact core today:
every corpus entry with a declared `solve` run and every register witness,
`ein solve FILE --json-summary` and `--single`, 474 runs over 237 files at
`d4ff0f46`, of which 27 report a core and none has 0 facts (2026-11-03). W12 is
`fixed`, and S1e2.2.13 dropped.

**A dated note, 2026-11-04 (S1e2.2.19).** That census read root's cores, the
summary's `region.core`; a dead node's core, which no summary carries, could
be empty: `examples/ein-bugs/wrong-answers/core-activator-guard-only.ein`'s N
member `{(p a)}` reported a 0-fact core (`ein solve FILE --dump-answer DIR`,
`nogoods.json`, at `a41eb76e`), its refuting plan matching no fact and
compiled from an activator the node derived, which no derivation recorded
(W19). Since S1e2.2.19 (`94c7bcfb`) the core follows that activator and is
`(p a)`. The closing above stands, a dead node's core included: over
the stage's sweep (390 files, 2026-11-04), no dead node of an oracle cube
and no member of N reports a 0-fact core but on a run stopped past
`--region-limit`, which dumps the members it met with none
(`lattice/06_friends_tables.ein`, 68; S1e2.2.19 § Notes).

## Q-M1e2.5 — Does S1f.10.8's ruling move here?

W3 cannot be fixed until someone rules *forbid* or *respect* on a refutation
that rests on `(absent P)` over an extendable relation, and that ruling is
M1f's [S1f.10.8](../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.8_refutation_under_absent.md).
The proposal is to run its T1 and its ruling as S1e2.2.5's first task, leaving
S1f.10.8 the corollary it writes into `absent_semantics.md`. The alternative
is that W3 waits for M1f.

**Moved here 2026-11-01**, into a phase of its own:
[P1e2.4](p1e2.4_what_the_switch_left/README.md)'s S1e2.4.1, beside the `--models key` failure the region
gallery found. The user filed both *"in additional phase in m1e2"*. What is
still open is the ruling itself, *forbid* or *respect*. W3's four rows were
flipped to `fixed` at the switch by route 2, which learns from no refutation.
That flip assumed *respect*, and the stage puts it in front of the user with
the exposed set re-counted.

**Ruled 2026-11-02: *forbid*.** The user's reading: *"That is exactly what I meant (absent q ?x) => false - this is incorrect form of obligation, isn't it?"*
Over a relation a commitment can still extend, the shape is an obligation
written as a refutation, and a program that writes it is to be refused with
the replacement named. W3's four rows re-open with the refusal as `right`.
[S1e2.2.5](p1e2.2_wrong_answers/s1e2.2.5_naf_upward_closure.md) builds the
refusal and settles F1–F5
([S1e2.4.1 § 5](p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md#5-the-ruling)).

**Note, 2026-11-02 — P1e2.1b's taint census**
([`taint_census.md`](p1e2.1b_non_monotone_programs/taint_census.md),
`cargo run --release -p ein-infer --example taint_census`). On the 18 flagged
cubes within the limit, 122 dead nodes: 100 rest on no `absent` over D, and
none of those has an alive superset; every dead hole rests on one.
`ein-infer/tests/region.rs::an_untainted_death_is_dead_in_every_superset`
holds it per commit. The taint has to follow each plan's activator: on the
written `ACTIVATOR_UNDER_ABSENT`, read off provenance alone, a dead hole is
untainted beside an alive superset. What a route may learn before the refusal
ships, and from whatever F2 and F3 leave loading, is
[F21](../followups/f21_non_monotone_programs_decided.md)'s D6.

**Note, 2026-11-04 — W17**
([S1e2.2.17](p1e2.2_wrong_answers/s1e2.2.17_forbid_activator_chain.md)). The
ruling's *a relation a commitment can still extend* was built as the static
check's D, whose closure follows the plans root compiled. Where a plan a node
compiles writes, D is too small: five programs whose refutation rests on an
`absent` through an activator chain that first appears at a node loaded with
a dead hole, and one of them, `forbid-chain-node-writes.ein`, was a
counterexample to the taint test above read with D. *forbid* reads D⁺ since
(`f5a0dca9`), D followed through every rule with parameters whose activator
relation it holds, refuses all five, and the taint test reads D⁺ too. The
ruling is unchanged: what moved is how *extendable* is computed.

## Q-M1e2.6 — Does P1e2.3 become a phase?

Nine surfaces that misstate a run whose answer is right, and one reachable
panic, are listed in the [milestone README](README.md#p1e23--surfaces-that-misreport-the-answer)
with witnesses (since 2026-11-03 a pointer to [the phase's re-take](p1e2.3_surfaces_that_misreport_the_answer/README.md#the-re-take-2026-11-02)). They are correctness issues and not wrong answers. Whether
they belong in this milestone, a later one or a followup is the user's call.

**Note, 2026-10-18 — S1e2.1a.5's switch removed eight of the ten surfaces**,
each re-checked on its witness against the region read-out:

| | after the switch |
|---|---|
| S1 | gone: no surface prints *"(not certified — pass --exhaustive)"*, and a `NOT CHECKED` names the question the run asked (`-n 1`, a line, a budget) |
| S2 | gone: the trace is `# Region trace`, headed by `asked`, the counts and `exhausted`; a stopped run never says R is empty. `trace_unsat.md` was removed |
| S3 | gone: `ein-summary/2` has no `type`; a stopped run's `region` says `exhausted: false`, `empty: null`, and why it stopped |
| S4 | gone: `-H` lists the universe the run used, and it is `root.hypgen.emitted` — 56 on `zebra2`, 96 on `zebra2-minus-15-obligations` |
| S5 | gone: `-E` and `-m` are retired and refused |
| S6 | gone: a class with no row says *"the goal has no row: undetermined here"* |
| S7 | gone: a class's rows are sorted and deduplicated, and the read-out prints them all |
| S8 | gone: `nested-core.ein`'s core prints `(not (p A))`, the s-expression |
| S9 | the tree's, and no command runs it |
| P1 | **still panics**: `ein render constraints corpus/fuzz_findings/kwpair-below-the-filter.ein` exits 101 at `dot_util.rs:150` |

So the question is now P1's alone.

**Note, 2026-11-01 — S4 is not gone.** S1e2.1a.6's lever test
(`ein-cli/tests/lever_invariance.rs`) found that the note above read it on
`zebra2`, where the two agree. `summary::root_block` still runs the walk's
`hypgen::generate` on a fork of root, which reads the program's lookahead and
kill-cache flags and `EIN_OBLIGATION_CHOICE`, and counts root's facts after
it. On `branching/02_one_dead_one_alive.ein` `-H` lists 48 candidates and
`root.hypgen.emitted` says 30, and `root.facts` says 27 where root's fixpoint
holds 23, the other four being the kill cache's `(not h)` writes. The block
moves with the program's flags on 19 programs, and `-L` and `-K` leave it
alone. No answer moves. The witnesses and the reproduction are in
[S1e2.3.1](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md), which carries the README's S4 paragraph since 2026-11-03.
So the question is P1's and S4's.

**Note, 2026-11-03 — S4 is fixed** by P1e2.3's
[S1e2.3.1](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md),
on the user's ruling (a) of that day: the block is root's fixpoint and H as
every route starts from them. Root's fixpoint on `branching/02` holds 22, not
the 23 the note above says (the 23 was the block with the program's lookahead
off, still counting one `emit_closed` write); the block now reads 22 and 48.
This question closes at S1e2.3.4.

**Ruled 2026-11-02: P1e2.3 becomes a phase**, in the user's order of the
milestone: P1e2.4, P1e2.1b, P1e2.1c, then P1e2.3, then the wrong answers
([README § Phases](README.md#phases)). It is specified when it starts, from P1,
S4 and whatever P1e2.1b and P1e2.1c leave.

**Note, 2026-11-02 — one more for P1e2.3 to read, from P1e2.4's S1e2.4.2.**
It is not a misreport. Every line printed is true, but the forms printed after
the read-out are outside the user's 100-line ruling.
[`read_out.md` § 4.1](p1e2.1a_the_solution_region/read_out.md#41-past-100-lines-stdout-lists-what-fits)
cuts the read-out's listings at 100 lines and says nothing of `--models
key|grid`, `-p`, `-P` or `--diff-models`, which print every listed state.
`ein solve examples/saturation/type-exclusivity/pets.ein --models grid` prints
11 339 grids (2.2 MB), and before S1e2.4.2 its `--models key` printed an
11 339-row table. Two things are open: whether the forms share the 100 lines,
and whether `--dump-answer`'s `answer.txt` is where the rest goes. That is a
surface question
([the stage file](p1e2.4_what_the_switch_left/s1e2.4.2_models_key_declines.md#notes)).

**Specified 2026-11-02**, at the user's *"write P1e2.3 plans also"*:
[P1e2.3](p1e2.3_surfaces_that_misreport_the_answer/README.md), four stages and
5 days. Its re-take at `7b33c3ab` corrects the notes above in three places.
**S5 is not gone**: it moved to `-g`, whose `.max(1)` reads 0 and a negative
as 1 with no word. **S4 is wider than the 2026-11-01 note**:
`root.hypgen.emitted` differs from H on 71 of 212 files and `root.facts` from
root's fixpoint on 175, mostly by `emit_closed`'s `(__closed__ …)` writes, and
root's fixpoint on `branching/02` is 22, not 23. **P1 panics on a second
door**, `render program`. The forms note above is the phase's S10. S1–S3 and
S6–S9 are gone, each with what holds it or *nothing checks it*. The question
closes at the phase's S1e2.3.4.

### Closed 2026-11-03 — every surface gone, under its ruling

The user ruled every question of the phase on 2026-11-03, Q1 to Q6, and
[S1e2.3.4](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.4_the_inherited_questions_and_the_record.md)
re-took [the phase's table](p1e2.3_surfaces_that_misreport_the_answer/README.md#the-re-take-2026-11-02) the same
day: **no row is live**, and each names what holds it.

| stage | ruling | what it closed |
|---|---|---|
| [S1e2.3.1](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md) | Q1 (a): the summary's `root` block is root's fixpoint and H, config-free; `leftover` config-free; `EIN_OBLIGATION_CHOICE` retired | S4 |
| [S1e2.3.2](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.2_a_keyword_pair_panics_two_views.md) | Q2 (a): `value_label` labels a keyword pair `:key value` for every caller | P1, on both doors |
| [S1e2.3.3](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.3_the_forms_past_100_lines.md) | Q3 (a): the forms share the read-out's 100 lines, drawn from the states stdout showed; `--dump-answer` writes each uncut, one file per form | S10 |
| [S1e2.3.4](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.4_the_inherited_questions_and_the_record.md) | Q4 (a): Q-M1e.5 closed by the ruling of 2026-10-16; Q5 (b): `-g 0` is no node lines and a negative is refused, Q-M1e.17 closed; Q6 (a): S7 and S8 pinned | S5, and S7 and S8 held |

S1–S3, S6 and S9 went at S1e2.1a.5's switch, each held by the test the table
names. The user added three stages on 2026-11-03 that are not surfaces,
S1e2.3.5 (test scratch left in `/tmp`), S1e2.3.7 (four lines that misname
what they print) and S1e2.3.6 (the nightly workflow), so this question did not
wait for them. All three are done, and the phase closed 2026-11-03.

## Q-M1e2.7 — The answer is a region of the lattice

**A reading given 2026-10-15**, by the user, in the review of W1
([S1e2.1.7](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)). It replaces
the question *which commitments are solutions* with *what shape is the set of
nodes that are*. Recorded verbatim:

> "maximal alive commitment" solution definition was formulated before we came
> to obligations mechanism; Basically between bottom bound defined by
> obligations and top bound defined by restrictions (emitted false,
> contradictions) also could be not only single possible model but also could
> be multiple models; […] Let's think about what does solution means on the
> search lattice (set-indexed DAG) - some early layers L0,L1,... with few
> hypothesis doesn't meet obligations and some hight layers L10,L100,L1000
> (that we never reach) produce conflicts; So clearly solution is a subset of
> lattice nodes on multiple layers that does meet obligations and doesn't
> produce conflicts; this is let's say problem solution topology - some of
> nodes are naturally combining like B+C and produce what I named before
> "maximal alive commitment". But in fact we have at first a whole solutions
> topology {empty, A, B, C, (B, C)} and an upper bound of this sub-lattice
> {A, (B, C)}; Deinitely 100% we can't throw out A because it's size is
> smaller than (B, C) if we threat "maximal commitment" literally - smaller
> number of facts. […] Main conclusion from W1 analysis - no solutions is very
> simple case - obligations and contradictions overlap leave no space for
> solutions; one solution is probably a sub-lattice with sinle bottom node,
> not just single valid node; while ambiguity is a sub-lattice maybe even
> partitioned, with multiple bottom nodes; […] So we probably can have not
> only "maximal" solutions, but also "minimal" solutions make sense, for
> example if we have obligations but no restrictions defined, with no
> restrictions the only maximal solution would be fully connected graph, but
> that is a trivial and we probably don't look for it and more than that it
> could be computationally difficult to enumerate all lattice nodes up to
> fully connected graph in the bottom; So our initial simple {no solutions,
> single solution, multiple solutios} transforms into solution sub-lattice
> and it's topology - bottom line minimal solutions and upper line maximal
> solutions. And also, obvious, value of config
> :enable-pre-branch-lookahead must not affect solution values, because no
> config option affects problem search lattice, config may only affect search
> speed.

The user draws the lattice with L1 at the top, so a *bottom node* in the
drawing is a large set. In the language of bounds, the **lower bound** is the
minimal solutions and the **upper bound** the maximal ones. The restatement
below uses *minimal line* and *maximal line* for the two, which reads the
same either way up.

### The reading, restated

A node of the lattice is a commitment `C` (a set of hypotheses). Saturate
root plus `C`, and the node is **dead** (it derives `(false)`), or alive and
**owing** (an obligation the program states is unwitnessed), or alive and
**discharged**. The region is

> **R = { C : C is alive and discharged }**

and it is the answer. Its **minimal line** is R's minimal elements, its
**maximal line** R's maximal ones.

- **Without holes.** Under monotone rules, *dead* is upward-closed (a
  contradiction stays) and so is *discharged* (a witness stays). So R has no
  holes: every node between two of its members is a member. R is then fully
  described by its two lines, which is why two lines are enough.
  *Corrected 2026-10-15:* a witness stays, but a new **debt** can arrive, so
  *discharged* is upward-closed only when no obligation's trigger can grow
  with the commitment. See [§ Holes without `absent`](#holes-without-absent-2026-10-15).
- **With holes.** `absent` breaks both closures. A superset of a dead node can
  be alive again (W3, Q-M1e.9), and a fact derived under an `absent` survives
  the commitment that falsifies its guard (W8). Those two cases become *R has
  holes*, and the two lines no longer describe it.
- **Which line carries the information** depends on the program. With
  restrictions only, the minimal line is ∅ and the maximal line is the
  answer. With obligations only, the maximal line is *everything* (the
  user's "fully connected graph") and the minimal line is the answer. With
  both, the lines meet. A well-posed puzzle has |R| = 1.
- **No solution** is R = ∅, and it has two kinds: no alive node at all (root
  is dead), `Contradiction`; or alive nodes that all owe, `Open` (the
  2026-08-25 ruling, `closure/03`).
- **Config never changes R.** No config option affects the problem's search
  lattice, so none may change the answer. That makes every lever-only register
  row genuinely wrong. It is stronger than the milestone's acceptance line,
  which only forbade a lever to change a *certified* answer. A **budget**
  (`-m`, `-E`) does not change R either. It stops the walk early, and the
  read-out must then say the answer is partial. That last sentence is the
  restatement's reading and is not the user's words.

### Measured

These numbers come from `probes/region.py` (since S1e2.1a.1 [`utils/region.py`](../../utils/region.py)), a brute-force
instrument. It enumerates every subset of a program's candidate hypotheses,
adds it to the program as given facts, saturates with
`:enable-forced-positive` and `:enable-pre-branch-lookahead` off, and reads
the result off `ein saturate` and `--json-summary`'s `owes.root`. Taken
2026-10-15 at `caef67a`. Every row re-takes with one command.

| program | shape | \|R\| | minimal line | maximal line | engine today |
|---|---|---:|---|---|---|
| `branching/15` (W1) | restrictions only | 5 | ∅ | `{(p A)}`, `{(p B),(p C)}` | `Solution`, k = 1 |
| `lattice/02` (W1) | restrictions only | 7 | ∅ | the three pairs | k = 3 at default; `Contradiction` under `-L` |
| `forced-2-two` (W1) | restrictions, two firings | 1 | ∅ | ∅ | `Contradiction` |
| `owed-two-step` (W1) | both; nothing fits | 0 (∅ alive and owing 1) | — | — | `Contradiction` |
| `owed-seats` (W5) | obligations only | 7 | the two matchings | every guest on every chair | lattice k = 3, tree k = 6 |
| `owed-seats` + `(functional seats)` + `(injective seats)` | both | 2 | the two matchings | the same two | k = 2 ✓ |
| `ein-bugs/naf-upward-closure` (W3) | a restriction under `absent` | 3, **with a hole** | ∅ | `{(p A),(q A)}` | the model `{(q A)}` |

The last row is the first **measured hole**. `{(p A)}` is dead
(`(p A)` without `(q A)`), and it lies between ∅ and `{(p A),(q A)}`, which
are both in R. The engine answers `{(q A)}`, which is not on either line, so
under any reading of (a) the answer is wrong. That is W3.

The prototype's first run read obligations through `ein solve -m 0` instead,
and the commitment `{(seats Ann C1)}` on the last row came back owing
nothing. The search had written `(seats Bob C2)` at root as
`<forced-positive>`, plus two `<lookahead-dies-immediately>` negatives. That
fact follows from the obligation, not from any rule. So the oracle that
computes R has to classify by **saturation alone**, with no search-time root
writes. This is W2's mechanism seen from the other side: a forced positive
treats an obligation as if it were a restriction.

### What it settles

- **W1, all seven rows, confirmed** on 2026-10-15, with no split. In R's
  terms W1 is one defect, *a node on the maximal line is never recorded*,
  and layer 0 is the case where the maximal line is `{∅}`. The four lever rows are
  wrong by the config principle alone.
- [Q-M1e.6](../../docs/history/m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
  definition becomes a special case. Its maximality arm is the maximal line,
  right where no obligation is stated. Its *owes nothing* arm makes every
  node of R a solution, which is Q-M1e2.1's literal k = 7.
- [Q-M1e2.1](#q-m1e21--is-a-discharged-superset-of-a-discharged-state-a-new-model)
  becomes sub-question (a) below.

### The cube, and what replaces `k` (2026-10-15)

The user's follow-up, verbatim:

> well, so instead of single number k that may be k=0, k=1. k>1 we have at
> least 3 numbers: |R|, |top(R)|, |bottom(R)|; this is only if we don't have
> holes in R; So we could have both - not connected partitions of R and one
> partition could have holes; holes also have top and bollom bounds, what does
> they mean, do we need to count them, or only outer bound of R has meaning?
> One more notice, without merging lattice looks like hypercube, is it true?
> How we can use it?

**The lattice is the n-cube, exactly.** Without merging, the nodes are the
subsets of the n candidates ordered by inclusion, and the diagram is the
n-dimensional hypercube:
- a node is an n-bit word;
- an edge adds or removes one fact;
- layer k is the words with k bits set, C(n,k) of them.

Merging commitments that saturate to one state (sub-question (e)) gives a
different lattice, one node per distinct state, and that one is no longer a
cube.

**Without holes, two lines determine R, and |R| follows.** A set with no
holes is exactly every node between some minimal node and some maximal node
above it, so |R| is derived and says little on its own. What describes R is
its **sub-cubes**. An interval [a … b] is every set between a and b: the facts
in a are fixed in, those outside b are fixed out, and the rest are free. Its
**dimension** is |b| − |a|, the number of free facts. R is a union of maximal
sub-cubes, and |R| is their sizes by inclusion–exclusion (`branching/15`:
2 + 4 − 1 = 5).

**Components.** These are R's connected pieces under one-fact edges.
- They appear only when a program has both kinds of constraint. With
  restrictions only, R connects through ∅. With obligations only, it connects
  through the full set.
- Without holes, each component has its own two lines, and R's lines are their
  union.
- A component is not the count of solutions: `branching/15` has one
  component and two maximal nodes.
- **A well-posed puzzle is the case where every component is a single point**
  (dimension 0). There the component count, the minimal line and the maximal
  line all give the classical k.

**Holes.** A hole is a node outside R that lies between two members of R.
Under monotone rules there are none, because both *dead* and *discharged* are
preserved as facts are added. So every hole comes from `absent`.
*Corrected the same day: from `absent`, or from an obligation's trigger
([§ Holes without `absent`](#holes-without-absent-2026-10-15)).* A hole is a
failure that **adding a fact repairs**: in W3, `{(p A)}` dies only because
`(q A)` has not been said yet. It is a debt written as a restriction.
- **The outer lines are the answer.** Hole nodes are not solutions.
- **The hole count is a certificate.** Zero holes means the two lines describe
  R exactly, and that every learned nogood is sound.
- **A hole's minimal nodes are the useful diagnostic.** Each one is where a
  repairable failure starts, and its core names the `absent` guard responsible.
- **Its repair edges matter more than its maximal nodes.** A repair edge goes
  from a hole node up into R, labelled with the fact that revives it. In W3 it
  is `{(p A)} + (q A) → {(p A),(q A)}`.
- **Holes are where pruning is unsound.** `{(p A)}` is a minimal dead set with
  an alive superset. A learned nogood there prunes `{(p A),(q A)}`, which is a
  node of R. That is the whole of W3.

**The lines are decided by edges.** Without holes:
- a node of R is maximal exactly when every upward edge leads to a dead node;
- a node of R is minimal exactly when every downward edge leads to an owing
  node.

Both are local tests, and the search's layer n+1 already looks at those upward
edges. So **W1's fix is a local test on the cube**. Across a hole, a local test
and the true global answer can disagree.

**Two lists determine the answer when there are no holes.** Let N be the
minimal dead sets (the minimal nogoods) and E the minimal sets that discharge
every obligation. Then:
- the **minimal line** is the members of E that are alive;
- the **maximal line** is the largest sets containing no member of N, kept
  where discharged. Equivalently, it is the complements of the minimal sets
  that hit every member of N.

This is "bottom bound from obligations, top bound from restrictions", made
exact. It is the duality of Reiter's diagnosis theory and of MUS/MCS
enumeration (MARCO), and it is the route past n = 20: the lines can be
enumerated from N and E without visiting all 2ⁿ nodes. A dead state's debt is
not read today (`owes.root` says 0 on a dead root whatever it owes), so E
needs the oracle to record *dead* and *owing* as two independent bits.

**Symmetry.** Interchangeable constants, such as the two chairs, permute the
cube's coordinates, so R must map to itself under that permutation. An answer
that does not is a defect: W5's *chosen by name order* is one. An oracle can
also enumerate one node per symmetry class.

**Prior art: Poole's Theorist.** David Poole, *A logical framework for default
reasoning*, Artificial Intelligence 36(1), 1988
([doi:10.1016/0004-3702(88)90077-X](https://doi.org/10.1016/0004-3702(88)90077-X)).
The user's notes on it are `plans/followups/f19_pool_theorist/`. Theorist has
facts, a set H of possible hypotheses, and constraints.
- A **scenario** is a consistent set of hypotheses, and an **extension**
  follows from a maximal scenario. R's maximal line is the extensions that
  discharge every obligation.
- An **explanation** is a minimal scenario that entails the goal. R's
  minimal line is the explanations, with the obligations as the goal.
- Theorist's **skeptical / credulous** distinction maps onto a line:
  facts true at every node of it, or at some node. M1d's *certain core*
  ([representations.md](../../docs/history/m1d_satisfiability/representations.md))
  is the skeptical reading of a model set.

Measured with the merged `probes/region.py` (now [`utils/region.py`](../../utils/region.py)), same
programs and settings as the table above:

| program | components | maximal sub-cubes (dimension) | holes | minimal dead sets N | duality |
|---|---:|---|---|---|---|
| `branching/15` | 1 | [∅…{A}] 1, [∅…{B,C}] 2 | — | {A,B}, {A,C} | holds |
| `lattice/02` | 1 | [∅…pair] 2, three times | — | {a,b,c} | holds |
| `forced-2-two` | 1 | [∅…∅] 0 | — | {q one}, {q two} | holds |
| `owed-two-step` | 0 | — | — | {likes Ann Soup}, {likes Ann Stew} | holds (R empty) |
| `owed-seats` | 1 | [matching…all 4] 2, twice | — | — | holds |
| `owed-seats` + `functional` + `injective` | **2** | the two matchings, dimension 0 | — | the 4 clashing pairs | holds |
| `naf-upward-closure` (W3) | 1 | [∅…{q}] 1, [{q}…{p,q}] 1 | **{p}** | {p} | **fails**: the complement rule gives {q}, R's maximal line is {p,q} |

The duality holds on every program without holes and fails exactly on the
one with a hole.

### Holes without `absent` (2026-10-15)

Measured while writing the oracle. [`examples/lattice/04_obligation_trigger_hole.ein`](../../examples/lattice/04_obligation_trigger_hole.ein)
has no `absent` in any saturation rule, and `total-owed meal is-a` over a
`Diner` type that seating a guest derives:

```
$ python3 utils/region.py examples/lattice/04_obligation_trigger_hole.ein '(seats Ann C1)' '(meal Ann Soup)'
L0: ∅=ok
L1: {(seats Ann C1)}=owes 1  {(meal Ann Soup)}=ok
L2: {(seats Ann C1), (meal Ann Soup)}=ok
region: 3 nodes;  minimal: ∅;  maximal: {(seats Ann C1), (meal Ann Soup)};  holes: {(seats Ann C1)}
```

The owing node is a hole. An obligation is the negation of its witness, so
its **trigger** reads at the same polarity an `absent` does. A hole needs a
read that a commitment can change at non-monotone polarity. An `absent` in a
saturation rule is one such read, and an obligation's trigger is the other.
An `absent` over a relation no commitment reaches is neither, which covers
zebra2's spatial `absent`s and the stdlib's `is-a` scans. The classification,
its theorem and the detector are
[P1e2.1b § What non-monotone means](p1e2.1b_non_monotone_programs/README.md#what-non-monotone-means-and-what-is-not)
and [S1e2.1a.1](p1e2.1a_the_solution_region/s1e2.1a.1_the_oracle.md).

**The non-monotone case moved to a phase of its own** the same day, on the
user's answer to the oracle's first question: *"move non-monotonic solution
question into phase p1e2.1b"*. Sub-question (d) moved with it.

### Sub-questions, for [P1e2.1a](p1e2.1a_the_solution_region/README.md)'s first stage

- **(a) What does `k` count?** The minimal line, the maximal line, both
  (two counts), or the informative one by the program's shape? And which
  verdict word goes with which? Since the follow-up above, `k` may be replaced
  by a descriptor:
  - the component count;
  - for each component, its two lines and its sub-cube dimensions;
  - the minimal holes.

  The question is then which of these the verdict line prints.
- **(b) How does a program say which line it asks for?** By its shape
  (obligations stated or not), or by a query keyword the language does not
  have yet?
- **(c) How does `:expect` state a region?** Today it states a model set,
  one line's worth.
- **(d) What is reported when R has holes?** *Moved to
  [P1e2.1b](p1e2.1b_non_monotone_programs/README.md)'s
  [S1e2.1b.3](p1e2.1b_non_monotone_programs/s1e2.1b.3_holes.md) on 2026-10-15,
  and deferred to [F21](../followups/f21_non_monotone_programs_decided.md)'s D4
  on 2026-11-02, unruled: S1e2.1b.3 drafts the options and their
  consequences.* Both lines no longer describe
  R. Is the answer then the list of R's maximal convex pieces, or a
  qualifier, or a refusal (Q-M1e2.2's *refuse what cannot be stratified*)?
  The follow-up's recommendation: the outer lines stay the answer, and the
  read-out adds the minimal holes, each with the `absent` guard of its core
  and its repair edges. A hole is never counted as a solution.
- **(e) Commitments or states?** A node is a commitment, but a model is a
  state, and two commitments can saturate to one state. Q-M1e.6's
  *integrated* hypotheses are the same question. Is R deduplicated by state
  before its lines are taken?
- **(f) The trivial line.** The obligations-only maximal line can be
  exponentially far down, and the user called it trivial. Is it reported,
  named as trivial, or left out?
- **(g) What `exhausted` certifies.** The lines reported, not the walk
  ([Q-M1e.8](../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)).
- **(h) The universe.** R is over the generator's candidate list at root.
  W4 (a fresh constant never becomes a candidate) and W6 (relations never
  hypothesised) are cases where that list is too small. The oracle checks the
  walk against the list. Does R's definition also say what the list must
  contain? There is also a conflict with the config principle.
  [`solution_semantics.md` § 1](../../docs/kernel/inference/solution_semantics.md)
  defines `alive₀` *after the forced-positive cascade*, and the cascade is a
  config switch (`:enable-forced-positive`). So today the universe itself
  depends on config.
- **(i) Components.** Is the answer reported per component? Is a component
  the unit an `Ambiguity` is about, given that in a well-posed puzzle the
  components are the models?

### The oracle's limit (ruled 2026-10-15)

The lattice has 2ⁿ nodes. The user's words: *"we can safely limit n with 16
by default and warn user simply if asked for 16<=n<=20 and show intensive
warning if user asks to go through lattice with n>=20"*. The two ranges
overlap at 16 and at 20, and the plan reads them as follows:

| n | behaviour |
|---|---|
| ≤ 16 | runs, silently |
| 17–19 | refused unless asked for; when asked, runs with a one-line warning |
| ≥ 20 | refused unless asked for; when asked, runs after a prominent warning that states 2ⁿ and the projected cost |

### The rulings, and the questions back (2026-10-16)

The user's answer to the sub-questions, verbatim. The part of the same message
that adds a stage to P1e2.1b is quoted in
[S1e2.1b.6](p1e2.1b_non_monotone_programs/s1e2.1b.6_monotone_and_non_monotone_programs_compared.md),
and the W11 sentence in
[S1e2.2.12](p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md#the-verdict-moves-under-a-lever-2026-10-16).

> What `solve` could answer? for example "does alive and discharged solutions
> exist?" - it's enough to prove at least one; "is it the only single
> solution?"; "how many different solutions exist?" it is |R|; is
> minimal/maximal solution trivial? if not how many minimal/maximal solutions
> exist? sub-cube dimentions; number of components; enumerate all/min/max
> solutions; is the problem/solution/program monotonic? is it sinlge
> component? what else could `solve` answer about solution region topology?
> our default behaviour that searches for maximal solutions is not completely
> wrong but it is relevant only in special case with no obligations. So it has
> to be reworked.
>
> In general non-monotonic case R is completely defined only by full set of
> nodes. Is that correct? Let's go from it, and simplify the results if our
> case allows or if it is reqested explicitely. (a) Let's change the meaning of
> k now from |max(R)| to |R|. (b) first of all `ein solve` study the solution
> region topology, and quering some specific subset of like min line or max
> line is secondary, more than that query goal could become incompletely
> specified with some solution, also query is not the best place for hrules -
> need to split solution-related features like hrules from queries, we solve
> once and then queries answer questions about region and about specific
> solutions; Does it make sense? (c) :expect has to be the answer for query,
> new updated query, and another expect has to be a full expected region
> definition, maybe (solve :hrules ...) and (solution (or (model ...) (model
> ...) ...)) and (query :goal ... :expect <only facts from goal, query
> response, multiple models, deduplicated, maybe different models still
> produce identical goals>) ; (d) moved (e) probably we need this information
> in full solution that some commitments saturate to the same state, what does
> it mean in practice? we can easily saturate but yet can't so easily split,
> and don't know if smaller commitments are solutions if saturated is, so
> commitments give more information, but we need also saturation equvalence
> information, propose a way to expose all of it; (f) obligations only max
> line is a set of all hypothesis, that is why I called it trivial; is that
> true? (g) exhaused gives all found information about region, if limited by
> levels or iterations or time or anything - then information is not full, but
> if not limited then it is full information about region; region is a
> certificate of the lattice, right? (h) it is limitations of our current Ein
> - we don't emit new names, only combine existing, that guarantees lattice is
> finit; emitting new names/atoms will make everything even more complex, we
> can't handle it yet, and we don't know if we actually need it; If the
> hyothesis list is too small (too small for what? for producing expected
> solution?) - then ein program is wrong, or ein solve has a bug, isn't it? Or
> I didn't understand the question then explain better. I really forgot what
> "forced positives" are? (i) No, definitely components can't be models.
> Models are models. Components are non-intersecting subsets of models, e.g.
> {A} {B,C}; e.g. 2 groups of friends want to sit at the table, and all
> friends wants to sit at the same table, so if we have 2 tables we have 2
> components correspondign to group-table, and inside each component
> permutations of seat-friend; it can't be model, but query can ask e.g. only
> what table selected, and then seats will be hidden information; 'Ambiguity'
> is still relevant when we expect a one and only one solution.
> 'Contradiction' is relevant when we expect some solution. In other cases, by
> default, we are exploring, studying the solution region.

#### Where each sub-question stands

| | the user's answer | status | goes to |
|---|---|---|---|
| **general** | R is, in general, its full node set; start there and simplify where the case allows or a query asks | **ruled**. The plan's answer to *"is that correct?"* is below | S1e2.1a.3's page; [S1e2.1b.6](p1e2.1b_non_monotone_programs/s1e2.1b.6_monotone_and_non_monotone_programs_compared.md)'s ladder |
| **(a)** | `k` = \|R\|, not \|max R\| | **ruled**, and its unit too (second answer): `k` is replaced by **k_s** (states) and **k_c** (commitments), both reported | S1e2.1a.5, S1e2.1a.9 |
| **(b)** | `ein solve` studies the region first. Asking for one line is secondary. `hrules` move out of `(query …)`. Solve once, then queries ask about the region and about single solutions | **ruled**. The plan's answer to *"does it make sense?"* is below | S1e2.1a.4; [Q-M1e2.8](#q-m1e28--where-does-the-rework-of-ein-solve-go) for the search |
| **(c)** | three forms: `(solve :hrules …)`; `(solution (or (model …) …))`, the full expected region; `(query :goal … :expect …)`, the goal's projection, deduplicated | **ruled** as the direction. The grammar is S1e2.1a.4's page diff | S1e2.1a.4 |
| **(d)** | moved | P1e2.1b; **deferred to F21's D4, 2026-11-02** | [S1e2.1b.3](p1e2.1b_non_monotone_programs/s1e2.1b.3_holes.md) (evidence: [`hole_census.md`](p1e2.1b_non_monotone_programs/hole_census.md)); [F21](../followups/f21_non_monotone_programs_decided.md) |
| **(e)** | asks for a way to expose saturation equivalence alongside commitments | **ruled**: k_s and k_c (second answer); `(region …)` with `(merged (to from) …)` (third answer) | S1e2.1a.1's record; S1e2.1a.4, S1e2.1a.5 |
| **(f)** | asked: *is the obligations-only maximal line the whole candidate set?* | **ruled for P1e2.1a's scope** (second answer): yes, for a monotone program, and it is trivial. The non-monotone counterexample below is S1e2.1b.6's | S1e2.1a.3's page |
| **(g)** | `exhausted` means complete information about R; a limited run gives partial information; *"region is a certificate of the lattice, right?"* | **ruled for P1e2.1a's scope** (second answer): exhausted = the full region; a limit makes it partial | S1e2.1a.5, S1e2.1a.9 |
| **(h)** | no new names, so the lattice is finite; asks what *too small* means; asks what forced positives are | **ruled for P1e2.1a's scope** (second answer): the universe is the candidate list, and a list too small for a model is a program error or an engine bug. ∅ is a solution exactly when root is alive and discharged, whatever the generator proposes | S1e2.1a.3; W4, W6; W15 (S1e2.2.14) and W14 (S1e2.2.15, the obligations rung's universe, [Q-M1e2.10](#q-m1e210--when-the-obligations-rung-cannot-trust-its-domain-scan-whose-candidates-is-r-over) (a)), both fixed 2026-11-04 |
| **(i)** | a component is a set of models, never a model. `Ambiguity` judges an expectation of exactly one solution and `Contradiction` an expectation of some. Without an expectation the read-out explores | **ruled**, and the user's grouping is named **projection classes** (second answer) | S1e2.1a.5; [`definitions.md`](p1e2.1a_the_solution_region/definitions.md) |

#### Measured for these answers

Taken 2026-10-16 at `777d224a`, by running every `.ein` under `examples/`,
`tests/` and `corpus/` (242 files) through `ein solve FILE --dump-region DIR`,
with a 60 s timeout each. 154 regions were computed, 29 were refused by the
limit, and 59 files produced none (they failed to load or timed out).
T1e2.1a.3.1's census re-took all of it the same day, over the corpus and the
register rather than every file, and replaces this table:
[§ The census](#the-census-2026-10-16).

| | regions |
|---|---:|
| R non-empty | 118 |
| **∅ ∈ R**: no obligation that root owes | **107** |
| exactly one state in R | 68 |
| exactly one maximal node | 103 |
| … of which R holds more than one state | **35** |
| R counted in commitments ≠ R counted in states | **14** |
| flagged non-monotone by the static check | 13 |

Of the 14 where the two counts differ, the largest is
`features/02_star_in_identifiers.ein`: 32 768 commitments and 728 states. On
`branching/05_mini_zebra.ein` it is 8 and 5.

#### The plan's answers

**General: is R, in general, its full node set?** Yes.

- A non-monotone R can be any subset of the cube. S1e2.1a.8's synthesizer
  writes a program for any target (2 100 random targets, 0 mismatches), except
  a dead ∅ under a non-empty R.
- There are 2^(2ⁿ) possible regions, so no shorter description exists in
  general.
- *Full set* still has an exact compact form, the union of R's maximal
  sub-cubes, which the oracle already computes.

The simplifications follow in order, each licensed by one property:

1. **Convex** (no holes): the two lines are exact.
2. **Monotone** (the static check): the lines follow from N and E without
   enumerating, and a certificate is N + E.
3. **Shape**: restrictions only gives min = {∅}; obligations only gives
   max = {⊤}.

S1e2.1b.6 writes this ladder down.

**(a) × (e): what `k` = |R| counts.** The two measured facts above bear on it.

- **Commitments against states.** On 14 corpus regions, R counted in
  commitments is larger than R counted in states, up to 45× (32 768 against
  728). A puzzle that states its completeness as obligations (`bijective`,
  `total`) is the extreme case. Its unique model is reached by every
  commitment that saturates to it, so R in commitments counts the ways to
  *guess* the model, not models. This is argued, not measured: zebra2's cube,
  with n = 56, is out of reach. **The plan recommends k = the number of states
  in R, with the commitment count printed beside it.**
- **Programs that state no obligation.** ∅ is in R on 107 of the 118 non-empty
  regions. In a program with restrictions only, every consistent partial
  commitment is a solution, as the user's W1 topology `{∅, A, B, C, (B,C)}`
  has it. So k = |R| is large there even when the maximal line is a single
  node: 35 regions have one maximal node and several states. For example,
  `syntax/constraint-scopes.ein` has 4 096 states. This is what (a) means and
  is not a defect. It does mean that *"is it the only single solution?"* has
  two forms:
  - |R| = 1, for a program that states what it owes;
  - |max R| = 1, for one that does not.

  An expectation of exactly one solution, (i), has to say which. The
  alternative is that a puzzle states its completeness as obligations, and
  then R collapses to its models.

**(b) Does it make sense?** Yes.

- It matches what the engine already depends on. The lattice is fixed by the
  candidate list, which `hrules` select, and a query's goal is a projection of
  an answer. Today `:hrules` sits on `(query …)`: 19 corpus files put it there
  (outside comments, 2026-10-16). In principle, two queries could then define
  two lattices. The split makes *one program, one lattice, one R* structural.
- *"incompletely specified"* is real. In a program with restrictions only, the
  goal has no row at ∅. Today's read-out prints that case as *(no query goal to
  project)* (inventory S6). A query over R therefore has to say *undetermined
  in these solutions*, or be asked over a named subset, which (b) makes
  secondary.

**(c)** Two notes for S1e2.1a.4's grammar.

- `(solution …)` listing every model is exact, but it explodes: 728 states on
  `features/02`. It should also accept region shapes:
  - a sub-cube, `[a … b]`;
  - the two lines, with a `:convex` claim that the runner checks.
- A `(model …)` inside `(solution …)` should name a **state**. *Naming a
  relation closes it* carries over unchanged.

The query's `:expect` over the goal's projection, deduplicated, is the same
notion as (i)'s *what the table is* below.

**(e) A proposal.** For a monotone program, saturation is a closure operator
on the cube:

- **extensive**: C ⊆ cl(C);
- **idempotent**: cl(cl(C)) = cl(C). Measured: 0 of 32 074 moving alive nodes
  land on another state (S1e2.1a.2);
- **monotone**: C ⊆ D ⇒ cl(C) ⊆ cl(D). This follows from the theorem.
  S1e2.1b.6 adds the test.

So each state s is a class `{C : cl(C) = s}`:

- **its top is s itself**, the closed set;
- **its bottoms are its minimal generators**: the smallest commitments that
  force s;
- **C ∈ R ⇔ cl(C) ∈ R.** This is the easy direction the user named: whether a
  smaller commitment is a solution takes one saturation.
- **The hard direction, *split*, is the minimal generators.** They are what
  has to be guessed to reach s. The size of the smallest one is the number of
  hypotheses the solution needs, which is the branch depth
  [`zebra_walkthrough.md`](../../docs/kernel/inference/zebra_walkthrough.md)
  tabulates.

**What to expose:**

- per state in R: its closed set, its minimal generators, and the size of its
  class;
- `k` counted in states, with \|R\| in commitments beside it;
- the merged diagram: the closed sets ordered by inclusion, as an `ein render
  region` view. Closed sets are closed under intersection, so it is a lattice.
  This is formal concept analysis's closed sets and minimal generators (Ganter
  & Wille, 1999).

Today the oracle keys a class by its numerically smallest mask. The proposal
keys it by its closed set, which is unique. For a non-monotone program the
classes still exist, but cl need not be monotone. That cell belongs to
S1e2.1b.6.

**(f) Yes, for a monotone program.** If nothing dies and *discharged* is
upward-closed, the full candidate set ⊤ is in R whenever R is non-empty, so the
maximal line is {⊤}. `owed-seats` (⊤ = every guest on every chair) and
`examples/lattice/synth/trivial_line.ein` show it.

**It is not true when an obligation's trigger can grow.** A probe written with
`utils/region_synth.py` on 2026-10-16 (target: candidates `ABC`; R = `[A … AB]`;
everything else owing; nothing dead) passes `--check`. Its maximal line is
`{A,B}`, and ⊤ = `{A,B,C}` owes, because `C` triggers a debt that nothing
pays. The static check flags it.

The dual also holds: with restrictions only, the minimal line is {∅}. Both
trivial lines are visible statically from the program's shape. The plan
recommends naming them in one word and never enumerating or chasing them.

**The minimal line's name withdrawn, 2026-11-07** (the inventory's S-17 and
F1). As built, the read-out named {∅} *the trivial line: no obligation is
stated* where no node the run met owed, so it said so of programs that state
an obligation root discharges, and a stopped run said it or not by its
`--region-limit`. The user ruled (a), *drop the clause*: the row reads
`1 state — ∅`, the `∅` row above it gives the reason, and the summary's
`minimal.trivial` reads *∅ is in R*. The maximal line's name, printed only on
an exhausted run, stays. **Not built**:
[S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md).

**(g) Yes, with one correction to today's word.**

- *Today* `exhausted = true` certifies the **walk**, not R
  ([Q-M1e.8](../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)).
  W1 is a run that is `exhausted` and still misses a node of R.
- Under the ruling, `exhausted = true` means *the reported region is R*.
- A limited run reports what it found as a lower bound, and every node in it
  is checked.

*"Region is a certificate of the lattice"* reads the other way round in the
plan's terms. **R is the answer.** A *certificate* is what lets a reader check
a claimed R without recomputing it:

- **monotone**: small. It is N and E, one saturation per member, plus the
  static verdict;
- **non-monotone**: in general nothing smaller than every node's
  classification.

**(h) The question, explained.**

- **The universe is the candidate list.** R is defined over the candidates the
  generator offers at root, and **too small** means *too small to contain
  every model of the program*: a fact some model needs is not a candidate, so
  that model is not a node at all. There are two ways today:
  - **W4.** A rule asserts a constant the ontology lacks.
    `examples/ein-bugs/alive-set-fresh-name.ein` is eleven lines on which that
    costs a verdict. Under the user's rule that Ein emits no new names, that
    program is **ill-formed**. The consequence the plan draws is that the
    alive-set invariant, today a `warn` line, becomes a load error.
    *Overruled by the [third answer](#the-third-answer-2026-10-16):* rules are
    part of the program, so the name is not new, and W4 is an engine bug.
  - **W6.** A relation the program could have in a model is never
    hypothesised, such as `knows` in `completeness.md` §3c. Whether that is a
    bug depends on what the list is *supposed* to hold.

  **The ruling sought:** the universe is defined declaratively, as every
  type-correct ground atom of every hypothesisable relation over the declared
  objects. The generator's list is checkable against it, and a shortfall is an
  **engine bug**. A program that needs a name outside it is a **program
  error**.
- **The second half of (h): the universe must not depend on config.**
  `solution_semantics.md` § 1 takes `alive₀` *after* the forced-positive
  cascade. The oracle already uses the config-free list. The ruling sought is
  that the page defines the universe that way too.

**Forced positives.** When exactly one candidate hypothesis is still alive at
root, because every other one has been refuted, the engine promotes it into
root as a **fact**. Its provenance is `<forced-positive>` with no premises.
Then root is re-saturated, and the step repeats while one candidate remains.
The code is `solve.rs`'s `promote_forced_positives`, behind
`:enable-forced-positive`, which is on by default. The reasoning behind it is
*"every model extending root must contain it"*. That holds only if ∅ is not
itself a solution. W2 is the case where ∅ is the solution: on `forced-2`, ∅ is
the whole of R, the cascade promotes the one candidate, root dies, and the
engine answers `Contradiction`. Under the config principle, a forced positive
can be at most a search-time shortcut. It can never define the universe or the
answer.

**(i) Agreed, with one clash to resolve.** The oracle computes a component as
a set of nodes of R connected by one-fact edges. Under that definition, the
two-tables example does **not** give one component per table.

- With seating that is `functional` and `injective`, every full seating is an
  isolated node. Swapping two guests passes through a node with a guest in two
  seats, which is dead, or a node with a guest in none, which owes. So each
  seating is a component of its own.
- That is measured: `owed-seats` with `functional` and `injective` has
  **2 components**, the two matchings, each of dimension 0, as does
  `synth/one_model_per_component`.

The grouping the user describes, *models that agree on the table*, is the
partition a **query's goal** induces: the user's *"seats will be hidden
information"*, and (c)'s deduplicated projection. **The plan recommends** that
*component* keep its topological meaning, and that the user's notion be named
**projection classes**: a query's answers, each class being the models that
give one goal row set. A query answers with projection classes. Components
describe R's shape.

### The second answer (2026-10-16)

The user's reply to the plan's answers, verbatim:

> "when exactly one candidate is still alive at root, the engine promotes it
> to a root fact and re-saturates" - how does it differ from handling other
> previous candidates? engine also take them and add to root and saturate.
> What the differense for the last one alive? Empty set is a solution if
> sat(root) is already discharged and alive, it doesn't depends on hypgen at
> all. More than that - there could be no hypothesis at all, e.g. hrules
> doesn't match anything. But root could be already a solution. Trivial but
> valid case. (i) Provide me a definition table, in a file, what is fact, kb,
> state, model, commitment, solution, region, component, ... etc. Yes, what I
> described is a projection classes. […] (h) what is "alive-set invariant" ?
> In W11 (rule fire-0 (?P) ...) ?P variable not used. We are now at p1e2.1a,
> where solve will be changed and expect will be changed, and k changes it's
> meaning, but you keep asking about W11 k=0 vs k=2; It doesn't make sense
> now. We will revisit W11 twice after the refactoring. Once at
> plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md and
> again at plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md;
> k unit - k_states and k_commitments are different numbers, we need both,
> replace k with k_s and k_c; (f) (g) (h) - Again, we are in p1e2.1a - we
> solve monotone problems here, will solve non-monotone in next phase;
> Q-M1e2.8 - yes

and, in the same turn:

> change solve behaviour; the default simple `solve` without flags we get
> exhausive certifying full region description; while with some flags can
> simplify the task - e.g. look for any solution, or expect there is a single
> solution - so k>=2 is a contradiction; or look for only maximal solutions,
> or only for minimal solutions

The elided part asks for the seating example, which is banked as
[`examples/lattice/06_friends_tables.ein`](../../examples/lattice/06_friends_tables.ein),
with [`07_two_pairs_two_tables.ein`](../../examples/lattice/07_two_pairs_two_tables.ein)
beside it.

What it rules:

| | ruling | where it lands |
|---|---|---|
| **∅** | ∅ is a solution exactly when `sat(root)` is alive and discharged. That does not depend on the generator, and it holds when the generator proposes nothing | `definitions.md`; S1e2.1a.3's page; S1e2.1a.9 |
| **(a), (e)** | `k` is replaced by **k_s** (\|R\| in states) and **k_c** (\|R\| in commitments), both reported | S1e2.1a.5, S1e2.1a.9 |
| **(f), (g), (h)** | answered for the **monotone** case, which is P1e2.1a's scope. The non-monotone halves are P1e2.1b's | S1e2.1a.3; S1e2.1b.6 |
| **(i)** | the user's grouping is **projection classes**, and a definitions table exists: [`definitions.md`](p1e2.1a_the_solution_region/definitions.md) | S1e2.1a.4, S1e2.1a.5 |
| **W11** | not a P1e2.1a question. It is revisited twice after the refactoring: in [S1e2.1.7](p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md)'s review, then in [S1e2.2.12](p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md) | those two stages |
| **Q-M1e2.8** | yes: S1e2.1a.9 now, P1e2.1c next | [Q-M1e2.8](#q-m1e28--where-does-the-rework-of-ein-solve-go) |
| **`solve`'s default** | `ein solve` with no flags gives an **exhaustive, certifying, full description of R**. Flags make the task simpler: **any** solution; **expect a single solution**, so that k ≥ 2 contradicts the expectation; **maximal** solutions only; **minimal** solutions only | S1e2.1a.9 |

**The forced positive, answered.** An ordinary candidate is **entered**. The
search forks root, writes the candidate into the fork, saturates, and judges
that one node. Root does not change, and the node is one of many.

A forced positive is written **into root itself**, as a fact with no premises,
and root is re-saturated. From then on every node holds it, and the node
without it, ∅, is gone from the walk.

The *last alive* condition is the warrant: *if a solution must add a
candidate and all but one are refuted, every solution holds that one.* That is
sound only when ∅ is not itself a solution, which is the user's point. Probes
taken 2026-10-16:

- **No candidate at all.** The `hrule` matches nothing. `solve` answers
  `Solution`, model ∅, so the trivial case is handled.
- **Two alive candidates** that exclude each other. The engine answers
  `Ambiguity`, k = 2. R is `{∅, {(p A)}, {(p B)}}`, and ∅ is missing.
- **One alive candidate**, the other dead alone. The cascade promotes `(p A)`
  into root, and the engine answers `Solution` `{(p A)}`. R is
  `{∅, {(p A)}}`, and ∅ is missing.

So ∅ is recorded only when the generator proposes nothing. That follows from
today's **complete**, *the active rung proposes nothing*, which is a property
of the generator. Under the ruling it is replaced by **discharged**, a
property of the state. W1's layer-0 row is the same defect.

**The alive-set invariant (h).** It is the M1 kernel's assumption that rules
assert no new objects, no new relations, and that hypotheses connect only
existing names. From that, the alive set, and with it the candidate list, is a
function of root alone. Three mechanisms rely on it: the per-KB recompute of
`alive`, the `state_key` dedup behind `k`, and the tree traversal's
exhaustiveness. Since M1e S1e.3.3 `ein_infer::invariant` checks its static
half at load: every constant a rule's `:assert` names must be in the program's
facts. A breach is a `warn` line, not an error.
`examples/ein-bugs/alive-set-fresh-name.ein` is the eleven-line breach that
costs a verdict. *Corrected by the third answer:* rules are part of the
program, so the name that rule asserts is a program name. The program is
well-formed, and the missing candidates are an engine bug (W4).

### The third answer (2026-10-16)

The user, verbatim:

> in examples/lattice/06_friends_tables.ein if group doesn't fit - group
> doesn't sit (e.g. go to another restraunt); So not total-owed, but local
> group owed; WHY we still need to write someting into the root, when those
> forced positives arrive??? Alive-set invariant question - rules are part of
> root, so rules can't assert new names. "display which commitments saturate
> to the same state" - yes, full region info shows all valid commitments and
> all states that have multiple generating commitments with minimal ones;
> something like (region (model ...) (model ...) ...) (merged (to from) (to
> from)))

| | ruling | the plan's note | where it lands |
|---|---|---|---|
| **root is not the search's to write** | the user's question: why write into root at all when a forced positive arrives? | **There is no need.** Root is `sat(program)`, and ∅ *is* root, a node like any other. In a monotone program, if every candidate but h is dead alone, every node holding another candidate is dead, so the alive nodes are at most ∅ and {h}. Two saturations then decide the whole region, and nothing has to be written. Promoting h into root decides nothing more, loses ∅, and makes the outcome depend on a config switch (`:enable-forced-positive`, and the lookahead that decides what *sole survivor* means). **Proposed: root is immutable. The search keeps what it learns in its own state.** The search's negative writes into root fall under the same principle, and they are parked with W11 | S1e2.1a.9 (route 1 writes nothing into root); [S1e2.2.2](p1e2.2_wrong_answers/s1e2.2.2_forced_positive_refutes.md) (W2); P1e2.1c's [S1e2.1c.5](p1e2.1c_the_region_search/s1e2.1c.5_wrong_answers_re_scoped.md) |
| **(h), the alive-set invariant** | rules are part of the program, so a name a rule asserts is a name of the program, never a new one | This **corrects the plan's second answer**, which called `alive-set-fresh-name.ein` ill-formed. It is well-formed. `Z` is a program name, the universe has to include it, and the missing candidates are an **engine bug**. That is Q-M1e.21's option (b): seed the names from the rules' `:assert` constants | [S1e2.2.4](p1e2.2_wrong_answers/s1e2.2.4_fresh_name_unreachable.md) (W4), **built 2026-11-04**; widened the same day to every name of the program, a `:match` constant and a nested fact's argument included, by [S1e2.2.14](p1e2.2_wrong_answers/s1e2.2.14_unread_names.md) (W15, [Q-M1e2.9](#q-m1e29--which-constants-are-names-of-the-program) (a)); `definitions.md` |
| **(e), how a state's class is shown** | the full region lists **every valid commitment**, and **every state with several generating commitments, with its minimal ones**. The sketch is `(region (model …) (model …) …)` and `(merged (to from) (to from) …)` | For a monotone program the sketch is **exact and compact**. A state's class is the union of the intervals [from … to] over its minimal generators: if g ⊆ C ⊆ s and cl(g) = s, then s = cl(g) ⊆ cl(C) ⊆ cl(s) = s. So `(region …)` listing the states and `(merged …)` listing their minimal generators names every valid commitment, and k_c follows by inclusion–exclusion. **Proposed:** the read-out prints this block, and the language reads it back as an expectation. A run's region can then be banked as its own claim | S1e2.1a.4, S1e2.1a.5 |
| **06's obligation** | a group that does not fit goes elsewhere. The obligation is the group's, not `total-owed` | **The program is then not monotone, by construction.** *Once a member sits, the others owe a seat* is an obligation whose trigger grows with the commitment, the second source of holes. The cube: R is 21 nodes (∅; the pair alone, 8; the trio alone, 4; both, 8), and **166 holes**, every partly seated group. The engine records 8. The natural way to write a conditional requirement makes trigger holes, so they are P1e2.1b's common case, not an exotic one. The everyone-sits version is kept as the monotone twin | [`06_friends_tables.ein`](../../examples/lattice/06_friends_tables.ein); S1e2.1b.6 |

### What `solve` can answer about R (2026-10-16)

The user's questions, then the plan's additions (marked +). The cost column
says what an answer needs:

- **witness**: one node and its saturation;
- **lines**: enumerating a line, via N and E when the program is monotone;
- **R**: all of R, or its sub-cube cover;
- **static**: the program alone.

| question | answered by | cost |
|---|---|---|
| does R have a node? | one node of R, checked | witness |
| is it the only solution? | \|R\| = 1, or \|max R\| = 1 (see (a)); a witness plus a proof of no other | lines |
| how many solutions? | `k` = \|R\| (in states and in commitments, (e)) | R, or the sub-cubes by inclusion–exclusion when convex |
| is the minimal / maximal line trivial? | min = {∅}, max = {⊤}, from the program's shape | static |
| how many minimal / maximal solutions, and which? | \|min R\|, \|max R\|, enumerated | lines |
| how many components; is it one? | components under one-fact edges | R |
| sub-cube dimensions | each maximal sub-cube's free facts | R |
| enumerate all / min / max | the node set, or one line | R / lines |
| is the program monotone? | the static check's sites, and the violations the cube observes | static, then R |
| + if R is empty, which kind and why? | no alive node: a core. Every alive node owes: the debts no alive extension pays | witness per kind |
| + does it have holes? | hole count, minimal holes with their guard and repair edges | R |
| + which commitments give one state? | per state: its closed set and minimal generators (e) | R |
| + how much must be guessed? | the smallest generator's size per state | R |
| + which facts hold in every solution, in some, in none? | backbone, credulous and impossible sets: Theorist's skeptical and credulous readings | lines, for a monotone program |
| + which facts decide the others? | per-fact counts; M1d's determining key (`--models key`) | R |
| + what does a query's goal see? | projection classes (i), deduplicated (c) | R |
| + which rule shapes which part of R? | per minimal nogood its rule chain; per minimal owing node its obligations | lines |
| + which rules or facts can be dropped without changing R? | redundancy: R recomputed without each | R per rule |
| + what would make R a single state? | the smallest set of facts or rules whose addition leaves one state, and per candidate fact how far it splits R | R |
| + is R symmetric? | R mapped onto itself under interchangeable constants. An answer that is not is a defect (W5) | R |
| + how far apart are the solutions? | the minimum Hamming distance between states | R |
| + is the answer complete? | `exhausted` (g), and the certificate | — |
| + what is the universe? | the candidate list, and where it differs from `alive₀` (h) | static |

The questions marked **R** are the ones the user named for improving a theory
and for searching for one. They are why the region is the object of study and
not its first node. Which of them `ein solve` answers by default, and which
behind a flag, is S1e2.1a.5's to propose and the user's to rule. **Ruled
2026-10-16**
([`read_out.md` § 11](p1e2.1a_the_solution_region/read_out.md#11-the-users-rulings-2026-10-16)):
*"bare ein solve prints as much details as possible, except full lattice"*,
which is that page's § 10 *by default* column as proposed.

### The census (2026-10-16)

[S1e2.1a.3](p1e2.1a_the_solution_region/s1e2.1a.3_the_definition.md) put the
corpus beside the rulings: `utils/region_census.py --all --levers`, 159
regions computed, written up in
[`region_census.md`](p1e2.1a_the_solution_region/region_census.md). **No
measured fact contradicts a ruling**
([§ 11 there](p1e2.1a_the_solution_region/region_census.md#11-what-each-ruling-meets-in-the-census)).
The numbers each ruling meets:

- **general**: holes only on programs the static check flags, 5 of 13;
- **(a)**: on the 51 non-empty monotone regions with a candidate, k_s = 1 on
  12, and 25 have one maximal node and several states;
- **(a) × (e)**: k_c ≠ k_s on 13 regions, up to 32 768 against 728;
- **(f)**: restrictions only gives min = {∅} on 25 of 25, obligations only
  gives max = {⊤} on 5 of 5;
- **(h)**: alive₀ ≠ H on 29 programs, every dropped candidate a dead
  singleton, every promotion a lost ∅;
- **∅**: in R on 41 of the 51, and every exception owes at root;
- **the config principle**: no lever changes R on 158 of the 159, and
  `enable-symmetric-mirror` changes it on `features/06`.

93 of the 146 monotone cubes have no candidate at all, which is why the
counts above leave them out; the first take above did not.

## Q-M1e2.8 — Where does the rework of `ein solve` go?

The user, 2026-10-16: *"our default behaviour that searches for maximal
solutions is not completely wrong but it is relevant only in special case with
no obligations. So it has to be reworked"*. And under (b): *"first of all `ein
solve` study the solution region topology"*.

[P1e2.1a](p1e2.1a_the_solution_region/README.md) does not make the search find
R ([§ Where this phase stops](p1e2.1a_the_solution_region/README.md#where-this-phase-stops-and-p1e22-starts)),
and [P1e2.2](p1e2.2_wrong_answers/README.md) fixes the walk one case at a
time. Neither is a rework. There is also a constraint: under (a),
S1e2.1a.5 has to report `k` = |R|, and the walk cannot count R. It records
complete nodes and does not expand them.

The options:

1. **In P1e2.1a, now: the oracle becomes `solve`'s answer wherever it
   reaches.** This would be a new stage, S1e2.1a.9.
   - At n ≤ the limit, `ein solve` computes R exactly. That is the general
     case, stable and proved by construction. The corpus's 150 cubes take
     20 s in total (S1e2.1a.1).
   - Above the limit, the walk runs, and the read-out says the region is
     partial.
   - Before this can go into the gate, its cost in `cargo test`'s corpus sweep
     has to be measured.
2. **A phase of its own, P1e2.1c, after P1e2.1b and before P1e2.2:** the region
   search built from S1e2.1b.6's strategies (dualisation of N and E for a
   monotone program, the guard split otherwise). P1e2.2 is then re-scoped
   against it, because W1, W2 and W5 are symptoms of walking for the maximal
   line and may close by construction.
3. **After M1e2.** P1e2.2 fixes the walk as planned, and the rework is a
   milestone of its own.

**The plan recommends 1 now and 2 next.** Option 1 is what lets S1e2.1a.5
print the ruled `k` truthfully on everything the oracle reaches. Option 2
decides whether P1e2.2's 27.5 days are spent on a walk that is about to be
replaced.

**Ruled 2026-10-16: yes, 1 now and 2 next.** The same turn ruled what the
default is: *"the default simple `solve` without flags we get exhausive
certifying full region description"*, with flags that simplify the task.
Option 1 is [S1e2.1a.9](p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md),
and option 2 is [P1e2.1c](p1e2.1c_the_region_search/README.md).

## Q-M1e2.9 — Which constants are names of the program?

Put to the user on 2026-11-04, after S1e2.2.4 seeded the blind enumerator's
objects with every constant a rule's `:assert` names (Q-M1e.21 (b)).
[`solution_semantics.md` § 1.2](../../docs/kernel/inference/solution_semantics.md)
says *a name is a constant the program mentions, in a fact or in a rule*, and
the engine's candidates were narrower: the arguments of the KB's top-level
facts, plus the `:assert` constants. A read-only probe at `16a05caf`
(`build/s1e2.2.4-probe/r2/`, `r3/`) found three kinds of constant the program
writes that never become a candidate:

1. **only in a rule's `:match`**, e.g. `(q ?a Z) → (good ?a)`. No corpus
   program has one.
2. **only inside a nested fact**, e.g. `(not (q Z A))`. Declaring each such
   name moves R on seven corpus programs: the three
   `hypothesis-contradiction/*-disproved` and W7's `binding-key-layout` pair
   (k_s 1 → 2 each), `syntax/equality.ein` (k_s 64 → 4 096) and
   `syntax/arg-kinds.ein` (H 19 → 41, both runs partial).
3. **only in a `(query …)`**: `syntax/query-full.ein`'s `Water` and
   `ein-bugs/query-goal-free-head-unsat.ein`'s `b`, neither of whose R moves.

The options were (a) § 1.2 literally, kinds 1 and 2 are names and 3 is not;
(b) all three; (c) today's set, and § 1.2 amended to say it.

**Ruled 2026-11-04: (a)**, *§ 1.2 literally*. Every constant written in a fact
at any depth, or anywhere in a rule (`:match` and `:assert`), is a name of the
program and so a candidate object. A constant written only in a query is not:
R is the program's, and a query asks about it, so adding a query never changes
R. Under ruling (h) of [Q-M1e2.7](#q-m1e27--the-answer-is-a-region-of-the-lattice)
a candidate list that lacks a name a model needs is an engine bug, so kinds 1
and 2 are a wrong answer, filed as **W15** and fixed by
[S1e2.2.14](p1e2.2_wrong_answers/s1e2.2.14_unread_names.md), which runs
before [S1e2.2.7](p1e2.2_wrong_answers/s1e2.2.7_binding_key_layout.md)
because W7's two witnesses are among the seven.

**Built 2026-11-04** by [S1e2.2.14](p1e2.2_wrong_answers/s1e2.2.14_unread_names.md):
`invariant::program_names` is the one list of the program's names (every
constant in an argument position of a fact, at any depth, and every constant
in a rule's `:match` or `:assert`; no query is read), and
`hypgen::candidate_objects` unions it into the names index. W15's eight rows
are fixed, each witness answering its declared twin's R and the oracle's;
`syntax/arg-kinds` moves too (H 19 → 41) and has no row, its R a lower bound
both ways. W7's `right` is restated to R under the ruling. The stdlib writes
no constant in a rule's argument position, so no import moved a program's
names.

**A string, 2026-11-04** ([S1e2.2.18](p1e2.2_wrong_answers/s1e2.2.18_strings_in_rules.md),
W18): the ruling unchanged, read through a string. A string a rule writes is
the symbol it names, as a fact's is, so it is a constant in a rule and a name
of the program, as that atom is; the list did not read one until then, and
the compiler did not lower one. A string that is a keyword's value inside a
clause is text, and not a name. The fix moves the names of two corpus
programs, W18's `str-in-assert` and `str-in-hrule` (`lbl`), and of no other.

## Q-M1e2.10 — When the obligations rung cannot trust its domain scan, whose candidates is R over?

Put to the user on 2026-11-04 by
[S1e2.2.15](p1e2.2_wrong_answers/s1e2.2.15_obligations_rung_universe.md)'s step 1
([T1e2.2.15.1](p1e2.2_wrong_answers/s1e2.2.15_obligations_rung_universe.md#task-t1e22151--witnesses-banked-w14-filed-every-move-listed)).
W14: the obligations rung builds H by running each obligation guard's domain
scan over root's KB, so a domain value only a commitment derives is in no
candidate, and R is empty or owing where a model exists. Q-M1e2.7's ruling (h)
makes that an engine bug, and the stage's default fix is
[`domain_contract.md`](../../docs/history/m1d_satisfiability/domain_contract.md)'s
C4 as it is written, *where the guard's relation can grow, the rung declines
and falls through to the blind generator*, with *can grow* read as D, the
relations a commitment can change ([`monotone.rs`](../../ein.rs/crates/ein-infer/src/monotone.rs)).

What step 1 found: D is at relation grain (the user's ruling of 2026-11-02,
S1e2.1b.1 (a)), so the decline takes each witness's `-declared` control too.
The control names the value at root, but its guard reads the same relation,
and that relation is in D whether or not root already holds the value. On
two of the four witnesses the blind enumerator's R is larger than the rung's
on the control, because the blind enumerator proposes every pair of an
untyped relation and the facts no rule refutes or requires are free:

| witness (`examples/ein-bugs/wrong-answers/`) | today | the control over the rung's candidates | the blind enumerator, the rung declined |
|---|---|---|---|
| `oblig-fresh-name.ein` | H 1, R empty | H 2, 1 state, `{(likes Ann Z)}` | H 6, the same state |
| `oblig-derived-domain.ein` | H 1, R empty | H 2, 1 state, `{(likes Ann Pie)}` | H 6, the same state |
| `oblig-fresh-name-two.ein` | H 2, R empty, `{(likes Bob Soup)}` owing | H 4, 3 states | H 12, **12 states**: `(likes Ann Bob)` and `(likes Bob Ann)` free |
| `oblig-stdlib-total-owed.ein` | H 1, R empty | H 2, 1 state | H 6, **16 states**: the four `likes` with a non-Person subject free (the blind enumerator is untyped, C1) |

Each cell is `ein solve` and `ein oracle` at `338b5892`; the last column is
the witness with a bare `(open)` rule added, which declines the rung today
(`build/s1e2.2.15/blindprev/`), and a prototype of the decline answers the
same on the witness and on its control (`build/s1e2.2.15/proto.diff`). No
existing corpus program moves under the prototype: 72 of 3 366 sweep runs
move, every one on the eight banked files.

- **(a) *decline*** — the stage's default, and **the agent's
  recommendation**. R is the blind enumerator's region whenever a guard
  scans a relation in D: `W14/oblig-fresh-name-two`'s `right` is the twelve
  states' maximal one and `W14/oblig-stdlib-total-owed`'s the sixteen's, and
  the two controls' `:expect` claims move with them in the fixing commit (3 →
  12 states, 1 → 16); every control's H moves. It is C4 as written, the
  existing decline's own consequence (a guard over a relation the rung
  proposes already falls through this way), consistent with Q-M1e2.3 (a), R
  over the universe as scoped, and with C1's ruling (Q-M1e.12 (a)) that the
  untyped blind rung's free facts are the fixture's under-specification. The
  stage's 1.5 days.
- **(b) *widen*** — the rung keeps H and proposes what each guard's domain
  reaches under D (for instance its scan at the fixpoint of root with every
  candidate committed, repeated until H stops growing). R is over the rung's
  candidates: the two rows' `right` is the control's R as it is today (3
  states, 1 state), and no control moves. A reach analysis the stage file
  declined (*not this stage: it changes what the rung proposes on every
  obligation program, and the probe found no program that needs it*), about a
  day more, and on a flagged program such as the stdlib witness the top
  node's fixpoint is not every node's.

The register files `W14/oblig-fresh-name-two` and
`W14/oblig-stdlib-total-owed` under (a), each with (b) as a
`[[case.right.alternative]]`; the other two rows have one answer under both.

**Ruled 2026-11-04: (a)**, *decline* (the user: *"(a) Decline
(Recommended)"*). The obligations rung declines when an obligation's trigger
or guard, its witness step aside, reads a relation in D, and the ladder falls
to the blind enumerator, whose objects carry every name of the program since
S1e2.2.14: R is that enumerator's region. `W14/oblig-fresh-name-two` and
`W14/oblig-stdlib-total-owed` keep (a) as `right` and (b) as the record, and
the two controls' claims move to 12 and 16 states with the fix.

**Built 2026-11-04** by [S1e2.2.15](p1e2.2_wrong_answers/s1e2.2.15_obligations_rung_universe.md)
(`bde8f53b`): `oblgen::generate` declines, after C4's own check, when an
obligation's trigger or a guard beside its witness reads a relation in D,
which `monotone::dynamic` computes, seeded with the relations the rung would
propose, for the rung and for the static check alike; the `rung` line says
*an obligation scans `food`, a relation a commitment can change*. W14's four
rows are fixed, each witness answering its control's R under the decline (1,
12, 1 and 16 states) and the oracle's. No program already in the corpus
moved (3 366 sweep runs).

## Q-M1e2.11 — Which derivations is the unsat core the smallest over, and may a config flag change it?

Put to the user on 2026-11-04 by
[S1e2.2.11](p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md)'s step 1
([T1e2.2.11.1](p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md#task-t1e22111--the-closure-size)).
W10: `Kb::record_justification` keeps at most 32 alternatives per fact, by
premise count, and `explain.rs` minimises frontier size over what it kept, so
`examples/ein-bugs/alt-cap-core.ein` reports a 3-fact core where every
derivation gives a 2-fact one. [Q-M1e.15](../../docs/history/m1e_review_processing/open_questions.md#q-m1e15--the-alternatives-cap-decides-which-unsat-core-is-reported)
was never ruled, and the stage took its recommendation, (3): keep beside the
capped list the arrival with the smallest closure, *bounded and exact for
what the search asks*.

What step 1 measured (every number the stage file's, at `3011fae2`): a
prototype of (3) and an uncapped build (`MAX_ALT_JUSTIFICATIONS = 1 000 000`)
move the same 8 of the corpus sweep's 6 510 runs, every one `alt-cap-core`'s
core, 3 facts → 2. Ten programs reach the cap under R's routes, always on
`(false)` in a dead node, and none of their cores moves. `zebra2-bad`'s core
is unchanged. (3) costs +7 % (528 → 566 ms, `utils/bench_env.sh --core 4`, best of 7) on `ein solve examples/zebra2.ein`, the
uncapped build ×2.15 (528 → 1 133 ms). Two findings make the fix a ruling:

**(i) (3) is not exact, and no bounded retention is.** A fact's best
derivation for its own sake need not be the one the target needs: the saving
can lie between two facts. [`alt-cap-shared.ein`](../../examples/ein-bugs/wrong-answers/alt-cap-shared.ein) (a probe under `probes/` until T1e2.2.11.2 promoted it):
`(x K)` has 33 one-given derivations and a 34th over `(a K)` `(b K)`, `(z K)`
only that pair, and `(false)` both; over every derivation the core is `{(a
K), (b K)}`, but the 34th is `(x K)`'s per-fact worst, so premise count, (1)'s
estimate and (3)'s closure all drop it at the full list, and the core is `{(a
K), (b K), (c C01)}`. Today 3, under (3) 3, uncapped 2. Any list of fixed
length must drop one of more incomparable derivations, and a program can need
the dropped one. The search's own budget (64 environments a label) has the
same shape, but its cut is reported, `Explanation::exhausted`; the cap's is
not. *(Dated note, 2026-11-05, M1e2 S1e2.2.23: to a library caller only,
until that stage made every surface that prints a core say it was cut, W23.)*

- **(a) *kept*** — (3), and the promise says what it is over: the smallest
  frontier over the derivations the store keeps, every derivation of a fact
  up to 32, past that the 32 with fewest premises and the one with the
  smallest closure. `alt-cap-shared` is the promise's stated limit, pinned by
  a test, and no row. W10 fixed.
- **(b) *every derivation, the residual deferred*** — **the agent's
  recommendation**. (3) now, and the promise stays the revisit's, the
  smallest frontier over the program's derivations. `alt-cap-core` is fixed;
  `alt-cap-shared` is filed as a second W10 row, `open`, its witness promoted
  to the corpus, and deferred to a followup that makes the core exact without
  the cap's memory: re-derive a full list's refused derivations when the
  search reaches that fact, which bounds memory and pays only on the ten
  programs above. (2)'s objection, *a less likely wrong answer*, is (3)'s too,
  and this keeps it visible.
- **(c) *no cap*** — exact by construction, at ×2.15 (528 → 1 133 ms) on `zebra2`'s
  describe, and a fact derived n ways holds n records, unbounded.

**(ii) `:record-alternative-justifications false` decides the core, today.**
[`probes/alt-cap-below.ein`](probes/alt-cap-below.ein) is `alt-cap-core` with
two witnesses, so no list is full; [`alt-cap-below-noalts.ein`](probes/alt-cap-below-noalts.ein)
adds `(config :record-alternative-justifications false)`. At HEAD the first
reports `(w1 X) (w2 X)` and the second `(p1 Y01) (p2 Y01) (p3 Y01)`: off, the
proof is a tree, and the core is the primary chain's. No corpus program
separates the two settings, which is why `lever_invariance.rs` holds; any fix
of W10, (a), (b) or (c), makes `alt-cap-core` separate them, and that test
fails (1 of 5 640 cells under the prototype). `configuration.md` § 2.2 calls
the flag *explanation only*, and § 2.5 and the test hold the read-out uncut,
its core rows among them. `examples/zebra.ein` sets the flag off on purpose:
without it `ein solve examples/zebra.ein` takes 188.1 s against
0.95 s and reports the same read-out and the same core on each of N's 458
members. On the corpus the flag buys speed and changes no core; on
`alt-cap-core`, once W10 is fixed, it changes one.

- **(a) *retire the flag*** — as `enable-symmetric-mirror` was at W13: every
  program records alternatives, and a `(config …)` that sets the flag is
  refused with the stage and why. `zebra.ein` loses its config line and pays
  188.1 s where it took 0.95 s, for the same cores.
- **(b) *the core is exempt under that one flag*** — **the agent's
  recommendation**. The flag stays, as what lets `zebra.ein` solve at all;
  `configuration.md` § 2.5 says that it may change which core a dead node's
  refutation names and nothing else (R, every state, the verdict words and
  `refuted by` do not move, since the primary is the same either way); the
  core promise says it is over the derivations the program records; and
  `lever_invariance.rs` holds every row but the core's under it, with the
  flag named as the one exemption.

**Ruled 2026-11-04: (i)(b) and (ii)(b)** (the user: *"(b) Every; defer
rest"* and *"(b) Exempt the core"*). The promise stays the smallest frontier
over every derivation the program records; S1e2.2.11 builds (3), and
`W10/alt-cap-core` is fixed by it; `alt-cap-shared` is W10's second row,
`open`, its witness promoted to `examples/ein-bugs/wrong-answers/` with a
corpus entry, reviewed `deferred` and owned by a new followup,
[F22](../followups/f22_exact_core_past_the_cap.md), which re-derives a full
list's refused derivations when the core search reaches that fact, and which
the milestone's amended acceptance names at the close. The flag stays:
`configuration.md` § 2.2 and § 2.5 say it may change which core a dead
node's refutation names and nothing else, the core promise says it is over
the derivations the program records, and `lever_invariance.rs` holds every
row but the core's under it, the flag named as the one exemption, with a
test that it stays that narrow. The `alt-cap-below` probes stay probes:
`alt-cap-core.ein` is the exemption's witness once W10 is fixed.

**Built 2026-11-04** by [S1e2.2.11](p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md)
(T1e2.2.11.2): `ein-core::kb` keeps, per fact, the smallest closure an arrival
has given it and that arrival beside the capped list (`Kb::closure`,
`Kb::best`), which `Kb::justifications` offers the search and the `.einb`
keeps (format 1.1, a `CLOSURE` section); the register's schema /6 holds a
row's core; `alt-cap-core.ein` reports `(w1 X) (w2 X)`, its twin's, and
`alt-cap-shared.ein` the three facts it did. No other corpus program's core
moved (the sweep's 6 510 runs).

**A dated note, 2026-11-04 (S1e2.2.19).** *Every derivation the program
records* was over each derivation's premises, and a plan of a rule with
parameters rests on the activator it was compiled from too, which no record
named: a dead node's core left out a commitment that reaches its refutation
only through one (W19). Since S1e2.2.19 a derivation's AND-node is its
premises and the activator where a node derived or committed it, on the
user's ruling on
[Q-M1e2.12](#q-m1e212--which-activators-does-a-dead-nodes-core-follow),
(c); this ruling, the promise and the exemption are unchanged.

**A dated note, 2026-11-05 (S1e2.2.21).** *Every derivation the program
records* was over each contradiction's witness alone, a pair's positive
side, and over a derivation's premises and activator, not the facts a
`forall` it judged read. Since S1e2.2.21 (`67d07fab`) a contradiction's
facts are explained together, a pair's two sides as one AND-node, and a
derivation rests on the facts its universals read, each at its own
smallest frontier, on the user's ruling on
[Q-M1e2.15](#q-m1e215--what-does-a-derivation-through-a-forall-rest-on),
(b); the promise's second residual, a core the exact search would give
smaller through a universal (13 of `zebra2-hints`' 73 N cores), is F22's
beside the cap's. This ruling and the exemption are unchanged: the
`lever_invariance` exemption still names the core alone.

## Q-M1e2.12 — Which activators does a dead node's core follow?

Put to the user on 2026-11-04 by
[S1e2.2.19](p1e2.2_wrong_answers/s1e2.2.19_core_misses_activator.md)'s step 1
([T1e2.2.19.1](p1e2.2_wrong_answers/s1e2.2.19_core_misses_activator.md#task-t1e22191--the-witnesses-banked-w19-filed-every-core-measured)).
W19: a derivation records the premises its plan matched and not the activator
the plan was compiled from, so a dead node's core leaves out a commitment that
reaches its refutation only through an activator.
[`forbid-chain-no-absent.ein`](../../examples/ein-bugs/wrong-answers/forbid-chain-no-absent.ein)'s
dead `{(p a)}` reports `(seed a)`, a fact root holds while root is alive, and
[`core-activator-guard-only.ein`](../../examples/ein-bugs/wrong-answers/core-activator-guard-only.ein)'s
reports nothing. The fix records each derivation's activator and lets the
frontier search follow it, as it follows a premise. Which activators it
follows decides which facts a core names, and the stage file stops there: if
following only a derived one moves a corpus core beyond the witnesses, or the
two readings differ on one, *which facts a core names is the user's
question*. Both happened.

What step 1 measured: a prototype with the reading as a lever, and the
stage's sweep, S1e2.2.18's with this stage's probes, 384 files × 24 runs,
every core a run reports compared against `a41eb76e`'s (2026-11-04,
`build/s1e2.2.19/`, the numbers the stage file's):

| reading | an activator is followed when | corpus programs whose cores move, past the witness | cores moved | `corpus_shapes` lines |
|---|---|---:|---:|---:|
| (a) *every* | always | 40 | 69 057 | 471 |
| (b) *a derived one*, the stage file's narrower | its own primary derivation is a rule's | 3 | 4 496 | 82 |
| (c) *one only a node derives* | the node derived or committed it: a fork's record | 0 | 0 | 11 |

- **(a) *every activator*.** A core then names the declarations its refuting
  plans were compiled from: `(functional is-a)`, `(transitive co-located)`
  and `(symmetric co-located)` across `branching/01`–`/11`, `(bijective
  color-loc)` on `domain_elim/b_branch`, `(functional seats)` or
  `(functional sits)` and their `injective` on `lattice/05`–`/07`, and in
  root's core each stdlib
  violation test's declaration (`tests/stdlib/algebra/09`–`/20`,
  `bijection/04`–`/06`, `elim/02`–`/04`, `slots/05`–`/06`, `typing/02`). It
  is the reading under which every core, read with the rules' text alone,
  derives `(false)`. Some cores stop naming their commitment: with the
  `(false)` derivations larger, a pair contradiction's positive side,
  explained alone, wins more often (S1e2.2.19 § Notes): 4 103 more dead-node
  cores name none of their commitment (`bijection/06` 4 097,
  `branching/12` 6). A `.einb` must carry each record's activator, since
  root's are followed: a format change.
- **(b) *a derived activator*.** It does not keep a declaration out: one
  enters through the activators root derives from it. `(bijective
  color-loc)` joins 132 of `domain_elim/b_branch`'s 135 cores through
  `(functional color-loc)` and `(injective color-loc)`, which root's
  `bijective-properties` derives from it; `(type-hierarchy is-a*)` joins
  `typing/02`'s root core; on `bijection/06` 263 cores gain `(bijective
  seats)`, `(relation seats Guest Chair)` and `(typecheck-hierarchy is-a*)`
  through `typecheck-setup`'s activators, and 4 097 swap the commitment
  `(seats Ann Bob)` for root's `(seats Ann C1)`, the pair contradiction
  again. As worded it leaves `W19/core-activator-committed` wrong, a
  commitment that is the activator being no rule's; read as *a rule derived
  it or the node committed it*, it moves the same corpus cores. A `.einb`
  format change too.
- **(c) *an activator only a node derives*** — **the agent's
  recommendation**. Root's activators, stated or derived, are the program's,
  as the plans root compiles from them are its rules; a plan only a node
  compiles, the rules root has no plan of (S1e2.2.17's D⁺), rests on the
  activator the node derived or committed, and the core follows it. W19's
  three rows are fixed and no corpus core moves: the sweep moves 43 runs,
  the witness's and the probes' cores, and two `alt` events on each of the
  four `binding-key` programs, since a derivation's identity gains its
  activator (narration). *With the program's rules* in the stage's
  acceptance reads as root's plans: `(p a) (seed a)` derives `(q a)` by
  `mk`, which compiles `q`'s plan, and so to `(false)`. No `.einb` change: a
  `.einb` is root's KB, whose activators are never followed. The boundary
  is two corpus entries banked for it, `core-activator-given.ein` and
  `core-activator-root-derived.ein`, whose cores (a) moves and (b) moves
  one of.

**Ruled 2026-11-04: (c)** (the user, on T1e2.2.19.1's hand-back,
`d51dc9a1`): an activator only a node derives or commits is followed into a
dead node's core, and root's activators, stated or derived, are not. The
core of a refutation that rests on a plan only a node compiles names what
the node added to compile it; root's plans are the program's rules, as
before. S1e2.2.19 builds it, and W19's three rows are fixed by it.

**Built 2026-11-04** by
[S1e2.2.19](p1e2.2_wrong_answers/s1e2.2.19_core_misses_activator.md)
(T1e2.2.19.2, `94c7bcfb`): every firing records its activator
(`Prov::activator`), `walks::node_activator` follows one whose primary
record is a fork's, and `explain`'s AND-node and `walk_premises` read it.
W19's three witnesses report `(p a) (seed a)`, `(r2 a) (seed a)` and
`(p a)`, in either build, `ein oracle` agreeing; the two boundaries keep
`(p a)`; no corpus core moved (the stage's sweep, 390 files, 2026-11-04).
`absent_premises` was left reading the premises: following the activator
there moved the non-monotone detector's status on W17's witnesses studied
past the refusal, a label (S1e2.2.19 § Notes).

## Q-M1e2.13 — What does a wildcard or a range in a rule's pattern mean?

Found 2026-11-04 by [S1e2.2.18](p1e2.2_wrong_answers/s1e2.2.18_strings_in_rules.md)'s
T3 writing the pages, and parked in its [§ Notes](p1e2.2_wrong_answers/s1e2.2.18_strings_in_rules.md#notes).
[`01_grammar.md`](../../docs/kernel/ir/03-ein-lang/01_grammar.md)'s token
table calls `_` a *head / arg wildcard in patterns*, and a `RANGE` such as
`1..3` UML-style cardinality. The rule compiler lowers both, in a `:match`,
as an opaque slot that matches nothing: `(rule w () :match (holds _) …)`
never fires on `(holds a)`, and `(rule r () :match (n 1..3) …)` never on
`(n 2)`, in silence; a head `_`, `(_ a)`, stops compilation with
`ein.inference.compile.CompileError: unbound relation head ?Wildcard() …`, a
Python spelling, exit 1 (re-taken at `876767e6`, `build/w20-probe/`). No
rule under `examples/`, `tests/` or `stdlib/` writes either (711 rule
forms, 2026-11-04).

Put to the user on 2026-11-04 at P1e2.2's end, with three options: (a)
**W20**: `_` matches any value, as the grammar says, in the head and in an
argument, and a range in a rule is refused at load with a line in Ein's
spelling, since no fact can hold one; (b) refuse both at load, the grammar's
row corrected; (c) park to a followup, no row.

**Ruled 2026-11-04: (a)**, the recommendation. W20 is
[S1e2.2.20](p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md),
which records the defaults the ruling leaves (`_` under an `absent`, in a
guard, in an `:assert`; a head `_`'s cost).

**The head half re-ruled 2026-11-05**: matching a head `_` needs a second
matching path, so S1e2.2.20's step 1 put
[Q-M1e2.14](#q-m1e214--how-does-a-head-wildcard-match-every-relation) to the
user, who ruled (c): a head `_` is refused at load, as a range is. **Built
2026-11-05** by
[S1e2.2.20](p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md)
(T1e2.2.20.2, `eddea251`): `_` in a rule's pattern is a slot that matches any
value and binds nothing (`Slot::Any`), and the loader refuses a range in a
rule and a `_` written as a relation, one line per slot; W20's five rows are
fixed, and no answer moved outside W20's programs and probes (the stage's
sweep, 438 files, 2026-11-05).

**Superseded on the range 2026-11-06** by
[Q-M1e2.24](#q-m1e224--are-there-numbers-in-ein): Ein has no numbers and no
ranges, so the token goes rather than a range in a rule being refused at load.
**Built 2026-11-08** by
[S1f.5.3](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.3_range_retired.md)
(T1f.5.3.2, `65d1ad85`): there is no `RANGE` token, the loader's range line is
gone, and `1..3` was a parse error wherever it was written until M1f S1f.5.5's
swap (T1f.5.5.4, 2026-11-09) made it one name, S1f.5.4's ruling: `(n 1..3)` in
a rule is a pattern over that name since, and `W20/range-in-match`'s `right`
is the cube of 8 it answers. The wildcard half stood until the same swap, which
refuses a bare `_` at the parse in every position (S1f.5.1's ruling,
Q-M1e2.22).

## Q-M1e2.14 — How does a head wildcard match every relation?

Put to the user on 2026-11-05 by
[S1e2.2.20](p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md)'s
step 1 ([T1e2.2.20.1](p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md#task-t1e22201--the-witnesses-banked-w20-filed-every-move-listed)).
Q-M1e2.13 (a) rules that `_` in a rule's pattern matches any value *in the
head and in an argument*. The argument half is a slot that binds nothing,
built as a prototype and measured: every argument-wildcard witness answers its
twin's R on every surface, and no corpus run moves. The head half is not a
slot. M1 compiles a plan per `(rule, activator)` pair and binds a relation
head by the activator (Q29), so a `RelStep` names one relation, and
[`wild-in-head.ein`](../../examples/ein-bugs/wrong-answers/wild-in-head.ein)'s
`(rule w () :match (_ a) :assert (hit w))` stops compilation today, exit 1,
with `unbound relation head ?Wildcard() …`, as `(?r a)` does with no
parameter `?r`. The stage file stops here: matching a head `_` needs a
second matching path, and which one, at what cost, is the user's.

What every way must decide first: **every relation** includes the kernel's.
A fact of one argument `a` can be a declaration's own `(relation a)`, a
rule's activator `(total a)`, a stored negative's `not`, or `is-a*`, so `(_ a)`
and `(_ ?x ?y)` match those too, and on `zebra2.ein` root holds facts of 40
relations, 25 of them named as a rule is, the activators the stdlib's setup
derives among them, `(functional color-loc)`, `(total color-loc)` (`ein
saturate --dump`, `build/s1e2.2.20/head_census.py`, 2026-11-05, at
`c73a0796`).

What step 1 measured (`head_census.py` and `names_census.json`, the corpus's
326 programs under `examples/`, `tests/` and `stdlib/`, 261 of which
saturate, 2026-11-05, at `c73a0796`): root's fixpoint holds facts of a median
4 relations and at most 41 (`ein-bugs/zebra2-bad`), a median 10 facts and at
most 570, and its largest one extent is a median 6 facts and at most 372
(`zebra.ein`); a program names a median 11 names and at most 93, a median 6
relations and rules and at most 52 (`zebra2-hints`). On `zebra2.ein`: 82
names, 18 relations and 26 rules; root holds 388 facts, its largest extent
103 (`is-a*`).

- **(a) A plan per relation.** The compiler expands a head `_` the way a
  parameter its activator binds is expanded: one plan, or one disjunct, per
  relation, with the head written in. The matcher and the saturator do not
  change, and every reader of a compiled step sees an ordinary one. The
  set it expands over must hold every relation a fact can ever have, which a
  node can add to (a commitment, an activator a node derives, a parameter
  bound to a name), so it is every name of the program: 82 plans for one
  head `_` on `zebra2.ein`, 44 if objects are left out, and the product for
  several in one conjunction (6 724 for two). Under `absent` it is one
  guard per relation, since a nested `(or …)` is refused. The readers of a
  rule's pattern rather than its plans (*forbid*'s D⁺, which reads a rule
  root has no plan of from its pattern; the alive-set invariant; the
  loader's pattern views) learn one case: a head `_` reads every relation.
- **(b) A step over every relation.** One plan; a `RelStep` whose relation
  is *any*, and a second matching path: the matcher scans the KB's every
  fact of the arity (on `zebra2.ein`'s root 388 facts, where one extent is
  at most 103), the saturator's index wakes the plan on every new fact, and
  every reader of a step's relation learns *every relation*: 52 sites in 13
  files of `ein-infer` read one (`r.rel`, `step.rel`, a nested slot's
  `rel`: `match_.rs` 10, `compile.rs` 10, `monotone.rs` 8, `detector.rs` 6,
  `oblgen.rs` 5, `saturator.rs` 4, `shape.rs` 3, six more one each),
  among them the static check's D, *forbid*'s D⁺ and the obligations rung's
  decline, the three readers whose misreadings were W14 and W17.
- **(c) Refuse a head `_` at load** — **the agent's recommendation**. The
  argument `_` matches any value, as built; a head `_` is refused with a
  line in Ein's spelling, as a range is, and `01_grammar.md`'s token table
  calls `_` an *argument wildcard in patterns*. A relation head stays what
  Q29 made it, an atom or a parameter its activator binds. No corpus
  program writes one (S1e2.2.20's scan, 733 rule and 7 macro forms,
  2026-11-05), what it would match includes the kernel's own facts, which a
  rule's author is unlikely to mean, and either way to match it adds a path
  every static reader must know, the class of reader W14 and W17 were
  defects of. The row `W20/wild-in-head` then has a refusal as its `right`,
  as `W20/range-in-match` does.

Whichever is ruled, the line the head `_` prints today goes: it is the
Python IR's spelling of the pattern (`SForm(head=Wildcard(), …)`), under
`ein saturate` with `ein.inference.compile.CompileError:` before it.

**Ruled 2026-11-05: (c)** (the user, on T1e2.2.20.1's hand-back,
`d8cfc4dd`): a head `_` is refused at load, with a line in Ein's spelling,
as a range is, wherever a `_` would be a relation head: a premise's, a
nested pattern's such as `(box (_ a))`, one under `absent` such as
`(absent (_ b))`, and a conclusion's. The argument `_` matches any value, as
prototyped. `W20/wild-in-head`'s `right` becomes a refusal, and
`01_grammar.md`'s token table row reads *argument wildcard in patterns*.
S1e2.2.20 builds it.

**Built 2026-11-05** by
[S1e2.2.20](p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md)
(T1e2.2.20.2, `eddea251`): `ein_ir::from_ir`'s `refuse_no_fact_holds`
refuses a `_` written as a relation beside a range, one line per slot, and
`W20/wild-in-head` is fixed with the refusal as its `right`. Measured on a
prototype before the edit, the head refusal moved 216 runs on nine files, W20's
head witness and the head probes, and nothing else; no program the loader
accepts reaches the compiler's unbound-head arm through `_` since.

## Q-M1e2.15 — What does a derivation through a `forall` rest on?

Put to the user on 2026-11-05 by
[S1e2.2.21](p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md)'s step 1
([T1e2.2.21.1](p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md#task-t1e22211--a-witness-banked-w21-filed-every-core-measured)).
W21: a pair contradiction's core is its positive side's frontier alone, so
where `X` is root's it names none of the commitment
([`pair-core-root-given.ein`](../../examples/ein-bugs/wrong-answers/pair-core-root-given.ein)
reports `(p a)` for the dead `{(q a)}`). The stage's default fix explains a
pair's two sides together. Prototyped, and every core the corpus reports
compared against `cdcc3994`'s (the stage's sweep, 441 files × 24 runs,
2026-11-05, `build/s1e2.2.21/`), it does what it was filed for, and the
other way too. The stage file stops on both: a contradiction other than a
pair with W21's defect, and cores that named their commitment and stop.

**What moved.** 69 201 dead-node cores start naming their set
(`branching/01`, `/02`, `/12`, `tests/stdlib/bijection/06`). 2 172 stop:
`zebra.ein` 372, `zebra2` and `zebra2-obligations` 127 each,
`zebra2-minus-15` and its `-obligations` twin 773 each; and `zebra2-hints`'
73 still name none. Each is then decided by a **direct ⊥ concluded through
a `forall`**: `zebra2`'s `{(drink-loc Tea House-4)}` reports `(is-a Blue
Color) (relation color-loc Color House)` by `total`, root's. `(forall ?b G
B)` is `(absent (and G (absent B)))`, and a firing records the facts its plan
matched (`Prov::premises`) and an `absent` it judged as a pattern
(`Prov::absent`, *recorded, not interpreted*). A universal that holds rests
on facts, though: for each `?b` that G matches, the fact B matched. `total`'s
`(false)` rests on the five `(not (color-loc Blue House-k))`, which the
commitment derives, and records the two facts it matched. At HEAD it lost to
a pair's one-fact core; once a pair's core covers both sides, it wins. A
positive concluded through a `forall` is the same, and its frontier leaves
out the values it excluded every other one by. Ten stdlib rules write one:
`total`, `surjective`, `slot-no-room`, `slot-no-fill`, `slot-elimination`,
`slot-fill`, and the two `domain-elimination`s, `range-elimination` and
`no-room-left`. The defect is not new: it is why some of HEAD's cores fail
the derivation half of the class-4 claim (110 of 1 255 distinct parts, on
`zebra.ein`, `zebra2` and `zebra2-minus-15` and their twins, and the three
`type-exclusivity` programs, `build/s1e2.2.21/derivecheck.py`), and the pair
fix only makes it decide more cores.

Measured, beside HEAD and the pair fix alone:

| reading | cores naming none of their set, of 214 964 | parts deriving no contradiction | `ein solve examples/zebra2.ein` | `zebra2-minus-15.ein` |
|---|---:|---:|---:|---:|
| HEAD | 71 208 | 110 of 1 255 | 0.59 s | 8.11 s |
| (a) the pair alone | 4 179 | 0 of 1 316 | 1.30 s | 15.08 s |
| (b) and a `forall`'s support, each at its smallest | 68 (`lattice/06`, N6) | 0 of 5 427 | 3.68 s | 31.65 s |
| (c) and its support, searched | 0 of 73 on `zebra2-hints` | — | stopped at 150 s | — |

(best of three, release, 2026-11-05, not through `bench_env.sh`; no core at
all, `zebra2` 0.50 s and `zebra2-minus-15` 7.37 s.)

- **(a) *A universal rests on nothing it read; the pair alone, the universal
  later*.** W21's fix lands as filed, and the universal is a new case, its
  own stage or a followup. Meanwhile 2 172 cores that named their
  commitment stop, `zebra2-hints`' 73 still name none, and the corpus-wide
  test this stage owes cannot hold on those six programs, so W21's
  acceptance is not met for them.
- **(b) *A universal rests on the facts it read, each at its smallest
  frontier*** — **the agent's recommendation**. A firing records, for each
  `forall` it judged, the facts its body matched, and the core search folds
  each in at its smallest environment, beside the premises it searches as
  today. Every core the corpus reports names its set and, with root,
  derives a contradiction, but `lattice/06`'s 68 empty ones, which are
  P1e2.3's N6. Cores grow (`zebra2`'s N, one fact each at HEAD, 2 to 32);
  a `.einb` carries the support (format 1.2). It is not the smallest over
  every derivation: on `zebra2-hints`, 13 of N's 73 cores are larger than
  (c)'s, the promise's residual, which joins `W10/alt-cap-shared`'s in
  [F22](../followups/f22_exact_core_past_the_cap.md). It costs ×6 on
  `zebra2`'s bare `ein solve`, which prints no core and takes one for every
  dead node it visits; taking a core only where a surface reports it would
  give that back under any reading, parked in the stage file's § Notes.
- **(c) *Its support, searched like a premise*.** Exact over the recorded
  derivations, and unaffordable as the search stands: 7.5 s per member of N
  against 3 ms on `zebra2-hints`, cut by its budget (`exhausted = false`),
  542 s for its `--dump-answer`.

Whichever is ruled, a pair's two sides are folded **jointly**, the smallest
union over both sides' environments (the stage's default 2): under (b) the
stage file's literal *each side at its smallest* differs on `zebra2` and
its twin (36 cores each) and `zebra2-minus-15` and its twin (121 each), a
larger core on 99 of each 157 and the same size on the rest, never smaller.

**Ruled 2026-11-05: (b)** (the user, on T1e2.2.21.1's hand-back,
`a98a94e3`): a derivation through a `forall` rests on each fact the
universal read, each taken at its own smallest frontier, beside the premises
the search folds as it does; a pair's two sides are folded jointly, as the
stage's default 2 has it. The residual, the cores (c) would give smaller (13
of `zebra2-hints`' 73 N cores, the stage's measurement), joins
`W10/alt-cap-shared`'s in [F22](../followups/f22_exact_core_past_the_cap.md),
and F22 and the register's notes say so. S1e2.2.21 builds it, and files the
`forall` gap as rows of W21.

**Ruled with it, the same day: a core is taken only where a surface prints
one**, applying the user's standing *minimum work per question*. A bare
`ein solve` takes none; the read-out, `--dump-answer`, `--json-summary`,
`--trace`, `ein oracle --dump` and every other surface that prints a core
take the ones they print. It must move no answer: the stage's corpus-wide
test and every surface give the cores an eager run gives, and what moves is
listed before the edit, as step 1 listed the fix's. S1e2.2.21 builds it,
inside the stage.

**Built 2026-11-05** by
[S1e2.2.21](p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md):
(b) by T1e2.2.21.2 (`67d07fab`), `Prov::universal` recorded at fire time and
each fact folded in at its own smallest frontier, a contradiction's facts
explained together, W21's four rows and the `forall` gap's two
(`W21/forall-core-false`, `-elim`) fixed, the `.einb` at format 1.2; the
lazy core by T1e2.2.21.3 (`3fb9d712`), `explore`, `region` and `downset`
taking a core only where a surface prints one, 0 of the sweep's 10 752 runs
moved, a bare `ein solve examples/zebra2.ein` 0.53 s against 0.58 s at
HEAD and 3.65 s eager (2026-11-05). `ein-infer/tests/corpus_cores.rs`
(T1e2.2.21.4, `af521c6e`) holds that every core the corpus reports names its
commitment and, with root, derives its contradiction.

## Q-M1e2.16 — What does the close do with the inventory's items?

**Asked 2026-11-05.** The user asked to see an inventory before the milestone's close, verbatim: *"stop before close m1e2, I want to see inventory of known yet incorrect results, known solution gaps and their intersections with monotone/non-monotone lattices and region"*. [The inventory of 2026-11-05](inventory_2026-11-05.md) was taken on a snapshot build at `cdcc3994`, from every register row, every stage's Notes, every open question and followup, and a sweep of the 326 corpus programs.

It counts these live items:
- 15 wrong answers (W), 7 with no owner;
- 14 surfaces that misreport a right answer (S), 10 with no owner;
- 11 diagnostics (D), none with an owner;
- 19 gaps (G), each with its ruling.

It also found two wrong answers nothing named:
- **F1**, a keyword prefix split in a rule;
- **F2**, a core search cut at its budget, printed as the smallest.

On every one of the 204 corpus programs both finish, `ein solve` and `ein oracle` agree on k_s, k_c, |N|, the holes and the static verdict. Every live defect the corpus reaches on a monotone program is in a core; the ones that reach R itself on a monotone program are shapes no corpus program writes. Every corpus defect in R is on a non-monotone program, and F21's.

Put to the user the same day, with the agent's recommendation in six parts.

**Ruled 2026-11-05: as recommended** (*"File W22, W23; fix pre-close"* and *"Accept all"*):
- **F1 and F2** are W22 and W23, fixed before the close: [S1e2.2.22](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md) and [S1e2.2.23](p1e2.2_wrong_answers/s1e2.2.23_core_search_cut.md). W23's fix says a core was cut, as N6 says a core was not taken.
- **(1)** `_` in a guard, a premise that is not a form, and a nested unbound relation or keyword pair are refused at load, in one stage: W24, [S1e2.2.24](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md).
- **(2)** The two cores that are not the smallest (W-8, a premise-less alternative never recorded, and W-10, the closure estimate) are [F22](../followups/f22_exact_core_past_the_cap.md)'s.
- **(3)** The surfaces are split three ways:
  - a reloaded dump's provenance joins [S1e2.3.8](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md);
  - `refuted by` and `--json-report`'s missing rows are [S1e2.3.10](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_refuted_by_and_report_rows.md);
  - the detector-internal labels (S-6 to S-9, S-11) are [F21](../followups/f21_non_monotone_programs_decided.md)'s.
- **(4)** The eleven diagnostics are a followup, [F23](../followups/f23_diagnostics_in_eins_words.md).
- **(5)** The live wrong answers with an owner and no row (W-2, W-5, W-6 to F21; W-8, W-9 to F22; W-15 to M1f) are filed as `open`, deferred rows: [S1e2.2.25](p1e2.2_wrong_answers/s1e2.2.25_the_deferred_rows.md).
- **(6)** The close's re-take is the register's tests plus the inventory's solve-against-oracle sweep, not `probes/capture.sh`'s W lines and inventory § 4's sweeps, which read the walk. [The milestone's acceptance](README.md#acceptance-for-the-milestone) is amended.

The close waits on the user's go-ahead after the inventory is re-taken.

## Q-M1e2.17 — Where does a keyword end?

Put to the user on 2026-11-05 by
[S1e2.2.22](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md)'s step 1
([T1e2.2.22.1](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md#task-t1e22221--the-witnesses-banked-w22-filed-every-move-listed)).
The stage makes a keyword a whole word wherever the parser reads one, as
[`00_ebnf.md`](../../docs/kernel/ir/03-ein-lang/00_ebnf.md) § 1, fact 1,
says. Step 1 parsed every reserved word in `lex.rs`'s `LITERALS`, the 19 the
parser matches as a literal, followed by each of eleven name characters, in
every place the parser reads one, under nine argument shapes, with the
kernel's eight other heads as controls (53 460 parses,
`build/s1e2.2.22/cases.py`, at `35186bc0`). At `35186bc0` 2 691 of them
split. The prototype refuses a literal that a word character follows
(`match_literal`), and it moves 2 256: 2 208 to the whole name, and 48 trace
events to a parse error. **435 still split**, every one a reserved word
followed by `-` and a digit: `(neq-1 ?x)` reads as `(neq -1 ?x)`, at
`35186bc0` and under the prototype.

The grammar has two sets of name characters:

- **`SYMBOL`'s body**, `[A-Za-z0-9_*.-]`: `-`, `.` and `*` continue a name,
  as in `foo-1` and `std.rule`.
- **The word boundary after a reserved word**, `WordChar` (`_`, a letter, a
  digit). Fact 1 excludes a reserved word from `SYMBOL` where none of these
  follows it: *"`rule-x` is not [a `SYMBOL`] (the `-` is not a word
  character, so the boundary holds and the exclusion fires)"*.

So `neq-1` is no name anywhere in the language: `(p neq-1)` is a parse error
at `35186bc0` and under the prototype. After `not`, `and`, `or` and `neq` a
value follows, and `-` is the one character that both continues a name and
starts a value, an `INT`'s sign. So `(neq-1 ?x)` is read as the keyword and
the value `-1`. A guard `(neq-1 ?x)` holds of every `?x` but `-1`, and an
`:assert (neq-1 ?x)` stores `(neq -1 a)`. `.` and `*` start no value, so
`(neq.x a)` and `(neq* a b)` are parse errors either way. No file under
`examples/`, `tests/`, `stdlib/`, `corpus/`, `docs/` or `plans/` writes the
shape (`grep -rnE '\((not|and|or|neq)-[0-9]'`, 2026-11-05).

**(i) Where does a keyword end?**

- **(a) At the word boundary fact 1 states** — **the agent's
  recommendation**. A keyword ends at the first character that is not a word
  character, for the literal as for `SYMBOL`'s exclusion, so the two
  readings of a reserved word never overlap: where the literal matches,
  `SYMBOL` does not. `(neq-1 ?x)` stays `(neq -1 ?x)` and `(rule-x A)` stays
  a parse error. It moves nothing beyond the prototype's 2 256 cases, and no
  answer outside W22's programs (step 1's sweep).
- **(b) Where a name could not continue**, `SYMBOL`'s characters. `(neq-1
  ?x)`, `(and-1 …)`, `(or-1 …)` and `(not-1)` become parse errors, since
  `neq-1` is no `SYMBOL` either: the 435 cases move to an error. No corpus
  program moves.
- **(c) (b), and `neq-1` a name.** The exclusion's boundary moves to
  `SYMBOL`'s characters too, so `rule-x`, `rule.x`, `not-a` and `neq-1`
  become names: fact 1's own examples move, and so do the decision table's
  `(rule-x A)` and `(rule.x A)`, from a parse error to a fact. This changes
  the language beyond what the stage was asked to do.

**(ii) A trace event.** A `(trace …)` holds only trace events, so a trace
word read as a whole word has no other reading. `(trace (stepx :a b))` is the
step `x` at `35186bc0`, and under the fix it is a parse error, as `(trace
(foo :a b))` is. This covers 48 of the probe's cases and the decision table's
five trace rows, `ok` → an error. The stage file's default is *no new
error*, and its acceptance is *no reserved word followed by a name character
is split anywhere the parser reads one*. The two disagree here.

- **(a) The acceptance: a parse error** — **the agent's recommendation**.
  The loader ignores a `(trace …)`, no corpus file writes such an event, and
  the engine writes `(step <name> …)`.
- **(b) The default: the six trace words keep matching as a prefix**, so
  `(trace (stepx …))` stays the step `x`.

Neither part moves an answer outside W22's programs, or any corpus program,
under any of its options.

**Ruled 2026-11-05: (i)(a), (ii)(a), as recommended** (the user, on
T1e2.2.22.1's hand-back, `6de048e3`). A keyword ends at the first character
that is not a word character, the boundary fact 1 gives `SYMBOL`'s
exclusion, so the literal and the `SYMBOL` readings of a reserved word never
overlap; `(neq-1 ?x)` stays `(neq -1 ?x)` and `(rule-x A)` a parse error. A
trace word is a whole word too, and `(trace (stepx :a b))` is a parse error,
as `(trace (foo :a b))` is. S1e2.2.22 builds both, as prototyped.

**Built 2026-11-05** by S1e2.2.22 (`ed45a470`, T1e2.2.22.2): `ein_ir::lex::match_literal`
reads a reserved word only where no word character follows it, as
prototyped; W22's fourteen rows are fixed, each witness answering its twin's
R on every surface in either build, and no answer outside W22's programs
moved. `00_ebnf.md` § 1's fact 1 and § 3, and `defined_behaviour.md` § 1.4,
say so since T1e2.2.22.3.

## Q-M1e2.18 — What else in a rule reads nothing, in silence?

Put to the user on 2026-11-05 by
[S1e2.2.24](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md)'s step 1
([T1e2.2.24.1](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md#task-t1e22241--the-witnesses-banked-every-shape-found-w24-filed-every-move-listed)).
The user ruled the stage's three shapes on Q-M1e2.16 (1): `_` in a guard, a
premise that is not a form, and a pattern inside a pattern whose relation is
a variable no parameter binds or that holds a keyword pair, each refused at
load. Step 1 probed every argument kind in a guard, every premise that is not
a form and every place a nested pattern can take, in a rule, an hrule and an
obligation (165 probes, `build/s1e2.2.24/shapes/`, at `32736ac8`, release and
debug alike). It found **four more shapes that read nothing, in silence**,
which the ruling does not name. No corpus, stdlib or `tests/` program writes
any of them: a prototype that reports each, run with `ein saturate` over the
sweep's 690 files (every `.ein` under `examples/`, `tests/`, `stdlib/` and
`corpus/fuzz_findings/`, and the stages' probes), finds them only in this
stage's probes and one S1e2.2.20 probe (`build/s1e2.2.24/scan.py`,
2026-11-05).

**(i) A form as a guard's argument.** A guard's arguments are raw IR nodes
(`compile.rs`' `guard_arg`, `plan::GuardArgKind::Node`), and a form is one,
so it is equal to no value: `(eq ?x (f a))` never holds, even where `?x` is
bound to the fact `(f a)`, and `(neq ?x (f a))` always holds. Two forms
compare as written: `(eq (f a) (f a))` holds, and `(eq (f ?y) (f ?z))` does
not, whatever `?y` and `?z` are. It is `_`'s mechanism; a form, unlike `_`,
names a value a fact can hold (a nested fact, Q40).

- **(a) Refuse it at load, as `_` is** — **the agent's recommendation**.
  No program writes one, and a guard's arguments become a variable, a name,
  a string or an integer.
- **(b) Lower a ground form to the nested fact it names**, so `(eq ?x (f a))`
  holds where `?x` is that fact, and refuse a form that holds a variable or
  `_`. It makes a guard able to compare nested facts, which nothing asks for,
  and the compiler must intern a fact the store may not hold.

**(ii) A guard on a variable no earlier premise binds.** A guard is judged
where it is written, and a variable not yet bound compares as a null, equal
only to another unbound one (`plan::GuardArgKind::Reg`). So a variable bound
nowhere reads nothing, `(eq ?x ?u)` never holds, `(neq ?x ?u)` always does
and `(eq ?u ?v)` holds; and so does one bound only by a later premise:
`(and (eq ?x a) (p ?x))` never fires, and `(and (neq ?x a) (p ?x))` fires
for every `?x`, `a` included. The second is the shape a user is likeliest to
write. Under an `absent`, a variable the enclosing premises bound before it
counts as bound, and one bound after it does not, as the compiler scopes it.

- **(a) Refuse it at load**, naming the variable: a guard compares values
  the premises before it bind — **the agent's recommendation**. No program
  writes one.
- **(b) Move a guard after the premises that bind its variables**, since a
  conjunction does not order its members, and refuse one whose variable
  nothing binds. It changes the compiled plan's step order, which the plan
  shape prints, and gives a guard a meaning its position does not.

**(iii) A keyword pair as a guard's argument.** `(eq ?x :k a)` parses (`eq`
is a generic list; `(neq ?x :k a)` is a parse error, `NeqForm` being
shape-pinned) and compares the pair as a raw IR node, so it never holds.

- **(a) Refuse it at load** — **the agent's recommendation**.
- **(b) Drop it, as Q32 drops a top-level pattern's keyword pair**, which
  leaves `(eq ?x)`: the compiler refuses that at its arity, with a
  `CompileError` in the Python spelling (F23's).

**(iv) A relation parameter its activator binds to a nested fact.** `(rule r
(?R) :match (box (?R b)) …)` with the activator `(r (a b))`: a nested fact
binds no parameter (`compile.rs`' seed), so the inner pattern is an opaque
slot and never matches, in silence; at the top level, `(?R b)`, compilation
stops with the `CompileError` in the Python spelling. The rule's shape is
fine; its activator binds nothing. The loader refuses an activator that
binds such a parameter to an integer (S1e2.2.4's
`validate_relation_parameters`).

- **(a) Refuse it at load where the activator is written**, a fact or a
  literal in a rule's `:assert`, as an integer is, and in the compiler where
  a rule derives it through a variable — **the agent's recommendation**.
- **(b) Leave it**: the top level keeps its `CompileError` (F23's), and the
  nested pattern keeps matching nothing.

**(v) A corpus fixture that writes the second shape.**
`examples/broken/compile/empty_absent.ein`, `(and (p ?x) (absent ?x))`, is
S1a.3.1's compile-negative fixture for a premise the compiler returns no
steps for. It is refused today by a `CompileError` in the Python spelling,
*`` `(absent …)` sub-plan compiled to no steps in SForm(head=Atom(name='absent'), args=(Var(name='x'),)) … ``*,
exit 1, and the load pass refuses it, exit 1, with its line: its answer does
not move, its line does, and it no longer fails at compile, which is what
`examples/broken/compile/` holds (its README, and `ein-corpus`'s
`negatives_are_grouped_by_where_they_fail`). `unbound_head.ein` beside it is
the top-level `(?R ?x)`, which the stage file's default 1 already moves to
the load line, with the same consequence.

- **(a) Refuse it at load with the rest, and move both fixtures to
  `examples/broken/load/`**, `load-negative`, each `.expected` its load line
  — **the agent's recommendation**. No answer moves, and the Python spelling
  goes from both.
- **(b) Leave an `(absent X)` whose whole body is no form to the compiler**,
  so `empty_absent.ein` keeps its `CompileError` (F23's to respell), and only
  `unbound_head.ein` moves.

Under every option no answer moves outside this stage's probes: no program
writes any of (i)–(iv), and (v) moves a refused program's line, not its exit
(step 1's sweep, 693 files, 18 018 runs, 2026-11-05). The recommendation
refuses each of (i)–(iv) at load in S1e2.2.24, each a W24 row filed and fixed
in T2 with its probe as the witness, as W21's `forall` rows were.

**Ruled 2026-11-05: (i)(a), (ii)(a), (iii)(a), (iv)(a), (v)(a), as
recommended** (the user, on T1e2.2.24.1's hand-back, `f8c10730`). A form, a
variable no premise before it binds and a keyword pair, as a guard's
argument, are refused at load, the variable named; a relation parameter its
activator binds to a nested fact is refused at load where the activator is
written, a fact or a literal in a rule's `:assert`, as an integer is, and by
the compiler where a rule derives the activator through a variable. Each of
(i)–(iv) is a W24 row, its probe the witness, filed and fixed by S1e2.2.24's
T2, as W21's `forall` rows were. `examples/broken/compile/empty_absent.ein`
and `unbound_head.ein` are refused at load and move to
`examples/broken/load/`, `load-negative`, each `.expected` its load line.

**Built 2026-11-05** by S1e2.2.24 (`3ab9a231`, T1e2.2.24.2), as ruled:
`ein_ir::from_ir`'s `refuse_no_fact_holds` reads a `:match` as premises with
the variables the premises before each bind, and refuses a form, a keyword
pair and a variable no premise before it binds as a guard's argument, beside
W24's three shapes; `validate_relation_parameters` refuses an activator that
binds a relation parameter to a nested fact where it is written, and the
compiler one a rule derives (`head_not_a_name`). Each of (i)–(iv) is a W24
row filed and fixed by that commit, its step-1 probe the witness, and the
two fixtures moved. What it moved beyond step 1's list was measured on a
prototype before the edit: the 31 probes the four refuse and two extras, no
corpus program; and one matcher unit test whose program (ii) refuses, kept
with parameters (`defined_behaviour.md` § 4.9).

**(ii)(a) superseded 2026-11-07** by [Q-M1e2.26](#q-m1e226--are-eq-and-neq-relations): `eq` and `neq` are
relations, and a rule's premise means the same in any order of its conjuncts, so a guard on a variable no
premise before it binds is answered, not refused. `W24/guard-before-binder`'s `right` is restated by
[S1f.5.7](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md#premises). (i)(a) is superseded
too, the same day: a nested fact as a guard's argument is answered; (iii)(a) and (iv)(a) stand, and `_` in a guard stays refused (Q-M1e2.22).

## Q-M1e2.19 — What do `--minimal` and `--maximal` answer past the limit?

Put to the user on 2026-11-05 by
[S1e2.2.25](p1e2.2_wrong_answers/s1e2.2.25_the_deferred_rows.md)'s re-take
([T1e2.2.25.1](p1e2.2_wrong_answers/s1e2.2.25_the_deferred_rows.md#the-finding-two-modes-disagree-past-the-limit)),
which stopped on it: it is none of that stage's items, and two modes of one
tool disagreeing is a finding, not a deferred row.

**The finding.** Past `--region-limit`, on a program the static check flags,
describe (`ein solve FILE`) certifies the members it met with no member of R
met below them, which is `{∅}` when ∅ is in R, and says the maximal line is
not known. `ein solve FILE --minimal` labels every member it met as on the
minimal line, and `--maximal` every member as on the maximal line:

- `examples/saturation/square-unique/corner-house.ein`, at the default limit
  of 16 (118 candidates): describe lists one minimal state, ∅; `--minimal`
  prints *minimal line 65 214 states (a lower bound: the run stopped first)*
  and labels each state `· minimal`, 65 213 of them non-empty sets with ∅
  below them; `--maximal` labels the same 65 214 `· maximal`, ∅ among them.
  The summary agrees with each read-out (`lines: ["minimal"]` under
  `--minimal`, `lines: []` under describe, for one state).
- `features/01` (64 655 states), `features/04_open` (64 761),
  `square-unique/cul-de-sac` (64 817) and `/terminus` (64 514) the same, and
  `lattice/06_friends_tables` labels 21 minimal where describe certifies 1.
- The smallest: `examples/lattice/10_fifo_gap.ein --region-limit 1`.
  `--minimal` labels ∅ and `{(a one)}` minimal, `--maximal` the same two
  maximal, and `ein oracle` gives minimal `{∅}` and maximal `{(a one), (b
  one)}`.

Release and debug alike (`ein solve FILE`, `--minimal`, `--maximal`,
`--json-summary`, at `94c75204`, 2026-11-05). The approved interface says
otherwise ([`solve_interface.md` § 3.3](p1e2.1a_the_solution_region/solve_interface.md#33-a-program-the-static-check-flags):
past the limit, *states printed as members of R and not as line members*).
The cause is `explore::flagged`: past the limit a line mode asking describe
is not one of the questions answered in layer order, so it asks
`cube_in_order` with `Which::All` and then labels the members it met with
its own line. Nothing holds the line modes past the limit: `explore.rs`
holds every mode to the oracle within its limit, and no corpus run asks a
line mode of these programs.

Put to the user the same day, in two parts, with the agent's
recommendation:

- **(1) Where it goes.** A new stage, S1e2.2.26, files it as a register row,
  W27, and fixes it before the close: after S1e2.2.25 and before P1e2.3's
  S1e2.3.8–.10 — **the agent's recommendation**. It is a mode disagreeing with
  describe and, under a lowered limit, with the oracle (*one tool, consistent
  answers*), and no followup owns it.
- **(2) What the fix answers.** The line modes answer what describe
  certifies: past `--region-limit` on a flagged program, `--minimal` lists
  the members met with no member of R met below them (`{∅}` when ∅ is in R),
  and `--maximal` lists none and says the maximal line is not known (it
  needs the whole cube) — **the agent's recommendation**.

**Ruled 2026-11-05: (1) and (2), as recommended** (the user, through the
coordinator, the same day). S1e2.2.26 files it as W27 and fixes it before
the close, after S1e2.2.25 and before P1e2.3's S1e2.3.8–.10. Past
`--region-limit` on a program the static check flags, `--minimal` certifies
exactly what describe certifies, a met member with no member of R met below
it (`{∅}` when ∅ is in R), counts the other members met and leaves them to
`--dump-answer`; `--maximal` certifies none and says the maximal line is not
known, as describe does. Owner: [S1e2.2.26](p1e2.2_wrong_answers/s1e2.2.26_line_modes_past_the_limit.md) (W27).

**Built 2026-11-05** by S1e2.2.26 (`c21f93fc`, T1e2.2.26.2), as ruled: past
the limit a line mode answers from describe's partial description
(`explore::flagged`, `Space::partial_cube`), so `--minimal` lists the members
describe certifies on the minimal line, {∅} on every program the stage
re-took, and every `--maximal` question lists none and says the maximal line
is not known (`--single`: *not decided*); each line row is describe's, the
other members met are counted and not listed, and `--dump-answer` writes
each. W27's four rows are fixed, and
`ein-render/tests/line_modes_past_the_limit.rs` holds every line mode past
the limit to describe and, where it is known, to the oracle. Step 1 found no
monotone route that mislabels and no line member describe certifies that
the oracle does not, so the ruling's scope was the whole defect.

**Extended 2026-11-07 (the inventory's S-16 and P-14).** Past the limit, with
∅ in R, `--minimal --single` still said *not decided* and `--minimal -n N`
*may hold more*, which this ruling did not name. The user ruled (b): *"b, use
the ∅ rule in both"*. Where ∅ is in R the minimal line is {∅}, so every
minimal question answers it after one saturation. **Not built**:
[S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md),
which restates the `right` of W27's two minimal rows (`exhausted`, `others`).

## Q-M1e2.20 — Does a claim compare values, or their printed text?

Put to the user on 2026-11-06 by
[S1e2.3.8](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md)'s
step 1
([T1e2.3.8.1](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md#the-finding-a-claim-compares-printed-text)),
which stopped on it: the stage moves no answer by its brief, and the one
printer moves two.

**The finding.** A claim, a query's `:expect` and the solve form's
`(region …)`, and a query's projection classes compare facts by their
printed text, never by value: `ein_ir::expect::render` (the claim's facts,
from the program's text) against `ein_infer::events::sexpr` (the state's,
from the KB), which *"must agree byte for byte"*, and
`read_out::classes`, keyed by the goal's atoms as text. Until the one printer
a name printed as its raw text, so the int `1` and the name `"1"` were one
text, and so were `(p "x y")` and the two-argument `(p x y)`:

- [`claim-int-name.ein`](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/claim-int-name.ein),
  `(holds "1")` with `(query :goal (holds ?v) :expect (model (holds 1)))`:
  `ein test -v` says `holds` and exits 0. The claim names the int, the only
  state holds the name.
- [`claim-int-name-solve.ein`](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/claim-int-name-solve.ein),
  the solve form's `(region (model (holds 1)))` over the same state: `holds`,
  exit 0.
- [`classes-int-name.ein`](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_probes/classes-int-name.ein),
  a goal `(got ?v)` that the int `1` answers in some states and the name
  `"1"` in others: `ein solve` counts *48 states → 8 projection classes*,
  merging each class with the int and the name; by value they are 16.

Release and debug alike, at `51b9fa7e` (2026-11-05). S1e2.3.8's prototype of
the one printer moves all three: each claim `FAILED`, exit 1, its lines
naming `(holds "1")` against `(holds 1)`, and 16 classes. **No corpus program
and no register witness moves**: over the stage's sweep (35 328 runs on 736
files, every corpus program, the register's witnesses and every earlier
stage's probes, 2026-11-05) no exit code moves but the two claim probes', and
no claim's outcome and no count moves on any corpus program.

Put to the user the same day, with the agent's recommendation:

- **(a) The fix moves them, and the register says so.** The one printer's
  text is injective (a name and an int, a name with a space and two names,
  print apart), so the comparison by text becomes a comparison by value with
  no code of its own: the claims and the classes above move to right in
  S1e2.3.8's fixing commit. The case is filed as a register row, W28, `open`
  with the three probes banked as witnesses, and fixed by the same commit as
  the printer, as every wrong answer has been — **the agent's
  recommendation**. The register's schema needs a claim's outcome and a
  query's class count in `now` and `right` (a `/12`), which is T2's.
- **(b) The stage keeps the old comparison.** Claims and classes compare
  the raw text, as H's order does, so the stage moves no answer and the
  printed claim lines disagree with what they compare; W28 is filed `open`
  and deferred to a stage that compares by value.
- **(c) Compare by value now, apart from the printer.** Claims and classes
  read the facts' `repr`, as the round-trip test does: the same answers as
  (a), and a second comparison in `claim.rs` and `read_out.rs` that the
  printer would no longer need.

**Ruled 2026-11-06: (a), as recommended** (the user, through the
coordinator, the same day). The one printer's text is one-to-one with
values, so a claim and a query's classes compare by value with no comparison
of their own, and the three probes' answers move to right in the printer's
fixing commit. The case is W28, the three probes banked as its witnesses
(`examples/ein-bugs/wrong-answers/claim-int-name.ein`, `-solve`,
`classes-int-name.ein`, the last made smaller: eight states, two classes for
four), filed `open` by T1e2.3.8.2 under the register's schema `/12` (a
`now`'s `claim` and `queries`, and a refuted claim's exit 1), and fixed in
the commit that lands the printer. Owner:
[S1e2.3.8](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md) (W28).

**Built 2026-11-06** by S1e2.3.8 (`712f23fd`, T1e2.3.8.3), as ruled: the one
printer prints a value as Ein text the loader reads back to the same value,
so the claims and the classes, which compare its text, compare values. W28's
three rows answer right in either build: `claim-int-name` and
`claim-int-name-solve` FAILED, `ein solve` exiting 1 with the claim on
stderr, and `classes-int-name` four classes. No corpus program's claim or
class count moved.

One more move is the printer's alone, and is listed rather than asked: a
register row's recorded region is text, so `W18/str-in-match`'s
`[case.region]` reads `maximal = [["(hit 1)", …]]` where the library now
prints `(hit "1")`, the same candidate over the name `1` (the row's note says
so). T2 re-banks the text; no status, verdict or count moves.

## Q-M1e2.21 — Which rule does `refuted by` name where the core derives its contradiction through two?

Put to the user on 2026-11-06 by
[S1e2.3.10](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_refuted_by_and_report_rows.md)'s
step 1
([T1e2.3.10.1](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_refuted_by_and_report_rows.md#the-finding-a-core-derives-its-contradiction-through-two-rules)),
which paused on it: its brief stops where a printed core has two derivations
of its contradiction through different rules and the stage's method does
not pick one.

**What N7's fix does.** A dead node's `refuted by` named the rules of the
first contradiction `contradiction::detect` returns, each fact's primary,
which need not be a rule the printed core's derivation uses: at `50a33188`
65 559 distinct printed cores per commit name such a rule (67 928 nightly,
2026-11-06), `alt-cap-core.ein`'s `narrow` beside the core `(w1 X) (w2 X)`
that only `wide` refutes among them. The fix names, for each fact of the
contradiction the core was taken over (the search's own tie-break picks it),
the last step of a derivation from the core: a recorded justification whose
premises, activator and universal reads the core derives, or `(given)` for a
fact the core holds itself. One rule for a `(false …)`, up to two for a
pair, as before. The method says *"where two derivations give cores of the
same size, it names the one the core is"*: that picks the contradiction.

**The finding.** Within the contradiction, the core often derives one of its
facts through **two rules**: both are derivations of it from the printed
core, so *"the one the core is"* picks neither. **5 041 of the 230 154
distinct printed cores per commit, over 16 programs; 6 917 of 370 924 over
20 in the nightly's corpus** (`corpus_cores.rs` under the prototype,
release, 2026-11-06). `tests/stdlib/bijection/06_blind_enumeration.ein`'s
`(seats C1 Ann)`, both arguments ill-typed, refuted through
`typecheck-arg-0` and through `typecheck-arg-1`; `branching/02`'s
`(co-located Blue H1) (co-located Red H1) (is-a Blue Color) (is-a Red
Color)`, whose `(co-located Blue Red)` it derives through `symmetric` and
through `transitive`. The smallest witnesses are banked beside the stage
file:
[`two-rules.ein`](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_probes/two-rules.ein),
`(p A)` and two rules `:match (p ?x) :assert (false)`, `wide-a` firing
first, and `-reordered`, `narrow-b` first.

No answer moves under any option: no R, count, class, core's facts, exit
code or register row (the stage's sweep, 41 944 runs over 749 files, and
`cargo test` under the prototype). Each option prints one of the rules the
core's derivations end in, which the new claim holds on every corpus core.

- **(a) The first recorded derivation the core supports.** The fact's
  primary where the core derives it, else the first alternative the store
  keeps that it derives (fewest premises first), else the smallest-closure
  arrival. `two-rules` names `wide-a` and `-reordered` `narrow-b`, as at
  HEAD; `branching/02`'s core `sibling-exclusive, transitive`. It moves
  70 239 dead sets' rules over 26 corpus programs (the sweep, by distinct
  dead set), each from a rule its core does not use or from another
  contradiction than the core's. It keeps
  `:record-alternative-justifications false`'s exemption to the core: off,
  the store keeps each fact's primary, and where the core is the same the
  primaries are a derivation from it, so `refuted by` moves only where the
  core does. It depends on the store's order, the program's priorities and
  arrival in commitment order (no interner since S1e2.2.12): two programs one
  `:priority` apart can name two rules beside one core, as the probe pair
  does — **the agent's recommendation**.
- **(b) The least rule name among the rules whose step the core supports.**
  A function of the printed core and the program's rules alone, never of the
  firing order: `two-rules` and `-reordered` both `narrow-b`;
  `branching/02`'s core `sibling-exclusive, symmetric`. 1 663 dead sets over
  16 corpus programs print otherwise than under (a). The alternatives flag
  can then move `refuted by` beside an unmoved core (`two-rules.ein`:
  `narrow-b` with the alternatives, `wide-a` without), so the config
  principle's exemption grows from *the core* to *the core and its rules,
  apart*; on no corpus program whose R is known (`lever_invariance.rs` green
  under both).
- **(c) Every rule whose step the core supports**, sorted:
  `two-rules`' `narrow-b, wide-a`, `bijection/06`'s
  `typecheck-arg-0, typecheck-arg-1`. Nothing chosen and order-free, but not
  one rule per printed core, the method's default, and a list that grows with
  the program's redundancy; the alternatives flag moves it as (b)'s does.

**Ruled 2026-11-06: (a), as recommended** (the user, through the
coordinator, the same day). `refuted by` names the first recorded
derivation the core supports: the fact's primary where the core derives
it, else the first alternative the store keeps that the core derives
(fewest premises first), else the smallest-closure arrival.
`:record-alternative-justifications false` keeps its exemption as it
stands, `refuted by` moving only where the core moves. The `two-rules` pair,
two programs one `:priority` apart naming two rules beside one core, is the
accepted cost, and the pages say so. Owner:
[S1e2.3.10](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_refuted_by_and_report_rows.md) (N7).

**Built 2026-11-06** by S1e2.3.10 (`a3bb9e70`, T1e2.3.10.2), as ruled:
`explain::Explanation::steps` and `region::Node::refuted_by`.
`ein-infer/tests/corpus_cores.rs` holds every printed core's rules to the
first recorded step of each fact of its contradiction, over 274 programs and
230 154 cores per commit (5 041 through two rules) and 288, 370 924 and
6 917 nightly (2026-11-06); `lever_invariance.rs` holds the exemption. The
goldens moved are step 1's list, byte for byte; no answer, exit code or
register row moved.

## Q-M1e2.22 — Does Ein need a wildcard?

Raised 2026-11-06 by the user, reading W-22 in
[the no-owner inventory](inventory_2026-11-06_no_owner.md): *"what for do w need
the wildcard? why not allow _ in any namees and replace _ with ?_w0 ?_w1
?_wildcard ?etc; Why can't we just use simple variables insead of special wildcard
`_`?"*

**What the agent measured** (2026-11-06, on throwaway probes, at `916d97ac`). A
variable used once answers as `_` does: the same facts on seven shapes, the same
candidates from an hrule, and the same R on W20's three witnesses and on a
non-monotone program. In one place it does better: where a premise can match two
ways, `_` reported a four-fact core and the variable the two-fact one, at root and
at a dead node. No rule under `examples/`, `tests/` or `stdlib/` writes `_` (711
rule forms, Q-M1e2.13, 2026-11-04), and 10 of the 67 `(rule` and `(hrule` forms in
`stdlib/` already write a `:match` variable no other clause reads (the agent's
count, 2026-11-06). What `_` gives is convenience: no accidental join, a stated
intent, and familiarity.

**Three options were put**, on what a bare `_` means afterwards: (A) an ordinary
name, so `(holds _)` silently never fires, as W20 fixed; (B) reserved and refused,
with `_` free elsewhere in a name; (C) sugar, the loader rewriting each `_` to a
fresh variable. The agent recommended (B).

**Ruled 2026-11-06: (B).** The user: *"I agree, simple `_` reserved and refused
because of potential ambiguity, but remain valid in any place in atom and variable
names `_a_in_b_` `_______01` `A________B`"*.

It reverses [Q-M1e2.13](#q-m1e213--what-does-a-wildcard-or-a-range-in-a-rules-pattern-mean)
(a) for `_`, whose ruling on a range stands, and absorbs
[Q-M1e2.14](#q-m1e214--how-does-a-head-wildcard-match-every-relation) (c). **Not
built.** Owner: [S1f.5.1](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md),
which records the defaults the ruling leaves.

## Q-M1e2.23 — What is a string for?

Raised 2026-11-06 by the user, reading S-15: *"I actually don't remember when we
started using strings as atoms - objects and relations? Where ein allows usage of
C-style string literals?"*

**What the agent found** (2026-11-06, at `916d97ac`): no ruling. The first grammar gave
`STRING` to *source-sentence provenance + `:why` templates only* (`c44ccfb5`,
2026-05-18). The Rust loader held a fact's string as the name it spells from its first
commit (`687b91bf`, 2026-08-18; earlier not traced), `01_grammar.md` said so at
S1e2.2.16, and S1e2.2.18 extended it to rules by default (`259c16e4`, 2026-11-04). W16,
W18, W28 and S-15 came of it. Outside keyword values, 13 files write a string as a
value, 12 of them those defects' witnesses and the other `syntax/arg-kinds`. Put as (a)
keep them, the agent's recommendation, for names with a space or a non-ASCII letter, or
(b) back to the original limit.

**Ruled 2026-11-06: (b).** The user: *"I don't see any reason to use strings as names,
do you? I think we should allow use strings only for NL - :why, :source, and other
places where NL text or NL text template with variables lives; Is there any
objections?"* The agent raised none: `:why`, `:source` and `:goal-text` are the only
keywords in the tree that hold natural language, S1f.5.1's names replace every spelling
a string was needed for but a non-ASCII letter, and widening bare names for that needs
no string. It withdrew its recommendation.

It overrides S1e2.2.18's default. **Not built.** Owner:
[S1f.5.2](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md). **One
part is put to the user**: G-6, a variable or a range written in a fact, default
refused, since a state file could not reload a name printed `"?x"` or `"1..5"`.

**What a string holds, ruled the same day**, on F5: *"strings literals are unicode with
reasonable escape symbols, so potentially ein program can emit unicode text, use it in test,
dont force using it in examples but place a couple of examples just to show that option"*.
The agent reads *reasonable* as `\n` `\t` `\r` `\"` `\\`, a new `\u{…}`, and any other
`\X` refused, which S1f.5.2 shows before it builds.

## Q-M1e2.24 — Are there numbers in Ein?

Raised 2026-11-06 by the user, on being told G-6 covers *a range written in a fact*: *"Wait,
what range????? `1..5` - what it is? Where did you find it in Ein?"* The agent traced it to the
first grammar's *UML-style cardinality*, for a `:cardinality` no program has written and no code
reads ([F6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#f6-a-range-has-no-user)).

**Ruled 2026-11-06.** The user: *"we don't have native numbers in Ein and therefore we don't have
number ranges either, and never had and don't have plant to have numbers or ranges"*.

**Ranges.** The `RANGE` token goes, with
[S1f.5.3](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.3_range_retired.md). **Built 2026-11-08**
(T1f.5.3.2, `65d1ad85`): no terminal reads a range, and `1..5` was a parse error until M1f S1f.5.5's swap
(T1f.5.5.4, 2026-11-09) made it one name, S1f.5.4's ruling. It supersedes
[Q-M1e2.13](#q-m1e213--what-does-a-wildcard-or-a-range-in-a-rules-pattern-mean) on a range.

**Numbers.** No operation reads a value as a quantity: the built-in predicates are `eq` and
`neq`. The agent put to the user that the language still has an `INT` token and a value kind, on
[F7](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#f7-digits-in-a-value). **Ruled the same day:** *"YES, digits
are not numbers, digits are plain names, and refuse ?x in facts; (p 1) and (p 01) are VERY DIFFERENT
facts, (p "1") has to be refused"*. **Not built.** Owner:
[S1f.5.4](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md), with `(p "1")` and `?x` in
[S1f.5.2](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md).
[F8](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#f8-a-digit-label-becomes-a-candidate), whether a digit label
joins the universe, was put to the user. `predicates.rs`' *numeric … primitives are deferred to
followups* no longer states the intent, and S1f.5.3 corrects it.

**Ruled 2026-11-07, on F8 and on what a name is.** The user: *"yes, `0`, `1` - any alphanumeric plus
connectors signs (`_`,`-`) plus decorators signs (`$`, `%`, `^`, `*`, ...) string without space considered
a NAME, except variables that marked with `?` that sits in front of the name"*. **Not built.** A digit
label is a candidate like any other name. `-7` and `1..5` are plain names, where the plans had them refused,
and a reserved word is a whole token, so `neq-1` is a name, which overrides
[Q-M1e2.17](#q-m1e217--where-does-a-keyword-end) (i)(a). [S1f.5.5 § The name
rule](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#the-name-rule) holds the rule, what it moves and the
agent's readings: `?` and `:` only in front, `?_` refused, `#` and `|` not name characters, names ASCII.

**Ruled the same day, on those readings.** The user: *"`?` and `:` - only for variables and kvargs, not
allowed in other places; `?_` - refused; `;`,`#`,`|` - for comments, also not allowed in names; `,` commas
also not allowed; better specify list of allowed chars in name; names STAY ASCII FOREVER! никаких Жуков в
коде! В NL строках - пожалуйста, в строках что угодно, но не в коде. `:priority` planned to be deleted in
plans/m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.6_rule_priority.md"* (the
Russian: *no "Жук"s in the code; in natural-language strings, anything, but not in code*). **Not built.**
A name is spelled from a list of allowed characters: ASCII letters, digits, the connectors `_ - .` and
thirteen decorators, of which `$ % ^ *` are the user's and the other nine the agent's reading of *"..."*
([S1f.5.5 § The name rule](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#the-name-rule)). Names
are ASCII forever, which replaces S1f.5.2's default 3, *parked for M2*. `:priority` gets nothing new here:
it keeps a run of digits until [S1f.5.15](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md)
deletes it.

**Ruled the same day, on the agent's list.** The user: *"escape char `\` is forbidden in names, but others ' `
[ ] { } lets allow for now, so 'name' is an atom with name = "'name'", or f' is also an allowed atom and {} is
also"*. **Not built.** `\` is never in a name. `'`, `` ` ``, `[`, `]`, `{` and `}` are plain name characters
*for now*, with no meaning: `'name'` is a name, quotes included, and differs from `name`. The nine decorators
the agent read into *"..."* stand.

## Q-M1e2.25 — How strict is the grammar, and what holds the parser to it?

Raised 2026-11-06 by the user, in this review: *"We have to revisit Ein grammer; previously when where
using parser generators and EBNF it was easier, now seems that our hand-written parser is way too
permissive"*.

**What the agent found** (2026-11-06). The looseness is the grammar's, and the parser inherited it:
the generated parser's `grammar.lark` (deleted at `4c1a5b39`) has the same token regexes, no required
separator and one `?value` for every position, and S1a.1.1 built the hand-written parser at accept/reject
parity. What the rewrite lost is the single source: the grammar file was the parser, and now
`00_ebnf.md`, the parser and the loader are three statements, held together by tests. One `Value` for
every position also moves what a position may hold into `from_ir.rs`, refusal by refusal, and M2's GBNF
is generated from the same permissive `00_ebnf.md`.

**The instruction stands as a stage**: [S1f.5.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md), a
production for each position. The agent put two routes to the user: the hand-written parser held to the
EBNF by a test, which it recommended, or a parser generated at build time.

**Ruled 2026-11-07: generated.** The user: *"propose parser generator in Rust"*. This overrides the agent's
recommendation, and design/04 § 1 and design/12 § 2 as records of the earlier decision.

**Proposed the same day: LALRPOP, with its own lexer. Put to the user.** The agent wrote one grammar for six
generators and measured them against today's parser
([the probe](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_parser_probe/README.md)). LALRPOP fails the build on an
ambiguous grammar, where pest builds it in silence; it parses `zebra2.ein` in 240 µs against today's 165, plus
150 µs once per run; and it costs 5 crates at run time and 45 with the build. Its grammar file holds Rust
actions and has no public reader, so a small script takes the grammar back out, for `00_ebnf.md` and M2's
GBNF; a 167-line one agreed with the parser on 390 of 390 files. The runner-up is pest. **Default in the same
stage:** `defined_behaviour.md` § 1.1 to § 1.4's quirks go, and a parse error reports the true token position
and what was expected.

**Asked the same day: which generator supports the most?** The user: *"What parser will support more and
richer features?"* The agent's answer: **grmtools**, the only candidate with all four things the plans ask
of a parser: an ambiguous grammar fails the build, a public reader of the grammar file, the grammar loaded at
run time (F2), and recovery that reports every error of a run with ranked repairs (M20's diagnostics). The
last two were tried on Ein's draft: the grammar read at run time agrees with the compiled build on 390 of 390
files, and CPCT+ reported three of four planted mistakes in one run. It parses `zebra2.ein` in 2.1 ms, 1.9 ms
of it its lexer, and adds 41 crates at run time
([S1f.5.5 § Which supports the most](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#which-supports-the-most)).
**Ruled the same day: grmtools.** The user: *"Let's take grmtools now because of features, if speed will
ever become an issue - will add super-optimized hand-written parser"*. **Not built.** The grammar becomes
`ein-ir`'s `ein.l` and `ein.y`, generated at compile time, so a conflict fails the build; `cfgrammar` and
`lrlex` read them for `00_ebnf.md`'s blocks, in a test. Choosing it found
[F11](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so): CPCT+'s
report can differ between two runs on the same file, since equal-length repairs are ranked in a `HashSet`'s
order and recovery stops at a 500 ms wall-clock budget. The agent's default was to parse with recovery off.
**Ruled the same day: recovery on.** The user: *"no, enable recovery, let's benefit from full-featured
grmtools"*. **Not built.** S1f.5.5 vendors `lrpar` with a patch: equal repairs keep the order the search
found them in, and a count of search steps replaces the clock. On the probe the patched crate gave one report
per file over three runs of 5 000 corpus mutants, where the unpatched one differed on 1 032 (2026-11-07). A
refused file reports every error recovery reaches, each with up to three ranked repairs, and stays refused:
no repaired program is loaded. Sending the patch upstream was the user's call, and **ruled 2026-11-08: not
sent**. The user: *"don't send the lrpar patch upstream either"*, as on the Graphviz bug; it stays vendored,
`ein.rs/vendor/lrpar/ein.patch` (S1f.5.5's T3, `54715b5f`).
**S-15, ruled the same day, as recommended:** `ein solve --diff-models FILE` reads the file's facts through
the generated parser and compares them by value, so other spacing or a trailing comment still matches, and a
fact that does not parse is refused before the run. A task of S1f.5.5,
[T1f.5.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#task-t1f556----diff-models-compares-by-value-s-15).

## Q-M1e2.26 — Are `eq` and `neq` relations?

Raised 2026-11-07 by the user, reading the inventory's S-18: a query whose goal is a guard prints *the goal
holds: it has no variable* for `(neq w ?y)`, and a goal that holds a guard can carry no claim that holds,
since the class lists `(neq w a)` and the loader refuses a claim that names `neq`
([S1f.5.7 § The case](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md#the-case)). The agent
had proposed refusing a guard on its own in a goal and leaving it out of the class.

The user: *"the question is much deeper here; At some point I said that some forms that are not relations
facts (not implemented as relation facts) for example rules instantiations actually look like a relation
facts and actually have meaning as relation facts, for example in zebra2 co-located is a rule, but
(co-located nation-loc Englishman color-loc Red) it is naturally looks like a fact of relation `co-located`
and means a fact; I think the same here eq and neq are reserved words, tests, guargs, but we have to admit
that they work as a relations of equality and non-equality."*

**Ruled the same day: yes.** On the agent's three readings of a free variable (refuse it; `eq` binds and
`neq` is refused; list both over the program's names), the user: *"logically, result for (query :goal (neq
w ?y)) are all objects from the program except w - (model (neq w a)) (model (neq w b) ..., and for (query
:goal (eq w ?y)) - only w - single model (eq w w); It is trivial, but all the more reason to solve it
correctly instead of reject"*. So `eq` and `neq` are relations whose facts are fixed by name identity,
computed and never stored, and a query over them is answered: `neq` with a free side gives every name of the
program but the other, and `eq` binds the free side to the other. A claim may name them. **Not built**:
[S1f.5.7](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md).

**Ruled the same day, on the two questions and G-21.** The user: *"1 - default is true; 2 - we have to
follow logical correctness at first, so premises should follow the same rule; G-21 - yes, rule can conclude
wrong (eq x y) or (neq x x), what is the problem? I expect that correct meaningfull ein program will not, but
I don't want add logically unnecessary restrictions without a reason; Is there any?"* So the names are the
program's names list (Q-M1e2.9 (a)), relation and type names included; a rule's premise follows the same
reading, in any order of its conjuncts, which **supersedes Q-M1e2.18 (ii)(a)** and restates
`W24/guard-before-binder`'s `right`; and a rule may conclude either. The agent found no reason to refuse a
conclusion: a true one adds nothing, a false one is a contradiction as `(false)` is, names stay distinct, and
it adds no non-monotone site. Today such a conclusion is stored as a fact, so `(neq a a)` sits in root's KB
while a guard computes it false. **The same day, correcting the agent:** *"Seems we already decided not to use `_`? So (eq _ _), (neq ?x _) is
refused during load; (eq ?x :k a) is wrong for 2 reasons - there is no defined `:k` kvarg and `eq` is 2-ary
relation"*. So `_` in a guard stays refused (Q-M1e2.22) and `W24/wild-in-guard` keeps its `right`, and a
keyword pair as a guard's argument stays refused. **Ruled the same day, as the default:** `W24/form-in-guard`, a
nested fact as a guard's argument, is answered, since a nested fact is a value (the user: *"yes, keep that
default"*).

**F12, the same reading, ruled the same day.** An empty `:match` is the empty conjunction, true, so a rule or
an hrule with `:match ()` fires, once or once per activator, where today it never does; `(and)` stays
refused, since `and` takes a conjunct (the user: *"1 fire; 2 same; 3 reject (and) while it is the same as ()
and `and` should have arguments"*). **Not built**:
[S1f.5.9](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md).

**F13, where the reading stops, ruled the same day.** A conclusion's variable that no premise or parameter
binds, `(rule r () :match (p ?x) :assert (q ?y))`, logically concludes `(q y)` of every name. The user asked
its value; the agent found almost none, and the meaning stays writable explicitly, with a typed premise or
`(eq ?y ?y)`. So the implicit form is refused at load, a restriction with a reason: it forbids nothing that
can be meant, and catches a likely typo. The user: *"yes, file F13 with the refusal"*. **Not built**:
[S1f.5.10](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md).

## Q-M1e2.27 — What do `[]` and `null` mean in Ein's JSON?

Raised 2026-11-07 by the user, reading the inventory's D-14: an owing hole is written `"core": []` beside
`"core_status": null`, where S1e2.3.9 spells *no core* `null`, and the re-take found `ein oracle --dump` writing
`core: []` and `refuted_by: []` on every node that is not dead
([S1f.5.8 § The case](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md#the-case)).

The user: *"[] - meaning is something like "empty set"; null - meaning is "some single object does not
exist"; which fits better in which case?"* The agent's answer: a core is one object, a set of facts, so a node
with no contradiction has no core, `null`, and `[]` is a core that exists and is empty, *the empty set of
facts suffices*, which no corpus program reaches today (F12). A dead node whose core a run did not keep is
`null` with `core_status: "not-kept"` saying why. **Ruled the same day:** *"Yes, plan it that way"*. The rule
holds for every field of every JSON surface: `[]` where the set is known and empty, `null` where the object
does not exist or is not known. The summary's `holes` already follows it. **Not built**:
[S1f.5.8](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md).

## Q-M1e2.28 — Which keywords does each form take?

Raised 2026-11-07 by the user, reading the inventory's D-15, `02_patterns.md`'s `:where` rows: *"I don't
remember we had `:where` kvarg; we must have only fixed defined kwargs, list me all with description"*. The
agent listed every keyword, form by form, from the loader and the grammar page, and probed each form with an
unknown one on the release binary. Only `query`, `solve` and `config` refuse a keyword they do not know. A
`relation`, a `rule`, an `hrule`, a fact, an `import` and a trace `step` ignore one. A keyword pair inside a
term is dropped: in a `:match` (Q32's `:where`), a pattern, an `:assert`, a `:goal` and a macro call. So
`:match (and (p ?x) :where (neq ?x a))` fires on `(p a)`
([S1f.5.11 § The case](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md#the-case)).

The agent proposed a closed list per form and four defaults: `:cardinality` retired, never read and its value
a range; a query's `:mode` retired, obsolete and ignored; a fact's `:rule` and `:using` kept; `:why` on a fact
refused. The user: *"yes, plan to remove `:where`"*. On G-22, a keyword pair beside an `:assert`'s
conjuncts, the same day: *"G-22 already answered also - there are fixed set of allowed kwargs, if parser sees
:k or :G :notes or whatever else unknown kwarg - it is an error"*.

**Ruled:** each form takes the keywords its row of
[S1f.5.11 § The lists](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md#the-lists) names, and any
other keyword is an error, refused before anything runs. A term and a nested fact take no keyword, so
`:where` is refused wherever it is written. That settles G-15 and G-22 with D-15. It supersedes the half of
[Q32](../open_questions.md#q32--where-semantics) (2026-05-20) that left a `:where` pair to be dropped in
silence. Q32's move of a guard to a positional premise, `(neq ?a ?c)`, stands. A retired keyword gets no
hint of its own: it is an unknown keyword like any other. **Not built**:
[S1f.5.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md)'s generated parser makes them
parse errors at its swap, credited to
[S1f.5.11](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md), which keeps the pages and the
compiler's drops. Until the user's re-plan of 2026-11-08 S1f.5.11 was to check the lists in the loader first,
on the hand-written parser.
