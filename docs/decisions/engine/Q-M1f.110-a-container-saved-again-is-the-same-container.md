# Q-M1f.110 — A container saved again is the same container

`ein kb save` of a `.einb` writes the file it read, byte for byte. The writer stores the derivation records up to
the last one the KB references, a believed fact's primary or alternative derivation or a closure's smallest arrival,
each at its position, so no stored id moves; the records a reader pushes when it opens a container, which nothing
in it references, are past that point and are not written. A record inside the prefix that nothing references, a
fact stated twice, is written as it always was. A save holds no time, so two saves of one program are one file.

**Why.** A cache saved again must not grow, and its ids must stay the ones its sections store, so that what it
reads back is the KB it was saved from.

**Held by.** `ein_einb::sections::write_prov`, with `Kept`; `ein-cli/tests/einb_cli.rs`:
`a_container_saved_again_is_the_container`; `ein-einb/tests/roundtrip.rs`:
`every_corpus_entry_round_trips_into_an_empty_and_a_shared_interner`.
