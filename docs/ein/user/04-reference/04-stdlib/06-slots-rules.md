# The slots' rules

How `std.slots` works: every rule of the module, as it writes it and as the engine draws it. What a program
writes to use it is [the page before](05-slots.md); how to read a drawing is
[the index](README.md#how-a-rule-is-shown).

The parameters every rule shares: `?R` is the co-location relation, `?isa` the membership relation, `?sub` the
relation that puts a label under `?super`, `?index` the label whose members name the slots; in the spatial rules
`?S` is the relation that holds between slots and between values, and `?PT` the label of the positions.

## The setup

Two rules with no parameter read the two declarations a program writes and derive the activators of the others.

#### `slot-partition-setup`

Fans `(slot-partition R isa sub Super Index)` out into the activators of the partition rules.

<!-- rule std.slots slot-partition-setup -->
```lisp
(rule slot-partition-setup ()
  :match  (slot-partition ?R ?isa ?sub ?super ?index)
  :assert (and (slot-locate      ?R ?isa ?index)
               (slot-exclusive   ?R ?isa ?sub ?super)
               (slot-occupied    ?R ?isa ?sub ?super)
               (slot-negative    ?R ?isa ?index)
               (slot-elimination ?R ?isa ?sub ?super ?index)
               (slot-fill        ?R ?isa ?sub ?super ?index)
               (slot-no-room     ?R ?isa ?sub ?super ?index)
               (slot-no-fill     ?R ?isa ?sub ?super ?index)
               (slot-owed-room   ?R ?isa ?sub ?super ?index)
               (slot-owed-fill   ?R ?isa ?sub ?super ?index))
  :why    "(slot-partition {?R} …) ⟹ locate / exclusion / occupancy / negative-propagation / elimination / fill / no-room / no-fill / owed-room / owed-fill activators.")
```

```dot
digraph rule_slot_partition_setup_lhs_rhs {
  rankdir=TB;
  label="slot-partition-setup"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?R_L" [label="?R", shape=diamond];
    "?isa_L" [label="?isa", shape=diamond];
    "?sub_L" [label="?sub", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "h1_slot-partition_L" [shape=octagon, label="slot-partition", color="#9467bd", fontcolor="#9467bd"];
    "h1_slot-partition_L" -> "?R_L" [label="1", color="#9467bd"];
    "h1_slot-partition_L" -> "?isa_L" [label="2", color="#9467bd"];
    "h1_slot-partition_L" -> "?sub_L" [label="3", color="#9467bd"];
    "h1_slot-partition_L" -> "?super_L" [label="4", color="#9467bd"];
    "h1_slot-partition_L" -> "?index_L" [label="5", color="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?R_R" [label="?R", shape=diamond];
    "?isa_R" [label="?isa", shape=diamond];
    "?index_R" [label="?index", shape=diamond];
    "?sub_R" [label="?sub", shape=diamond];
    "?super_R" [label="?super", shape=diamond];
    "h2_slot-locate_R" [shape=octagon, label="slot-locate", color="#8c564b", fontcolor="#8c564b"];
    "h2_slot-locate_R" -> "?R_R" [label="1", color="#8c564b", style=dashed];
    "h2_slot-locate_R" -> "?isa_R" [label="2", color="#8c564b", style=dashed];
    "h2_slot-locate_R" -> "?index_R" [label="3", color="#8c564b", style=dashed];
    "h3_slot-exclusive_R" [shape=octagon, label="slot-exclusive", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h3_slot-exclusive_R" -> "?R_R" [label="1", color="#7f7f7f", style=dashed];
    "h3_slot-exclusive_R" -> "?isa_R" [label="2", color="#7f7f7f", style=dashed];
    "h3_slot-exclusive_R" -> "?sub_R" [label="3", color="#7f7f7f", style=dashed];
    "h3_slot-exclusive_R" -> "?super_R" [label="4", color="#7f7f7f", style=dashed];
    "h4_slot-occupied_R" [shape=octagon, label="slot-occupied", color="#17becf", fontcolor="#17becf"];
    "h4_slot-occupied_R" -> "?R_R" [label="1", color="#17becf", style=dashed];
    "h4_slot-occupied_R" -> "?isa_R" [label="2", color="#17becf", style=dashed];
    "h4_slot-occupied_R" -> "?sub_R" [label="3", color="#17becf", style=dashed];
    "h4_slot-occupied_R" -> "?super_R" [label="4", color="#17becf", style=dashed];
    "h5_slot-negative_R" [shape=octagon, label="slot-negative", color="#e377c2", fontcolor="#e377c2"];
    "h5_slot-negative_R" -> "?R_R" [label="1", color="#e377c2", style=dashed];
    "h5_slot-negative_R" -> "?isa_R" [label="2", color="#e377c2", style=dashed];
    "h5_slot-negative_R" -> "?index_R" [label="3", color="#e377c2", style=dashed];
    "h6_slot-elimination_R" [shape=octagon, label="slot-elimination", color="#8c564b", fontcolor="#8c564b"];
    "h6_slot-elimination_R" -> "?R_R" [label="1", color="#8c564b", style=dashed];
    "h6_slot-elimination_R" -> "?isa_R" [label="2", color="#8c564b", style=dashed];
    "h6_slot-elimination_R" -> "?sub_R" [label="3", color="#8c564b", style=dashed];
    "h6_slot-elimination_R" -> "?super_R" [label="4", color="#8c564b", style=dashed];
    "h6_slot-elimination_R" -> "?index_R" [label="5", color="#8c564b", style=dashed];
    "h7_slot-fill_R" [shape=octagon, label="slot-fill", color="#1f77b4", fontcolor="#1f77b4"];
    "h7_slot-fill_R" -> "?R_R" [label="1", color="#1f77b4", style=dashed];
    "h7_slot-fill_R" -> "?isa_R" [label="2", color="#1f77b4", style=dashed];
    "h7_slot-fill_R" -> "?sub_R" [label="3", color="#1f77b4", style=dashed];
    "h7_slot-fill_R" -> "?super_R" [label="4", color="#1f77b4", style=dashed];
    "h7_slot-fill_R" -> "?index_R" [label="5", color="#1f77b4", style=dashed];
    "h8_slot-no-room_R" [shape=octagon, label="slot-no-room", color="#8c564b", fontcolor="#8c564b"];
    "h8_slot-no-room_R" -> "?R_R" [label="1", color="#8c564b", style=dashed];
    "h8_slot-no-room_R" -> "?isa_R" [label="2", color="#8c564b", style=dashed];
    "h8_slot-no-room_R" -> "?sub_R" [label="3", color="#8c564b", style=dashed];
    "h8_slot-no-room_R" -> "?super_R" [label="4", color="#8c564b", style=dashed];
    "h8_slot-no-room_R" -> "?index_R" [label="5", color="#8c564b", style=dashed];
    "h9_slot-no-fill_R" [shape=octagon, label="slot-no-fill", color="#8c564b", fontcolor="#8c564b"];
    "h9_slot-no-fill_R" -> "?R_R" [label="1", color="#8c564b", style=dashed];
    "h9_slot-no-fill_R" -> "?isa_R" [label="2", color="#8c564b", style=dashed];
    "h9_slot-no-fill_R" -> "?sub_R" [label="3", color="#8c564b", style=dashed];
    "h9_slot-no-fill_R" -> "?super_R" [label="4", color="#8c564b", style=dashed];
    "h9_slot-no-fill_R" -> "?index_R" [label="5", color="#8c564b", style=dashed];
    "h10_slot-owed-room_R" [shape=octagon, label="slot-owed-room", color="#8c564b", fontcolor="#8c564b"];
    "h10_slot-owed-room_R" -> "?R_R" [label="1", color="#8c564b", style=dashed];
    "h10_slot-owed-room_R" -> "?isa_R" [label="2", color="#8c564b", style=dashed];
    "h10_slot-owed-room_R" -> "?sub_R" [label="3", color="#8c564b", style=dashed];
    "h10_slot-owed-room_R" -> "?super_R" [label="4", color="#8c564b", style=dashed];
    "h10_slot-owed-room_R" -> "?index_R" [label="5", color="#8c564b", style=dashed];
    "h11_slot-owed-fill_R" [shape=octagon, label="slot-owed-fill", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h11_slot-owed-fill_R" -> "?R_R" [label="1", color="#ff7f0e", style=dashed];
    "h11_slot-owed-fill_R" -> "?isa_R" [label="2", color="#ff7f0e", style=dashed];
    "h11_slot-owed-fill_R" -> "?sub_R" [label="3", color="#ff7f0e", style=dashed];
    "h11_slot-owed-fill_R" -> "?super_R" [label="4", color="#ff7f0e", style=dashed];
    "h11_slot-owed-fill_R" -> "?index_R" [label="5", color="#ff7f0e", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-spatial-setup`

Fans `(slot-spatial R S isa PT)` out into the activators of the spatial rules.

<!-- rule std.slots slot-spatial-setup -->
```lisp
(rule slot-spatial-setup ()
  :match  (slot-spatial ?R ?S ?isa ?PT)
  :assert (and (slot-adjacent-fwd     ?R ?S ?isa ?PT)
               (slot-adjacent-bwd     ?R ?S ?isa ?PT)
               (slot-adjacent-fwd-neg ?R ?S ?isa ?PT)
               (slot-adjacent-bwd-neg ?R ?S ?isa ?PT)
               (slot-prune-fwd        ?R ?S ?isa ?PT)
               (slot-prune-bwd        ?R ?S ?isa ?PT)
               (slot-endpoint-fwd     ?R ?S ?isa ?PT)
               (slot-endpoint-bwd     ?R ?S ?isa ?PT))
  :why    "(slot-spatial {?R} {?S} …) ⟹ the eight directional propagation / pruning / endpoint activators.")
```

```dot
digraph rule_slot_spatial_setup_lhs_rhs {
  rankdir=TB;
  label="slot-spatial-setup"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?R_L" [label="?R", shape=diamond];
    "?S_L" [label="?S", shape=diamond];
    "?isa_L" [label="?isa", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "h1_slot-spatial_L" [shape=octagon, label="slot-spatial", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_slot-spatial_L" -> "?R_L" [label="1", color="#ff7f0e"];
    "h1_slot-spatial_L" -> "?S_L" [label="2", color="#ff7f0e"];
    "h1_slot-spatial_L" -> "?isa_L" [label="3", color="#ff7f0e"];
    "h1_slot-spatial_L" -> "?PT_L" [label="4", color="#ff7f0e"];
  }
  subgraph cluster_rhs { label="assert";
    "?R_R" [label="?R", shape=diamond];
    "?S_R" [label="?S", shape=diamond];
    "?isa_R" [label="?isa", shape=diamond];
    "?PT_R" [label="?PT", shape=diamond];
    "h2_slot-adjacent-fwd_R" [shape=octagon, label="slot-adjacent-fwd", color="#bcbd22", fontcolor="#bcbd22"];
    "h2_slot-adjacent-fwd_R" -> "?R_R" [label="1", color="#bcbd22", style=dashed];
    "h2_slot-adjacent-fwd_R" -> "?S_R" [label="2", color="#bcbd22", style=dashed];
    "h2_slot-adjacent-fwd_R" -> "?isa_R" [label="3", color="#bcbd22", style=dashed];
    "h2_slot-adjacent-fwd_R" -> "?PT_R" [label="4", color="#bcbd22", style=dashed];
    "h3_slot-adjacent-bwd_R" [shape=octagon, label="slot-adjacent-bwd", color="#bcbd22", fontcolor="#bcbd22"];
    "h3_slot-adjacent-bwd_R" -> "?R_R" [label="1", color="#bcbd22", style=dashed];
    "h3_slot-adjacent-bwd_R" -> "?S_R" [label="2", color="#bcbd22", style=dashed];
    "h3_slot-adjacent-bwd_R" -> "?isa_R" [label="3", color="#bcbd22", style=dashed];
    "h3_slot-adjacent-bwd_R" -> "?PT_R" [label="4", color="#bcbd22", style=dashed];
    "h4_slot-adjacent-fwd-neg_R" [shape=octagon, label="slot-adjacent-fwd-neg", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h4_slot-adjacent-fwd-neg_R" -> "?R_R" [label="1", color="#ff7f0e", style=dashed];
    "h4_slot-adjacent-fwd-neg_R" -> "?S_R" [label="2", color="#ff7f0e", style=dashed];
    "h4_slot-adjacent-fwd-neg_R" -> "?isa_R" [label="3", color="#ff7f0e", style=dashed];
    "h4_slot-adjacent-fwd-neg_R" -> "?PT_R" [label="4", color="#ff7f0e", style=dashed];
    "h5_slot-adjacent-bwd-neg_R" [shape=octagon, label="slot-adjacent-bwd-neg", color="#bcbd22", fontcolor="#bcbd22"];
    "h5_slot-adjacent-bwd-neg_R" -> "?R_R" [label="1", color="#bcbd22", style=dashed];
    "h5_slot-adjacent-bwd-neg_R" -> "?S_R" [label="2", color="#bcbd22", style=dashed];
    "h5_slot-adjacent-bwd-neg_R" -> "?isa_R" [label="3", color="#bcbd22", style=dashed];
    "h5_slot-adjacent-bwd-neg_R" -> "?PT_R" [label="4", color="#bcbd22", style=dashed];
    "h6_slot-prune-fwd_R" [shape=octagon, label="slot-prune-fwd", color="#e377c2", fontcolor="#e377c2"];
    "h6_slot-prune-fwd_R" -> "?R_R" [label="1", color="#e377c2", style=dashed];
    "h6_slot-prune-fwd_R" -> "?S_R" [label="2", color="#e377c2", style=dashed];
    "h6_slot-prune-fwd_R" -> "?isa_R" [label="3", color="#e377c2", style=dashed];
    "h6_slot-prune-fwd_R" -> "?PT_R" [label="4", color="#e377c2", style=dashed];
    "h7_slot-prune-bwd_R" [shape=octagon, label="slot-prune-bwd", color="#7f7f7f", fontcolor="#7f7f7f"];
    "h7_slot-prune-bwd_R" -> "?R_R" [label="1", color="#7f7f7f", style=dashed];
    "h7_slot-prune-bwd_R" -> "?S_R" [label="2", color="#7f7f7f", style=dashed];
    "h7_slot-prune-bwd_R" -> "?isa_R" [label="3", color="#7f7f7f", style=dashed];
    "h7_slot-prune-bwd_R" -> "?PT_R" [label="4", color="#7f7f7f", style=dashed];
    "h8_slot-endpoint-fwd_R" [shape=octagon, label="slot-endpoint-fwd", color="#2ca02c", fontcolor="#2ca02c"];
    "h8_slot-endpoint-fwd_R" -> "?R_R" [label="1", color="#2ca02c", style=dashed];
    "h8_slot-endpoint-fwd_R" -> "?S_R" [label="2", color="#2ca02c", style=dashed];
    "h8_slot-endpoint-fwd_R" -> "?isa_R" [label="3", color="#2ca02c", style=dashed];
    "h8_slot-endpoint-fwd_R" -> "?PT_R" [label="4", color="#2ca02c", style=dashed];
    "h9_slot-endpoint-bwd_R" [shape=octagon, label="slot-endpoint-bwd", color="#d62728", fontcolor="#d62728"];
    "h9_slot-endpoint-bwd_R" -> "?R_R" [label="1", color="#d62728", style=dashed];
    "h9_slot-endpoint-bwd_R" -> "?S_R" [label="2", color="#d62728", style=dashed];
    "h9_slot-endpoint-bwd_R" -> "?isa_R" [label="3", color="#d62728", style=dashed];
    "h9_slot-endpoint-bwd_R" -> "?PT_R" [label="4", color="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

## The partition rules

### Placing

#### `slot-locate`

Transitivity with its conclusion pinned to the slot's index: a clue between two values becomes a placement of the
first.

<!-- rule std.slots slot-locate -->
```lisp
(rule slot-locate (?R ?isa ?index)
  :match  (and (?R ?a ?b)
               (?R ?b ?i)
               (?isa ?i ?index)
               (neq ?a ?i))
  :assert (?R ?a ?i)
  :why    "{?a} shares a slot with {?b}, and {?b} is at {?i} ⟹ {?a} is at {?i}.")
```

```dot
digraph rule_slot_locate_lhs_rhs {
  rankdir=TB;
  label="slot-locate"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?i_L" [label="?i", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "?a_L" -> "?b_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "?b_L" -> "?i_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "?i_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?a_L" -> "?i_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?i_R" [label="?i", shape=diamond];
    "?a_R" -> "?i_R" [label="?R", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

### The negatives a placement implies

#### `slot-exclusive`

Two distinct members of one label are never in one slot.

<!-- rule std.slots slot-exclusive -->
```lisp
(rule slot-exclusive (?R ?isa ?sub ?super)
  :match  (and (?isa ?a ?T)
               (?sub ?T ?super)
               (?isa ?b ?T)
               (neq ?a ?b))
  :assert (not (?R ?a ?b))
  :why    "{?a} and {?b} are distinct {?T}s — they cannot share a slot under {?R}.")
```

```dot
digraph rule_slot_exclusive_lhs_rhs {
  rankdir=TB;
  label="slot-exclusive"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?T_L" [label="?T", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?a_L" -> "?T_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?T_L" -> "?super_L" [label="?sub", color="#9467bd", fontcolor="#9467bd"];
    "?b_L" -> "?T_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?a_L" -> "?b_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?a_R" -> "?b_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-occupied`

A value co-located with a member of a label is co-located with no other member of it.

<!-- rule std.slots slot-occupied -->
```lisp
(rule slot-occupied (?R ?isa ?sub ?super)
  :match  (and (?R ?a ?b)
               (?isa ?b ?T)
               (?sub ?T ?super)
               (?isa ?c ?T)
               (neq ?c ?b))
  :assert (not (?R ?a ?c))
  :why    "{?a}'s slot already holds the {?T} {?b} ⟹ it does not hold {?c}.")
```

```dot
digraph rule_slot_occupied_lhs_rhs {
  rankdir=TB;
  label="slot-occupied"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?T_L" [label="?T", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?c_L" [label="?c", shape=diamond];
    "?a_L" -> "?b_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "?b_L" -> "?T_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?T_L" -> "?super_L" [label="?sub", color="#9467bd", fontcolor="#9467bd"];
    "?c_L" -> "?T_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?c_L" -> "?b_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?c_R" [label="?c", shape=diamond];
    "?a_R" -> "?c_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-negative`

A value co-located with something excluded from a slot is excluded from that slot.

<!-- rule std.slots slot-negative -->
```lisp
(rule slot-negative (?R ?isa ?index)
  :match  (and (?R ?a ?b)
               (not (?R ?b ?i))
               (?isa ?i ?index))
  :assert (not (?R ?a ?i))
  :why    "{?a} shares a slot with {?b}, and {?b} cannot be at {?i} ⟹ neither can {?a}.")
```

```dot
digraph rule_slot_negative_lhs_rhs {
  rankdir=TB;
  label="slot-negative"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?i_L" [label="?i", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "?a_L" -> "?b_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "?b_L" -> "?i_L" [label="¬?R", color="#d62728", fontcolor="#d62728"];
    "?i_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?i_R" [label="?i", shape=diamond];
    "?a_R" -> "?i_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

### The last candidate left

Both are written with `std.macro`'s `forall`: every other candidate is stated excluded.

#### `slot-elimination`

The one index left for a value is its slot.

<!-- rule std.slots slot-elimination -->
```lisp
(rule slot-elimination (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?a ?Ta)
               (?sub ?Ta ?super)
               (neq ?Ta ?index)
               (?isa ?i ?index)
               (forall ?i_other
                 (and (?isa ?i_other ?index) (neq ?i_other ?i))
                 (not (?R ?a ?i_other))))
  :assert (?R ?a ?i)
  :why    "every slot except {?i} is excluded for {?a} ⟹ {?a} is at {?i}.")
```

```dot
digraph rule_slot_elimination_lhs_rhs {
  rankdir=TB;
  label="slot-elimination"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?Ta_L" [label="?Ta", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "?i_L" [label="?i", shape=diamond];
    "?a_L" -> "?Ta_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?Ta_L" -> "?super_L" [label="?sub", color="#9467bd", fontcolor="#9467bd"];
    "?Ta_L" -> "?index_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    "?i_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_forall_L_0 {
      label="forall (∀)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?i_other_L" [label="?i_other", shape=diamond];
      "?i_other_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?i_other_L" -> "?i_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
      "?a_L" -> "?i_other_L" [label="¬?R", color="#d62728", fontcolor="#d62728"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?i_R" [label="?i", shape=diamond];
    "?a_R" -> "?i_R" [label="?R", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-fill`

The one member of a label left for a slot fills it.

<!-- rule std.slots slot-fill -->
```lisp
(rule slot-fill (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?i ?index)
               (?sub ?Tv ?super)
               (neq ?Tv ?index)
               (?isa ?v ?Tv)
               (forall ?v_other
                 (and (?isa ?v_other ?Tv) (neq ?v_other ?v))
                 (not (?R ?i ?v_other))))
  :assert (?R ?i ?v)
  :why    "every {?Tv} except {?v} is excluded for slot {?i} ⟹ {?v} is at {?i}.")
```

```dot
digraph rule_slot_fill_lhs_rhs {
  rankdir=TB;
  label="slot-fill"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?i_L" [label="?i", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "?Tv_L" [label="?Tv", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?v_L" [label="?v", shape=diamond];
    "?i_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?Tv_L" -> "?super_L" [label="?sub", color="#9467bd", fontcolor="#9467bd"];
    "?Tv_L" -> "?index_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    "?v_L" -> "?Tv_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_forall_L_0 {
      label="forall (∀)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?v_other_L" [label="?v_other", shape=diamond];
      "?v_other_L" -> "?Tv_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?v_other_L" -> "?v_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
      "?i_L" -> "?v_other_L" [label="¬?R", color="#d62728", fontcolor="#d62728"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?i_R" [label="?i", shape=diamond];
    "?v_R" [label="?v", shape=diamond];
    "?i_R" -> "?v_R" [label="?R", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

### Nothing left

#### `slot-no-room`

A value with every slot excluded is a contradiction.

<!-- rule std.slots slot-no-room -->
```lisp
(rule slot-no-room (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?a ?Ta)
               (?sub ?Ta ?super)
               (neq ?Ta ?index)
               (forall ?i (?isa ?i ?index) (not (?R ?a ?i))))
  :assert (false)
  :why    "{?a} has every slot excluded — there is nowhere left for it.")
```

```dot
digraph rule_slot_no_room_lhs_rhs {
  rankdir=TB;
  label="slot-no-room"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?Ta_L" [label="?Ta", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "?a_L" -> "?Ta_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?Ta_L" -> "?super_L" [label="?sub", color="#9467bd", fontcolor="#9467bd"];
    "?Ta_L" -> "?index_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    subgraph cluster_forall_L_0 {
      label="forall (∀)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?i_L" [label="?i", shape=diamond];
      "?i_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?a_L" -> "?i_L" [label="¬?R", color="#d62728", fontcolor="#d62728"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "h1_false_R" [shape=octagon, label="false", color="#2ca02c", fontcolor="#2ca02c"];
  }
}
```
<!-- /rule -->

#### `slot-no-fill`

A slot with every member of a label excluded is a contradiction.

<!-- rule std.slots slot-no-fill -->
```lisp
(rule slot-no-fill (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?i ?index)
               (?sub ?Tv ?super)
               (neq ?Tv ?index)
               (forall ?v (?isa ?v ?Tv) (not (?R ?i ?v))))
  :assert (false)
  :why    "slot {?i} has every {?Tv} excluded — it has no {?Tv} left.")
```

```dot
digraph rule_slot_no_fill_lhs_rhs {
  rankdir=TB;
  label="slot-no-fill"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?i_L" [label="?i", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "?Tv_L" [label="?Tv", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?i_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?Tv_L" -> "?super_L" [label="?sub", color="#9467bd", fontcolor="#9467bd"];
    "?Tv_L" -> "?index_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    subgraph cluster_forall_L_0 {
      label="forall (∀)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?v_L" [label="?v", shape=diamond];
      "?v_L" -> "?Tv_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?i_L" -> "?v_L" [label="¬?R", color="#d62728", fontcolor="#d62728"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "h1_false_R" [shape=octagon, label="false", color="#2ca02c", fontcolor="#2ca02c"];
  }
}
```
<!-- /rule -->

### What is still owed

The obligations of the two rules above: the same question where nothing is stated yet. Each concludes `(open ?R)`,
derives nothing, and is read once over a state that has stopped changing.

#### `slot-owed-room`

A value in no slot yet: the relation owes it one.

<!-- rule std.slots slot-owed-room -->
```lisp
(rule slot-owed-room (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?a ?Ta)
               (?sub ?Ta ?super)
               (neq ?Ta ?index)
               (absent (and (?isa ?i ?index) (?R ?a ?i))))
  :assert (open ?R)
  :why    "{?a} is in no slot yet — some {?index} is still owed.")
```

```dot
digraph rule_slot_owed_room_lhs_rhs {
  rankdir=TB;
  label="slot-owed-room"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?Ta_L" [label="?Ta", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "?a_L" -> "?Ta_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?Ta_L" -> "?super_L" [label="?sub", color="#9467bd", fontcolor="#9467bd"];
    "?Ta_L" -> "?index_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?i_L" [label="?i", shape=diamond];
      "?i_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?a_L" -> "?i_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?R_R" [label="?R", shape=diamond];
    "h1_open_R" [shape=octagon, label="open", color="#2ca02c", fontcolor="#2ca02c"];
    "h1_open_R" -> "?R_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-owed-fill`

A slot with no member of a label yet: the relation owes it one.

<!-- rule std.slots slot-owed-fill -->
```lisp
(rule slot-owed-fill (?R ?isa ?sub ?super ?index)
  :match  (and (?isa ?i ?index)
               (?sub ?Tv ?super)
               (neq ?Tv ?index)
               (absent (and (?isa ?v ?Tv) (?R ?i ?v))))
  :assert (open ?R)
  :why    "slot {?i} has no {?Tv} yet — one is still owed.")
```

```dot
digraph rule_slot_owed_fill_lhs_rhs {
  rankdir=TB;
  label="slot-owed-fill"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?i_L" [label="?i", shape=diamond];
    "?index_L" [label="?index", shape=diamond];
    "?Tv_L" [label="?Tv", shape=diamond];
    "?super_L" [label="?super", shape=diamond];
    "?i_L" -> "?index_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?Tv_L" -> "?super_L" [label="?sub", color="#9467bd", fontcolor="#9467bd"];
    "?Tv_L" -> "?index_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?v_L" [label="?v", shape=diamond];
      "?v_L" -> "?Tv_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?i_L" -> "?v_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?R_R" [label="?R", shape=diamond];
    "h1_open_R" [shape=octagon, label="open", color="#2ca02c", fontcolor="#2ca02c"];
    "h1_open_R" -> "?R_R" [label="1", color="#2ca02c", style=dashed];
  }
}
```
<!-- /rule -->

## The spatial rules

Each reads a clue `(?S a b)` between two values, and the structure `(?S p q)` between two positions, which it
tells from a clue by `(?isa p ?PT)`. The `absent` clusters read the structure alone.

### A neighbour that is the only one

#### `slot-adjacent-fwd`

With `b` at a position whose only source under `?S` is `p2`, the value `a` is at `p2`.

<!-- rule std.slots slot-adjacent-fwd -->
```lisp
(rule slot-adjacent-fwd (?R ?S ?isa ?PT)
  :match  (and (?S ?a ?b)
               (?R ?b ?p1) (?isa ?p1 ?PT)
               (?S ?p2 ?p1) (?isa ?p2 ?PT)
               (absent (and (?S ?p_o ?p1) (?isa ?p_o ?PT) (neq ?p_o ?p2))))
  :assert (?R ?a ?p2)
  :why    "{?b} is at {?p1}, whose only {?S}-source is {?p2}, and {?S}({?a},{?b}) ⟹ {?a} is at {?p2}.")
```

```dot
digraph rule_slot_adjacent_fwd_lhs_rhs {
  rankdir=TB;
  label="slot-adjacent-fwd"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?p1_L" [label="?p1", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "?p2_L" [label="?p2", shape=diamond];
    "?a_L" -> "?b_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?b_L" -> "?p1_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "?p1_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?p2_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?p2_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p_o_L" [label="?p_o", shape=diamond];
      "?p_o_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
      "?p_o_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?p_o_L" -> "?p2_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?p2_R" [label="?p2", shape=diamond];
    "?a_R" -> "?p2_R" [label="?R", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-adjacent-bwd`

The reverse: with `a` at a position whose only target is `p1`, the value `b` is at `p1`.

<!-- rule std.slots slot-adjacent-bwd -->
```lisp
(rule slot-adjacent-bwd (?R ?S ?isa ?PT)
  :match  (and (?S ?a ?b)
               (?R ?a ?p2) (?isa ?p2 ?PT)
               (?S ?p2 ?p1) (?isa ?p1 ?PT)
               (absent (and (?S ?p2 ?p_o) (?isa ?p_o ?PT) (neq ?p_o ?p1))))
  :assert (?R ?b ?p1)
  :why    "{?a} is at {?p2}, whose only {?S}-target is {?p1}, and {?S}({?a},{?b}) ⟹ {?b} is at {?p1}.")
```

```dot
digraph rule_slot_adjacent_bwd_lhs_rhs {
  rankdir=TB;
  label="slot-adjacent-bwd"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?p2_L" [label="?p2", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "?p1_L" [label="?p1", shape=diamond];
    "?a_L" -> "?b_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?a_L" -> "?p2_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "?p2_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?p2_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?p1_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p_o_L" [label="?p_o", shape=diamond];
      "?p2_L" -> "?p_o_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
      "?p_o_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?p_o_L" -> "?p1_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?b_R" [label="?b", shape=diamond];
    "?p1_R" [label="?p1", shape=diamond];
    "?b_R" -> "?p1_R" [label="?R", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-adjacent-fwd-neg`

The contrapositive of the first: `a` excluded from the only source of a position excludes `b` from that position.

<!-- rule std.slots slot-adjacent-fwd-neg -->
```lisp
(rule slot-adjacent-fwd-neg (?R ?S ?isa ?PT)
  :match  (and (?S ?a ?b)
               (not (?R ?a ?p2)) (?isa ?p2 ?PT)
               (?S ?p2 ?p1) (?isa ?p1 ?PT)
               (absent (and (?S ?p_o ?p1) (?isa ?p_o ?PT) (neq ?p_o ?p2))))
  :assert (not (?R ?b ?p1))
  :why    "{?a} cannot be at {?p2}, the only {?S}-source of {?p1} ⟹ {?b} cannot be at {?p1}.")
```

```dot
digraph rule_slot_adjacent_fwd_neg_lhs_rhs {
  rankdir=TB;
  label="slot-adjacent-fwd-neg"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?p2_L" [label="?p2", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "?p1_L" [label="?p1", shape=diamond];
    "?a_L" -> "?b_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?a_L" -> "?p2_L" [label="¬?R", color="#d62728", fontcolor="#d62728"];
    "?p2_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?p2_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?p1_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p_o_L" [label="?p_o", shape=diamond];
      "?p_o_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
      "?p_o_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?p_o_L" -> "?p2_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?b_R" [label="?b", shape=diamond];
    "?p1_R" [label="?p1", shape=diamond];
    "?b_R" -> "?p1_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-adjacent-bwd-neg`

The contrapositive of the second: `b` excluded from the only target of a position excludes `a` from that position.

<!-- rule std.slots slot-adjacent-bwd-neg -->
```lisp
(rule slot-adjacent-bwd-neg (?R ?S ?isa ?PT)
  :match  (and (?S ?a ?b)
               (not (?R ?b ?p1)) (?isa ?p1 ?PT)
               (?S ?p2 ?p1) (?isa ?p2 ?PT)
               (absent (and (?S ?p2 ?p_o) (?isa ?p_o ?PT) (neq ?p_o ?p1))))
  :assert (not (?R ?a ?p2))
  :why    "{?b} cannot be at {?p1}, the only {?S}-target of {?p2} ⟹ {?a} cannot be at {?p2}.")
```

```dot
digraph rule_slot_adjacent_bwd_neg_lhs_rhs {
  rankdir=TB;
  label="slot-adjacent-bwd-neg"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?p1_L" [label="?p1", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "?p2_L" [label="?p2", shape=diamond];
    "?a_L" -> "?b_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?b_L" -> "?p1_L" [label="¬?R", color="#d62728", fontcolor="#d62728"];
    "?p1_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?p2_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?p2_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p_o_L" [label="?p_o", shape=diamond];
      "?p2_L" -> "?p_o_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
      "?p_o_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?p_o_L" -> "?p1_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?p2_R" [label="?p2", shape=diamond];
    "?a_R" -> "?p2_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

### Pruning

#### `slot-prune-fwd`

Once `b` is placed, `a` is excluded from every position that is not a source of `b`'s.

<!-- rule std.slots slot-prune-fwd -->
```lisp
(rule slot-prune-fwd (?R ?S ?isa ?PT)
  :match  (and (?S ?a ?b)
               (?R ?b ?p1) (?isa ?p1 ?PT)
               (?isa ?p_o ?PT)
               (neq ?p_o ?p1)
               (absent (?S ?p_o ?p1)))
  :assert (not (?R ?a ?p_o))
  :why    "{?b} is at {?p1}, so {?a} is at an {?S}-source of {?p1}; {?p_o} is not one.")
```

```dot
digraph rule_slot_prune_fwd_lhs_rhs {
  rankdir=TB;
  label="slot-prune-fwd"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?p1_L" [label="?p1", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "?p_o_L" [label="?p_o", shape=diamond];
    "?a_L" -> "?b_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?b_L" -> "?p1_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "?p1_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?p_o_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?p_o_L" -> "?p1_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p_o_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?p_o_R" [label="?p_o", shape=diamond];
    "?a_R" -> "?p_o_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-prune-bwd`

Once `a` is placed, `b` is excluded from every position that is not a target of `a`'s.

<!-- rule std.slots slot-prune-bwd -->
```lisp
(rule slot-prune-bwd (?R ?S ?isa ?PT)
  :match  (and (?S ?a ?b)
               (?R ?a ?p2) (?isa ?p2 ?PT)
               (?isa ?p_o ?PT)
               (neq ?p_o ?p2)
               (absent (?S ?p2 ?p_o)))
  :assert (not (?R ?b ?p_o))
  :why    "{?a} is at {?p2}, so {?b} is at an {?S}-target of {?p2}; {?p_o} is not one.")
```

```dot
digraph rule_slot_prune_bwd_lhs_rhs {
  rankdir=TB;
  label="slot-prune-bwd"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?p2_L" [label="?p2", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "?p_o_L" [label="?p_o", shape=diamond];
    "?a_L" -> "?b_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?a_L" -> "?p2_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "?p2_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?p_o_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?p_o_L" -> "?p2_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p2_L" -> "?p_o_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?b_R" [label="?b", shape=diamond];
    "?p_o_R" [label="?p_o", shape=diamond];
    "?b_R" -> "?p_o_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

### The ends

#### `slot-endpoint-fwd`

A position with no source under `?S` cannot hold `b`.

<!-- rule std.slots slot-endpoint-fwd -->
```lisp
(rule slot-endpoint-fwd (?R ?S ?isa ?PT)
  :match  (and (?S ?a ?b)
               (?isa ?p1 ?PT)
               (absent (and (?isa ?p2 ?PT) (?S ?p2 ?p1))))
  :assert (not (?R ?b ?p1))
  :why    "{?p1} has no {?S}-source, and {?b} needs one ({?S}({?a},{?b})) ⟹ {?b} is not at {?p1}.")
```

```dot
digraph rule_slot_endpoint_fwd_lhs_rhs {
  rankdir=TB;
  label="slot-endpoint-fwd"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?p1_L" [label="?p1", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "?a_L" -> "?b_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?p1_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p2_L" [label="?p2", shape=diamond];
      "?p2_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?p2_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?b_R" [label="?b", shape=diamond];
    "?p1_R" [label="?p1", shape=diamond];
    "?b_R" -> "?p1_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->

#### `slot-endpoint-bwd`

A position with no target under `?S` cannot hold `a`.

<!-- rule std.slots slot-endpoint-bwd -->
```lisp
(rule slot-endpoint-bwd (?R ?S ?isa ?PT)
  :match  (and (?S ?a ?b)
               (?isa ?p2 ?PT)
               (absent (and (?isa ?p1 ?PT) (?S ?p2 ?p1))))
  :assert (not (?R ?a ?p2))
  :why    "{?p2} has no {?S}-target, and {?a} needs one ({?S}({?a},{?b})) ⟹ {?a} is not at {?p2}.")
```

```dot
digraph rule_slot_endpoint_bwd_lhs_rhs {
  rankdir=TB;
  label="slot-endpoint-bwd"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?p2_L" [label="?p2", shape=diamond];
    "?PT_L" [label="?PT", shape=diamond];
    "?a_L" -> "?b_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    "?p2_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p1_L" [label="?p1", shape=diamond];
      "?p1_L" -> "?PT_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?p2_L" -> "?p1_L" [label="?S", color="#2ca02c", fontcolor="#2ca02c"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?p2_R" [label="?p2", shape=diamond];
    "?a_R" -> "?p2_R" [label="¬?R", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```
<!-- /rule -->
