%% The analysis as Ein facts: the vocabulary's declarations (analysis/vocabulary.v1.ein), then each
%% sentence's AMR graph as facts, each quoting its sentence as its :source. The slot analysis of
%% steps/step1-analysis.v1.md; the form is docs/einaf/09-analysis.md § 3's.
An analysis of the text, written before you by a trained parser and not by a person: each sentence's meaning as an Abstract Meaning Representation (AMR), a graph, written as Ein facts in a vocabulary of the analysis's own, declared at its top; each fact's :source is the sentence it was read from. A node is named by its sentence's number and the parser's variable (s4-z1). A concept is a verb's sense (drink-01), a noun (house) or one of AMR's own (person, thing, name, and_, amr-unknown, which is what a question asks). The roles ARG0 to ARG5 are a sense's participants, the doer first, in the same places whether the sentence is active or passive; amr-negative is a negation; a name's words are joined by _, as in Old_Gold; quant is a quantity. A word of Ein's own that AMR writes is followed by _, as and_. The parser can read a sentence wrong. The (solve :no-search) at the end says that the analysis states every fact there is. These facts are for reading: the statements you write are the text's, in your own relations, and none of these facts is one of them.{{entities}}

```ein
{{analysis}}
```
