# The rules: rewriting the graph

A rule rewrites the graph a knowledge base is ([the knowledge base](01-knowledge-base.md)). This page does the
rewriting first: a pattern, which is the same graph with variables in it; a match of a pattern in a knowledge
base; the conditions a match can be held to; a conclusion; and the knowledge base that rewriting at a match
gives. Then it says how a rule is declared, what a declaration must meet, and the kinds of rule. Everything
here is about one knowledge base as it stands. When a rule is applied, in what order, and what a knowledge base
becomes under all its rules is [the part after this one](../02-kb-evolution/01-firing.md).

## A pattern: the graph, with variables

Let $X$ be a set of **variables**, written with a mark that is no name character, so $X \cap \Sigma^{+} =
\emptyset$. A **pattern graph** is a graph of the shape a knowledge base has, with one more sort of node: a node
with no outgoing arrow is an atom, labelled by a name, or a **variable**, labelled by a variable. A node with
outgoing arrows is a **fact pattern**, and the four conditions hold as they do of a knowledge base: one label is
one node, one list of arrows is one node, no path of arrows returns to its start, and a first arrow points at a
node with no outgoing arrow, an atom or a variable. As terms:

$$
t \;::=\; a \;\mid\; x \;\mid\; s(t_1, \dots, t_n), \qquad a \in \Sigma^{+},\quad x \in X,\quad s \in (\Sigma^{+} \setminus W_{\!c}) \cup X
$$

within the shapes the reserved atoms fix: $\lnot t$ is the fact pattern $\operatorname{not}(t)$ with $t$ a fact
pattern that is no negation, and $\bot$ is $\operatorname{false}()$. A fact pattern with no variable under any of
its arrows, at any depth, is a fact. Some of the fact patterns of a pattern graph are its **premises**, as some
of the facts of a knowledge base are held.

The pattern graph below is two premises over three variables:

```dot
digraph pattern {
  rankdir=TB; nodesep=0.4; ranksep=0.6;
  node [shape=box, fontname="Helvetica", fontsize=11, height=0.3];
  edge [fontname="Helvetica", fontsize=10];
  { rank=min; "owns"; "lives-in"; }
  node [shape=diamond];
  { rank=max; "?x"; "?p"; "?h"; }
  node [shape=octagon, label="", width=0.3, height=0.3, fixedsize=true, style=filled, fillcolor="#d9d9d9"];
  q1 [xlabel="q1"]; q2 [xlabel="q2"];
  q1 -> "owns" [label="1"];
  q1 -> "?p" [label="2"];
  q1 -> "?x" [label="3"];
  q2 -> "lives-in" [label="1"];
  q2 -> "?p" [label="2"];
  q2 -> "?h" [label="3"];
}
```

$$
q_1 = \operatorname{owns}(p, x), \qquad q_2 = \operatorname{lives-in}(p, h)
$$

A diamond is a variable. A box is an atom and an octagon a fact pattern, filled where it is a premise, as in a
knowledge base. The variable $p$ is one node, under an arrow of each premise: that the two premises share it is
what the pattern says.

## A match

A **substitution** $\theta$ gives objects to variables, and $t\theta$ is $t$ with each variable replaced by its
object. A **match** of a pattern graph in a knowledge base $K$ is a substitution $\theta$ of its variables under
which every premise is a held fact:

$$
t\theta \in K \qquad \text{for every premise } t
$$

On the graphs, a match is a **homomorphism** $m$ from the pattern graph to the graph of $K$: a map of nodes that
fixes the atoms, keeps the arrows in their order, and sends a premise to a held fact,

$$
m(a) = a, \qquad \operatorname{out}(m(t)) = \big(m(t_1), m(t_2), \dots, m(t_k)\big) \ \text{ where } \operatorname{out}(t) = (t_1, t_2, \dots, t_k), \qquad m(t) \in K \ \text{ for a premise } t
$$

and $m$ is $\theta$ on the variables and $t \mapsto t\theta$ on the fact patterns. Four things follow from the
definition:

- a fact pattern matches only a fact with **as many arrows**: matching is by count;
- a variable under a first arrow is sent to an **atom**, since a first arrow points at one, and a variable under
  another arrow to any object, an atom or a fact;
- a fact pattern that is no premise is sent to a fact that need not be held: the pattern
  $\operatorname{says}(x, \operatorname{owns}(x, y))$ matches into the nested fact, which is how a pattern reads
  what a fact is about;
- $m$ **need not be injective**: two variables may be sent to one object.

The pattern above has exactly one match in the knowledge base of three facts drawn beside it:

```dot
digraph a_match {
  rankdir=TB; nodesep=0.45; ranksep=0.8;
  node [shape=box, fontname="Helvetica", fontsize=11, height=0.3];
  edge [fontname="Helvetica", fontsize=10];
  "owns"; "lives-in"; "left-of";
  subgraph cluster_pattern { label="the pattern"; labelloc=b; fontname="Helvetica"; fontsize=10; style=rounded; color="#888888";
    node [shape=diamond];
    "?x"; "?p"; "?h";
    node [shape=octagon, label="", width=0.3, height=0.3, fixedsize=true, style=filled, fillcolor="#d9d9d9"];
    q1 [xlabel="q1"]; q2 [xlabel="q2"];
  }
  subgraph cluster_kb { label="the knowledge base"; labelloc=b; fontname="Helvetica"; fontsize=10; style=rounded; color="#888888";
    "Dog"; "Norwegian"; "House-1"; "House-2";
    node [shape=octagon, label="", width=0.3, height=0.3, fixedsize=true, style=filled, fillcolor="#d9d9d9"];
    f1 [xlabel="f1"]; f2 [xlabel="f2"]; f3 [xlabel="f3"];
  }
  "owns" -> q1 [label="1", dir=back];
  q1 -> "?p" [label="2"];
  q1 -> "?x" [label="3"];
  "lives-in" -> q2 [label="1", dir=back];
  q2 -> "?p" [label="2"];
  q2 -> "?h" [label="3"];
  "owns" -> f1 [label="1", dir=back];
  f1 -> "Norwegian" [label="2"];
  f1 -> "Dog" [label="3"];
  "lives-in" -> f2 [label="1", dir=back];
  f2 -> "Norwegian" [label="2"];
  f2 -> "House-1" [label="3"];
  "left-of" -> f3 [label="1", dir=back];
  f3 -> "House-1" [label="2"];
  f3 -> "House-2" [label="3"];
  edge [style=dotted, color="#1f77b4", fontcolor="#1f77b4", constraint=false, arrowhead=vee];
  q1 -> f1 [label="m"];
  q2 -> f2 [label="m"];
  "?p" -> "Norwegian" [label="m"];
  "?x" -> "Dog" [label="m"];
  "?h" -> "House-1" [label="m"];
}
```

$$
\theta = \{\, p \mapsto \mathsf{Norwegian},\ x \mapsto \mathsf{Dog},\ h \mapsto \mathsf{House\text{-}1} \,\}, \qquad
m(q_1) = f_1 = \operatorname{owns}(\mathsf{Norwegian}, \mathsf{Dog}), \qquad
m(q_2) = f_2 = \operatorname{lives-in}(\mathsf{Norwegian}, \mathsf{House\text{-}1})
$$

A dotted arrow is the match, drawn for the five nodes it moves; the atoms
$\mathsf{owns}$ and $\mathsf{lives\text{-}in}$ are the same nodes in the pattern and in the knowledge base, and
$m$ fixes them. $f_3$ is no image of the pattern: a match is into a part of the graph.

## The conditions on a match

A **pattern** is premises with conditions. A condition adds no node to what is matched; it restricts which
substitutions count. With $t, t'$ terms and $\gamma$ ranging over the parts of a pattern:

$$
\begin{aligned}
\gamma &\;::=\; t \;\mid\; t = t' \;\mid\; t \neq t' \;\mid\; \operatorname{absent}(\gamma_1 \land \cdots \land \gamma_k) \\
\varphi &\;::=\; \top \;\mid\; \gamma_1 \land \cdots \land \gamma_k \;\mid\; (\gamma_{1,1} \land \cdots) \;\lor\; \cdots \;\lor\; (\gamma_{l,1} \land \cdots)
\end{aligned}
$$

A $t$ that stands as a part is a premise, a fact pattern; in an equality or an inequality a term is a variable,
an atom, or a fact with no variable
([Q-M1f.30](../../../decisions/language/Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md)). A
disjunction is a whole pattern and no part of one
([Q-M1f.74](../../../decisions/language/Q-M1f.74-a-nested-or-a-macros-expansion-writes-is-refused.md)).
$K \models \varphi\theta$, *$\varphi$ holds in $K$ under $\theta$*, is defined on the shape of the pattern:

$$
\begin{aligned}
K &\models \top\theta && \text{always} \\
K &\models t\,\theta && \iff\ t\theta \in K \\
K &\models (t = t')\theta && \iff\ t\theta \text{ and } t'\theta \text{ are one object} \\
K &\models (t \ne t')\theta && \iff\ t\theta \text{ and } t'\theta \text{ are two objects} \\
K &\models (\gamma_1 \land \cdots \land \gamma_k)\theta && \iff\ K \models \gamma_i\theta \ \text{ for every } i \\
K &\models (\varphi_1 \lor \cdots \lor \varphi_l)\theta && \iff\ K \models \varphi_i\theta \ \text{ for some } i \\
K &\models \operatorname{absent}(G)\theta && \iff\ \text{there is no } \theta' \supseteq \theta \text{ with } K \models G\theta'
\end{aligned}
$$

- **The empty pattern** $\top$ holds under the empty substitution and no other: it is matched once
  ([Q-M1e2.36](../../../decisions/language/Q-M1e2.36-an-empty-match-is-true-and-fires-once.md)).
- **A premise** is the homomorphism condition above. A stored negation is nothing new: the premise $\lnot t$ is
  the fact pattern $\operatorname{not}(t)$, and it is sent to a held fact of $\mathsf{not}$ and to nothing
  else.
- **An equality or an inequality** is computed from the two objects and reads no fact: equality is identity, and
  no knowledge base holds one
  ([Q-M1e2.26](../../../decisions/language/Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)).
- **A conjunction** is one pattern graph: the union of its premises, with every condition of every part.
- **A disjunction** is several patterns, and a match of it is a match of one of them.
- **An absence** $\operatorname{absent}(G)$ is a negative condition: the match counts only where the pattern $G$
  has no match that extends it. The variables of $G$ that $\theta$ leaves free are the absence's own, and it
  reads *for no objects in their places*.

**Which variables a substitution gives objects to.** A conjunction is a list, and a variable is **bound** at a
part by the parts before it: by a premise it stands in, or by an equality or an inequality it stands in. A
premise binds its variables to whatever the homomorphism sends them to. A variable that no premise of its
conjunction binds, and that stands in an equality or an inequality, ranges over a finite set $N$ of atoms that
the pattern is matched *over*: for the rules of a program, the program's names, [below](#the-program-and-its-names)
([Q-M1f.34](../../../decisions/language/Q-M1f.34-a-guard-that-binds-its-variable-binds-it-where.md)). In
$\operatorname{absent}(G)$ the substitution $\theta$ is the one the parts before the absence give, so its own
variables are those of $G$ that nothing before it binds:

$$
\operatorname{owns}(p, x) \land \operatorname{absent}(\operatorname{lives-in}(p, h)) \ \ \text{reads} \ \ \lnot \exists h.\ \operatorname{lives-in}(p\theta, h),
\qquad
\operatorname{absent}(\operatorname{lives-in}(p, h)) \land \operatorname{owns}(p, x) \ \ \text{reads} \ \ \lnot \exists p, h.\ \operatorname{lives-in}(p, h)
$$

A **match** of a pattern $\varphi$ in $K$ over $N$ is a substitution $\theta$ of the variables that stand in
$\varphi$ outside every absence, with $K \models \varphi\theta$.

**Two forms written with these**, and no part of the definition: the bounded universal and the open fact,

$$
\operatorname{forall}(x;\ G,\ B) \;=\; \operatorname{absent}\big(G \land \operatorname{absent}(B)\big), \qquad\qquad \operatorname{unknown}(t) \;=\; \operatorname{absent}(t) \land \operatorname{absent}(\lnot t)
$$

the first read *for every $x$ with $G$, $B$ holds*. With them each of the three statuses of a fact $f$ that an
alive knowledge base can give it ([the knowledge base](01-knowledge-base.md#a-fact-its-negation-both-or-neither))
has a pattern that holds exactly there:

| $\operatorname{status}_K(f)$ | $K$ holds | the pattern |
|---|---|---|
| asserted | $f$ | $f$ |
| negated | $\lnot f$ | $\lnot f$ |
| open | neither | $\operatorname{unknown}(f)$ |

**The premise $\lnot f$ is never the absence of $f$**:

$$
K \models \lnot f \iff \lnot f \in K, \qquad\qquad K \models \operatorname{absent}(f) \iff f \notin K
$$

for a fact $f$. The two differ on every open fact, where the second holds and the first does not.

A pattern with an absence, drawn with labelled arrows
([the knowledge base](01-knowledge-base.md#drawn-with-labelled-arrows)): the third rule of the three-house example,
$\operatorname{is-a}(h, \mathsf{House}) \land \operatorname{absent}(\operatorname{lives-in}(p, h))$, with its
conclusion $\operatorname{vacant}(h)$ in the lower panel.

```dot
digraph rule_nobody_home_lhs_rhs {
  rankdir=TB;
  label="nobody-home"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?h_L" [label="?h", shape=diamond];
    "House_L" [label="House", shape=rectangle];
    "?h_L" -> "House_L" [label="is-a", color="#7f7f7f", fontcolor="#7f7f7f"];
    subgraph cluster_absent_L_0 {
      label="absent (∄)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?p_L" [label="?p", shape=diamond];
      "?p_L" -> "?h_L" [label="lives-in", color="#9467bd", fontcolor="#9467bd"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?h_R" [label="?h", shape=diamond];
    "h1_vacant_R" [shape=octagon, label="vacant", color="#ff7f0e", fontcolor="#ff7f0e"];
    "h1_vacant_R" -> "?h_R" [label="1", color="#ff7f0e", style=dashed];
  }
}
```

The dashed box is the absence: the variable $p$ is its own, and the match counts where no object in its place
makes the arrow in the box a held fact.

## A conclusion, and the result of rewriting

A **conclusion** $\psi$ is again fact structures with variables: a finite set of **literals**, each one of

$$
\ell \;::=\; t \;\mid\; \lnot x \;\mid\; t = t' \;\mid\; t \neq t'
$$

a fact pattern, $\lnot t$ and $\bot$ among them; the negation of whatever fact a variable stands for; or a
**check**, an equality or an inequality
([Q-M1f.351](../../../decisions/language/Q-M1f.351-a-bare-variable-or-name-is-no-whole-conclusion.md)). For a
substitution $\theta$ that gives every variable of $\psi$ an object, the **instance** $\psi\theta$ is a set of
facts, the union over the literals of

$$
t \mapsto \{ t\theta \}, \qquad
\lnot x \mapsto \begin{cases} \{ \lnot\,\theta(x) \} & \theta(x) \in F \\ \emptyset & \theta(x) \in \Sigma^{+} \end{cases}, \qquad
(t = t') \mapsto \begin{cases} \emptyset & t\theta = t'\theta \\ \{ \bot \} & t\theta \neq t'\theta \end{cases}, \qquad
(t \neq t') \mapsto \begin{cases} \{ \bot \} & t\theta = t'\theta \\ \emptyset & t\theta \neq t'\theta \end{cases}
$$

An atom has no negation, so the second literal adds nothing where its variable holds one
([Q-M1f.128](../../../decisions/language/Q-M1f.128-a-firing-that-meets-the-negation-of-a-name.md)), and a check
that fails is a contradiction, as $\bot$ is.

**The result of rewriting** a knowledge base $K$ by a pattern $\varphi$ and a conclusion $\psi$ **at a match**
$\theta$ of $\varphi$ in $K$ is the knowledge base

$$
K \cup \psi\theta
$$

It is a set of facts, defined by $K$, $\psi$ and $\theta$ alone. Its graph is the graph of $K$ with the fact
nodes of $\psi\theta$ glued on along the match: each new fact points at the objects $\theta$ found and at the
atoms the conclusion names. Three things are true of it by the definition:

$$
K \subseteq K \cup \psi\theta, \qquad
\psi\theta \subseteq K \implies K \cup \psi\theta = K, \qquad
\operatorname{names}(K \cup \psi\theta) \subseteq \operatorname{names}(K) \cup \operatorname{names}(\psi) \cup N
$$

Nothing is removed and no object is changed, so a rewriting never undoes a fact, and a negative conclusion adds
a stored negation, which is a fact. A conclusion already held changes nothing. And the result names no atom but
those of $K$, of the conclusion and of $N$: every object $\theta$ gives is one a premise found under an argument
arrow of $K$, or an atom of $N$.

The match above, with the conclusion $\operatorname{pet-at}(x, h)$, gives the knowledge base below. The dashed
octagon and its dashed arrows are what the rewriting added:

```dot
digraph rewritten {
  rankdir=TB; nodesep=0.35; ranksep=0.6;
  node [shape=box, fontname="Helvetica", fontsize=11, height=0.3];
  edge [fontname="Helvetica", fontsize=10];
  { rank=min; "owns"; "lives-in"; "left-of"; "pet-at"; }
  { rank=max; "Dog"; "Norwegian"; "House-1"; "House-2"; }
  node [shape=octagon, label="", width=0.3, height=0.3, fixedsize=true, style=filled, fillcolor="#d9d9d9"];
  f1 [xlabel="f1"]; f2 [xlabel="f2"]; f3 [xlabel="f3"];
  c [xlabel="c", style="filled,dashed"];
  { rank=same; f1; f2; f3; c; }
  f1 -> "owns" [label="1"];
  f1 -> "Norwegian" [label="2"];
  f1 -> "Dog" [label="3"];
  f2 -> "lives-in" [label="1"];
  f2 -> "Norwegian" [label="2"];
  f2 -> "House-1" [label="3"];
  f3 -> "left-of" [label="1"];
  f3 -> "House-1" [label="2"];
  f3 -> "House-2" [label="3"];
  c -> "pet-at" [label="1", style=dashed];
  c -> "Dog" [label="2", style=dashed];
  c -> "House-1" [label="3", style=dashed];
}
```

$$
K \cup \psi\theta \;=\; \{ f_1, f_2, f_3 \} \cup \{ c \}, \qquad c = \operatorname{pet-at}(x, h)\,\theta = \operatorname{pet-at}(\mathsf{Dog}, \mathsf{House\text{-}1})
$$

The same pattern and conclusion drawn with labelled arrows, the pattern in the upper panel and the conclusion in
the lower. A diamond is a variable, a solid arrow a premise with three arrows, a dashed arrow a fact the
conclusion adds:

```dot
digraph rule_pet_follows_owner_lhs_rhs {
  rankdir=TB;
  label="pet-follows-owner"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?p_L" [label="?p", shape=diamond];
    "?x_L" [label="?x", shape=diamond];
    "?h_L" [label="?h", shape=diamond];
    "?p_L" -> "?x_L" [label="owns", color="#ff7f0e", fontcolor="#ff7f0e"];
    "?p_L" -> "?h_L" [label="lives-in", color="#9467bd", fontcolor="#9467bd"];
  }
  subgraph cluster_rhs { label="assert";
    "?x_R" [label="?x", shape=diamond];
    "?h_R" [label="?h", shape=diamond];
    "?x_R" -> "?h_R" [label="pet-at", color="#9467bd", fontcolor="#9467bd", style=dashed];
  }
}
```

### As a span

In the algebraic reading of graph rewriting a rule is a span of graphs, usually written with three letters that
mean other things on these pages; here it is

$$
L \;\xleftarrow{\ l\ }\; I \;\xrightarrow{\ r\ }\; L^{+}
$$

$L$ the pattern to find, $L^{+}$ what replaces it, and the interface $I$ the part both keep. Rewriting at a
match $m : L \to G$ deletes from $G$ what $L$ has and $I$ lacks, and glues on what $L^{+}$ has and $I$ lacks:
the double-pushout construction. A pattern and a conclusion delete nothing, so their span is the degenerate one,

$$
I = L, \qquad l = \operatorname{id}, \qquad L^{+} = L \cup \psi
$$

with $L$ the pattern graph of the premises of $\varphi$, and the double pushout reduces to one gluing. With
$G(K)$ the graph of $K$:

```dot
digraph gluing {
  node [shape=box, style=rounded, fontname="Helvetica", fontsize=11];
  edge [fontname="Helvetica", fontsize=10];
  L  [label="L\nthe pattern"];
  Lp [label="L⁺ = L ∪ ψ\nthe pattern with the conclusion"];
  G  [label="G(K)\nthe knowledge base"];
  Gp [label="G(K ∪ ψθ)\nthe result of rewriting"];
  { rank=same; L -> Lp [label="  r, an inclusion  "]; }
  { rank=same; G -> Gp [label="  an inclusion  "]; }
  L -> G   [label="  m, the match"];
  Lp -> Gp [label="  m, extended"];
}
```

The drawing is of four graphs and three kinds of map, not of a knowledge base. The square commutes, and
$G(K \cup \psi\theta)$ is its pushout **among the graphs in which one list of arrows is one node**: the
conclusion's facts are glued on along the match, and a fact of the conclusion that $K$ already has is the node
$K$ has, not a second one. In graphs that may hold two nodes with one list of arrows the pushout would add a
copy; the second condition of [the graph](01-knowledge-base.md#the-graph) is what makes the gluing a union of
sets of facts. Both horizontal arrows are inclusions. The conditions are read on the same square:

- **an equality or an inequality is an application condition on the match**: it restricts which matches count,
  and adds nothing to $L$;
- **an absence is a negative application condition**: a match counts only where it does not extend to a match
  of the absent pattern.

The pattern and the conclusion above drawn as one graph with labelled arrows, $L$ solid and what $L^{+}$ adds
dashed:

```dot
digraph rule_pet_follows_owner_overlay {
  label="pet-follows-owner"; labelloc=t;
  ranksep=0.55;
  "?p" [label="?p", shape=diamond];
  "?x" [label="?x", shape=diamond];
  "?h" [label="?h", shape=diamond];
  "?p" -> "?x" [label="owns", color="#ff7f0e", fontcolor="#ff7f0e"];
  "?p" -> "?h" [label="lives-in", color="#9467bd", fontcolor="#9467bd"];
  "?x" -> "?h" [label="pet-at", color="#9467bd", fontcolor="#9467bd", style=dashed];
}
```

## A rule, declared

A **rule** is a pattern and a conclusion declared under a name, with parameters and a reason
([Q4](../../../decisions/language/Q4-a-rule-is-a-pattern-a-conclusion-and-a-reason.md)):

$$
\rho = (\bar{p},\ \varphi,\ \psi), \qquad \operatorname{name}(\rho) \in \Sigma^{+} \setminus W, \qquad \bar{p} = (p_1, \dots, p_m) \in X^{m},\ m \ge 0
$$

| part | in the terms above |
|---|---|
| its **name** | a name. Where the rule has parameters the atom of that name is a relation: the facts under it are the rule's activators |
| its **parameters** $\bar{p}$ | variables that no part of $\varphi$ need bind: one more premise binds them, the **activator pattern** $\operatorname{name}(\rho)(p_1, \dots, p_m)$ |
| its **pattern** $\varphi$ | a pattern, matched over the names of the program |
| its **conclusion** $\psi$ | a conclusion over the variables the pattern and the parameters bind |
| its **reason** | a text, with places for the variables: what an instance of the rule reads as. It is no part of what the rule states |

How each part is spelt is [the user's page on rules](../../user/03-syntax-and-semantics/05-rules.md).

**Activators.** An **activator** of $\rho$ in $K$ is a held fact that the activator pattern matches, and it
gives the parameters their objects:

$$
\operatorname{Act}_K(\rho) = \{\, a \in K \;:\; \operatorname{out}(a) = (\operatorname{name}(\rho), v_1, \dots, v_m) \,\}, \qquad
\rho[a] = (\varphi[a],\ \psi[a]) \ \text{ with each } p_i \text{ replaced by } v_i
$$

$\rho[a]$ is the rule **applied under** $a$: a pattern and a conclusion with no parameter left. A rule with no
parameter, $m = 0$, has no activator pattern and is applied as it stands, once. A rule with parameters has one
application for each activator: none where $\operatorname{Act}_K(\rho) = \emptyset$, and a fact under the rule's
name with another count of arguments is no activator
([Q-M1f.203](../../../decisions/language/Q-M1f.203-a-kernel-marker-and-an-activator-are-held-to-no.md)). Two
activators that differ in any argument are two facts, so two applications, and an activator gives a parameter
whatever object stands in its place, an atom or a fact
([Q-M1f.43](../../../decisions/engine/Q-M1f.43-a-plans-activator-is-every-argument.md)): $\mathsf{1}$ and
$\mathsf{01}$ are two atoms, so $\operatorname{note}(\operatorname{edge}, \mathsf{1})$ and
$\operatorname{note}(\operatorname{edge}, \mathsf{01})$ are two applications of a rule named $\operatorname{note}$.

A **match of a rule** in $K$ is a pair $(a, \theta)$ of an activator $a \in \operatorname{Act}_K(\rho)$, or
nothing in its place where $m = 0$, and a match $\theta$ of $\varphi[a]$ in $K$. **The result of rewriting $K$ by
$\rho$ at $(a, \theta)$** is

$$
K \cup \psi[a]\theta
$$

The rule of transitivity has one parameter, which stands under the first arrow of its premises and of its
conclusion. With its activator pattern:

```dot
digraph rule_transitive {
  rankdir=TB; nodesep=0.4; ranksep=0.6;
  node [shape=box, fontname="Helvetica", fontsize=11, height=0.3];
  edge [fontname="Helvetica", fontsize=10];
  { rank=min; "transitive"; }
  node [shape=diamond];
  "?R";
  { rank=max; "?a"; "?b"; "?c"; }
  node [shape=octagon, label="", width=0.3, height=0.3, fixedsize=true, style=filled, fillcolor="#d9d9d9"];
  act [xlabel="act"]; q1 [xlabel="q1"]; q2 [xlabel="q2"];
  c [xlabel="c", style="filled,dashed"];
  { rank=same; q1; q2; c; }
  act -> "transitive" [label="1"];
  act -> "?R" [label="2"];
  "?R" -> q1 [label="1", dir=back];
  q1 -> "?a" [label="2"];
  q1 -> "?b" [label="3"];
  "?R" -> q2 [label="1", dir=back];
  q2 -> "?b" [label="2"];
  q2 -> "?c" [label="3"];
  "?R" -> c [label="1", dir=back, style=dashed];
  c -> "?a" [label="2", style=dashed];
  c -> "?c" [label="3", style=dashed];
  "?a" -> "?c" [label="≠", dir=none, style=dotted, constraint=false];
}
```

$$
\operatorname{transitive}[R]:\qquad \mathit{act} = \operatorname{transitive}(R), \qquad q_1 = R(a, b), \quad q_2 = R(b, c), \quad a \neq c, \qquad c = R(a, c)
$$

The variable $R$ is one diamond: the activator pattern points at it with its second arrow, and both premises and
the conclusion point at it with their first. The dotted line with no head is the inequality, a condition and no fact;
the dashed octagon is the conclusion. In a knowledge base that holds
$\operatorname{transitive}(\operatorname{left-of})$ the rule is applied under that fact, with
$\mathsf{left\text{-}of}$ in the diamond's place, and $\operatorname{transitive}[\operatorname{left-of}]$ is
$\operatorname{left-of}(a, b) \land \operatorname{left-of}(b, c) \land a \neq c$ with the conclusion
$\operatorname{left-of}(a, c)$. This is where *a relation is a node* pays: the activator is an ordinary fact
whose argument is a relation, so a property of a relation is a fact about it, and one rule serves every relation
that has the property ([Q27](../../../decisions/language/Q27-a-relations-properties-are-facts-beside.md)).

**What a rule states.** A knowledge base $K$ **satisfies** $\rho$ when rewriting by it at any match changes
nothing:

$$
\forall a \in \operatorname{Act}_K(\rho).\ \forall \theta.\ \ K \models \varphi[a]\theta \ \implies\ \psi[a]\theta \subseteq K
$$

with the one empty application where $m = 0$. This is a sentence about $K$ alone: a rule reads and concludes
facts of one knowledge base, and nothing in it reads another, how many there are, or how one was reached
([Q-M1d.7](../../../decisions/language/Q-M1d.7-a-rule-is-a-sentence-about-the-state-it-fires.md)).

### The conditions of a declaration

A declaration is a rule only if each of these holds; a text that fails one is no program.

$$
\begin{aligned}
&\operatorname{vars}(\psi) \;\subseteq\; \bar{p} \;\cup\; \bigcap_{i \le l} \operatorname{bound}(\varphi_i) && \text{where } \varphi = \varphi_1 \lor \cdots \lor \varphi_l \\
&x \text{ stands under a first arrow in } \varphi \text{ or } \psi \implies x \in \bar{p} \\
&\operatorname{ar}(t) = n && \text{for a fact pattern } t \text{ of a relation declared with } n \text{ labels} \\
&\operatorname{name}(\rho) \neq \operatorname{name}(\rho') && \text{for two rules } \rho \neq \rho' \text{ of one program}
\end{aligned}
$$

- **Every variable of the conclusion is bound by every way of satisfying the pattern**, or is a parameter:
  $\operatorname{bound}(\varphi_i)$ is the variables that stand in the conjunction $\varphi_i$ outside every
  absence
  ([Q-M1e2.37](../../../decisions/language/Q-M1e2.37-a-conclusions-variable-is-bound-by-every-way.md)). So an
  instance $\psi[a]\theta$ is always defined, and is a set of facts with no variable.
- **A variable in a relation's place is a parameter**, so only an activator gives a relation's place its atom.
  An activator that would put a fact there is no activator of a program, whether the text writes it or some
  rule's conclusion could make it
  ([Q-M1f.1](../../../decisions/language/Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md)).
- **A fact pattern of a declared relation has the declared count**
  ([Q-M1f.315](../../../decisions/language/Q-M1f.315-a-form-of-a-declared-relation-has-as-many.md)). One whose
  relation is a parameter is held to no count, and matches by count
  ([Q-M1f.202](../../../decisions/language/Q-M1f.202-a-relation-written-as-a-variable-is-held-to-no.md)).
- **A rule's name is declared once.**
- **A conclusion is literals and nothing else**: no disjunction, and no fact made of several. *One of these* is
  not said by a rewriting rule. A conjunction in a conclusion is read flat, as the set of its literals, at any depth of
  nesting, and the empty conclusion adds nothing.
- **A conclusion $\lnot x$ is of a variable that holds a fact**: where the text shows that $x$ can hold an atom,
  the declaration is no rule
  ([Q-M1f.127](../../../decisions/language/Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md)).

A pattern may read a relation of which the knowledge base has no fact and no declaration: it is a rule, and it
has no match ([Q-M1f.120](../../../decisions/language/Q-M1f.120-a-rule-may-read-a-relation-nothing-makes.md)).

### The program, and its names

A program $P$ is read into its knowledge base $K_0$ ([the knowledge base](01-knowledge-base.md#the-knowledge-base-of-a-program))
and its finite set of rules, also written $P$ where a rule set is meant. A rule is no fact: $K_0$ holds no node
for a rule's pattern or its conclusion. What a declaration of a rule adds to $K_0$ is one kind of fact: for
each fact $t$, a fact pattern with no variable, that is nested in a literal of its conclusion,

$$
\operatorname{written}(t) \in K_0
$$

as $K_0$ holds it for each fact nested in a written fact, a negation and the fact it denies read as one there
too. The **names of a rule** are the atoms under an
argument arrow of its fact patterns, at any depth, and those its equalities and inequalities compare; the
**names of the program** are those of its knowledge base and of its rules:

$$
\operatorname{names}(P) \;=\; \operatorname{names}(K_0) \;\cup \bigcup_{\rho \in P} \operatorname{names}(\rho)
$$

a finite subset of $\Sigma^{+}$, fixed by the text. A name that only a question writes is not among them
([Q-M1e2.9](../../../decisions/language/Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)).
$\operatorname{names}(P)$ is the set $N$ a rule's pattern is matched over, so by the third property of a
rewriting, for every match of a rule of $P$ in a knowledge base $K$,

$$
\operatorname{names}(K \cup \psi[a]\theta) \;\subseteq\; \operatorname{names}(K) \cup \operatorname{names}(P)
$$

**a rewriting brings in no name** ([Q-M1e.21](../../../decisions/hypotheses/Q-M1e.21-a-rule-brings-in-no-name.md)).

## The kinds of rule

### By its conclusion

What a rule does to a knowledge base it matches in is read off its conclusion:

| the conclusion holds | the result of rewriting |
|---|---|
| facts, stored negations among them | holds them |
| $\bot$ | is dead: the rule is a **constraint**, a pattern that no alive knowledge base matches and satisfies |
| a check | is $K$ where the check holds of the match, and dead where it does not |
| nothing | is $K$ |

Two more kinds of declaration have a rule's parts and are not rewriting rules: neither adds its conclusion to
the knowledge base it matches in. Each is defined where it is used, in the part after this one
([the first](../02-kb-evolution/04-hypothesis-generation.md#3-the-hypothesis-rules-what-the-program-says-to-guess),
[the second](../02-kb-evolution/10-satisfiability.md)).

### By what its variables stand for

The kinds below are not exclusive: a rule is of the kind of its widest variable.

| kind | its variables stand for | applied |
|---|---|---|
| **first-order** | objects under argument arrows; every relation is an atom of the pattern | once, as it stands |
| **relation-polymorphic** | objects, and relations: a variable stands under a first arrow | once for each activator that gives its parameters their objects |
| **structural** | a whole region of the graph: what remains, which is the last | as the first two kinds are, through a universal premise |

#### First-order

Every relation of the pattern is an atom, and the variables stand under argument arrows. The pattern is a fixed
shape of the graph up to which objects fill it, and it matches wherever the shape occurs. The first rule of the
three-house example is one, with no parameter; it is the pattern and the conclusion drawn above,

$$
\operatorname{owns}(p, x) \;\land\; \operatorname{lives-in}(p, h) \;\implies\; \operatorname{pet-at}(x, h)
$$

#### Relation-polymorphic

At least one variable stands under a first arrow, as $R$ does in the rule of transitivity above. Such a variable
is never free to range over every relation: it is a parameter, and the rule has one application for each
activator.

**A declaration is read like any fact.** A relation's declaration is a fact of $\mathsf{relation}$, so a pattern
over it is an ordinary premise: $\operatorname{relation}(R, x, y)$ gives $x$ and $y$ the labels of a relation of
two arguments. That is how a rule states what the labels are for. With $R$ a parameter,

$$
\operatorname{relation}(R, t_1, t_2) \;\land\; R(x_1, x_2) \;\land\; \operatorname{is-a}(x_1, t_1) \;\implies\; \operatorname{fits}(x_1, t_1)
$$

matches each fact of $R$ whose first argument is, by a fact of $\operatorname{is-a}$, a member of the first
label, and concludes that it fits. Applied under an activator that names $\operatorname{lives-in}$, in the
knowledge base of [the declaration's drawing](01-knowledge-base.md#what-is-where), it has one match, whose
result holds $\operatorname{fits}(\mathsf{Norwegian}, \mathsf{Person})$. It has no match at a fact
$\operatorname{lives-in}(\mathsf{House\text{-}1}, \mathsf{Norwegian})$, which is a fact all the same. A program
with no such rule gives its labels no meaning
([Q-M1f.372](../../../decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)).

#### Structural

A structural statement is about a region of the graph rather than a few facts in it: *exactly one object
remains*, *no other object takes this place*. **Ein has no aggregate.** No function counts the facts of a
relation, and the only conditions whose truth is computed are equality and inequality
([Q-M1e2.26](../../../decisions/language/Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)): the
truth of a fact is data, which a pattern can find. A structural statement is an ordinary rule whose pattern holds a universal
premise. The standard library's rule of **domain elimination** concludes the one object left for an argument
once every other is denied. With $R$ the relation and $m$ the program's membership relation as its parameters:

$$
\begin{aligned}
&\operatorname{functional}(R) \;\land\; \operatorname{total}(R) \;\land\; \operatorname{relation}(R, A, B) \;\land\; m(a, A) \;\land\; m(b, B) \\
&\quad \land\; \operatorname{forall}\big(b';\ \ m(b', B) \land b' \ne b,\ \ \lnot R(a, b')\big) \\
&\implies\ R(a, b)
\end{aligned}
$$

Drawn with labelled arrows:

```dot
digraph rule_domain_elimination_lhs_rhs {
  rankdir=TB;
  label="domain-elimination"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?R_L" [label="?R", shape=diamond];
    "?A_L" [label="?A", shape=diamond];
    "?B_L" [label="?B", shape=diamond];
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "h1_functional_L" [shape=octagon, label="functional", color="#bcbd22", fontcolor="#bcbd22"];
    "h1_functional_L" -> "?R_L" [label="1", color="#bcbd22"];
    "h2_total_L" [shape=octagon, label="total", color="#1f77b4", fontcolor="#1f77b4"];
    "h2_total_L" -> "?R_L" [label="1", color="#1f77b4"];
    "h3_relation_L" [shape=octagon, label="relation", color="#bcbd22", fontcolor="#bcbd22"];
    "h3_relation_L" -> "?R_L" [label="1", color="#bcbd22"];
    "h3_relation_L" -> "?A_L" [label="2", color="#bcbd22"];
    "h3_relation_L" -> "?B_L" [label="3", color="#bcbd22"];
    "?a_L" -> "?A_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    "?b_L" -> "?B_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
    subgraph cluster_forall_L_0 {
      label="forall (∀)"; style="dashed,rounded"; color="#888888"; fontcolor="#888888";
      "?b_other_L" [label="?b_other", shape=diamond];
      "?b_other_L" -> "?B_L" [label="?isa", color="#bcbd22", fontcolor="#bcbd22"];
      "?b_other_L" -> "?b_L" [label="≠", dir=none, style=dotted, color="#555555", fontcolor="#555555", constraint=false];
      "?a_L" -> "?b_other_L" [label="¬?R", color="#d62728", fontcolor="#d62728"];
    }
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?b_R" [label="?b", shape=diamond];
    "?a_R" -> "?b_R" [label="?R", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
  }
}
```

The dashed box is the universal premise: the variable inside it is its own, the red arrow is a stored negation,
and the box reads *every other member of the second label is denied*. The octagon with three numbered arrows is
the premise that reads the declaration, with the name of $\mathsf{relation}$ written in it and its arrows
numbered by argument. The arrows labelled by a
variable are premises whose relation is a parameter. The statement is in the pattern language, not beneath it.

## The four levels a knowledge base describes itself in

A program speaks at four **levels**, and each is a part an atom or a fact plays, not a sort of node:

| level | what is there | in the three-house example |
|---|---|---|
| **L0, the things named** | atoms under argument arrows | $\mathsf{Norwegian}$, $\mathsf{House\text{-}1}$, $\mathsf{House}$ |
| **L1, facts** | facts over them | $\operatorname{lives-in}(\mathsf{Norwegian}, \mathsf{House\text{-}1})$ |
| **L2, relations** | declarations, and the facts about a relation that say what holds of its facts | the declaration of $\operatorname{left-of}$; $\operatorname{transitive}(\operatorname{left-of})$ |
| **L3, rules** | patterns with conclusions | the rule of transitivity |

One atom may play at several levels: $\mathsf{left\text{-}of}$ is the relation of two facts and an argument of a
third, and $\mathsf{transitive}$ names a rule and is the relation of its activator. **Each level reads the
levels below it:**

```dot
digraph four_levels {
  rankdir=LR;
  node [shape=box, style=rounded, fontname="Helvetica", fontsize=11];
  edge [fontname="Helvetica", fontsize=10];
  L3 [label="L3 rules"];
  L2 [label="L2 relations\ndeclarations and facts about relations"];
  L1 [label="L1 facts"];
  L0 [label="L0 the things named"];
  L3 -> L1 [label="match, conclude"];
  L3 -> L2 [label="applied under a fact about a relation", style=dashed];
  L2 -> L1 [label="fix the count of"];
  L1 -> L0 [label="point at, with an argument arrow"];
}
```

The drawing is of the levels, not of a knowledge base.

- **rules** match facts and conclude facts, and a relation-polymorphic rule is applied under a fact of L2;
- **relations** stand over facts: a fact's relation is an atom, whose declaration fixes how many arguments the
  fact has and whose property facts are the activators of the rules that hold of it;
- **facts** point at the things named with their argument arrows;
- **the things named** are leaves.

**Where the stack folds back on itself.** Two folds keep the reading downward:

- **L2 is readable as L1.** A declaration is facts of $\mathsf{relation}$, and a property is a fact about a
  relation, so a pattern reads a relation's labels and its properties as it reads any fact. The levels are a
  reading of one graph, and the graph describes its own schema.
- **L3 is the program's text.** A conclusion is literals; none is a rule or a declaration, and the graph holds
  no node for a rule. So a rewriting adds facts, and adds no rule, no declaration and no name.

Rules that read and write rules, a level over L3, are what
[rules as data](../03-rule-evolution/01-rules-as-data.md) takes up: what of Ein describes itself in Ein, and
where rules as data would begin.
