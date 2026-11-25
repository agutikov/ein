# E-25 — A whole state is in no JSON key

**Kind:** gap
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none. [S2.1.3](../../../../docs/einaf/03-feedback.md): the library gives a state as its facts over the candidates' relations, root's included (`read_out::Listed::model`), and no more; the feedback object's state digest is of that list and says so (`of: model`), and a whole state's digest would need the state's KB, which no read-out keeps

## What is missing

**A whole state is in no JSON key**, and no command prints a program's canonical dump; `--dump-answer` writes each state as a file

## Evidence

[analysis § 2 and § 3](../../analysis/README.md#p24--the-loop-audit)

## What it costs the application

a caller of the binary cannot hash a state or a program; a caller of the library can
