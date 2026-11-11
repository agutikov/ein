# P1g.1 — Survey and design: what Ein represents today, shown on real programs

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** M1f complete; of P1f.6,
[S1f.6.17](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md)'s gates and its
ruling on their names.
**Stops for the user:** the whole phase is one stop. Its page, `design.md` in this directory, is read and
ruled on before any other phase writes a file outside `plans/` and `build/`.

## What it is for

The instruction asks five things that a design must answer before a theory is written:

> from binary logic to preposition logic, what can be represented already and how? […] how preposition logic
> formulas can be represented? rules of inference? what are hypothesis in this case? […] First order logic,
> what we can implement already and how, what is nearly possible, what is required.

The phase answers each **on a program whose answer the binary computed**, the way
[S1f.6.1](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.1_design_proposal.md) did for the
calculator: a prototype under `build/m1g/` (gitignored), a page that shows each program with every artefact
of its run, the questions each with a recommendation, and nothing under `stdlib/`, `examples/` or `tests/`
until the user rules. It sorts what it tries into three piles, which are the instruction's own:

- **possible today:** a program in the shipped grammar, with its answer;
- **nearly possible:** a program that works with a detour, and the detour named (a marker fact where a list
  takes no value; a context carried as data where a rule cannot read another state);
- **requires the engine:** a program that cannot be written, or that does not stop, or whose answer cannot be
  seen, with the smallest change that would allow it. These are the rows of
  [P1g.6](../p1g.6_engine_requirements/README.md).

The planning agent's six scratch probes ([M1g § First probes](../README.md#first-probes-2026-11-11)) are a
first reading of the first pile and of two rows of the third. The phase re-takes them as kept programs and
does not rely on them.

## The stages

| ID | title | depends on | status |
|---|---|---|---|
| [S1g.1.1](s1g.1.1_what_ein_represents_today.md) | What Ein represents today: one program per subject, each with its computed answer and every artefact | S1f.6.17 | planned |
| [S1g.1.2](s1g.1.2_two_levels.md) | Two levels: Ein's rules as rules of inference, and a logic written in Ein; what a hypothesis is in each setting | S1g.1.1 | planned |
| [S1g.1.3](s1g.1.3_conventions.md) | The conventions: a formula's shape, the connectives' names, the truth values, the request protocol, and where things ship | S1g.1.1 | planned; three questions **for the user** |
| [S1g.1.4](s1g.1.4_engine_requirements_and_the_design_page.md) | The engine requirements, and the design page the user reads | S1g.1.1 to .3 | planned; **stops for the user** |

## The order

S1g.1.1 first; S1g.1.2 and S1g.1.3 beside each other on its programs; S1g.1.4 gathers the three into
`design.md` and stops.

## Acceptance

- `design.md`: every program's answer computed, every artefact linked (`ein solve`'s read-out, `ein saturate
  --dump`, `ein test` on its `:expect`, `--trace`, and the views `ein render` draws), every number with its
  command and date.
- Each of the instruction's questions has a section that answers it, with the program it rests on.
- The three piles, each item with its program or the reason it has none.
- The list of engine requirements, each with its witness, and P1g.6's stage list restated to it.
- The questions for the user, each with a recommendation, and the user's rulings recorded verbatim under
  `design.md` § The rulings, as the calculator's are.
- The commit is docs only. The prototype is under `build/`.

## What it may find

- That something the plan files under *nearly possible* is a wrong answer of the engine. It is then a row of
  `corpus/wrong_answers.toml` before anything else is done with it.
- That a pile is wrong: the plan's phases P1g.2 to P1g.5 are written against the coordinator's expectations,
  and the stop is where they are corrected. A phase whose premise the survey refutes is re-planned, not run.
