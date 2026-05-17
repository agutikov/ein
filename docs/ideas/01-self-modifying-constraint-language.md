# Self-modifying constraint language with LLM ↔ harness loop

## The idea, in one paragraph

Put into the LLM's system prompt:
1. the GBNF syntax (its meta-grammar);
2. the *currently applied* output-constraint grammar, which itself
   includes a sub-language for editing the constraints;
3. an explanation that an external harness will apply any
   modifications the LLM emits — so the LLM controls its own output
   syntax;
4. a task on the syntax itself (e.g. "optimise the language for
   proving theorems about graphs").

Then loop the LLM's output back into its input. Syntax is enforced by
GBNF; semantics (compileability / interpretability) is the open
question.

## User's own words

> What could system prompt look like if I want to put there:
> - GBNF syntax itself
> - current applied syntax of output constraints in gbnf, that include
>   language for updating the the constraints
> - explanations for llm that external harness will apply modifications
>   if it will output them (it will - so llm can control its own
>   output syntax
> - request to do something with the syntax, for example optimize it
>   for something, problem set is not yet defined need to investigate,
>   for example invent the optimal language for graph theorems
>   proving,
>
> Then cycle llms output into input.
> Dont need to control syntax because it will be done with gbnf.
> But semantics - something like compileability/interpretability, how
> to control it?
> How gbnf editing language could look like?
> What is better - simple syntax like lisp with heavy semantics on
> atoms meaning, or more expressive syntax like haskell or logic pl or
> csp languages or something else?

## Architecture sketch (extracted from the description)

```
        ┌────────────────────────────────────────────────┐
        │ system prompt:                                 │
        │   - GBNF meta-grammar                          │
        │   - currently applied grammar G_t              │
        │     (which includes a sub-language for         │
        │      proposing updates to G_t)                 │
        │   - "harness will apply your modifications"    │
        │   - task on the syntax / problem set           │
        └────────────────────────────────────────────────┘
                              │
                              ▼
                            LLM (constrained by G_t)
                              │
                              ▼
                emits: content + grammar-edit terms
                              │
                              ▼
                   external harness
                  ┌───────────────────┐
                  │ validate edits    │
                  │ recompile → G_{t+1}│
                  │ run semantic      │
                  │   checker (?)     │
                  │ score / accept    │
                  └───────────────────┘
                              │
                              ▼
                       back into the LLM
```

## The open questions the user is actually asking

1. **How do you control semantics when GBNF only controls syntax?**
   Compileability? Interpretability? A separate verifier? A type
   system? An interpreter?
2. **What does the grammar-editing language look like?** Is it a
   *direct* GBNF mutator or a higher-level DSL that compiles to GBNF?
3. **Surface-syntax choice for the LLM-emitted IR**:
   simple S-expression with heavy semantic load on atom names? vs
   Haskell-style? vs logic-programming style? vs CSP-style?
4. **What is the actual task** to give the loop? E.g. "invent the
   optimal language for graph theorem proving" — a placeholder; the
   problem set still needs to be defined.
5. **Stability**: how to prevent the loop from drifting into private,
   non-interpretable, locally optimal-but-meaningless dialects (the
   "private-language" failure mode in multi-agent systems).

## Why this is the most ambitious idea in the raw set

It combines:

- **GBNF / grammar-constrained decoding** (token-level syntax
  enforcement).
- **Reflective / meta-circular evaluators** (the artefact under
  construction is the constraint system itself).
- **Self-modifying formal languages**, with all the risks that
  implies.

Closest existing references (per the discussion that followed):

- SMT-LIB / Z3 (S-expression IR + strict semantics — the cleanest
  template).
- PLT Redex (DSL for grammar + operational semantics + reduction
  rules).
- K Framework (executable language semantics + analysis / proof).
- ACL2 (Lisp + provable execution semantics).
- miniKanren (relational programming kernel; small, clean).
- *Not* arbitrary direct GBNF mutation — better an immutable kernel +
  a high-level update DSL that compiles to GBNF + a typechecker /
  semantic checker (a "semantic firewall").

## Constraints / non-negotiables the user implied

- **GBNF stays as the syntactic firewall.** It is necessary but not
  sufficient.
- **Cycle output → input is the central loop.** The system is a
  *protocol-evolution* loop, not a single-shot generator.
- **The LLM is allowed to modify its own constraints** (within the
  harness's veto power).

## Connections (context, not answers)

- LLM-side mechanics and existing constrained-reasoning frameworks:
  [01-llm-constrained-generation.md](../index/01-llm-constrained-generation.md).
- Surface-syntax recommendation (S-expression / homoiconic):
  [04-programming-languages.md](../index/04-programming-languages.md).
- Semantics layer choices (Z3, miniKanren, K, PLT Redex, ACL2):
  [02-solvers-csp-sat-smt.md](../index/02-solvers-csp-sat-smt.md),
  [03-theorem-proving-formal-methods.md](../index/03-theorem-proving-formal-methods.md).

## Concrete next step possibilities

(Listed so the idea isn't lost as pure philosophy — none committed.)

- A minimal "kernel-plus-update-DSL" prototype: tiny S-expression
  IR + a `(language-update …)` form + harness that validates &
  recompiles to GBNF.
- A *bounded* problem domain: pick "language for theorems about
  finite directed graphs" (or even just "language for Zebra-style
  puzzles") so the loop has something concrete to optimise.
- Versioned grammar with rollback, so divergent dialects can be
  compared instead of overwritten.
