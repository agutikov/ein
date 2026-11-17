# Syntax and semantics

Every concept an Ein program is written in, one page each: what it is, how a program writes it, and what it means.
The [reference](../reference/README.md) lists every form and every refusal; these pages say what each form is for and
how the concepts fit together. The [guide](../guide/README.md) teaches the same ground by solving a puzzle.

## The four representations

Each concept is stated four ways, under the same four headings, in the same order, near the top of its page:

| heading | what it shows | why it is there |
|---|---|---|
| **In Ein** | the forms a program writes, in a block that loads and runs, or an excerpt marked with `…` | what to type |
| **As a graph** | the concept as `ein render` draws it, the view named | the knowledge base is a graph, and every form has a fixed drawing |
| **In words** | the English the program carries in its `:why`, `:source` and `:goal-text`, and the sentences the engine prints from it | every statement a program makes can be read back as English |
| **In mathematics** | the definition, in plain notation | what the concept is, independent of how it is written or drawn |

Every block of output on these pages is the binary's, from the command named beside it. After the four come the
details, each refusal linked to the [diagnostics](../reference/diagnostics.md), and a list of related pages.

## The pages

**What the knowledge base holds**

- [Objects](objects.md): a name, the node it names, and the five kinds of node; the compact and the detailed drawing.
- [Relations](relations.md): a declaration, its signature and arity, and a relation as data.
- [Facts](facts.md): a relation applied to values, where each fact came from, and a fact held inside a fact.
- [Ontology](ontology.md): everything is a node, types are ordinary names, and nothing about them is built in.

**What a program says about it**

- [Rules](rules.md): a pattern, a conclusion and a reason; parameters and activators; `absent`.
- [Constraints](constraints.md): `(false)` and `(not …)`, what cannot be, the upper bound of the answer.
- [Properties and operators](properties-and-operators.md): the relation algebra as facts and rules.
- [Obligations](obligations.md): `(open ?R)`, what a state still owes, the lower bound of the answer.
- [Hypotheses](hypotheses.md): what the engine may assume, and how a program says where to look.
- [Theories](theory.md): a rule set as a whole, how its rules interact, and a theory as a module.
- [Macros and imports](macros.md): patterns written once, and modules brought in whole.

**What a program asks, and what it gets**

- [Queries](query.md): `(query …)`, `(solve …)`, and the claims a program states about its answer.
- [Solutions and models](solutions-and-models.md): the answer as a region of states, and what each reads as.
- [Natural language](natural-language.md): the texts a program writes, and where the engine prints them.

## Reading order

[Objects](objects.md), [relations](relations.md) and [facts](facts.md) first: they are the vocabulary every other page
uses. Then [rules](rules.md) and [constraints](constraints.md), which are most of what a program writes, and
[solutions and models](solutions-and-models.md), which is what it gets back. [Theories](theory.md) is the page to read
before writing a library, or importing several. [Ontology](ontology.md) is for when the self-describing side matters:
before a rule that reads relations, types or rules as data. The guide's
[Jack drinks coffee](../guide/05_jack_drinks_coffee.md) works one small example through all four representations.

## See also

- [The reference](../reference/README.md): the grammar, the forms, the standard library, the command line, the
  read-out and every diagnostic.
- [The guide](../guide/README.md): learn Ein by solving the Zebra puzzle.
- How the engine computes what these pages define, stated so that any implementation is held to it:
  [the reasoning pages](../../reasoning/README.md).
