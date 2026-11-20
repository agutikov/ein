# Q-M1f.394 — A fact is in the global context, in a fact's context, or in both

A fact is **held** when the knowledge base holds it, and the facts held are the **global context**. A fact is **in a
context** when a held fact points at it, as an argument or deeper: the context of a fact is the held facts from
which a path of argument arrows leads to it, a **local** context beside the global one, and each of those facts
speaks about it. The two are independent, so a fact of the graph is held and in no context, in a context and not
held, or both, and it is one node in each case
([Q-M1f.380](Q-M1f.380-an-objects-identity-is-its-name-or-its-tuple.md)).

**A stored negation and a hypothesis are contexts.** `(not P)` is a held fact that points at `P`: where a program
imports the contradiction's rules, `P` held beside it is a contradiction
([Q-M1f.557](../stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)). It is no mirror of the global
context: it holds the facts something stated or concluded as denied, so a fact may be in neither, which is
*unknown*. `(open P)` is a held fact that points at `P` too, and the kernel reads it at root: `P` is a hypothesis,
and holding `(open P)` holds nothing of `P`
([Q-M1f.566](../hypotheses/Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md)). Every other relation's facts
are opaque to the kernel, and mean what a program's rules make of them.

**The words, each in its place.** *Object* is the word of the core theory, on the graphs, for every node: an atom
or a fact ([Q-M1f.379](Q-M1f.379-an-object-is-a-node-of-the-knowledge-base-graph.md)). *Value* is used only of a
rule's variable: the object a variable is bound to is its value, and an object is a value only while bound. *Term*
is a word of the syntax, a written form. *Graph node* is fine, and *relational node* is not used. The pages of
`docs/ein/ein.rs/` name things as software documentation needs.

**Why.** *Alice thinks that five is two plus two* says nothing of whether five is two plus two, and the same fact
may be stated, said, or both. Two words for two independent properties let a page say which it means; one word for
the node wherever it stands keeps it one fact. A word that calls the fact in a context something other than a fact
makes the sentence *the same fact is held and is in a context* unsayable.

**Held by.** The pages: [the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § The global
context, and the context of a fact, and § A context gives a fact no truth;
[facts](../../ein/user/03-syntax-and-semantics/03-facts.md), § A fact inside a fact; the glossary's *Held, global
context, context* and *Value*. The claims of `examples/beliefs/02_packing.ein`, in which a fact a witness only
claims is not held.
