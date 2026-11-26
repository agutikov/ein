# S2.2.4's runs, tabled

Written by `analyse.py` from `runs/*/*/run.json`; nothing here is typed by hand.

## steps

| runs | stopped | a complete program | it loads | --single | the answer (a rough reading) | tokens generated, a run |
|---:|---|---:|---:|---|---|---|
| 16 | step 3's output does not parse, or is empty 1 | 16 | 14 | inconsistent 2, invalid 2, one 9, several 3 | a line, where the text gives no answer 1, no line 12, not the gold 1, the gold, and other values 2 | 403 to 25476, 96412 in all |

| item | step | prompt tok | gen tok | stop | parses | at root | H from (size) | findings at the step |
|---|---|---:|---:|---|---|---|---|---|
| zebra | 1 | 4787 | 1032 | end-of-turn | yes | not decided | blind (7840) | none |
| zebra | 2 | 12627 | 1160 | end-of-turn | yes | inconsistent | stuck (0) | none |
| zebra | 3 | 13869 | 308 | end-of-turn | yes | inconsistent | no-search (0) | mechanism-not-the-one-stated 1, selection-inactive 5 |
| youngest | 1 | 4628 | 127 | end-of-turn | yes | not decided | blind (7) | none |
| youngest | 2 | 11571 | 12288 | max-tokens | yes | not decided | blind (1103) | undeclared 4 |
| youngest | 3 | 24508 | 254 | end-of-turn | yes | not decided | no-search (0) | undeclared 4 |
| seating | 1 | 4655 | 243 | end-of-turn | yes | not decided | blind (27) | undeclared 6 |
| seating | 2 | 11714 | 454 | end-of-turn | yes | inconsistent | obligations (0) | rewritten 3 |
| seating | 3 | 12837 | 76 | end-of-turn | yes | inconsistent | obligations (0) | rewritten 3 |
| bob1 | 1 | 4636 | 142 | end-of-turn | yes | not decided | blind (0) | none |
| bob1 | 2 | 11589 | 122 | end-of-turn | yes | not decided | blind (0) | none |
| bob1 | 3 | 12351 | 230 | end-of-turn | yes | not decided | no-search (0) | none |
| gap | 1 | 4637 | 180 | end-of-turn | yes | not decided | blind (0) | undeclared 1 |
| gap | 2 | 11628 | 148 | end-of-turn | yes | not decided | blind (0) | name-not-listed 1 |
| gap | 3 | 12391 | 213 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1 |
| gap_fact | 1 | 4656 | 241 | end-of-turn | yes | not decided | blind (0) | none |
| gap_fact | 2 | 11708 | 148 | end-of-turn | yes | not decided | blind (0) | name-not-listed 1 |
| gap_fact | 3 | 12471 | 220 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1 |
| contra | 1 | 4636 | 142 | end-of-turn | yes | not decided | blind (0) | none |
| contra | 2 | 11589 | 122 | end-of-turn | yes | not decided | blind (0) | none |
| contra | 3 | 12351 | 218 | end-of-turn | yes | not decided | no-search (0) | none |
| unused | 1 | 4656 | 244 | end-of-turn | yes | not decided | blind (0) | none |
| unused | 2 | 11711 | 12288 | max-tokens | yes | invalid | — (0) | none |
| unused | 3 | 24612 | 12288 | max-tokens | yes | invalid | — (0) | none |
| bystander | 1 | 4643 | 168 | end-of-turn | yes | not decided | blind (0) | none |
| bystander | 2 | 11622 | 127 | end-of-turn | yes | not decided | blind (0) | none |
| bystander | 3 | 12384 | 233 | end-of-turn | yes | not decided | no-search (0) | none |
| before | 1 | 4637 | 150 | end-of-turn | yes | not decided | blind (0) | none |
| before | 2 | 11598 | 127 | end-of-turn | yes | not decided | blind (0) | none |
| before | 3 | 12360 | 230 | end-of-turn | yes | not decided | no-search (0) | none |
| kinds | 1 | 4637 | 168 | end-of-turn | yes | not decided | blind (0) | undeclared 1 |
| kinds | 2 | 11616 | 123 | end-of-turn | yes | not decided | blind (0) | name-not-listed 1 |
| kinds | 3 | 12378 | 154 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1 |
| kinds_full | 1 | 4646 | 214 | end-of-turn | yes | not decided | blind (0) | undeclared 1 |
| kinds_full | 2 | 11671 | 123 | end-of-turn | yes | not decided | blind (0) | name-not-listed 1 |
| kinds_full | 3 | 12433 | 121 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1 |
| more | 1 | 4639 | 157 | end-of-turn | yes | invalid | — (0) | none |
| more | 2 | 11607 | 98 | end-of-turn | yes | invalid | — (0) | none |
| more | 3 | 12369 | 148 | end-of-turn | yes | invalid | — (0) | none |
| boxes12 | 1 | 4640 | 192 | end-of-turn | yes | not decided | blind (56) | wrong-count 1 |
| boxes12 | 2 | 11643 | 229 | end-of-turn | yes | not decided | blind (272) | wrong-count 1 |
| boxes12 | 3 | 12490 | 12288 | max-tokens | no: step 3:10:1: unexpected end of input | — | — (0) | none |
| boxes12 | | | | | stopped: step 3's output does not parse, or is empty | | | |
| boxes13 | 1 | 4640 | 164 | end-of-turn | yes | invalid | — (0) | none |
| boxes13 | 2 | 11615 | 736 | end-of-turn | yes | not decided | blind (306) | undeclared 7, wrong-count 1 |
| boxes13 | 2b | 12840 | 12288 | max-tokens | yes | not decided | blind (306) | undeclared 7, wrong-count 1 |
| boxes13 | 3 | 25300 | 12288 | max-tokens | yes | not decided | blind (306) | mechanism-not-stated 1, undeclared 7, wrong-count 1 |
| pencils | 1 | 4670 | 304 | end-of-turn | yes | not decided | blind (215) | number-as-name 1 |
| pencils | 2 | 11785 | 406 | end-of-turn | yes | not decided | blind (3232) | number-as-name 2, undeclared 3 |
| pencils | 3 | 12795 | 12288 | max-tokens | yes | not decided | blind (3232) | mechanism-not-stated 1, number-as-name 2, undeclared 3 |

| item | --no-search | --single | the answer | gold | findings | categories (src/gen/rel/rule/assum/frame) |
|---|---|---|---|---|---|---|
| zebra | inconsistent | inconsistent | no line | Norwegian, Japanese | mechanism-not-the-one-stated 1, selection-inactive 5 | 50/25/9/0/12/8 |
| youngest | not decided | one | no line | Cy | undeclared 4 | 5/12/3/0/1/4 |
| seating | inconsistent | inconsistent | no line | Bob | rewritten 3 | 15/6/4/0/5/3 |
| bob1 | not decided | one | Bob has 1 apples. / Bob has 3 apples. / Bob has 5 apples. (the gold, and other values) | 3 | none | 2/3/0/2/0/4 |
| gap | not decided | one | no line | none | name-not-listed 1 | 2/3/0/2/0/5 |
| gap_fact | not decided | one | no line | 3, 6, 9 | name-not-listed 1 | 3/3/0/2/0/5 |
| contra | not decided | one | Bob has 5 apples. (a line, where the text gives no answer) | none | none | 2/3/0/2/0/4 |
| unused | invalid | invalid | no line | 3 | none | 0/0/0/0/0/0 |
| bystander | not decided | one | Carol has 1 apples. / Carol has 3 apples. / Carol has 5 apples. / Carol has 7 apples. / Carol has 9 apples. (the gold, and other values) | 9 | none | 3/3/0/2/0/4 |
| before | not decided | one | Bob had 1 apples before. / Bob had 3 apples before. / Bob had 5 apples before. (not the gold) | 7 | none | 2/3/0/2/0/4 |
| kinds | not decided | one | no line | none | name-not-listed 1 | 2/3/0/1/0/4 |
| kinds_full | not decided | one | no line | 8 | name-not-listed 1 | 4/3/0/1/0/4 |
| more | invalid | invalid | no line | Alice, 7 | none | 0/0/0/0/0/0 |
| boxes12 | not decided | several | no line | 4 | mechanism-not-stated 1, wrong-count 1 | 3/3/0/0/0/4 |
| boxes13 | not decided | several | no line | 5 | mechanism-not-stated 1, undeclared 7, wrong-count 1 | 7/3/0/0/3/4 |
| pencils | not decided | several | no line | 48, 10 | mechanism-not-stated 1, number-as-name 2, undeclared 3 | 5/3/4/0/8/5 |

Step 1, what the acceptance asks:

| item | forms | statements | answer queries | other forms | declarations (no :why) | memberships | sentences with no fact | sources no sentence | goal's relations (in statements) | as the hand's: statements, memberships |
|---|---:|---:|---:|---:|---|---:|---|---:|---|---|
| zebra | 53 | 19 | 1 | 0 | 7 (0) | 26 | 0 of 17 | 0 | drinks, owns (2) | 2 of 19, 0 of 25 |
| youngest | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | is-youngest (0) | 0 of 2, 0 of 3 |
| seating | 11 | 9 | 1 | 0 | 1 (0) | 0 | 0 of 5 | 0 | sits (1) | 3 of 9, 0 of 0 |
| bob1 | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | has (1) | 0 of 2, 0 of 0 |
| gap | 6 | 2 | 1 | 0 | 3 (0) | 0 | 0 of 3 | 0 | sum-of (0) | 0 of 2, 0 of 0 |
| gap_fact | 7 | 3 | 1 | 0 | 3 (0) | 0 | 0 of 6 | 0 | has, sum-of (1) | — |
| contra | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | has (1) | — |
| unused | 9 | 5 | 1 | 0 | 3 (0) | 0 | 0 of 5 | 0 | has (1) | — |
| bystander | 6 | 3 | 1 | 0 | 2 (0) | 0 | 0 of 4 | 0 | has (1) | — |
| before | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | has (1) | — |
| kinds | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 2 | 0 | sum-of (0) | — |
| kinds_full | 7 | 4 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | sum-of (0) | — |
| more | 4 | 2 | 1 | 0 | 1 (0) | 0 | 0 of 3 | 0 | gt, has (1) | 0 of 2, 0 of 0 |
| boxes12 | 7 | 3 | 1 | 0 | 3 (0) | 0 | 0 of 3 | 0 | need-boxes (0) | — |
| boxes13 | 6 | 3 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | need-boxes (0) | — |
| pencils | 10 | 5 | 1 | 0 | 4 (0) | 0 | 0 of 4 | 0 | count (1) | — |

Steps 2 and 3, what the acceptance asks:

| item | step 2: imports | selecting facts | not in catalogue / not listed / inactive | markers | ontology kept / dropped / added / redeclared | rewritten | step 3: rules (markers) | mechanism stated | writer |
|---|---|---:|---|---:|---|---:|---|---|---|
| zebra | std.algebra std.slots | 9 | 0 / 0 / 0 | 0 | 33 / 0 / 25 / 0 | 0 | 0 (0) | no-search and obligations | no-search |
| youngest | std.algebra std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 7 / 0 | 0 | 0 (0) | no-search | no-search |
| seating | std.algebra std.slots | 2 | 0 / 0 / 0 | 0 | 1 / 0 / 16 / 0 | 3 | 0 (0) | obligations | obligations |
| bob1 | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| gap | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| gap_fact | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| contra | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| unused | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 15 / 0 | 0 | 0 (0) | none | — |
| bystander | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| before | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| kinds | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 1 (1) | no-search | no-search |
| kinds_full | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 1 (1) | no-search | no-search |
| more | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 1 / 0 / 0 / 0 | 0 | 1 (1) | none | — |
| boxes12 | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 0 / 0 | 0 | 1 (2) | none | blind |
| boxes13 | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 1 | 2 / 0 / 8 / 0 | 0 | 0 (0) | none | blind |
| pencils | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 4 / 0 / 12 / 0 | 0 | 0 (0) | none | blind |

## whole

| runs | stopped | a complete program | it loads | --single | the answer (a rough reading) | tokens generated, a run |
|---:|---|---:|---:|---|---|---|
| 16 | the output does not parse, or is empty 1 | 15 | 14 | inconsistent 3, invalid 1, one 11 | a line, where the text gives no answer 1, no line 10, not the gold 2, the gold 1, the gold, and other values 1 | 113 to 12288, 21770 in all |

| item | step | prompt tok | gen tok | stop | parses | at root | H from (size) | findings at the step |
|---|---|---:|---:|---|---|---|---|---|
| zebra | whole | 11852 | 12288 | max-tokens | no: step whole:1088:1: unexpected end of input | — | — (0) | none |
| zebra | | | | | stopped: the output does not parse, or is empty | | | |
| youngest | whole | 11693 | 557 | end-of-turn | yes | not decided | no-search (0) | wrong-count 2 |
| seating | whole | 11720 | 623 | end-of-turn | yes | inconsistent | stuck (0) | selection-inactive 2, undeclared 3 |
| bob1 | whole | 11701 | 459 | end-of-turn | yes | not decided | no-search (0) | none |
| gap | whole | 11702 | 683 | end-of-turn | yes | not decided | no-search (0) | wrong-count 4 |
| gap_fact | whole | 11721 | 940 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1, undeclared 11, wrong-count 3 |
| contra | whole | 11701 | 647 | end-of-turn | yes | not decided | no-search (0) | undeclared 6 |
| unused | whole | 11721 | 694 | end-of-turn | yes | not decided | no-search (0) | source-not-a-sentence 4 |
| bystander | whole | 11708 | 558 | end-of-turn | yes | inconsistent | no-search (0) | number-as-name 1, undeclared 7 |
| before | whole | 11702 | 497 | end-of-turn | yes | not decided | no-search (0) | none |
| kinds | whole | 11702 | 483 | end-of-turn | yes | not decided | no-search (0) | answer-in-template 1, source-not-a-sentence 2 |
| kinds_full | whole | 11711 | 556 | end-of-turn | yes | not decided | no-search (0) | answer-in-template 1, source-not-a-sentence 4, undeclared 6 |
| more | whole | 11704 | 113 | end-of-turn | yes | invalid | — (0) | none |
| boxes12 | whole | 11705 | 735 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1, source-not-a-sentence 2, undeclared 7, wrong-count 4 |
| boxes13 | whole | 11705 | 644 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1, source-not-a-sentence 4, undeclared 2, wrong-count 3 |
| pencils | whole | 11735 | 1293 | end-of-turn | yes | inconsistent | no-search (0) | name-not-listed 2, number-as-name 6, undeclared 21 |

| item | --no-search | --single | the answer | gold | findings | categories (src/gen/rel/rule/assum/frame) |
|---|---|---|---|---|---|---|
| zebra | no program | | | | | |
| youngest | not decided | one | no line | Cy | wrong-count 2 | 5/3/1/2/0/4 |
| seating | inconsistent | inconsistent | no line | Bob | selection-inactive 2, undeclared 3 | 3/10/4/0/14/2 |
| bob1 | not decided | one | Bob has 5 apples. (not the gold) | 3 | none | 2/3/0/2/0/4 |
| gap | not decided | one | no line | none | wrong-count 4 | 5/3/1/2/3/6 |
| gap_fact | not decided | one | no line | 3, 6, 9 | name-not-listed 1, undeclared 11, wrong-count 3 | 6/3/1/6/1/4 |
| contra | not decided | one | Bob has 5 apples. (a line, where the text gives no answer) | none | undeclared 6 | 5/3/0/2/3/4 |
| unused | not decided | one | Bob has 1 apples. / Bob has 3 apples. / Bob has 5 apples. (the gold, and other values) | 3 | source-not-a-sentence 4 | 10/3/1/2/4/5 |
| bystander | inconsistent | inconsistent | no line | 9 | number-as-name 1, undeclared 7 | 3/3/0/2/7/4 |
| before | not decided | one | Bob had 1 apples before. / Bob had 3 apples before. / Bob had 5 apples before. (not the gold) | 7 | none | 2/3/0/2/0/4 |
| kinds | not decided | one | no line | none | answer-in-template 1, source-not-a-sentence 2 | 2/3/0/2/0/4 |
| kinds_full | not decided | one | They have 8 apples together. (the gold) | 8 | answer-in-template 1, source-not-a-sentence 4, undeclared 6 | 4/3/0/2/6/4 |
| more | invalid | invalid | no line | Alice, 7 | none | 0/0/0/0/0/0 |
| boxes12 | not decided | one | no line | 4 | name-not-listed 1, source-not-a-sentence 2, undeclared 7, wrong-count 4 | 3/3/0/4/5/5 |
| boxes13 | not decided | one | no line | 5 | name-not-listed 1, source-not-a-sentence 4, undeclared 2, wrong-count 3 | 6/3/1/2/4/6 |
| pencils | inconsistent | inconsistent | no line | 48, 10 | name-not-listed 2, number-as-name 6, undeclared 21 | 12/3/0/6/6/8 |

## steps-free

| runs | stopped | a complete program | it loads | --single | the answer (a rough reading) | tokens generated, a run |
|---:|---|---:|---:|---|---|---|
| 16 | none | 16 | 11 | inconsistent 2, invalid 5, one 8, several 1 | a line, where the text gives no answer 1, no line 12, not the gold 1, the gold, and other values 2 | 384 to 2500, 11312 in all |

| item | step | prompt tok | gen tok | stop | parses | at root | H from (size) | findings at the step |
|---|---|---:|---:|---|---|---|---|---|
| zebra | 1 | 4787 | 1032 | end-of-turn | yes | not decided | blind (7840) | none |
| zebra | 2 | 12627 | 1160 | end-of-turn | yes | inconsistent | stuck (0) | none |
| zebra | 3 | 13869 | 308 | end-of-turn | yes | inconsistent | no-search (0) | mechanism-not-the-one-stated 1, selection-inactive 5 |
| youngest | 1 | 4628 | 127 | end-of-turn | yes | not decided | blind (7) | none |
| youngest | 2 | 11571 | 607 | end-of-turn | yes | not decided | no-search (0) | selection-not-listed 3, several-answer-queries 1, undeclared 7 |
| youngest | 3 | 12825 | 103 | end-of-turn | yes | not decided | no-search (0) | selection-not-listed 3, several-answer-queries 1, undeclared 8 |
| seating | 1 | 4655 | 243 | end-of-turn | yes | not decided | blind (27) | undeclared 6 |
| seating | 2 | 11714 | 454 | end-of-turn | yes | inconsistent | obligations (0) | rewritten 3 |
| seating | 3 | 12837 | 76 | end-of-turn | yes | inconsistent | obligations (0) | rewritten 3 |
| bob1 | 1 | 4636 | 142 | end-of-turn | yes | not decided | blind (0) | none |
| bob1 | 2 | 11589 | 122 | end-of-turn | yes | not decided | blind (0) | none |
| bob1 | 3 | 12351 | 215 | end-of-turn | yes | not decided | no-search (0) | none |
| gap | 1 | 4637 | 180 | end-of-turn | yes | not decided | blind (0) | undeclared 1 |
| gap | 2 | 11628 | 148 | end-of-turn | yes | invalid | — (0) | none |
| gap | 3 | 12391 | 96 | end-of-turn | yes | invalid | — (0) | none |
| gap_fact | 1 | 4656 | 241 | end-of-turn | yes | not decided | blind (0) | none |
| gap_fact | 2 | 11708 | 148 | end-of-turn | yes | not decided | blind (0) | name-not-listed 1 |
| gap_fact | 3 | 12471 | 220 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1 |
| contra | 1 | 4636 | 142 | end-of-turn | yes | not decided | blind (0) | none |
| contra | 2 | 11589 | 122 | end-of-turn | yes | not decided | blind (0) | none |
| contra | 3 | 12351 | 218 | end-of-turn | yes | not decided | no-search (0) | none |
| unused | 1 | 4656 | 238 | end-of-turn | yes | not decided | blind (0) | source-not-a-sentence 1 |
| unused | 2 | 11705 | 401 | end-of-turn | yes | invalid | — (0) | none |
| unused | 3 | 12716 | 224 | end-of-turn | yes | invalid | — (0) | none |
| bystander | 1 | 4643 | 168 | end-of-turn | yes | not decided | blind (0) | none |
| bystander | 2 | 11622 | 127 | end-of-turn | yes | not decided | blind (0) | none |
| bystander | 3 | 12384 | 233 | end-of-turn | yes | not decided | no-search (0) | none |
| before | 1 | 4637 | 150 | end-of-turn | yes | not decided | blind (0) | none |
| before | 2 | 11598 | 127 | end-of-turn | yes | not decided | blind (0) | none |
| before | 3 | 12360 | 230 | end-of-turn | yes | not decided | no-search (0) | none |
| kinds | 1 | 4637 | 155 | end-of-turn | yes | not decided | blind (0) | source-not-a-sentence 2, undeclared 1 |
| kinds | 2 | 11603 | 123 | end-of-turn | yes | invalid | — (0) | none |
| kinds | 3 | 12365 | 106 | end-of-turn | yes | invalid | — (0) | none |
| kinds_full | 1 | 4646 | 157 | end-of-turn | yes | not decided | blind (0) | none |
| kinds_full | 2 | 11614 | 128 | end-of-turn | yes | not decided | blind (0) | none |
| kinds_full | 3 | 12376 | 236 | end-of-turn | yes | not decided | no-search (0) | none |
| more | 1 | 4639 | 157 | end-of-turn | yes | invalid | — (0) | none |
| more | 2 | 11607 | 98 | end-of-turn | yes | invalid | — (0) | none |
| more | 3 | 12369 | 134 | end-of-turn | yes | not decided | no-search (0) | undeclared 2, wrong-count 1 |
| boxes12 | 1 | 4640 | 181 | end-of-turn | yes | not decided | blind (56) | source-not-a-sentence 2, wrong-count 1 |
| boxes12 | 2 | 11632 | 346 | end-of-turn | yes | not decided | no-search (0) | several-answer-queries 1, source-not-a-sentence 2, wrong-count 1 |
| boxes12 | 3 | 12596 | 27 | end-of-turn | yes | invalid | — (0) | none |
| boxes13 | 1 | 4640 | 181 | end-of-turn | yes | not decided | blind (56) | source-not-a-sentence 2, wrong-count 1 |
| boxes13 | 2 | 11632 | 346 | end-of-turn | yes | not decided | no-search (0) | several-answer-queries 1, source-not-a-sentence 2, wrong-count 1 |
| boxes13 | 3 | 12596 | 27 | end-of-turn | yes | invalid | — (0) | none |
| pencils | 1 | 4670 | 304 | end-of-turn | yes | not decided | blind (215) | number-as-name 1 |
| pencils | 2 | 11785 | 406 | end-of-turn | yes | not decided | blind (3232) | number-as-name 2, undeclared 3 |
| pencils | 3 | 12795 | 198 | end-of-turn | yes | not decided | blind (3540) | mechanism-not-stated 1, number-as-name 2, selection-not-listed 8, undeclared 11 |

| item | --no-search | --single | the answer | gold | findings | categories (src/gen/rel/rule/assum/frame) |
|---|---|---|---|---|---|---|
| zebra | inconsistent | inconsistent | no line | Norwegian, Japanese | mechanism-not-the-one-stated 1, selection-inactive 5 | 50/25/9/0/12/8 |
| youngest | not decided | one | no line | Cy | selection-not-listed 3, several-answer-queries 1, undeclared 8 | 5/7/3/2/6/5 |
| seating | inconsistent | inconsistent | no line | Bob | rewritten 3 | 15/6/4/0/5/3 |
| bob1 | not decided | one | Bob has 1 apples. / Bob has 3 apples. / Bob has 5 apples. (the gold, and other values) | 3 | none | 2/3/0/2/0/4 |
| gap | invalid | invalid | no line | none | none | 0/0/0/0/0/0 |
| gap_fact | not decided | one | no line | 3, 6, 9 | name-not-listed 1 | 3/3/0/2/0/5 |
| contra | not decided | one | Bob has 5 apples. (a line, where the text gives no answer) | none | none | 2/3/0/2/0/4 |
| unused | invalid | invalid | no line | 3 | none | 0/0/0/0/0/0 |
| bystander | not decided | one | Carol has 1 apples. / Carol has 3 apples. / Carol has 5 apples. / Carol has 7 apples. / Carol has 9 apples. (the gold, and other values) | 9 | none | 3/3/0/2/0/4 |
| before | not decided | one | Bob had 1 apples before. / Bob had 3 apples before. / Bob had 5 apples before. (not the gold) | 7 | none | 2/3/0/2/0/4 |
| kinds | invalid | invalid | no line | none | none | 0/0/0/0/0/0 |
| kinds_full | not decided | one | no line | 8 | none | 0/3/1/2/4/3 |
| more | not decided | one | no line | Alice, 7 | undeclared 2, wrong-count 1 | 2/3/0/1/0/3 |
| boxes12 | invalid | invalid | no line | 4 | none | 0/0/0/0/0/0 |
| boxes13 | invalid | invalid | no line | 5 | none | 0/0/0/0/0/0 |
| pencils | not decided | several | no line | 48, 10 | mechanism-not-stated 1, number-as-name 2, selection-not-listed 8, undeclared 11 | 5/3/4/0/16/5 |

Step 1, what the acceptance asks:

| item | forms | statements | answer queries | other forms | declarations (no :why) | memberships | sentences with no fact | sources no sentence | goal's relations (in statements) | as the hand's: statements, memberships |
|---|---:|---:|---:|---:|---|---:|---|---:|---|---|
| zebra | 53 | 19 | 1 | 0 | 7 (0) | 26 | 0 of 17 | 0 | drinks, owns (2) | 2 of 19, 0 of 25 |
| youngest | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | is-youngest (0) | 0 of 2, 0 of 3 |
| seating | 11 | 9 | 1 | 0 | 1 (0) | 0 | 0 of 5 | 0 | sits (1) | 3 of 9, 0 of 0 |
| bob1 | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | has (1) | 0 of 2, 0 of 0 |
| gap | 6 | 2 | 1 | 0 | 3 (0) | 0 | 0 of 3 | 0 | sum-of (0) | 0 of 2, 0 of 0 |
| gap_fact | 7 | 3 | 1 | 0 | 3 (0) | 0 | 0 of 6 | 0 | has, sum-of (1) | — |
| contra | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | has (1) | — |
| unused | 9 | 5 | 1 | 0 | 3 (0) | 0 | 0 of 5 | 1 | has (1) | — |
| bystander | 6 | 3 | 1 | 0 | 2 (0) | 0 | 0 of 4 | 0 | has (1) | — |
| before | 5 | 2 | 1 | 0 | 2 (0) | 0 | 0 of 3 | 0 | has (1) | — |
| kinds | 5 | 2 | 1 | 0 | 2 (0) | 0 | 1 of 2 | 2 | sum-of (0) | — |
| kinds_full | 7 | 4 | 1 | 0 | 2 (0) | 0 | 2 of 3 | 0 | sum-of-apples (0) | — |
| more | 4 | 2 | 1 | 0 | 1 (0) | 0 | 0 of 3 | 0 | gt, has (1) | 0 of 2, 0 of 0 |
| boxes12 | 7 | 3 | 1 | 0 | 3 (0) | 0 | 1 of 3 | 2 | need-boxes (0) | — |
| boxes13 | 7 | 3 | 1 | 0 | 3 (0) | 0 | 1 of 3 | 2 | need-boxes (0) | — |
| pencils | 10 | 5 | 1 | 0 | 4 (0) | 0 | 0 of 4 | 0 | count (1) | — |

Steps 2 and 3, what the acceptance asks:

| item | step 2: imports | selecting facts | not in catalogue / not listed / inactive | markers | ontology kept / dropped / added / redeclared | rewritten | step 3: rules (markers) | mechanism stated | writer |
|---|---|---:|---|---:|---|---:|---|---|---|
| zebra | std.algebra std.slots | 9 | 0 / 0 / 0 | 0 | 33 / 0 / 25 / 0 | 0 | 0 (0) | no-search and obligations | no-search |
| youngest | std.algebra std.math.decimal-arith std.math.decimal std.math.arith | 6 | 0 / 3 / 0 | 0 | 2 / 0 / 7 / 0 | 0 | 1 (0) | no-search | no-search |
| seating | std.algebra std.slots | 2 | 0 / 0 / 0 | 0 | 1 / 0 / 16 / 0 | 3 | 0 (0) | obligations | obligations |
| bob1 | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (1) | no-search | no-search |
| gap | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 0 / 0 | 0 | 1 (1) | none | — |
| gap_fact | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| contra | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| unused | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 16 / 0 | 0 | 2 (2) | none | — |
| bystander | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| before | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| kinds | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 1 (1) | none | — |
| kinds_full | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 2 / 0 / 0 / 0 | 0 | 2 (2) | no-search | no-search |
| more | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 1 / 0 / 0 / 0 | 0 | 1 (1) | no-search | no-search |
| boxes12 | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 0 / 0 | 0 | 0 (0) | none | — |
| boxes13 | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 3 / 0 / 0 / 0 | 0 | 0 (0) | none | — |
| pencils | std.math.decimal-arith std.math.decimal std.math.arith | 0 | 0 / 0 / 0 | 0 | 4 / 0 / 12 / 0 | 0 | 0 (0) | none | blind |

## whole-free

| runs | stopped | a complete program | it loads | --single | the answer (a rough reading) | tokens generated, a run |
|---:|---|---:|---:|---|---|---|
| 16 | the output does not parse, or is empty 1 | 15 | 14 | inconsistent 3, invalid 1, one 11 | a line, where the text gives no answer 1, no line 10, not the gold 2, the gold 1, the gold, and other values 1 | 459 to 12288, 22210 in all |

| item | step | prompt tok | gen tok | stop | parses | at root | H from (size) | findings at the step |
|---|---|---:|---:|---|---|---|---|---|
| zebra | whole | 11852 | 12288 | max-tokens | no: step whole:9:9: unexpected `"There are five houses."`: a str | — | — (0) | none |
| zebra | | | | | stopped: the output does not parse, or is empty | | | |
| youngest | whole | 11693 | 557 | end-of-turn | yes | not decided | no-search (0) | wrong-count 2 |
| seating | whole | 11720 | 623 | end-of-turn | yes | inconsistent | stuck (0) | selection-inactive 2, undeclared 3 |
| bob1 | whole | 11701 | 459 | end-of-turn | yes | not decided | no-search (0) | none |
| gap | whole | 11702 | 683 | end-of-turn | yes | not decided | no-search (0) | wrong-count 4 |
| gap_fact | whole | 11721 | 940 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1, undeclared 11, wrong-count 3 |
| contra | whole | 11701 | 647 | end-of-turn | yes | not decided | no-search (0) | undeclared 6 |
| unused | whole | 11721 | 694 | end-of-turn | yes | not decided | no-search (0) | source-not-a-sentence 4 |
| bystander | whole | 11708 | 558 | end-of-turn | yes | inconsistent | no-search (0) | number-as-name 1, undeclared 7 |
| before | whole | 11702 | 497 | end-of-turn | yes | not decided | no-search (0) | none |
| kinds | whole | 11702 | 483 | end-of-turn | yes | not decided | no-search (0) | answer-in-template 1, source-not-a-sentence 2 |
| kinds_full | whole | 11711 | 556 | end-of-turn | yes | not decided | no-search (0) | answer-in-template 1, source-not-a-sentence 4, undeclared 6 |
| more | whole | 11704 | 553 | end-of-turn | yes | invalid | — (0) | none |
| boxes12 | whole | 11705 | 735 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1, source-not-a-sentence 2, undeclared 7, wrong-count 4 |
| boxes13 | whole | 11705 | 644 | end-of-turn | yes | not decided | no-search (0) | name-not-listed 1, source-not-a-sentence 4, undeclared 2, wrong-count 3 |
| pencils | whole | 11735 | 1293 | end-of-turn | yes | inconsistent | no-search (0) | name-not-listed 2, number-as-name 6, undeclared 21 |

| item | --no-search | --single | the answer | gold | findings | categories (src/gen/rel/rule/assum/frame) |
|---|---|---|---|---|---|---|
| zebra | no program | | | | | |
| youngest | not decided | one | no line | Cy | wrong-count 2 | 5/3/1/2/0/4 |
| seating | inconsistent | inconsistent | no line | Bob | selection-inactive 2, undeclared 3 | 3/10/4/0/14/2 |
| bob1 | not decided | one | Bob has 5 apples. (not the gold) | 3 | none | 2/3/0/2/0/4 |
| gap | not decided | one | no line | none | wrong-count 4 | 5/3/1/2/3/6 |
| gap_fact | not decided | one | no line | 3, 6, 9 | name-not-listed 1, undeclared 11, wrong-count 3 | 6/3/1/6/1/4 |
| contra | not decided | one | Bob has 5 apples. (a line, where the text gives no answer) | none | undeclared 6 | 5/3/0/2/3/4 |
| unused | not decided | one | Bob has 1 apples. / Bob has 3 apples. / Bob has 5 apples. (the gold, and other values) | 3 | source-not-a-sentence 4 | 10/3/1/2/4/5 |
| bystander | inconsistent | inconsistent | no line | 9 | number-as-name 1, undeclared 7 | 3/3/0/2/7/4 |
| before | not decided | one | Bob had 1 apples before. / Bob had 3 apples before. / Bob had 5 apples before. (not the gold) | 7 | none | 2/3/0/2/0/4 |
| kinds | not decided | one | no line | none | answer-in-template 1, source-not-a-sentence 2 | 2/3/0/2/0/4 |
| kinds_full | not decided | one | They have 8 apples together. (the gold) | 8 | answer-in-template 1, source-not-a-sentence 4, undeclared 6 | 4/3/0/2/6/4 |
| more | invalid | invalid | no line | Alice, 7 | none | 0/0/0/0/0/0 |
| boxes12 | not decided | one | no line | 4 | name-not-listed 1, source-not-a-sentence 2, undeclared 7, wrong-count 4 | 3/3/0/4/5/5 |
| boxes13 | not decided | one | no line | 5 | name-not-listed 1, source-not-a-sentence 4, undeclared 2, wrong-count 3 | 6/3/1/2/4/6 |
| pencils | inconsistent | inconsistent | no line | 48, 10 | name-not-listed 2, number-as-name 6, undeclared 21 | 12/3/0/6/6/8 |

## The Zebra puzzle against the steps by hand and zebra3.1.ein

Forms compared by their text without `:source` and layout; relations by name.

| condition | step | forms | as the hand's step | the hand's step's forms | relations it writes | the hand's |
|---|---|---:|---:|---:|---|---|
| steps | 1 | 53 | 4 | 55 | drinks instance is-immediately-to-the-right-of lives-in owns smokes | are-smoked-in are-smoked-next-to drinks instance is-drunk-in is-immediately-to-the-right-of lives-in lives-next-to owns smokes |
| steps | 2 | 69 | 16 | 74 | includes instance is-immediately-to-the-right-of slot-partition slot-spatial symmetric type | includes instance is-immediately-to-the-right-of slot-partition slot-spatial symmetric type |
| steps | 3 | 15 | 0 | 18 | converse meet solve top | compose converse meet top |
| steps | the program against zebra3.1.ein | 104 | 18 | 113 | | |
| whole | whole | 724 | 0 | 113 | instance source-instance | are-smoked-in are-smoked-next-to compose config converse drinks includes instance is-drunk-in is-immediately-to-the-right-of lives-in lives-next-to meet owns slot-partition slot-spatial smokes symmetr |
| steps-free | 1 | 53 | 4 | 55 | drinks instance is-immediately-to-the-right-of lives-in owns smokes | are-smoked-in are-smoked-next-to drinks instance is-drunk-in is-immediately-to-the-right-of lives-in lives-next-to owns smokes |
| steps-free | 2 | 69 | 16 | 74 | includes instance is-immediately-to-the-right-of slot-partition slot-spatial symmetric type | includes instance is-immediately-to-the-right-of slot-partition slot-spatial symmetric type |
| steps-free | 3 | 15 | 0 | 18 | converse meet solve top | compose converse meet top |
| steps-free | the program against zebra3.1.ein | 104 | 18 | 113 | | |
| whole-free | whole | 539 | 12 | 113 | instance source | are-smoked-in are-smoked-next-to compose config converse drinks includes instance is-drunk-in is-immediately-to-the-right-of lives-in lives-next-to meet owns slot-partition slot-spatial smokes symmetr |

## The check's findings, by condition, step and kind

| condition | step | findings |
|---|---|---|
| steps | 1 | number-as-name 1, undeclared 9, wrong-count 1 |
| steps | 2 | name-not-listed 4, number-as-name 2, rewritten 3, undeclared 14, wrong-count 2 |
| steps | 2b | undeclared 7, wrong-count 1 |
| steps | 3 | mechanism-not-stated 2, mechanism-not-the-one-stated 1, name-not-listed 4, number-as-name 2, rewritten 3, selection-inactive 5, undeclared 14, wrong-count 1 |
| steps-free | 1 | number-as-name 1, source-not-a-sentence 7, undeclared 8, wrong-count 2 |
| steps-free | 2 | name-not-listed 1, number-as-name 2, rewritten 3, selection-not-listed 3, several-answer-queries 3, source-not-a-sentence 4, undeclared 10, wrong-count 2 |
| steps-free | 3 | mechanism-not-stated 1, mechanism-not-the-one-stated 1, name-not-listed 1, number-as-name 2, rewritten 3, selection-inactive 5, selection-not-listed 11, several-answer-queries 1, undeclared 21, wrong-count 1 |
| whole | whole | answer-in-template 2, name-not-listed 5, number-as-name 7, selection-inactive 2, source-not-a-sentence 16, undeclared 63, wrong-count 16 |
| whole-free | whole | answer-in-template 2, name-not-listed 5, number-as-name 7, selection-inactive 2, source-not-a-sentence 16, undeclared 63, wrong-count 16 |
