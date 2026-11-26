# S2.2.5's tables

Written by `analyse.py` from the runs replayed on the final code (`replay.sh`, `replayed/`) and the committed records (`runs/`). *Word problems, one shot*: prompts were developed on these problems, and this is no result. The answer is read by the stand-in reader of the answer's comparison (`einaf::answer::score`), against the gold short answer of `plan.tsv`.

**Replay**: 29 of 29 replayed programs are the committed ones byte for byte.

**Against S2.2.4** (its `steps` condition, the same requests): 37 calls of the library condition wrote S2.2.4's output byte for byte; read otherwise by the cut or after it: unused 2 (cut), unused 3, boxes12 3 (cut), boxes13 2b (cut), boxes13 3 (cut), pencils 3 (cut); otherwise different: none.

## The Zebra puzzle's three texts (condition library)

| text | step 1 | step 2 | step 3 | `--single` | the check's findings | the answer printed | gold | read |
|---|---|---|---|---|---|---|---|---|
| zebra | 53 forms, end-of-turn; root not-decided, H `blind` | 69 forms, end-of-turn; root inconsistent, H `stuck` | 15 forms, end-of-turn; root inconsistent, H `no-search` | inconsistent | `mechanism-not-the-one-stated` 1, `selection-inactive` 5 | The statements contradict each other. | Norwegian water\|Japanese zebra | wrong |
| zebra-minus-15 | 53 forms, end-of-turn; root not-decided, H `blind` | 74 forms, end-of-turn; root inconsistent, H `stuck` | 17 forms, end-of-turn; root inconsistent, H `stuck` | inconsistent | `selection-inactive` 8 | The statements contradict each other. | @undetermined | wrong |
| zebra-contradiction | 56 forms, end-of-turn; root not-decided, H `blind` | 78 forms, end-of-turn; root inconsistent, H `obligations` | 9 forms, end-of-turn; root inconsistent, H `no-search` | inconsistent | `mechanism-not-the-one-stated` 1, `selection-inactive` 1 | The statements contradict each other. | @contradiction | right |

## The word problems, one shot: the library alone, the main arms' condition

| text | its problems | parsed (steps) | loads | `--single` | the check's findings | the answer query's classes | the answer printed | the kernel's line | the README's answer | read |
|---|---|---|---|---|---|---|---|---|---|---|
| bob1 | bob1 | 1: 5 forms, 2: 5 forms, 3: 3 forms | yes | one | none | 1 (3 rows) | Bob has 1 apples. / Bob has 3 apples. / Bob has 5 apples. | Bob has (n z 1) apples. / Bob has (n z 3) apples. / Bob has (n z 5) apples. | 3 | wrong |
| gap | gap, gap_assume | 1: 6 forms, 2: 6 forms, 3: 3 forms | yes | one | `name-not-listed` 1 | 1 (0 rows) | The statements do not determine the answer. | — | not stated: Alice's apples | right |
| gap_fact | gap_fact | 1: 7 forms, 2: 6 forms, 3: 3 forms | yes | one | `name-not-listed` 1 | 1 (0 rows) | The statements do not determine the answer. | — | Bob 3, Alice 6, together 9 | wrong |
| contra | contra | 1: 5 forms, 2: 5 forms, 3: 3 forms | yes | one | none | 1 (1 rows) | Bob has 5 apples. | Bob has (n z 5) apples. | a contradiction | wrong |
| unused | unused | 1: 9 forms, 2: 21 forms (cut), 3: 3 forms | no | invalid | none | — | No answer: the program was refused. | — | 3 | no-answer |
| bystander | bystander | 1: 6 forms, 2: 5 forms, 3: 3 forms | yes | one | none | 1 (5 rows) | Carol has 1 apples. / Carol has 3 apples. / Carol has 5 apples. / Carol has 7 apples. / Carol has 9 apples. | Carol has (n z 1) apples. / Carol has (n z 3) apples. / Carol has (n z 5) apples. / Carol has (n z 7) apples. / Carol has (n z 9) apples. | 9 | wrong |
| before | before | 1: 5 forms, 2: 5 forms, 3: 3 forms | yes | one | none | 1 (3 rows) | Bob had 1 apples before. / Bob had 3 apples before. / Bob had 5 apples before. | Bob had (n z 1) apples before. / Bob had (n z 3) apples before. / Bob had (n z 5) apples before. | 7 (the theory answers no answer) | wrong |
| kinds | kinds, kinds_assume, kinds_listed | 1: 5 forms, 2: 5 forms, 3: 2 forms | yes | one | `name-not-listed` 1 | 1 (0 rows) | The statements do not determine the answer. | — | not stated: Alice's apples | right |
| kinds_full | kinds_full | 1: 7 forms, 2: 5 forms, 3: 2 forms | yes | one | `name-not-listed` 1 | 1 (0 rows) | The statements do not determine the answer. | — | 8 | wrong |
| more | more | 1: 4 forms, 2: 4 forms, 3: 2 forms | no | invalid | none | — | No answer: the program was refused. | — | Alice, by 7 | no-answer |
| boxes12 | boxes12 | 1: 7 forms, 2: 6 forms, 3: 1 forms (cut) | yes | several | `mechanism-not-stated` 1, `wrong-count` 1 | 1 (0 rows, a lower bound) | The statements do not determine the answer. | — | 4 | wrong |
| boxes13 | boxes13 | 1: 6 forms, 2: 13 forms, 2b: 0 forms (cut), 3: 0 forms (cut) | yes | several | `mechanism-not-stated` 1, `undeclared` 7, `wrong-count` 1 | 1 (0 rows, a lower bound) | The statements do not determine the answer. | — | 4 or 5, by the reading of need | right |
| pencils | pencils | 1: 10 forms, 2: 19 forms, 3: 0 forms (cut) | yes | several | `mechanism-not-stated` 1, `number-as-name` 2, `undeclared` 3 | 1 (0 rows, a lower bound) | The statements do not determine the answer. | — | 47 each, 21 left over | wrong |

Of 13 texts: 11 load; read **right** 3, wrong 8, no answer 2.

## The word problems, one shot: `--module-dir examples/word_problems/`, the widest condition of the library ablation, labelled so

| text | its problems | parsed (steps) | loads | `--single` | the check's findings | the answer query's classes | the answer printed | the kernel's line | the README's answer | read |
|---|---|---|---|---|---|---|---|---|---|---|
| bob1 | bob1 | 1: 5 forms, 2: 5 forms, 3: 3 forms | yes | one | none | 1 (3 rows) | Bob has 1 apples. / Bob has 3 apples. / Bob has 5 apples. | Bob has (n z 1) apples. / Bob has (n z 3) apples. / Bob has (n z 5) apples. | 3 | wrong |
| gap | gap, gap_assume | 1: 6 forms, 2: 6 forms, 3: 3 forms | yes | one | `name-not-listed` 1 | 1 (0 rows) | The statements do not determine the answer. | — | not stated: Alice's apples | right |
| gap_fact | gap_fact | 1: 7 forms, 2: 16 forms, 3: 5 forms | no | invalid | none | — | No answer: the program was refused. | — | Bob 3, Alice 6, together 9 | no-answer |
| contra | contra | 1: 5 forms, 2: 5 forms, 3: 3 forms | yes | one | none | 1 (1 rows) | Bob has 5 apples. | Bob has (n z 5) apples. | a contradiction | wrong |
| unused | unused | 1: 9 forms, 2: 19 forms, 3: 3 forms | no | invalid | none | — | No answer: the program was refused. | — | 3 | no-answer |
| bystander | bystander | 1: 6 forms, 2: 15 forms (cut), 3: 5 forms | yes | one | `undeclared` 7 | 1 (1 rows) | Carol has 9 apples. | Carol has (n z 9) apples. | 9 | right |
| before | before | 1: 5 forms, 2: 5 forms, 3: 3 forms | yes | one | none | 1 (3 rows) | Bob had 1 apples before. / Bob had 3 apples before. / Bob had 5 apples before. | Bob had (n z 1) apples before. / Bob had (n z 3) apples before. / Bob had (n z 5) apples before. | 7 (the theory answers no answer) | wrong |
| kinds | kinds, kinds_assume, kinds_listed | 1: 5 forms, 2: 5 forms, 3: 3 forms | no | invalid | none | — | No answer: the program was refused. | — | not stated: Alice's apples | no-answer |
| kinds_full | kinds_full | 1: 7 forms, 2: 5 forms, 3: 3 forms | yes | one | `name-not-listed` 1 | 1 (0 rows) | The statements do not determine the answer. | — | 8 | wrong |
| more | more | 1: 4 forms, 2: 4 forms, 3: 3 forms | no | invalid | none | — | No answer: the program was refused. | — | Alice, by 7 | no-answer |
| boxes12 | boxes12 | 1: 7 forms, 2: 18 forms (cut), 3: 3 forms | no | invalid | none | — | No answer: the program was refused. | — | 4 | no-answer |
| boxes13 | boxes13 | 1: 6 forms, 2: 9 forms, 3: 0 forms (cut) | yes | several | `mechanism-not-stated` 1, `undeclared` 2, `wrong-count` 1 | 1 (0 rows, a lower bound) | The statements do not determine the answer. | — | 4 or 5, by the reading of need | right |
| pencils | pencils | 1: 10 forms, 2: 20 forms, 3: 3 forms | yes | one | `number-as-name` 1, `undeclared` 10 | 1 (0 rows) | The statements do not determine the answer. | — | 47 each, 21 left over | wrong |

Of 13 texts: 8 load; read **right** 3, wrong 5, no answer 5.

## Each call

| condition | text | step | prompt tokens | generated | stop | s | parsed | cut | at root | H | findings |
|---|---|---|---:|---:|---|---:|---|---|---|---|---|
| library | zebra | 1 | 4787 | 1032 | end-of-turn | 16.3 | 53 forms |  | not-decided | blind | none |
| library | zebra | 2 | 12627 | 1160 | end-of-turn | 19.9 | 69 forms |  | inconsistent | stuck | none |
| library | zebra | 3 | 13869 | 308 | end-of-turn | 5.3 | 15 forms |  | inconsistent | no-search | `mechanism-not-the-one-stated` 1, `selection-inactive` 5 |
| library | zebra-minus-15 | 1 | 4777 | 1041 | end-of-turn | 16.4 | 53 forms |  | not-decided | blind | none |
| library | zebra-minus-15 | 2 | 12626 | 1168 | end-of-turn | 19.9 | 74 forms |  | inconsistent | stuck | `selection-inactive` 6 |
| library | zebra-minus-15 | 3 | 13848 | 336 | end-of-turn | 5.8 | 17 forms |  | inconsistent | stuck | `selection-inactive` 8 |
| library | zebra-contradiction | 1 | 4793 | 1065 | end-of-turn | 16.8 | 56 forms |  | not-decided | blind | none |
| library | zebra-contradiction | 2 | 12666 | 1218 | end-of-turn | 20.8 | 78 forms |  | inconsistent | obligations | `selection-inactive` 1 |
| library | zebra-contradiction | 3 | 13934 | 164 | end-of-turn | 2.9 | 9 forms |  | inconsistent | no-search | `mechanism-not-the-one-stated` 1, `selection-inactive` 1 |
| library | bob1 | 1 | 4636 | 142 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | none |
| library | bob1 | 2 | 11589 | 122 | end-of-turn | 2.2 | 5 forms |  | not-decided | blind | none |
| library | bob1 | 3 | 12351 | 230 | end-of-turn | 3.9 | 3 forms |  | not-decided | no-search | none |
| library | gap | 1 | 4637 | 180 | end-of-turn | 2.8 | 6 forms |  | not-decided | blind | `undeclared` 1 |
| library | gap | 2 | 11628 | 148 | end-of-turn | 2.7 | 6 forms |  | not-decided | blind | `name-not-listed` 1 |
| library | gap | 3 | 12391 | 213 | end-of-turn | 3.8 | 3 forms |  | not-decided | no-search | `name-not-listed` 1 |
| library | gap_fact | 1 | 4656 | 241 | end-of-turn | 3.9 | 7 forms |  | not-decided | blind | none |
| library | gap_fact | 2 | 11708 | 148 | end-of-turn | 2.7 | 6 forms |  | not-decided | blind | `name-not-listed` 1 |
| library | gap_fact | 3 | 12471 | 220 | end-of-turn | 3.8 | 3 forms |  | not-decided | no-search | `name-not-listed` 1 |
| library | contra | 1 | 4636 | 142 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | none |
| library | contra | 2 | 11589 | 122 | end-of-turn | 2.2 | 5 forms |  | not-decided | blind | none |
| library | contra | 3 | 12351 | 218 | end-of-turn | 3.7 | 3 forms |  | not-decided | no-search | none |
| library | unused | 1 | 4656 | 244 | end-of-turn | 3.9 | 9 forms |  | not-decided | blind | none |
| library | unused | 2 | 11711 | 12288 | max-tokens | 216.7 | 21 forms | kept 21, repeats 0 | invalid | — | none |
| library | unused | 3 | 12790 | 234 | end-of-turn | 4.2 | 3 forms |  | invalid | — | none |
| library | bystander | 1 | 4643 | 168 | end-of-turn | 2.7 | 6 forms |  | not-decided | blind | none |
| library | bystander | 2 | 11622 | 127 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | none |
| library | bystander | 3 | 12384 | 233 | end-of-turn | 4.0 | 3 forms |  | not-decided | no-search | none |
| library | before | 1 | 4637 | 150 | end-of-turn | 2.4 | 5 forms |  | not-decided | blind | none |
| library | before | 2 | 11598 | 127 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | none |
| library | before | 3 | 12360 | 230 | end-of-turn | 3.9 | 3 forms |  | not-decided | no-search | none |
| library | kinds | 1 | 4637 | 168 | end-of-turn | 2.7 | 5 forms |  | not-decided | blind | `undeclared` 1 |
| library | kinds | 2 | 11616 | 123 | end-of-turn | 2.2 | 5 forms |  | not-decided | blind | `name-not-listed` 1 |
| library | kinds | 3 | 12378 | 154 | end-of-turn | 2.7 | 2 forms |  | not-decided | no-search | `name-not-listed` 1 |
| library | kinds_full | 1 | 4646 | 214 | end-of-turn | 3.4 | 7 forms |  | not-decided | blind | `undeclared` 1 |
| library | kinds_full | 2 | 11671 | 123 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | `name-not-listed` 1 |
| library | kinds_full | 3 | 12433 | 121 | end-of-turn | 2.1 | 2 forms |  | not-decided | no-search | `name-not-listed` 1 |
| library | more | 1 | 4639 | 157 | end-of-turn | 2.5 | 4 forms |  | invalid | — | none |
| library | more | 2 | 11607 | 98 | end-of-turn | 1.8 | 4 forms |  | invalid | — | none |
| library | more | 3 | 12369 | 148 | end-of-turn | 2.7 | 2 forms |  | invalid | — | none |
| library | boxes12 | 1 | 4640 | 192 | end-of-turn | 3.1 | 7 forms |  | not-decided | blind | `wrong-count` 1 |
| library | boxes12 | 2 | 11643 | 229 | end-of-turn | 4.1 | 6 forms |  | not-decided | blind | `wrong-count` 1 |
| library | boxes12 | 3 | 12490 | 12288 | max-tokens | 218.0 | 1 forms | kept 1, repeats 0 | not-decided | blind | `mechanism-not-stated` 1, `wrong-count` 1 |
| library | boxes13 | 1 | 4640 | 164 | end-of-turn | 2.6 | 6 forms |  | invalid | — | none |
| library | boxes13 | 2 | 11615 | 736 | end-of-turn | 12.5 | 13 forms |  | not-decided | blind | `undeclared` 7, `wrong-count` 1 |
| library | boxes13 | 2b | 12840 | 12288 | max-tokens | 218.0 | 0 forms | kept 0, repeats 0 | — | — | none |
| library | boxes13 | 3 | 12997 | 12288 | max-tokens | 218.9 | 0 forms | kept 0, repeats 0 | — | — | none |
| library | pencils | 1 | 4670 | 304 | end-of-turn | 4.8 | 10 forms |  | not-decided | blind | `number-as-name` 1 |
| library | pencils | 2 | 11785 | 406 | end-of-turn | 6.8 | 19 forms |  | not-decided | blind | `number-as-name` 2, `undeclared` 3 |
| library | pencils | 3 | 12795 | 12288 | max-tokens | 218.8 | 0 forms | kept 0, repeats 0 | — | — | none |
| module-dir | bob1 | 1 | 4636 | 142 | end-of-turn | 2.2 | 5 forms |  | not-decided | blind | none |
| module-dir | bob1 | 2 | 14542 | 122 | end-of-turn | 2.2 | 5 forms |  | not-decided | blind | none |
| module-dir | bob1 | 3 | 15304 | 221 | end-of-turn | 4.1 | 3 forms |  | not-decided | no-search | none |
| module-dir | gap | 1 | 4637 | 180 | end-of-turn | 2.9 | 6 forms |  | not-decided | blind | `undeclared` 1 |
| module-dir | gap | 2 | 14581 | 148 | end-of-turn | 2.7 | 6 forms |  | not-decided | blind | `name-not-listed` 1 |
| module-dir | gap | 3 | 15344 | 219 | end-of-turn | 4.0 | 3 forms |  | not-decided | no-search | `name-not-listed` 1 |
| module-dir | gap_fact | 1 | 4656 | 241 | end-of-turn | 3.9 | 7 forms |  | not-decided | blind | none |
| module-dir | gap_fact | 2 | 14661 | 314 | end-of-turn | 5.6 | 16 forms |  | not-decided | blind | `name-not-listed` 1 |
| module-dir | gap_fact | 3 | 15590 | 486 | end-of-turn | 8.7 | 5 forms |  | invalid | — | none |
| module-dir | contra | 1 | 4636 | 142 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | none |
| module-dir | contra | 2 | 14542 | 122 | end-of-turn | 2.2 | 5 forms |  | not-decided | blind | none |
| module-dir | contra | 3 | 15304 | 221 | end-of-turn | 4.1 | 3 forms |  | not-decided | no-search | none |
| module-dir | unused | 1 | 4656 | 244 | end-of-turn | 3.9 | 9 forms |  | not-decided | blind | none |
| module-dir | unused | 2 | 14664 | 392 | end-of-turn | 7.0 | 19 forms |  | invalid | — | none |
| module-dir | unused | 3 | 15667 | 222 | end-of-turn | 4.1 | 3 forms |  | invalid | — | none |
| module-dir | bystander | 1 | 4643 | 168 | end-of-turn | 2.7 | 6 forms |  | not-decided | blind | none |
| module-dir | bystander | 2 | 14575 | 12288 | max-tokens | 222.0 | 15 forms | kept 15, repeats 0 | not-decided | blind | `undeclared` 7 |
| module-dir | bystander | 3 | 15520 | 446 | end-of-turn | 7.9 | 5 forms |  | not-decided | no-search | `undeclared` 7 |
| module-dir | before | 1 | 4637 | 150 | end-of-turn | 2.4 | 5 forms |  | not-decided | blind | none |
| module-dir | before | 2 | 14551 | 127 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | none |
| module-dir | before | 3 | 15313 | 227 | end-of-turn | 4.1 | 3 forms |  | not-decided | no-search | none |
| module-dir | kinds | 1 | 4637 | 168 | end-of-turn | 2.7 | 5 forms |  | not-decided | blind | `undeclared` 1 |
| module-dir | kinds | 2 | 14569 | 123 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | `name-not-listed` 1 |
| module-dir | kinds | 3 | 15331 | 249 | end-of-turn | 4.6 | 3 forms |  | invalid | — | none |
| module-dir | kinds_full | 1 | 4646 | 214 | end-of-turn | 3.4 | 7 forms |  | not-decided | blind | `undeclared` 1 |
| module-dir | kinds_full | 2 | 14624 | 123 | end-of-turn | 2.3 | 5 forms |  | not-decided | blind | `name-not-listed` 1 |
| module-dir | kinds_full | 3 | 15386 | 218 | end-of-turn | 3.8 | 3 forms |  | not-decided | no-search | `name-not-listed` 1 |
| module-dir | more | 1 | 4639 | 157 | end-of-turn | 2.5 | 4 forms |  | invalid | — | none |
| module-dir | more | 2 | 14560 | 98 | end-of-turn | 1.8 | 4 forms |  | invalid | — | none |
| module-dir | more | 3 | 15322 | 233 | end-of-turn | 4.3 | 3 forms |  | invalid | — | none |
| module-dir | boxes12 | 1 | 4640 | 192 | end-of-turn | 3.1 | 7 forms |  | not-decided | blind | `wrong-count` 1 |
| module-dir | boxes12 | 2 | 14596 | 12288 | max-tokens | 221.9 | 18 forms | kept 18, repeats 0 | invalid | — | none |
| module-dir | boxes12 | 3 | 15601 | 289 | end-of-turn | 5.1 | 3 forms |  | invalid | — | none |
| module-dir | boxes13 | 1 | 4640 | 164 | end-of-turn | 2.6 | 6 forms |  | invalid | — | none |
| module-dir | boxes13 | 2 | 14568 | 256 | end-of-turn | 4.6 | 9 forms |  | not-decided | blind | `undeclared` 2, `wrong-count` 1 |
| module-dir | boxes13 | 3 | 15470 | 12288 | max-tokens | 223.2 | 0 forms | kept 0, repeats 0 | — | — | none |
| module-dir | pencils | 1 | 4670 | 304 | end-of-turn | 4.8 | 10 forms |  | not-decided | blind | `number-as-name` 1 |
| module-dir | pencils | 2 | 14738 | 386 | end-of-turn | 6.8 | 20 forms |  | not-decided | blind | `number-as-name` 1, `undeclared` 10 |
| module-dir | pencils | 3 | 15728 | 219 | end-of-turn | 4.0 | 3 forms |  | not-decided | no-search | `number-as-name` 1, `undeclared` 10 |

## The check's findings, by condition, step and kind

| condition | step | kind | count |
|---|---|---|---:|
| library | 1 | `number-as-name` | 1 |
| library | 1 | `undeclared` | 3 |
| library | 1 | `wrong-count` | 1 |
| library | 2 | `name-not-listed` | 4 |
| library | 2 | `number-as-name` | 2 |
| library | 2 | `selection-inactive` | 7 |
| library | 2 | `undeclared` | 10 |
| library | 2 | `wrong-count` | 2 |
| library | 3 | `mechanism-not-stated` | 1 |
| library | 3 | `mechanism-not-the-one-stated` | 2 |
| library | 3 | `name-not-listed` | 4 |
| library | 3 | `selection-inactive` | 14 |
| library | 3 | `wrong-count` | 1 |
| module-dir | 1 | `number-as-name` | 1 |
| module-dir | 1 | `undeclared` | 3 |
| module-dir | 1 | `wrong-count` | 1 |
| module-dir | 2 | `name-not-listed` | 4 |
| module-dir | 2 | `number-as-name` | 1 |
| module-dir | 2 | `undeclared` | 19 |
| module-dir | 2 | `wrong-count` | 1 |
| module-dir | 3 | `name-not-listed` | 2 |
| module-dir | 3 | `number-as-name` | 1 |
| module-dir | 3 | `undeclared` | 17 |

## The Zebra puzzle, step by step against the steps by hand

Forms compared with their `:source` and their layout taken out; a step's part is what it wrote (the model's output; the hand's step less the forms of the step before).

| step | the model's forms | of them the hand's | the hand's forms | facts by sentence: the same / the model's alone / the hand's alone | declared relations: both / the model's alone / the hand's alone | selections, the model's |
|---|---:|---:|---:|---|---|---|
| 1 | 53 | 4 | 55 | 2 / 43 / 42 | 6 / has / are-smoked-in, are-smoked-next-to, is-drunk-in, lives-next-to | — |
| 2 | 69 | 14 | 39 | 0 / 31 / 0 | 2 / color-and-house, drinker-and-house, drinks, has, house-of, instance, is-immediately-to-the-right-of, lives-in, owner-and-house, owns, smoker-and-house, smokes, thing-and-house / are-smoked-by, cigarette-and-house, drink-and-house, is-drunk-by, is-kept-in, is-lived-in-by, is-owned-by, man-and-house, pet-and-house, same-house | includes, slot-partition, slot-spatial, symmetric |
| 3 | 15 | 0 | 18 | 0 / 0 / 0 | 0 / — / — | converse, meet, top |

## The library's rules root fired, the model's Zebra program against the reference

Read off `ein saturate PROGRAM --events F --events-level verbose`, the `fire` lines by rule, each program with `(config :record-alternative-justifications false)` appended (no firing changes; E-126); the reference is the steps by hand's step 3, `einaf/contract/zebra/step3.ein`.

| rule | the model's | the reference's |
|---|---:|---:|
| `compose` | 0 | 4 |
| `converse` | 676 | 68 |
| `includes` | 45 | 22 |
| `meet` | 191 | 5 |
| `std.algebra.converse-pair-symmetric` | 10 | 10 |
| `std.algebra.self-converse-is-symmetric` | 2 | 2 |
| `std.algebra.symmetric-is-self-converse` | 2 | 2 |
| `std.algebra.symmetric-negative` | 941 | 396 |
| `std.algebra.symmetric-negative-setup` | 2 | 2 |
| `std.contradiction.clash-of-0` | 15 | 22 |
| `std.contradiction.clash-of-2` | 15 | 22 |
| `std.core.inconsistent` | 1 | 0 |
| `std.slots.slot-adjacent-bwd` | 20 | 2 |
| `std.slots.slot-adjacent-bwd-neg` | 25 | 35 |
| `std.slots.slot-adjacent-fwd` | 20 | 2 |
| `std.slots.slot-adjacent-fwd-neg` | 25 | 35 |
| `std.slots.slot-elimination` | 220 | 7 |
| `std.slots.slot-endpoint-bwd` | 30 | 5 |
| `std.slots.slot-endpoint-fwd` | 30 | 5 |
| `std.slots.slot-exclusive` | 340 | 120 |
| `std.slots.slot-fill` | 220 | 7 |
| `std.slots.slot-locate` | 5742 | 6 |
| `std.slots.slot-negative` | 6424 | 108 |
| `std.slots.slot-no-fill` | 66 | 0 |
| `std.slots.slot-no-room` | 20 | 0 |
| `std.slots.slot-occupied` | 6064 | 136 |
| `std.slots.slot-partition-setup` | 1 | 1 |
| `std.slots.slot-prune-bwd` | 480 | 10 |
| `std.slots.slot-prune-fwd` | 480 | 10 |
| `std.slots.slot-spatial-setup` | 1 | 2 |
| `symmetric` | 596 | 50 |
| `top` | 165 | 100 |

