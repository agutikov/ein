# E-123 — No value says whether a selecting fact was taken up, and an unused one is silent whatever the reason

**Kind:** gap
**Status:** open
**Filed:** by S2.2.4, 2026-11-26, on `7aaefe6f`
**Owner:** none

## What

**No value the kernel returns says whether a fact that selects a theory was taken up.** A selection is a fact whose
name a rule of an imported module takes as its activator (`(transitive older)`), or a fact of a relation a module
reads and no module declares (`(slot-partition …)`). Whether the rule then did anything is in root's narration alone:
the `--events` protocol's `fire` and `owe` lines, each with its rule, its activator and its premises, as text a
reader parses ([`05-events.md`](../../../../../docs/ein/ein.rs/05-events.md)). EinAf's check reads them so
(`einaf::kernel::root`, the check's second finding, read off root's saturation), and three different cases look the
same there, **no firing at root**:

- **a misspelt selection**, a name the file does not list from the module that declares the rule: the fact is the
  file's own and inert (E-11), which the text shows and the kernel does not say;
- **a selection over labels that have no members**: the model's Zebra run wrote `(top smoker-and-house instance Smoker
  House)` where nothing is `(instance … Smoker)`, four such `top` and a `meet` over one of them: each loads, is listed,
  and fires nothing anywhere;
- **a read-back whose premises a state adds above root**: the Zebra puzzle's steps by hand select `(compose is-kept-in
  is-lived-in-by is-owned-by)` and `(compose is-owned-by lives-in is-kept-in)`, which fire only once a commitment
  places the pets, and are right.

Nothing the kernel returns tells the second from the third: the answer query's row is missing in both, and the
description lists the states without saying which selected rule never fired in any of them.

## Evidence

S2.2.4's record, [`../../steps/`](../../steps/README.md): the run `steps/zebra`, step 3, five findings
`selection-inactive` (the four `top` over `Smoker`, `Drinker`, `Owner`, `Color` and the `meet` over the colour);
`einaf steps check einaf/contract/zebra/step3.ein --text einaf/contract/zebra/text.txt`, two (the two `compose`
above); `einaf/tests/catalogue.rs`, which holds each entry's selection active by the same reading and leaves out the
library's checks (`functional`, `injective`), which fire only where a state breaks them.

## What it costs the application

A selection that does nothing is the commonest silent failure of steps 2 and 3, and the one the user's *select, do
not invent* rests on. A reader has to parse the narration and, to tell a wrong selection from a waiting one, ask the
states above root what fired there, which no question of the kernel's returns as a value.
