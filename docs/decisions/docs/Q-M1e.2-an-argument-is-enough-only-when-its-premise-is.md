# Q-M1e.2 — An argument is enough only when its premise is enforced

A claim may be settled by a written argument rather than by a check only when the premise the argument rests on is
itself enforced: checked by something that fails when the premise stops being true. The check must enforce the
premise itself and not something next to it: an argument that cites a real, green, well-named test is unsupported
when that test asserts a different claim. The argument is written beside the code it is about, where the next reader
of that code looks, and not in a record of the work.

**Why.** An unenforced premise is not probably true. Each one that was probed failed: *`dead` is monotone* fell to a
twenty-line program with an `absent`, *no rule asserts a name the program lacks* to an eleven-line one, and the third
was cited to a test that unioned equality classes by hand and asserted something else, which left the premise with
no check at all. A citation of the wrong test is harder to see than no citation, because it is what stops anybody
looking. Together with [Q-M1e.1](Q-M1e.1-a-behaviour-is-refuted-by-a-banked-probe.md) it is one rule read from two
ends: a risk is not argued away, and this is what an argument must stand on where one is accepted.

**Held by.** `docs/ein/standard-of-proof.md`, Rule 2 and its table; the alive-set invariant, a premise first
argued and now enforced, by `ein-infer/tests/alive_invariant.rs`.
