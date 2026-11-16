# The standard of proof

What it takes to call a claim about Ein *settled*. Two rules, one read from each end: what refutes a claim, and when
an argument is enough to accept one. They apply to any claim about the engine, whatever states it: a page of
`docs/ein/`, a comment beside the code, a review. The pages of `docs/ein/` are the one statement of what Ein is meant
to do that is not also the implementation, and a claim on them is worth what checks it.

Both rules are decisions, and their files are where they are held:
[Q-M1e.1](../decisions/docs/Q-M1e.1-a-behaviour-is-refuted-by-a-banked-probe.md), *what counts as refuted*, and
[Q-M1e.2](../decisions/docs/Q-M1e.2-an-argument-is-enough-only-when-its-premise-is.md), *when an argument is enough*.
A third holds numbers to the same standard:
[Q-M1e.4](../decisions/docs/Q-M1e.4-an-exact-number-in-a-page-carries-the-command.md).

## Rule 1 — what counts as **refuted**

| the claim is about… | it is refuted by… |
|---|---|
| a **behaviour**: *the engine does X* | an **executed probe, kept as a test** that fails if the behaviour appears. Not a reading of the source, not an argument from the design |
| an **absence**: *nothing checks Y* | **naming the thing** that checks it: the test, or the refusal |
| a **risk**: *X could happen* | **nothing.** A risk is not refuted by argument |

The third is the one that will be argued with, so it is a closure rule: a risk has exactly three honest outcomes:
**fixed**; **accepted**, with the argument written beside the code it is about (Rule 2); or **postponed**, with what
it waits for. *"I could not build it"* is not a refutation: it is a probe that did not reproduce, and it is kept as
one.

## Rule 2 — when an **argument** is enough

> **An argument suffices when its premise is itself enforced.**

*Accepted* is a first-class outcome, and a written argument is often the right one. What makes it acceptable is not
its plausibility but whether the thing it rests on is checked by something that fails when it stops being true.

Four premises Ein's answers rest on, and what holds each:

| premise | the argument it carries | held by |
|---|---|---|
| everything Ein lists, it lists in a canonical order | a run's output is the same run after run, and under any job count | the ordering tests, which fail on an order that differs; [Q-M1a.7](../decisions/engine/Q-M1a.7-a-job-count-moves-nothing-a-run-prints.md) |
| *dead is upward-closed*: a commitment that derives `(false)` stays dead under every larger one | no state above a dead commitment is in the answer (the read-out's line `L1`) | a **refusal**: the one shape that breaks it, a refutation resting on an `absent` a commitment can still fill, is refused at load ([Q-M1e2.5](../decisions/language/Q-M1e2.5-a-refutation-may-not-rest-on-an-absent.md)) |
| *no rule asserts a name the program lacks* | the candidates Ein can guess are made of the program's names alone | a test that fails when a run reaches a name outside them ([Q-M1e.21](../decisions/hypotheses/Q-M1e.21-a-rule-brings-in-no-name.md), [Q-M1e2.9](../decisions/language/Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)) |
| *names are never merged*: equality is name identity | no match reads past a name to an equal one | a test that the store's equality classes stay inert and no matcher reads them ([Q1](../decisions/language/Q1-the-knowledge-base-is-a-typed-labelled.md)) |

Each of the last three was once a premise nothing enforced, and each failed when it was probed: the first two by a
small program, the third by a probe that the test then cited for it left green. Three unenforced premises probed, and
three wrong, is not a sample; it is the only evidence there is about what an unenforced premise is worth.

**The third failed differently, which is why it has its own sentence.** Its premise may well have been *true*; what
was false was the **citation**. The test cited for it united two names by hand and then asserted that the *matcher*
ignores the union, a different claim, and a probe that made the store unite names on every stored fact left it green.
So Rule 2 has a second question, and it is the one that is easy to skip: **does the named test enforce the premise,
or something next to it?** An argument that cites a real, green, well-named test can still be unsupported, and that is
harder to see than an argument with no citation, because the citation is what stops anybody looking. When the answer
is *next to it*, the citation is not evidence that the argument is enough; it is evidence that nobody has checked.

## Where an argument goes

**Beside the code it is about.** The next reader of the code has the same question, and finds the answer where the
question arises. A rule *about* claims, like this page, sits beside the pages whose claims it is about.

## The worked example

The premise *dead is upward-closed* reads as a definition: if commitment X is dead and X ⊆ Y, then Y is dead, because
saturation only adds facts and nothing retracts. It is false for a program like this one:

```lisp
(relation is-a T T :why "{?1} is a {?2}")
(relation p T :why "{?1} has p")
(relation q T :why "{?1} has q")
(is-a A T)
(rule bad ()
  :match  (and (p ?x) (absent (q ?x)))
  :assert (false)
  :why    "{?x} has p without q")
```

`{(p A)}` derives `(false)`, since `(q A)` is missing, and `{(p A), (q A)}` does not: the larger commitment is alive.
The rule calls a state that has not yet decided `q` a contradiction. Nothing in the definition said so, and several of
the engine's conclusions rested on the premise. So the premise is enforced the only way it can be: Ein refuses the
shape at load, `rule 'bad': concludes (false) from (absent …) over q, a relation a commitment can still extend`, and
names the forms that say what was meant: close the relation, state the requirement as an obligation, or conclude from
a stored negative ([Q-M1e2.5](../decisions/language/Q-M1e2.5-a-refutation-may-not-rest-on-an-absent.md);
[`examples/ein-bugs/naf-upward-closure.ein`](../../examples/ein-bugs/naf-upward-closure.ein) is the program, kept as a
test). The premise was written down, believed, load-bearing and wrong. That is what Rule 2 is for.

## See also

- [The diagnostics](user/reference/diagnostics.md): what the engine refuses and prints, as opposed to what it takes
  to know it.
- [Solutions and models](user/syntax-and-semantics/solutions-and-models.md): the answer, whose lines are premises of
  this kind.
- [`absent` and negation as failure](../kernel/inference/absent_semantics.md): the corollaries the upward-closure
  premise is read beside.
