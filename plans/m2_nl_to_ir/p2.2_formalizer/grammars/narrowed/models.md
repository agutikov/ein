# The programs the model wrote that loaded, read by the root (before: `98e6e0a3`'s file)

60 programs; the narrowed root accepts 57, the root before it 59.

| record | programs | accepted by both | refused by both | accepted before, refused now |
|---|---:|---:|---:|---:|
| S2.2.2, the probe | 27 | 26 | 1 | 0 |
| S2.2.3, the first plan | 16 | 15 | 0 | 1 |
| S2.2.3, the second plan | 15 | 14 | 0 | 1 |
| S2.2.3, the third plan | 2 | 2 | 0 | 0 |

| record | run | reading | the root before | the narrowed root | what it holds that the root leaves out |
|---|---|---|---|---|---|
| S2.2.2, the probe | `first-experiment.off.zebra` | fenced | refused at 158:8, ' ' | refused at 5:42, '`' | '`' in a comment; a `config` form |
| S2.2.3, the first plan | `whole-grammar.off.first-experiment` | whole | accepted | refused at 5:40, '`' | '`' in a comment |
| S2.2.3, the second plan | `whole-fenced.off.first-experiment` | fenced | accepted | refused at 5:42, '`' | '`' in a comment |
