# Reserved engine strings

Engine-**internal** vocabulary: the strings the engine keys on that are not a program's own names. Each lives behind
one named constant in [`ein_core::terms`](../../../ein.rs/crates/ein-core/src/terms.rs),
[`ein_core::text`](../../../ein.rs/crates/ein-core/src/text.rs) or the module that reads it, and a name is reserved
if and only if it appears here or among the names the language reserves for a program to write (`relation`, `not`,
`absent`, `eq`, `open`, …, which `ein_core::terms::RESERVED` lists with the two below).

**The words around a program's English are the engine's too**: the provenance words a `--dump-answer` state file
writes after a fact (`source`, `rule`, `hypothesis`), the trace's *Derives* and *Premises*, and the read-out's
labels. Every text a program writes, its `:why`s, `:source`s and `:goal-text`s, is the domain's, which the engine
fills and never edits ([Q-M1f.12](../../decisions/read-out/Q-M1f.12-natural-language-text-reaches-a-fixed-set.md)).

## The engine's names

A name that triggers behaviour hard-coded in the engine is written `__dunder__`, lexically distinct from a
program's relation and rule names; a bare name never triggers engine behaviour. Each name below is written only
with its fixed meaning, and is refused as a program's own:

| name | a program may | a program may not | refused by |
|---|---|---|---|
| `__closed__`, `__symmetric__` | **use** it wherever a name is used: as the relation of a fact, a premise, a conclusion, a goal or a claim's fact, and as a value, `(imply1 functional __closed__)` among them | **bind** it: declare a relation of that name, name a rule, an hrule or a macro after it, or import a module `:as` it | the loader, *relation '__closed__' shadows a reserved kernel name* (and *rule*, *hrule*, *macro*, *alias*): the two are in `ein_core::terms::RESERVED`, the names a declarator may not bind |
| `<forced-positive>`, `<monotonic-unconditional>`, `<lookahead-dies-immediately>` | — | write it anywhere: each names a conclusion no rule drew, which a program cannot | the parse: the grammar's `WRITEBACK` token, which no production takes (*unexpected `<forced-positive>`: the engine's writeback rule name (reserved)*) |

So a printed `:rule <lookahead-dies-immediately>` would always be the engine's, and a rule's firings never print
`:rule __symmetric__`, the native mirror's. The name in a string or a comment is not the name, and a longer run that
starts with one (`<forced-positive>x`) is the name it spells.
[`ein-ir/tests/engine_names.rs`](../../../ein.rs/crates/ein-ir/tests/engine_names.rs) holds each name in each
position. Both groups, with the query rule and the mirror's two variable names, are interned first, at fixed places,
by `ein_core::terms::ENGINE`, so the interner does not grow with them during a search
([`design/parallelism.md`](design/parallelism.md)).

## Bookkeeping carrier heads

**None.** A hypothesis is a fact with a provenance *kind*, `hypothesis` (`ein_core::prov::ProvKind`), never a fact
wrapped in a carrier head, so the canonical state form,
[`canon::state_key`](../../../ein.rs/crates/ein-infer/src/canon.rs), excludes nothing: it is the sorted fact set of
the state, free of provenance. A carrier head added later would have to be registered here **and** excluded by
`state_key`, so that it does not perturb a state's identity ([`design/search.md`](design/search.md)).

## Synthetic writeback rule names

The three names of the second row above name a rule that exists in no `(rule …)` form: a conclusion the search would
write back to root, justified by a property of the search (every alternative refuted) rather than by a derivation
in any one state. No code of the engine writes one; the names stay reserved, interned in `ein_core::terms::ENGINE`,
so that a writer added later cannot collide with a program's name.

Their shape is the **contract** a writer keeps: an empty premise list. Such a fact has no fact-level premises, so
every provenance walk **grounds out** on it: it ends the walk and contributes nothing to a frontier, and an unsat
core never names a search conclusion as a given.
[`Kb::record_justification`](../../../ein.rs/crates/ein-core/src/kb.rs) keeps the contract in both directions: it
refuses to record a premise-free derivation as an alternative (that would give a fact the empty environment,
*derivable from nothing*, and collapse every explanation through it), and it refuses to attach alternatives to a
fact whose first derivation is premise-free. [`explain.rs`](../../../ein.rs/crates/ein-infer/src/explain.rs) treats
such a fact as a terminal labelled with the empty environment.

## Names the parser and the loader make up

Six names stand for something a program writes no name for. Each starts with `#`, which is no name character (`#|`
opens a comment), so no program can spell one. `ein_core::text::INTERNAL_NAMES` lists them,
`ein_core::text::is_internal` is the test the dumper and `(import … :symbols …)` read, and
[`ein-ir/tests/generated_parser.rs`](../../../ein.rs/crates/ein-ir/tests/generated_parser.rs)'
`no_name_the_loader_makes_up_is_a_name` holds that none is a name of the lexer.

| name | names | printed as |
|---|---|---|
| `#empty` | the empty form `()`: an empty name list | `()` |
| `#nested` | a nested form whose head is not a name, `(r (_ a))` | `(_ …)` |
| `#params` | a rule's or a macro's parameter list | the list, with no head |
| `#list` | a fact's `:using` list, in the parse tree | the list, with no head |
| `#query` | the rule a query's `:goal` compiles to, so a binding has a rule to be reported under (`ein_core::terms::QUERY_RULE`) | a quoted name |
| `#written` | the loader's step a `(written T)` fact is derived by, from the fact it was read in (`ein_core::terms::WRITTEN_STEP`) | a quoted name, `:rule "#written"` in `ein saturate --dump` |

`:match ()` is the empty conjunction, which fires once, and `:assert ()` the empty conclusion, which asserts nothing:
each compiles with no synthetic head, so no run reaches a value `#empty`.

## Provenance kinds

A closed set the engine branches on, `ein_core::prov::ProvKind`: `source` (a file wrote it), `rule` (a rule derived
it), `hypothesis` (a commitment assumed it), `rejected`. The words are the ones a state file and a trace print
([`design/data-model.md`](design/data-model.md)).

## `__closed__` — engine effect

`(__closed__ R)` has one engine side, in [`hypgen.rs`](../../../ein.rs/crates/ein-infer/src/hypgen.rs) (`CLOSED`):
the generator reads the `(__closed__ R)` facts in the KB and contributes **no candidate for R**. The marks are the
ones a program writes or a rule derives (`std.closure`'s `infer-closure` asserts `(__closed__ ?R)`), and H holds the
candidates of every other relation. It is genuinely the engine's: a saturation rule can neither suppress the
generator nor observe that no rule asserts R. The hypothesis mechanisms are read together, and a closure that
contradicts an hrule or a list is refused ([`design/search.md`](design/search.md)).

## `__symmetric__` — engine effect

`(__symmetric__ R)` closes R's extension under argument swap **natively in the saturator**: each `(R a b)` produces
`(R b a)` directly (a self-loop `a = a` and a mirror already present are skipped), as a firing with rule
`__symmetric__` that threads the source edge as its premise and is narrated as a `mirror` event
([`events.md`](events.md)). Its single source is `SYMMETRIC` and the mirror machinery (`symmetric_rels`,
`enqueue_mirror_sources`, `next_mirror_firing`) in [`saturator.rs`](../../../ein.rs/crates/ein-infer/src/saturator.rs)
([`design/saturation.md`](design/saturation.md)).

**The mark is read in the saturation it comes to hold in.** Written as a fact, derived by a rule at root, or derived
in a node's saturation once a commitment brings a rule to it: the saturation in which `(__symmetric__ R)` comes to
hold offers every fact of R already stored to the mirror, as it does for a relation marked from its first step, and
every fact of R stored after the mark as it arrives. So the fixpoint is closed under the swap however the mark came
to hold, a second saturation of it writes nothing, and one symmetry stated as a fact or by a rule has one root and
one R. Each mirror's derivation records the mark as its activator, as a rule's firing records the fact that
activated its plan, so a dead node's core follows the mark where the node derived it and names the commitment that
brought it, and never follows a mark root holds (`ein-infer/tests/corpus_cores.rs`;
`ein-infer/tests/stdlib_semantics.rs`'s `the_native_mirror_reads_a_mark_a_rule_derives` and
`a_derived_mark_answers_what_its_control_answers`).

**A cost, not a capability.** It computes the same closure as the stdlib's `symmetric` rule (`std.algebra`), pinned
by `stdlib_semantics.rs`'s `the_native_mirror_computes_the_same_closure_as_the_stdlib_rule`, and skips the plan and
the match the rule pays per mirror. It is opt-in by marking the relation: with nothing marked the mirror queue is
empty and a saturation takes the path with no mirror. Symmetry itself is the program's, never the engine's
([Q6](../../decisions/answer/Q6-symmetry-is-the-programs-not-the-engines.md)).

## Hypothesis-scoping keys

Two `(solve …)` keywords the hypothesis generator reads to scope which relations the blind enumerator proposes
about: `HYPOTHESIS_RELATIONS` and `NO_HYPOTHESIS` in [`hypgen.rs`](../../../ein.rs/crates/ein-infer/src/hypgen.rs).

| key | effect |
|---|---|
| `:hypothesis-relations` | **an allow-list**: candidates only for the listed relations (unset: every eligible one) |
| `:no-hypothesis` | **a deny-list**: never a candidate on the listed relations; saturation rules on them still fire |

Neither touches the saturator: they scope generation only, where `(__closed__ R)` above also states that no rule
derives R. The mechanisms are read together on root's fixpoint (`ein_infer::mechanisms`): a relation in both lists,
or listed and closed, is refused before any search, and a list an hrule leaves filtering nothing is a note
([`design/search.md`](design/search.md)).
