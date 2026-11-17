# Syntax and semantics

Every concept an Ein program is written in, one page each: what it is, how a program writes it, and what it means.
The [reference](../04-reference/README.md) lists every form and every refusal; these pages say what each form is for and
how the concepts fit together. The [guide](../02-guide/README.md) teaches the same ground by solving a puzzle.

## The four representations

A page on a core notion states it four ways, under the same four headings, in the same order, near the top of the
page, and **the four are of one example**, so a reader can lay them side by side
([Q-M1f.371](../../../decisions/docs/Q-M1f.371-a-core-notion-is-shown-four-ways-in-the-users.md)):

| heading | what it shows | why it is there |
|---|---|---|
| **In Ein** | a small program that loads and runs, and what `ein` prints for it. Ein is the main language of these pages: it is what a reader writes | what to type |
| **As a graph** | that program as the engine draws it, in a `dot` block, with the command that drew it: `ein render kb` for what the knowledge base holds, `ein render rule` for a rule as a rewrite, `ein render region` for the lattice of commitments | the knowledge base is a graph, and every form has a fixed drawing |
| **In words** | what the Ein says: the English the program carries in its `:why`, `:source` and `:goal-text`, and the sentences the engine prints from it | every statement a program makes can be read back as English |
| **In mathematics** | what the notion is, as a formula: the set, the relation, the implication a rule is, the condition an obligation owes, what a hypothesis adds, what the answer is | what the concept is, independent of how it is written or drawn |

Which pages owe the four, and which do not:

| pages | the notion | owes |
|---|---|---|
| [objects](01-objects.md), [relations](02-relations.md), [facts](03-facts.md), [ontology](04-ontology.md) | what the knowledge base holds | all four |
| [rules](05-rules.md), [constraints](06-constraints.md), [properties and operators](07-properties-and-operators.md), [obligations](08-obligations.md), [hypotheses](09-hypotheses.md), [theories](10-theory.md) | what a program says about it | all four, and every rule of the page's example is drawn, there or on the page its program is taken from |
| [queries](12-query.md), [solutions and models](13-solutions-and-models.md) | what is asked and what the answer is | all four |
| [macros and imports](11-macros.md) | syntax: a pattern and its expansion, a module brought in under a name | Ein and words. No graph and no formula for the form; a macro that means something, `forall`, has both |
| [natural language](14-natural-language.md) | syntax: `:why`, `:source` and `:goal-text`, the built-in connection between a form and its English | Ein and words, and nothing else |

DOT the named command wrote, less its comments and font settings, a region's tooltips, layer labels, legend and
the line of its title that names the run, and the facts a page says it leaves out; a drawing made by hand
the facts a page says it leaves out; a drawing made by hand
says so. A formula is TeX. After the four come the details, each refusal linked to the
[diagnostics](../04-reference/07-diagnostics.md), and a list of related pages.

## The pages

**What the knowledge base holds**

- [Objects](01-objects.md): a name and the node it names; no types, and how a rule gives a label a meaning; the compact
  and the detailed drawing.
- [Relations](02-relations.md): a declaration, its labels and arity, and a relation as data.
- [Facts](03-facts.md): a relation applied to values, where each fact came from, and a fact held inside a fact.
- [Ontology](04-ontology.md): everything is a node, membership and inheritance are a relation and rules of the
  program's own, and nothing about them is built in.

**What a program says about it**

- [Rules](05-rules.md): a pattern, a conclusion and a reason; a rule as a rewrite of the graph; parameters and
  activators; `absent`.
- [Constraints](06-constraints.md): `(false)` and `(not …)`, what cannot be, the upper bound of the answer.
- [Properties and operators](07-properties-and-operators.md): the relation algebra as facts and rules.
- [Obligations](08-obligations.md): `(open ?R)`, what a state still owes, the lower bound of the answer.
- [Hypotheses](09-hypotheses.md): what the engine may assume, and how a program says where to look.
- [Theories](10-theory.md): a rule set as a whole, how its rules interact, and a theory as a module.
- [Macros and imports](11-macros.md): a pattern written once, called and expanded; `forall`; modules brought in whole.

**What a program asks, and what it gets**

- [Queries](12-query.md): `(query …)`, `(solve …)`, and the claims a program states about its answer.
- [Solutions and models](13-solutions-and-models.md): the answer as a region of states, and what each reads as.
- [Natural language](14-natural-language.md): the texts a program writes, and where the engine prints them.

## Reading order

[Objects](01-objects.md), [relations](02-relations.md) and [facts](03-facts.md) first: they are the vocabulary every other page
uses. Then [rules](05-rules.md) and [constraints](06-constraints.md), which are most of what a program writes, and
[solutions and models](13-solutions-and-models.md), which is what it gets back. [Theories](10-theory.md) is the page to read
before writing a library, or importing several. [Ontology](04-ontology.md) is for when the self-describing side matters:
before a rule that reads relations, labels or rules as data. The guide's
[Jack drinks coffee](../02-guide/05-jack-drinks-coffee.md) works one small example through all four representations.

## See also

- [The reference](../04-reference/README.md): the grammar, the forms, the standard library, the command line, the
  read-out and every diagnostic.
- [The guide](../02-guide/README.md): learn Ein by solving the Zebra puzzle.
- How the engine computes what these pages define, stated so that any implementation is held to it:
  [the reasoning pages](../../reasoning/README.md).
