# Part 1 — the verdict *dead*: `false`, `not`, `relation`, the key

[← the bundle](../README.md) · [← the phase](../../README.md) · [your answers](ANSWERS.md)

The open questions of the phase's first part ([the order](../../README.md#the-order-two-parts)). An answer goes
into [`ANSWERS.md`](ANSWERS.md) here; an item left blank keeps its default.

**What S1f.21.11 cannot take by itself.** No default; each is shown on programs here and on the corpus by that
stage's examples.

| # | item | the question | state |
|---|---|---|---|
| 35 | [Q-M1f.557](35-Q-M1f.557/README.md) | how the contradiction's rules reach every relation and count: an undeclared relation, and one of seven arguments, have none once the kernel's check goes | **not ruled**; five ways; proposed, since the user's remark that it may be a feature and no gap: (d), the contradiction is `std.contradiction`'s, on by import for the relations a program declares |
| 36 | [Q-M1f.447](36-Q-M1f.447/README.md) | the argument-count check, once `relation` is a convention: kept at load, a rule's verdict, or dropped | **no default**; yours to think about |

**A default is proposed or taken; an answer is wanted only where you would have it otherwise.**

| # | item | the question | the default |
|---|---|---|---|
| 37 | [Q-M1f.558](37-Q-M1f.558/README.md) | *forbid*, once the contradiction is rules: seven corpus programs would be refused | (a) *forbid* reads the module's contradiction as it reads the kernel's; the seven load as now |
| 38 | [Q-M1f.559](38-Q-M1f.559/README.md) | the elimination of a double negation, and the module every program gets: it is refused beside four corpus programs | (a) it stays a module a program imports; `std.core` gets the contradiction alone |
| 40 | [Q-M1f.552 to .556](40-defaults-taken/README.md) | five readings the two finished stages took and built | as built; the fourth not as planned: two modules for the negation |
| 41 | [S1f.21.10 as built](41-S1f.21.10-readings/README.md) | the key's five readings, and four consequences of a module every program gets: an import of it refused, older saved files no longer load, `$EIN_STDLIB`, the container's format | as built (Q-M1f.562, Q-M1f.563); shown on six programs |
| 42 | [the open findings](42-findings/README.md) | what becomes of V-8, V-10, V-11, V-13, V-15, V-16 and V-17 | a disposition each |

**Left for later on the user's word, and not asked**: how `std.core` is customised (Q-M1f.551).
**Out of the bundle, ruled**: item 33, the names (Q-M1f.399, with the key's word, Q-M1f.550), 2026-11-19; its page
and programs are in `git log`, last at `984137bf`. Item 43, one way to say *must* and one to say *may* (Q-M1f.561), declined the same
day: every existing way to control the hypotheses stays; its analysis and its programs are
[`one_way_each.md`](../../one_way_each.md).

**The binary.** Items 35 to 38 are run with `build/review-bin/ein-head-ca98dec2`, the gate build of master at the
merge of S1f.21.6 and S1f.21.8, and its `stdlib/`. `tools/run.sh` in an item re-takes its runs into `runs/`; both
are ignored, so they are on this machine only.

## Found while building

- **35**: the rules derive `(false)` from a pair of a declared relation of one argument and nothing from the same
  pair of an undeclared relation or of one of seven arguments; both are dead today by the kernel's check alone.
  `clash-of-0` fires on every declaration, since root holds `(relation p)` beside `(relation p Thing)`.
  The user's rule, the relation a variable of the pattern with no parameter, is refused at load in either order of
  its premises: *a relation is named by a name or by a parameter its activator binds*.
- **36**: a fact and a rule's conclusion at another count than the declaration's are each refused at load, with the
  declaration quoted; an undeclared relation at two counts loads, and each query answers its count.
- **37**: the soup program loads without the rules, 4 states, flagged by the static check, and is refused with
  them, by `clash-0` and `clash-2` over `meal`. Of the seven corpus programs, four are the witnesses of three open
  rows of the wrong-answer register.
- **38**: one program, four forms: 16 states with no import; 4 with `std.negation` and the hrule named once; refused
  with it named twice; 16 with `std.contradiction` and it named twice.
- **41**: a rule that concludes `(false)` and the same rule stating the key itself give the same dead root, the same
  core and the same `refuted by`; `(think Ann (false))` is alive; `:ein dead`, both clauses on one rule and `(import
  std.core)` are each refused with a line of their own; a state file dumped the day before no longer loads.

