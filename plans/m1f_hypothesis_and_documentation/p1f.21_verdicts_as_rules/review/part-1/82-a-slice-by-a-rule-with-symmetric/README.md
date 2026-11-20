# 82 · A slice by a rule, activated with `symmetric`, loads now

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-34--the-milestones-l-109-program-loads)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it.

## What it is

The milestone's finding L-109, left as your call: your slice by a rule, `about-1`, whose parameter names the
relation of the facts it reads, appended to `examples/zebra2.ein` with `(about-1 symmetric)`, was refused at load,
because a rule of `std.algebra` *could* derive an activator binding a relation's parameter to a fact. That
refusal is the warning now (S1f.21.17): the program loads under one warning that names `std.algebra.imply1`, and
root derives `(in-slice-1 symmetric next-to)`. Whether a slice is had by such a rule was the question, and still
is.

## The question

- **(a)** It is: the program loads and answers under its warning, and L-109 is closed by that. **Default.**
- **(b)** It is not what a slice should be: say what you want of it.

## On the built engine

**How it was taken.** The final binary, `build/review-bin/ein-final-7b72bd84`, the release build of `7b72bd84`, with
the checkout's `stdlib/`, on the corpus at `7b72bd84`; its `--version` is `runs/version.txt`. `tools/run.sh`
re-takes every run into `runs/` (both ignored by git): for the program, the read-out (`about-1.solve.txt`), `-p`,
`--monotone`, `ein show hyps`, `ein saturate --dump`, the events and the summary (`about-1.events.jsonl`, 30 MB, and
`about-1.summary.json`), the answer and the visited lattice as files, and the knowledge base, the lattice and the
second query's slice drawn (`about-1.kb.svg`, `about-1.lattice.svg`, `about-1.query-2.svg`, each beside its `.dot`);
for [`examples/zebra2.ein`](../../../../../../examples/zebra2.ein) as it is, the read-out, `ein show hyps` and
`ein saturate --dump`, to set beside them.

**No oracle and no region.** H is 56 candidates, so the cube is 2^56 saturations: `ein oracle` refuses it, *56 root
candidates, over --region-limit 16: the region is 2^56 saturations and was not computed*, exit 2
(`runs/about-1.oracle.txt`), and with no cube there is no region to draw. The lattice is the one a describing run
visits.

**The programs.**

- [`zebra2-about-1.ein`](programs/zebra2-about-1.ein): `examples/zebra2.ein` whole, a header before it, and the three
  forms of [the milestone's L-109](../../../../findings.md) after it, as the row writes them:
  `(rule about-1 (?F) :match (and (?F ?r) (relation ?r)) :assert (in-slice-1 ?F ?r) :why "{?F} says something of {?r}")`,
  `(about-1 symmetric)` and `(query :goal (in-slice-1 ?f ?r))`. `runs/copy-diff.txt` diffs its body against
  zebra2, exit 0, no line.
- [`zebra2-about-1-bijective.ein`](programs/zebra2-about-1-bijective.ein): the same with `(about-1 bijective)`, the
  run L-109 set beside it, which loaded then (`runs/twin-diff.txt`: that line and the header differ). Its read-out,
  `ein saturate --dump`, `ein show hyps` and the second query's slice (`runs/bijective.*`).

### The finding's figures, and today's

| | L-109's figure (as it was), then V-34's | **the built engine** | the run |
|---|---|---|---|
| loads? | L-109: refused, `ein solve` exit 1, *kb load error: rule 'std.algebra.imply1': its :assert writes (?R2 ?a) …*; V-34: `ein saturate` exits 0 | **loads**: every command run exits 0 but the oracle (above) | `runs/about-1.solve.txt`, `.saturate-dump.txt`, `.show-hyps.txt` |
| the warning | V-34: one, naming `std.algebra.imply1` | **one line**, on every command: *rule 'std.algebra.imply1': its :assert writes (?R2 ?a), whose ?a is rule 'symmetric''s parameter ?rel, which names a relation, and (?R1 ?a) can bind ?a to a fact, … so such an activator activates nothing* | `runs/about-1.solve.txt` (its last line before the exit), `runs/about-1.kb.stderr.txt` |
| what root derives of the slice | V-34: `(in-slice-1 symmetric next-to)` | **`(in-slice-1 symmetric next-to :rule about-1)`**, the one fact of `in-slice-1` (0 before saturation, 1 after) | `runs/about-1.saturate-dump.txt`, `runs/about-1.in-slice-1.txt` |
| flagged? | — | **no**: `lattice monotone (the static check) — R is convex: no hole`, no `WARNING` row; `--monotone`: `yes — no site` | `runs/about-1.solve.txt`, `runs/about-1.monotone.txt` |
| the second query, `(in-slice-1 ?f ?r)` | — | **one row**, `?f = symmetric · ?r = next-to`; `as :expect (model (in-slice-1 symmetric next-to))` | `runs/about-1.solve.txt` |
| the puzzle's query | — | as zebra2's: *The Norwegian drinks water in House-1; the Japanese owns zebra in House-5* | `runs/about-1.solve.txt` |
| the bijective twin | L-109: exit 0, query 2 a row for each of zebra2's bijections, `?f = bijective · ?r = color-loc` the first | **exit 0, no warning**; 5 rows, `color-loc`, `drink-loc`, `nation-loc`, `pet-loc`, `smoke-loc`, each `?f = bijective` | `runs/bijective.solve.txt`, `runs/bijective.in-slice-1.txt` |

### Beside zebra2's own

| | `examples/zebra2.ein` | **with L-109's forms** | the runs |
|---|---|---|---|
| H | 56 candidates, ``note: …: H, from `union`: 56 candidates`` | **the same 56**, the same note; sorted, the two lists are the same (`runs/hyps-diff.txt`, exit 0, no line) | `runs/zebra2.show-hyps.txt`, `runs/about-1.show-hyps.txt` |
| R | `states (k_s) 1`, `commitments (k_c) 261 982`, `∅ not in R — root owes 36 instances`, `universe 56 candidates, 21 dead alone` | **the same rows**, from `asked` to `work`, line for line | `runs/zebra2.solve.txt`, `runs/about-1.solve.txt` |
| the queries | one | two: the puzzle's and the slice's | |

So the three forms add one fact at root, `(in-slice-1 symmetric next-to)`, and one warning line, and change neither
H nor R. The slice drawn, `runs/about-1.query-2.svg` (with `--fold`, `runs/about-1.query-2-fold.svg`), is that one
fact, by `about-1`, with its arrows to `in-slice-1`, `symmetric` and `next-to`; the whole knowledge base is
`runs/about-1.kb.svg`.
