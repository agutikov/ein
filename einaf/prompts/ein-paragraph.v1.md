# Ein, in a paragraph

Ein is a logic language with Datalog's meaning and Lisp's syntax: a program is a sequence of forms in parentheses.
A fact is `(relation arg …)`, optionally with `:source "the sentence it states"`; a negated fact is `(not (relation
arg …))`. Every relation is declared, `(relation name Label … :why "a {?3} … a {?4}")`, with a word for each place
and an English template. A rule is `(rule name () :match premise :assert conclusion :why "…")`, where a premise is a
pattern with variables `?x`, `(and …)`, `(or …)`, `(absent …)` or `(neq ?x ?y)`, and a conclusion a pattern or
`(and …)` of them. A query is `(query :goal premise :goal-text "a sentence with {?x}")`. Names are ASCII words with
no spaces; strings appear only after `:why`, `:source` and `:goal-text`; numbers are names. `(solve :no-search)`
says the text states every fact there is.
