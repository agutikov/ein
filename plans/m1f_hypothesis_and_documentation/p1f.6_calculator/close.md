# P1f.6 — the close, prepared (2026-11-15)

**Phase:** [P1f.6](README.md). Prepared 2026-11-15, once every stage was built. **The phase closed the same day**,
on the user's answers to [§ What the user is asked](#what-the-user-is-asked): [§ The close](#the-close-2026-11-15).
The sections before it are the page as the user read it. The pattern is the two closes before it,
P1f.5's and
P1f.7's: where the stages stand, the findings, the
defaults, the acceptance re-taken, and what is left for the user. Like them, it proposes no `docs/history/` record:
this directory stays where it is.

## Where the phase stands

- **Every stage is built, but S1f.6.39's T7.** [The stage table](README.md#the-stages) has 37 ids (18 and 19
  unassigned): 35 done or closed, S1f.6.39 at its T7 (below), and [S1f.6.14](s1f.6.14_expressions.md) moved to M1g's
  [S1g.10.2](../../m1g_logic/p1g.10_school_algebra/s1g.10.2_expressions_and_evaluation.md) (reply 10). The last
  builds landed on 2026-11-15, S1f.6.39's T1 to T5, S1f.6.16's build with S1f.6.17's T1f.6.17.7, S1f.6.32 and
  S1f.6.33, under gate #4, green on `fc0e5541`
  ([S1f.6.33 § The gate (#4)](s1f.6.33_lowest_terms_on_request.md#the-gate-4-for-the-batch-of-s1f632-and-s1f633)).
- **S1f.6.39's T6 is ruled.** The user, 2026-11-15: *"T6 rulings: all as you recommended"*, [Q-M1f.287](../open_questions.md#q-m1f287--one-pass-for-corpus_shapes-and-id_order_invariance) to [Q-M1f.294](../open_questions.md#q-m1f294--answer_dumps-heavy_dump-witness-and-the-feature-tests-the-pool-does-not-reach), each
  as [the stage's table](s1f.6.39_the_gate_made_fast.md#t6--the-audit-of-what-each-sweep-guards-and-what-it-needs)
  proposed it. **T7 applies them**, in a run of its own that records the rulings in those rows and in the stage file;
  this page restates none of them. [E-72](findings.md) is T7's ([Q-M1f.286](../open_questions.md#q-m1f286--id_order_invariance-keeps-its-plain-run), [Q-M1f.287](../open_questions.md#q-m1f287--one-pass-for-corpus_shapes-and-id_order_invariance)).
- **The question files are answered.** [`questions.md`](questions.md) and
  [`questions_round2.md`](questions_round2.md) have no reply outstanding
  ([`replies_2026-11-12.md` § What is still the user's](replies_2026-11-12.md#what-is-still-the-users)): section I
  stood, section J's rows took their recommendations, question 55 (a number printed as a number) is M1g's S1g.6.3's
  and 73 was left.
- **One item of the replies is no stage yet**: the cryptarithm theory, `std.math.crypt` (reply 42), which round 2
  places after item 39 ([S1f.6.27](s1f.6.27_ein_test_by_the_cheapest_route.md), done) and **after P1f.10**; it gets a
  stage file when P1f.10 has run ([§ The order](replies_2026-11-12.md#the-order)).
- **The register** reads 322 fixed, 16 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort |
  uniq -c`, 2026-11-15). The phase filed W88 to W95, 21 rows, each `open` and held to the unchanged engine before
  its fix, and all 21 are fixed; the 16 open rows are F21's, F22's, F24's and S1f.10.6's, none of them this phase's.
  No wrong answer was met preparing this close.

## The findings

[`findings.md`](findings.md), E-1 to E-75, read row by row on 2026-11-15:

| state | rows | which |
|---|---:|---|
| fixed or closed by a stage of this phase | 27 | E-2 (S1f.6.23); E-5, E-8, E-11, E-22 (S1f.6.25: `ab36658d`, `a4bf10f8`, `076bb20e`, `e9d692e0`); E-19 (S1f.6.26); E-27 (S1f.6.12's build); E-30, E-46 (S1f.6.27); E-33 (S1f.6.28, `41806bcd`); E-20, E-32, E-41, E-49, E-50, E-53 (S1f.6.35; E-53 is W93, `dc94b947`); E-38 (S1f.6.38's build); E-47, E-52 (S1f.6.34); E-58, E-59 (S1f.6.31); E-63, E-65, E-67 (S1f.6.37; W94 `0d144bef`, W95 `fc9bd066`); E-48 ([Q-M1f.226](../open_questions.md#q-m1f226--an-obligations-name-in-a-closing-rule-and-alive_invariants-r0) (b)); E-73 (S1f.6.39's T4, `13cdfbb1`); E-74 (S1f.6.16's build, `a2f9ca5a`) |
| ruled to stay as it is | 5 | E-4 (reply 44 (a)); E-28 (reply 50 (a)); E-62 and E-64 (accepted 2026-11-14); E-31's remainder ([Q-M1f.224](../open_questions.md#q-m1f224--what-a-bare-description-pays-for-n), defaulted; its free part built by S1f.6.27) |
| owned outside the phase | 23 | P1f.10: E-29, E-51, and its S1f.10.9 E-1, E-7, E-54; M1g's [S1g.6.2](../../m1g_logic/p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md): E-3, E-12, E-15, E-24, E-25, E-26; [S1g.6.3](../../m1g_logic/p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md): E-9, E-10, E-13, E-14; [P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md): E-18's claim ([Q-M1f.219](../open_questions.md#q-m1f219--a-querys-false-on-a-program-that-only-owes)), E-23; M1h's [P1h.4](../../m1h_beyond_word_problems/README.md#p1h4--the-formula-and-the-calculation): E-17; [F21](../../followups/f21_non_monotone_programs_decided.md): E-66; [F23](../../followups/f23_diagnostics_in_eins_words.md): E-21, E-68 (D-14); [F24](../../followups/f24_written_provenance.md): E-6; [Q-M1f.8](../open_questions.md#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it): E-16 |
| owned by this phase's S1f.6.39, T7 | 1 | E-72 |
| **open, no owner: a disposition proposed below** | 19 | E-34, E-35, E-36, E-37, E-39, E-40, E-42, E-43, E-44, E-45 (its remainder), E-55, E-56, E-57, E-60, E-61, E-69, E-70, E-71, E-75 |

**Checked against the tree** (2026-11-15, the gate profile's binary of `fc0e5541`): W88 to W95 `fixed` and held to
their right answers (`wrong_answers.rs`, passed); the tests the closed rows name pass where § Acceptance runs them;
E-32's copy of `bits.ein` is gone, both example directories importing `std.math.bits` (`c2902aca`). Three rows were
re-taken, and say so in their proposal: E-42, E-69 and E-71.

### Proposed dispositions, the user's to confirm

Each is also written into its row's last cell in [`findings.md`](findings.md), marked *proposed*. Nothing is moved
into a receiving plan until the user confirms.

| row | the finding, in a line | proposed | why |
|---|---|---|---|
| E-34 | an activator of another count than its rule's parameters activates nothing, in silence | **F23**, a new D row | the count check S1f.6.23 made for a relation's name and not for a rule's ([Q-M1f.203](../open_questions.md#q-m1f203--the-kernels-own-relations-and-an-activators-count)); a silent acceptance at load, D-8's and D-9's family; no answer moves |
| E-35 | a kernel marker's fact at another count loads; a rule derives a declared relation at another count through a parameter | **split**: the marker's count to **F23** with E-34, the parameter's to **M12** with E-55 | the second needs the loader to read what a relation parameter can hold, which it does not ([Q-M1f.202](../open_questions.md#q-m1f202--a-relation-written-through-a-parameter-and-the-declared-count)): rules as data. Since S1f.6.16's build every shared rule of the calculator writes through parameters; a miswired base would fail `arith_theory.rs`' and `binary_theory.rs`' answers |
| E-36 | saying that a value is a name ranges a guard over the program's names, at each operand | **M1g's S1g.6.2** | a cost of the matcher beside E-3 and E-12, which that stage owns; the remedy (one probe after a ranged guard, or a kernel *is a name* test) is the engine's |
| E-37 | two guards on one free variable mean two things in their two orders, against `01_grammar.md`'s bold *any order of its conjuncts* | **fix before close, a page**: the bold sentence restated to what the engine does, order counting between two guards on a variable nothing else binds | the engine does what the page's next sentence says, and `operand-undeclared` rests on it ([Q-M1f.208](../open_questions.md#q-m1f208--how-the-theory-says-that-an-operand-is-a-name)). The alternative, guards made order-free, is an engine change that moves that rule, and would go to S1g.6.2 with E-36 |
| E-39 | a tagged call written under the theory's own tag is answered as a request nobody stated | **ruled as it stands** | the tagged calls are an interface ([Q-M1f.189](../open_questions.md#q-m1f189--a-users-program-calling-stdariths-tagged-calls), S1f.6.30's T1), and since S1f.6.35 a program writes a tag only by listing it or by its full name, so a call under the theory's tag is made on purpose |
| E-40 | the review bundle's trace index names a step by its first conclusion where its rule has no `:why` | **F23** | a label that names one conclusion of several, reached only under `--no-require-why`; the step's own page has the whole line |
| E-42 | a load diagnostic reaches stderr and nothing else | **F23** | re-taken: S1f.6.34's `note:` prints on a `.einb` too (`examples/word_problems/kinds_listed.ein`, saved and solved), and `--json-summary` has no key that carries it; what surfaces a diagnostic reaches is F23's subject |
| E-43 | a cut region's last line names nothing where every state was shown | **F23** | two comment lines of a listing mis-say what was left out, with R, N, H and the exit right: D-3's and D-6's kind |
| E-44 | on an R empty by owing, root's own debts are on no stdout row where root is not on the frontier | **M1g's P1g.7** | E-23's neighbour, which P1g.7 owns with [Q-M1f.219](../open_questions.md#q-m1f219--a-querys-false-on-a-program-that-only-owes): the read-out of an R that is empty by owing |
| E-45 | no gate step runs `utils/read_out_examples.py` | **fix before close**: a `--check` of the script that `read_out_examples.rs` runs, as `digit_tables.rs` runs the table generator's | small, and AGENTS.md counts the page among the generated files with a `--check` or a test in the gate; the test holds the page and skips the variant's block, which is how the script stopped running unseen |
| E-55 | a generic rule of a module that comes whole makes S1f.5.18's check refuse a program that writes one of the module's rule names as an argument | **M12**, with E-69 and E-35's second half | S1f.5.18's check reads a rule's name written as an argument as the target a derived activator could bind (Q-M1f.41, .42): a question of rules as data |
| E-56 | eleven mutants of `std.algebra`'s and `std.typing`'s rules, masked by the lemmas a whole import brings | **ruled as it stands** | equivalent to their rules in every program, since every import brings its module whole (§ The mutation survivors); banked with the reason |
| E-57 | `ein test`'s refute rule refutes no region claim but `(false)` | **ruled with [Q-M1f.256](../open_questions.md#q-m1f256--which-claims-root-decides-under---no-search)'s default (a)** | the row is that default's consequence; its (b) would close it, and is the user's to choose |
| E-60 | any parameterised rule turns S1f.5.18's check on for the whole program | **ruled as it stands**, by S1f.6.16's ruling (a) | the user chose the shared file at the trial's measured cost; since the build every calculator program pays it. Bought back, if wanted, by M12 or S1g.6.2 |
| E-61 | a parameterised rule saturates 3 to 5 % slower than its written-out twin | **ruled as it stands**, by S1f.6.16's ruling (a) | as E-60 |
| E-69 | a parameterised rule a closing rule names makes a program that imports `std.algebra` a load error | **M12**, with E-55 | the same reading of S1f.5.18's check. Re-taken: the build connects `std.math.positional` by activators and names none of its rules as an argument, and `examples/beyond_arith/equation_two_unknowns.ein`, `std.algebra` beside the calculator, loads and holds its claim (`ein test`) |
| E-70 | a check that reads an `absent` comes after every calculation | **ruled as it stands** | the documented semantics of `absent` ([`absent_semantics.md` § Evaluation points](../../../docs/ein/reasoning/kb-evolution/absent-and-naf.md#the-boundary)), and why S1f.6.38's design reads `written` |
| E-71 | a reloaded state file states more `written` facts than its state held | **F24** | re-taken: a state file writes a derived fact with no `:rule` (none in `tests/stdlib/math/decimal-arith/01_add.ein`'s), so the loader cannot tell it from a written one; what a written fact's provenance may claim is F24's question |
| E-75 | a parameterised conclusion's nested fact is not stated as written | **ruled as it stands** | [`diagnostics.md` § 2.30](../../../docs/ein/user/reference/diagnostics.md#230-written-is-the-loaders-what-a-file-writes-stated-at-load) states a conclusion's nested facts *that hold no variable*, and a parameter is a variable; no check reads `written` of a term a parameter heads. Stating it once per activator is what the page's *a nested fact a rule assembles from a variable is stated by nothing* rules out |

In all: **F23** five (E-34, E-35's first half, E-40, E-42, E-43), **M12** three (E-35's second half, E-55, E-69), **M1g's
S1g.6.2** one (E-36), **M1g's P1g.7** one (E-44), **F24** one (E-71), **fixed before the close** two (E-37, a page;
E-45, a `--check`), **ruled as it stands** seven (E-39, E-56, E-57, E-60, E-61, E-70, E-75).

## The defaults

Every Q-M1f row this phase raised, [Q-M1f.135](../open_questions.md#q-m1f135--which-facts-a-flat-import-brings) to [Q-M1f.303](../open_questions.md#q-m1f303--rat-simplify-asks-stdmathgcd-the-numerator-first), by its state on 2026-11-15:

- **Ruled, confirmed, answered or accepted by the user** (57): .146, .147, .149 to .151, .153, .158, .163, .166 to
  .168, .173 to .175, .179 to .183, .185 to .189, .198, .210, .225, .226, .231 to .236, .247 to .249, .263 to .266,
  .269, .271 to .277, and **.287 to .294, ruled 2026-11-15 as recommended, T7 applying them**. Not repeated here.
- **Taken by the user's silence on 2026-11-12** (31): section I of [`questions.md`](questions.md), *"each of these
  stands unless you object"*, came back empty: .135 to .145, .148, .152, .154 to .157, .159 to .162, .164, .165,
  .169 to .172, .176 to .178, .184. Three of them, .135, .140 and .141, on what a selection brings, are moot since
  S1f.6.35 brings every module whole.
- **Defaulted, the user's to revise** (81): the table. *Touches* says what a revision would move: **ships**, what a
  user of `ein` or of the stdlib sees (the language, a surface, a module's answer); *tests*, the suites, the gate, the
  corpus or a measurement's method; *docs*; **moot**, superseded by a later build, which the close proposes to mark so
  in the row.

| row | the question | the default taken | touches |
|---|---|---|---|
| [Q-M1f.190](../open_questions.md#q-m1f190--which-variants-of-the-binary-prototypes-are-banked-where-the-page-recommends-none) | which variants of the binary prototypes were banked where the page recommends none | the simplest its results hold under | tests |
| [Q-M1f.191](../open_questions.md#q-m1f191--the-generators-of-the-binary-examples-are-run-by-hand) | the binary examples' generators, run by hand | `gen_binary.py` and `gen_gates.py` in the example directories, outside the gate | **moot**: `gen_binary.py` retired (`c2902aca`); `utils/gen_gates.py` is in the gate (`bool_theory.rs`) |
| [Q-M1f.192](../open_questions.md#q-m1f192--one-table-file-in-two-example-directories) | one table file in two example directories | a copy, held equal by `gen_binary.py --check` | **moot**: the copy retired, both import `std.math.bits` (`c2902aca`) |
| [Q-M1f.193](../open_questions.md#q-m1f193--the-sizes-of-the-banked-binary-programs) | the sizes of the banked binary programs | three inputs a search, operands to seventeen bits, none `slow` | tests |
| [Q-M1f.194](../open_questions.md#q-m1f194--which-of-the-word-problem-prototypes-programs-are-banked-and-under-what-names) | which word-problem programs were banked, under what names | the recommended variant of each choice | tests |
| [Q-M1f.195](../open_questions.md#q-m1f195--one-file-for-the-two-step-problem) | one file for the two-step problem | `boxes12.ein`, `readings` imported | tests |
| [Q-M1f.196](../open_questions.md#q-m1f196--the-word-problem-modules-as-corpus-entries) | the word-problem modules as corpus entries | a `positive` entry each; `gaps.ein` states no `solve` | tests |
| [Q-M1f.197](../open_questions.md#q-m1f197--what-holds-the-banked-word-problems) | what holds the banked word problems | each file's `:expect`; no test of their own | tests |
| [Q-M1f.199](../open_questions.md#q-m1f199--how-the-skipped-views-of-the-dot-sweep-are-pinned) | how the DOT sweep's skipped views are pinned | the count printed, their directories named in the test | tests |
| [Q-M1f.200](../open_questions.md#q-m1f200--the-names-of-an-integer-divisions-results) | the names of an integer division's results | `int-quot-euclid-of`, `-trunc-of`, `-floor-of`, each `(… X Y Q R)` | **ships** |
| [Q-M1f.201](../open_questions.md#q-m1f201--what-a-bare-relation-r-declares-of-its-count) | what a bare `(relation R)` declares of its count | none: `(R)` is its one form | **ships** |
| [Q-M1f.202](../open_questions.md#q-m1f202--a-relation-written-through-a-parameter-and-the-declared-count) | a relation written through a parameter, and the declared count | not read (E-35) | **ships** |
| [Q-M1f.203](../open_questions.md#q-m1f203--the-kernels-own-relations-and-an-activators-count) | the kernel's own relations, and an activator's count | neither held, each a finding (E-34, E-35) | **ships** |
| [Q-M1f.204](../open_questions.md#q-m1f204--which-written-places-the-declared-count-is-held-in) | which written places the declared count is held in | a fact's `:using`, a claim's facts, a macro's body where called; no `(solve …)` list | **ships** |
| [Q-M1f.205](../open_questions.md#q-m1f205--where-the-check-of-a-malformed-number-runs) | where the malformed-number check runs | a pass of its own at the operand | **ships** |
| [Q-M1f.206](../open_questions.md#q-m1f206--how-a-fault-reaches-its-request) | how a fault reaches its request | `(bad-operand TAG FAULT)`, ten rules from a tag to its request, a rule a flag | **ships** |
| [Q-M1f.207](../open_questions.md#q-m1f207--what-a-let-that-cannot-be-read-flags) | what a `let` that cannot be read flags | the `let` itself, whether or not a request uses it | **ships** |
| [Q-M1f.208](../open_questions.md#q-m1f208--how-the-theory-says-that-an-operand-is-a-name) | how the theory says that an operand is a name | two guards on a free variable, `(neq ?v z) (eq ?v ?x)`, in that order (E-36, E-37) | **ships** |
| [Q-M1f.209](../open_questions.md#q-m1f209--the-binary-example-theory-and-the-check) | the binary example theory and the check | `gen_binary.py` leaves the check out | **moot**: binary's numeral is `std.math.binary`'s, checked since S1f.6.16's build |
| [Q-M1f.211](../open_questions.md#q-m1f211--which-fact-carries-the-flag-of-a-division-by-zero-at-the-call) | which fact carries a division by zero's flag at the call | (ii): the call's, and a public request's beside it, two flags a public division | **ships** |
| [Q-M1f.212](../open_questions.md#q-m1f212--a-theorys-wrong-answer-and-the-register) | a theory's wrong answer and the register | not a register row: the register holds the engine's | tests |
| [Q-M1f.213](../open_questions.md#q-m1f213--the-binary-example-theory-and-the-flag-at-the-call) | the binary example theory and the flag at the call | `gen_binary.py` carries the rule over | **moot**: the rule is `std.math.positional`'s, shared by both bases |
| [Q-M1f.214](../open_questions.md#q-m1f214--the-derives-line-of-a-firing-with-several-conclusions) | the *Derives* line of a firing with several conclusions | one line, every conclusion in order | **ships** |
| [Q-M1f.215](../open_questions.md#q-m1f215--a-qualified-import-and-what-a-module-states-as-a-program) | a qualified import and a module's `(query …)`, `(solve …)`, `(config …)` | a wrong answer, W90; no import brings them, nor `(trace …)` | **ships** |
| [Q-M1f.216](../open_questions.md#q-m1f216--a-state-file-that-re-writes-less-than-the-program-as-loaded-and-the-register) | a state file that re-writes less than the program as loaded | a register row, W91; a listed name live in the tree-shake | **ships**; the tree-shake retired by S1f.6.35, W91 fixed |
| [Q-M1f.217](../open_questions.md#q-m1f217--when-the-loader-says-that-a-selection-left-a-relations-rules-behind) | when the loader says a selection left a relation's rules behind | one `note:` a form | **moot**: the line retired with E-20 by S1f.6.35 |
| [Q-M1f.218](../open_questions.md#q-m1f218--what-a-count-past-the-runs-reach-says) | what a count past the run's reach says | exact, reported up to 2^63 − 1, *or more* | **ships** |
| [Q-M1f.219](../open_questions.md#q-m1f219--a-querys-false-on-a-program-that-only-owes) | a query's `(false)` on a program that only owes | no change; the row says which empty R; P1g.7's | **ships** |
| [Q-M1f.220](../open_questions.md#q-m1f220--the-form-of-the-rows-for-what-root-owes) | the form of the rows for what root owes | `owed` rows under `∅`, four on stdout and a line | **ships** |
| [Q-M1f.221](../open_questions.md#q-m1f221--one-question-a-file-and-the-fall-back) | one question a file, and the fall-back | the least question that decides the claims; describe for what it leaves | **ships** |
| [Q-M1f.222](../open_questions.md#q-m1f222--a-claim-refuted-by-a-state-a-run-found) | a claim refuted by a state a run found | under `ein test`, a found state of an unlisted class refutes | **ships** |
| [Q-M1f.223](../open_questions.md#q-m1f223--what-the-surfaces-of-ein-test-say-of-the-question) | what `ein test`'s surfaces say of the question | `-v` names it; `ein-test-report/5` with `route` and `fell_back_from` | **ships** |
| [Q-M1f.224](../open_questions.md#q-m1f224--what-a-bare-description-pays-for-n) | what a bare description pays for N | no change: a bare `ein solve` takes N's transversals (E-31) | **ships** |
| [Q-M1f.227](../open_questions.md#q-m1f227--how-a-run-of-the-biggest-calculation-is-held-to-about-16-gb-and-how-long-it-may-take) | how the biggest calculation is held to about 16 GB | a cgroup a run, 16.0 GB, no swap, 600 s | tests |
| [Q-M1f.228](../open_questions.md#q-m1f228--what-the-interface-of-the-tagged-calls-holds-beyond-the-seven-names-round-2-showed) | what the tagged calls' interface holds beyond round 2's seven names | what the callers write: `sub-walk`, `short`, `operand`, `operand-is`, a carry in | **ships** |
| [Q-M1f.229](../open_questions.md#q-m1f229--what-the-census-of-imported-names-reads-and-how) | what the census of imported names reads | the stdlib's modules and the example theories, statically | tests |
| [Q-M1f.230](../open_questions.md#q-m1f230--how-the-proposals-are-prototyped-and-priced) | how the import proposals were prototyped and priced | `std.arith` alone under `$EIN_STDLIB`, two sweeps | tests |
| [Q-M1f.237](../open_questions.md#q-m1f237--how-a-name-prints-under-h) | how a name prints under (h) | the main file's short name, else the full name | **ships** |
| [Q-M1f.238](../open_questions.md#q-m1f238--a-full-name-of-a-module-no-import-brings) | a full name of a module no import brings | refused at load | **ships** |
| [Q-M1f.239](../open_questions.md#q-m1f239--a-declaration-under-an-imported-modules-full-name) | a declaration under an imported module's full name | refused at load | **ships** |
| [Q-M1f.240](../open_questions.md#q-m1f240--entry-rules-and-__closed__-lists-under-h) | entry rules and `__closed__` lists under (h) | they stay in the shipped modules | **ships** |
| [Q-M1f.241](../open_questions.md#q-m1f241--a-declared-short-name-with-a-dot) | a declared short name with a dot | refused at load | **ships** |
| [Q-M1f.242](../open_questions.md#q-m1f242--a-dead-or-redundant-hypothesis-declaration) | a dead or redundant hypothesis declaration | one `note:` on stderr, no exit moved | **ships** |
| [Q-M1f.243](../open_questions.md#q-m1f243--an-obligation-read-as-declared-or-as-owed) | an obligation, read as declared or as owed | as declared | **ships** |
| [Q-M1f.244](../open_questions.md#q-m1f244--o--c-beside-an-hrule) | O + C beside an hrule | a conflict whatever generates | **ships** |
| [Q-M1f.245](../open_questions.md#q-m1f245--where-the-conflict-check-runs) | where the conflict check runs | at load, on every command, on a second load's root | **ships** |
| [Q-M1f.246](../open_questions.md#q-m1f246--the-corpuss-twenty-four-programs-in-conflict) | the corpus's twenty-four programs in conflict | each corrected, one kept as the fixture | tests |
| [Q-M1f.250](../open_questions.md#q-m1f250--a-listed-name-a-module-states-facts-of-and-does-not-declare) | a listed name a module states facts of and does not declare | a name, bound to that relation | **ships** |
| [Q-M1f.251](../open_questions.md#q-m1f251--a-prefixed-name-that-names-nothing-the-module-declares) | a prefixed name that names nothing the module declares | refused at load | **ships** |
| [Q-M1f.252](../open_questions.md#q-m1f252--an-empty-segment-or-a--in-a-modules-name) | an empty segment or a `/` in a module's name | refused at load | **ships** |
| [Q-M1f.253](../open_questions.md#q-m1f253--how-a-name-prints-under-as) | how a name prints under `:as` | the alias spelling, `a.sum-of` | **ships** |
| [Q-M1f.254](../open_questions.md#q-m1f254--s1f634s-conflicts-beside-solve-no-search) | S1f.6.34's conflicts beside `(solve :no-search)` | (a): every conflict stands, read as declared | **ships** |
| [Q-M1f.255](../open_questions.md#q-m1f255--what-ein-solve---no-search-prints) | what `ein solve --no-search` prints | (a): root, and an `R` row *not searched* | **ships** |
| [Q-M1f.256](../open_questions.md#q-m1f256--which-claims-root-decides-under---no-search) | which claims root decides under `--no-search` | (a): what a description stopped after root decides (E-57) | **ships** |
| [Q-M1f.257](../open_questions.md#q-m1f257--ein-test---no-searchs-exit-code) | `ein test --no-search`'s exit code | (a): a claim not checked moves no exit code | **ships** |
| [Q-M1f.258](../open_questions.md#q-m1f258--which-commands-and-options-take---no-search) | which commands and options take `--no-search` | (a): `ein solve` and `ein test`; `ein oracle` refuses it | **ships** |
| [Q-M1f.259](../open_questions.md#q-m1f259----json-summary-under---no-search) | `--json-summary` under `--no-search` | (a): `asked.ask = "no-search"`, `region` and `verdict` null, root's standing in `root` | **ships** |
| [Q-M1f.260](../open_questions.md#q-m1f260--which-of-s1f634s-twenty-four-say-no-search) | which of S1f.6.34's twenty-four say `:no-search` | (a): the 21 whose question was root alone | tests |
| [Q-M1f.261](../open_questions.md#q-m1f261--the-decimal-theorys-module-name-after-the-user-wrote-stdmatharith-in-passing) | the decimal theory's module name | `std.math.decimal-arith`, entry `decimal-arith` | **ships** |
| [Q-M1f.262](../open_questions.md#q-m1f262--a-records-link-to-a-file-the-layout-moved) | a record's link to a file the layout moved | the target re-pointed, the text as it was | docs |
| [Q-M1f.267](../open_questions.md#q-m1f267--the-binary-theorys-names) | the binary theory's names | `b2`; every other name `bin-`; `to-bin`, `bin-of`, `to-dec`, `dec-of` | **ships** |
| [Q-M1f.268](../open_questions.md#q-m1f268--how-far-binary-flags-a-malformed-request) | how far binary flags a malformed request | as far as decimal | **ships** |
| [Q-M1f.270](../open_questions.md#q-m1f270--the-connecting-macros-body-written-or-generated) | the connecting macro's body, written or generated | written by hand in `std.math.positional`, held by a test | **ships** |
| [Q-M1f.278](../open_questions.md#q-m1f278--how-a-module-below-let-flags-a-named-constant-inside-a-numeral) | how a module below `let` flags a named constant inside a numeral | `named` is `std.math.decimal`'s, written by `let-named` | **ships** |
| [Q-M1f.279](../open_questions.md#q-m1f279--what-a-written-fact-is-derived-by-and-where-it-shows) | what derives a `written` fact, and where it shows | the loader's step `#written`, shown where the mirrors show | **ships** |
| [Q-M1f.280](../open_questions.md#q-m1f280--a-program-that-writes-written) | a program that writes `written` | refused at load | **ships** |
| [Q-M1f.281](../open_questions.md#q-m1f281--a-let-that-names-a-digit) | a `let` that names a digit | the `let`'s fault | **ships** |
| [Q-M1f.282](../open_questions.md#q-m1f282--the-flag-of-an-integer-division-whose-operand-is-at-fault) | the flag of an integer division whose operand is at fault | two flags, the tag's and the request's | **ships** |
| [Q-M1f.283](../open_questions.md#q-m1f283--the-two-new-modules-names-and-their-place) | the two new modules' names and their place | entries `arith` and `decimal`; `z` a relation of no argument | **ships** |
| [Q-M1f.284](../open_questions.md#q-m1f284--the-worker-pools-count-its-heavy-lane-and-its-memory-bound) | the worker pool's count, its heavy lane and its memory bound | 16 workers, a quarter of them the heavy lane, 24 GB | tests |
| [Q-M1f.285](../open_questions.md#q-m1f285--the-nightlys-and-the-dev-hosts-test-steps-on-the-gate-profile) | the nightly's and the dev host's test steps on the gate profile | every test step of every tier | tests |
| [Q-M1f.286](../open_questions.md#q-m1f286--id_order_invariance-keeps-its-plain-run) | `id_order_invariance` keeps its plain run | as built (E-72); [Q-M1f.287](../open_questions.md#q-m1f287--one-pass-for-corpus_shapes-and-id_order_invariance), ruled since, is T7's | tests |
| [Q-M1f.295](../open_questions.md#q-m1f295--binarys-numeral-is-stdmathbinarys) | binary's numeral | a module `std.math.binary`, on `std.math.decimal`'s pattern | **ships** |
| [Q-M1f.296](../open_questions.md#q-m1f296--the-bit-tables-by-the-digit-tables-generator) | the bit tables | `utils/gen_arith_tables.py` writes both tables, one `--check` | tests |
| [Q-M1f.297](../open_questions.md#q-m1f297--binarys-tagged-calls-an-interface-what-stdmathconvert-reads) | binary's tagged calls; what `std.math.convert` reads; a `(bin-arith-rule …)` fact | an interface; binary's circuit, as is; none | **ships** |
| [Q-M1f.298](../open_questions.md#q-m1f298--the-two-shared-sentences-that-wrote-decimals-numeral) | the two shared sentences that wrote decimal's numeral | the base's constructor in the sentence | **ships** |
| [Q-M1f.299](../open_questions.md#q-m1f299--gcd0-0) | gcd(0, 0) | zero, and lcm(0, 0) zero | **ships** |
| [Q-M1f.300](../open_questions.md#q-m1f300--stdmathgcds-tagged-call-an-interface) | `std.math.gcd`'s tagged call | an interface, `(gcd-call TAG X Y)` answered by `(gcd-done TAG G)` | **ships** |
| [Q-M1f.301](../open_questions.md#q-m1f301--what-gcd-call-does-before-euclids-chain) | what `gcd-call` does before Euclid's chain | each number put into normal form first | **ships** |
| [Q-M1f.302](../open_questions.md#q-m1f302--what-stdmathrats-unreduced-answer-fixes-and-rat-norm-under-b) | what `std.math.rat`'s unreduced answer fixes | the numerator in normal form, D with no high zero, nothing reduced | **ships** |
| [Q-M1f.303](../open_questions.md#q-m1f303--rat-simplify-asks-stdmathgcd-the-numerator-first) | which term `rat-simplify` hands `std.math.gcd` first | the numerator, as before | **ships** |

## Acceptance, re-taken

[§ Acceptance](README.md#acceptance), bullet by bullet, on 2026-11-15, the gate profile's binary and tests of
`fc0e5541` (HEAD's code). Run here: `ein test tests/`, **265 files, 460 expectations held**, and the five calculator
example directories, 68 files, 137 held; and twelve test binaries, **all green**: `cargo test --profile gate -p
ein-infer -p ein-cli -p ein-ir --test arith_theory --test int_theory --test rat_theory --test num9_theory --test
gcd_theory --test binary_theory --test bool_theory --test stdlib_coverage --test arith_search_free --test
import_facts --test wrong_answers --test digit_tables`. The batch and the mutation census were not re-run: each is
cited from the last stage that ran it.

| bullet | | what holds it |
|---|---|---|
| **Importable** | held | `tests/stdlib/math/decimal-arith/` and `num9/` under `ein test`; `arith_theory.rs`' `every_small_request_is_right`, `requests_of_every_length_are_right`; `num9_theory.rs`. `std.arith` is `std.math.decimal-arith` since S1f.6.31 |
| **A library of facts importable flat**, the register's rows fixed | held | W88's four rows `fixed` (`wrong_answers.rs`); `import_facts.rs`; since S1f.6.35 every import brings its module whole |
| **Search-free, held by a test** | held | `arith_search_free.rs`' `every_arithmetic_program_is_search_free`; `arith_theory.rs`' `the_theory_writes_one_absent_over_let`, one `absent` a base: over `let` in `std.math.decimal-arith`, over `bin-let` in `std.math.binary-arith` |
| **Right, held by a test** | held, with the rationals' normal form as ruled | `digit_tables.rs` (both tables, every row); `arith_theory.rs`' `high_zeros_are_read_by_value` and the harness; `num9_theory.rs`, exhaustive at width 2; `binary_theory.rs`, `int_theory.rs`, `rat_theory.rs`, `gcd_theory.rs`. A rational's answer is unreduced since S1f.6.33 ([Q-M1f.183](../open_questions.md#q-m1f183--lowest-terms-by-stdariths-division-and-what-it-costs) ruled (b)), in the normal form [Q-M1f.302](../open_questions.md#q-m1f302--what-stdmathrats-unreduced-answer-fixes-and-rat-norm-under-b) states; `rat-simplify` gives lowest terms |
| **The rule of the theory** | held | `arith_theory.rs`' `no_rule_joins_two_facts_on_a_number`, every module of the calculator; the batch by hand, whole, by S1f.6.33 ([its record](s1f.6.33_lowest_terms_on_request.md#record-2026-11-15), 2026-11-15) |
| **Division by zero**, its flag and then `(false)`; overflow `std.num9`'s alone | held | `a_division_by_zero_is_flagged_and_false`, `a_tagged_call_by_zero_is_flagged_whoever_made_it`; `division-by-zero-is-false` in `std.math.arith`; `num9/02_add_overflow.ein`, `06_mul_overflow.ein` and `num9_theory.rs` |
| **Malformed numbers** | held | `a_malformed_request_is_flagged_once_and_never_answered`, `an_undeclared_name_is_flagged_and_a_let_answers_it`, `a_let_that_cannot_be_read_is_flagged`, `what_stays_silent`; since S1f.6.38 the numeral checked where it is written, `tests/stdlib/math/decimal/` and `binary/`, what stays silent in `stdlib/README.md` and `decimal/05_what_stays_silent.ein` |
| **The stdlib's own gates** | held | `stdlib_coverage.rs` (every rule activated, every program an `:expect`); the census covers every new module, 498 of 617 killed and the 119 survivors the banked set (S1f.6.33, 2026-11-15); every generator's `--check` in the gate: `gen_arith_tables.py` by `digit_tables.rs`, `gen_num9.py` by `num9_theory.rs`, `gen_gates.py` by `bool_theory.rs`, `gen_arith_examples.py` by `arith_search_free.rs`' `the_generated_examples_are_what_the_generator_writes`; `gen_binary.py` retired. `utils/read_out_examples.py`, no generator of the calculator's, has none (E-45) |
| **The extensions** | held | `std.math.int` and `std.math.rat`, the four operations each, `int_theory.rs` and `rat_theory.rs`, programs under `tests/stdlib/math/int/` and `rat/`; the rationals' normal form as above |
| **The expression layer** built only if ruled in | held | not built: moved to M1g's S1g.10.2 (reply 10) |
| **The investigation** a page whose claims carry their commands; a wrong answer a register row before its fix | held, by the stages' records | [`word-problems.md`](../../../docs/ein/user/guide/word-problems.md), each probe with its command (not re-read claim by claim here); W93 (`50fe016e`), W94 and W95 (`eccab155`) filed `open` before their fixes |
| **Every relation and rule carries a `:why`** | held | the loader requires it (S1f.5.14); the stdlib and `tests/` load without `--no-require-why` |
| **`findings.md` has no row without a ruling, a fix or an owner** | **not held** here; **held at the close** | 19 rows, [§ The findings](#the-findings); held once the user confirms the dispositions proposed there, which the user did ([§ The close](#the-close-2026-11-15)) |

## The mutation survivors

The phase banked **72** survivors and saw **3** banked ones killed: the bank was 50 lines before it and is 119
(`git diff 76ede241 HEAD -- tests/mutation_survivors.txt`, 2026-11-15). The three killed are `bijective-setup`'s two
exchanged activators (S1f.6.34's fixtures) and `converse-illtyped-dom` without its `neq` (S1f.6.35's fix of E-53).
Each of the 72 has its reason in [`tests/README.md`](../../../tests/README.md):

| stage | lines | the reason's section | verdict |
|---|---:|---|---|
| S1f.6.12's build | 12 | [`stdlib/math/decimal-arith/`](../../../tests/README.md#stdlibmathdecimal-arith-a-theory-whose-answers-a-program-cannot-check) | **same R, other facts**: `operand-undeclared` without one of its ten digits' `neq`, `bad-request-undeclared`'s conclusion and `undeclared-is-false`'s premise exchanged (those two `std.math.positional`'s since S1f.6.16). Each differs only in a program a flag already kills, by a second flag or the same flag spelt otherwise, so R is empty either way; `arith_theory.rs` holds the flags' text |
| S1f.6.24 | 1 | the same | same R, other facts: `div-by-zero`'s premise exchanged; a public division by zero has two flags, either one `(false)`; `arith_search_free.rs` holds the two |
| S1f.6.35 | 11 | [the census of 2026-11-14](../../../tests/README.md) (*431 of 502*) | **equivalent** in every program: eight `(converse …)` premises exchanged, `symmetric` either way, `type-hierarchy-converse`; the lemmas `converse-pair-symmetric` and `symmetric-is-self-converse` come with every import of the module, so each mutant derives its rule's facts (E-56) |
| S1f.6.17's build | 18 | [`stdlib/bool/`](../../../tests/README.md#stdlibbool-the-sixteen-functions-by-rules-and-a-circuit) | **equivalent** over the module's closed tables: thirteen `(inv …)` premises exchanged, `inv` its own converse on the two bits; five conclusions exchanged, each relation an involution over the sixteen columns |
| S1f.6.38's build | 22 | [`stdlib/math/decimal/`](../../../tests/README.md#stdlibmathdecimal-a-numeral-checked-where-it-is-written) | same R, other facts: the four flags of `std.math.decimal`, six of `std.math.int` and six of `std.math.rat` with their arguments exchanged, the three `…-is-false` with their premise's variables exchanged (equivalent outright), `int-bad-div` either way; **and `numeral-prefix-named` without `(neq ?m z)`, which is not equivalent**: below |
| S1f.6.16's build | 8 | [the second base](../../../tests/README.md#stdlibmathbinary-arith-binary-bits-and-convert-the-second-base) | binary's twins of the line above: same R, other facts, **but `bin-numeral-prefix-named` without `(neq ?m z)`**: below |

**The pair the records call equivalent is not.** `numeral-prefix-named` without its `(neq ?m z)` flags every numeral
that ends in `z` wherever `(named z)` holds. Through the protocol `(named z)` comes only from a `(let z …)`, which
`let-zero` flags already, so the program is dead either way, which is the reason `tests/README.md` gives. But a
program may write `(named z)` itself, listing `named` from `std.math.decimal`, and then the shipped rule flags nothing
and the mutant flags `(n z 3)`:

```lisp
(import std.math.decimal-arith :symbols (decimal-arith))
(import std.math.decimal :symbols (n named))
(import std.math.arith :symbols (z))
(relation price Item Amount :why "{?1} costs {?2}")
(__closed__ price)
(price apple (n z 3))
(named z)
(solve :expect (region (model)))
```

`ein test` on it: held on the shipped stdlib, **FAILED** on a copy with the guard dropped, *the claim lists 1 state
and R is empty* (`EIN_STDLIB=<the copy>`, the gate profile's binary, 2026-11-15). Binary's twin is killed the same
way, `(bin-named z)` beside `(b2 z 1)`; and a `(let z …)` beside a numeral ending in `z`, decimal's or binary's,
holds `(false)` on both stdlibs, as the reason says. The probes are in the scratch directory of this run and are not
banked. Whether to kill the pair, or keep it banked with the reason made exact, is the user's:
[§ What the user is asked](#what-the-user-is-asked), 2.

## Loose ends tidied with this page

- [The phase README](README.md): its status names this close; the stage table's cells for S1f.6.7, S1f.6.9, S1f.6.10,
  S1f.6.12, S1f.6.13, S1f.6.16, S1f.6.17, S1f.6.20, S1f.6.30, S1f.6.35 and S1f.6.38 lead with the state they are in
  and point at the record, where they led with *stopped for the user* or *built but for*; the order's items 4 and 6
  to 15 say they are done, and item 19 is this close. S1f.6.39's row and the order's item 17 are T7's to restate.
- [`open_questions.md`](../open_questions.md)'s index had no rows for Q-M1f.254 to .260, .263 to .265 and .284 to
  .303: the 22 that are not T7's are added; .287 to .294's are T7's, with its record of the rulings.
- [`findings.md`](findings.md): the 19 proposals, each in its row, marked *proposed*.
- [M1f's README](../README.md): P1f.6's count of stages, and the order after this close. `AGENTS.md`'s pointer to
  the current work.

## What the user is asked

1. **The findings' dispositions**, [§ Proposed dispositions](#proposed-dispositions-the-users-to-confirm): confirm
   them, or name the rows to change. Two are fixes before the close, both small: **E-37**, `01_grammar.md`'s bold
   sentence restated to what the engine does with two guards; **E-45**, a `--check` for
   `utils/read_out_examples.py` that `read_out_examples.rs` runs. Once confirmed, each owned row goes into its owner's
   plan as F21's did (*handed over by M1f*).
2. **The two survivors that are not equivalent**: **(a)** bank the probe and its binary twin as tests under
   `tests/stdlib/math/decimal/` and `binary/`, which kills both and pins what a program that writes the module's
   own `named` gets; **(b)** keep both banked and make `tests/README.md`'s reason exact: *equivalent on every program
   that states a name through `let` only; a program that writes `named z` itself tells them apart*. **Recommended
   (b)**: (a) would pin a program outside the protocol [Q-M1f.278](../open_questions.md#q-m1f278--how-a-module-below-let-flags-a-named-constant-inside-a-numeral) defaulted.
3. **The defaults**, [§ The defaults](#the-defaults): any of the 81 to revise; the **ships** rows are the ones a
   revision would move for a user. And marking the five **moot** rows (.191, .192, .209, .213, .217) *superseded*
   in `open_questions.md`, proposed.
4. **The order of P1f.10 and P1f.20**, neither started, after this phase:
   P1f.10, the structure of the hypothesis set, which now also holds
   E-1, E-7, E-29, E-51 and E-54 and which `std.math.crypt` waits for; and
   [P1f.20](../p1f.20_docs_refactor/README.md), the documentation tree.
5. **The close itself**: the default is that P1f.6 closes once T7 is committed on a green gate and what item 1
   confirms as *fix before close* is in, with no `docs/history/` record, as P1f.5 and P1f.7 closed.

## The user's rulings (2026-11-15)

The user, 2026-11-15, on [§ What the user is asked](#what-the-user-is-asked), after the five were put with a
recommendation each: *"all as you recommended"*. So:

1. **The 19 dispositions are confirmed** as [§ Proposed dispositions](#proposed-dispositions-the-users-to-confirm)
   proposes them. Each owned row goes into its owner's plan (*handed over by M1f, 2026-11-15*), as F21's did; the
   seven *ruled as it stands* are closed with that ruling; **E-37 and E-45 are fixed before the close**.
2. **(a), the two survivors are killed**: the probe that writes `(named z)` beside `(n z 3)`, and its binary twin
   `(bin-named z)` beside `(b2 z 1)`, are banked as tests under `tests/stdlib/math/decimal/` and `binary/`, and the
   two lines leave `tests/mutation_survivors.txt`. (The prep run had recommended (b); the user took (a), on the ground
   that the program loads, the guard changes its answer, and a banked probe is the standard of proof.)
3. **The 81 defaults stand** as [§ The defaults](#the-defaults) tables them; **the five moot rows**, Q-M1f.191, .192,
   .209, .213 and .217, are marked *superseded* in [`open_questions.md`](../open_questions.md).
4. **P1f.10 runs before P1f.20**: P1f.20 rewrites the pages, so it describes the engine after P1f.10's changes.
5. **The close**: P1f.6 closes once S1f.6.39's T7 is committed on a green gate and items 1's two fixes and 2's two
   tests are in, with no `docs/history/` record.

## The close (2026-11-15)

**P1f.6 is closed**, on the user's rulings above, once S1f.6.39's T7 was committed (`bcdb91f8`) and the fixes and
tests items 1 and 2 asked for were in. No `docs/history/` record, as P1f.5 and P1f.7 closed; this directory stays.
**P1f.10 is next, then P1f.20** (ruling 4).

What the close did, each commit docs only unless it says otherwise:

1. **The dispositions.** E-37 fixed, a page (`a6ba0a06`): `01_grammar.md`'s bold sentence reads *a guard's place
   among the relation premises of its conjunction does not change what the premise means; the order of two guards on
   a variable no relation premise binds does*, with E-37's pair and a guard that waits, each form checked by `ein
   test` first; `defined_behaviour.md`'s sentence likewise. The owned rows handed over (`29c34c4f`), each a row or a
   section *handed over by M1f, 2026-11-15*: F23's D-15 to D-19 (E-34, E-35's first half, E-40, E-42, E-43), M12
   (E-35's second half, E-55, E-69), M1g's S1g.6.2 (E-36), M1g's P1g.7 (E-44), F24 (E-71), each re-taken where its
   witness runs. The seven ruled as they stand closed in their rows with the reason above. E-45 fixed, code
   (`cd23e205`): `utils/read_out_examples.py --check`, run by `read_out_examples.rs`' `the_page_is_what_the_script_writes`
   with the binary under test, the script's runs on a pool of `--jobs` threads. [`findings.md`](findings.md) has no
   row without a ruling, a fix or an owner.
2. **The survivors, (a)**, code (`ff061d7a`, `a51a40ee`): `tests/stdlib/math/decimal/06_a_programs_own_named_z.ein`
   and `binary/07_a_programs_own_named_z.ein`, `(named z)` beside `(n z 3)` and `(bin-named z)` beside `(b2 z 1)`,
   each R = {∅} with a sum answered, a corpus entry and its golden lines alone. `python3 utils/stdlib_mutants.py -k
   numeral-prefix-named --jobs 16`: both `drop-neq` mutants killed, the two `swap-assert` ones still banked; the two
   lines left `tests/mutation_survivors.txt`, 117 survivors now, and `tests/README.md` says what kills them.
3. **The defaults** stand; Q-M1f.191, .192, .209, .213 and .217 are marked *superseded* in
   [`open_questions.md`](../open_questions.md), in the index and in each row (`29c34c4f`).
4. **The order**, P1f.10 before P1f.20, in [M1f's README](../README.md) and [the phase README](README.md#the-order)
   (`29c34c4f`).
5. **Q-M1f.304 ruled (d)** after T7 (the user, *"(d)"*), code (`63f5e4b5`, `93732e85`): no test runs past the
   default `--region-budget`. `route_invariance`'s 2²⁵ test is gone, with the dev host's step that ran it; the
   default-budget test holds a program route 1 does not finish and the state route does by the cut, and names it
   ([the row](../open_questions.md#q-m1f304--route_invariance-past-the-default-budget-past-any-shards-five-minutes)).
   Every other test or tier step that runs a region budget above the default was listed for the main session and
   left as it is.

**The gate, in the runner's image** (`utils/ci_container.sh`, timed by `python3 utils/gate_times.py run`, AC
throughout, 16 workers): **green on `93732e85`, 14.1 min (843.5 s)**, the test step 672.2 s and clippy 151.9 s; the
slowest binaries `explore` 59.0 s and `lever_invariance` 55.2 s, the peak 10.4 GiB (`int_theory`). Two runs before it
failed, each on what the close had just added, each fixed and the run taken again: on `63f5e4b5`, `arith_search_free`,
whose arithmetic programs each hold an answer, and the two probes had made no request (fixed by `a51a40ee`); on
`a51a40ee`, `world_anchors`, since the rewritten `route_invariance` header named a zebra program by its path (fixed by
`93732e85`). **The nightly, in the runner's image** (`utils/ci_container.sh --nightly`, the same way):
**green on `93732e85`, 23.5 min (1 410.6 s)** with its jobs one after another: `deep-corpus` 314.2 s, the four
`deep-corpus-shards` 182.7 s at most, the twelve census shards 544.9 s together, every one `--check` green, 500 of
617 mutants killed and the 117 survivors the banked set; the slowest binary a shard of `answer_round_trip`, 109.7 s.
Both runs are in
[S1f.6.39's record](s1f.6.39_the_gate_made_fast.md#in-the-runners-image-at-the-close-2026-11-15) with their
per-step and per-binary tables; the logs are in `build/p1f6-close/`.

**Acceptance's last bullet now holds**: no row of [`findings.md`](findings.md) is without a ruling, a fix or an owner.
The register reads 322 fixed, 16 open and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`,
2026-11-15), none of the open rows this phase's; no wrong answer was met at the close. `ein test tests/`: 267 files,
464 expectations held (the gate profile's binary, 2026-11-15).
