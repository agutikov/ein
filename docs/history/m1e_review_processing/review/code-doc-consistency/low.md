# Code ↔ doc consistency — Low

## The five history-page banners omit `ein test` from the CLI enumeration

**Severity:** Low
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `docs/api/{ein,ir,kb,inference,trace}.md` (the shared 🏛 banner: "the CLI: ein solve · ein saturate · ein render · ein kb")

### Finding

The CLI has had five subcommands since M1c S1c.1.3. The banner is the one part of a history page that is supposed to describe the *present*, and it is copied identically five times, so one fix is five.

---

## Guide chapter 4's transcript does not match actual output

**Severity:** Low
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug

**Locations**
- `docs/guide/04_solving_the_whole_puzzle.md:66-68`

### Finding

Shown as two bindings per line in a different order than the binary prints (one per line: h_water, h_zebra, who_water, who_zebra). Content right, layout wrong; a reader diffing against a real run sees a mismatch. Nothing runs the guide's transcripts — the same rot class the embedding test was built to prevent, unguarded here.

### Recommendation

Re-paste from a run; consider extending the embedding-style diff to the guide's one big transcript.

---

## render_lattice's fallback comment states a wrong reason at the one CLI site that triggers it

**Severity:** Low
**Confidence:** High
**Topic:** Code-doc consistency
**Classification:** documentation bug (in-code)

**Locations**
- `ein.rs/crates/ein-render/src/lattice_dag.rs:288-292` vs `ein.rs/crates/ein-cli/src/render.rs:79-84`, `cmdline.rs:146-150`

### Finding

The comment says "no stored lattice (store_lattice=False) — showing the solution frontier instead", emitted even when the solve ran with store_lattice=true (as `ein render lattice` always does). The real reason — no per-commitment SetNode DAG exists at all — is given correctly by the `--view` help text, so the two disagree.
