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
// Every action is one call into the builder `b`, which makes today's
// `ein_ir::Ast`, node for node and `Loc` for `Loc`, so the loader that reads
// it does not change. The builder is the probe's `grammar/build_ast.rs`.
%start Program
%parse-param p: &::std::cell::RefCell<b::B>
%token "UNDERSCORE" "VARUNDERSCORE" "KEYWORD" "BAD" "BADTEXT" "BADESCAPE"
%expect-unused "UNDERSCORE" "VARUNDERSCORE" "KEYWORD" "BAD" "BADTEXT" "BADESCAPE"
%epp "(" "("
%epp ")" ")"
%epp "NAME" "a name"
%epp "VAR" "a variable"
%epp "TEXT" "a string"
%epp "DIGITS" "digits"
%epp "UNDERSCORE" "_ (reserved)"
%epp "VARUNDERSCORE" "?_ (reserved)"
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
Name -> b::Id:
    "NAME" { b::name(p, $lexer, $1) } | "DIGITS" { b::name(p, $lexer, $1) }
  | "relation" { b::name(p, $lexer, $1) } | "step" { b::name(p, $lexer, $1) }
  | "eq" { b::name(p, $lexer, $1) } | "absent" { b::name(p, $lexer, $1) }
  | "false" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "open" { b::name(p, $lexer, $1) } | "model" { b::name(p, $lexer, $1) }
  | "region" { b::name(p, $lexer, $1) } | "merged" { b::name(p, $lexer, $1) }
  | "owing" { b::name(p, $lexer, $1) }
  ;
// A name a declarator binds: a relation's, a rule's, a macro's, an alias.
// Not one of the kernel's reserved names (`ein_core::RESERVED`).
DeclName -> b::Id:
    "NAME" { b::name(p, $lexer, $1) } | "DIGITS" { b::name(p, $lexer, $1) }
  | "step" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) } | "region" { b::name(p, $lexer, $1) }
  | "merged" { b::name(p, $lexer, $1) } | "owing" { b::name(p, $lexer, $1) }
  ;
// A top-level fact's relation: not `relation`, which declares, and not a
// claim's `region`, `merged` or `owing`, which belong under `(solve :expect)`.
FactHead -> b::Id:
    "NAME" { b::name(p, $lexer, $1) } | "DIGITS" { b::name(p, $lexer, $1) }
  | "step" { b::name(p, $lexer, $1) } | "eq" { b::name(p, $lexer, $1) }
  | "absent" { b::name(p, $lexer, $1) } | "false" { b::name(p, $lexer, $1) }
  | "true" { b::name(p, $lexer, $1) } | "open" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) }
  ;
// A premise's relation: not `eq` or `absent`, which are shaped below, and
// not `open`, a verdict and never a premise.
PremiseHead -> b::Id:
    "NAME" { b::name(p, $lexer, $1) } | "DIGITS" { b::name(p, $lexer, $1) }
  | "relation" { b::name(p, $lexer, $1) } | "step" { b::name(p, $lexer, $1) }
  | "false" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) } | "region" { b::name(p, $lexer, $1) }
  | "merged" { b::name(p, $lexer, $1) } | "owing" { b::name(p, $lexer, $1) }
  | Var { $1 }
  ;
// A conclusion's relation: not `eq`, `false` or `open`, which are shaped.
ConclusionHead -> b::Id:
    "NAME" { b::name(p, $lexer, $1) } | "DIGITS" { b::name(p, $lexer, $1) }
  | "relation" { b::name(p, $lexer, $1) } | "step" { b::name(p, $lexer, $1) }
  | "absent" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) } | "region" { b::name(p, $lexer, $1) }
  | "merged" { b::name(p, $lexer, $1) } | "owing" { b::name(p, $lexer, $1) }
  | Var { $1 }
  ;
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
  ;
NotFact -> b::Id:
    "(" "not" NestedFact FactClauses ")" { b::named(p, "not", b::cat(b::one($3), $4)) }
  ;
NestedFact -> b::Id:
    "(" Name FactArgs ")" { b::list(p, $2, $3) }
  ;
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
  | ":priority" "DIGITS" { b::pair(p, $lexer, $1, b::count(p, $lexer, $2)) }
  ;
RuleClauses -> Vec<b::Id>: { b::none() } | RuleClauses RuleClause { b::push($1, $2) } ;

// A :match is a premise, or `()`, the empty conjunction (S1f.5.9). A
// conjunction or a disjunction has at least one member: `(and)` is refused,
// and `()` is the one spelling of the empty conjunction (F12).
Match -> b::Id: "(" ")" { b::empty(p) } | Premise { $1 } ;
Premise -> b::Id:
    "(" PremiseHead Args ")" { b::list(p, $2, $3) }
  | Connective { $1 }
  ;
Premises -> Vec<b::Id>: Premise { b::one($1) } | Premises Premise { b::push($1, $2) } ;
// What a premise may be besides a pattern: a conjunction, a disjunction, a
// stored negation, negation as failure over one premise, or a guard.
Connective -> b::Id:
    "(" "and" Premises ")" { b::named(p, "and", $3) }
  | "(" "or" Premises ")" { b::named(p, "or", $3) }
  | "(" "not" Pattern ")" { b::named(p, "not", b::one($3)) }
  | "(" "absent" Premise ")" { b::list(p, b::name(p, $lexer, $2), b::one($3)) }
  | Guard { $1 }
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
// `absent`, which the connectives shape.
Args -> Vec<b::Id>: { b::none() } | Args Arg { b::push($1, $2) } ;
Arg -> b::Id: Name { $1 } | Var { $1 } | "(" ArgHead Args ")" { b::list(p, $2, $3) } | Connective { $1 } ;
ArgHead -> b::Id:
    "NAME" { b::name(p, $lexer, $1) } | "DIGITS" { b::name(p, $lexer, $1) }
  | "relation" { b::name(p, $lexer, $1) } | "step" { b::name(p, $lexer, $1) }
  | "false" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "open" { b::name(p, $lexer, $1) } | "model" { b::name(p, $lexer, $1) }
  | "region" { b::name(p, $lexer, $1) } | "merged" { b::name(p, $lexer, $1) }
  | "owing" { b::name(p, $lexer, $1) } | Var { $1 }
  ;

// A term: a fact with variables in it, as a conclusion, a stored negation or
// a guard holds one, and a negated one, as a fact may hold. No connective.
Pattern -> b::Id: "(" AnyHead Terms ")" { b::list(p, $2, $3) } ;
AnyHead -> b::Id: Name { $1 } | Var { $1 } ;
Term -> b::Id:
    Name { $1 } | Var { $1 } | Pattern { $1 }
  | "(" "not" Pattern ")" { b::named(p, "not", b::one($3)) }
  ;
Terms -> Vec<b::Id>: { b::none() } | Terms Term { b::push($1, $2) } ;

// An :assert is a conclusion, `()` (nothing), or `(open)` / `(open ?R)`,
// which is a whole conclusion and never part of one (M1d S1d.2.3).
Assert -> b::Id:
    "(" ")" { b::empty(p) }
  | Conclusion { $1 }
  | "(" "open" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  | "(" "open" AnyHead ")" { b::list(p, b::name(p, $lexer, $2), b::one($3)) }
  ;
Conclusion -> b::Id:
    "(" ConclusionHead Terms ")" { b::list(p, $2, $3) }
  | "(" "and" Conclusions ")" { b::named(p, "and", $3) }
  | "(" "not" Pattern ")" { b::named(p, "not", b::one($3)) }
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
  | "(" "not" NestedFact ")" { b::named(p, "not", b::one($3)) }
  | "(" "neq" FactArg FactArg ")" { b::named(p, "neq", vec![$3, $4]) }
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
Macro -> b::Id: "(" "macro" DeclName MacroParams Template ")" { b::named(p, "macro", vec![$3, $4, $5]) } ;
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
