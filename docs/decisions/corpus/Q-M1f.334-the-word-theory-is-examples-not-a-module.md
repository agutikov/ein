# Q-M1f.334 — The word theory is examples, not a module of the stdlib

The theory of what the words of a school problem mean lives in `examples/word_problems/`, its modules beside the
problems that import them by file name, and not in the stdlib.

**Why.** Imported into a program, the theory occupies ordinary words, `has`, `gave`, `holds`, `asked`, `value`,
`together`, which a program may want for its own relations; and it covers the readings its problems need and no
more ([Q-M1f.335](Q-M1f.335-the-word-theory-holds-only-the-readings.md)). A module of the stdlib is a theory every
program may import, with a test program per rule.

**Held by.** `examples/word_problems/README.md`; the modules `words.ein`, `gaps.ein`, `readings.ein` and
`listed_is_all.ein`, imported by file name; `stdlib/`, which holds none of them.
