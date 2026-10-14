# Semantics — Medium

## `k` and `stats.solution_nodes` split by name in S1d.2.6, but two user-facing surfaces still print the wrong one

**Severity:** Medium
**Confidence:** High
**Topic:** Semantics
**Classification:** design ambiguity (one site documented-deliberate, one undefended)

**Locations**
- `ein.rs/crates/ein-cli/src/test.rs:832-838` vs `:784-795`
- `ein.rs/crates/ein-render/src/answer.rs:419-433` (cross-ref `review/correctness/medium.md`)

### Finding

M1d S1d.3.3 deliberately split two numbers that had always agreed: `verdict.k` counts *models*, `stats.solution_nodes` counts what the search recorded; they part on `Open`. But in `ein test`'s verbose per-query header, `k = {}` prints `stats.solution_nodes` (deliberate per the comment at :784: "what it has printed since S1c.1.3"), so the human-facing `k =` and the machine-facing `ran.k` disagree *by name* on every Open entry — the exact vocabulary confusion S1d.2.6 elsewhere eliminated. The Solution table arm has the same inheritance (see correctness/medium.md).

### Impact

A consumer reconciling the verbose header against `--json-report` sees two different values both labelled `k`.

### Recommendation

Rename the verbose header's label (`recorded =`) or print `Verdict::k` with the search count beside it, as the Ran row already does.

---

## The Aborted summary breaks the "no sometimes-fields" principle the rest of the schema states three times

**Severity:** Medium
**Confidence:** High
**Topic:** Semantics
**Classification:** design ambiguity

**Locations**
- `ein.rs/crates/ein-cli/src/summary.rs:600-632` vs `:139-141, 160-173, 585-595`

### Finding

`build_aborted`'s summary omits the `open_states` key from `verdict` and the whole `leftover` block, both of which the normal build emits unconditionally. summary.rs justifies unconditional emission three times with "a field that appears only sometimes is a field a consumer has to guess about" (the rule the `owes` and `config` blocks follow) — yet the Aborted shape breaks it: a consumer switching on schema `ein-summary/1` sees `verdict.open_states` and `leftover` only on non-aborted runs.

### Impact

Machine consumers must special-case aborts; the schema's own design rule says they shouldn't have to.

### Recommendation

Emit the fields present-and-empty on aborts (as `owes` already is), or record the exception as deliberate in the schema's doc.

---

## Tree-traversal reporting semantics are under-specified relative to the shipped surface

**Severity:** Medium
**Confidence:** High
**Topic:** Semantics
**Classification:** design ambiguity

**Locations**
- `ein.rs/crates/ein-infer/src/solve.rs:916, 991-1013`
- `docs/kernel/inference/events.md:172-183` (no `traversal` row)
- `tests/tree_traversal.rs:75-77` (the only statement of the `-n`/`-m` behaviour)

### Finding

Under `EIN_TRAVERSAL=tree`: `exhausted` is reported `false` by design (termination-by-discharge), dead branches contribute nothing to nogood counters or the proof, `-n`/`-m` are ignored, and the `traversal` event the decline path emits is documented nowhere in the events protocol, whose page claims to be "every step the engine took". T1d.10.6.4 is honestly recorded as the open question for what a tree reports where a lattice reports layers — but the pieces that already ship (the event, the counter behavior, the flag interactions) have no statement anywhere a user or consumer would look.

### Impact

The headline M1d capability is observable only through surfaces whose meaning under it is unstated.

### Recommendation

Document the shipped subset now (one paragraph in events.md + the EIN_TRAVERSAL block), independent of the open design question.

### Cross-references

- `review/correctness/high.md` (the tree-traversal defect cluster).
