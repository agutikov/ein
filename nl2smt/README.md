# `nl2smt/` — natural language and SMT, one folder

The two scratch areas of the 2021 prototype, `nlp/` and `smt/`, in one place
([M1j](../plans/m1j_nlp_smt_poc/README.md) T1). Nothing here is wired into the
engine: no crate depends on it and the gate does not run it.

| | what it is |
|---|---|
| [`smt/`](smt/) | three hand-written SMT-LIB encodings, `einstain-problem.smt`, `einstain-problem-minus-15.smt` and `4-queens.smt` |
| [`prototype/`](prototype/) | two throwaway scripts from the prototype's look at dependency parsing, `xxx.py` (NLTK) and `xxx-link.py` (link grammar over the English Zebra text) |

## The hand-written encodings

`smt/` holds what the Zebra puzzle and 4-queens look like when written for a
solver rather than for the graph engine. **M3 (SMT integration) was dropped
2026-08-18**: Ein has no solver back-end and none is planned; see
[`plans/README.md`](../plans/README.md) § Roadmap.
[M10](../plans/m10_external_benchmarks/README.md) counts the three files as
three of its benchmark corpus's encodings, already written, so their content
does not change.

> **The `CVC4` submodule was deinitialised at M1a
> [S1a.10.5](../docs/history/m1a_rust/README.md#s1a105--the-grammar-as-ebnf).**
> It pointed at `CVC4/CVC4` at version 1.8 (2021), at `smt/CVC4`, and was never
> built by anything here; what it cost was a `git clone --recurse-submodules`
> fetching a large repository for a dropped milestone. Nothing that *reads*
> these files needed it — M10's benchmark uses **CVC5**, which is a different
> program with a different name. Re-add it if some later stage actually wants
> 1.8:
>
> ```sh
> git submodule add https://github.com/CVC4/CVC4.git nl2smt/CVC4
> ```

External-tech notes on solvers live in
[`docs/lib/02-solvers-csp-sat-smt.md`](../docs/lib/02-solvers-csp-sat-smt.md).

- CVC4 user manual: <http://cvc4.cs.stanford.edu/wiki/User_Manual>
- CVC4 source: <https://github.com/CVC4/CVC4>

## The prototype's scripts

[M2 S2.6.4](../plans/m2_nl_to_ir/p2.6_ablations/s2.6.4_representation_ablations.md)
is the stage that gives the two scripts a purpose: a *measured* A/B on whether
feeding link-grammar output to the LLM improves the formalization, with these
two files as the starting point. It was a phase of its own (P2.5) until the
2026-08-23 reshape of M2 around [`EinAf.md`](../plans/m2_nl_to_ir/EinAf.md)
made it one arm of the representation ablation; nothing about the experiment
changed.

> **The `link-grammar` submodule was deinitialised at M1a
> [S1a.10.5](../docs/history/m1a_rust/README.md#s1a105--the-grammar-as-ebnf).**
> It pointed at `opencog/link-grammar`, at `nlp/link-grammar`, and was never
> checked out, so what it cost was a `git clone --recurse-submodules` fetching
> it for an experiment that has not run. S2.6.4 re-adds it in one command when
> it does — and its possible outcome is "deprecate the submodule", so
> registering it now would pre-empt the decision the stage exists to take:
>
> ```sh
> git submodule add https://github.com/opencog/link-grammar.git nl2smt/link-grammar
> ```

## Reading

Parsing and grammar:

- <https://www.nltk.org/book/ch08.html>
- <https://nlp.stanford.edu/software/lex-parser.shtml>
- <http://www.link.cs.cmu.edu/link/>
- <https://www.abisource.com/projects/link-grammar/#download>
- <https://www.abisource.com/projects/link-grammar/api/index.html#sents>
- <https://www.abisource.com/projects/link-grammar/dict/index.html>
- <https://www.abisource.com/projects/link-grammar/dict/summarize-links.html>
- <https://github.com/opencog/link-grammar>
- <https://github.com/opencog/relex>
- <https://wiki.opencog.org/w/RelEx_Dependency_Relationship_Extractor>
- <https://github.com/opencog>
- <https://wiki.opencog.org/w/Real_World_Reasoning>
- <https://en.wikipedia.org/wiki/Category:Grammar_frameworks>
- <https://ru.wikipedia.org/wiki/%D0%93%D1%80%D0%B0%D0%BC%D0%BC%D0%B0%D1%82%D0%B8%D0%BA%D0%B0_%D1%81%D0%BE%D1%81%D1%82%D0%B0%D0%B2%D0%BB%D1%8F%D1%8E%D1%89%D0%B8%D1%85>

Semantic networks and WordNet:

- <http://people.duke.edu/~mccann/mwb/15semnet.htm>
- <https://en.wikipedia.org/wiki/Semantic_network#Examples>
- <https://en.wikipedia.org/wiki/WordNet#Licensed_vs._Open_WordNets>
- <http://trimc-nlp.blogspot.com/2015/06/python-nltk-and-wordnet.html>
- <https://en.wiktionary.org/wiki/Wiktionary:Semantic_relations>
- <https://wordnet.princeton.edu/documentation>
- <https://medium.com/parrot-prediction/dive-into-wordnet-with-nltk-b313c480e788>
- <https://en.wikipedia.org/wiki/MultiNet>
- <http://www.jfsowa.com/pubs/semnet.htm>

Other:

- <http://graphit-lang.org/getting-started>
