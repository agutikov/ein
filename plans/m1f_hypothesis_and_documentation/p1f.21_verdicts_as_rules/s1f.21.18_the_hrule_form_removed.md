# S1f.21.18 — the `hrule` form and the keyword `:hrules` removed

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 on the user's ruling of [Q-M1f.567](../open_questions.md#q-m1f567--the-hrule-form-dropped-for-a-rule-that-concludes-open-f): *"The special form (hrule ...) is not required
anymore so we remove it and keyword :hrules."* **Not started**: it starts on the user's word.
**Waits on:** [S1f.21.12](s1f.21.12_hypotheses_as_facts.md), which makes a rule that concludes `(open F)` a
hypothesis rule and reads the form as one.
**In place of:** [S1f.21.16](s1f.21.16_hrules_lists_activators.md), which the user dropped (*"not start S1f.21.16,
will drop it after :hrules removed"*): its stage file is deleted with this stage's last commit.
**Writes:** every program, page and test that writes the form or the clause; the grammar and the loader;
`ein-infer/src/hrule.rs` and `mechanisms.rs`; `utils/gen_zebra2_variants.py` and the other scripts that name it.

## What it does

A hypothesis rule is written as a rule, and its activators as facts:

```lisp
;; before                                         ;; after
(hrule guess (?R ?T)                              (rule guess (?R ?T)
  :match  (is-a ?x ?T)                              :match  (is-a ?x ?T)
  :assert (?R ?x)                                   :assert (open (?R ?x))
  :why    "guess: is {?x} {?R}?")                   :why    "guess: is {?x} {?R}?")
(solve :hrules (guess (chosen Drink)              (guess chosen Drink)
                      (served Drink)))            (guess served Drink)
```

An activator is a fact the program writes, anywhere in it (the user: *"by "Activate anywhere" I meant in program,
not necessary in :hrules list"*). Then the form `hrule` and the keyword `:hrules` are deleted: no second spelling
is kept (the phase's rule, no backward compatibility).

## What it touches, counted today

| what | count | the command |
|---|---:|---|
| `.ein` files under `examples/`, `tests/` and `stdlib/` that write `(hrule` | 119 | `grep -c '(hrule ' -r examples tests stdlib --include='*.ein' \| awk -F: '$2>0' \| wc -l` |
| of them, files that write the `:hrules` clause | 53 | `grep -rl ':hrules' examples tests stdlib --include='*.ein' \| wc -l` |
| pages under `docs/` that name `hrule` | 93 | `grep -rl 'hrule' docs --include='*.md' \| wc -l` |
| crate sources and tests that name it | 112 | `grep -rl 'hrule' ein.rs/crates --include='*.rs' \| wc -l` |
| scripts under `utils/` | 16 | `grep -rl 'hrule' utils \| wc -l` |

The four generated zebra files are `utils/gen_zebra2_variants.py`'s, and two of them are *`zebra2` with the
`(hrule guess …)` and the `:hrules` clause deleted and nothing else*: the generator and that sentence of
`AGENTS.md` are written again for the new spelling.

## Tasks

1. **The rewrite, by a script**, every program at once; then **the invariance**: each program's `ein show hyps`
   and its summary's `region` before and after, the same, since S1f.21.12 reads the form as the rule it becomes.
   A program that differs is shown, not banked.
2. **A hypothesis rule that is declared and not activated** (a clause that names none of it) becomes a rule with
   no activator: nothing to write. A program that activates one hypothesis rule from several declared keeps the
   others unactivated.
3. **The form and the keyword deleted**: the grammar, the loader, `hrule.rs`, the conflicts of `mechanisms.rs` that
   name a hypothesis rule, the events' and `ein show hyps`' rung name; the refusals that go and the ones that
   stay, each with its row of the diagnostics page.
4. **The pages**, with [S1f.21.14](s1f.21.14_the_pages_of_open.md): the reference, the guide, the reasoning pages'
   ladder; the decisions the ruling supersedes, rewritten to the id that carries it.
5. **The scripts and the generators**, each with its `--check`.

## What must hold

$R$ of every corpus program as S1f.21.12 left it; every mode against the oracle; `ein-cli/tests/import_invariance.rs`;
no `hrule` left in a project file but where a page says the form is gone, by `grep`; `check_generated.py`.

## Acceptance

`grep -rn 'hrule' ein.rs/crates/ein-ir/src/ein.y` prints nothing; a program that writes `(hrule …)` or `:hrules` is
refused with the parse's own message; the gate, the nightly and the container gate.

## Record

Nothing yet.
