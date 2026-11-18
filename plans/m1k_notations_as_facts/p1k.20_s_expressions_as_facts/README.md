# P1k.20 — arbitrary S-expressions as Ein facts

**Milestone:** [M1k](../README.md)
**Status:** created 2026-11-18 on the user's instruction (below); **not started without the user's word**.
**Depends on:** nothing; P1k.10's examples are its pattern
([`s1k.10.1_examples.md`](../p1k.10_graph_to_kb/s1k.10.1_examples.md)).

## The instruction

The user, 2026-11-18, verbatim:

> p1k.20: arbitrary S-expressions as Ein facts: Lisp, SMT, link grammar output; same approach - just pick random files
> (from repo and outside), parse as Ein and render ;

## What it is

An Ein program is S-expressions, and a form whose head is a name and whose arguments are names or forms is a fact,
nested where an argument is a form: `(owns Ann (pet Dog))`. Much of what Lisp, SMT-LIB and link grammar write has the
same shape. The phase takes such files as they are, parses them as Ein, loads and draws what loads, and records
what reads, what does not, and why; *"same approach"* as P1k.10's examples: real files, a prototype only where a
repair is shown, the binary for every verdict.

**Where to look**, a first list for the survey:
- in the repo: `nl2smt/smt/*.smt` (SMT-LIB, hand-written), the SMT and link grammar output `nl2smt/` writes and
  tests, its Racket converter's sources, Ein's own programs (`examples/`, `stdlib/`) as a control;
- outside: SMT-LIB benchmarks, Scheme, Racket, Common Lisp, Clojure and Emacs Lisp sources, link grammar's
  constituent output (`(S (NP …) (VP …))`), and other S-expression notations (KIF, PDDL, s-expression dumps of ASTs)
  if the survey finds them worth a row.

**What the survey expects to meet**, each to be counted on the files and shown with one case: a head that is no name
(`((lambda (x) x) 1)`, a number, a string); strings and numbers as arguments (a string is natural-language text in
Ein, only after `:why`, `:source` or `:goal-text`; `1` is a name); `;` and `#|…|#` comments; quote, quasiquote and
reader macros (`'x`, `` `x ``, `#t`, `#(…)`); dotted pairs `(a . b)`; keywords `:k` (Ein's keywords are closed per
form); vectors `[…]` and `{…}`; `|a b|` symbols and characters outside Ein's names; `?x`, a variable in Ein; `_`,
refused; a head that is one of Ein's own words (`rule`, `relation`, `query`, `solve`, `config`, `import`, `not`, `and`,
`or`, `eq`, `neq`, `false`, `absent`); `()` and a form of one word; nesting past the parser's limit; one relation at
several counts of arguments (Ein holds a declared relation to its count; an undeclared one may vary).

## Stages

| stage | what | state |
|---|---|---|
| S1k.20.1 | **The examples**: random files from the repo and outside, each parsed as Ein, loaded and drawn (`ein render kb`), as they are and after the smallest repair a prototype shows; a table of what read and what did not and why, a section per failure kind with counts, and what the evidence suggests for a reader | not started |
| S1k.20.2 | **The design**, on the user's rulings: what reads an S-expression file (`ein show`, beside P1k.10's DOT), and what each failure kind does | not started |
| S1k.20.3 | **The build** | not started |
| S1k.20.4 | **The close** | last |

## Rules of the phase

The milestone's ([`../README.md`](../README.md)): third-party files are read and never run, kept in an ignored folder
under `build/` with their URLs, and only what the stage writes itself is committed.
