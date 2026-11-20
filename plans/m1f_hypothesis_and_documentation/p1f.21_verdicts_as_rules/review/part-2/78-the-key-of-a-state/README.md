# 78 · The key of a state: built at its own size, or left

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-86--a-states-key-sorts-every-fact-the-state-holds-roots-among-them) ·
[the cost page](../../../costs/README.md#the-final-table-before-after-improved-after)

**No build waits on this**: the phase's cost work is done but for it, if you want it.

## What it is

At every state of a search the engine takes the state's canonical key, which tells two commitments that reach one
state apart from two that do not. To make it, the node function collects **every fact the state holds**, root's
whole fixpoint among them, sorts them, and then filters root's facts back out to keep the state's own. Root's
facts are the same at every state.

| program | facts at root | states built | the key | the filter |
|---|---:|---:|---:|---:|
| `send_more_money.ein` | 1 867 | 18 129 | 37.7 % of the instructions | 9.3 % |
| `equation_two_unknowns.ein` | 2 306 | 1 191 | 10.9 % | 2.1 % |

The figures are S1f.21.23's, from a profile with call stacks of the engine as it stands; taken again on the final
binary they are the same within a point ([§ On the built engine](#on-the-built-engine)). The engine did this
before the phase: it is no cost the phase added, and the final table already has the engine back where it was.

## What building it would be

The same move as S1f.21.23's, which you ruled built (*"of course build"*, Q-M1f.674): a state's own facts are in
its layers above root's mark, which the engine reads since that stage, so the part of the key above root can be
read and sorted at its own size. **What a stage would settle first**: whether anything reads the key whole, with
root's facts in it. **What it may not move**: any key, so any class of states, any answer, any firing; held as
S1f.21.21 and S1f.21.23 held theirs, by `ein solve -p` and the digest of the events on every corpus file.

## The question

- **(a)** Not built: the engine is back where it was, and this is a gain beyond it. **Default.**
- **(b)** Built, a stage of the phase like S1f.21.23.
- **(c)** Not in this phase: kept as a finding for the work after it.

## On the built engine

The profile the table above is from was taken by S1f.21.23 on its own branch. Here it is taken again on the final
binary, the release build of `7b72bd84` (`runs/version.txt`), with the command the finding names, three times a
program: `perf record --call-graph lbr -e instructions:u` of `taskset -c 0-15 ein solve FILE --jobs 1`, each
reported with `perf report --stdio --children --sort symbol -F overhead_children,overhead,symbol -g none`
(`runs/<program>.<run>.perf-report.txt`, the recordings beside them as `.perf.data`). `tools/run.sh` re-takes
it all and reads the figures off each report into `runs/figures.tsv`. The machine was on AC (`runs/power-online.txt`).

**What is measured.** *The key* is `ein_infer::canon::state_key`, with what it calls (the report's *children*
column). *The filter* is the line after it in the node function, the vector `node_in_region` builds of the
key's facts that root does not hold (`!axes.root_facts.contains(f)`, its closure `{closure#4}`), which perf
reports as a symbol of its own (its *self* column). The figures are shares of the instructions the P-cores ran;
the E-core counter counts next to nothing under the pin.

| program | facts at root | saturations | the key: S1f.21.23's | the key: today, three runs | the filter: S1f.21.23's | the filter: today, three runs |
|---|---:|---:|---:|---:|---:|---:|
| `send_more_money.ein` | 1 867 | 18 129 | 37.7 % | 37.9 %, 39.2 %, 38.5 % | 9.3 % | 8.9 %, 9.1 %, 9.8 % |
| `equation_two_unknowns.ein` | 2 306 | 1 191 | 10.9 % | 10.2 %, 10.7 %, 11.0 % | 2.1 % | 2.9 %, 2.2 %, 2.7 % |

The facts at root and the saturations are the run's own (`runs/counts.tsv`, from `runs/<program>.summary.json`),
the same as the table above. **The figures are what S1f.21.23 gave**, within what a sampled profile can tell: a run
holds about 2 000 to 4 000 samples of `send_more_money.ein` and about 1 000 of `equation_two_unknowns.ein`
(`runs/figures.tsv`), so a share moves by about a point from run to run. The instructions counted agree to within
0.1 % across the three runs of each program. On `send_more_money.ein` the key and the filter together are 46.9 %
to 48.4 % of the instructions, the *about 47 %* of the bundle's index; the node function as a whole is about 90 %.
