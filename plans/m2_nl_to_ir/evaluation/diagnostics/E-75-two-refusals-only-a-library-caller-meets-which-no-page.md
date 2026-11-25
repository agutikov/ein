# E-75 — Two refusals only a library caller meets, which no page states

**Kind:** gap
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What is missing

**Two refusals only a library caller meets, which no page states**: a file-relative import with no base directory (`import`) and `read_goal`'s *not one goal* (`goal`)

## Evidence

`ein_ir::imports` (*file-relative import needs a base directory*), `ein_ir`'s `read_goal`

## What it costs the application

a caller that loads text with no directory meets a refusal the reference does not have
