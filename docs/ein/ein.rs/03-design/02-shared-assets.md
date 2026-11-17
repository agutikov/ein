# Shared assets: one stdlib, one corpus

Where the Ein library and the corpus live, how the engine finds the library, and how drift between copies is caught.
The stdlib's resolution is [`ein_ir::stdlib`](../../../../ein.rs/crates/ein-ir/src/stdlib.rs); the corpus manifest's
reader is the dev-only crate `ein-corpus` ([`manifest.rs`](../../../../ein.rs/crates/ein-corpus/src/manifest.rs)).

## 1. Why one copy

A stdlib kept in two places makes every result ambiguous: a changed answer would read as *the engine moved* when the
*program* did. The stdlib is not test data; it is part of the semantics every program that imports it runs under.
The same holds, more weakly, of the corpus: a corpus kept in two places is two corpora. So each is checked in once,
and everything reads those bytes.

## 2. The stdlib directory

The stdlib is the repository's [`stdlib/`](../../../../stdlib/README.md), the single source of truth. No crate keeps a
checked-in copy. A module `std.x` is `x.ein` and `std.math.x` is `math/x.ein`: a dot is a directory, at any depth.
`stdlib/MANIFEST.sha256`, one line per module (`<sha256>  <path>`), is what identifies a directory as the stdlib
(`ein_ir::stdlib::MARKER`): a directory called `stdlib/` proves nothing on its own.

## 3. Resolution

`Source` is where a module's text comes from, resolved in three steps (`resolve_with`):

1. **`$EIN_STDLIB`**, an explicit override, which always wins (`Source::Override`). It must name a directory that
   carries the marker; one that does not is refused with a message saying so (`Source::problem`), rather than read as
   an empty library.
2. **The checkout**, `stdlib/` found by walking up from the executable and confirmed by the marker
   (`Source::Checkout`). A source tree is authoritative, so editing a module takes effect with no rebuild.
3. **The embedded copy**, compiled into `ein-ir` by `include_dir!` (`Source::Embedded`), which is what makes `ein` one
   self-contained binary.

A module none of them holds is reported as *module not found at `<path>`*, naming the path the source would hold it
at (`Source::module_path`), so no fallback is hidden. `ein --version` prints which source answered and its directory
([`10-toolchain-and-layout.md`](10-toolchain-and-layout.md)). Import resolution is the consumer of this
([`04-ir-frontend.md`](04-ir-frontend.md)).

## 4. Drift

Three copies of the stdlib can disagree: the checkout, the embedded bytes of a built binary, and the manifest. Each
pair is checked:

- **the embedded copy against the manifest**: `stdlib.rs`'s tests `the_embedded_copy_matches_the_manifest` and
  `the_embedded_copy_has_no_extra_modules`. `include_dir!` copies the tree at build time, so a binary built before an
  edit carries the old bytes, and a binary whose stdlib differs from the checkout's is correct about a different
  program, which no output diff could diagnose. The manifest is a build dependency of `ein-ir`, so it cannot go stale
  against the bytes compiled in;
- **the checkout against the manifest**: `the_checkout_has_no_module_the_manifest_does_not_list`, at any depth;
- **a derived artefact against the stdlib it was made under**: a `.einb` container records the manifest's digest
  (`ein_einb::meta::stdlib_digest`), so a stdlib edit invalidates every container made before it
  ([`09-binary-format.md`](09-binary-format.md)).

## 5. The corpus

The corpus is every `.ein` file under `examples/`, `stdlib/` and `tests/`, and
[`corpus/corpus.toml`](../../../../corpus/README.md) is its manifest: one `[[entry]]` per file, so every reader of the
corpus enumerates the same entries with the same runs. An entry holds:

| field | |
|---|---|
| `path` | the file, from the repository's root |
| `group` | what the file is for (`positive`, `regression`, `stdlib`, `parse-negative`, `load-negative`): a reader selects by group |
| `runs` | the invocations the file is exercised under; a run is dropped only when it does not ask the file's question |
| `levers` | the configuration levers the lever sweep runs it under |
| `slow` | a measured claim: the entry's declared runs cost a second or more together, recorded in `cost_ms` (`ein_corpus::manifest::SLOW_MS`) |
| `heavy_dump` | its describe dump writes at least `ein_corpus::manifest::HEAVY_DUMP_FILES` state files |

**The manifest is complete.** A `.ein` file under `examples/`, `stdlib/` or `tests/` with no entry fails
`manifest.rs`'s `every_ein_file_has_an_entry`, so the corpus cannot silently miss a file. **`slow` is checked in both
directions**: `slow_matches_the_recorded_cost` fails an entry flagged below the line or unflagged above it, against
its recorded `cost_ms`, with no engine run; `corpus_cli.rs`'s `the_slow_flag_still_describes_the_sweep` compares the
recorded cost with the sweep it has just run; and `the_slow_set_is_exactly_these_fifteen` names the set. A `slow`
entry stays out of the default sweep; every sweep reads it with `EIN_CORPUS_SLOW=1`.

The negative fixtures are corpus files too: a program the parser or the loader refuses lives under
`examples/broken/` with its expected message beside it (`<name>.expected`), so the text a test asserts is a file of
the corpus rather than an inline string.
