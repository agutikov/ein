%grmtools {yacckind: Grmtools}
%start Program
%token "UNDERSCORE" "BAD" "BADTEXT"
%expect-unused "UNDERSCORE" "BAD" "BADTEXT"
%%
Program -> Vec<Node<'input>>:
  { Vec::new() }
  | Program Form { let mut v=$1; v.push($2); v }
  ;
Form -> Node<'input>:
  Relation { $1 }
  | Rule { $1 }
  | Hrule { $1 }
  | Query { $1 }
  | Solve { $1 }
  | Config { $1 }
  | Trace { $1 }
  | Macro { $1 }
  | Import { $1 }
  | NotFact { $1 }
  | Fact { $1 }
  ;
Name -> Node<'input>:
  "WORD" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  | "DIGITS" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  | "relation" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  | "step" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
Head -> Node<'input>:
  "WORD" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  | "DIGITS" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  | "step" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
Var -> Node<'input>:
  "VAR" { Node::Var($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
Text -> Node<'input>:
  "TEXT" { Node::Str($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
Count -> Node<'input>:
  "DIGITS" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WRelation -> Node<'input>:
  "relation" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WRule -> Node<'input>:
  "rule" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WHrule -> Node<'input>:
  "hrule" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WQuery -> Node<'input>:
  "query" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WSolve -> Node<'input>:
  "solve" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WConfig -> Node<'input>:
  "config" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WTrace -> Node<'input>:
  "trace" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WMacro -> Node<'input>:
  "macro" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WImport -> Node<'input>:
  "import" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WNot -> Node<'input>:
  "not" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WAnd -> Node<'input>:
  "and" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WOr -> Node<'input>:
  "or" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WNeq -> Node<'input>:
  "neq" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
WStep -> Node<'input>:
  "step" { Node::Name($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32) }
  ;
NameStar -> Vec<Node<'input>>:
  { Vec::new() }
  | NameStar Name { let mut v=$1; v.push($2); v }
  ;
VarStar -> Vec<Node<'input>>:
  { Vec::new() }
  | VarStar Var { let mut v=$1; v.push($2); v }
  ;
VarPlus -> Vec<Node<'input>>:
  Var { vec![$1] }
  | VarPlus Var { let mut v=$1; v.push($2); v }
  ;
Names -> Node<'input>:
  "(" NameStar ")" { Node::List($2) }
  ;
Params -> Node<'input>:
  "(" VarStar ")" { Node::List($2) }
  ;
Params1 -> Node<'input>:
  "(" VarPlus ")" { Node::List($2) }
  ;
HruleItem -> Node<'input>:
  Name { $1 }
  | Names { $1 }
  ;
HruleItemS -> Vec<Node<'input>>:
  { Vec::new() }
  | HruleItemS HruleItem { let mut v=$1; v.push($2); v }
  ;
Hrules -> Node<'input>:
  "(" HruleItemS ")" { Node::List($2) }
  ;
NamePlus -> Vec<Node<'input>>:
  Name { vec![$1] }
  | NamePlus Name { let mut v=$1; v.push($2); v }
  ;
Using -> Node<'input>:
  Names { $1 }
  | "(" WAnd NamePlus ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  ;
TArg -> Node<'input>:
  Var { $1 }
  | Term { $1 }
  ;
TArgPlus -> Vec<Node<'input>>:
  TArg { vec![$1] }
  | TArgPlus TArg { let mut v=$1; v.push($2); v }
  ;
RelClause -> Vec<Node<'input>>:
  ":why" Text { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  ;
RelClauseS -> Vec<Node<'input>>:
  { Vec::new() }
  | RelClauseS RelClause { let mut v=$1; v.extend($2); v }
  ;
Relation -> Node<'input>:
  "(" WRelation Name NameStar RelClauseS ")" { let mut r=vec![$2,$3]; r.extend($4); r.extend($5); Node::List(r) }
  ;
RuleClause -> Vec<Node<'input>>:
  ":match" Term { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":assert" Term { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":why" Text { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":priority" Count { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  ;
RuleClauseS -> Vec<Node<'input>>:
  { Vec::new() }
  | RuleClauseS RuleClause { let mut v=$1; v.extend($2); v }
  ;
Rule -> Node<'input>:
  "(" WRule Name Params RuleClauseS ")" { let mut r=vec![$2,$3,$4]; r.extend($5); Node::List(r) }
  ;
Hrule -> Node<'input>:
  "(" WHrule Name Params RuleClauseS ")" { let mut r=vec![$2,$3,$4]; r.extend($5); Node::List(r) }
  ;
QueryClause -> Vec<Node<'input>>:
  ":goal" Term { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":goal-text" Text { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":expect" Term { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":over" Name { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":no-hypothesis" Names { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  ;
QueryClauseP -> Vec<Node<'input>>:
  QueryClause { $1 }
  | QueryClauseP QueryClause { let mut v=$1; v.extend($2); v }
  ;
Query -> Node<'input>:
  "(" WQuery QueryClauseP ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  ;
SolveClause -> Vec<Node<'input>>:
  ":expect" Term { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":no-hypothesis" Names { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":hrules" Hrules { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":hypothesis-relations" Names { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  ;
SolveClauseS -> Vec<Node<'input>>:
  { Vec::new() }
  | SolveClauseS SolveClause { let mut v=$1; v.extend($2); v }
  ;
Solve -> Node<'input>:
  "(" WSolve SolveClauseS ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  ;
ConfigPair -> Vec<Node<'input>>:
  "KEYWORD" Name { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  ;
ConfigPairS -> Vec<Node<'input>>:
  { Vec::new() }
  | ConfigPairS ConfigPair { let mut v=$1; v.extend($2); v }
  ;
Config -> Node<'input>:
  "(" WConfig ConfigPairS ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  ;
StepClause -> Vec<Node<'input>>:
  ":rule" Name { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":using" Using { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":derives" FactTerm { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  ;
StepClauseS -> Vec<Node<'input>>:
  { Vec::new() }
  | StepClauseS StepClause { let mut v=$1; v.extend($2); v }
  ;
Step -> Node<'input>:
  "(" WStep Name StepClauseS ")" { let mut r=vec![$2,$3]; r.extend($4); Node::List(r) }
  ;
StepS -> Vec<Node<'input>>:
  { Vec::new() }
  | StepS Step { let mut v=$1; v.push($2); v }
  ;
Trace -> Node<'input>:
  "(" WTrace StepS ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  ;
Macro -> Node<'input>:
  "(" WMacro Name Params1 Term ")" { Node::List(vec![$2,$3,$4,$5]) }
  ;
ImportClause -> Vec<Node<'input>>:
  ":symbols" Names { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":as" Name { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  ;
ImportClauseS -> Vec<Node<'input>>:
  { Vec::new() }
  | ImportClauseS ImportClause { let mut v=$1; v.extend($2); v }
  ;
Import -> Node<'input>:
  "(" WImport Name ImportClauseS ")" { let mut r=vec![$2,$3]; r.extend($4); Node::List(r) }
  ;
FactClause -> Vec<Node<'input>>:
  ":source" Text { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":rule" Name { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  | ":using" Using { vec![Node::Kw($lexer.span_str($1.as_ref().unwrap().span()),$span.start() as u32),$2] }
  ;
FactClauseS -> Vec<Node<'input>>:
  { Vec::new() }
  | FactClauseS FactClause { let mut v=$1; v.extend($2); v }
  ;
NotFact -> Node<'input>:
  "(" WNot FactTerm FactClauseS ")" { let mut r=vec![$2,$3]; r.extend($4); Node::List(r) }
  ;
FactArg -> Node<'input>:
  Name { $1 }
  | FactTerm { $1 }
  ;
FactArgS -> Vec<Node<'input>>:
  { Vec::new() }
  | FactArgS FactArg { let mut v=$1; v.push($2); v }
  ;
Fact -> Node<'input>:
  "(" Head FactArgS FactClauseS ")" { let mut r=vec![$2]; r.extend($3); r.extend($4); Node::List(r) }
  ;
FactTerm -> Node<'input>:
  "(" Name FactArgS ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  ;
ItemS -> Vec<Node<'input>>:
  { Vec::new() }
  | ItemS Item { let mut v=$1; v.push($2); v }
  ;
Term -> Node<'input>:
  "(" WAnd TArgPlus ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  | "(" WOr TArgPlus ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  | "(" WNot TArg ")" { Node::List(vec![$2,$3]) }
  | "(" WNeq Item Item ")" { Node::List(vec![$2,$3,$4]) }
  | "(" PHead ItemS ")" { let mut r=vec![$2]; r.extend($3); Node::List(r) }
  ;
PHead -> Node<'input>:
  Name { $1 }
  | Var { $1 }
  ;
Item -> Node<'input>:
  Name { $1 }
  | Var { $1 }
  | Term { $1 }
  ;
%%
use common::Node;
