# E-91 — A core's sentences stand beside root's core alone

**Kind:** gap
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What is missing

**A core's sentences stand beside root's core alone**: `region.core_sources` is root's; a hole's core (`region.holes.minimal[].core`), `N`'s members' (`nogoods.json`), `nodes.json` and `--events`' `enter` lines carry no `:source`

## Evidence

`ein solve examples/ein-bugs/zebra2-bad.ein --json-summary F`: `region.core_sources` `["injected contradiction", null, null]`; `--dump-answer D`'s `nogoods.json`

## What it costs the application

the feedback for a dead commitment above root names its facts, not their sentences
