#!/usr/bin/env python3
"""Write the deep-term probe for a chain of N steps to stdout.

A chain of N `step` facts, and a rule that nests a term one deeper at each
step, so that root's fixpoint holds a term N deep, which the query binds.
`(solve :no-search)` keeps the blind enumerator off the chain's N names, so
the term's depth is the one cost.

    python3 deep_term.py 5000 > /tmp/deep_term_5000.ein
"""

import sys

n = int(sys.argv[1]) if len(sys.argv) > 1 else 5000
print(""";;; a term nested one deeper at each of a chain's steps: root's fixpoint
;;; holds a term as deep as the chain is long, and the query binds it
(relation at N N :why "{?3} is reached by {?4}")
(relation step N N :why "{?3} steps to {?4}")
(rule deeper ()
  :match (and (at ?a ?t) (step ?a ?b))
  :assert (at ?b (s ?t))
  :why "a step nests its count one deeper")
(at a0 zero)""")
for i in range(n):
    print(f"(step a{i} a{i + 1})")
print(f'(query :goal (at a{n} ?t) :goal-text "{{?t}}")')
print("(solve :no-search)")
