# S1f.6.9 — Width: overflow detection, width extension, and numbers written in reverse order

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08. **Stops for the user** on the representation before building it.
**Depends on:** [S1f.6.6](s1f.6.6_division.md). It may run beside S1f.6.7 and S1f.6.8 where it changes no file
they touch.

## What it settles

The user: *"with overflow detection or width extension if it is possible, I'm not sure now but maybe relation
could have variadic arguments then can write numbers in reverse order e.g. (num 8 7 0 3 2 1) == 123078"*.

1. **Overflow detection, audited.** Each operation reports overflow as S1f.6.1 ruled, and the audit checks
   every operation and extension. A test holds that no result is ever a truncated number with no flag.
2. **Width extension within the fixed form.**
   - Widening: `num9` to `num18` by padding zeros on the left.
   - Operations across two widths.
   - If the user wants them, hardware's double-width results: a sum's carry, a product's high half.
   - Whether one set of rules can serve every width while each width is its own relation; if not, the rules per
     width are generated, as the tables are.
3. **The representation, three candidates:**
   - **(a) Fixed width, `(num9 …)`.** Today's form; the width is the relation.
   - **(b) A nested little-endian list,** `(d 8 (d 7 (d 0 (d 3 (d 2 (d 1 nil))))))` for 123078.
     - It needs no language change: a rule matches a head digit and a tail.
     - Carries pass from the head down the tail, so the least significant digit comes first, the order the
       user proposed.
     - Any width up to the generated parser's nesting limit.
     - The circuit is recursive: one rule per operation and case, not one per column.
   - **(c) A variadic relation,** `(num 8 7 0 3 2 1)`, written flat.
     - This is a grammar and matcher change. A relation would take any number of arguments, and a pattern
       would match a first argument and *the rest*, which is list matching, and Ein has none.
     - It needs the user's ruling and its own stage. The grammar is final at P1f.5's close, so this stage only
       specifies it.

## Tasks

1. **The overflow audit and its test.**
2. **Width extension,** for the ruled fixed form, with its tests.
3. **The representation, on a prototype.** Addition, and multiplication if it is reachable, in (b), measured
   against (a) by firings and rounds on the same operands. Then (c) written as a grammar proposal with real
   programs: what a rule over a variadic number would look like, and what the matcher would need.
4. **The stop.** Show the user (a), (b) and (c) with their programs and measures, and recommend one.
5. **After the ruling:**
   - (a) stays: nothing more.
   - (b): the four operations and both extensions move to it, or gain conversions between the two forms, as
     ruled.
   - (c): a stage is specified for the grammar change, outside this phase.

## Acceptance

- The overflow audit and its test.
- Width extension for the fixed form, tested.
- The user's ruling on the representation, recorded with their words, and built or specified as they ruled.
