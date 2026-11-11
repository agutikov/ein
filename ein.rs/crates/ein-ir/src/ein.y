%grmtools {yacckind: Grmtools}
// Ein's grammar: S1f.5.5's final grammar, step 2 (T1f.5.5.2), 2026-11-08.
//
// One production per position: what a fact's argument, a premise, a
// conclusion, a guard's argument, a claim and each keyword's value may hold,
// each stated where it is used. There is no shared `Value`. A shape a
// production does not list is a parse error at its first token, so it needs
// no line in the loader. What stays the loader's is what a context-free
// grammar cannot see: arity against a signature, which keywords a form
// requires and that each appears once, a variable no parameter or premise
// binds, a relation named by a parameter its activator binds to a fact, the
// macro table, imports, and *forbid*.
//
// Two rulings of 2026-11-08 came after step 2 and are in it (T1f.5.5.3): a
// rule takes no `:priority`, which S1f.5.15 deletes before the swap, so the
// keyword is refused here as any other a rule does not take, and a run of
// digits is no token of its own; and a macro takes a `:why`.
//
// Every action is one call into the builder `b`, which makes today's
// `ein_ir::Ast`, node for node and `Loc` for `Loc`, so the loader that reads
// it does not change. The builder is `grammar/build_ast.rs`; `build.rs`
// generates the parser from this file and `ein.l` (T1f.5.5.3).
%start Program
%parse-param p: &::std::cell::RefCell<b::B>
%token "UNDERSCORE" "VARUNDERSCORE" "WRITEBACK" "KEYWORD" "BAD" "BADTEXT" "BADESCAPE"
%expect-unused "UNDERSCORE" "VARUNDERSCORE" "WRITEBACK" "KEYWORD" "BAD" "BADTEXT" "BADESCAPE"
%epp "(" "("
%epp ")" ")"
%epp "NAME" "a name"
%epp "VAR" "a variable"
%epp "TEXT" "a string"
%epp "UNDERSCORE" "_ (reserved)"
%epp "VARUNDERSCORE" "?_ (reserved)"
%epp "WRITEBACK" "the engine's writeback rule name (reserved)"
%epp "KEYWORD" "a keyword this form does not take"
%epp "BAD" "a token that is not a name"
%epp "BADTEXT" "a string glued to what follows"
%epp "BADESCAPE" "a string with an unknown escape"
%%
Program -> Vec<b::Id>:
    { b::none() }
  | Program Form { b::push($1, $2) }
  ;

Form -> b::Id:
    Relation { $1 } | Rule { $1 } | Query { $1 } | Solve { $1 } | Config { $1 }
  | Trace { $1 } | Macro { $1 } | Import { $1 } | NotFact { $1 } | Fact { $1 }
  ;

// ── Names ────────────────────────────────────────────────────────────
// A name, wherever any name goes: a fact's argument, a pattern's, a list's.
// Not `open`, the verdict, which `Assert` alone takes (M1f S1f.5.23, the
// user's ruling on Q-M1f.66): no relation, value or name is `open`.
Name -> b::Id:
    "NAME" { b::name(p, $lexer, $1) }
  | "relation" { b::name(p, $lexer, $1) } | "step" { b::name(p, $lexer, $1) }
  | "eq" { b::name(p, $lexer, $1) } | "absent" { b::name(p, $lexer, $1) }
  | "false" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) }
  | "region" { b::name(p, $lexer, $1) } | "merged" { b::name(p, $lexer, $1) }
  | "owing" { b::name(p, $lexer, $1) }
  ;
// A name a declarator binds: a relation's, a rule's, a macro's, an alias.
// Not one of the kernel's reserved names (`ein_core::RESERVED`).
DeclName -> b::Id:
    "NAME" { b::name(p, $lexer, $1) }
  | "step" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) } | "region" { b::name(p, $lexer, $1) }
  | "merged" { b::name(p, $lexer, $1) } | "owing" { b::name(p, $lexer, $1) }
  ;
// A top-level fact's relation: not `relation`, which declares, not a
// claim's `region`, `merged` or `owing`, which belong under `(solve :expect)`,
// and not `open`, the verdict, which no fact is of (M1f S1f.5.23). Not
// `absent`, which is negation as failure, a premise's test and no relation a
// state holds a fact of (M1f S1f.7.7), and not `false`, which is shaped below:
// it takes no argument (M1f S1f.7.6). `eq` stays: a written equality is
// checked, never stored (M1f S1f.5.7).
FactHead -> b::Id:
    "NAME" { b::name(p, $lexer, $1) }
  | "step" { b::name(p, $lexer, $1) } | "eq" { b::name(p, $lexer, $1) }
  | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) }
  ;
// The relation of a value: of a fact inside a fact, a pattern inside a
// pattern, a conclusion's or a guard's argument. A value is a name, or a fact
// a state could hold, an atom of an ordinary relation or its negation (the
// user's ruling of 2026-11-11, M1f P1f.7's value rule): so never `eq`, which
// is computed (S1f.7.10), `absent`, a test (S1f.7.7), or `false`, which takes
// no argument and is shaped where it is written (S1f.7.6).
ValueHead -> b::Id:
    "NAME" { b::name(p, $lexer, $1) }
  | "relation" { b::name(p, $lexer, $1) } | "step" { b::name(p, $lexer, $1) }
  | "true" { b::name(p, $lexer, $1) } | "model" { b::name(p, $lexer, $1) }
  | "region" { b::name(p, $lexer, $1) } | "merged" { b::name(p, $lexer, $1) }
  | "owing" { b::name(p, $lexer, $1) }
  ;
// A premise's relation: not `eq` or `absent`, which are shaped below, not
// `false`, which takes no argument (M1f S1f.7.6), and not `open`, a verdict
// and never a premise.
PremiseHead -> b::Id: ValueHead { $1 } | Var { $1 } ;
// A conclusion's relation: not `eq`, `false` or `open`, which are shaped,
// and not `absent`, which no fact is of (M1f S1f.7.7).
ConclusionHead -> b::Id: ValueHead { $1 } | Var { $1 } ;
Var -> b::Id: "VAR" { b::var(p, $lexer, $1) } ;
Text -> b::Id: "TEXT" { b::text(p, $lexer, $1) } ;
Bool -> b::Id: "true" { b::name(p, $lexer, $1) } | "false" { b::name(p, $lexer, $1) } ;

Names -> Vec<b::Id>: { b::none() } | Names Name { b::push($1, $2) } ;
NamesPlus -> Vec<b::Id>: Name { b::one($1) } | NamesPlus Name { b::push($1, $2) } ;
Vars -> Vec<b::Id>: { b::none() } | Vars Var { b::push($1, $2) } ;
VarsPlus -> Vec<b::Id>: Var { b::one($1) } | VarsPlus Var { b::push($1, $2) } ;
// `(a b c)`: a list of names, `()` the empty one.
NameList -> b::Id:
    "(" ")" { b::empty(p) }
  | "(" Name Names ")" { b::list(p, $2, $3) }
  ;
NameListPlus -> b::Id: "(" Name Names ")" { b::list(p, $2, $3) } ;

// ── Facts ────────────────────────────────────────────────────────────
// No variable, no `_`, no string, no `()` and no keyword: a fact holds names
// and facts.
Fact -> b::Id:
    "(" FactHead FactArgs FactClauses ")" { b::top(p, $2, $3, $4) }
  | "(" "false" FactClauses ")" { b::top(p, b::name(p, $lexer, $2), b::none(), $3) }
  ;
// The stored negation of a fact, or `(not (eq a b))`, which is checked as a
// written `(eq a b)` is (S1f.5.7).
NotFact -> b::Id:
    "(" "not" NestedFact FactClauses ")" { b::named(p, "not", b::cat(b::one($3), $4)) }
  | "(" "not" EqFact FactClauses ")" { b::named(p, "not", b::cat(b::one($3), $4)) }
  ;
// A fact as a value: its relation is an ordinary one, or it is `(false)`.
NestedFact -> b::Id:
    "(" ValueHead FactArgs ")" { b::list(p, $2, $3) }
  | "(" "false" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  ;
// `(eq a b)` where a fact of equality may be written and is checked: under a
// top-level `(not …)` and in a claim. Never a value.
EqFact -> b::Id: "(" "eq" FactArgs ")" { b::list(p, b::name(p, $lexer, $2), $3) } ;
FactArg -> b::Id:
    Name { $1 }
  | NestedFact { $1 }
  | "(" "not" NestedFact ")" { b::named(p, "not", b::one($3)) }
  ;
FactArgs -> Vec<b::Id>: { b::none() } | FactArgs FactArg { b::push($1, $2) } ;
// A fact says where it came from: given (`:source`) or derived (`:rule`,
// `:using`). `:using` lists the premises, each a fact.
FactClause -> b::Id:
    ":source" Text { b::pair(p, $lexer, $1, $2) }
  | ":rule" Name { b::pair(p, $lexer, $1, $2) }
  | ":using" "(" NestedFacts ")" { b::pair(p, $lexer, $1, b::premises(p, $3)) }
  ;
FactClauses -> Vec<b::Id>: { b::none() } | FactClauses FactClause { b::push($1, $2) } ;
NestedFacts -> Vec<b::Id>: NestedFact { b::one($1) } | NestedFacts NestedFact { b::push($1, $2) } ;

// ── Relations ────────────────────────────────────────────────────────
Relation -> b::Id:
    "(" "relation" DeclName Names RelationClauses ")" { b::named(p, "relation", b::cat(b::cat(b::one($3), $4), $5)) }
  ;
RelationClauses -> Vec<b::Id>:
    { b::none() }
  | RelationClauses ":why" Text { b::push($1, b::pair(p, $lexer, $2, $3)) }
  ;

// ── Rules ────────────────────────────────────────────────────────────
Rule -> b::Id:
    "(" "rule" DeclName Params RuleClauses ")" { b::named(p, "rule", b::cat(vec![$3, $4], $5)) }
  | "(" "hrule" DeclName Params RuleClauses ")" { b::named(p, "hrule", b::cat(vec![$3, $4], $5)) }
  ;
Params -> b::Id: "(" Vars ")" { b::params(p, $2) } ;
RuleClause -> b::Id:
    ":match" Match { b::pair(p, $lexer, $1, $2) }
  | ":assert" Assert { b::pair(p, $lexer, $1, $2) }
  | ":why" Text { b::pair(p, $lexer, $1, $2) }
  ;
RuleClauses -> Vec<b::Id>: { b::none() } | RuleClauses RuleClause { b::push($1, $2) } ;

// A :match is a premise, or `()`, the empty conjunction (S1f.5.9). A
// conjunction or a disjunction has at least one member: `(and)` is refused,
// and `()` is the one spelling of the empty conjunction (F12).
Match -> b::Id: "(" ")" { b::empty(p) } | Premise { $1 } ;
Premise -> b::Id:
    "(" PremiseHead Args ")" { b::list(p, $2, $3) }
  | "(" "false" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  | Connective { $1 }
  ;
Premises -> Vec<b::Id>: Premise { b::one($1) } | Premises Premise { b::push($1, $2) } ;
// What a premise may be besides a pattern: a conjunction, a disjunction, a
// stored negation, negation as failure over one premise, or a guard.
Connective -> b::Id:
    "(" "and" Premises ")" { b::named(p, "and", $3) }
  | "(" "or" Premises ")" { b::named(p, "or", $3) }
  | "(" "not" Negated ")" { b::named(p, "not", b::one($3)) }
  | "(" "absent" Premise ")" { b::list(p, b::name(p, $lexer, $2), b::one($3)) }
  | Guard { $1 }
  ;
// What a premise's or a conclusion's `(not …)` negates: a pattern, or an
// equality, `(not (eq a b))`, which is computed as `(neq a b)` is (S1f.5.7).
Negated -> b::Id:
    Pattern { $1 }
  | "(" "eq" Term Term ")" { b::list(p, b::name(p, $lexer, $2), vec![$3, $4]) }
  ;
// `eq` and `neq` compare two values (S1f.5.7): a name, a variable, or a
// fact, which is a value (W24/form-in-guard, ruled answered).
Guard -> b::Id:
    "(" "eq" Term Term ")" { b::list(p, b::name(p, $lexer, $2), vec![$3, $4]) }
  | "(" "neq" Term Term ")" { b::named(p, "neq", vec![$3, $4]) }
  ;
// A pattern's argument: a name, a variable, a pattern inside it, or, since
// a macro such as `forall` is called as a pattern and takes premises, a
// connective. Its relation is a name or a variable, but never `eq` or
// `absent`, which the connectives shape, never `false` with an argument (M1f
// S1f.7.6), and never `open`, the verdict (M1f S1f.5.23; the loader refused
// one here until then). A connective that is still a pattern's argument once
// the macros are expanded is the loader's to refuse (M1f S1f.7.3): the
// grammar cannot tell a macro's call from a pattern.
Args -> Vec<b::Id>: { b::none() } | Args Arg { b::push($1, $2) } ;
Arg -> b::Id:
    Name { $1 } | Var { $1 } | "(" ArgHead Args ")" { b::list(p, $2, $3) }
  | "(" "false" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  | Connective { $1 }
  ;
ArgHead -> b::Id: ValueHead { $1 } | Var { $1 } ;

// A term: a fact with variables in it, as a conclusion, a stored negation or
// a guard holds one, and a negated one, as a fact may hold. No connective,
// and its relation is a value's (`ValueHead`), or a variable.
Pattern -> b::Id:
    "(" ValueHead Terms ")" { b::list(p, $2, $3) }
  | "(" Var Terms ")" { b::list(p, $2, $3) }
  | "(" "false" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  ;
AnyHead -> b::Id: Name { $1 } | Var { $1 } ;
Term -> b::Id:
    Name { $1 } | Var { $1 } | Pattern { $1 }
  | "(" "not" Pattern ")" { b::named(p, "not", b::one($3)) }
  ;
Terms -> Vec<b::Id>: { b::none() } | Terms Term { b::push($1, $2) } ;

// An :assert is a conclusion, `()` (nothing), or `(open)` / `(open ?R)`,
// which is a whole conclusion and never part of one (M1d S1d.2.3). These two
// are the only places `open` is written (M1f S1f.5.23, Q-M1f.66): `?R` is a
// variable or a relation's name, and no name is `open`.
Assert -> b::Id:
    "(" ")" { b::empty(p) }
  | Conclusion { $1 }
  | "(" "open" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  | "(" "open" AnyHead ")" { b::list(p, b::name(p, $lexer, $2), b::one($3)) }
  ;
Conclusion -> b::Id:
    "(" ConclusionHead Terms ")" { b::list(p, $2, $3) }
  | "(" "and" Conclusions ")" { b::named(p, "and", $3) }
  | "(" "not" Negated ")" { b::named(p, "not", b::one($3)) }
  | "(" "not" Var ")" { b::named(p, "not", b::one($3)) }
  | "(" "false" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  | Guard { $1 }
  ;
Conclusions -> Vec<b::Id>: Conclusion { b::one($1) } | Conclusions Conclusion { b::push($1, $2) } ;

// ── Queries and the lattice ──────────────────────────────────────────
Query -> b::Id: "(" "query" QueryClausesPlus ")" { b::named(p, "query", $3) } ;
QueryClause -> b::Id:
    ":goal" Premise { b::pair(p, $lexer, $1, $2) }
  | ":goal-text" Text { b::pair(p, $lexer, $1, $2) }
  | ":over" Name { b::pair(p, $lexer, $1, $2) }
  | ":expect" Claim { b::pair(p, $lexer, $1, $2) }
  ;
QueryClausesPlus -> Vec<b::Id>: QueryClause { b::one($1) } | QueryClausesPlus QueryClause { b::push($1, $2) } ;
// A query's claim: one model, a set of them, or none (M1c S1c.1.2).
Claim -> b::Id:
    Model { $1 }
  | "(" "or" Models ")" { b::named(p, "or", $3) }
  | "(" "false" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  ;
Model -> b::Id: "(" "model" ClaimFacts ")" { b::list(p, b::name(p, $lexer, $2), $3) } ;
Models -> Vec<b::Id>: Model { b::one($1) } | Models Model { b::push($1, $2) } ;
// A claim lists facts, a stored negation, or an `eq` / `neq` fact (S1f.5.7).
ClaimFact -> b::Id:
    NestedFact { $1 }
  | EqFact { $1 }
  | "(" "not" NestedFact ")" { b::named(p, "not", b::one($3)) }
  | "(" "not" EqFact ")" { b::named(p, "not", b::one($3)) }
  | "(" "absent" ClaimAtom ")" { b::list(p, b::name(p, $lexer, $2), b::one($3)) }
  | "(" "neq" FactArg FactArg ")" { b::named(p, "neq", vec![$3, $4]) }
  ;
// What a class says a state does not hold: the atom of a goal's `(absent …)`.
// The one place `absent` heads a form that is no premise, and no fact either:
// a claim lists a query's projection classes (M1e2 S1e2.1a.5).
ClaimAtom -> b::Id:
    NestedFact { $1 }
  | EqFact { $1 }
  | "(" "not" NestedFact ")" { b::named(p, "not", b::one($3)) }
  | "(" "not" EqFact ")" { b::named(p, "not", b::one($3)) }
  ;
ClaimFacts -> Vec<b::Id>: { b::none() } | ClaimFacts ClaimFact { b::push($1, $2) } ;

Solve -> b::Id: "(" "solve" SolveClauses ")" { b::named(p, "solve", $3) } ;
SolveClause -> b::Id:
    ":hrules" Hrules { b::pair(p, $lexer, $1, $2) }
  | ":hypothesis-relations" NameList { b::pair(p, $lexer, $1, $2) }
  | ":no-hypothesis" NameList { b::pair(p, $lexer, $1, $2) }
  | ":expect" RegionClaim { b::pair(p, $lexer, $1, $2) }
  ;
SolveClauses -> Vec<b::Id>: { b::none() } | SolveClauses SolveClause { b::push($1, $2) } ;
// `(h1 h2)`, or an hrule with its activator's arguments: `(guess (color-loc
// Color House) …)`.
Hrules -> b::Id:
    "(" ")" { b::empty(p) }
  | "(" Name HruleItems ")" { b::list(p, $2, $3) }
  ;
HruleItem -> b::Id: Name { $1 } | NameListPlus { $1 } ;
HruleItems -> Vec<b::Id>: { b::none() } | HruleItems HruleItem { b::push($1, $2) } ;
// The expected region (M1e2 S1e2.1a.4).
RegionClaim -> b::Id:
    "(" "region" RegionItems ")" { b::list(p, b::name(p, $lexer, $2), $3) }
  | "(" "false" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  ;
RegionItem -> b::Id:
    Model { $1 }
  | "(" "merged" Model MergedFrom ")" { b::list(p, b::name(p, $lexer, $2), vec![$3, $4]) }
  | "(" "owing" Models ")" { b::list(p, b::name(p, $lexer, $2), $3) }
  ;
RegionItems -> Vec<b::Id>: { b::none() } | RegionItems RegionItem { b::push($1, $2) } ;
MergedFrom -> b::Id: Model { $1 } | "(" "or" Models ")" { b::named(p, "or", $3) } ;

Config -> b::Id: "(" "config" ConfigClauses ")" { b::named(p, "config", $3) } ;
ConfigClause -> b::Id:
    ":lattice-sanity-check" Bool { b::pair(p, $lexer, $1, $2) }
  | ":record-alternative-justifications" Bool { b::pair(p, $lexer, $1, $2) }
  ;
ConfigClauses -> Vec<b::Id>: { b::none() } | ConfigClauses ConfigClause { b::push($1, $2) } ;

// ── Macros and imports ───────────────────────────────────────────────
// A macro's body is a template: any position may be one of its parameters.
// A macro may carry an NL template, as a relation and a rule do (the user,
// 2026-11-08): `:why` once, after the body.
Macro -> b::Id:
    "(" "macro" DeclName MacroParams Template ")" { b::named(p, "macro", vec![$3, $4, $5]) }
  | "(" "macro" DeclName MacroParams Template ":why" Text ")" { b::named(p, "macro", vec![$3, $4, $5, b::pair(p, $lexer, $6, $7)]) }
  ;
MacroParams -> b::Id: "(" VarsPlus ")" { b::params(p, $2) } ;
Template -> b::Id:
    Var { $1 }
  | "(" "and" Templates ")" { b::named(p, "and", $3) }
  | "(" "or" Templates ")" { b::named(p, "or", $3) }
  | "(" "not" Template ")" { b::named(p, "not", b::one($3)) }
  | "(" "neq" TemplateItem TemplateItem ")" { b::named(p, "neq", vec![$3, $4]) }
  | "(" AnyHead TemplateItems ")" { b::list(p, $2, $3) }
  ;
Templates -> Vec<b::Id>: Template { b::one($1) } | Templates Template { b::push($1, $2) } ;
TemplateItem -> b::Id: Name { $1 } | Template { $1 } ;
TemplateItems -> Vec<b::Id>: { b::none() } | TemplateItems TemplateItem { b::push($1, $2) } ;

Import -> b::Id: "(" "import" Name ImportClauses ")" { b::named(p, "import", b::cat(b::one($3), $4)) } ;
ImportClause -> b::Id:
    ":symbols" NameListPlus { b::pair(p, $lexer, $1, $2) }
  | ":as" DeclName { b::pair(p, $lexer, $1, $2) }
  ;
ImportClauses -> Vec<b::Id>: { b::none() } | ImportClauses ImportClause { b::push($1, $2) } ;

// ── The trace: engine output, read back by `ein render` ──────────────
Trace -> b::Id: "(" "trace" Steps ")" { b::named(p, "trace", $3) } ;
Steps -> Vec<b::Id>: { b::none() } | Steps Step { b::push($1, $2) } ;
Step -> b::Id: "(" "step" Name StepClauses ")" { b::named(p, "step", b::cat(b::one($3), $4)) } ;
StepClause -> b::Id:
    ":rule" Name { b::pair(p, $lexer, $1, $2) }
  | ":using" StepUsing { b::pair(p, $lexer, $1, $2) }
  | ":derives" Derived { b::pair(p, $lexer, $1, $2) }
  | ":source" Text { b::pair(p, $lexer, $1, $2) }
  | ":assumes" Derived { b::pair(p, $lexer, $1, $2) }
  ;
StepClauses -> Vec<b::Id>: { b::none() } | StepClauses StepClause { b::push($1, $2) } ;
// The steps and givens a step used: `(c10)`, `(s1 s2)`, `(and s1 c15)`.
StepUsing -> b::Id: NameListPlus { $1 } | "(" "and" NamesPlus ")" { b::named(p, "and", $3) } ;
Derived -> b::Id: NestedFact { $1 } | "(" "not" NestedFact ")" { b::named(p, "not", b::one($3)) } ;
%%
use super::b;
