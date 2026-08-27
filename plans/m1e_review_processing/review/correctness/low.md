# Correctness — Low

## Interner/FactStore u32 arena offsets are bounded by id count, not by arena bytes

**Severity:** Low
**Confidence:** High
**Topic:** Correctness
**Classification:** code bug (unreachable in practice, contradicts a stated principle)

**Locations**
- `ein.rs/crates/ein-core/src/intern.rs:26-32, 116-127`
- `ein.rs/crates/ein-core/src/facts.rs:126-141`

### Finding

The Interner stores span starts as `u32` (`self.arena.len() as u32`, intern.rs:119) and FactStore stores `args_at` as `u32` (facts.rs:129), but the CAPACITY guard limits only the *number of ids* (2^30). A table with fewer than 2^30 entries whose total text/args exceed 4 GiB would silently wrap the offset and corrupt spans, with no error. The CAPACITY doc comment ("Reaching it needs ≥ 4 GB of symbol text") conflates the two limits.

### Impact

Unreachable for any corpus-scale input — but the module's stated design principle is that hitting a limit "is an error somebody can read rather than a silent wrap into another value's identity" (intern.rs:29-31), and this wrap is exactly the silent kind.

### Recommendation

A checked cast or an arena-size guard beside the existing id-count guard; fix the doc comment to state both limits.
