# The language, and three decisions' sentences

[← the bundle](../README.md)

Re-taken with `build/review-bin/ein-head-17085b27`, every P1f.23 stage merged; each item has a `rerun.sh`. Since the first
look (`ein-head-b5fb8d70`) one run moved, item 24's goal over `is-a` in a program that names none, now refused
(Q-M1f.436), and the drawings, now canonical; the rest is said under each item's *Found while building*, and below.

| # | item | the question | default |
|---|---|---|---|
| 08 | [V-7](08-V-7/README.md) | a relation only an activator names is not counted | as is (leaning: the registry takes it) |
| 09 | [F21 N-15](09-F21-N-15/README.md) | *everybody's thoughts are false* with `absent` is dead: twin of `W8/fifo-gap-root`? | none; nothing filed |
| 16 | [Q-M1f.412](16-Q-M1f.412/README.md) | the `is-a` row dropped: does a schema id move? | (a) no id moves for it (the report is `/6` for the diff) |
| 22 | [L-92](22-L-92/README.md) | Q-M1f.362's sentence on the index | kept |
| 23 | [L-93](23-L-93/README.md) | two stale sentences of the pages (a third found) | left |
| 24 | [L-94](24-L-94/README.md) | Q-M1f.119 misses a fact's argument | kept |

## Found while building

- **08**: **the options move answers.** The relation registry decides what the blind enumerator treats as an object:
  in `v7_blind.ein`, `happy` is guessed about (8 states); with it counted, 1 state. Declaring `=` in the witness
  takes it from {∅} to 12 candidates and 4 096 states. A rule that concludes a constant relation behaves the same
  today; 0 corpus programs have V-7's case, 29 the constant one (the census, now over 888 programs). The KB view
  draws `=` as one box under the activator's arrow `2` and the derived fact's arrow `1`. `program_relations` no
  longer adds `is-a` (Q-M1f.436), which V-7 does not turn on.
- **09**: the same mechanism as `fifo-gap-root`, one admission per NAF round in agenda order. `x2b` does not move with
  written order (the agenda admits the `(false)` rule first) but does with a conclusion nothing reads
  (`x2b-negation` is alive, R={∅}); `x2b-fact-a-first` is exactly `fifo-gap-root`'s shape. The first look's
  drawing gap is gone: `ein render kb` now draws the held `(confused Ann)` whatever the rule is named (Q-M1f.432).
- **16**: the question stands; the report is `ein-test-report/6` today for `ein test --diff`'s key, so option (b)
  now reads `/7`, and a reader keyed on the old ids refuses the report by its id. **The page's second half is
  answered**: nothing in the engine reads `is-a` by its name (Q-M1f.436, Q-M1f.421; `--models` removed), and a
  derived `is-a` is drawn as any derived fact. The two `--models grid` runs are dropped, `--models` being refused
  (exit 2), and their programs `grid-rows.ein` and `grid-rows-is-b.ein` with them.
- **22**: measured again: a join on a whole fact is quadratic, one level in it is linear, the ratios as at the
  first look (the times higher under another agent's load). A re-wording is written. `kb.rs:1158`'s comment is stale
  too.
- **23**: a third stale sentence in the same paragraph: a `--dump-answer` state *is refused when it is loaded again*;
  today the saved file loads. `text.rs`' `NESTED_HEAD` comment repeats the stale examples. Nothing moved.
- **24**: the refusal's line misses the fact's argument and the activator's name; `12-query.md` and `02-syntax.md`
  leave out the activator's way, which Q-M1f.119 names. **New since the first look**: a goal over `is-a` in a
  program that writes none is refused (Q-M1f.436), and **Q-M1f.119 and `02-syntax.md` § Query still say `is-a` is
  every program's**, where `12-query.md`, Q-M1f.114 and `07-diagnostics.md` were brought to it.
