# Documentation — Medium

## Systemic count rot: every number not enforced by a test has drifted at least one milestone behind

**Severity:** Medium
**Confidence:** High
**Topic:** Documentation

**Locations**
- `tests/README.md:187, 189-190, 198`
- `corpus/README.md:91, 101, 112, 211`
- `examples/README.md:13`
- `stdlib/README.md:81`
- `utils/README.md:27, 49, 129-133`; `utils/stdlib_census.py:26` (docstring)
- `README.md:73` ("703 tests" — the gate now passes 738), `:341-344` ("eighteen scripts" — utils has 23), `:600-607` (Layout rows: docs/history missing m1d, --version described as four things where the report has five lines)
- `ein.rs/crates/ein-infer/tests/stdlib_coverage.rs:8, 20, 32, 210` and `ein.rs/crates/ein-cli/tests/corpus_cli.rs:9, 44-45, 97-103, 107, 216` (doc comments)
- `docs/kernel/defined_behaviour.md:135` ("23 of the 30" broken/load fixtures — the directory holds ~37)

### Finding

Verified instances: 73-vs-77 stdlib rules; 45/47/56 tests/stdlib programs (inconsistent within corpus/README itself); 180/189/197 corpus entries; 225-vs-280 cells; 622/641-vs-~990 sweep cells; 84-vs-89 renderables; 703-vs-738 gate tests; eighteen-vs-23 scripts. By contrast, every number a test pins (77 of 77, 56 expectations, 49/516 lines, the embedding output) is exactly right — the repo's own "a page nothing runs goes stale" thesis, demonstrated on its own docs. The stdlib row of corpus/README was patched for S1d.2.2 (+2) but not S1d.2.4 (+9), which dates most of the rot to one milestone.

### Impact

A reviewer or tool trusting the READMEs' counts is wrong about the suite's current size in at least six places; and because CLAUDE.md claims the doc tree is checked by cargo test, the reader has no way to know which numbers are in the checked class.

### Recommendation

Either stop stating exact counts in prose (say "one per rule; the census script prints the number"), or generate the counts (several already have instruments: stdlib_census, corpus_cost). At minimum, one counting pass now.

---

## Dangling references across the doc tree

**Severity:** Medium
**Confidence:** High
**Topic:** Documentation

**Locations**
- `examples/README.md:28-29`, `stdlib/README.md:139-140` ("see C2" — no such file; almost certainly a deleted-plans link reduced to its text)
- `docs/kernel/ir/01-ein-graph/02_rules.md:586-587` (bare "M1 P1.3"-style paths)
- `docs/kernel/inference/zebra_walkthrough.md:52` (`s1.6.5_idea08_checklist.md`)
- `docs/kernel/architecture.md:132` (`r6_seam.md`)
- `docs/kernel/inference/algorithm_layer_n.md:42, 522` (`[project-set-search-unified memory]`)
- `docs/kernel/inference/README.md:916, 1060, 1064` and `lattice_diagrams.md:216, 251-252` (anchors §3d.iii/§3d.iv/§3d.vii/§3e that do not exist in the target)

### Finding

The plans-tree deletions (M1a/M1c/M1d) left link text whose targets are gone, and several section anchors never existed in the linked page. The "C2" references are the worst: the referent (the two-ontology design comparison) is now unlocatable from either document.

### Recommendation

A link-checking pass (the rustdoc step already catches this class for crate docs — a markdown-link checker over docs/ would catch these); for C2, either restore the content from git history into docs/history or delete the sentences.
