# The Boolean modules' rules

How `std.bool.gates`, `std.bool.props`, `std.bool.circuit` and `std.bool.inputs` work: every rule of the four, as
the module writes it and as the engine draws it. What a program writes to use them is
[the page before](18-boolean-functions.md); how to read a drawing is [the index](README.md#how-a-rule-is-shown).

None of these rules has a parameter, and none writes `absent`, `not` or `or`. A rule's `:why` says what one firing
does, in the words a trace prints.

## `std.bool.gates`

The table `gate2` is facts. Sixteen rules, one a function, copy a function's four rows into a relation of its own,
`(Xor A B OUT)`; they differ in the function's name alone.

#### `Zero-rows`

<!-- rule std.bool.gates Zero-rows -->
```lisp
(rule Zero-rows ()
  :match  (gate2 Zero ?a ?b ?o)
  :assert (Zero ?a ?b ?o)
  :why    "the table's row: Zero of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Zero_rows_lhs_rhs {
  rankdir=TB;
  label="Zero-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Zero_L" [label="Zero", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Zero_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Zero_R" [shape=octagon, label="Zero", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_Zero_R" -> "?a_R" [label="1", color="#ff7f0e", style=dashed];
    "h2_Zero_R" -> "?b_R" [label="2", color="#ff7f0e", style=dashed];
    "h2_Zero_R" -> "?o_R" [label="3", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `And-rows`

<!-- rule std.bool.gates And-rows -->
```lisp
(rule And-rows ()
  :match  (gate2 And ?a ?b ?o)
  :assert (And ?a ?b ?o)
  :why    "the table's row: And of {?a} and {?b} is {?o}")
```

```dot
digraph rule_And_rows_lhs_rhs {
  rankdir=TB;
  label="And-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "And_L" [label="And", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "And_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_And_R" [shape=octagon, label="And", color="#17becf", fontcolor="#17becf"];
    "h2_And_R" -> "?a_R" [label="1", color="#17becf", style=dashed];
    "h2_And_R" -> "?b_R" [label="2", color="#17becf", style=dashed];
    "h2_And_R" -> "?o_R" [label="3", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `Andn-rows`

<!-- rule std.bool.gates Andn-rows -->
```lisp
(rule Andn-rows ()
  :match  (gate2 Andn ?a ?b ?o)
  :assert (Andn ?a ?b ?o)
  :why    "the table's row: Andn of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Andn_rows_lhs_rhs {
  rankdir=TB;
  label="Andn-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Andn_L" [label="Andn", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Andn_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Andn_R" [shape=octagon, label="Andn", color="#e377c2", fontcolor="#e377c2"];
    "h2_Andn_R" -> "?a_R" [label="1", color="#e377c2", style=dashed];
    "h2_Andn_R" -> "?b_R" [label="2", color="#e377c2", style=dashed];
    "h2_Andn_R" -> "?o_R" [label="3", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

#### `Fst-rows`

<!-- rule std.bool.gates Fst-rows -->
```lisp
(rule Fst-rows ()
  :match  (gate2 Fst ?a ?b ?o)
  :assert (Fst ?a ?b ?o)
  :why    "the table's row: Fst of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Fst_rows_lhs_rhs {
  rankdir=TB;
  label="Fst-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Fst_L" [label="Fst", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Fst_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Fst_R" [shape=octagon, label="Fst", color="#17becf", fontcolor="#17becf"];
    "h2_Fst_R" -> "?a_R" [label="1", color="#17becf", style=dashed];
    "h2_Fst_R" -> "?b_R" [label="2", color="#17becf", style=dashed];
    "h2_Fst_R" -> "?o_R" [label="3", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `Randn-rows`

<!-- rule std.bool.gates Randn-rows -->
```lisp
(rule Randn-rows ()
  :match  (gate2 Randn ?a ?b ?o)
  :assert (Randn ?a ?b ?o)
  :why    "the table's row: Randn of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Randn_rows_lhs_rhs {
  rankdir=TB;
  label="Randn-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Randn_L" [label="Randn", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Randn_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Randn_R" [shape=octagon, label="Randn", color="#bcbd22", fontcolor="#bcbd22"];
    "h2_Randn_R" -> "?a_R" [label="1", color="#bcbd22", style=dashed];
    "h2_Randn_R" -> "?b_R" [label="2", color="#bcbd22", style=dashed];
    "h2_Randn_R" -> "?o_R" [label="3", color="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

#### `Snd-rows`

<!-- rule std.bool.gates Snd-rows -->
```lisp
(rule Snd-rows ()
  :match  (gate2 Snd ?a ?b ?o)
  :assert (Snd ?a ?b ?o)
  :why    "the table's row: Snd of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Snd_rows_lhs_rhs {
  rankdir=TB;
  label="Snd-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Snd_L" [label="Snd", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Snd_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Snd_R" [shape=octagon, label="Snd", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_Snd_R" -> "?a_R" [label="1", color="#1f77b4", style=dashed];
    "h2_Snd_R" -> "?b_R" [label="2", color="#1f77b4", style=dashed];
    "h2_Snd_R" -> "?o_R" [label="3", color="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `Xor-rows`

<!-- rule std.bool.gates Xor-rows -->
```lisp
(rule Xor-rows ()
  :match  (gate2 Xor ?a ?b ?o)
  :assert (Xor ?a ?b ?o)
  :why    "the table's row: Xor of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Xor_rows_lhs_rhs {
  rankdir=TB;
  label="Xor-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Xor_L" [label="Xor", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Xor_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Xor_R" [shape=octagon, label="Xor", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_Xor_R" -> "?a_R" [label="1", color="#ff7f0e", style=dashed];
    "h2_Xor_R" -> "?b_R" [label="2", color="#ff7f0e", style=dashed];
    "h2_Xor_R" -> "?o_R" [label="3", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `Or-rows`

<!-- rule std.bool.gates Or-rows -->
```lisp
(rule Or-rows ()
  :match  (gate2 Or ?a ?b ?o)
  :assert (Or ?a ?b ?o)
  :why    "the table's row: Or of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Or_rows_lhs_rhs {
  rankdir=TB;
  label="Or-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Or_L" [label="Or", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Or_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Or_R" [shape=octagon, label="Or", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_Or_R" -> "?a_R" [label="1", color="#1f77b4", style=dashed];
    "h2_Or_R" -> "?b_R" [label="2", color="#1f77b4", style=dashed];
    "h2_Or_R" -> "?o_R" [label="3", color="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `Nor-rows`

<!-- rule std.bool.gates Nor-rows -->
```lisp
(rule Nor-rows ()
  :match  (gate2 Nor ?a ?b ?o)
  :assert (Nor ?a ?b ?o)
  :why    "the table's row: Nor of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Nor_rows_lhs_rhs {
  rankdir=TB;
  label="Nor-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Nor_L" [label="Nor", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Nor_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Nor_R" [shape=octagon, label="Nor", color="#17becf", fontcolor="#17becf"];
    "h2_Nor_R" -> "?a_R" [label="1", color="#17becf", style=dashed];
    "h2_Nor_R" -> "?b_R" [label="2", color="#17becf", style=dashed];
    "h2_Nor_R" -> "?o_R" [label="3", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `Xnor-rows`

<!-- rule std.bool.gates Xnor-rows -->
```lisp
(rule Xnor-rows ()
  :match  (gate2 Xnor ?a ?b ?o)
  :assert (Xnor ?a ?b ?o)
  :why    "the table's row: Xnor of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Xnor_rows_lhs_rhs {
  rankdir=TB;
  label="Xnor-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Xnor_L" [label="Xnor", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Xnor_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Xnor_R" [shape=octagon, label="Xnor", color="#d62728", fontcolor="#d62728"];
    "h2_Xnor_R" -> "?a_R" [label="1", color="#d62728", style=dashed];
    "h2_Xnor_R" -> "?b_R" [label="2", color="#d62728", style=dashed];
    "h2_Xnor_R" -> "?o_R" [label="3", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `Nsnd-rows`

<!-- rule std.bool.gates Nsnd-rows -->
```lisp
(rule Nsnd-rows ()
  :match  (gate2 Nsnd ?a ?b ?o)
  :assert (Nsnd ?a ?b ?o)
  :why    "the table's row: Nsnd of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Nsnd_rows_lhs_rhs {
  rankdir=TB;
  label="Nsnd-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Nsnd_L" [label="Nsnd", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Nsnd_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Nsnd_R" [shape=octagon, label="Nsnd", color="#e377c2", fontcolor="#e377c2"];
    "h2_Nsnd_R" -> "?a_R" [label="1", color="#e377c2", style=dashed];
    "h2_Nsnd_R" -> "?b_R" [label="2", color="#e377c2", style=dashed];
    "h2_Nsnd_R" -> "?o_R" [label="3", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

#### `Orn-rows`

<!-- rule std.bool.gates Orn-rows -->
```lisp
(rule Orn-rows ()
  :match  (gate2 Orn ?a ?b ?o)
  :assert (Orn ?a ?b ?o)
  :why    "the table's row: Orn of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Orn_rows_lhs_rhs {
  rankdir=TB;
  label="Orn-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Orn_L" [label="Orn", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Orn_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Orn_R" [shape=octagon, label="Orn", color="#bcbd22", fontcolor="#bcbd22"];
    "h2_Orn_R" -> "?a_R" [label="1", color="#bcbd22", style=dashed];
    "h2_Orn_R" -> "?b_R" [label="2", color="#bcbd22", style=dashed];
    "h2_Orn_R" -> "?o_R" [label="3", color="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

#### `Nfst-rows`

<!-- rule std.bool.gates Nfst-rows -->
```lisp
(rule Nfst-rows ()
  :match  (gate2 Nfst ?a ?b ?o)
  :assert (Nfst ?a ?b ?o)
  :why    "the table's row: Nfst of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Nfst_rows_lhs_rhs {
  rankdir=TB;
  label="Nfst-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Nfst_L" [label="Nfst", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Nfst_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Nfst_R" [shape=octagon, label="Nfst", color="#d62728", fontcolor="#d62728"];
    "h2_Nfst_R" -> "?a_R" [label="1", color="#d62728", style=dashed];
    "h2_Nfst_R" -> "?b_R" [label="2", color="#d62728", style=dashed];
    "h2_Nfst_R" -> "?o_R" [label="3", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `Rorn-rows`

<!-- rule std.bool.gates Rorn-rows -->
```lisp
(rule Rorn-rows ()
  :match  (gate2 Rorn ?a ?b ?o)
  :assert (Rorn ?a ?b ?o)
  :why    "the table's row: Rorn of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Rorn_rows_lhs_rhs {
  rankdir=TB;
  label="Rorn-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Rorn_L" [label="Rorn", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Rorn_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Rorn_R" [shape=octagon, label="Rorn", color="#9467bd", fontcolor="#9467bd"];
    "h2_Rorn_R" -> "?a_R" [label="1", color="#9467bd", style=dashed];
    "h2_Rorn_R" -> "?b_R" [label="2", color="#9467bd", style=dashed];
    "h2_Rorn_R" -> "?o_R" [label="3", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `Nand-rows`

<!-- rule std.bool.gates Nand-rows -->
```lisp
(rule Nand-rows ()
  :match  (gate2 Nand ?a ?b ?o)
  :assert (Nand ?a ?b ?o)
  :why    "the table's row: Nand of {?a} and {?b} is {?o}")
```

```dot
digraph rule_Nand_rows_lhs_rhs {
  rankdir=TB;
  label="Nand-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "Nand_L" [label="Nand", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "Nand_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_Nand_R" [shape=octagon, label="Nand", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_Nand_R" -> "?a_R" [label="1", color="#1f77b4", style=dashed];
    "h2_Nand_R" -> "?b_R" [label="2", color="#1f77b4", style=dashed];
    "h2_Nand_R" -> "?o_R" [label="3", color="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `One-rows`

<!-- rule std.bool.gates One-rows -->
```lisp
(rule One-rows ()
  :match  (gate2 One ?a ?b ?o)
  :assert (One ?a ?b ?o)
  :why    "the table's row: One of {?a} and {?b} is {?o}")
```

```dot
digraph rule_One_rows_lhs_rhs {
  rankdir=TB;
  label="One-rows"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "One_L" [label="One", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "One_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?o_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?o_R" [label="?o", shape=diamond];
    "h2_One_R" [shape=octagon, label="One", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h2_One_R" -> "?a_R" [label="1", color="#ff7f0e", style=dashed];
    "h2_One_R" -> "?b_R" [label="2", color="#ff7f0e", style=dashed];
    "h2_One_R" -> "?o_R" [label="3", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

### The entry

The closing rule: it closes the table, the sixteen relations and the module's other relations, and names the
sixteen rules.

#### `gates`

<!-- rule std.bool.gates gates -->
```lisp
(rule gates ()
  :match  ()
  :assert (and (__closed__ gate2) (__closed__ column-name) (__closed__ inv) (__closed__ le)
               (__closed__ bit) (__closed__ gates-rule)
               (__closed__ Zero) (__closed__ And) (__closed__ Andn) (__closed__ Fst)
               (__closed__ Randn) (__closed__ Snd) (__closed__ Xor) (__closed__ Or)
               (__closed__ Nor) (__closed__ Xnor) (__closed__ Nsnd) (__closed__ Orn)
               (__closed__ Nfst) (__closed__ Rorn) (__closed__ Nand) (__closed__ One)
               (gates-rule Zero-rows) (gates-rule And-rows) (gates-rule Andn-rows)
               (gates-rule Fst-rows) (gates-rule Randn-rows) (gates-rule Snd-rows)
               (gates-rule Xor-rows) (gates-rule Or-rows) (gates-rule Nor-rows)
               (gates-rule Xnor-rows) (gates-rule Nsnd-rows) (gates-rule Orn-rows)
               (gates-rule Nfst-rows) (gates-rule Rorn-rows) (gates-rule Nand-rows)
               (gates-rule One-rows))
  :why    "the sixteen functions are written and derived, and never guessed: every relation of the module is closed")
```

```dot
digraph rule_gates_lhs_rhs {
  rankdir=TB;
  label="gates"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "h1_#empty_L" [shape=octagon, label="#empty", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "gate2_R" [label="gate2", shape=rectangle];
    "column-name_R" [label="column-name", shape=rectangle];
    "inv_R" [label="inv", shape=rectangle];
    "le_R" [label="le", shape=rectangle];
    "bit_R" [label="bit", shape=rectangle];
    "gates-rule_R" [label="gates-rule", shape=rectangle];
    "Zero_R" [label="Zero", shape=rectangle];
    "And_R" [label="And", shape=rectangle];
    "Andn_R" [label="Andn", shape=rectangle];
    "Fst_R" [label="Fst", shape=rectangle];
    "Randn_R" [label="Randn", shape=rectangle];
    "Snd_R" [label="Snd", shape=rectangle];
    "Xor_R" [label="Xor", shape=rectangle];
    "Or_R" [label="Or", shape=rectangle];
    "Nor_R" [label="Nor", shape=rectangle];
    "Xnor_R" [label="Xnor", shape=rectangle];
    "Nsnd_R" [label="Nsnd", shape=rectangle];
    "Orn_R" [label="Orn", shape=rectangle];
    "Nfst_R" [label="Nfst", shape=rectangle];
    "Rorn_R" [label="Rorn", shape=rectangle];
    "Nand_R" [label="Nand", shape=rectangle];
    "One_R" [label="One", shape=rectangle];
    "Zero-rows_R" [label="Zero-rows", shape=rectangle];
    "And-rows_R" [label="And-rows", shape=rectangle];
    "Andn-rows_R" [label="Andn-rows", shape=rectangle];
    "Fst-rows_R" [label="Fst-rows", shape=rectangle];
    "Randn-rows_R" [label="Randn-rows", shape=rectangle];
    "Snd-rows_R" [label="Snd-rows", shape=rectangle];
    "Xor-rows_R" [label="Xor-rows", shape=rectangle];
    "Or-rows_R" [label="Or-rows", shape=rectangle];
    "Nor-rows_R" [label="Nor-rows", shape=rectangle];
    "Xnor-rows_R" [label="Xnor-rows", shape=rectangle];
    "Nsnd-rows_R" [label="Nsnd-rows", shape=rectangle];
    "Orn-rows_R" [label="Orn-rows", shape=rectangle];
    "Nfst-rows_R" [label="Nfst-rows", shape=rectangle];
    "Rorn-rows_R" [label="Rorn-rows", shape=rectangle];
    "Nand-rows_R" [label="Nand-rows", shape=rectangle];
    "One-rows_R" [label="One-rows", shape=rectangle];
    "h2___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h2___closed___R" -> "gate2_R" [label="1", color="#e377c2", style=dashed];
    "h3___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h3___closed___R" -> "column-name_R" [label="1", color="#e377c2", style=dashed];
    "h4___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h4___closed___R" -> "inv_R" [label="1", color="#e377c2", style=dashed];
    "h5___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h5___closed___R" -> "le_R" [label="1", color="#e377c2", style=dashed];
    "h6___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h6___closed___R" -> "bit_R" [label="1", color="#e377c2", style=dashed];
    "h7___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h7___closed___R" -> "gates-rule_R" [label="1", color="#e377c2", style=dashed];
    "h8___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h8___closed___R" -> "Zero_R" [label="1", color="#e377c2", style=dashed];
    "h9___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h9___closed___R" -> "And_R" [label="1", color="#e377c2", style=dashed];
    "h10___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h10___closed___R" -> "Andn_R" [label="1", color="#e377c2", style=dashed];
    "h11___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h11___closed___R" -> "Fst_R" [label="1", color="#e377c2", style=dashed];
    "h12___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h12___closed___R" -> "Randn_R" [label="1", color="#e377c2", style=dashed];
    "h13___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h13___closed___R" -> "Snd_R" [label="1", color="#e377c2", style=dashed];
    "h14___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h14___closed___R" -> "Xor_R" [label="1", color="#e377c2", style=dashed];
    "h15___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h15___closed___R" -> "Or_R" [label="1", color="#e377c2", style=dashed];
    "h16___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h16___closed___R" -> "Nor_R" [label="1", color="#e377c2", style=dashed];
    "h17___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h17___closed___R" -> "Xnor_R" [label="1", color="#e377c2", style=dashed];
    "h18___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h18___closed___R" -> "Nsnd_R" [label="1", color="#e377c2", style=dashed];
    "h19___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h19___closed___R" -> "Orn_R" [label="1", color="#e377c2", style=dashed];
    "h20___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h20___closed___R" -> "Nfst_R" [label="1", color="#e377c2", style=dashed];
    "h21___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h21___closed___R" -> "Rorn_R" [label="1", color="#e377c2", style=dashed];
    "h22___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h22___closed___R" -> "Nand_R" [label="1", color="#e377c2", style=dashed];
    "h23___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h23___closed___R" -> "One_R" [label="1", color="#e377c2", style=dashed];
    "h24_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h24_gates-rule_R" -> "Zero-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h25_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h25_gates-rule_R" -> "And-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h26_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h26_gates-rule_R" -> "Andn-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h27_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h27_gates-rule_R" -> "Fst-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h28_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h28_gates-rule_R" -> "Randn-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h29_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h29_gates-rule_R" -> "Snd-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h30_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h30_gates-rule_R" -> "Xor-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h31_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h31_gates-rule_R" -> "Or-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h32_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h32_gates-rule_R" -> "Nor-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h33_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h33_gates-rule_R" -> "Xnor-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h34_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h34_gates-rule_R" -> "Nsnd-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h35_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h35_gates-rule_R" -> "Orn-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h36_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h36_gates-rule_R" -> "Nfst-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h37_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h37_gates-rule_R" -> "Rorn-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h38_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h38_gates-rule_R" -> "Nand-rows_R" [label="1", color="#2ca02c", style=dashed];
    "h39_gates-rule_R" [shape=octagon, label="gates-rule", color="#2ca02c", fontcolor="#2ca02c"];
    "h39_gates-rule_R" -> "One-rows_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

## `std.bool.props`

### A function's column, and the functions made from it

`column-of` gathers a function's four rows into one fact, `(column F A B C D)`, its outputs at 00, 01, 10 and 11.
Most properties are then a pattern on that one fact. The five rules after it relate two functions: each matches
the two columns, with the same variables in exchanged places or with `inv` between them.

#### `column-of`

<!-- rule std.bool.props column-of -->
```lisp
(rule column-of ()
  :match  (and (gate2 ?f 0 0 ?a) (gate2 ?f 0 1 ?b) (gate2 ?f 1 0 ?c) (gate2 ?f 1 1 ?d))
  :assert (column ?f ?a ?b ?c ?d)
  :why    "the four rows of {?f}")
```

```dot
digraph rule_column_of_lhs_rhs {
  rankdir=TB;
  label="column-of"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?a_L" [label="?a", shape=diamond];
    "1_L" [label="1", shape=rectangle];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?b_L" [label="4", color="#7f7f7f"];
    "h3_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h3_gate2_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h3_gate2_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h3_gate2_L" -> "?c_L" [label="4", color="#7f7f7f"];
    "h4_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h4_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h4_gate2_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h4_gate2_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h4_gate2_L" -> "?d_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?c_R" [label="?c", shape=diamond];
    "?d_R" [label="?d", shape=diamond];
    "h5_column_R" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h5_column_R" -> "?f_R" [label="1", color="#d62728", style=dashed];
    "h5_column_R" -> "?a_R" [label="2", color="#d62728", style=dashed];
    "h5_column_R" -> "?b_R" [label="3", color="#d62728", style=dashed];
    "h5_column_R" -> "?c_R" [label="4", color="#d62728", style=dashed];
    "h5_column_R" -> "?d_R" [label="5", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `swap`

<!-- rule std.bool.props swap -->
```lisp
(rule swap ()
  :match  (and (column ?f ?a ?b ?c ?d) (column ?g ?a ?c ?b ?d))
  :assert (swap-of ?f ?g)
  :why    "{?g}'s rows at 01 and 10 are {?f}'s at 10 and 01")
```

```dot
digraph rule_swap_lhs_rhs {
  rankdir=TB;
  label="swap"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "?g_L" [label="?g", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
    "h2_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h2_column_L" -> "?g_L" [label="1", color="#d62728"];
    "h2_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h2_column_L" -> "?c_L" [label="3", color="#d62728"];
    "h2_column_L" -> "?b_L" [label="4", color="#d62728"];
    "h2_column_L" -> "?d_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?g_R" [label="?g", shape=diamond];
    "?f_R" -> "?g_R" [label="swap-of", color="#7f7f7f", fontcolor="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

#### `complement`

<!-- rule std.bool.props complement -->
```lisp
(rule complement ()
  :match  (and (column ?f ?a ?b ?c ?d) (inv ?a ?a1) (inv ?b ?b1) (inv ?c ?c1) (inv ?d ?d1)
               (column ?g ?a1 ?b1 ?c1 ?d1))
  :assert (complement-of ?f ?g)
  :why    "every output of {?g} is the negation of {?f}'s")
```

```dot
digraph rule_complement_lhs_rhs {
  rankdir=TB;
  label="complement"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "?a1_L" [label="?a1", shape=diamond];
    "?b1_L" [label="?b1", shape=diamond];
    "?c1_L" [label="?c1", shape=diamond];
    "?d1_L" [label="?d1", shape=diamond];
    "?g_L" [label="?g", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
    "?a_L" -> "?a1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "?b_L" -> "?b1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "?c_L" -> "?c1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "?d_L" -> "?d1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "h2_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h2_column_L" -> "?g_L" [label="1", color="#d62728"];
    "h2_column_L" -> "?a1_L" [label="2", color="#d62728"];
    "h2_column_L" -> "?b1_L" [label="3", color="#d62728"];
    "h2_column_L" -> "?c1_L" [label="4", color="#d62728"];
    "h2_column_L" -> "?d1_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?g_R" [label="?g", shape=diamond];
    "?f_R" -> "?g_R" [label="complement-of", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `dual`

<!-- rule std.bool.props dual -->
```lisp
(rule dual ()
  :match  (and (column ?f ?a ?b ?c ?d) (inv ?a ?a1) (inv ?b ?b1) (inv ?c ?c1) (inv ?d ?d1)
               (column ?g ?d1 ?c1 ?b1 ?a1))
  :assert (dual-of ?f ?g)
  :why    "{?g} is {?f} with both inputs and the output negated")
```

```dot
digraph rule_dual_lhs_rhs {
  rankdir=TB;
  label="dual"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "?a1_L" [label="?a1", shape=diamond];
    "?b1_L" [label="?b1", shape=diamond];
    "?c1_L" [label="?c1", shape=diamond];
    "?d1_L" [label="?d1", shape=diamond];
    "?g_L" [label="?g", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
    "?a_L" -> "?a1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "?b_L" -> "?b1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "?c_L" -> "?c1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "?d_L" -> "?d1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "h2_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h2_column_L" -> "?g_L" [label="1", color="#d62728"];
    "h2_column_L" -> "?d1_L" [label="2", color="#d62728"];
    "h2_column_L" -> "?c1_L" [label="3", color="#d62728"];
    "h2_column_L" -> "?b1_L" [label="4", color="#d62728"];
    "h2_column_L" -> "?a1_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?g_R" [label="?g", shape=diamond];
    "?f_R" -> "?g_R" [label="dual-of", color="#2ca02c", fontcolor="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `left-neg`

<!-- rule std.bool.props left-neg -->
```lisp
(rule left-neg ()
  :match  (and (column ?f ?a ?b ?c ?d) (column ?g ?c ?d ?a ?b))
  :assert (left-negated ?f ?g)
  :why    "{?g} is {?f} with its first input negated")
```

```dot
digraph rule_left_neg_lhs_rhs {
  rankdir=TB;
  label="left-neg"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "?g_L" [label="?g", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
    "h2_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h2_column_L" -> "?g_L" [label="1", color="#d62728"];
    "h2_column_L" -> "?c_L" [label="2", color="#d62728"];
    "h2_column_L" -> "?d_L" [label="3", color="#d62728"];
    "h2_column_L" -> "?a_L" [label="4", color="#d62728"];
    "h2_column_L" -> "?b_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?g_R" [label="?g", shape=diamond];
    "?f_R" -> "?g_R" [label="left-negated", color="#ff7f0e", fontcolor="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `right-neg`

<!-- rule std.bool.props right-neg -->
```lisp
(rule right-neg ()
  :match  (and (column ?f ?a ?b ?c ?d) (column ?g ?b ?a ?d ?c))
  :assert (right-negated ?f ?g)
  :why    "{?g} is {?f} with its second input negated")
```

```dot
digraph rule_right_neg_lhs_rhs {
  rankdir=TB;
  label="right-neg"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "?g_L" [label="?g", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
    "h2_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h2_column_L" -> "?g_L" [label="1", color="#d62728"];
    "h2_column_L" -> "?b_L" [label="2", color="#d62728"];
    "h2_column_L" -> "?a_L" [label="3", color="#d62728"];
    "h2_column_L" -> "?d_L" [label="4", color="#d62728"];
    "h2_column_L" -> "?c_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?g_R" [label="?g", shape=diamond];
    "?f_R" -> "?g_R" [label="right-negated", color="#e377c2", fontcolor="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

### Properties of one function

*For every input* is the whole column in one premise: `commutative-r` matches a column whose outputs at 01 and 10
are one variable. Associativity is checked a point at a time, by four rows of the table, and holds where all eight
points do.

$$
\operatorname{commutative}(F) \iff F(0, 1) = F(1, 0), \qquad \operatorname{idempotent}(F) \iff F(0, 0) = 0 \land F(1, 1) = 1
$$

#### `commutative-r`

<!-- rule std.bool.props commutative-r -->
```lisp
(rule commutative-r ()
  :match  (column ?f ?a ?b ?b ?d)
  :assert (commutative ?f)
  :why    "{?f} gives the same at 01 and at 10")
```

```dot
digraph rule_commutative_r_lhs_rhs {
  rankdir=TB;
  label="commutative-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h2_commutative_R" [shape=octagon, label="commutative", color="#9467bd", fontcolor="#9467bd"];
    "h2_commutative_R" -> "?f_R" [label="1", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `assoc-point`

<!-- rule std.bool.props assoc-point -->
```lisp
(rule assoc-point ()
  :match  (and (gate2 ?f ?a ?b ?x) (gate2 ?f ?x ?c ?l) (gate2 ?f ?b ?c ?y) (gate2 ?f ?a ?y ?l))
  :assert (assoc-at ?f ?a ?b ?c)
  :why    "({?a} {?f} {?b}) {?f} {?c} and {?a} {?f} ({?b} {?f} {?c}) are both {?l}")
```

```dot
digraph rule_assoc_point_lhs_rhs {
  rankdir=TB;
  label="assoc-point"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?x_L" [label="?x", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?l_L" [label="?l", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?x_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?x_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?c_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?l_L" [label="4", color="#7f7f7f"];
    "h3_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h3_gate2_L" -> "?b_L" [label="2", color="#7f7f7f"];
    "h3_gate2_L" -> "?c_L" [label="3", color="#7f7f7f"];
    "h3_gate2_L" -> "?y_L" [label="4", color="#7f7f7f"];
    "h4_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h4_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h4_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h4_gate2_L" -> "?y_L" [label="3", color="#7f7f7f"];
    "h4_gate2_L" -> "?l_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?c_R" [label="?c", shape=diamond];
    "h5_assoc-at_R" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h5_assoc-at_R" -> "?f_R" [label="1", color="#7f7f7f", style=dashed];
    "h5_assoc-at_R" -> "?a_R" [label="2", color="#7f7f7f", style=dashed];
    "h5_assoc-at_R" -> "?b_R" [label="3", color="#7f7f7f", style=dashed];
    "h5_assoc-at_R" -> "?c_R" [label="4", color="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

#### `associative-r`

<!-- rule std.bool.props associative-r -->
```lisp
(rule associative-r ()
  :match  (and (assoc-at ?f 0 0 0) (assoc-at ?f 0 0 1) (assoc-at ?f 0 1 0) (assoc-at ?f 0 1 1)
               (assoc-at ?f 1 0 0) (assoc-at ?f 1 0 1) (assoc-at ?f 1 1 0) (assoc-at ?f 1 1 1))
  :assert (associative ?f)
  :why    "{?f} associates at all eight inputs")
```

```dot
digraph rule_associative_r_lhs_rhs {
  rankdir=TB;
  label="associative-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "1_L" [label="1", shape=rectangle];
    "h1_assoc-at_L" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_assoc-at_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_assoc-at_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h1_assoc-at_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h1_assoc-at_L" -> "0_L" [label="4", color="#7f7f7f"];
    "h2_assoc-at_L" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_assoc-at_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_assoc-at_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h2_assoc-at_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h2_assoc-at_L" -> "1_L" [label="4", color="#7f7f7f"];
    "h3_assoc-at_L" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_assoc-at_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h3_assoc-at_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h3_assoc-at_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h3_assoc-at_L" -> "0_L" [label="4", color="#7f7f7f"];
    "h4_assoc-at_L" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h4_assoc-at_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h4_assoc-at_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h4_assoc-at_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h4_assoc-at_L" -> "1_L" [label="4", color="#7f7f7f"];
    "h5_assoc-at_L" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h5_assoc-at_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h5_assoc-at_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h5_assoc-at_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h5_assoc-at_L" -> "0_L" [label="4", color="#7f7f7f"];
    "h6_assoc-at_L" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h6_assoc-at_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h6_assoc-at_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h6_assoc-at_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h6_assoc-at_L" -> "1_L" [label="4", color="#7f7f7f"];
    "h7_assoc-at_L" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h7_assoc-at_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h7_assoc-at_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h7_assoc-at_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h7_assoc-at_L" -> "0_L" [label="4", color="#7f7f7f"];
    "h8_assoc-at_L" [shape=octagon, label="assoc-at", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h8_assoc-at_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h8_assoc-at_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h8_assoc-at_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h8_assoc-at_L" -> "1_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h9_associative_R" [shape=octagon, label="associative", color="#2ca02c", fontcolor="#2ca02c"];
    "h9_associative_R" -> "?f_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `idempotent-r`

<!-- rule std.bool.props idempotent-r -->
```lisp
(rule idempotent-r ()
  :match  (and (gate2 ?f 0 0 0) (gate2 ?f 1 1 1))
  :assert (idempotent ?f)
  :why    "{?f} of 0 and 0 is 0, and of 1 and 1 is 1")
```

```dot
digraph rule_idempotent_r_lhs_rhs {
  rankdir=TB;
  label="idempotent-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "1_L" [label="1", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h3_idempotent_R" [shape=octagon, label="idempotent", color="#2ca02c", fontcolor="#2ca02c"];
    "h3_idempotent_R" -> "?f_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `reads-first-r`

<!-- rule std.bool.props reads-first-r -->
```lisp
(rule reads-first-r ()
  :match  (and (gate2 ?f 0 ?b ?x) (gate2 ?f 1 ?b ?y) (inv ?x ?y))
  :assert (reads-first ?f)
  :why    "with the second input {?b}, changing the first changes {?f}")
```

```dot
digraph rule_reads_first_r_lhs_rhs {
  rankdir=TB;
  label="reads-first-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?b_L" [label="?b", shape=diamond];
    "?x_L" [label="?x", shape=diamond];
    "1_L" [label="1", shape=rectangle];
    "?y_L" [label="?y", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?x_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?y_L" [label="4", color="#7f7f7f"];
    "?x_L" -> "?y_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h3_reads-first_R" [shape=octagon, label="reads-first", color="#1f77b4", fontcolor="#1f77b4"];
    "h3_reads-first_R" -> "?f_R" [label="1", color="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `reads-second-r`

<!-- rule std.bool.props reads-second-r -->
```lisp
(rule reads-second-r ()
  :match  (and (gate2 ?f ?a 0 ?x) (gate2 ?f ?a 1 ?y) (inv ?x ?y))
  :assert (reads-second ?f)
  :why    "with the first input {?a}, changing the second changes {?f}")
```

```dot
digraph rule_reads_second_r_lhs_rhs {
  rankdir=TB;
  label="reads-second-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?x_L" [label="?x", shape=diamond];
    "1_L" [label="1", shape=rectangle];
    "?y_L" [label="?y", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?x_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?y_L" [label="4", color="#7f7f7f"];
    "?x_L" -> "?y_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h3_reads-second_R" [shape=octagon, label="reads-second", color="#2ca02c", fontcolor="#2ca02c"];
    "h3_reads-second_R" -> "?f_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `ignores-first-r`

<!-- rule std.bool.props ignores-first-r -->
```lisp
(rule ignores-first-r ()
  :match  (column ?f ?a ?b ?a ?b)
  :assert (ignores-first ?f)
  :why    "{?f}'s rows do not change with the first input")
```

```dot
digraph rule_ignores_first_r_lhs_rhs {
  rankdir=TB;
  label="ignores-first-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h2_ignores-first_R" [shape=octagon, label="ignores-first", color="#2ca02c", fontcolor="#2ca02c"];
    "h2_ignores-first_R" -> "?f_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `ignores-second-r`

<!-- rule std.bool.props ignores-second-r -->
```lisp
(rule ignores-second-r ()
  :match  (column ?f ?a ?a ?c ?c)
  :assert (ignores-second ?f)
  :why    "{?f}'s rows do not change with the second input")
```

```dot
digraph rule_ignores_second_r_lhs_rhs {
  rankdir=TB;
  label="ignores-second-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="5", color="#d62728"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h2_ignores-second_R" [shape=octagon, label="ignores-second", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_ignores-second_R" -> "?f_R" [label="1", color="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

### Identity and absorbing elements

One-sided first, each from two rows of the table, and the two-sided from both.

#### `right-identity`

<!-- rule std.bool.props right-identity -->
```lisp
(rule right-identity ()
  :match  (and (gate2 ?f 0 ?e 0) (gate2 ?f 1 ?e 1))
  :assert (right-identity-of ?f ?e)
  :why    "{?f} of a bit and {?e} is that bit")
```

```dot
digraph rule_right_identity_lhs_rhs {
  rankdir=TB;
  label="right-identity"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?e_L" [label="?e", shape=diamond];
    "1_L" [label="1", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?e_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?e_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?e_R" [label="?e", shape=diamond];
    "?f_R" -> "?e_R" [label="right-identity-of", color="#bcbd22", fontcolor="#bcbd22", style=dashed];
  }
}
```
<!-- /rule -->

#### `left-identity`

<!-- rule std.bool.props left-identity -->
```lisp
(rule left-identity ()
  :match  (and (gate2 ?f ?e 0 0) (gate2 ?f ?e 1 1))
  :assert (left-identity-of ?f ?e)
  :why    "{?f} of {?e} and a bit is that bit")
```

```dot
digraph rule_left_identity_lhs_rhs {
  rankdir=TB;
  label="left-identity"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?e_L" [label="?e", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "1_L" [label="1", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?e_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?e_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?e_R" [label="?e", shape=diamond];
    "?f_R" -> "?e_R" [label="left-identity-of", color="#17becf", fontcolor="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

#### `identity`

<!-- rule std.bool.props identity -->
```lisp
(rule identity ()
  :match  (and (right-identity-of ?f ?e) (left-identity-of ?f ?e))
  :assert (identity-of ?f ?e)
  :why    "{?e} is neutral for {?f} on both sides")
```

```dot
digraph rule_identity_lhs_rhs {
  rankdir=TB;
  label="identity"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?e_L" [label="?e", shape=diamond];
    "?f_L" -> "?e_L" [label="right-identity-of", color="#bcbd22", fontcolor="#bcbd22"];
    "?f_L" -> "?e_L" [label="left-identity-of", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?e_R" [label="?e", shape=diamond];
    "?f_R" -> "?e_R" [label="identity-of", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `right-absorbing`

<!-- rule std.bool.props right-absorbing -->
```lisp
(rule right-absorbing ()
  :match  (and (gate2 ?f 0 ?e ?e) (gate2 ?f 1 ?e ?e))
  :assert (right-absorbing-of ?f ?e)
  :why    "{?f} of any bit and {?e} is {?e}")
```

```dot
digraph rule_right_absorbing_lhs_rhs {
  rankdir=TB;
  label="right-absorbing"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "?e_L" [label="?e", shape=diamond];
    "1_L" [label="1", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?e_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?e_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?e_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?e_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?e_R" [label="?e", shape=diamond];
    "?f_R" -> "?e_R" [label="right-absorbing-of", color="#2ca02c", fontcolor="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `left-absorbing`

<!-- rule std.bool.props left-absorbing -->
```lisp
(rule left-absorbing ()
  :match  (and (gate2 ?f ?e 0 ?e) (gate2 ?f ?e 1 ?e))
  :assert (left-absorbing-of ?f ?e)
  :why    "{?f} of {?e} and any bit is {?e}")
```

```dot
digraph rule_left_absorbing_lhs_rhs {
  rankdir=TB;
  label="left-absorbing"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?e_L" [label="?e", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "1_L" [label="1", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?e_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?e_L" [label="4", color="#7f7f7f"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?e_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?e_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?e_R" [label="?e", shape=diamond];
    "?f_R" -> "?e_R" [label="left-absorbing-of", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `absorbing`

<!-- rule std.bool.props absorbing -->
```lisp
(rule absorbing ()
  :match  (and (right-absorbing-of ?f ?e) (left-absorbing-of ?f ?e))
  :assert (absorbing-of ?f ?e)
  :why    "{?e} decides {?f} on both sides")
```

```dot
digraph rule_absorbing_lhs_rhs {
  rankdir=TB;
  label="absorbing"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?e_L" [label="?e", shape=diamond];
    "?f_L" -> "?e_L" [label="right-absorbing-of", color="#2ca02c", fontcolor="#2ca02c"];
    "?f_L" -> "?e_L" [label="left-absorbing-of", color="#1f77b4", fontcolor="#1f77b4"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "?e_R" [label="?e", shape=diamond];
    "?f_R" -> "?e_R" [label="absorbing-of", color="#ff7f0e", fontcolor="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

### Post's five classes

A function is in a class by a rule over its column or its rows, and *outside* a class by a rule with one witness:
a row, or two rows, that the class forbids. So being outside needs no `absent`. A function outside all five is
complete alone.

#### `preserves-zero-r`

<!-- rule std.bool.props preserves-zero-r -->
```lisp
(rule preserves-zero-r ()
  :match  (gate2 ?f 0 0 0)
  :assert (preserves-zero ?f)
  :why    "{?f} of 0 and 0 is 0")
```

```dot
digraph rule_preserves_zero_r_lhs_rhs {
  rankdir=TB;
  label="preserves-zero-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h2_preserves-zero_R" [shape=octagon, label="preserves-zero", color="#2ca02c", fontcolor="#2ca02c"];
    "h2_preserves-zero_R" -> "?f_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `preserves-one-r`

<!-- rule std.bool.props preserves-one-r -->
```lisp
(rule preserves-one-r ()
  :match  (gate2 ?f 1 1 1)
  :assert (preserves-one ?f)
  :why    "{?f} of 1 and 1 is 1")
```

```dot
digraph rule_preserves_one_r_lhs_rhs {
  rankdir=TB;
  label="preserves-one-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "1_L" [label="1", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "1_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h2_preserves-one_R" [shape=octagon, label="preserves-one", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_preserves-one_R" -> "?f_R" [label="1", color="#7f7f7f", style=dashed];
  }
}
```
<!-- /rule -->

#### `monotone-r`

<!-- rule std.bool.props monotone-r -->
```lisp
(rule monotone-r ()
  :match  (and (column ?f ?a ?b ?c ?d) (le ?a ?b) (le ?a ?c) (le ?b ?d) (le ?c ?d))
  :assert (monotone ?f)
  :why    "{?f}'s outputs never fall as an input rises")
```

```dot
digraph rule_monotone_r_lhs_rhs {
  rankdir=TB;
  label="monotone-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
    "?a_L" -> "?b_L" [label="le", color="#1f77b4", fontcolor="#1f77b4"];
    "?a_L" -> "?c_L" [label="le", color="#1f77b4", fontcolor="#1f77b4"];
    "?b_L" -> "?d_L" [label="le", color="#1f77b4", fontcolor="#1f77b4"];
    "?c_L" -> "?d_L" [label="le", color="#1f77b4", fontcolor="#1f77b4"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h2_monotone_R" [shape=octagon, label="monotone", color="#d62728", fontcolor="#d62728"];
    "h2_monotone_R" -> "?f_R" [label="1", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `affine-r`

<!-- rule std.bool.props affine-r -->
```lisp
(rule affine-r ()
  :match  (and (column ?f ?a ?b ?c ?d) (gate2 Xor ?a ?b ?x) (gate2 Xor ?c ?d ?x))
  :assert (affine ?f)
  :why    "{?f}'s four outputs have an even number of ones")
```

```dot
digraph rule_affine_r_lhs_rhs {
  rankdir=TB;
  label="affine-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "Xor_L" [label="Xor", shape=rectangle];
    "?x_L" [label="?x", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "Xor_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?x_L" [label="4", color="#7f7f7f"];
    "h3_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_gate2_L" -> "Xor_L" [label="1", color="#7f7f7f"];
    "h3_gate2_L" -> "?c_L" [label="2", color="#7f7f7f"];
    "h3_gate2_L" -> "?d_L" [label="3", color="#7f7f7f"];
    "h3_gate2_L" -> "?x_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h4_affine_R" [shape=octagon, label="affine", color="#e377c2", fontcolor="#e377c2"];
    "h4_affine_R" -> "?f_R" [label="1", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

#### `self-dual-r`

<!-- rule std.bool.props self-dual-r -->
```lisp
(rule self-dual-r ()
  :match  (dual-of ?f ?f)
  :assert (self-dual ?f)
  :why    "{?f} is its own dual")
```

```dot
digraph rule_self_dual_r_lhs_rhs {
  rankdir=TB;
  label="self-dual-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?f_L" -> "?f_L" [label="dual-of", color="#2ca02c", fontcolor="#2ca02c"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h1_self-dual_R" [shape=octagon, label="self-dual", color="#9467bd", fontcolor="#9467bd"];
    "h1_self-dual_R" -> "?f_R" [label="1", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `escapes-zero-r`

<!-- rule std.bool.props escapes-zero-r -->
```lisp
(rule escapes-zero-r ()
  :match  (gate2 ?f 0 0 1)
  :assert (escapes-zero ?f)
  :why    "{?f} of 0 and 0 is 1")
```

```dot
digraph rule_escapes_zero_r_lhs_rhs {
  rankdir=TB;
  label="escapes-zero-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "1_L" [label="1", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "1_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h2_escapes-zero_R" [shape=octagon, label="escapes-zero", color="#9467bd", fontcolor="#9467bd"];
    "h2_escapes-zero_R" -> "?f_R" [label="1", color="#9467bd", style=dashed];
  }
}
```
<!-- /rule -->

#### `escapes-one-r`

<!-- rule std.bool.props escapes-one-r -->
```lisp
(rule escapes-one-r ()
  :match  (gate2 ?f 1 1 0)
  :assert (escapes-one ?f)
  :why    "{?f} of 1 and 1 is 0")
```

```dot
digraph rule_escapes_one_r_lhs_rhs {
  rankdir=TB;
  label="escapes-one-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "1_L" [label="1", shape=rectangle];
    "0_L" [label="0", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "1_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "1_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "0_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h2_escapes-one_R" [shape=octagon, label="escapes-one", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_escapes-one_R" -> "?f_R" [label="1", color="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `escapes-monotone-r`

<!-- rule std.bool.props escapes-monotone-r -->
```lisp
(rule escapes-monotone-r ()
  :match  (and (gate2 ?f ?a ?b 1) (le ?a ?c) (le ?b ?d) (gate2 ?f ?c ?d 0))
  :assert (escapes-monotone ?f)
  :why    "{?f} is 1 at {?a}{?b} and 0 at {?c}{?d}, which is not lower")
```

```dot
digraph rule_escapes_monotone_r_lhs_rhs {
  rankdir=TB;
  label="escapes-monotone-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "1_L" [label="1", shape=rectangle];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "0_L" [label="0", shape=rectangle];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "1_L" [label="4", color="#7f7f7f"];
    "?a_L" -> "?c_L" [label="le", color="#1f77b4", fontcolor="#1f77b4"];
    "?b_L" -> "?d_L" [label="le", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?c_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?d_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "0_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h3_escapes-monotone_R" [shape=octagon, label="escapes-monotone", color="#e377c2", fontcolor="#e377c2"];
    "h3_escapes-monotone_R" -> "?f_R" [label="1", color="#e377c2", style=dashed];
  }
}
```
<!-- /rule -->

#### `escapes-affine-r`

<!-- rule std.bool.props escapes-affine-r -->
```lisp
(rule escapes-affine-r ()
  :match  (and (column ?f ?a ?b ?c ?d) (gate2 Xor ?a ?b ?x) (gate2 Xor ?c ?d ?y) (inv ?x ?y))
  :assert (escapes-affine ?f)
  :why    "{?f}'s four outputs have an odd number of ones")
```

```dot
digraph rule_escapes_affine_r_lhs_rhs {
  rankdir=TB;
  label="escapes-affine-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?d_L" [label="?d", shape=diamond];
    "Xor_L" [label="Xor", shape=rectangle];
    "?x_L" [label="?x", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "h1_column_L" [shape=octagon, label="column", color="#d62728", fontcolor="#d62728"];
    "h1_column_L" -> "?f_L" [label="1", color="#d62728"];
    "h1_column_L" -> "?a_L" [label="2", color="#d62728"];
    "h1_column_L" -> "?b_L" [label="3", color="#d62728"];
    "h1_column_L" -> "?c_L" [label="4", color="#d62728"];
    "h1_column_L" -> "?d_L" [label="5", color="#d62728"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "Xor_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?x_L" [label="4", color="#7f7f7f"];
    "h3_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_gate2_L" -> "Xor_L" [label="1", color="#7f7f7f"];
    "h3_gate2_L" -> "?c_L" [label="2", color="#7f7f7f"];
    "h3_gate2_L" -> "?d_L" [label="3", color="#7f7f7f"];
    "h3_gate2_L" -> "?y_L" [label="4", color="#7f7f7f"];
    "?x_L" -> "?y_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h4_escapes-affine_R" [shape=octagon, label="escapes-affine", color="#8c564b", fontcolor="#8c564b"];
    "h4_escapes-affine_R" -> "?f_R" [label="1", color="#8c564b", style=dashed];
  }
}
```
<!-- /rule -->

#### `escapes-self-dual-r`

<!-- rule std.bool.props escapes-self-dual-r -->
```lisp
(rule escapes-self-dual-r ()
  :match  (and (gate2 ?f ?a ?b ?x) (inv ?a ?a1) (inv ?b ?b1) (gate2 ?f ?a1 ?b1 ?x))
  :assert (escapes-self-dual ?f)
  :why    "{?f} gives {?x} at {?a}{?b} and at its opposite")
```

```dot
digraph rule_escapes_self_dual_r_lhs_rhs {
  rankdir=TB;
  label="escapes-self-dual-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?x_L" [label="?x", shape=diamond];
    "?a1_L" [label="?a1", shape=diamond];
    "?b1_L" [label="?b1", shape=diamond];
    "h1_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h1_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h1_gate2_L" -> "?a_L" [label="2", color="#7f7f7f"];
    "h1_gate2_L" -> "?b_L" [label="3", color="#7f7f7f"];
    "h1_gate2_L" -> "?x_L" [label="4", color="#7f7f7f"];
    "?a_L" -> "?a1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "?b_L" -> "?b1_L" [label="inv", color="#9467bd", fontcolor="#9467bd"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?a1_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?b1_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?x_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h3_escapes-self-dual_R" [shape=octagon, label="escapes-self-dual", color="#2ca02c", fontcolor="#2ca02c"];
    "h3_escapes-self-dual_R" -> "?f_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `complete-alone-r`

<!-- rule std.bool.props complete-alone-r -->
```lisp
(rule complete-alone-r ()
  :match  (and (escapes-zero ?f) (escapes-one ?f) (escapes-monotone ?f) (escapes-affine ?f) (escapes-self-dual ?f))
  :assert (complete-alone ?f)
  :why    "{?f} is outside each of Post's five classes")
```

```dot
digraph rule_complete_alone_r_lhs_rhs {
  rankdir=TB;
  label="complete-alone-r"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?f_L" [label="?f", shape=diamond];
    "h1_escapes-zero_L" [shape=octagon, label="escapes-zero", color="#9467bd", fontcolor="#9467bd"];
    "h1_escapes-zero_L" -> "?f_L" [label="1", color="#9467bd"];
    "h2_escapes-one_L" [shape=octagon, label="escapes-one", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_escapes-one_L" -> "?f_L" [label="1", color="#1f77b4"];
    "h3_escapes-monotone_L" [shape=octagon, label="escapes-monotone", color="#e377c2", fontcolor="#e377c2"];
    "h3_escapes-monotone_L" -> "?f_L" [label="1", color="#e377c2"];
    "h4_escapes-affine_L" [shape=octagon, label="escapes-affine", color="#8c564b", fontcolor="#8c564b"];
    "h4_escapes-affine_L" -> "?f_L" [label="1", color="#8c564b"];
    "h5_escapes-self-dual_L" [shape=octagon, label="escapes-self-dual", color="#2ca02c", fontcolor="#2ca02c"];
    "h5_escapes-self-dual_L" -> "?f_L" [label="1", color="#2ca02c"];
  }
  subgraph cluster_rhs { label="assert";
    "?f_R" [label="?f", shape=diamond];
    "h6_complete-alone_R" [shape=octagon, label="complete-alone", color="#1f77b4", fontcolor="#1f77b4"];
    "h6_complete-alone_R" -> "?f_R" [label="1", color="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

### The entry

The closing rule: it closes the module's relations and names each rule above and the entry `gates`. Its drawing
is not shown: it has a node for each of those facts.

#### `props`

<!-- rule std.bool.props props -->
```lisp
(rule props ()
  :match  ()
  :assert (and (__closed__ column) (__closed__ swap-of) (__closed__ complement-of)
               (__closed__ dual-of) (__closed__ left-negated) (__closed__ right-negated)
               (__closed__ commutative) (__closed__ associative) (__closed__ idempotent)
               (__closed__ reads-first) (__closed__ reads-second) (__closed__ ignores-first)
               (__closed__ ignores-second) (__closed__ right-identity-of)
               (__closed__ left-identity-of) (__closed__ identity-of)
               (__closed__ right-absorbing-of) (__closed__ left-absorbing-of)
               (__closed__ absorbing-of) (__closed__ preserves-zero) (__closed__ preserves-one)
               (__closed__ monotone) (__closed__ affine) (__closed__ self-dual)
               (__closed__ escapes-zero) (__closed__ escapes-one) (__closed__ escapes-monotone)
               (__closed__ escapes-affine) (__closed__ escapes-self-dual)
               (__closed__ complete-alone) (__closed__ assoc-at) (__closed__ props-rule)
               (props-rule column-of) (props-rule swap) (props-rule complement) (props-rule dual)
               (props-rule left-neg) (props-rule right-neg) (props-rule commutative-r)
               (props-rule assoc-point) (props-rule associative-r) (props-rule idempotent-r)
               (props-rule reads-first-r) (props-rule reads-second-r)
               (props-rule ignores-first-r) (props-rule ignores-second-r)
               (props-rule right-identity) (props-rule left-identity) (props-rule identity)
               (props-rule right-absorbing) (props-rule left-absorbing) (props-rule absorbing)
               (props-rule preserves-zero-r) (props-rule preserves-one-r) (props-rule monotone-r)
               (props-rule affine-r) (props-rule self-dual-r) (props-rule escapes-zero-r)
               (props-rule escapes-one-r) (props-rule escapes-monotone-r)
               (props-rule escapes-affine-r) (props-rule escapes-self-dual-r)
               (props-rule complete-alone-r) (props-rule gates))
  :why    "what is known of the sixteen is derived from the rows and never guessed: every relation of the module is closed")
```
<!-- /rule -->

## `std.bool.circuit`

`propagate` is the simulation: a gate, the values on its two inputs, and the table's row for them, in one match.
`one-value` is the one check.

#### `propagate`

<!-- rule std.bool.circuit propagate -->
```lisp
(rule propagate ()
  :match  (and (gate ?g ?f ?a ?b ?o) (val ?a ?x) (val ?b ?y) (gate2 ?f ?x ?y ?z))
  :assert (val ?o ?z)
  :why    "{?g}: {?f} of {?x} and {?y} is {?z}, on {?o}")
```

```dot
digraph rule_propagate_lhs_rhs {
  rankdir=TB;
  label="propagate"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?g_L" [label="?g", shape=diamond];
    "?f_L" [label="?f", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?o_L" [label="?o", shape=diamond];
    "?x_L" [label="?x", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "?z_L" [label="?z", shape=diamond];
    "h1_gate_L" [shape=octagon, label="gate", color="#2ca02c", fontcolor="#2ca02c"];
    "h1_gate_L" -> "?g_L" [label="1", color="#2ca02c"];
    "h1_gate_L" -> "?f_L" [label="2", color="#2ca02c"];
    "h1_gate_L" -> "?a_L" [label="3", color="#2ca02c"];
    "h1_gate_L" -> "?b_L" [label="4", color="#2ca02c"];
    "h1_gate_L" -> "?o_L" [label="5", color="#2ca02c"];
    "?a_L" -> "?x_L" [label="val", color="#1f77b4", fontcolor="#1f77b4"];
    "?b_L" -> "?y_L" [label="val", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_gate2_L" [shape=octagon, label="gate2", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h2_gate2_L" -> "?f_L" [label="1", color="#7f7f7f"];
    "h2_gate2_L" -> "?x_L" [label="2", color="#7f7f7f"];
    "h2_gate2_L" -> "?y_L" [label="3", color="#7f7f7f"];
    "h2_gate2_L" -> "?z_L" [label="4", color="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?o_R" [label="?o", shape=diamond];
    "?z_R" [label="?z", shape=diamond];
    "?o_R" -> "?z_R" [label="val", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `one-value`

<!-- rule std.bool.circuit one-value -->
```lisp
(rule one-value ()
  :match  (and (val ?w ?x) (val ?w ?y) (neq ?x ?y))
  :assert (false)
  :why    "the wire {?w} carries both {?x} and {?y}")
```

```dot
digraph rule_one_value_lhs_rhs {
  rankdir=TB;
  label="one-value"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?w_L" [label="?w", shape=diamond];
    "?x_L" [label="?x", shape=diamond];
    "?y_L" [label="?y", shape=diamond];
    "?w_L" -> "?x_L" [label="val", color="#1f77b4", fontcolor="#1f77b4"];
    "?w_L" -> "?y_L" [label="val", color="#1f77b4", fontcolor="#1f77b4"];
    "?x_L" -> "?y_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
  }
  subgraph cluster_rhs { label="assert";
    "h1_false_R" [shape=octagon, label="false", color="#2ca02c", fontcolor="#2ca02c"];
  }
}
```
<!-- /rule -->

#### `circuit`

<!-- rule std.bool.circuit circuit -->
```lisp
(rule circuit ()
  :match  ()
  :assert (and (__closed__ gate) (__closed__ val) (__closed__ circuit-rule)
               (circuit-rule propagate) (circuit-rule one-value) (circuit-rule gates))
  :why    "a circuit is simulated and never guessed: every relation of the theory is closed")
```

```dot
digraph rule_circuit_lhs_rhs {
  rankdir=TB;
  label="circuit"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "h1_#empty_L" [shape=octagon, label="#empty", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "gate_R" [label="gate", shape=rectangle];
    "val_R" [label="val", shape=rectangle];
    "circuit-rule_R" [label="circuit-rule", shape=rectangle];
    "propagate_R" [label="propagate", shape=rectangle];
    "one-value_R" [label="one-value", shape=rectangle];
    "gates_R" [label="gates", shape=rectangle];
    "h2___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h2___closed___R" -> "gate_R" [label="1", color="#e377c2", style=dashed];
    "h3___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h3___closed___R" -> "val_R" [label="1", color="#e377c2", style=dashed];
    "h4___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h4___closed___R" -> "circuit-rule_R" [label="1", color="#e377c2", style=dashed];
    "h5_circuit-rule_R" [shape=octagon, label="circuit-rule", color="#17becf", fontcolor="#17becf"];
    "h5_circuit-rule_R" -> "propagate_R" [label="1", color="#17becf", style=dashed];
    "h6_circuit-rule_R" [shape=octagon, label="circuit-rule", color="#17becf", fontcolor="#17becf"];
    "h6_circuit-rule_R" -> "one-value_R" [label="1", color="#17becf", style=dashed];
    "h7_circuit-rule_R" [shape=octagon, label="circuit-rule", color="#17becf", fontcolor="#17becf"];
    "h7_circuit-rule_R" -> "gates_R" [label="1", color="#17becf", style=dashed];
  }
}
```
<!-- /rule -->

## `std.bool.inputs`

`in-drives` puts a hypothesis `(in WIRE BIT)` on the wire as a value. `required` refutes a state in which a
required wire carries the other bit. That an input has exactly one value is not a rule of this module: it writes
two facts of `std.algebra`'s, `(total-owed in is-a)` and `(functional in)`, and the
[obligation](03-relation-algebra-rules.md#the-totality-obligations) and the
[check](03-relation-algebra-rules.md#the-cardinality-checks) they activate do the rest.

#### `in-drives`

<!-- rule std.bool.inputs in-drives -->
```lisp
(rule in-drives ()
  :match  (in ?w ?v)
  :assert (val ?w ?v)
  :why    "the input {?w} is set to {?v}")
```

```dot
digraph rule_in_drives_lhs_rhs {
  rankdir=TB;
  label="in-drives"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?w_L" [label="?w", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?w_L" -> "?v_L" [label="in", color="#7f7f7f", fontcolor="#7f7f7f"];
  }
  subgraph cluster_rhs { label="assert";
    "?w_R" [label="?w", shape=diamond];
    "?v_R" [label="?v", shape=diamond];
    "?w_R" -> "?v_R" [label="val", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `required`

<!-- rule std.bool.inputs required -->
```lisp
(rule required ()
  :match  (and (require ?w ?v) (val ?w ?u) (neq ?u ?v))
  :assert (false)
  :why    "{?w} is required to carry {?v} and carries {?u}")
```

```dot
digraph rule_required_lhs_rhs {
  rankdir=TB;
  label="required"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?w_L" [label="?w", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?u_L" [label="?u", shape=diamond];
    "?w_L" -> "?v_L" [label="require", color="#d62728", fontcolor="#d62728"];
    "?w_L" -> "?u_L" [label="val", color="#1f77b4", fontcolor="#1f77b4"];
    "?u_L" -> "?v_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
  }
  subgraph cluster_rhs { label="assert";
    "h1_false_R" [shape=octagon, label="false", color="#2ca02c", fontcolor="#2ca02c"];
  }
}
```
<!-- /rule -->

#### `inputs`

<!-- rule std.bool.inputs inputs -->
```lisp
(rule inputs ()
  :match  ()
  :assert (and (__closed__ is-a) (__closed__ require) (__closed__ inputs-rule)
               (inputs-rule in-drives) (inputs-rule required) (inputs-rule circuit))
  :why    "types and requirements are written, never guessed")
```

```dot
digraph rule_inputs_lhs_rhs {
  rankdir=TB;
  label="inputs"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "h1_#empty_L" [shape=octagon, label="#empty", color="#17becf", fontcolor="#17becf"];
  }
  subgraph cluster_rhs { label="assert";
    "is-a_R" [label="is-a", shape=rectangle];
    "require_R" [label="require", shape=rectangle];
    "inputs-rule_R" [label="inputs-rule", shape=rectangle];
    "in-drives_R" [label="in-drives", shape=rectangle];
    "required_R" [label="required", shape=rectangle];
    "circuit_R" [label="circuit", shape=rectangle];
    "h2___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h2___closed___R" -> "is-a_R" [label="1", color="#e377c2", style=dashed];
    "h3___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h3___closed___R" -> "require_R" [label="1", color="#e377c2", style=dashed];
    "h4___closed___R" [shape=octagon, label="__closed__", color="#e377c2", fontcolor="#e377c2"];
    "h4___closed___R" -> "inputs-rule_R" [label="1", color="#e377c2", style=dashed];
    "h5_inputs-rule_R" [shape=octagon, label="inputs-rule", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h5_inputs-rule_R" -> "in-drives_R" [label="1", color="#ff7f0e", style=dashed];
    "h6_inputs-rule_R" [shape=octagon, label="inputs-rule", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h6_inputs-rule_R" -> "required_R" [label="1", color="#ff7f0e", style=dashed];
    "h7_inputs-rule_R" [shape=octagon, label="inputs-rule", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h7_inputs-rule_R" -> "circuit_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->
