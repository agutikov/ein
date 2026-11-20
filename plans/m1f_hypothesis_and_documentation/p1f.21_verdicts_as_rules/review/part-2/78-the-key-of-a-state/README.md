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

The figures are S1f.21.23's, from a profile with call stacks of the engine as it stands. The engine did this
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
