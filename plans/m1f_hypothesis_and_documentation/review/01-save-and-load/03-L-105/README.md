# L-105 — `utils/nl_census.py` reads state-file comments that no longer exist

## 1. The question

`utils/nl_census.py` measures how much of what Ein answers reads as English. Its *answer half* solves each corpus
entry with `ein solve FILE --dump-answer DIR`, then reads every state file the run wrote, finds the facts below the
`;; ── the state` marker and each fact's comment, and counts per fact whether it has a **statement** (its relation's
`:why`), a **reason** (a `:source` or its rule's `:why`) and so is **rendered**, by register. A saved file has no such
marker and no fact, so on today's binary the census finds no fact and reports zeros, silently. The call: the census
reads `ein saturate --dump` of each saved file and `--trace` for the English, or its answer half is retired with
[Q-M1f.414](../01-Q-M1f.414/README.md)'s (a).

Recorded: [`findings.md`](../../../findings.md#the-release-voice-where-the-check-does-not-read-and-the-gate), row
**L-105**; its witness is `grep -n 'STATE_MARK' utils/nl_census.py`.

## 2. What is so today

**The script's usage** ([`runs/nl_census.help.txt`](runs/nl_census.help.txt)): two halves, the flag (the loader,
with and without a required `:why`) and the answer; `-k SUBSTR` narrows the answer half to entries whose path holds
`SUBSTR`, `--answer-only` drops the flag half. The smallest scope it takes is one corpus entry:
`--answer-only -k tests/saved/02_a_choice` ([`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein),
copied from `tests/saved/`: R is two states, root and {(note (bad a))}).

**With the earlier binary** (`ein-a8d595eb`, state files with comments):
[`runs/nl_census.before-ein-a8d595eb.txt`](runs/nl_census.before-ein-a8d595eb.txt) (output)

```text
entry                                                        states   facts  statement   reason  rendered
tests/saved/02_a_choice_with_a_fact_inside_it.ein                 2       4    100.0 %   25.0 %   100.0 %
```

**With today's binary**: [`runs/nl_census.today.txt`](runs/nl_census.today.txt) (output)

```text
entry                                                        states   facts  statement   reason  rendered
tests/saved/02_a_choice_with_a_fact_inside_it.ein                 2       0        —        —         —  
```

It still counts the two state files, and finds no fact in them; it exits 0 and prints nothing on stderr
([`runs/nl_census.today.stderr.txt`](runs/nl_census.today.stderr.txt), empty). The diff:
[`runs/nl_census.before-vs-today.diff`](runs/nl_census.before-vs-today.diff); the `--json` copies and their diff:
[`before`](runs/nl_census.before-ein-a8d595eb.json), [`today`](runs/nl_census.today.json),
[`diff`](runs/nl_census.before-vs-today.json.diff).

The same on a second entry with a source, a rule and the blind enumerator's commitments,
`examples/saturation/implies/org-chart.ein` ([`programs/org-chart.ein`](programs/org-chart.ein)): 3 states, 12 facts,
75.0 % rendered (the three `(implies boss-of leads)` facts fall back) before; 0 facts today
([`before`](runs/nl_census.org-chart.before-ein-a8d595eb.txt), [`today`](runs/nl_census.org-chart.today.txt),
[`diff`](runs/nl_census.org-chart.before-vs-today.diff)).

*Read, not run*: the flag half and `--check` read the loader's refusals through `ein kb save`, not state files, so
L-105 does not touch them.

## 3. The options

**(1) The census reads `ein saturate --dump` of each saved file, and `--trace` for the English.** A change to the
script. What it would count, **by hand, a MOCK** ([`tools/mock_census.py`](tools/mock_census.py), which reuses the
census's own readers of a program, `program_names` and `register`, on today's outputs: each saved file's
`ein saturate --dump`, and `--trace`):

- [`runs/02_a_choice_with_a_fact_inside_it.census.MOCK.txt`](runs/02_a_choice_with_a_fact_inside_it.census.MOCK.txt)
- [`runs/org-chart.census.MOCK.txt`](runs/org-chart.census.MOCK.txt)

| program | read from | states | facts | statement | reason | rendered |
|---|---|---:|---:|---:|---:|---:|
| `tests/saved/02_a_choice…` | the earlier state files' comments (the census as it was) | 2 | 4 | 100.0 % | 25.0 % | 100.0 % |
| | MOCK: `saturate --dump`, the program's `relation` facts left out | 2 | 4 | 100.0 % | 25.0 % | 100.0 % |
| | MOCK: `saturate --dump`, every fact it prints | 2 | 20 | 20.0 % | 5.0 % | 20.0 % |
| `org-chart` | the earlier state files' comments | 3 | 12 | 75.0 % | 58.3 % | 75.0 % |
| | MOCK: `saturate --dump`, `relation` facts left out | 3 | 12 | 75.0 % | 58.3 % | 75.0 % |
| | MOCK: `saturate --dump`, every fact | 3 | 24 | 37.5 % | 29.2 % | 37.5 % |

Read off the dump, with the program's `relation` facts left out as the state file left them, the census counts what
it counted before, on both programs: the dump prints each fact's origin (`:source "…"`, `:rule r`, `:hypothesis N`),
and the census computes the statement and the reason from the program's texts as it did. Counting every fact the
dump prints adds the kernel's `relation` facts, which have no text, and halves the rate; so the script would have to
leave them out. The per-state dumps it reads: [`02…models-01`](runs/02_a_choice_with_a_fact_inside_it.models-01.saturate-dump.today.txt),
[`02…models-02`](runs/02_a_choice_with_a_fact_inside_it.models-02.saturate-dump.today.txt),
[`org-chart models-01`](runs/org-chart.models-01.saturate-dump.today.txt),
[`-02`](runs/org-chart.models-02.saturate-dump.today.txt), [`-03`](runs/org-chart.models-03.saturate-dump.today.txt).

What `--trace` adds is a different count, of the English the binary writes rather than the English it could write
([`02…trace`](runs/02_a_choice_with_a_fact_inside_it.trace.today.md), [`org-chart trace`](runs/org-chart.trace.today.md)):

| program | steps (a filled `:why` quote) | `Derives` with a statement | `Premises` naming a source | `Assuming` (an s-expression) | distinct facts written with a statement |
|---|---:|---:|---:|---:|---:|
| `tests/saved/02_a_choice…` | 1 | 1 | 0 | 1 | 1 of the 4 the census counts |
| `org-chart` | 2 | 2 | 1 | 2 | 2 of the 12 |

The dump says what *can* be rendered (the ladder, every fact); the trace, what *is* rendered (the derivation only).

**(2) Retire the answer half with Q-M1f.414's (a).** The English is the trace's, and the census of a state's facts
measured a surface that is gone. The flag half and `--check` stay (read, not run). `utils/check_tool_mirrors.py` reads
the script's `KERNEL` list, which both halves keep, and the plans still name the census as an instrument: M1g's
`s1g.6.3_seeing_nested_facts.md`, T1g.6.3.3 (*"the natural-language census … before and after on the calculator's
and the logic's programs"*), and M1h's `p1h.10_a_school_text/README.md` (read, not run: `grep -rn nl_census plans/`).

## 4. The default

None of its own: the row is **the user's call**, with Q-M1f.414: *the census reads `ein saturate --dump` of each saved
file and `--trace` for the English, or is retired with Q-M1f.414's (a)*. The script is as it was: today it reports
zeros.

## 5. Files

- [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein) — copied from `tests/saved/`.
- [`programs/org-chart.ein`](programs/org-chart.ein) — copied from `examples/saturation/implies/`.
- [`runs/nl_census.help.txt`](runs/nl_census.help.txt) — the script's `--help`.
- [`runs/nl_census.before-ein-a8d595eb.txt`](runs/nl_census.before-ein-a8d595eb.txt) / [`.json`](runs/nl_census.before-ein-a8d595eb.json) / [`.stderr.txt`](runs/nl_census.before-ein-a8d595eb.stderr.txt) — the census, earlier binary, `-k tests/saved/02_a_choice`.
- [`runs/nl_census.today.txt`](runs/nl_census.today.txt) / [`.json`](runs/nl_census.today.json) / [`.stderr.txt`](runs/nl_census.today.stderr.txt) — the same, today's binary.
- [`runs/nl_census.before-vs-today.diff`](runs/nl_census.before-vs-today.diff), [`runs/nl_census.before-vs-today.json.diff`](runs/nl_census.before-vs-today.json.diff) — their diffs.
- [`runs/nl_census.org-chart.before-ein-a8d595eb.txt`](runs/nl_census.org-chart.before-ein-a8d595eb.txt) / [`.json`](runs/nl_census.org-chart.before-ein-a8d595eb.json), [`runs/nl_census.org-chart.today.txt`](runs/nl_census.org-chart.today.txt) / [`.json`](runs/nl_census.org-chart.today.json), [`runs/nl_census.org-chart.before-vs-today.diff`](runs/nl_census.org-chart.before-vs-today.diff) — the same on `org-chart`.
- `runs/*.dump-answer.today/`, `runs/*.dump-answer.before-ein-a8d595eb/` — `--dump-answer` of both programs, each binary.
- `runs/*.models-NN.saturate-dump.today.txt` — `ein saturate --dump` of each of today's saved files.
- [`runs/02_a_choice_with_a_fact_inside_it.trace.today.md`](runs/02_a_choice_with_a_fact_inside_it.trace.today.md), [`runs/org-chart.trace.today.md`](runs/org-chart.trace.today.md) — `--trace`.
- [`runs/02_a_choice_with_a_fact_inside_it.census.MOCK.txt`](runs/02_a_choice_with_a_fact_inside_it.census.MOCK.txt), [`runs/org-chart.census.MOCK.txt`](runs/org-chart.census.MOCK.txt) — **MOCK** of option (1).
- [`tools/mock_census.py`](tools/mock_census.py) — the mock's counter.

## 6. Re-run

From the repo root:

```sh
D=plans/m1f_hypothesis_and_documentation/review/01-save-and-load/03-L-105
H=build/review-bin/ein-head-b5fb8d70
A=build/review-bin/ein-a8d595eb; EA=$PWD/build/wt-rev-a8d/stdlib
python3 utils/nl_census.py --help
python3 utils/nl_census.py --bin $H --answer-only -k tests/saved/02_a_choice --json $D/runs/nl_census.today.json
EIN_STDLIB=$EA python3 utils/nl_census.py --bin $A --answer-only -k tests/saved/02_a_choice --json $D/runs/nl_census.before-ein-a8d595eb.json
python3 utils/nl_census.py --bin $H --answer-only -k implies/org-chart
EIN_STDLIB=$EA python3 utils/nl_census.py --bin $A --answer-only -k implies/org-chart
for p in 02_a_choice_with_a_fact_inside_it org-chart; do
  $H solve $D/programs/$p.ein --dump-answer $D/runs/$p.dump-answer.today
  EIN_STDLIB=$EA $A solve $D/programs/$p.ein --dump-answer $D/runs/$p.dump-answer.before-ein-a8d595eb
  $H solve $D/programs/$p.ein --trace $D/runs/$p.trace.today.md
  for f in $D/runs/$p.dump-answer.today/models/*.ein; do $H saturate --dump $f > $D/runs/$p.models-$(basename $f .ein).saturate-dump.today.txt; done
  python3 -I $D/tools/mock_census.py $D/programs/$p.ein $D/runs $p > $D/runs/$p.census.MOCK.txt
done
```

## 7. Found while building

- **The census fails silently**: on today's binary it reports each entry's states and 0 facts, `—` for every rate,
  and exits 0 with nothing on stderr; nothing says the marker it looks for was absent. A run of it now reads as *no
  fact to count*, not *the format moved*.

## 8. Your call

- **(1)** the census reads `ein saturate --dump` of each saved file (the program's `relation` facts left out, which
  gives the counts it gave before) and `--trace` for the English the binary writes.
- **(2)** the answer half is retired with Q-M1f.414's (a); the flag half and `--check` stay; M1g's T1g.6.3.3 names it.
- No default of its own; the script reports zeros until one is taken.
