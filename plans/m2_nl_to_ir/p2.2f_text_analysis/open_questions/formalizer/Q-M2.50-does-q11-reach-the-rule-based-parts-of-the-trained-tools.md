# Q-M2.50 — Does Q11's ruling reach the rule-based parts of the trained tools, and WordNet?

**Status:** open

**Raised 2026-11-27 by [S2.2f.1](../../s2.2f.1_the_check.md)**, making the environment.

**The question.** [Q11](../../../open_questions/formalizer/Q11-link-grammar-value.md) was ruled: *"link-grammar will
not be used, any strictly pre-defined grammars will not be used."* The trained tools the check ran have parts that
are written by hand, none of them a grammar of English's sentences:

- **amrlib's RBW aligner**, rule-based: it aligns an AMR node to a word by its lemma (and a table of number words),
  97 of XFM's 183 nodes; IBM's parser aligns every node by what it learned.
- **spaCy's pipeline** has an `attribute_ruler` and a lemmatizer of lookup tables beside its trained tagger, parser
  and entities; Stanza's lemmatizer is trained, with a dictionary.
- **WordNet** (Open English WordNet, through the `wn` package, not NLTK) is a lexicon written by hand: the sense
  inventory the trained disambiguator chooses from, and the hypernyms that give a kind.

Is any of these what the ruling excludes?

**The default that stands**: none is. A grammar is a set of rules over a sentence's structure; an aligner, a
lemmatizer's tables and a lexicon are not, and each sits under or beside a trained model. S2.2f.2 prefers IBM's
learned alignment where a node's word is needed, and the word senses, which do not pass the check, are used by
nothing.

**The alternatives.**

1. **The default.**
2. **Only what is learned**: no rule-based aligner (a node's sentence is still known, one graph a sentence; its word
   only from IBM's parser), spaCy's entities without its lemmas, and no WordNet.

**What S2.2f.2 took under the default (2026-11-27)**: the recorded analyses keep every parser's graphs with their
aligner's markers and name the aligner's kind; **a form shows a node's words only where the aligner is learned**
(IBM's), so that the default analysis, SPRING's, shows each node's sentence and no word, and amrlib's RBW markers are
kept and counted in no form ([L-M2.46](../../findings/text-analysis/L-M2.46-only-ibms-parser-aligns-a-node-to-its-word-by-what-it-learned.md)).
spaCy's named entities are its trained recognizer's; its lemmas are given to no form. Under the second alternative
nothing of the forms changes.

**What bears on it.** [The check's § 1 and § 5](../../check/README.md#1-the-tools); the phase's own condition on
its rule-reading arm, *"if rules relies on finite amount of terms that AMR uses, not rely on NL words"*, which is
about rules over AMR, not about the tools that make it.
