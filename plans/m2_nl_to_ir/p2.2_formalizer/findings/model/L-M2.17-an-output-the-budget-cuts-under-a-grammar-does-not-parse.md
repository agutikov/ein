# L-M2.17 — An output the budget cuts under a grammar does not parse, and ends the run

**Status:** closed by S2.2.5 (`8c2037b0`) for the one shot: an output the budget cut is read as the forms written before the cut
**Found:** by S2.2.4, 2026-11-26, in its runs of the steps with the model (`7aaefe6f`)
**Owner:** [S2.2.5](../../s2.2.5_from_text.md), whose command reads a step's output; and the loop,
[P2.4](../../../p2.4_loop/README.md)

## What

**A grammar holds every token to a text that can still become a program; it does not end one at the budget.** A
generation that repeats a form, a premise or a comment until its budget of generated tokens stops mid-form, inside
the fence, and what it wrote does not parse: *unexpected end of input, repair: Insert `)`*. The stage's plan said a
reply that does not parse under a grammar is a bug of the grammar; these are not: every token was the grammar's, and
the text is a prefix of one it allows. In S2.2.4's runs under the steps' grammars: `boxes12`'s step 3 (a rule whose
`:match` repeats `(sum-of ?a ?b ?t)` and a numeral to the budget) and the one call on the Zebra puzzle (the fact
`(source-instance House-1 :source "The Norwegian lives in the first house.")` written 718 times from the output's ninth line
to the budget of 12 288, after five memberships of the houses). A step's output that does not parse ends the run ([`einaf::formalize`](../../../../../einaf/src/formalize.rs)),
and where it is step 3's, the program of steps 1 and 2 is the complete one.

## Evidence

[S2.2.4's record](../../steps/README.md): `runs/steps/boxes12/step3.out.ein` and `runs/whole/zebra/stepwhole.out.ein`,
each 12 288 tokens, the stop `max-tokens`, the parse's one error at the text's end.

## What it costs

A call's whole budget, and the forms the output did write before the loop: a reader that took the output up to its
last complete form would have them, a program less the loop's last form. No reader does today; it is a choice of
the command's, which S2.2.5 makes, recorded as a default.

## What was done

S2.2.5's choice, a default of its record: **`einaf from-text` reads an output the budget cut as the forms written
before the cut** (`einaf::formalize::Cut::KeepForms`): every top-level form the output completed, the one the cut
stopped in dropped, every exact repeat of an earlier form dropped (the loop), the comments between kept; a step that
keeps no form ends the run as one that does not parse does. `einaf steps run` keeps the reading above (`Cut::End`), so
that S2.2.4's records replay as recorded. On S2.2.5's 29 runs, 8 calls ran to the budget: under the library alone
`unused`'s step 2 (21 forms kept, and step 3 wrote over them), `boxes12`'s step 3 (1 kept), `boxes13`'s step 2's second
call and step 3 and `pencils`' step 3 (none kept: a form never closed, or comment lines alone); with the word theory's
modules given, `boxes12`'s step 2, `boxes13`'s step 3 and `bystander`'s step 2, whose 15 forms kept gave the one right
numeric answer of the stage ([S2.2.5's report](../../from_text/README.md)). No exact repeat of a form was dropped:
the loops were inside a form or in comment lines.

