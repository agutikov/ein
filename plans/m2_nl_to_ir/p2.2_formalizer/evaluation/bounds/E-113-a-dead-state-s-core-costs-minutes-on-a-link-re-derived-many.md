# E-113 — A dead state's core costs minutes on a link re-derived many ways, unless the program sets a lever

**Kind:** weakness
**Status:** open
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none; [F22](../../../../followups/f22_exact_core_past_the_cap.md) (the core made canonical) is where its
cost would be weighed

## What

**A dead state's core costs minutes on a link re-derived many ways, unless the program sets a lever.** Where a
surface prints the core of every dead state a run visits (`--events`, `--dump-lattice`, `render lattice`), the
default search for the smallest core reads every derivation of each fact, and one generic link re-derived by
several rules makes that search explode: `zebra3.1.ein` without `(config :record-alternative-justifications false)`
ran `ein solve --single --events` for 4 min 55 s of CPU and was stopped, its event file unchanged for the last three
minutes; with the line, 74 ms, as the run itself (`--single` alone, about 40 ms). `zebra.ein` carries the line for the
same reason (its comment: about 22.7 s a dead branch), and `zebra2.ein` and `zebra3.4.ein`, five projections, need
none (`--events` in 144 and 162 ms). The lever changes which core is named and no state
([Q-M1e2.30](../../../../../docs/decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)).

## Evidence

`ein solve examples/zebra3.1.ein --single --events E.jsonl` against the same file with its `(config …)` line taken
out (`timeout 300` stops it); `examples/zebra3.2.ein` and `zebra3.3.ein` alike (94 and 91 ms with the line).

## What it costs the application

A formalizer that writes the generic link, as the target shape does, writes a program the kernel answers in
milliseconds and explains, on the surfaces that explain every dead state, in minutes, and nothing in the program's
shape says which lever to set; the one-shot and the loop read a run's feedback, whose bound has to cover it.
