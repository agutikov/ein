# The rules every base shares: `std.math.positional`

Most of school arithmetic does not know its base: a walk takes a column and looks it up in a table, and nothing in
it says *ten* or *two*. Those rules are written here once, for decimal and binary, every relation a rule names a
parameter in capitals. This page is the module: it has no usage of its own, since a program never writes its macro
and never imports it. It comes with a base, [`std.math.decimal-arith`](08-natural-numbers.md) or
[`std.math.binary-arith`](14-binary.md).

## What a program sees of it

A shared rule prints by its own full name whichever base activated it: `std.math.positional.add-column` in a
trace, `std.math.positional.malformed-is-false` under `refuted by`. This request has a name no `let` states, and
the rule that refutes it is the shared one:

```lisp
(import std.math.decimal-arith :symbols (add sum-of))
(import std.math.decimal :symbols (n))
(import std.math.arith :symbols (z))

(add foo (n z 5))

(query :goal (sum-of ?x ?y ?s))
```

<!-- output -->
```text
  unsat core        (add foo (n z 5))
  refuted by        std.math.positional.undeclared-is-false

  (false)
```

A state also holds one activator fact for each shared rule and each base that connected it. These are the ones of
the addition walk in a decimal program, from `ein solve -P`:

<!-- output -->
```text
  (std.math.positional.add-carry-out std.math.decimal-arith.add-walk std.math.decimal-arith.turn std.math.decimal-arith.b)
  (std.math.positional.add-column std.math.decimal-arith.add-walk n std.math.digits.fa std.math.decimal-arith.b)
  (std.math.positional.add-column-first std.math.decimal-arith.add-walk n std.math.digits.fa std.math.decimal-arith.b)
  (std.math.positional.add-column-second std.math.decimal-arith.add-walk n std.math.digits.fa std.math.decimal-arith.b)
  (std.math.positional.add-end std.math.decimal-arith.add-walk std.math.decimal-arith.strip)
```

## How it works

### The connection: one macro, called once by a base

The module declares no relation and has one macro, `positional`, whose one call inside a base's closing rule
activates every shared rule with that base's relations
([Q-M1f.266](../../../../decisions/stdlib/math/Q-M1f.266-a-base-is-connected-to-the-shared-rules-by-one.md)):
`(positional operand operand-is n let named add-walk fa b strip turn nz done …)` in decimal's,
`(positional bin-operand … b2 bin-let …)` in binary's.

```lisp
(macro positional (?OPERAND ?OPERAND-IS ?N ?LET ?NAMED ?ADD-WALK ?FA ?B ?STRIP ?TURN ?NZ
                   ?DONE ?ADD ?ADD-FIRST ?ADD-SECOND ?SUM ?SUM-OF ?SUB-WALK ?FS ?SHORT ?SUB
                   ?SUB-FIRST ?SUB-SECOND ?DIFFERENCE ?SHORTFALL ?DIFF-OF ?CMP-WALK ?CL
                   ?ORDER-IS ?CMP ?CMP-FIRST ?CMP-SECOND ?ORDER ?CMP-OF ?TOP-FIRST
                   ?TOP-FIRST-IS ?MUL-CALL ?MUL-DIGITS ?MUL-STEP ?MUL-SUM ?MUL ?MUL-FIRST
                   ?MUL-SECOND ?PRODUCT ?PRODUCT-OF ?DIV-DIGITS ?DIV-STEP ?DIV-DONE ?DIV
                   ?DIV-FIRST ?DIV-SECOND ?DIV-CALL ?QUOTIENT ?QUOT-OF ?DIV-M1 ?BAD-OPERAND
                   ?NOT-A-NUMBER ?MALFORMED ?BAD-REQUEST ?NO-LET ?UNDECLARED)
  (and (operand-zero ?OPERAND ?OPERAND-IS)
       (operand-number ?OPERAND ?N ?OPERAND-IS)
       (let-named ?LET ?NAMED)
       (operand-name ?OPERAND ?NAMED ?LET ?OPERAND-IS)
       (add-column ?ADD-WALK ?N ?FA ?B)
       (add-column-first ?ADD-WALK ?N ?FA ?B)
       (add-column-second ?ADD-WALK ?N ?FA ?B)
       (add-end ?ADD-WALK ?STRIP)
       (add-carry-out ?ADD-WALK ?TURN ?B)
       (strip-zero ?STRIP ?B)
       (strip-top ?STRIP ?B ?NZ ?TURN)
       (strip-none ?STRIP ?DONE)
       (turn-step ?TURN ?B ?N)
       (turn-end ?TURN ?DONE)
       (add-ask ?ADD ?OPERAND ?ADD-FIRST)
       (add-ask-second ?OPERAND-IS ?ADD-FIRST ?OPERAND ?ADD-SECOND)
       (add-start ?OPERAND-IS ?ADD-SECOND ?ADD-WALK ?SUM)
       (add-answer ?DONE ?SUM ?SUM-OF)
       (sub-column ?SUB-WALK ?N ?FS ?B)
       (sub-column-first ?SUB-WALK ?N ?FS ?B)
       (sub-column-second ?SUB-WALK ?N ?FS ?B)
       (sub-end ?SUB-WALK ?STRIP)
       (sub-borrow-out ?SUB-WALK ?SHORT)
       (sub-ask ?SUB ?OPERAND ?SUB-FIRST)
       (sub-ask-second ?OPERAND-IS ?SUB-FIRST ?OPERAND ?SUB-SECOND)
       (sub-start ?OPERAND-IS ?SUB-SECOND ?SUB-WALK ?DIFFERENCE ?SHORTFALL)
       (sub-answer ?DONE ?DIFFERENCE ?DIFF-OF)
       (sub-answer-short ?DONE ?SHORTFALL ?DIFF-OF)
       (cmp-column ?CMP-WALK ?N ?CL)
       (cmp-column-first ?CMP-WALK ?N ?CL)
       (cmp-column-second ?CMP-WALK ?N ?CL)
       (cmp-end ?CMP-WALK ?ORDER-IS)
       (cmp-ask ?CMP ?OPERAND ?CMP-FIRST)
       (cmp-ask-second ?OPERAND-IS ?CMP-FIRST ?OPERAND ?CMP-SECOND)
       (cmp-start ?OPERAND-IS ?CMP-SECOND ?CMP-WALK ?ORDER)
       (cmp-answer ?ORDER-IS ?ORDER ?CMP-OF)
       (top-first-step ?TOP-FIRST ?N ?B)
       (top-first-end ?TOP-FIRST ?TOP-FIRST-IS)
       (mul-turn ?MUL-CALL ?TOP-FIRST ?MUL-DIGITS)
       (mul-begin ?TOP-FIRST-IS ?MUL-DIGITS ?MUL-STEP)
       (mul-next ?DONE ?MUL-SUM ?MUL-STEP)
       (mul-end ?MUL-STEP ?DONE)
       (mul-ask ?MUL ?OPERAND ?MUL-FIRST)
       (mul-ask-second ?OPERAND-IS ?MUL-FIRST ?OPERAND ?MUL-SECOND)
       (mul-start ?OPERAND-IS ?MUL-SECOND ?MUL-CALL ?PRODUCT)
       (mul-answer ?DONE ?PRODUCT ?PRODUCT-OF)
       (div-begin ?TOP-FIRST-IS ?DIV-DIGITS ?DIV-STEP)
       (div-end ?DIV-STEP ?DIV-DONE)
       (div-ask ?DIV ?OPERAND ?DIV-FIRST)
       (div-ask-second ?OPERAND-IS ?DIV-FIRST ?OPERAND ?DIV-SECOND)
       (div-start ?OPERAND-IS ?DIV-SECOND ?DIV-CALL ?QUOTIENT)
       (div-answer ?DIV-DONE ?QUOTIENT ?QUOT-OF)
       (div-by-zero ?DONE ?DIV-M1 ?QUOTIENT ?DIV)
       (div-call-by-zero ?DONE ?DIV-M1 ?DIV-CALL)
       (operand-digit ?OPERAND ?NZ ?BAD-OPERAND ?NOT-A-NUMBER ?N)
       (operand-digit-zero ?OPERAND ?BAD-OPERAND ?NOT-A-NUMBER)
       (let-twice ?LET ?MALFORMED)
       (let-zero ?LET ?MALFORMED)
       (let-digit ?LET ?NZ ?MALFORMED)
       (let-digit-zero ?LET ?MALFORMED)
       (let-to-name ?LET ?NAMED ?MALFORMED)
       (let-to-digit ?LET ?NZ ?MALFORMED ?N)
       (let-to-digit-zero ?LET ?MALFORMED)
       (bad-add-first ?BAD-OPERAND ?ADD-FIRST ?BAD-REQUEST ?ADD)
       (bad-add-second ?BAD-OPERAND ?ADD-SECOND ?BAD-REQUEST ?ADD)
       (bad-sub-first ?BAD-OPERAND ?SUB-FIRST ?BAD-REQUEST ?SUB)
       (bad-sub-second ?BAD-OPERAND ?SUB-SECOND ?BAD-REQUEST ?SUB)
       (bad-cmp-first ?BAD-OPERAND ?CMP-FIRST ?BAD-REQUEST ?CMP)
       (bad-cmp-second ?BAD-OPERAND ?CMP-SECOND ?BAD-REQUEST ?CMP)
       (bad-mul-first ?BAD-OPERAND ?MUL-FIRST ?BAD-REQUEST ?MUL)
       (bad-mul-second ?BAD-OPERAND ?MUL-SECOND ?BAD-REQUEST ?MUL)
       (bad-div-first ?BAD-OPERAND ?DIV-FIRST ?BAD-REQUEST ?DIV)
       (bad-div-second ?BAD-OPERAND ?DIV-SECOND ?BAD-REQUEST ?DIV)
       (bad-request-malformed ?BAD-REQUEST ?NOT-A-NUMBER ?MALFORMED)
       (bad-request-undeclared ?BAD-REQUEST ?NO-LET ?UNDECLARED)
       (malformed-is-false ?MALFORMED)
       (undeclared-is-false ?UNDECLARED))
  :why "the rules every positional base shares, each activated once with this base's relations: {?N} the numeral, {?ADD-WALK} the addition's walk, {?FA} its table, and the rest")
```

A parameter is the relation a base names for it: `?N` the numeral's constructor, `?B` the accumulator, `?FA`,
`?FS`, `?CL` and `?NZ` the tables of the full adder, the full subtractor, the comparator and *not zero*; the rest
are named as decimal names them, `?ADD-WALK` for `add-walk`. The digits `0` and `1`, which every base has, are
written as themselves: the carry and the borrow in and out, the high zero, the digit `0` as no number. So are `z`,
zero, and `eq`, a comparison's start.

Each rule below is activated by the fact of its name the macro states, so in a drawing a parameter is a diamond
like any variable, and a premise headed by one is an octagon labelled with the parameter. A rule's `:why` says
what one firing does, in the words a trace prints.

The module keeps the calculator's rule: no rule joins two facts on a number. Each has one premise that holds
numbers, and its others are rows of a table.

### The operands

A request's operand is looked up before anything is computed: zero and a numeral stand for themselves, and a name
stands for the number its `let` states. `let-named` marks each such name `named`, which is how the numeral's check
tells a constant from a stray name.

#### `operand-zero`

<!-- rule std.math.positional operand-zero -->
```lisp
(rule operand-zero (?OPERAND ?OPERAND-IS)
  :match  (?OPERAND ?k z)
  :assert (?OPERAND-IS ?k z)
  :why    "zero stands for itself")
```

```dot
digraph rule_operand_zero_lhs_rhs {
  rankdir=TB;
  label="operand-zero"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "?k_L" -> "z_L" [label="?OPERAND", color="#d62728", fontcolor="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?k_R" -> "z_R" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

#### `operand-number`

<!-- rule std.math.positional operand-number -->
```lisp
(rule operand-number (?OPERAND ?N ?OPERAND-IS)
  :match  (?OPERAND ?k (?N ?p ?a))
  :assert (?OPERAND-IS ?k (?N ?p ?a))
  :why    "a number stands for itself")
```

```dot
digraph rule_operand_number_lhs_rhs {
  rankdir=TB;
  label="operand-number"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?N ?p ?a)_L" [label="(?N ?p ?a)", shape=rectangle];
    "?k_L" -> "(?N ?p ?a)_L" [label="?OPERAND", color="#d62728", fontcolor="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "(?N ?p ?a)_R" [label="(?N ?p ?a)", shape=rectangle];
    "?k_R" -> "(?N ?p ?a)_R" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

#### `let-named`

<!-- rule std.math.positional let-named -->
```lisp
(rule let-named (?LET ?NAMED)
  :match  (?LET ?x ?v)
  :assert (?NAMED ?x)
  :why    "{?x} is the name of a number")
```

```dot
digraph rule_let_named_lhs_rhs {
  rankdir=TB;
  label="let-named"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?x_L" -> "?v_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "h1_NAMED_R" [shape=octagon, label="?NAMED", color="#bcbd22", fontcolor="#bcbd22"];
    "h1_NAMED_R" -> "?x_R" [label="1", color="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

#### `operand-name`

<!-- rule std.math.positional operand-name -->
```lisp
(rule operand-name (?OPERAND ?NAMED ?LET ?OPERAND-IS)
  :match  (and (?OPERAND ?k ?x) (?NAMED ?x) (?LET ?x ?v))
  :assert (?OPERAND-IS ?k ?v)
  :why    "{?x} names {?v}")
```

```dot
digraph rule_operand_name_lhs_rhs {
  rankdir=TB;
  label="operand-name"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "?x_L" [label="?x", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?k_L" -> "?x_L" [label="?OPERAND", color="#d62728", fontcolor="#d62728"];
    "h1_NAMED_L" [shape=octagon, label="?NAMED", color="#bcbd22", fontcolor="#bcbd22"];
    "h1_NAMED_L" -> "?x_L" [label="1", color="#bcbd22"];
    "?x_L" -> "?v_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?v_R" [label="?v", shape=diamond];
    "?k_R" -> "?v_R" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

### The addition walk

One walk from the units. With $x_i$ and $y_i$ the digits of the two numbers, $c_i$ the carry into column $i$ and
$b$ the base, each column is one row of the adder's table:

$$
x_i + y_i + c_i = b \cdot c_{i+1} + s_i, \qquad c_0 \in \{0, 1\}
$$

`add-column` takes a column where both numbers have a digit, `add-column-first` and `add-column-second` one where
a number has run out, and the walk ends by `add-end`, with no carry, or by `add-carry-out`, which puts the last
carry in front. Every circuit that adds, a sum, a product's rows, a conversion, runs this walk under a tag of its
own.

#### `add-column`

<!-- rule std.math.positional add-column -->
```lisp
(rule add-column (?ADD-WALK ?N ?FA ?B)
  :match  (and (?ADD-WALK ?k (?N ?p ?a) (?N ?q ?c) ?e ?acc)
               (?FA ?a ?c ?e ?s ?f))
  :assert (?ADD-WALK ?k ?p ?q ?f (?B ?s ?acc))
  :why    "{?a} + {?c} + carry {?e} is {?s}, carry {?f}: {?s} goes in front of the digits so far")
```

```dot
digraph rule_add_column_lhs_rhs {
  rankdir=TB;
  label="add-column"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?N ?p ?a)_L" [label="(?N ?p ?a)", shape=rectangle];
    "(?N ?q ?c)_L" [label="(?N ?q ?c)", shape=rectangle];
    "?e_L" [label="?e", shape=diamond];
    "?acc_L" [label="?acc", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?s_L" [label="?s", shape=diamond];
    "?f_L" [label="?f", shape=diamond];
    "h1_ADD-WALK_L" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h1_ADD-WALK_L" -> "?k_L" [label="1", color="#17becf"];
    "h1_ADD-WALK_L" -> "(?N ?p ?a)_L" [label="2", color="#17becf"];
    "h1_ADD-WALK_L" -> "(?N ?q ?c)_L" [label="3", color="#17becf"];
    "h1_ADD-WALK_L" -> "?e_L" [label="4", color="#17becf"];
    "h1_ADD-WALK_L" -> "?acc_L" [label="5", color="#17becf"];
    "h2_FA_L" [shape=octagon, label="?FA", color="#17becf", fontcolor="#17becf"];
    "h2_FA_L" -> "?a_L" [label="1", color="#17becf"];
    "h2_FA_L" -> "?c_L" [label="2", color="#17becf"];
    "h2_FA_L" -> "?e_L" [label="3", color="#17becf"];
    "h2_FA_L" -> "?s_L" [label="4", color="#17becf"];
    "h2_FA_L" -> "?f_L" [label="5", color="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?p_R" [label="?p", shape=diamond];
    "?q_R" [label="?q", shape=diamond];
    "?f_R" [label="?f", shape=diamond];
    "(?B ?s ?acc)_R" [label="(?B ?s ?acc)", shape=rectangle];
    "h3_ADD-WALK_R" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h3_ADD-WALK_R" -> "?k_R" [label="1", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "?p_R" [label="2", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "?q_R" [label="3", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "?f_R" [label="4", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "(?B ?s ?acc)_R" [label="5", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `add-column-first`

<!-- rule std.math.positional add-column-first -->
```lisp
(rule add-column-first (?ADD-WALK ?N ?FA ?B)
  :match  (and (?ADD-WALK ?k (?N ?p ?a) z ?e ?acc)
               (?FA ?a 0 ?e ?s ?f))
  :assert (?ADD-WALK ?k ?p z ?f (?B ?s ?acc))
  :why    "the second number has run out: {?a} + carry {?e} is {?s}, carry {?f}")
```

```dot
digraph rule_add_column_first_lhs_rhs {
  rankdir=TB;
  label="add-column-first"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?N ?p ?a)_L" [label="(?N ?p ?a)", shape=rectangle];
    "z_L" [label="z", shape=rectangle];
    "?e_L" [label="?e", shape=diamond];
    "?acc_L" [label="?acc", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?s_L" [label="?s", shape=diamond];
    "?f_L" [label="?f", shape=diamond];
    "h1_ADD-WALK_L" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h1_ADD-WALK_L" -> "?k_L" [label="1", color="#17becf"];
    "h1_ADD-WALK_L" -> "(?N ?p ?a)_L" [label="2", color="#17becf"];
    "h1_ADD-WALK_L" -> "z_L" [label="3", color="#17becf"];
    "h1_ADD-WALK_L" -> "?e_L" [label="4", color="#17becf"];
    "h1_ADD-WALK_L" -> "?acc_L" [label="5", color="#17becf"];
    "h2_FA_L" [shape=octagon, label="?FA", color="#17becf", fontcolor="#17becf"];
    "h2_FA_L" -> "?a_L" [label="1", color="#17becf"];
    "h2_FA_L" -> "0_L" [label="2", color="#17becf"];
    "h2_FA_L" -> "?e_L" [label="3", color="#17becf"];
    "h2_FA_L" -> "?s_L" [label="4", color="#17becf"];
    "h2_FA_L" -> "?f_L" [label="5", color="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?p_R" [label="?p", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?B ?s ?acc)_R" [label="(?B ?s ?acc)", shape=rectangle];
    "h3_ADD-WALK_R" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h3_ADD-WALK_R" -> "?k_R" [label="1", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "?p_R" [label="2", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "z_R" [label="3", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "?f_R" [label="4", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "(?B ?s ?acc)_R" [label="5", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `add-column-second`

<!-- rule std.math.positional add-column-second -->
```lisp
(rule add-column-second (?ADD-WALK ?N ?FA ?B)
  :match  (and (?ADD-WALK ?k z (?N ?q ?c) ?e ?acc)
               (?FA 0 ?c ?e ?s ?f))
  :assert (?ADD-WALK ?k z ?q ?f (?B ?s ?acc))
  :why    "the first number has run out: {?c} + carry {?e} is {?s}, carry {?f}")
```

```dot
digraph rule_add_column_second_lhs_rhs {
  rankdir=TB;
  label="add-column-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "(?N ?q ?c)_L" [label="(?N ?q ?c)", shape=rectangle];
    "?e_L" [label="?e", shape=diamond];
    "?acc_L" [label="?acc", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?c_L" [label="?c", shape=diamond];
    "?s_L" [label="?s", shape=diamond];
    "?f_L" [label="?f", shape=diamond];
    "h1_ADD-WALK_L" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h1_ADD-WALK_L" -> "?k_L" [label="1", color="#17becf"];
    "h1_ADD-WALK_L" -> "z_L" [label="2", color="#17becf"];
    "h1_ADD-WALK_L" -> "(?N ?q ?c)_L" [label="3", color="#17becf"];
    "h1_ADD-WALK_L" -> "?e_L" [label="4", color="#17becf"];
    "h1_ADD-WALK_L" -> "?acc_L" [label="5", color="#17becf"];
    "h2_FA_L" [shape=octagon, label="?FA", color="#17becf", fontcolor="#17becf"];
    "h2_FA_L" -> "0_L" [label="1", color="#17becf"];
    "h2_FA_L" -> "?c_L" [label="2", color="#17becf"];
    "h2_FA_L" -> "?e_L" [label="3", color="#17becf"];
    "h2_FA_L" -> "?s_L" [label="4", color="#17becf"];
    "h2_FA_L" -> "?f_L" [label="5", color="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?q_R" [label="?q", shape=diamond];
    "?f_R" [label="?f", shape=diamond];
    "(?B ?s ?acc)_R" [label="(?B ?s ?acc)", shape=rectangle];
    "h3_ADD-WALK_R" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h3_ADD-WALK_R" -> "?k_R" [label="1", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "z_R" [label="2", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "?q_R" [label="3", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "?f_R" [label="4", color="#17becf", style=dashed];
    "h3_ADD-WALK_R" -> "(?B ?s ?acc)_R" [label="5", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `add-end`

<!-- rule std.math.positional add-end -->
```lisp
(rule add-end (?ADD-WALK ?STRIP)
  :match  (?ADD-WALK ?k z z 0 ?acc)
  :assert (?STRIP ?k ?acc)
  :why    "no column is left and no carry: the digits are {?acc}")
```

```dot
digraph rule_add_end_lhs_rhs {
  rankdir=TB;
  label="add-end"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "0_L" [label="0", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_ADD-WALK_L" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h1_ADD-WALK_L" -> "?k_L" [label="1", color="#17becf"];
    "h1_ADD-WALK_L" -> "z_L" [label="2", color="#17becf"];
    "h1_ADD-WALK_L" -> "z_L" [label="3", color="#17becf"];
    "h1_ADD-WALK_L" -> "0_L" [label="4", color="#17becf"];
    "h1_ADD-WALK_L" -> "?acc_L" [label="5", color="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?acc_R" [label="?acc", shape=diamond];
    "?k_R" -> "?acc_R" [label="?STRIP", color="#ff7f0e", fontcolor="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `add-carry-out`

<!-- rule std.math.positional add-carry-out -->
```lisp
(rule add-carry-out (?ADD-WALK ?TURN ?B)
  :match  (?ADD-WALK ?k z z 1 ?acc)
  :assert (?TURN ?k (?B 1 ?acc) z)
  :why    "no column is left: the carry is the top digit, in front of {?acc}")
```

```dot
digraph rule_add_carry_out_lhs_rhs {
  rankdir=TB;
  label="add-carry-out"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "1_L" [label="1", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_ADD-WALK_L" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h1_ADD-WALK_L" -> "?k_L" [label="1", color="#17becf"];
    "h1_ADD-WALK_L" -> "z_L" [label="2", color="#17becf"];
    "h1_ADD-WALK_L" -> "z_L" [label="3", color="#17becf"];
    "h1_ADD-WALK_L" -> "1_L" [label="4", color="#17becf"];
    "h1_ADD-WALK_L" -> "?acc_L" [label="5", color="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "(?B 1 ?acc)_R" [label="(?B 1 ?acc)", shape=rectangle];
    "z_R" [label="z", shape=rectangle];
    "h2_TURN_R" [shape=octagon, label="?TURN", color="#9467bd", fontcolor="#9467bd"];
    "h2_TURN_R" -> "?k_R" [label="1", color="#9467bd", style=dashed];
    "h2_TURN_R" -> "(?B 1 ?acc)_R" [label="2", color="#9467bd", style=dashed];
    "h2_TURN_R" -> "z_R" [label="3", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

### The normal form

A walk leaves its digits in an accumulator, the top digit first. `strip` drops the high zeros, and `turn` turns
what is left round into the numeral's shape and hands it over as `done`. Zero comes out as `z`.

#### `strip-zero`

<!-- rule std.math.positional strip-zero -->
```lisp
(rule strip-zero (?STRIP ?B)
  :match  (?STRIP ?k (?B 0 ?r))
  :assert (?STRIP ?k ?r)
  :why    "a high zero is dropped")
```

```dot
digraph rule_strip_zero_lhs_rhs {
  rankdir=TB;
  label="strip-zero"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?B 0 ?r)_L" [label="(?B 0 ?r)", shape=rectangle];
    "?k_L" -> "(?B 0 ?r)_L" [label="?STRIP", color="#ff7f0e", fontcolor="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?r_R" [label="?r", shape=diamond];
    "?k_R" -> "?r_R" [label="?STRIP", color="#ff7f0e", fontcolor="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `strip-top`

<!-- rule std.math.positional strip-top -->
```lisp
(rule strip-top (?STRIP ?B ?NZ ?TURN)
  :match  (and (?STRIP ?k (?B ?a ?r)) (?NZ ?a))
  :assert (?TURN ?k (?B ?a ?r) z)
  :why    "the top digit {?a} is not zero: no high zero is left")
```

```dot
digraph rule_strip_top_lhs_rhs {
  rankdir=TB;
  label="strip-top"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?B ?a ?r)_L" [label="(?B ?a ?r)", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?k_L" -> "(?B ?a ?r)_L" [label="?STRIP", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_NZ_L" [shape=octagon, label="?NZ", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_NZ_L" -> "?a_L" [label="1", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "(?B ?a ?r)_R" [label="(?B ?a ?r)", shape=rectangle];
    "z_R" [label="z", shape=rectangle];
    "h2_TURN_R" [shape=octagon, label="?TURN", color="#9467bd", fontcolor="#9467bd"];
    "h2_TURN_R" -> "?k_R" [label="1", color="#9467bd", style=dashed];
    "h2_TURN_R" -> "(?B ?a ?r)_R" [label="2", color="#9467bd", style=dashed];
    "h2_TURN_R" -> "z_R" [label="3", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `strip-none`

<!-- rule std.math.positional strip-none -->
```lisp
(rule strip-none (?STRIP ?DONE)
  :match  (?STRIP ?k z)
  :assert (?DONE ?k z)
  :why    "no digit is left: the answer is zero")
```

```dot
digraph rule_strip_none_lhs_rhs {
  rankdir=TB;
  label="strip-none"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "?k_L" -> "z_L" [label="?STRIP", color="#ff7f0e", fontcolor="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?k_R" -> "z_R" [label="?DONE", color="#9467bd", fontcolor="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `turn-step`

<!-- rule std.math.positional turn-step -->
```lisp
(rule turn-step (?TURN ?B ?N)
  :match  (?TURN ?k (?B ?a ?r) ?acc)
  :assert (?TURN ?k ?r (?N ?acc ?a))
  :why    "the digit {?a} goes after {?acc}")
```

```dot
digraph rule_turn_step_lhs_rhs {
  rankdir=TB;
  label="turn-step"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?B ?a ?r)_L" [label="(?B ?a ?r)", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_TURN_L" [shape=octagon, label="?TURN", color="#9467bd", fontcolor="#9467bd"];
    "h1_TURN_L" -> "?k_L" [label="1", color="#9467bd"];
    "h1_TURN_L" -> "(?B ?a ?r)_L" [label="2", color="#9467bd"];
    "h1_TURN_L" -> "?acc_L" [label="3", color="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?r_R" [label="?r", shape=diamond];
    "(?N ?acc ?a)_R" [label="(?N ?acc ?a)", shape=rectangle];
    "h2_TURN_R" [shape=octagon, label="?TURN", color="#9467bd", fontcolor="#9467bd"];
    "h2_TURN_R" -> "?k_R" [label="1", color="#9467bd", style=dashed];
    "h2_TURN_R" -> "?r_R" [label="2", color="#9467bd", style=dashed];
    "h2_TURN_R" -> "(?N ?acc ?a)_R" [label="3", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `turn-end`

<!-- rule std.math.positional turn-end -->
```lisp
(rule turn-end (?TURN ?DONE)
  :match  (?TURN ?k z ?acc)
  :assert (?DONE ?k ?acc)
  :why    "turned round: the answer is {?acc}")
```

```dot
digraph rule_turn_end_lhs_rhs {
  rankdir=TB;
  label="turn-end"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_TURN_L" [shape=octagon, label="?TURN", color="#9467bd", fontcolor="#9467bd"];
    "h1_TURN_L" -> "?k_L" [label="1", color="#9467bd"];
    "h1_TURN_L" -> "z_L" [label="2", color="#9467bd"];
    "h1_TURN_L" -> "?acc_L" [label="3", color="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?acc_R" [label="?acc", shape=diamond];
    "?k_R" -> "?acc_R" [label="?DONE", color="#9467bd", fontcolor="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

### Addition: the request

A request asks its first operand, then its second, then starts the walk with no carry, and the walk's answer
becomes the result fact with the operands as the request wrote them.

#### `add-ask`

<!-- rule std.math.positional add-ask -->
```lisp
(rule add-ask (?ADD ?OPERAND ?ADD-FIRST)
  :match  (?ADD ?x ?y)
  :assert (?OPERAND (?ADD-FIRST ?x ?y) ?x)
  :why    "{?x} + {?y} is asked: first, the number {?x} stands for")
```

```dot
digraph rule_add_ask_lhs_rhs {
  rankdir=TB;
  label="add-ask"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "?x_L" -> "?y_L" [label="?ADD", color="#2ca02c", fontcolor="#2ca02c"];
  }
  subgraph cluster_rhs { label="assert";
    "(?ADD-FIRST ?x ?y)_R" [label="(?ADD-FIRST ?x ?y)", shape=rectangle];
    "?x_R" [label="?x", shape=diamond];
    "(?ADD-FIRST ?x ?y)_R" -> "?x_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `add-ask-second`

<!-- rule std.math.positional add-ask-second -->
```lisp
(rule add-ask-second (?OPERAND-IS ?ADD-FIRST ?OPERAND ?ADD-SECOND)
  :match  (?OPERAND-IS (?ADD-FIRST ?x ?y) ?u)
  :assert (?OPERAND (?ADD-SECOND ?x ?y ?u) ?y)
  :why    "the first operand is {?u}: now the number {?y} stands for")
```

```dot
digraph rule_add_ask_second_lhs_rhs {
  rankdir=TB;
  label="add-ask-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?ADD-FIRST ?x ?y)_L" [label="(?ADD-FIRST ?x ?y)", shape=rectangle];
    "?u_L" [label="?u", shape=diamond];
    "(?ADD-FIRST ?x ?y)_L" -> "?u_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?ADD-SECOND ?x ?y ?u)_R" [label="(?ADD-SECOND ?x ?y ?u)", shape=rectangle];
    "?y_R" [label="?y", shape=diamond];
    "(?ADD-SECOND ?x ?y ?u)_R" -> "?y_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `add-start`

<!-- rule std.math.positional add-start -->
```lisp
(rule add-start (?OPERAND-IS ?ADD-SECOND ?ADD-WALK ?SUM)
  :match  (?OPERAND-IS (?ADD-SECOND ?x ?y ?u) ?v)
  :assert (?ADD-WALK (?SUM ?x ?y) ?u ?v 0 z)
  :why    "add {?u} and {?v} from the units, with no carry in")
```

```dot
digraph rule_add_start_lhs_rhs {
  rankdir=TB;
  label="add-start"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?ADD-SECOND ?x ?y ?u)_L" [label="(?ADD-SECOND ?x ?y ?u)", shape=rectangle];
    "?v_L" [label="?v", shape=diamond];
    "(?ADD-SECOND ?x ?y ?u)_L" -> "?v_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?SUM ?x ?y)_R" [label="(?SUM ?x ?y)", shape=rectangle];
    "?u_R" [label="?u", shape=diamond];
    "?v_R" [label="?v", shape=diamond];
    "0_R" [label="0", shape=rectangle];
    "z_R" [label="z", shape=rectangle];
    "h1_ADD-WALK_R" [shape=octagon, label="?ADD-WALK", color="#17becf", fontcolor="#17becf"];
    "h1_ADD-WALK_R" -> "(?SUM ?x ?y)_R" [label="1", color="#17becf", style=dashed];
    "h1_ADD-WALK_R" -> "?u_R" [label="2", color="#17becf", style=dashed];
    "h1_ADD-WALK_R" -> "?v_R" [label="3", color="#17becf", style=dashed];
    "h1_ADD-WALK_R" -> "0_R" [label="4", color="#17becf", style=dashed];
    "h1_ADD-WALK_R" -> "z_R" [label="5", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `add-answer`

<!-- rule std.math.positional add-answer -->
```lisp
(rule add-answer (?DONE ?SUM ?SUM-OF)
  :match  (?DONE (?SUM ?x ?y) ?s)
  :assert (?SUM-OF ?x ?y ?s)
  :why    "{?x} + {?y} = {?s}")
```

```dot
digraph rule_add_answer_lhs_rhs {
  rankdir=TB;
  label="add-answer"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?SUM ?x ?y)_L" [label="(?SUM ?x ?y)", shape=rectangle];
    "?s_L" [label="?s", shape=diamond];
    "(?SUM ?x ?y)_L" -> "?s_L" [label="?DONE", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "?y_R" [label="?y", shape=diamond];
    "?s_R" [label="?s", shape=diamond];
    "h1_SUM-OF_R" [shape=octagon, label="?SUM-OF", color="#bcbd22", fontcolor="#bcbd22"];
    "h1_SUM-OF_R" -> "?x_R" [label="1", color="#bcbd22", style=dashed];
    "h1_SUM-OF_R" -> "?y_R" [label="2", color="#bcbd22", style=dashed];
    "h1_SUM-OF_R" -> "?s_R" [label="3", color="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

### Subtraction

The walk is the addition's over the subtractor's table,

$$
x_i - y_i - e_i = s_i - b \cdot e_{i+1}, \qquad e_0 = 0
$$

with $e_i$ the borrow. Which number is the smaller is not known before the top column, so `sub-start` runs two
walks, each number from the other: the one that ends with no borrow (`sub-end`) answers, and the one that ends with
a borrow (`sub-borrow-out`) says `short`. `sub-answer` and `sub-answer-short` put the one answer in the
difference's place or in the shortfall's.

#### `sub-column`

<!-- rule std.math.positional sub-column -->
```lisp
(rule sub-column (?SUB-WALK ?N ?FS ?B)
  :match  (and (?SUB-WALK ?k (?N ?p ?a) (?N ?q ?c) ?e ?acc)
               (?FS ?a ?c ?e ?s ?f))
  :assert (?SUB-WALK ?k ?p ?q ?f (?B ?s ?acc))
  :why    "{?a} - {?c} - borrow {?e} is {?s}, borrow {?f}: {?s} goes in front of the digits so far")
```

```dot
digraph rule_sub_column_lhs_rhs {
  rankdir=TB;
  label="sub-column"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?N ?p ?a)_L" [label="(?N ?p ?a)", shape=rectangle];
    "(?N ?q ?c)_L" [label="(?N ?q ?c)", shape=rectangle];
    "?e_L" [label="?e", shape=diamond];
    "?acc_L" [label="?acc", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?s_L" [label="?s", shape=diamond];
    "?f_L" [label="?f", shape=diamond];
    "h1_SUB-WALK_L" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h1_SUB-WALK_L" -> "?k_L" [label="1", color="#d62728"];
    "h1_SUB-WALK_L" -> "(?N ?p ?a)_L" [label="2", color="#d62728"];
    "h1_SUB-WALK_L" -> "(?N ?q ?c)_L" [label="3", color="#d62728"];
    "h1_SUB-WALK_L" -> "?e_L" [label="4", color="#d62728"];
    "h1_SUB-WALK_L" -> "?acc_L" [label="5", color="#d62728"];
    "h2_FS_L" [shape=octagon, label="?FS", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_FS_L" -> "?a_L" [label="1", color="#7f7f7f"];
    "h2_FS_L" -> "?c_L" [label="2", color="#7f7f7f"];
    "h2_FS_L" -> "?e_L" [label="3", color="#7f7f7f"];
    "h2_FS_L" -> "?s_L" [label="4", color="#7f7f7f"];
    "h2_FS_L" -> "?f_L" [label="5", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?p_R" [label="?p", shape=diamond];
    "?q_R" [label="?q", shape=diamond];
    "?f_R" [label="?f", shape=diamond];
    "(?B ?s ?acc)_R" [label="(?B ?s ?acc)", shape=rectangle];
    "h3_SUB-WALK_R" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h3_SUB-WALK_R" -> "?k_R" [label="1", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "?p_R" [label="2", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "?q_R" [label="3", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "?f_R" [label="4", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "(?B ?s ?acc)_R" [label="5", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-column-first`

<!-- rule std.math.positional sub-column-first -->
```lisp
(rule sub-column-first (?SUB-WALK ?N ?FS ?B)
  :match  (and (?SUB-WALK ?k (?N ?p ?a) z ?e ?acc)
               (?FS ?a 0 ?e ?s ?f))
  :assert (?SUB-WALK ?k ?p z ?f (?B ?s ?acc))
  :why    "the number taken away has run out: {?a} - borrow {?e} is {?s}, borrow {?f}")
```

```dot
digraph rule_sub_column_first_lhs_rhs {
  rankdir=TB;
  label="sub-column-first"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?N ?p ?a)_L" [label="(?N ?p ?a)", shape=rectangle];
    "z_L" [label="z", shape=rectangle];
    "?e_L" [label="?e", shape=diamond];
    "?acc_L" [label="?acc", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?s_L" [label="?s", shape=diamond];
    "?f_L" [label="?f", shape=diamond];
    "h1_SUB-WALK_L" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h1_SUB-WALK_L" -> "?k_L" [label="1", color="#d62728"];
    "h1_SUB-WALK_L" -> "(?N ?p ?a)_L" [label="2", color="#d62728"];
    "h1_SUB-WALK_L" -> "z_L" [label="3", color="#d62728"];
    "h1_SUB-WALK_L" -> "?e_L" [label="4", color="#d62728"];
    "h1_SUB-WALK_L" -> "?acc_L" [label="5", color="#d62728"];
    "h2_FS_L" [shape=octagon, label="?FS", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_FS_L" -> "?a_L" [label="1", color="#7f7f7f"];
    "h2_FS_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h2_FS_L" -> "?e_L" [label="3", color="#7f7f7f"];
    "h2_FS_L" -> "?s_L" [label="4", color="#7f7f7f"];
    "h2_FS_L" -> "?f_L" [label="5", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?p_R" [label="?p", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?B ?s ?acc)_R" [label="(?B ?s ?acc)", shape=rectangle];
    "h3_SUB-WALK_R" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h3_SUB-WALK_R" -> "?k_R" [label="1", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "?p_R" [label="2", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "z_R" [label="3", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "?f_R" [label="4", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "(?B ?s ?acc)_R" [label="5", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-column-second`

<!-- rule std.math.positional sub-column-second -->
```lisp
(rule sub-column-second (?SUB-WALK ?N ?FS ?B)
  :match  (and (?SUB-WALK ?k z (?N ?q ?c) ?e ?acc)
               (?FS 0 ?c ?e ?s ?f))
  :assert (?SUB-WALK ?k z ?q ?f (?B ?s ?acc))
  :why    "the number taken from has run out: 0 - {?c} - borrow {?e} is {?s}, borrow {?f}")
```

```dot
digraph rule_sub_column_second_lhs_rhs {
  rankdir=TB;
  label="sub-column-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "(?N ?q ?c)_L" [label="(?N ?q ?c)", shape=rectangle];
    "?e_L" [label="?e", shape=diamond];
    "?acc_L" [label="?acc", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?c_L" [label="?c", shape=diamond];
    "?s_L" [label="?s", shape=diamond];
    "?f_L" [label="?f", shape=diamond];
    "h1_SUB-WALK_L" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h1_SUB-WALK_L" -> "?k_L" [label="1", color="#d62728"];
    "h1_SUB-WALK_L" -> "z_L" [label="2", color="#d62728"];
    "h1_SUB-WALK_L" -> "(?N ?q ?c)_L" [label="3", color="#d62728"];
    "h1_SUB-WALK_L" -> "?e_L" [label="4", color="#d62728"];
    "h1_SUB-WALK_L" -> "?acc_L" [label="5", color="#d62728"];
    "h2_FS_L" [shape=octagon, label="?FS", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_FS_L" -> "0_L" [label="1", color="#7f7f7f"];
    "h2_FS_L" -> "?c_L" [label="2", color="#7f7f7f"];
    "h2_FS_L" -> "?e_L" [label="3", color="#7f7f7f"];
    "h2_FS_L" -> "?s_L" [label="4", color="#7f7f7f"];
    "h2_FS_L" -> "?f_L" [label="5", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?q_R" [label="?q", shape=diamond];
    "?f_R" [label="?f", shape=diamond];
    "(?B ?s ?acc)_R" [label="(?B ?s ?acc)", shape=rectangle];
    "h3_SUB-WALK_R" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h3_SUB-WALK_R" -> "?k_R" [label="1", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "z_R" [label="2", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "?q_R" [label="3", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "?f_R" [label="4", color="#d62728", style=dashed];
    "h3_SUB-WALK_R" -> "(?B ?s ?acc)_R" [label="5", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-end`

<!-- rule std.math.positional sub-end -->
```lisp
(rule sub-end (?SUB-WALK ?STRIP)
  :match  (?SUB-WALK ?k z z 0 ?acc)
  :assert (?STRIP ?k ?acc)
  :why    "no column is left and no borrow: the digits are {?acc}")
```

```dot
digraph rule_sub_end_lhs_rhs {
  rankdir=TB;
  label="sub-end"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "0_L" [label="0", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_SUB-WALK_L" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h1_SUB-WALK_L" -> "?k_L" [label="1", color="#d62728"];
    "h1_SUB-WALK_L" -> "z_L" [label="2", color="#d62728"];
    "h1_SUB-WALK_L" -> "z_L" [label="3", color="#d62728"];
    "h1_SUB-WALK_L" -> "0_L" [label="4", color="#d62728"];
    "h1_SUB-WALK_L" -> "?acc_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?acc_R" [label="?acc", shape=diamond];
    "?k_R" -> "?acc_R" [label="?STRIP", color="#ff7f0e", fontcolor="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-borrow-out`

<!-- rule std.math.positional sub-borrow-out -->
```lisp
(rule sub-borrow-out (?SUB-WALK ?SHORT)
  :match  (?SUB-WALK ?k z z 1 ?acc)
  :assert (?SHORT ?k)
  :why    "no column is left and a borrow is: the number taken from is the smaller")
```

```dot
digraph rule_sub_borrow_out_lhs_rhs {
  rankdir=TB;
  label="sub-borrow-out"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "1_L" [label="1", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_SUB-WALK_L" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h1_SUB-WALK_L" -> "?k_L" [label="1", color="#d62728"];
    "h1_SUB-WALK_L" -> "z_L" [label="2", color="#d62728"];
    "h1_SUB-WALK_L" -> "z_L" [label="3", color="#d62728"];
    "h1_SUB-WALK_L" -> "1_L" [label="4", color="#d62728"];
    "h1_SUB-WALK_L" -> "?acc_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "h2_SHORT_R" [shape=octagon, label="?SHORT", color="#9467bd", fontcolor="#9467bd"];
    "h2_SHORT_R" -> "?k_R" [label="1", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-ask`

<!-- rule std.math.positional sub-ask -->
```lisp
(rule sub-ask (?SUB ?OPERAND ?SUB-FIRST)
  :match  (?SUB ?x ?y)
  :assert (?OPERAND (?SUB-FIRST ?x ?y) ?x)
  :why    "{?x} - {?y} is asked: first, the number {?x} stands for")
```

```dot
digraph rule_sub_ask_lhs_rhs {
  rankdir=TB;
  label="sub-ask"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "?x_L" -> "?y_L" [label="?SUB", color="#1f77b4", fontcolor="#1f77b4"];
  }
  subgraph cluster_rhs { label="assert";
    "(?SUB-FIRST ?x ?y)_R" [label="(?SUB-FIRST ?x ?y)", shape=rectangle];
    "?x_R" [label="?x", shape=diamond];
    "(?SUB-FIRST ?x ?y)_R" -> "?x_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-ask-second`

<!-- rule std.math.positional sub-ask-second -->
```lisp
(rule sub-ask-second (?OPERAND-IS ?SUB-FIRST ?OPERAND ?SUB-SECOND)
  :match  (?OPERAND-IS (?SUB-FIRST ?x ?y) ?u)
  :assert (?OPERAND (?SUB-SECOND ?x ?y ?u) ?y)
  :why    "the first operand is {?u}: now the number {?y} stands for")
```

```dot
digraph rule_sub_ask_second_lhs_rhs {
  rankdir=TB;
  label="sub-ask-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?SUB-FIRST ?x ?y)_L" [label="(?SUB-FIRST ?x ?y)", shape=rectangle];
    "?u_L" [label="?u", shape=diamond];
    "(?SUB-FIRST ?x ?y)_L" -> "?u_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?SUB-SECOND ?x ?y ?u)_R" [label="(?SUB-SECOND ?x ?y ?u)", shape=rectangle];
    "?y_R" [label="?y", shape=diamond];
    "(?SUB-SECOND ?x ?y ?u)_R" -> "?y_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-start`

<!-- rule std.math.positional sub-start -->
```lisp
(rule sub-start (?OPERAND-IS ?SUB-SECOND ?SUB-WALK ?DIFFERENCE ?SHORTFALL)
  :match  (?OPERAND-IS (?SUB-SECOND ?x ?y ?u) ?v)
  :assert (and (?SUB-WALK (?DIFFERENCE ?x ?y) ?u ?v 0 z)
               (?SUB-WALK (?SHORTFALL ?x ?y) ?v ?u 0 z))
  :why    "take {?v} from {?u}, and {?u} from {?v}, from the units, with no borrow in: one of the two has an answer")
```

```dot
digraph rule_sub_start_lhs_rhs {
  rankdir=TB;
  label="sub-start"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?SUB-SECOND ?x ?y ?u)_L" [label="(?SUB-SECOND ?x ?y ?u)", shape=rectangle];
    "?v_L" [label="?v", shape=diamond];
    "(?SUB-SECOND ?x ?y ?u)_L" -> "?v_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?DIFFERENCE ?x ?y)_R" [label="(?DIFFERENCE ?x ?y)", shape=rectangle];
    "?u_R" [label="?u", shape=diamond];
    "?v_R" [label="?v", shape=diamond];
    "0_R" [label="0", shape=rectangle];
    "z_R" [label="z", shape=rectangle];
    "(?SHORTFALL ?x ?y)_R" [label="(?SHORTFALL ?x ?y)", shape=rectangle];
    "h1_SUB-WALK_R" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h1_SUB-WALK_R" -> "(?DIFFERENCE ?x ?y)_R" [label="1", color="#d62728", style=dashed];
    "h1_SUB-WALK_R" -> "?u_R" [label="2", color="#d62728", style=dashed];
    "h1_SUB-WALK_R" -> "?v_R" [label="3", color="#d62728", style=dashed];
    "h1_SUB-WALK_R" -> "0_R" [label="4", color="#d62728", style=dashed];
    "h1_SUB-WALK_R" -> "z_R" [label="5", color="#d62728", style=dashed];
    "h2_SUB-WALK_R" [shape=octagon, label="?SUB-WALK", color="#d62728", fontcolor="#d62728"];
    "h2_SUB-WALK_R" -> "(?SHORTFALL ?x ?y)_R" [label="1", color="#d62728", style=dashed];
    "h2_SUB-WALK_R" -> "?v_R" [label="2", color="#d62728", style=dashed];
    "h2_SUB-WALK_R" -> "?u_R" [label="3", color="#d62728", style=dashed];
    "h2_SUB-WALK_R" -> "0_R" [label="4", color="#d62728", style=dashed];
    "h2_SUB-WALK_R" -> "z_R" [label="5", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-answer`

<!-- rule std.math.positional sub-answer -->
```lisp
(rule sub-answer (?DONE ?DIFFERENCE ?DIFF-OF)
  :match  (?DONE (?DIFFERENCE ?x ?y) ?d)
  :assert (?DIFF-OF ?x ?y ?d z)
  :why    "{?x} - {?y} = {?d}, and nothing is short")
```

```dot
digraph rule_sub_answer_lhs_rhs {
  rankdir=TB;
  label="sub-answer"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?DIFFERENCE ?x ?y)_L" [label="(?DIFFERENCE ?x ?y)", shape=rectangle];
    "?d_L" [label="?d", shape=diamond];
    "(?DIFFERENCE ?x ?y)_L" -> "?d_L" [label="?DONE", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "?y_R" [label="?y", shape=diamond];
    "?d_R" [label="?d", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "h1_DIFF-OF_R" [shape=octagon, label="?DIFF-OF", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_DIFF-OF_R" -> "?x_R" [label="1", color="#7f7f7f", style=dashed];
    "h1_DIFF-OF_R" -> "?y_R" [label="2", color="#7f7f7f", style=dashed];
    "h1_DIFF-OF_R" -> "?d_R" [label="3", color="#7f7f7f", style=dashed];
    "h1_DIFF-OF_R" -> "z_R" [label="4", color="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

#### `sub-answer-short`

<!-- rule std.math.positional sub-answer-short -->
```lisp
(rule sub-answer-short (?DONE ?SHORTFALL ?DIFF-OF)
  :match  (?DONE (?SHORTFALL ?x ?y) ?r)
  :assert (?DIFF-OF ?x ?y z ?r)
  :why    "{?x} is short of {?y} by {?r}: the difference is zero")
```

```dot
digraph rule_sub_answer_short_lhs_rhs {
  rankdir=TB;
  label="sub-answer-short"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?SHORTFALL ?x ?y)_L" [label="(?SHORTFALL ?x ?y)", shape=rectangle];
    "?r_L" [label="?r", shape=diamond];
    "(?SHORTFALL ?x ?y)_L" -> "?r_L" [label="?DONE", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "?y_R" [label="?y", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?r_R" [label="?r", shape=diamond];
    "h1_DIFF-OF_R" [shape=octagon, label="?DIFF-OF", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_DIFF-OF_R" -> "?x_R" [label="1", color="#7f7f7f", style=dashed];
    "h1_DIFF-OF_R" -> "?y_R" [label="2", color="#7f7f7f", style=dashed];
    "h1_DIFF-OF_R" -> "z_R" [label="3", color="#7f7f7f", style=dashed];
    "h1_DIFF-OF_R" -> "?r_R" [label="4", color="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

### Comparison

One walk over the comparator's table, which carries the order of the digits below and lets a higher differing
digit overrule it. It starts at `eq`.

#### `cmp-column`

<!-- rule std.math.positional cmp-column -->
```lisp
(rule cmp-column (?CMP-WALK ?N ?CL)
  :match  (and (?CMP-WALK ?k (?N ?p ?a) (?N ?q ?c) ?o)
               (?CL ?o ?a ?c ?t))
  :assert (?CMP-WALK ?k ?p ?q ?t)
  :why    "the order below is {?o}; {?a} against {?c} above it makes it {?t}")
```

```dot
digraph rule_cmp_column_lhs_rhs {
  rankdir=TB;
  label="cmp-column"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?N ?p ?a)_L" [label="(?N ?p ?a)", shape=rectangle];
    "(?N ?q ?c)_L" [label="(?N ?q ?c)", shape=rectangle];
    "?o_L" [label="?o", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?t_L" [label="?t", shape=diamond];
    "h1_CMP-WALK_L" [shape=octagon, label="?CMP-WALK", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_CMP-WALK_L" -> "?k_L" [label="1", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "(?N ?p ?a)_L" [label="2", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "(?N ?q ?c)_L" [label="3", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "?o_L" [label="4", color="#7f7f7f"];
    "h2_CL_L" [shape=octagon, label="?CL", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_CL_L" -> "?o_L" [label="1", color="#ff7f0e"];
    "h2_CL_L" -> "?a_L" [label="2", color="#ff7f0e"];
    "h2_CL_L" -> "?c_L" [label="3", color="#ff7f0e"];
    "h2_CL_L" -> "?t_L" [label="4", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?p_R" [label="?p", shape=diamond];
    "?q_R" [label="?q", shape=diamond];
    "?t_R" [label="?t", shape=diamond];
    "h3_CMP-WALK_R" [shape=octagon, label="?CMP-WALK", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_CMP-WALK_R" -> "?k_R" [label="1", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "?p_R" [label="2", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "?q_R" [label="3", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "?t_R" [label="4", color="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

#### `cmp-column-first`

<!-- rule std.math.positional cmp-column-first -->
```lisp
(rule cmp-column-first (?CMP-WALK ?N ?CL)
  :match  (and (?CMP-WALK ?k (?N ?p ?a) z ?o)
               (?CL ?o ?a 0 ?t))
  :assert (?CMP-WALK ?k ?p z ?t)
  :why    "the second number has run out: the order below is {?o}, and {?a} against nothing makes it {?t}")
```

```dot
digraph rule_cmp_column_first_lhs_rhs {
  rankdir=TB;
  label="cmp-column-first"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?N ?p ?a)_L" [label="(?N ?p ?a)", shape=rectangle];
    "z_L" [label="z", shape=rectangle];
    "?o_L" [label="?o", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?t_L" [label="?t", shape=diamond];
    "h1_CMP-WALK_L" [shape=octagon, label="?CMP-WALK", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_CMP-WALK_L" -> "?k_L" [label="1", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "(?N ?p ?a)_L" [label="2", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "z_L" [label="3", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "?o_L" [label="4", color="#7f7f7f"];
    "h2_CL_L" [shape=octagon, label="?CL", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_CL_L" -> "?o_L" [label="1", color="#ff7f0e"];
    "h2_CL_L" -> "?a_L" [label="2", color="#ff7f0e"];
    "h2_CL_L" -> "0_L" [label="3", color="#ff7f0e"];
    "h2_CL_L" -> "?t_L" [label="4", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?p_R" [label="?p", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?t_R" [label="?t", shape=diamond];
    "h3_CMP-WALK_R" [shape=octagon, label="?CMP-WALK", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_CMP-WALK_R" -> "?k_R" [label="1", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "?p_R" [label="2", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "z_R" [label="3", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "?t_R" [label="4", color="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

#### `cmp-column-second`

<!-- rule std.math.positional cmp-column-second -->
```lisp
(rule cmp-column-second (?CMP-WALK ?N ?CL)
  :match  (and (?CMP-WALK ?k z (?N ?q ?c) ?o)
               (?CL ?o 0 ?c ?t))
  :assert (?CMP-WALK ?k z ?q ?t)
  :why    "the first number has run out: the order below is {?o}, and nothing against {?c} makes it {?t}")
```

```dot
digraph rule_cmp_column_second_lhs_rhs {
  rankdir=TB;
  label="cmp-column-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "(?N ?q ?c)_L" [label="(?N ?q ?c)", shape=rectangle];
    "?o_L" [label="?o", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?c_L" [label="?c", shape=diamond];
    "?t_L" [label="?t", shape=diamond];
    "h1_CMP-WALK_L" [shape=octagon, label="?CMP-WALK", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_CMP-WALK_L" -> "?k_L" [label="1", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "z_L" [label="2", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "(?N ?q ?c)_L" [label="3", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "?o_L" [label="4", color="#7f7f7f"];
    "h2_CL_L" [shape=octagon, label="?CL", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_CL_L" -> "?o_L" [label="1", color="#ff7f0e"];
    "h2_CL_L" -> "0_L" [label="2", color="#ff7f0e"];
    "h2_CL_L" -> "?c_L" [label="3", color="#ff7f0e"];
    "h2_CL_L" -> "?t_L" [label="4", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "?q_R" [label="?q", shape=diamond];
    "?t_R" [label="?t", shape=diamond];
    "h3_CMP-WALK_R" [shape=octagon, label="?CMP-WALK", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_CMP-WALK_R" -> "?k_R" [label="1", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "z_R" [label="2", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "?q_R" [label="3", color="#7f7f7f", style=dashed];
    "h3_CMP-WALK_R" -> "?t_R" [label="4", color="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

#### `cmp-end`

<!-- rule std.math.positional cmp-end -->
```lisp
(rule cmp-end (?CMP-WALK ?ORDER-IS)
  :match  (?CMP-WALK ?k z z ?o)
  :assert (?ORDER-IS ?k ?o)
  :why    "no column is left: the order is {?o}")
```

```dot
digraph rule_cmp_end_lhs_rhs {
  rankdir=TB;
  label="cmp-end"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "?o_L" [label="?o", shape=diamond];
    "h1_CMP-WALK_L" [shape=octagon, label="?CMP-WALK", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_CMP-WALK_L" -> "?k_L" [label="1", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "z_L" [label="2", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "z_L" [label="3", color="#7f7f7f"];
    "h1_CMP-WALK_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "?k_R" -> "?o_R" [label="?ORDER-IS", color="#ff7f0e", fontcolor="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `cmp-ask`

<!-- rule std.math.positional cmp-ask -->
```lisp
(rule cmp-ask (?CMP ?OPERAND ?CMP-FIRST)
  :match  (?CMP ?x ?y)
  :assert (?OPERAND (?CMP-FIRST ?x ?y) ?x)
  :why    "{?x} compared with {?y} is asked: first, the number {?x} stands for")
```

```dot
digraph rule_cmp_ask_lhs_rhs {
  rankdir=TB;
  label="cmp-ask"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "?x_L" -> "?y_L" [label="?CMP", color="#ff7f0e", fontcolor="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "(?CMP-FIRST ?x ?y)_R" [label="(?CMP-FIRST ?x ?y)", shape=rectangle];
    "?x_R" [label="?x", shape=diamond];
    "(?CMP-FIRST ?x ?y)_R" -> "?x_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `cmp-ask-second`

<!-- rule std.math.positional cmp-ask-second -->
```lisp
(rule cmp-ask-second (?OPERAND-IS ?CMP-FIRST ?OPERAND ?CMP-SECOND)
  :match  (?OPERAND-IS (?CMP-FIRST ?x ?y) ?u)
  :assert (?OPERAND (?CMP-SECOND ?x ?y ?u) ?y)
  :why    "the first operand is {?u}: now the number {?y} stands for")
```

```dot
digraph rule_cmp_ask_second_lhs_rhs {
  rankdir=TB;
  label="cmp-ask-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?CMP-FIRST ?x ?y)_L" [label="(?CMP-FIRST ?x ?y)", shape=rectangle];
    "?u_L" [label="?u", shape=diamond];
    "(?CMP-FIRST ?x ?y)_L" -> "?u_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?CMP-SECOND ?x ?y ?u)_R" [label="(?CMP-SECOND ?x ?y ?u)", shape=rectangle];
    "?y_R" [label="?y", shape=diamond];
    "(?CMP-SECOND ?x ?y ?u)_R" -> "?y_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `cmp-start`

<!-- rule std.math.positional cmp-start -->
```lisp
(rule cmp-start (?OPERAND-IS ?CMP-SECOND ?CMP-WALK ?ORDER)
  :match  (?OPERAND-IS (?CMP-SECOND ?x ?y ?u) ?v)
  :assert (?CMP-WALK (?ORDER ?x ?y) ?u ?v eq)
  :why    "compare {?u} with {?v} from the units: equal so far")
```

```dot
digraph rule_cmp_start_lhs_rhs {
  rankdir=TB;
  label="cmp-start"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?CMP-SECOND ?x ?y ?u)_L" [label="(?CMP-SECOND ?x ?y ?u)", shape=rectangle];
    "?v_L" [label="?v", shape=diamond];
    "(?CMP-SECOND ?x ?y ?u)_L" -> "?v_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?ORDER ?x ?y)_R" [label="(?ORDER ?x ?y)", shape=rectangle];
    "?u_R" [label="?u", shape=diamond];
    "?v_R" [label="?v", shape=diamond];
    "eq_R" [label="eq", shape=rectangle];
    "h1_CMP-WALK_R" [shape=octagon, label="?CMP-WALK", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_CMP-WALK_R" -> "(?ORDER ?x ?y)_R" [label="1", color="#7f7f7f", style=dashed];
    "h1_CMP-WALK_R" -> "?u_R" [label="2", color="#7f7f7f", style=dashed];
    "h1_CMP-WALK_R" -> "?v_R" [label="3", color="#7f7f7f", style=dashed];
    "h1_CMP-WALK_R" -> "eq_R" [label="4", color="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

#### `cmp-answer`

<!-- rule std.math.positional cmp-answer -->
```lisp
(rule cmp-answer (?ORDER-IS ?ORDER ?CMP-OF)
  :match  (?ORDER-IS (?ORDER ?x ?y) ?o)
  :assert (?CMP-OF ?x ?y ?o)
  :why    "{?x} against {?y}: {?o}")
```

```dot
digraph rule_cmp_answer_lhs_rhs {
  rankdir=TB;
  label="cmp-answer"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?ORDER ?x ?y)_L" [label="(?ORDER ?x ?y)", shape=rectangle];
    "?o_L" [label="?o", shape=diamond];
    "(?ORDER ?x ?y)_L" -> "?o_L" [label="?ORDER-IS", color="#ff7f0e", fontcolor="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "?y_R" [label="?y", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h1_CMP-OF_R" [shape=octagon, label="?CMP-OF", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_CMP-OF_R" -> "?x_R" [label="1", color="#ff7f0e", style=dashed];
    "h1_CMP-OF_R" -> "?y_R" [label="2", color="#ff7f0e", style=dashed];
    "h1_CMP-OF_R" -> "?o_R" [label="3", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

### Multiplication: the call

What a base shares of a product: the multiplier's digits turned round so that the top one comes first
(`top-first`), the sum so far handed from one row to the next, and the request. A row itself is the base's own:
decimal multiplies by a digit, binary adds the multiplicand or nothing.

#### `top-first-step`

<!-- rule std.math.positional top-first-step -->
```lisp
(rule top-first-step (?TOP-FIRST ?N ?B)
  :match  (?TOP-FIRST ?k (?N ?p ?a) ?acc)
  :assert (?TOP-FIRST ?k ?p (?B ?a ?acc))
  :why    "the digit {?a} goes in front of {?acc}")
```

```dot
digraph rule_top_first_step_lhs_rhs {
  rankdir=TB;
  label="top-first-step"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "(?N ?p ?a)_L" [label="(?N ?p ?a)", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_TOP-FIRST_L" [shape=octagon, label="?TOP-FIRST", color="#e377c2", fontcolor="#e377c2"];
    "h1_TOP-FIRST_L" -> "?k_L" [label="1", color="#e377c2"];
    "h1_TOP-FIRST_L" -> "(?N ?p ?a)_L" [label="2", color="#e377c2"];
    "h1_TOP-FIRST_L" -> "?acc_L" [label="3", color="#e377c2"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?p_R" [label="?p", shape=diamond];
    "(?B ?a ?acc)_R" [label="(?B ?a ?acc)", shape=rectangle];
    "h2_TOP-FIRST_R" [shape=octagon, label="?TOP-FIRST", color="#e377c2", fontcolor="#e377c2"];
    "h2_TOP-FIRST_R" -> "?k_R" [label="1", color="#e377c2", style=dashed];
    "h2_TOP-FIRST_R" -> "?p_R" [label="2", color="#e377c2", style=dashed];
    "h2_TOP-FIRST_R" -> "(?B ?a ?acc)_R" [label="3", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

#### `top-first-end`

<!-- rule std.math.positional top-first-end -->
```lisp
(rule top-first-end (?TOP-FIRST ?TOP-FIRST-IS)
  :match  (?TOP-FIRST ?k z ?acc)
  :assert (?TOP-FIRST-IS ?k ?acc)
  :why    "turned round: the digits, the top one first, are {?acc}")
```

```dot
digraph rule_top_first_end_lhs_rhs {
  rankdir=TB;
  label="top-first-end"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_TOP-FIRST_L" [shape=octagon, label="?TOP-FIRST", color="#e377c2", fontcolor="#e377c2"];
    "h1_TOP-FIRST_L" -> "?k_L" [label="1", color="#e377c2"];
    "h1_TOP-FIRST_L" -> "z_L" [label="2", color="#e377c2"];
    "h1_TOP-FIRST_L" -> "?acc_L" [label="3", color="#e377c2"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?acc_R" [label="?acc", shape=diamond];
    "?k_R" -> "?acc_R" [label="?TOP-FIRST-IS", color="#2ca02c", fontcolor="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `mul-turn`

<!-- rule std.math.positional mul-turn -->
```lisp
(rule mul-turn (?MUL-CALL ?TOP-FIRST ?MUL-DIGITS)
  :match  (?MUL-CALL ?k ?u ?v)
  :assert (?TOP-FIRST (?MUL-DIGITS ?k ?u) ?v z)
  :why    "to multiply {?u} by {?v}: first {?v}'s digits, the top one first")
```

```dot
digraph rule_mul_turn_lhs_rhs {
  rankdir=TB;
  label="mul-turn"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "?u_L" [label="?u", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "h1_MUL-CALL_L" [shape=octagon, label="?MUL-CALL", color="#bcbd22", fontcolor="#bcbd22"];
    "h1_MUL-CALL_L" -> "?k_L" [label="1", color="#bcbd22"];
    "h1_MUL-CALL_L" -> "?u_L" [label="2", color="#bcbd22"];
    "h1_MUL-CALL_L" -> "?v_L" [label="3", color="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?MUL-DIGITS ?k ?u)_R" [label="(?MUL-DIGITS ?k ?u)", shape=rectangle];
    "?v_R" [label="?v", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "h2_TOP-FIRST_R" [shape=octagon, label="?TOP-FIRST", color="#e377c2", fontcolor="#e377c2"];
    "h2_TOP-FIRST_R" -> "(?MUL-DIGITS ?k ?u)_R" [label="1", color="#e377c2", style=dashed];
    "h2_TOP-FIRST_R" -> "?v_R" [label="2", color="#e377c2", style=dashed];
    "h2_TOP-FIRST_R" -> "z_R" [label="3", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

#### `mul-begin`

<!-- rule std.math.positional mul-begin -->
```lisp
(rule mul-begin (?TOP-FIRST-IS ?MUL-DIGITS ?MUL-STEP)
  :match  (?TOP-FIRST-IS (?MUL-DIGITS ?k ?u) ?t)
  :assert (?MUL-STEP ?k ?u ?t z)
  :why    "nothing is summed yet")
```

```dot
digraph rule_mul_begin_lhs_rhs {
  rankdir=TB;
  label="mul-begin"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?MUL-DIGITS ?k ?u)_L" [label="(?MUL-DIGITS ?k ?u)", shape=rectangle];
    "?t_L" [label="?t", shape=diamond];
    "(?MUL-DIGITS ?k ?u)_L" -> "?t_L" [label="?TOP-FIRST-IS", color="#2ca02c", fontcolor="#2ca02c"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?u_R" [label="?u", shape=diamond];
    "?t_R" [label="?t", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "h1_MUL-STEP_R" [shape=octagon, label="?MUL-STEP", color="#17becf", fontcolor="#17becf"];
    "h1_MUL-STEP_R" -> "?k_R" [label="1", color="#17becf", style=dashed];
    "h1_MUL-STEP_R" -> "?u_R" [label="2", color="#17becf", style=dashed];
    "h1_MUL-STEP_R" -> "?t_R" [label="3", color="#17becf", style=dashed];
    "h1_MUL-STEP_R" -> "z_R" [label="4", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `mul-next`

<!-- rule std.math.positional mul-next -->
```lisp
(rule mul-next (?DONE ?MUL-SUM ?MUL-STEP)
  :match  (?DONE (?MUL-SUM ?k ?u ?rest) ?s)
  :assert (?MUL-STEP ?k ?u ?rest ?s)
  :why    "the sum so far is {?s}")
```

```dot
digraph rule_mul_next_lhs_rhs {
  rankdir=TB;
  label="mul-next"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?MUL-SUM ?k ?u ?rest)_L" [label="(?MUL-SUM ?k ?u ?rest)", shape=rectangle];
    "?s_L" [label="?s", shape=diamond];
    "(?MUL-SUM ?k ?u ?rest)_L" -> "?s_L" [label="?DONE", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?u_R" [label="?u", shape=diamond];
    "?rest_R" [label="?rest", shape=diamond];
    "?s_R" [label="?s", shape=diamond];
    "h1_MUL-STEP_R" [shape=octagon, label="?MUL-STEP", color="#17becf", fontcolor="#17becf"];
    "h1_MUL-STEP_R" -> "?k_R" [label="1", color="#17becf", style=dashed];
    "h1_MUL-STEP_R" -> "?u_R" [label="2", color="#17becf", style=dashed];
    "h1_MUL-STEP_R" -> "?rest_R" [label="3", color="#17becf", style=dashed];
    "h1_MUL-STEP_R" -> "?s_R" [label="4", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `mul-end`

<!-- rule std.math.positional mul-end -->
```lisp
(rule mul-end (?MUL-STEP ?DONE)
  :match  (?MUL-STEP ?k ?u z ?acc)
  :assert (?DONE ?k ?acc)
  :why    "no digit of the multiplier is left: the product is {?acc}")
```

```dot
digraph rule_mul_end_lhs_rhs {
  rankdir=TB;
  label="mul-end"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "?u_L" [label="?u", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "?acc_L" [label="?acc", shape=diamond];
    "h1_MUL-STEP_L" [shape=octagon, label="?MUL-STEP", color="#17becf", fontcolor="#17becf"];
    "h1_MUL-STEP_L" -> "?k_L" [label="1", color="#17becf"];
    "h1_MUL-STEP_L" -> "?u_L" [label="2", color="#17becf"];
    "h1_MUL-STEP_L" -> "z_L" [label="3", color="#17becf"];
    "h1_MUL-STEP_L" -> "?acc_L" [label="4", color="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?acc_R" [label="?acc", shape=diamond];
    "?k_R" -> "?acc_R" [label="?DONE", color="#9467bd", fontcolor="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `mul-ask`

<!-- rule std.math.positional mul-ask -->
```lisp
(rule mul-ask (?MUL ?OPERAND ?MUL-FIRST)
  :match  (?MUL ?x ?y)
  :assert (?OPERAND (?MUL-FIRST ?x ?y) ?x)
  :why    "{?x} x {?y} is asked: first, the number {?x} stands for")
```

```dot
digraph rule_mul_ask_lhs_rhs {
  rankdir=TB;
  label="mul-ask"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "?x_L" -> "?y_L" [label="?MUL", color="#8c564b", fontcolor="#8c564b"];
  }
  subgraph cluster_rhs { label="assert";
    "(?MUL-FIRST ?x ?y)_R" [label="(?MUL-FIRST ?x ?y)", shape=rectangle];
    "?x_R" [label="?x", shape=diamond];
    "(?MUL-FIRST ?x ?y)_R" -> "?x_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `mul-ask-second`

<!-- rule std.math.positional mul-ask-second -->
```lisp
(rule mul-ask-second (?OPERAND-IS ?MUL-FIRST ?OPERAND ?MUL-SECOND)
  :match  (?OPERAND-IS (?MUL-FIRST ?x ?y) ?u)
  :assert (?OPERAND (?MUL-SECOND ?x ?y ?u) ?y)
  :why    "the first operand is {?u}: now the number {?y} stands for")
```

```dot
digraph rule_mul_ask_second_lhs_rhs {
  rankdir=TB;
  label="mul-ask-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?MUL-FIRST ?x ?y)_L" [label="(?MUL-FIRST ?x ?y)", shape=rectangle];
    "?u_L" [label="?u", shape=diamond];
    "(?MUL-FIRST ?x ?y)_L" -> "?u_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?MUL-SECOND ?x ?y ?u)_R" [label="(?MUL-SECOND ?x ?y ?u)", shape=rectangle];
    "?y_R" [label="?y", shape=diamond];
    "(?MUL-SECOND ?x ?y ?u)_R" -> "?y_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `mul-start`

<!-- rule std.math.positional mul-start -->
```lisp
(rule mul-start (?OPERAND-IS ?MUL-SECOND ?MUL-CALL ?PRODUCT)
  :match  (?OPERAND-IS (?MUL-SECOND ?x ?y ?u) ?v)
  :assert (?MUL-CALL (?PRODUCT ?x ?y) ?u ?v)
  :why    "multiply {?u} by {?v}")
```

```dot
digraph rule_mul_start_lhs_rhs {
  rankdir=TB;
  label="mul-start"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?MUL-SECOND ?x ?y ?u)_L" [label="(?MUL-SECOND ?x ?y ?u)", shape=rectangle];
    "?v_L" [label="?v", shape=diamond];
    "(?MUL-SECOND ?x ?y ?u)_L" -> "?v_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?PRODUCT ?x ?y)_R" [label="(?PRODUCT ?x ?y)", shape=rectangle];
    "?u_R" [label="?u", shape=diamond];
    "?v_R" [label="?v", shape=diamond];
    "h1_MUL-CALL_R" [shape=octagon, label="?MUL-CALL", color="#bcbd22", fontcolor="#bcbd22"];
    "h1_MUL-CALL_R" -> "(?PRODUCT ?x ?y)_R" [label="1", color="#bcbd22", style=dashed];
    "h1_MUL-CALL_R" -> "?u_R" [label="2", color="#bcbd22", style=dashed];
    "h1_MUL-CALL_R" -> "?v_R" [label="3", color="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

#### `mul-answer`

<!-- rule std.math.positional mul-answer -->
```lisp
(rule mul-answer (?DONE ?PRODUCT ?PRODUCT-OF)
  :match  (?DONE (?PRODUCT ?x ?y) ?p)
  :assert (?PRODUCT-OF ?x ?y ?p)
  :why    "{?x} x {?y} = {?p}")
```

```dot
digraph rule_mul_answer_lhs_rhs {
  rankdir=TB;
  label="mul-answer"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?PRODUCT ?x ?y)_L" [label="(?PRODUCT ?x ?y)", shape=rectangle];
    "?p_L" [label="?p", shape=diamond];
    "(?PRODUCT ?x ?y)_L" -> "?p_L" [label="?DONE", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "?y_R" [label="?y", shape=diamond];
    "?p_R" [label="?p", shape=diamond];
    "h1_PRODUCT-OF_R" [shape=octagon, label="?PRODUCT-OF", color="#e377c2", fontcolor="#e377c2"];
    "h1_PRODUCT-OF_R" -> "?x_R" [label="1", color="#e377c2", style=dashed];
    "h1_PRODUCT-OF_R" -> "?y_R" [label="2", color="#e377c2", style=dashed];
    "h1_PRODUCT-OF_R" -> "?p_R" [label="3", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

### Division: the call

The start and the end of a long division, and the request. The steps between, bringing a digit down and finding a
digit of the quotient, are the base's own.

#### `div-begin`

<!-- rule std.math.positional div-begin -->
```lisp
(rule div-begin (?TOP-FIRST-IS ?DIV-DIGITS ?DIV-STEP)
  :match  (?TOP-FIRST-IS (?DIV-DIGITS ?k ?ms) ?t)
  :assert (?DIV-STEP ?k ?ms ?t z z)
  :why    "start at the top digit: nothing is left over yet, and the quotient is zero")
```

```dot
digraph rule_div_begin_lhs_rhs {
  rankdir=TB;
  label="div-begin"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?DIV-DIGITS ?k ?ms)_L" [label="(?DIV-DIGITS ?k ?ms)", shape=rectangle];
    "?t_L" [label="?t", shape=diamond];
    "(?DIV-DIGITS ?k ?ms)_L" -> "?t_L" [label="?TOP-FIRST-IS", color="#2ca02c", fontcolor="#2ca02c"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?ms_R" [label="?ms", shape=diamond];
    "?t_R" [label="?t", shape=diamond];
    "z_R" [label="z", shape=rectangle];
    "h1_DIV-STEP_R" [shape=octagon, label="?DIV-STEP", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_DIV-STEP_R" -> "?k_R" [label="1", color="#ff7f0e", style=dashed];
    "h1_DIV-STEP_R" -> "?ms_R" [label="2", color="#ff7f0e", style=dashed];
    "h1_DIV-STEP_R" -> "?t_R" [label="3", color="#ff7f0e", style=dashed];
    "h1_DIV-STEP_R" -> "z_R" [label="4", color="#ff7f0e", style=dashed];
    "h1_DIV-STEP_R" -> "z_R" [label="5", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `div-end`

<!-- rule std.math.positional div-end -->
```lisp
(rule div-end (?DIV-STEP ?DIV-DONE)
  :match  (?DIV-STEP ?k ?ms z ?t ?w)
  :assert (?DIV-DONE ?k ?w ?t)
  :why    "no digit is left: the quotient is {?w} and the remainder {?t}")
```

```dot
digraph rule_div_end_lhs_rhs {
  rankdir=TB;
  label="div-end"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "?ms_L" [label="?ms", shape=diamond];
    "z_L" [label="z", shape=rectangle];
    "?t_L" [label="?t", shape=diamond];
    "?w_L" [label="?w", shape=diamond];
    "h1_DIV-STEP_L" [shape=octagon, label="?DIV-STEP", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_DIV-STEP_L" -> "?k_L" [label="1", color="#ff7f0e"];
    "h1_DIV-STEP_L" -> "?ms_L" [label="2", color="#ff7f0e"];
    "h1_DIV-STEP_L" -> "z_L" [label="3", color="#ff7f0e"];
    "h1_DIV-STEP_L" -> "?t_L" [label="4", color="#ff7f0e"];
    "h1_DIV-STEP_L" -> "?w_L" [label="5", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "?w_R" [label="?w", shape=diamond];
    "?t_R" [label="?t", shape=diamond];
    "h2_DIV-DONE_R" [shape=octagon, label="?DIV-DONE", color="#2ca02c", fontcolor="#2ca02c"];
    "h2_DIV-DONE_R" -> "?k_R" [label="1", color="#2ca02c", style=dashed];
    "h2_DIV-DONE_R" -> "?w_R" [label="2", color="#2ca02c", style=dashed];
    "h2_DIV-DONE_R" -> "?t_R" [label="3", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `div-ask`

<!-- rule std.math.positional div-ask -->
```lisp
(rule div-ask (?DIV ?OPERAND ?DIV-FIRST)
  :match  (?DIV ?x ?y)
  :assert (?OPERAND (?DIV-FIRST ?x ?y) ?x)
  :why    "{?x} divided by {?y} is asked: first, the number {?x} stands for")
```

```dot
digraph rule_div_ask_lhs_rhs {
  rankdir=TB;
  label="div-ask"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "?x_L" -> "?y_L" [label="?DIV", color="#ff7f0e", fontcolor="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "(?DIV-FIRST ?x ?y)_R" [label="(?DIV-FIRST ?x ?y)", shape=rectangle];
    "?x_R" [label="?x", shape=diamond];
    "(?DIV-FIRST ?x ?y)_R" -> "?x_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `div-ask-second`

<!-- rule std.math.positional div-ask-second -->
```lisp
(rule div-ask-second (?OPERAND-IS ?DIV-FIRST ?OPERAND ?DIV-SECOND)
  :match  (?OPERAND-IS (?DIV-FIRST ?x ?y) ?u)
  :assert (?OPERAND (?DIV-SECOND ?x ?y ?u) ?y)
  :why    "the first operand is {?u}: now the number {?y} stands for")
```

```dot
digraph rule_div_ask_second_lhs_rhs {
  rankdir=TB;
  label="div-ask-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?DIV-FIRST ?x ?y)_L" [label="(?DIV-FIRST ?x ?y)", shape=rectangle];
    "?u_L" [label="?u", shape=diamond];
    "(?DIV-FIRST ?x ?y)_L" -> "?u_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?DIV-SECOND ?x ?y ?u)_R" [label="(?DIV-SECOND ?x ?y ?u)", shape=rectangle];
    "?y_R" [label="?y", shape=diamond];
    "(?DIV-SECOND ?x ?y ?u)_R" -> "?y_R" [label="?OPERAND", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `div-start`

<!-- rule std.math.positional div-start -->
```lisp
(rule div-start (?OPERAND-IS ?DIV-SECOND ?DIV-CALL ?QUOTIENT)
  :match  (?OPERAND-IS (?DIV-SECOND ?x ?y ?u) ?v)
  :assert (?DIV-CALL (?QUOTIENT ?x ?y) ?u ?v)
  :why    "divide {?u} by {?v}")
```

```dot
digraph rule_div_start_lhs_rhs {
  rankdir=TB;
  label="div-start"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?DIV-SECOND ?x ?y ?u)_L" [label="(?DIV-SECOND ?x ?y ?u)", shape=rectangle];
    "?v_L" [label="?v", shape=diamond];
    "(?DIV-SECOND ?x ?y ?u)_L" -> "?v_L" [label="?OPERAND-IS", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?QUOTIENT ?x ?y)_R" [label="(?QUOTIENT ?x ?y)", shape=rectangle];
    "?u_R" [label="?u", shape=diamond];
    "?v_R" [label="?v", shape=diamond];
    "h1_DIV-CALL_R" [shape=octagon, label="?DIV-CALL", color="#e377c2", fontcolor="#e377c2"];
    "h1_DIV-CALL_R" -> "(?QUOTIENT ?x ?y)_R" [label="1", color="#e377c2", style=dashed];
    "h1_DIV-CALL_R" -> "?u_R" [label="2", color="#e377c2", style=dashed];
    "h1_DIV-CALL_R" -> "?v_R" [label="3", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

#### `div-answer`

<!-- rule std.math.positional div-answer -->
```lisp
(rule div-answer (?DIV-DONE ?QUOTIENT ?QUOT-OF)
  :match  (?DIV-DONE (?QUOTIENT ?x ?y) ?q ?r)
  :assert (?QUOT-OF ?x ?y ?q ?r)
  :why    "{?x} divided by {?y} = {?q}, remainder {?r}")
```

```dot
digraph rule_div_answer_lhs_rhs {
  rankdir=TB;
  label="div-answer"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?QUOTIENT ?x ?y)_L" [label="(?QUOTIENT ?x ?y)", shape=rectangle];
    "?q_L" [label="?q", shape=diamond];
    "?r_L" [label="?r", shape=diamond];
    "h1_DIV-DONE_L" [shape=octagon, label="?DIV-DONE", color="#2ca02c", fontcolor="#2ca02c"];
    "h1_DIV-DONE_L" -> "(?QUOTIENT ?x ?y)_L" [label="1", color="#2ca02c"];
    "h1_DIV-DONE_L" -> "?q_L" [label="2", color="#2ca02c"];
    "h1_DIV-DONE_L" -> "?r_L" [label="3", color="#2ca02c"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "?y_R" [label="?y", shape=diamond];
    "?q_R" [label="?q", shape=diamond];
    "?r_R" [label="?r", shape=diamond];
    "h2_QUOT-OF_R" [shape=octagon, label="?QUOT-OF", color="#9467bd", fontcolor="#9467bd"];
    "h2_QUOT-OF_R" -> "?x_R" [label="1", color="#9467bd", style=dashed];
    "h2_QUOT-OF_R" -> "?y_R" [label="2", color="#9467bd", style=dashed];
    "h2_QUOT-OF_R" -> "?q_R" [label="3", color="#9467bd", style=dashed];
    "h2_QUOT-OF_R" -> "?r_R" [label="4", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

### Division by zero

The divisor times one is zero: the flag, at the request and at the call.

#### `div-by-zero`

<!-- rule std.math.positional div-by-zero -->
```lisp
(rule div-by-zero (?DONE ?DIV-M1 ?QUOTIENT ?DIV)
  :match  (?DONE (?DIV-M1 (?QUOTIENT ?x ?y) ?u ?v) z)
  :assert (division-by-zero (?DIV ?x ?y))
  :why    "the divisor of {?x} divided by {?y} is zero")
```

```dot
digraph rule_div_by_zero_lhs_rhs {
  rankdir=TB;
  label="div-by-zero"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?DIV-M1 (?QUOTIENT ?x ?y) ?u ?v)_L" [label="(?DIV-M1 (?QUOTIENT ?x ?y) ?u ?v)", shape=rectangle];
    "z_L" [label="z", shape=rectangle];
    "(?DIV-M1 (?QUOTIENT ?x ?y) ?u ?v)_L" -> "z_L" [label="?DONE", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "(?DIV ?x ?y)_R" [label="(?DIV ?x ?y)", shape=rectangle];
    "h1_division-by-zero_R" [shape=octagon, label="division-by-zero", color="#e377c2", fontcolor="#e377c2"];
    "h1_division-by-zero_R" -> "(?DIV ?x ?y)_R" [label="1", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

#### `div-call-by-zero`

<!-- rule std.math.positional div-call-by-zero -->
```lisp
(rule div-call-by-zero (?DONE ?DIV-M1 ?DIV-CALL)
  :match  (?DONE (?DIV-M1 ?k ?u ?v) z)
  :assert (division-by-zero (?DIV-CALL ?k ?u ?v))
  :why    "the divisor of {?u} divided by {?v}, wanted for {?k}, is zero")
```

```dot
digraph rule_div_call_by_zero_lhs_rhs {
  rankdir=TB;
  label="div-call-by-zero"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?DIV-M1 ?k ?u ?v)_L" [label="(?DIV-M1 ?k ?u ?v)", shape=rectangle];
    "z_L" [label="z", shape=rectangle];
    "(?DIV-M1 ?k ?u ?v)_L" -> "z_L" [label="?DONE", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "(?DIV-CALL ?k ?u ?v)_R" [label="(?DIV-CALL ?k ?u ?v)", shape=rectangle];
    "h1_division-by-zero_R" [shape=octagon, label="division-by-zero", color="#e377c2", fontcolor="#e377c2"];
    "h1_division-by-zero_R" -> "(?DIV-CALL ?k ?u ?v)_R" [label="1", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

### An operand's faults, and a `let`'s

A digit where a number goes, and a `let` that cannot be read, each found by one rule. An operand's fault is first
a fact about the operand's tag, `bad-operand`; the ten `bad-…-first` and `bad-…-second` rules carry it to the
request it belongs to, `bad-request-malformed` and `bad-request-undeclared` turn it into the flag, and the last two
rules make each flag a contradiction.

#### `operand-digit`

<!-- rule std.math.positional operand-digit -->
```lisp
(rule operand-digit (?OPERAND ?NZ ?BAD-OPERAND ?NOT-A-NUMBER ?N)
  :match  (and (?OPERAND ?k ?x) (?NZ ?x))
  :assert (?BAD-OPERAND ?k (?NOT-A-NUMBER ?x))
  :why    "the digit {?x} is not a number: the number is ({?N} z {?x})")
```

```dot
digraph rule_operand_digit_lhs_rhs {
  rankdir=TB;
  label="operand-digit"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "?x_L" [label="?x", shape=diamond];
    "?k_L" -> "?x_L" [label="?OPERAND", color="#d62728", fontcolor="#d62728"];
    "h1_NZ_L" [shape=octagon, label="?NZ", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_NZ_L" -> "?x_L" [label="1", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "(?NOT-A-NUMBER ?x)_R" [label="(?NOT-A-NUMBER ?x)", shape=rectangle];
    "?k_R" -> "(?NOT-A-NUMBER ?x)_R" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `operand-digit-zero`

<!-- rule std.math.positional operand-digit-zero -->
```lisp
(rule operand-digit-zero (?OPERAND ?BAD-OPERAND ?NOT-A-NUMBER)
  :match  (?OPERAND ?k 0)
  :assert (?BAD-OPERAND ?k (?NOT-A-NUMBER 0))
  :why    "the digit 0 is not a number: zero is z")
```

```dot
digraph rule_operand_digit_zero_lhs_rhs {
  rankdir=TB;
  label="operand-digit-zero"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?k_L" [label="?k", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?k_L" -> "0_L" [label="?OPERAND", color="#d62728", fontcolor="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?k_R" [label="?k", shape=diamond];
    "(?NOT-A-NUMBER 0)_R" [label="(?NOT-A-NUMBER 0)", shape=rectangle];
    "?k_R" -> "(?NOT-A-NUMBER 0)_R" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `let-twice`

<!-- rule std.math.positional let-twice -->
```lisp
(rule let-twice (?LET ?MALFORMED)
  :match  (and (?LET ?x ?v) (?LET ?x ?w) (neq ?v ?w))
  :assert (?MALFORMED (?LET ?x ?v))
  :why    "{?x} is named twice: {?v}, and {?w}")
```

```dot
digraph rule_let_twice_lhs_rhs {
  rankdir=TB;
  label="let-twice"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?w_L" [label="?w", shape=diamond];
    "?x_L" -> "?v_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
    "?x_L" -> "?w_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
    "?v_L" -> "?w_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
  }
  subgraph cluster_rhs { label="assert";
    "(?LET ?x ?v)_R" [label="(?LET ?x ?v)", shape=rectangle];
    "h1_MALFORMED_R" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_MALFORMED_R" -> "(?LET ?x ?v)_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `let-zero`

<!-- rule std.math.positional let-zero -->
```lisp
(rule let-zero (?LET ?MALFORMED)
  :match  (?LET z ?v)
  :assert (?MALFORMED (?LET z ?v))
  :why    "z is zero, and cannot name another number")
```

```dot
digraph rule_let_zero_lhs_rhs {
  rankdir=TB;
  label="let-zero"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "z_L" [label="z", shape=rectangle];
    "?v_L" [label="?v", shape=diamond];
    "z_L" -> "?v_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "(?LET z ?v)_R" [label="(?LET z ?v)", shape=rectangle];
    "h1_MALFORMED_R" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_MALFORMED_R" -> "(?LET z ?v)_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `let-digit`

<!-- rule std.math.positional let-digit -->
```lisp
(rule let-digit (?LET ?NZ ?MALFORMED)
  :match  (and (?LET ?x ?v) (?NZ ?x))
  :assert (?MALFORMED (?LET ?x ?v))
  :why    "{?x} is a digit, and cannot name a number")
```

```dot
digraph rule_let_digit_lhs_rhs {
  rankdir=TB;
  label="let-digit"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?x_L" -> "?v_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_NZ_L" [shape=octagon, label="?NZ", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_NZ_L" -> "?x_L" [label="1", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "(?LET ?x ?v)_R" [label="(?LET ?x ?v)", shape=rectangle];
    "h2_MALFORMED_R" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_MALFORMED_R" -> "(?LET ?x ?v)_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `let-digit-zero`

<!-- rule std.math.positional let-digit-zero -->
```lisp
(rule let-digit-zero (?LET ?MALFORMED)
  :match  (?LET 0 ?v)
  :assert (?MALFORMED (?LET 0 ?v))
  :why    "0 is a digit, and cannot name a number")
```

```dot
digraph rule_let_digit_zero_lhs_rhs {
  rankdir=TB;
  label="let-digit-zero"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "0_L" [label="0", shape=rectangle];
    "?v_L" [label="?v", shape=diamond];
    "0_L" -> "?v_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "(?LET 0 ?v)_R" [label="(?LET 0 ?v)", shape=rectangle];
    "h1_MALFORMED_R" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_MALFORMED_R" -> "(?LET 0 ?v)_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `let-to-name`

<!-- rule std.math.positional let-to-name -->
```lisp
(rule let-to-name (?LET ?NAMED ?MALFORMED)
  :match  (and (?LET ?x ?v) (?NAMED ?v))
  :assert (?MALFORMED (?LET ?x ?v))
  :why    "{?v} is a named constant: a let names a number, not another name")
```

```dot
digraph rule_let_to_name_lhs_rhs {
  rankdir=TB;
  label="let-to-name"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?x_L" -> "?v_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_NAMED_L" [shape=octagon, label="?NAMED", color="#bcbd22", fontcolor="#bcbd22"];
    "h1_NAMED_L" -> "?v_L" [label="1", color="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "(?LET ?x ?v)_R" [label="(?LET ?x ?v)", shape=rectangle];
    "h2_MALFORMED_R" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_MALFORMED_R" -> "(?LET ?x ?v)_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `let-to-digit`

<!-- rule std.math.positional let-to-digit -->
```lisp
(rule let-to-digit (?LET ?NZ ?MALFORMED ?N)
  :match  (and (?LET ?x ?v) (?NZ ?v))
  :assert (?MALFORMED (?LET ?x ?v))
  :why    "{?v} is a digit, not a number: the number is ({?N} z {?v})")
```

```dot
digraph rule_let_to_digit_lhs_rhs {
  rankdir=TB;
  label="let-to-digit"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?x_L" -> "?v_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_NZ_L" [shape=octagon, label="?NZ", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_NZ_L" -> "?v_L" [label="1", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "(?LET ?x ?v)_R" [label="(?LET ?x ?v)", shape=rectangle];
    "h2_MALFORMED_R" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_MALFORMED_R" -> "(?LET ?x ?v)_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `let-to-digit-zero`

<!-- rule std.math.positional let-to-digit-zero -->
```lisp
(rule let-to-digit-zero (?LET ?MALFORMED)
  :match  (?LET ?x 0)
  :assert (?MALFORMED (?LET ?x 0))
  :why    "0 is a digit, not a number: zero is z")
```

```dot
digraph rule_let_to_digit_zero_lhs_rhs {
  rankdir=TB;
  label="let-to-digit-zero"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?x_L" -> "0_L" [label="?LET", color="#7f7f7f", fontcolor="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "(?LET ?x 0)_R" [label="(?LET ?x 0)", shape=rectangle];
    "h1_MALFORMED_R" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_MALFORMED_R" -> "(?LET ?x 0)_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-add-first`

<!-- rule std.math.positional bad-add-first -->
```lisp
(rule bad-add-first (?BAD-OPERAND ?ADD-FIRST ?BAD-REQUEST ?ADD)
  :match  (?BAD-OPERAND (?ADD-FIRST ?x ?y) ?f)
  :assert (?BAD-REQUEST (?ADD ?x ?y) ?f)
  :why    "the first operand of {?x} + {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_add_first_lhs_rhs {
  rankdir=TB;
  label="bad-add-first"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?ADD-FIRST ?x ?y)_L" [label="(?ADD-FIRST ?x ?y)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?ADD-FIRST ?x ?y)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?ADD ?x ?y)_R" [label="(?ADD ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?ADD ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-add-second`

<!-- rule std.math.positional bad-add-second -->
```lisp
(rule bad-add-second (?BAD-OPERAND ?ADD-SECOND ?BAD-REQUEST ?ADD)
  :match  (?BAD-OPERAND (?ADD-SECOND ?x ?y ?u) ?f)
  :assert (?BAD-REQUEST (?ADD ?x ?y) ?f)
  :why    "the second operand of {?x} + {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_add_second_lhs_rhs {
  rankdir=TB;
  label="bad-add-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?ADD-SECOND ?x ?y ?u)_L" [label="(?ADD-SECOND ?x ?y ?u)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?ADD-SECOND ?x ?y ?u)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?ADD ?x ?y)_R" [label="(?ADD ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?ADD ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-sub-first`

<!-- rule std.math.positional bad-sub-first -->
```lisp
(rule bad-sub-first (?BAD-OPERAND ?SUB-FIRST ?BAD-REQUEST ?SUB)
  :match  (?BAD-OPERAND (?SUB-FIRST ?x ?y) ?f)
  :assert (?BAD-REQUEST (?SUB ?x ?y) ?f)
  :why    "the first operand of {?x} - {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_sub_first_lhs_rhs {
  rankdir=TB;
  label="bad-sub-first"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?SUB-FIRST ?x ?y)_L" [label="(?SUB-FIRST ?x ?y)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?SUB-FIRST ?x ?y)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?SUB ?x ?y)_R" [label="(?SUB ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?SUB ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-sub-second`

<!-- rule std.math.positional bad-sub-second -->
```lisp
(rule bad-sub-second (?BAD-OPERAND ?SUB-SECOND ?BAD-REQUEST ?SUB)
  :match  (?BAD-OPERAND (?SUB-SECOND ?x ?y ?u) ?f)
  :assert (?BAD-REQUEST (?SUB ?x ?y) ?f)
  :why    "the second operand of {?x} - {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_sub_second_lhs_rhs {
  rankdir=TB;
  label="bad-sub-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?SUB-SECOND ?x ?y ?u)_L" [label="(?SUB-SECOND ?x ?y ?u)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?SUB-SECOND ?x ?y ?u)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?SUB ?x ?y)_R" [label="(?SUB ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?SUB ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-cmp-first`

<!-- rule std.math.positional bad-cmp-first -->
```lisp
(rule bad-cmp-first (?BAD-OPERAND ?CMP-FIRST ?BAD-REQUEST ?CMP)
  :match  (?BAD-OPERAND (?CMP-FIRST ?x ?y) ?f)
  :assert (?BAD-REQUEST (?CMP ?x ?y) ?f)
  :why    "the first operand of {?x} compared with {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_cmp_first_lhs_rhs {
  rankdir=TB;
  label="bad-cmp-first"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?CMP-FIRST ?x ?y)_L" [label="(?CMP-FIRST ?x ?y)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?CMP-FIRST ?x ?y)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?CMP ?x ?y)_R" [label="(?CMP ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?CMP ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-cmp-second`

<!-- rule std.math.positional bad-cmp-second -->
```lisp
(rule bad-cmp-second (?BAD-OPERAND ?CMP-SECOND ?BAD-REQUEST ?CMP)
  :match  (?BAD-OPERAND (?CMP-SECOND ?x ?y ?u) ?f)
  :assert (?BAD-REQUEST (?CMP ?x ?y) ?f)
  :why    "the second operand of {?x} compared with {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_cmp_second_lhs_rhs {
  rankdir=TB;
  label="bad-cmp-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?CMP-SECOND ?x ?y ?u)_L" [label="(?CMP-SECOND ?x ?y ?u)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?CMP-SECOND ?x ?y ?u)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?CMP ?x ?y)_R" [label="(?CMP ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?CMP ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-mul-first`

<!-- rule std.math.positional bad-mul-first -->
```lisp
(rule bad-mul-first (?BAD-OPERAND ?MUL-FIRST ?BAD-REQUEST ?MUL)
  :match  (?BAD-OPERAND (?MUL-FIRST ?x ?y) ?f)
  :assert (?BAD-REQUEST (?MUL ?x ?y) ?f)
  :why    "the first operand of {?x} x {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_mul_first_lhs_rhs {
  rankdir=TB;
  label="bad-mul-first"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?MUL-FIRST ?x ?y)_L" [label="(?MUL-FIRST ?x ?y)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?MUL-FIRST ?x ?y)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?MUL ?x ?y)_R" [label="(?MUL ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?MUL ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-mul-second`

<!-- rule std.math.positional bad-mul-second -->
```lisp
(rule bad-mul-second (?BAD-OPERAND ?MUL-SECOND ?BAD-REQUEST ?MUL)
  :match  (?BAD-OPERAND (?MUL-SECOND ?x ?y ?u) ?f)
  :assert (?BAD-REQUEST (?MUL ?x ?y) ?f)
  :why    "the second operand of {?x} x {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_mul_second_lhs_rhs {
  rankdir=TB;
  label="bad-mul-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?MUL-SECOND ?x ?y ?u)_L" [label="(?MUL-SECOND ?x ?y ?u)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?MUL-SECOND ?x ?y ?u)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?MUL ?x ?y)_R" [label="(?MUL ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?MUL ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-div-first`

<!-- rule std.math.positional bad-div-first -->
```lisp
(rule bad-div-first (?BAD-OPERAND ?DIV-FIRST ?BAD-REQUEST ?DIV)
  :match  (?BAD-OPERAND (?DIV-FIRST ?x ?y) ?f)
  :assert (?BAD-REQUEST (?DIV ?x ?y) ?f)
  :why    "the first operand of {?x} divided by {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_div_first_lhs_rhs {
  rankdir=TB;
  label="bad-div-first"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?DIV-FIRST ?x ?y)_L" [label="(?DIV-FIRST ?x ?y)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?DIV-FIRST ?x ?y)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?DIV ?x ?y)_R" [label="(?DIV ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?DIV ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-div-second`

<!-- rule std.math.positional bad-div-second -->
```lisp
(rule bad-div-second (?BAD-OPERAND ?DIV-SECOND ?BAD-REQUEST ?DIV)
  :match  (?BAD-OPERAND (?DIV-SECOND ?x ?y ?u) ?f)
  :assert (?BAD-REQUEST (?DIV ?x ?y) ?f)
  :why    "the second operand of {?x} divided by {?y} is at fault: {?f}")
```

```dot
digraph rule_bad_div_second_lhs_rhs {
  rankdir=TB;
  label="bad-div-second"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "(?DIV-SECOND ?x ?y ?u)_L" [label="(?DIV-SECOND ?x ?y ?u)", shape=rectangle];
    "?f_L" [label="?f", shape=diamond];
    "(?DIV-SECOND ?x ?y ?u)_L" -> "?f_L" [label="?BAD-OPERAND", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "(?DIV ?x ?y)_R" [label="(?DIV ?x ?y)", shape=rectangle];
    "?f_R" [label="?f", shape=diamond];
    "(?DIV ?x ?y)_R" -> "?f_R" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-request-malformed`

<!-- rule std.math.positional bad-request-malformed -->
```lisp
(rule bad-request-malformed (?BAD-REQUEST ?NOT-A-NUMBER ?MALFORMED)
  :match  (?BAD-REQUEST ?request (?NOT-A-NUMBER ?thing))
  :assert (?MALFORMED ?request)
  :why    "an operand of {?request} is not a number: it holds {?thing}")
```

```dot
digraph rule_bad_request_malformed_lhs_rhs {
  rankdir=TB;
  label="bad-request-malformed"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?request_L" [label="?request", shape=diamond];
    "(?NOT-A-NUMBER ?thing)_L" [label="(?NOT-A-NUMBER ?thing)", shape=rectangle];
    "?request_L" -> "(?NOT-A-NUMBER ?thing)_L" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?request_R" [label="?request", shape=diamond];
    "h1_MALFORMED_R" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_MALFORMED_R" -> "?request_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `bad-request-undeclared`

<!-- rule std.math.positional bad-request-undeclared -->
```lisp
(rule bad-request-undeclared (?BAD-REQUEST ?NO-LET ?UNDECLARED)
  :match  (?BAD-REQUEST ?request (?NO-LET ?x))
  :assert (?UNDECLARED ?x ?request)
  :why    "{?request} uses the name {?x}, and no let states it")
```

```dot
digraph rule_bad_request_undeclared_lhs_rhs {
  rankdir=TB;
  label="bad-request-undeclared"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?request_L" [label="?request", shape=diamond];
    "(?NO-LET ?x)_L" [label="(?NO-LET ?x)", shape=rectangle];
    "?request_L" -> "(?NO-LET ?x)_L" [label="?BAD-REQUEST", color="#d62728", fontcolor="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "?request_R" [label="?request", shape=diamond];
    "?x_R" -> "?request_R" [label="?UNDECLARED", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `malformed-is-false`

<!-- rule std.math.positional malformed-is-false -->
```lisp
(rule malformed-is-false (?MALFORMED)
  :match  (?MALFORMED ?request)
  :assert (false)
  :why    "{?request} holds something that is not a number: the theory cannot read it")
```

```dot
digraph rule_malformed_is_false_lhs_rhs {
  rankdir=TB;
  label="malformed-is-false"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?request_L" [label="?request", shape=diamond];
    "h1_MALFORMED_L" [shape=octagon, label="?MALFORMED", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_MALFORMED_L" -> "?request_L" [label="1", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "h2_false_R" [shape=octagon, label="false", color="#2ca02c", fontcolor="#2ca02c"];
  }
}
```
<!-- /rule -->

#### `undeclared-is-false`

<!-- rule std.math.positional undeclared-is-false -->
```lisp
(rule undeclared-is-false (?UNDECLARED)
  :match  (?UNDECLARED ?x ?request)
  :assert (false)
  :why    "{?request} uses the name {?x}, which names no number")
```

```dot
digraph rule_undeclared_is_false_lhs_rhs {
  rankdir=TB;
  label="undeclared-is-false"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?x_L" [label="?x", shape=diamond];
    "?request_L" [label="?request", shape=diamond];
    "?x_L" -> "?request_L" [label="?UNDECLARED", color="#d62728", fontcolor="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "h1_false_R" [shape=octagon, label="false", color="#2ca02c", fontcolor="#2ca02c"];
  }
}
```
<!-- /rule -->
