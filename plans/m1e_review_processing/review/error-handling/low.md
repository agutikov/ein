# Error handling — Low

## `-n 0` is accepted while `--jobs 0` is refused with a reasoned message

**Severity:** Low
**Confidence:** High
**Topic:** Error handling
**Classification:** design ambiguity

**Locations**
- `ein.rs/crates/ein-cli/src/solve.rs:570-574`
- `ein.rs/crates/ein-cli/src/cmdline.rs:171-179` vs `:20-47`

### Finding

`solve` accepts `-n 0` (`py_int` allows 0; `stop_after` becomes `Some(0)`), even though `--jobs 0` is refused with a message arguing exactly that a flag with two readings should be refused. Stop-after-zero has no obvious meaning; the engine's treatment is whatever `SolveOptions{stop_after: Some(0)}` does. Likely ein.py parity, but unstated anywhere.

### Recommendation

Refuse it (matching the jobs_spec argument) or document its meaning and pin it with a test.

---

## Non-einb builds sniff 5 magic bytes where `is_einb` requires 8

**Severity:** Low
**Confidence:** High
**Topic:** Error handling
**Classification:** code bug

**Locations**
- `ein.rs/crates/ein-cli/src/common.rs:107`
- `ein.rs/crates/ein-einb/src/header.rs:14, 164-166`

### Finding

A file beginning `EINB\0xyz` is refused as "a .einb container and this build has no einb feature" in a `--no-default-features` build but treated as (invalid-UTF-8 or parse-error) text in a default build — a behavioral divergence between the two shipped feature sets on garbage input.

### Recommendation

Make the two literals one constant.
