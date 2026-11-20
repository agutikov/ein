# Q-M1f.332 — A word with two readings always has one chosen

A word a problem can mean two ways, *need* in *how many boxes do they need?*, has its readings as hypotheses, and a
reading is always chosen: `readings.ein`'s obligation `reading-owed` makes a state with such a question and no
reading owe one, so root is not in R, and every state of R names the reading its answer rests on. Where both
readings give the same answer, R holds two states and one answer.

**Why.** An answer depends on the operations that compute it, and which operations a problem asks for depends on
how its words are read; a state with no reading chosen has no answer to give. The form is also monotone, so the
program needs no search of the whole cube.

**Held by.** `examples/word_problems/boxes12.ein` and `boxes13.ein`, whose region claims list the two readings'
states; `readings.ein`'s obligation `reading-owed` and its hypothesis rule `read-need`.
