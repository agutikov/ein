# E-69 — One line, two sections

**Kind:** weakness
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What

**One line, two sections.** In seven places `07-diagnostics.md` states one line under two sections (a goal's guard over a form, § 2.1 and § 2.9; a goal's relation variable, § 2.15 and § 2.21; a connective where a value goes, § 2.16 and § 2.22; …), so a section is not a kind by itself: the kind is the section that states the check

## Evidence

[`07-diagnostics.md`](../../../../docs/ein/user/04-reference/07-diagnostics.md)'s `**Kind:**` lines, which say where they take another section's kind

## What it costs the application

a caller that maps a kind to a section reads the check's section, not the one a reader may find the line in
