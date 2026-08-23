# nlp/ — scratch area

Reading and two throwaway scripts (`xxx.py`, `xxx-link.py`) from the 2021
prototype's look at dependency parsing. Not wired into the engine.

[M2 P2.5](../plans/m2_nl_to_ir/p2.5_link_grammar_experiment/README.md) is the
phase that gives this directory a purpose: a *measured* A/B on whether feeding
link-grammar output to the LLM improves NL → IR quality, with these two files
as the starting point ([S2.5.1](../plans/m2_nl_to_ir/p2.5_link_grammar_experiment/s2.5.1_runner.md)).

> **The `link-grammar` submodule was deinitialised at M1a
> [S1a.10.5](../docs/history/m1a_rust/README.md#s1a105--the-removal).**
> It pointed at `opencog/link-grammar` and was never checked out, so what it
> cost was a `git clone --recurse-submodules` fetching it for an experiment
> that has not run. P2.5 re-adds it in one command when it does — and P2.5's
> possible outcome is "deprecate the submodule", so registering it now would
> pre-empt the decision the phase exists to take:
>
> ```sh
> git submodule add https://github.com/opencog/link-grammar.git nlp/link-grammar
> ```

## Reading

https://www.nltk.org/book/ch08.html

https://nlp.stanford.edu/software/lex-parser.shtml

http://www.link.cs.cmu.edu/link/

https://www.abisource.com/projects/link-grammar/#download

https://github.com/opencog/relex

https://wiki.opencog.org/w/RelEx_Dependency_Relationship_Extractor

https://github.com/opencog/link-grammar

https://github.com/opencog

https://en.wikipedia.org/wiki/Category:Grammar_frameworks

https://wiki.opencog.org/w/Real_World_Reasoning

https://www.abisource.com/projects/link-grammar/api/index.html#sents

https://www.abisource.com/projects/link-grammar/dict/index.html

https://www.abisource.com/projects/link-grammar/dict/summarize-links.html

https://ru.wikipedia.org/wiki/%D0%93%D1%80%D0%B0%D0%BC%D0%BC%D0%B0%D1%82%D0%B8%D0%BA%D0%B0_%D1%81%D0%BE%D1%81%D1%82%D0%B0%D0%B2%D0%BB%D1%8F%D1%8E%D1%89%D0%B8%D1%85

http://people.duke.edu/~mccann/mwb/15semnet.htm

https://en.wikipedia.org/wiki/Semantic_network#Examples

https://en.wikipedia.org/wiki/WordNet#Licensed_vs._Open_WordNets

http://trimc-nlp.blogspot.com/2015/06/python-nltk-and-wordnet.html

https://en.wiktionary.org/wiki/Wiktionary:Semantic_relations

https://wordnet.princeton.edu/documentation

https://medium.com/parrot-prediction/dive-into-wordnet-with-nltk-b313c480e788

https://en.wikipedia.org/wiki/MultiNet

http://www.jfsowa.com/pubs/semnet.htm





http://graphit-lang.org/getting-started







