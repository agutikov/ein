%% The analysis as text, in PENMAN: each sentence's AMR graph as the parser printed it, the aligner's
%% markers removed, under the sentence. The slot analysis of steps/step1-analysis.v1.md; the form is
%% docs/einaf/09-analysis.md § 3's.
An analysis of the text, written before you by a trained parser and not by a person: each sentence's meaning as an Abstract Meaning Representation (AMR), a graph in PENMAN notation under the line # ::snt and the sentence. A node is (variable / concept ...): a concept is a verb's sense (drink-01), a noun (house) or one of AMR's own (person, thing, name, and, amr-unknown, which is what a question asks). :ARG0 to :ARG5 are a sense's participants, the doer first, in the same places whether the sentence is active or passive, and :ARG0-of is that role read from its other end; :polarity - is a negation; a name is a name node whose words are its :op1, :op2 and so on; :quant is a quantity. A variable written again is the same node. The parser can read a sentence wrong, and the statements you write are the text's.{{entities}}

```
{{analysis}}
```
