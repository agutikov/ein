# E-119 — A relation's `:why` that names an arrow its declaration does not have loads, and prints the reference as text

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.2, 2026-11-26, on `35fcc5af`
**Owner:** none; the design: [§ 2.11 of the diagnostics](../../../../../docs/ein/user/04-reference/07-diagnostics.md#211-a-template-reference-no-reference-can-reach)

## What

**A model counts a relation's arrows from its arguments, not from the declaration.** The probe's step 1 on two word
problems writes `(relation has Person Object :why "{?3} has {?4} of {?5}")`, `(relation gave Person Person Object
:why "{?3} gave {?4} {?5} to {?6}")`, and four more such: six declarations whose `:why` names an arrow past the last
label, `{?5}` of a declaration of two labels, because the facts the model meant (`has Bob 5 apples`) have one more
argument than it declared. A relation's `:why` names its declaration's arrows by number and **is not read** at the
load: a reference no arrow has stays text, and the program loads with nothing said. Its facts then read, where a
rule prints the declaration, *"Person has Object of {?5}"*.

## Evidence

The probe's record, `plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/probe.jsonl.gz`, the lines `step1.off.apples`
(`has`, `gave`, `amount`) and `step1.off.more` (`has`, `more`, `diff-of`), their fenced readings, which load. `ein
solve` of `(relation has Person Object :why "{?3} has {?4} of {?5}") (has Bob apples) (solve :no-search)` exits 0
and says nothing.

## What it costs the application

The declaration's text is the one place a model writes what it means a relation's places to be, and a reference
past the last label is the model saying that the relation has more places than it declared: the facts it meant
have another count ([E-12](../../../evaluation/language/E-12-an-undeclared-relation-and-a-form-of-a-declared-relation-at.md)).
A load that read a relation's `:why` against its labels would name the mismatch where the model made it; today the
contract's check has to.
