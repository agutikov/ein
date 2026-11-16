# Q-M1f.10 — A `.einb` file holds no integer table

The `.einb` format, 2.0, has no `INTS` section: Ein has no integers, so a writer writes none and a reader requires
none. The section's kind number, 3, is retired and never reused, so a section of kind 3 is an unknown one and is
skipped as any unknown kind is. A file of format 1.x is refused with the version line, and is a cache to save again;
a stored value with the integer's tag is refused with its own sentence, `EinbError::Int`, and never read as a name
or a fact.

**Why.** The major version moves on any layout change, and a minor one only adds a section; a layout with a table
that no value can fill is a layout to drop at a major. A file that once held an integer has no value to read it as,
so it is refused rather than misread.

**Held by.** `ein_einb::header::FORMAT_MAJOR` and `Kind`; `ein-einb/tests/invalidation.rs`:
`a_file_that_holds_an_integer_is_refused`, which holds no section of kind 3 in what the writer writes, a 1.x header
refused and a tagged value refused. `ein --version` prints the container's format, `einb/2.0`.
