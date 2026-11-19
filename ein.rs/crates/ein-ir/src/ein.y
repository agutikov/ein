%grmtools {yacckind: Grmtools}
// Ein's grammar, over the tokens `ein.l` makes.
//
// One production per position: what a fact's argument, a premise, a
// conclusion, a guard's argument, a claim and each keyword's value may hold,
// each stated where it is used. There is no shared `Value`. A shape a
// production does not list is a parse error at its first token, so it needs
// no line in the loader. What stays the loader's is what a context-free
// grammar cannot see: arity against a signature, which keywords a form
// requires and that each appears once, a variable no parameter or premise
// binds, a relation named by a parameter its activator binds to a fact, the
// macro table and imports.
//
// Every action is one call into the builder `b`, which makes the
// `ein_ir::Ast` the loader reads, node for node and `Loc` for `Loc`. The
// builder is `grammar/build_ast.rs`; `build.rs` generates the parser from
// this file and `ein.l`.
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
// A file: a program's forms, or a saved file's (Q-M1f.401), whose wrappers
// stand at the top alone.
Program -> Vec<b::Id>:
    { b::none() }
  | Program TopForm { b::push($1, $2) }
  ;
TopForm -> b::Id: Form { $1 } | Saved { $1 } ;
Forms -> Vec<b::Id>: { b::none() } | Forms Form { b::push($1, $2) } ;

Form -> b::Id:
    Relation { $1 } | Rule { $1 } | Query { $1 } | Solve { $1 } | Config { $1 }
  | Trace { $1 } | Macro { $1 } | Import { $1 } | NotFact { $1 } | Fact { $1 }
  ;

// ── Names ────────────────────────────────────────────────────────────
// A name, wherever any name goes: a fact's argument, a pattern's, a list's.
// Not `open`, the verdict, which `Assert` alone takes (Q-M1f.66): no
// relation, value or name is `open`.
Name -> b::Id:
    "NAME" { b::name(p, $lexer, $1) }
  | "relation" { b::name(p, $lexer, $1) } | "step" { b::name(p, $lexer, $1) }
  | "eq" { b::name(p, $lexer, $1) } | "absent" { b::name(p, $lexer, $1) }
  | "false" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) }
  | "region" { b::name(p, $lexer, $1) } | "merged" { b::name(p, $lexer, $1) }
  | "owing" { b::name(p, $lexer, $1) }
  | "program" { b::name(p, $lexer, $1) } | "module" { b::name(p, $lexer, $1) }
  | "commit" { b::name(p, $lexer, $1) }
  ;
// A name a declarator binds: a relation's, a rule's, a macro's, an alias.
// Not one of the kernel's reserved names (`ein_core::RESERVED`).
DeclName -> b::Id:
    "NAME" { b::name(p, $lexer, $1) }
  | "step" { b::name(p, $lexer, $1) } | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) } | "region" { b::name(p, $lexer, $1) }
  | "merged" { b::name(p, $lexer, $1) } | "owing" { b::name(p, $lexer, $1) }
  | "program" { b::name(p, $lexer, $1) } | "module" { b::name(p, $lexer, $1) }
  | "commit" { b::name(p, $lexer, $1) }
  ;
// A top-level fact's relation: not `relation`, which declares, not a
// claim's `region`, `merged` or `owing`, which belong under `(solve :expect)`,
// not a saved file's `program`, `module` or `commit`, which wrap what it
// carries, and not `open`, the verdict, which no fact is of. Not `absent`, which is
// negation as failure, a premise's test and no relation a state holds a fact
// of, and not `false`, which is shaped below: it takes no argument. `eq`
// stays: a written equality is checked, never stored.
FactHead -> b::Id:
    "NAME" { b::name(p, $lexer, $1) }
  | "step" { b::name(p, $lexer, $1) } | "eq" { b::name(p, $lexer, $1) }
  | "true" { b::name(p, $lexer, $1) }
  | "model" { b::name(p, $lexer, $1) }
  ;
// The relation of a value: of a fact inside a fact, a pattern inside a
// pattern, a conclusion's or a guard's argument. An object is an atom, or a fact
// a state could hold, a fact of an ordinary relation or the stored negation
// of a fact a state could hold (Q-M1f.113, Q-M1f.391): so never `eq`, which
// is computed, `absent`, a test, or `false`, which takes no argument and is
// shaped where it is written.
ValueHead -> b::Id:
    "NAME" { b::name(p, $lexer, $1) }
  | "relation" { b::name(p, $lexer, $1) } | "step" { b::name(p, $lexer, $1) }
  | "true" { b::name(p, $lexer, $1) } | "model" { b::name(p, $lexer, $1) }
  | "region" { b::name(p, $lexer, $1) } | "merged" { b::name(p, $lexer, $1) }
  | "owing" { b::name(p, $lexer, $1) }
  | "program" { b::name(p, $lexer, $1) } | "module" { b::name(p, $lexer, $1) }
  | "commit" { b::name(p, $lexer, $1) }
  ;
// A premise's relation: not `eq` or `absent`, which are shaped below, not
// `false`, which takes no argument, and not `open`, a verdict
// and never a premise.
PremiseHead -> b::Id: ValueHead { $1 } | Var { $1 } ;
// A conclusion's relation: not `eq`, `false` or `open`, which are shaped,
// and not `absent`, which no fact is of.
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
// The stored negation of a fact, a negation among them, or `(not (eq a b))`,
// which is checked as a written `(eq a b)` is.
NotFact -> b::Id:
    "(" "not" NestedFact FactClauses ")" { b::named(p, "not", b::cat(b::one($3), $4)) }
  | "(" "not" NegFact FactClauses ")" { b::named(p, "not", b::cat(b::one($3), $4)) }
  | "(" "not" EqFact FactClauses ")" { b::named(p, "not", b::cat(b::one($3), $4)) }
  ;
// A stored negation as a value: of a fact, or of a stored negation, at any
// depth (Q-M1f.391). Never of a name, and never of an equality.
NegFact -> b::Id:
    "(" "not" NestedFact ")" { b::named(p, "not", b::one($3)) }
  | "(" "not" NegFact ")" { b::named(p, "not", b::one($3)) }
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
  | NegFact { $1 }
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
  // What a rule tells the engine, where another states an `:assert`
  // (Q-M1f.399): one word, a name and no fact. Which words are the engine's
  // is the loader's to hold, so that a word it does not know is refused with
  // the list of those it does.
  | ":ein" Name { b::pair(p, $lexer, $1, $2) }
  | ":why" Text { b::pair(p, $lexer, $1, $2) }
  ;
RuleClauses -> Vec<b::Id>: { b::none() } | RuleClauses RuleClause { b::push($1, $2) } ;

// A :match is a premise, or `()`, the empty conjunction (Q-M1e2.36). A
// conjunction or a disjunction has at least one member: `(and)` is refused,
// and `()` is the one spelling of the empty conjunction.
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
// What a premise's or a conclusion's `(not …)` negates: a pattern, the
// negation of one, or an equality, `(not (eq a b))`, which is computed as
// `(neq a b)` is.
Negated -> b::Id:
    Pattern { $1 }
  | NegPattern { $1 }
  | "(" "eq" Term Term ")" { b::list(p, b::name(p, $lexer, $2), vec![$3, $4]) }
  ;
// `eq` and `neq` compare two values: a name, a variable, or a fact, which is
// a value (Q-M1e2.26, form-in-guard).
Guard -> b::Id:
    "(" "eq" Term Term ")" { b::list(p, b::name(p, $lexer, $2), vec![$3, $4]) }
  | "(" "neq" Term Term ")" { b::named(p, "neq", vec![$3, $4]) }
  ;
// A pattern's argument: a name, a variable, a pattern inside it, or, since
// a macro such as `forall` is called as a pattern and takes premises, a
// connective. Its relation is a name or a variable, but never `eq` or
// `absent`, which the connectives shape, never `false` with an argument, and
// never `open`, the verdict. A connective that is still a pattern's argument
// once the macros are expanded is the loader's to refuse: the grammar cannot
// tell a macro's call from a pattern.
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
Term -> b::Id: Name { $1 } | Var { $1 } | Pattern { $1 } | NegPattern { $1 } ;
// The stored negation of a pattern, or of the negation of one, at any depth
// (Q-M1f.391). Never of a variable: `(not ?x)` is a conclusion's own
// literal, and stands nowhere else.
NegPattern -> b::Id:
    "(" "not" Pattern ")" { b::named(p, "not", b::one($3)) }
  | "(" "not" NegPattern ")" { b::named(p, "not", b::one($3)) }
  ;
Terms -> Vec<b::Id>: { b::none() } | Terms Term { b::push($1, $2) } ;

// An :assert is a conclusion, `()` (nothing), or `(open)` / `(open ?R)`,
// which is a whole conclusion and never part of one. These two are the only
// places `open` is written (Q-M1f.66): `?R` is a variable or a relation's
// name, and no name is `open`.
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
// A query's claim: one model, a set of them, or none.
Claim -> b::Id:
    Model { $1 }
  | "(" "or" Models ")" { b::named(p, "or", $3) }
  | "(" "false" ")" { b::list(p, b::name(p, $lexer, $2), b::none()) }
  ;
Model -> b::Id: "(" "model" ClaimFacts ")" { b::list(p, b::name(p, $lexer, $2), $3) } ;
Models -> Vec<b::Id>: Model { b::one($1) } | Models Model { b::push($1, $2) } ;
// A claim lists facts, a stored negation, or an `eq` / `neq` fact.
ClaimFact -> b::Id:
    NestedFact { $1 }
  | EqFact { $1 }
  | NegFact { $1 }
  | "(" "not" EqFact ")" { b::named(p, "not", b::one($3)) }
  | "(" "absent" ClaimAtom ")" { b::list(p, b::name(p, $lexer, $2), b::one($3)) }
  | "(" "neq" FactArg FactArg ")" { b::named(p, "neq", vec![$3, $4]) }
  ;
// What a class says a state does not hold: the atom of a goal's `(absent …)`.
// The one place `absent` heads a form that is no premise, and no fact either:
// a claim lists a query's projection classes.
ClaimAtom -> b::Id:
    NestedFact { $1 }
  | EqFact { $1 }
  | NegFact { $1 }
  | "(" "not" EqFact ")" { b::named(p, "not", b::one($3)) }
  ;
ClaimFacts -> Vec<b::Id>: { b::none() } | ClaimFacts ClaimFact { b::push($1, $2) } ;

Solve -> b::Id: "(" "solve" SolveClauses ")" { b::named(p, "solve", $3) } ;
SolveClause -> b::Id:
    ":hrules" Hrules { b::pair(p, $lexer, $1, $2) }
  | ":hypothesis-relations" NameList { b::pair(p, $lexer, $1, $2) }
  | ":no-hypothesis" NameList { b::pair(p, $lexer, $1, $2) }
  // A flag, the one keyword that takes no value: the program
  // states that its lattice is root alone. Its pair holds `()`.
  | ":no-search" { b::pair(p, $lexer, $1, b::empty(p)) }
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
// The expected region.
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
// A macro may carry an NL template, as a relation and a rule do: `:why`
// once, after the body.
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
Derived -> b::Id: NestedFact { $1 } | NegFact { $1 } ;

// ── A saved file: the program, its modules, the choices made ─────────
// `(program …)` and `(module NAME …)` hold a file's text as it was read,
// parsed here as the forms it is and carried as its bytes, which the loader
// reads again as the file they were (Q-M1f.401). `(commit …)` names the
// candidates of H a node commits, each a fact as written (Q-M1f.400). None
// stands inside another, and none is a fact.
Saved -> b::Id:
    "(" "program" Forms ")" { b::carried(p, $lexer, $2, None, $4) }
  | "(" "module" ModuleName Forms ")" { b::carried(p, $lexer, $2, Some($3), $5) }
  | "(" "commit" CommitFacts ")" { b::named(p, "commit", $3) }
  ;
// A module's name, and where its text starts: just after it.
ModuleName -> (b::Id, usize): Name { ($1, $span.end()) } ;
CommitFact -> b::Id: NestedFact { $1 } | NegFact { $1 } ;
CommitFacts -> Vec<b::Id>: CommitFact { b::one($1) } | CommitFacts CommitFact { b::push($1, $2) } ;
%%
use super::b;
