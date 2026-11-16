# Q-M1a.22 — A container's solve is its source's, byte for byte

`ein solve x.einb` prints what `ein solve x.ein` prints, byte for byte but for the path it echoes, with the same
stderr and exit code. That is promised where a fresh process opens one container: its interned tables re-intern into
the process's own ids, every id table is the identity, and the remap is skipped. A library caller that opens a
container into a process already holding other names gets the same answer, and its narration (a trace, which of two
equally valid justifications was recorded first) may differ.

**Why.** Every `ein` command is a fresh process opening one file, so the CLI always takes the identity path, and the
promise a container's user cares about, the answer, holds on any path. Byte identity off that path would ask the
remap to keep an order the engine does not keep for itself: which justification is recorded first already depends on
id order, and that order is narration, not answer.

**Held by.** `ein-cli/tests/einb_cli.rs`: `a_solve_of_a_container_is_byte_identical_to_a_solve_of_its_source`;
`ein-einb/tests/roundtrip.rs`: `every_corpus_entry_round_trips_into_an_empty_and_a_shared_interner`;
`ein-render/tests/id_order_invariance.rs`: `every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`,
every answer of every corpus program the same under a permuted id space. The identity test:
`ein_einb::tables::Maps::identity`.
