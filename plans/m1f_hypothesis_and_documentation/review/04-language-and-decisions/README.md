# The language, and three decisions' sentences

[← the bundle](../README.md)

| # | item | the question | default |
|---|---|---|---|
| 08 | [V-7](08-V-7/README.md) | a relation only an activator names is not counted | as is (leaning: the registry takes it) |
| 09 | [F21 N-15](09-F21-N-15/README.md) | *everybody's thoughts are false* with `absent` is dead: twin of `W8/fifo-gap-root`? | none; nothing filed |
| 16 | [Q-M1f.412](16-Q-M1f.412/README.md) | the `is-a` row dropped: does a schema id move? | (a) no id moves |
| 22 | [L-92](22-L-92/README.md) | Q-M1f.362's sentence on the index | kept |
| 23 | [L-93](23-L-93/README.md) | two stale sentences of the pages (a third found) | left |
| 24 | [L-94](24-L-94/README.md) | Q-M1f.119 misses a fact's argument | kept |

## Found while building

- **08**: **the options move answers.** The relation registry decides what the blind enumerator treats as an object:
  in `v7_blind.ein`, `happy` is guessed about (8 states); with it counted, 1 state. Declaring `=` in the witness
  takes it from {∅} to 12 candidates and 4 096 states. A rule that concludes a constant relation behaves the same
  today; 0 corpus programs have V-7's case, 29 the constant one (a grep).
- **09**: the same mechanism as `fifo-gap-root`, one admission per NAF round in agenda order. `x2b` does not move with
  written order (the agenda admits the `(false)` rule first) but does with a conclusion nothing reads
  (`x2b-negation` is alive, R={∅}); `x2b-fact-a-first` is exactly `fifo-gap-root`'s shape. Also: `ein render kb` of
  its root leaves out the held `(confused Ann)`, since the rule is named `confused` like its relation and the view
  takes the fact for an activator.
- **16**: a reader written against the old keys fails on all three schemas, whose ids did not move. `region.json` is
  written by `ein oracle --dump`, not `--dump-answer`. The KB view draws a derived `is-a` like a written one; the
  `--models grid` rows differ for `is-a` and a renamed twin.
- **22**: measured: a join on a whole fact is quadratic (25.9, 98.7, 389 ms at 2k, 4k, 8k facts); one level in, it is
  linear. A re-wording is written. `kb.rs:1158`'s comment is stale too.
- **23**: a third stale sentence in the same paragraph: a `--dump-answer` state *is refused when it is loaded again*;
  today the saved file loads. `text.rs`' `NESTED_HEAD` comment repeats the stale examples.
- **24**: the refusal's line misses three ways (a fact's argument, an activator's name, `is-a`/`eq`/`neq`), and
  `12-query.md` and `02-syntax.md` leave out the activator's way, which Q-M1f.119 names.
