# E-87 — `region.stopped.why` has two values no summary reaches

**Kind:** weakness
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What

**`region.stopped.why` has two values no summary reaches**: the writer spells `deadline` and `no-search`, and the clock's summary has no `region` while `--no-search`'s `region` is `null` but where a cap stopped it; the page lists `asked`, `budget`, `limit` and `steps`

## Evidence

`ein_render::dump::region_answer`'s `stopped`; [`08-read-out.md` § 11.2](../../../../docs/ein/user/04-reference/08-read-out.md#112-the-fields-no-row-prints-as-itself)

## What it costs the application

none today; a reader of the code may expect them
