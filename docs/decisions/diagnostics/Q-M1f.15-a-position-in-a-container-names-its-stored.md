# Q-M1f.15 — A position in a container names its stored program

A `.einb` container stores its program as canonical text, resolved and import-flattened, and a command that opens
one reads that text back through the parser and the loader. A position in it, which only a refusal prints, names
the container, `<program stored in x.einb>` (`<program stored in a .einb>` for a container read from bytes with no
name), and its line and column are the stored text's.

**Why.** No file holds the stored text, so a position has to say which text it is in, and the brackets mark a name
that is no path, as `<string>` does. The container keeps no source position past its provenance table, and a run
that answers prints no position, so `ein solve x.einb` prints what `ein solve x.ein` prints but the path it echoes.

**Held by.** `ein_einb::program_name`; `ein-cli/tests/program_read.rs`:
`a_containers_positions_name_its_stored_program`.
