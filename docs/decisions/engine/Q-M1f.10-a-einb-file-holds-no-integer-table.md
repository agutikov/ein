# Q-M1f.10 — A `.einb` file holds no integer table

The `.einb` format, 2.x, has no `INTS` section: Ein has no integers, so a writer writes none and a reader requires
none. The section's kind number, 3, is retired and never reused, so a section of kind 3 is an unknown one and is
skipped as any unknown kind is. A file of format 1.x is refused with the version line, and is a cache to save again;
a stored `Value`, the engine's word for an object, with the integer's tag is refused with its own sentence,
`EinbError::Int`, and never read as a name or a fact.

**Why.** The major version moves on any layout change, and a minor one only adds a section; a layout with a table
that no object can fill is a layout to drop at a major. A file that once held an integer has no object to read it
as, so it is refused rather than misread.

**Held by.** `ein_einb::header::FORMAT_MAJOR` and `Kind`; `ein-einb/tests/invalidation.rs`:
`a_file_that_holds_an_integer_is_refused`, which holds no section of kind 3 in what the writer writes, a 1.x header
refused and a tagged `Value` refused. `ein --version` prints the container's format, `einb/2.2`, its minor moved by each section
added ([Q-M1f.413](Q-M1f.413-a-nodes-container-opens-as-the-node.md)).