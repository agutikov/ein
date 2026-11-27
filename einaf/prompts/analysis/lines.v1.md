%% The analysis as text, a node a line: each sentence's AMR graph under the sentence, each node's line its
%% variable, its concept, its word where the parser aligned it by what it learned, and the roles it has.
%% The slot analysis of steps/step1-analysis.v1.md; the form is docs/einaf/09-analysis.md § 3's.
An analysis of the text, written before you by a trained parser and not by a person: each sentence's meaning as an Abstract Meaning Representation (AMR), a graph written a node a line under the sentence. A line is a node's variable, its concept, and each role it has with its target, the variable of another node, a value or a "string". A concept is a verb's sense (drink-01), a noun (house) or one of AMR's own (person, thing, name, and, amr-unknown, which is what a question asks); where the parser says which word it read a node from, the word follows the concept in brackets. :ARG0 to :ARG5 are a sense's participants, the doer first, in the same places whether the sentence is active or passive, and every role is written on the node it starts from; :polarity - is a negation; a name is a name node whose words are its :op1, :op2 and so on; :quant is a quantity. The parser can read a sentence wrong, and the statements you write are the text's.{{entities}}

```
{{analysis}}
```
