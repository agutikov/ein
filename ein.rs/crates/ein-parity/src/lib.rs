//! What a derivation's **narration** is, executable —
//! [design/01 §5](../../../../plans/m1a_rust/design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list).
//!
//! **Not part of the engine.** `publish = false`, and nothing outside a test
//! depends on it. It exists because
//! [S1a.6.9](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
//! left the same decision implemented six times — in the parity harness, in
//! three `tests/` files, in `ein-render`'s `shape.rs` and in its
//! `utils/ir_oracle.py` twin — each cut made one at a time as the next test
//! went red. A relaxation that has to be discovered by running the tests is
//! not a contract, so this crate is where the decision lives and the call
//! sites only say *what they are handing over*.
//!
//! # The rule
//!
//! > **A fork's derivation, and anything keyed on a dying fork's stopping
//! > point, is narration.**
//!
//! Since S1a.6.9 a fork *resumes* root's saturation rather than re-deriving it
//! ([D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)),
//! so two runs can reach the same answer while narrating different amounts of
//! the same derivation: a quarter of the firings, a different one of each
//! fact's equally valid justifications recorded first, and — with
//! `enable_fail_fast_fork` on — a different stopping point for a fork that
//! dies. What is *not* relaxed in any direction is the **answer** and the
//! **search**: the verdict, the model, the unsat core, every counter.
//!
//! # Why it outlived the harness
//!
//! The rule was written for two engines, and
//! [S1a.10.3](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md)
//! retired the harness that compared them. This crate survived, because
//! [S1a.10.1 §5](../../../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#5-what-the-successor-found)
//! showed the same three observables move **inside one engine**: run the
//! corpus twice with the id space permuted and 66 renderings move, all of them
//! a dying fork's stopping point, a firing count, or which of a fact's equally
//! valid justifications was recorded first. Nothing had to be added to
//! [`is_narration`] for that. So the rule stopped being a statement about two
//! implementations and became one about what a derivation *is*, which is why
//! it is a normalisation the engine's own goldens apply to themselves.
//!
//! Who applies it now:
//!
//! | call site | to what |
//! |---|---|
//! | `ein-render/tests/corpus_ops/mod.rs` | every op of every corpus file, before digesting it into `corpus_shapes.md5` |
//! | `ein-render/tests/id_order_invariance.rs` | the same, twice, under a permuted id space — and it is what says the 66 are the narrated 66 |
//! | `ein-render/src/shape.rs` | [`NARRATED`], so a shape and a golden agree on the word |
//! | `ein-infer/tests/event_cut_control.rs` | [`events`], through a deliberately mutated stream |
//!
//! Read as three mechanical consequences, which is all this crate is:
//!
//! 1. **A firing count is narration** — `firings=` / `"firings"` /
//!    `"n_firings"`, and the `wrote … (N steps, …)` line the CLI prints for a
//!    `--trace`. Value elided, presence kept: a record that *lost* its count
//!    still fails. [`blank_line`].
//! 2. **A rendered derivation is narration** — the markdown trace, the
//!    `slice` provenance cone, a fork's own `enterings/` dump, and the lattice
//!    DOT a snapshot draws from dead state keys. Compared for presence; the
//!    regression coverage that replaces the byte diff is the ein.rs goldens of
//!    [S1a.6.11](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md).
//!    [`is_narration`], [`blank_blocks`].
//! 3. **A dying fork's stopping point is narration** — a `dead-post`
//!    entering's unsat core and the state keys derived from it. The run's own
//!    answer — the *printed* core, `union_dead_cores`, every `summary.json`
//!    field — is compared exactly. [`blank_line`].
//!
//! The event stream gets its own module, [`events`], because eliding it line
//! by line would throw away the one thing worth comparing: **what each fork
//! derived**. §2 of that module is the measurement that chose the cut.
//!
//! # Turning it off
//!
//! [`strict()`] — `EIN_PARITY_STRICT=1` — restores the byte-identical contract
//! P1a.1–P1a.5 was built against. It is not a configuration the suite passes:
//! it is how the relaxation gets *measured*, by running a sweep under it and
//! reading off what the cut was covering. `id_order_invariance` prints the 66
//! by op under it; relaxed, it asserts they are zero.

pub mod events;

/// What a rendered derivation is replaced by. A block that *disappears*
/// still fails, which is the point of substituting rather than dropping.
pub const NARRATED: &str = "<narrated>";

/// Is the D3 relaxation off? `EIN_PARITY_STRICT=1`.
///
/// Everything else in this crate *is* the relaxation and applies it
/// unconditionally; a strict call site does not call it. That is deliberate —
/// a normalisation that reads an environment variable half way down cannot be
/// unit-tested without mutating process state, and `cargo test` runs a crate's
/// tests in one process, in threads.
pub fn strict() -> bool {
    matches!(
        std::env::var("EIN_PARITY_STRICT").as_deref(),
        Ok("1" | "true" | "yes")
    )
}

/// The closed list of **rendered derivations** — consequence 2 of the rule.
///
/// The name is whatever the call site calls the thing it is handing over: a
/// DOT view name (`dot_parity`), a shape block head (`trace_parity`,
/// `dump_shape`'s `=== …` tree), or a path inside a `--dump-states` tree. They
/// do not collide, and keeping them in one list is the difference between a
/// contract and six tolerances.
///
/// | name | what it renders | why it moved |
/// |---|---|---|
/// | `slice` | the solution's provenance cone | it *is* the firing list, drawn |
/// | `markdown`, `ir`, `ir-reparsed`, `no-proof` | `trace_shape`'s rendered trace and its IR | ein.rs's has a *Before any assumption* section ein.py has no counterpart for, and a spine a quarter the length |
/// | `dot solution`, `dot full` | the lattice DAG drawn **from a snapshot** | it keys dead nodes on the dead commitment's `state_key`, so even the node count moves. The renderer itself is still byte-compared through `dot_parity`'s `lattice` / `lattice-full`, which read a `LatticeProof` and label dead nodes by *commitment* |
/// | `enterings/…` | a fork's own firing list and state dump | the fork's derivation, in full |
/// | `*.md` written by `--trace` | the CLI's markdown trace | as `markdown` |
///
/// Nothing else. `--- answer`, `--- table`, `--- round-trip`, `summary.json`,
/// stdout, every state dump outside `enterings/` and the `lattice` /
/// `lattice-full` views are all still compared byte for byte, and none of
/// them moved.
pub fn is_narration(name: &str) -> bool {
    const NAMES: [&str; 7] = [
        "slice",
        "markdown",
        "ir",
        "ir-reparsed",
        "no-proof",
        "dot solution",
        "dot full",
    ];
    let name = name.trim();
    NAMES.contains(&name) || name.starts_with("enterings/")
}

/// Blank the narration **values** on one line — consequences 1 and 3.
///
/// Values, never keys: `"firings": 18` becomes `"firings": #`, so a record
/// that stopped carrying a firing count is still a difference. The `core`
/// rule is conditional on the same line naming a `dead-post` entering, which
/// is how both the JSON timeline (`"kind": "dead-post"`) and `hypgen_parity`'s
/// text shape (`kind=dead-post`) carry it — an **alive** entering's core is
/// the answer and is compared exactly.
pub fn blank_line(line: &str) -> String {
    let mut out = blank_counts(line);
    if line.contains("kind=dead-post") || line.contains("\"kind\": \"dead-post\"") {
        out = blank_bracketed(&out, &["core=[", "\"core\": ["], ']');
    }
    // `snapshot_shape`'s dead state keys — the same field the timeline's
    // `core` is, projected: the DAG merges dead commitments by state key, so
    // the keys are the dying forks' stopping points and the *count* is what
    // survives.
    if let Some(at) = out.find("deads          ") {
        let (head, tail) = out.split_at(at + "deads          ".len());
        let n = tail.matches("{").count();
        if tail.starts_with('[') {
            out = format!("{head}{n} key(s)");
        }
    }
    out
}

/// Every firing counter on a line, by value.
///
/// The four sites, each named: `hypgen_parity`'s `firings=` shape,
/// `state_dump.py`'s `"firings"` timeline record, the `enter` event's
/// `"n_firings"`, and `cli/solve.py`'s `wrote <path> (N steps, M refuted)`.
/// `M refuted` is **not** a firing count — it is how many commitments died,
/// which is T1 — so it stays.
///
/// An event line carries a fifth: its ordinal `"n"`, which counts **every**
/// event including the firings a shape filters out, so it moves with them.
/// [`events::comparable`] drops it structurally on a parsed log; a *text*
/// shape that quotes event lines gets it blanked here instead — and only on a
/// line that names its event kind, so an `"n"` that means something else in
/// some other artefact is left alone.
fn blank_counts(line: &str) -> String {
    const KEYS: [&str; 3] = ["firings=", "\"firings\": ", "\"n_firings\": "];
    let owned;
    let line: &str = if line.contains("\"e\": \"") {
        owned = blank_bracketed_digits(line, "\"n\": ");
        &owned
    } else {
        line
    };
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    // The earliest key wins each round, or `"n_firings"` and `firings=`
    // interleave on a line that carries both.
    while let Some((at, key)) = KEYS
        .iter()
        .filter_map(|k| rest.find(k).map(|i| (i, *k)))
        .min_by_key(|(i, _)| *i)
    {
        out.push_str(&rest[..at + key.len()]);
        rest = &rest[at + key.len()..];
        let end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        out.push('#');
        rest = &rest[end..];
    }
    out.push_str(rest);
    if out.starts_with("wrote ") {
        out = blank_steps(&out);
    }
    out
}

/// `wrote …/trace.md (20 steps, 2 refuted)` → `(# steps, 2 refuted)`.
fn blank_steps(line: &str) -> String {
    let Some(at) = line.find(" (") else {
        return line.to_string();
    };
    let rest = &line[at + 2..];
    let digits = rest.chars().take_while(char::is_ascii_digit).count();
    if digits == 0 || !rest[digits..].starts_with(" steps") {
        return line.to_string();
    }
    format!("{} (#{}", &line[..at], &rest[digits..])
}

/// Blank the digit run that follows every occurrence of `key`.
fn blank_bracketed_digits(s: &str, key: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(at) = rest.find(key) {
        out.push_str(&rest[..at + key.len()]);
        rest = &rest[at + key.len()..];
        let end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        if end == 0 {
            continue;
        }
        out.push('#');
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// Blank from just after the earliest `key` up to (not including) `close`.
fn blank_bracketed(s: &str, keys: &[&str], close: char) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some((at, key)) = keys
        .iter()
        .filter_map(|k| rest.find(k).map(|i| (i, *k)))
        .min_by_key(|(i, _)| *i)
    {
        out.push_str(&rest[..at + key.len()]);
        rest = &rest[at + key.len()..];
        let end = rest.find(close).unwrap_or(rest.len());
        out.push('#');
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// [`blank_line`] over a whole text.
pub fn blank(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        out.push_str(&blank_line(line));
        out.push('\n');
    }
    out
}

/// [`blank`], plus the body of every `marker`-headed block whose name
/// [`is_narration`] — consequence 2.
///
/// Two markers exist because two shapes do: `dump_shape` and `snapshot_shape`
/// render `=== <name>` followed by the file's bytes, and `trace_shape` renders
/// `--- <name>` followed by the rendering. The header always survives, so a
/// block that vanished is still a difference.
///
/// A block name is the whole remainder of the header line for `=== ` (it is a
/// path, and `enterings/00/firings.jsonl` has to stay one name) and the first
/// token for `--- ` (whose headers carry a trailing note).
pub fn blank_blocks(text: &str, marker: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut narrating = false;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix(marker) {
            let name = if marker.starts_with("---") {
                rest.split_whitespace().next().unwrap_or("")
            } else {
                rest.trim()
            };
            narrating = is_narration(name);
            out.push(line.to_string());
            if narrating {
                out.push(NARRATED.to_string());
            }
            continue;
        }
        if !narrating {
            out.push(blank_line(line));
        }
    }
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_firing_count_loses_its_value_and_keeps_its_key() {
        assert_eq!(
            blank_line("ENTER {x} kind=alive firings=18 facts=70"),
            "ENTER {x} kind=alive firings=# facts=70"
        );
        assert_eq!(
            blank_line(r#"{"event": "entering", "firings": 18, "kind": "alive"}"#),
            r#"{"event": "entering", "firings": #, "kind": "alive"}"#
        );
        assert_eq!(
            blank_line(r#"{"e": "enter", "n_firings": 240, "layer": 1}"#),
            r#"{"e": "enter", "n_firings": #, "layer": 1}"#
        );
        // A record that *lost* the field is still a difference — the whole
        // reason this blanks the value rather than dropping the key.
        assert_ne!(
            blank_line(r#"{"event": "entering", "kind": "alive"}"#),
            blank_line(r#"{"event": "entering", "firings": 18, "kind": "alive"}"#)
        );
    }

    #[test]
    fn an_event_lines_ordinal_is_blanked_and_nothing_elses_is() {
        assert_eq!(
            blank_line(r#"{"e": "enter", "n": 41, "layer": 1}"#),
            r#"{"e": "enter", "n": #, "layer": 1}"#
        );
        // Not an event line: whatever `n` means there, it is not the ordinal.
        let other = r#"{"relation": "r", "n": 41}"#;
        assert_eq!(blank_line(other), other);
    }

    #[test]
    fn the_trace_line_loses_its_steps_and_keeps_its_refutations() {
        assert_eq!(
            blank_line("wrote /o/trace.md (20 steps, 2 refuted)"),
            "wrote /o/trace.md (# steps, 2 refuted)"
        );
        // `refuted` is how many commitments died — T1, not narration.
        assert_ne!(
            blank_line("wrote /o/trace.md (20 steps, 2 refuted)"),
            blank_line("wrote /o/trace.md (20 steps, 3 refuted)")
        );
        // …and only on the line that says it wrote something.
        assert_eq!(
            blank_line("solved in 20 steps, 2 refuted"),
            "solved in 20 steps, 2 refuted"
        );
    }

    #[test]
    fn only_a_dying_forks_core_is_blanked() {
        assert_eq!(
            blank_line("ENTER {a b} kind=dead-post core=[(p x), (q y)] facts=9"),
            "ENTER {a b} kind=dead-post core=[#] facts=9"
        );
        // An alive entering's core is the answer.
        let alive = "ENTER {a b} kind=alive core=[(p x)] facts=9";
        assert_eq!(blank_line(alive), alive);
    }

    #[test]
    fn the_closed_list_is_the_only_thing_narrated() {
        assert!(is_narration("slice"));
        assert!(is_narration("enterings/00_c/firings.jsonl"));
        assert!(is_narration("markdown"));
        assert!(!is_narration("lattice"));
        assert!(!is_narration("lattice-full"));
        assert!(!is_narration("answer"));
        assert!(!is_narration("table"));
        assert!(!is_narration("summary.json"));
        assert!(!is_narration("00_timeline.jsonl"));
    }

    #[test]
    fn a_narrated_block_keeps_its_header() {
        let text = "=== summary.json\n{\"k\": 1}\n=== enterings/00/firings.jsonl\n{\"rule\": \"x\"}\n=== 00_timeline.jsonl\n{\"firings\": 7}";
        assert_eq!(
            blank_blocks(text, "=== "),
            "=== summary.json\n{\"k\": 1}\n=== enterings/00/firings.jsonl\n<narrated>\n=== 00_timeline.jsonl\n{\"firings\": #}"
        );
        // A narrated block that disappeared is still a difference, because
        // its header is not narration.
        assert_ne!(
            blank_blocks(text, "=== "),
            blank_blocks(
                "=== summary.json\n{\"k\": 1}\n=== 00_timeline.jsonl\n{\"firings\": 7}",
                "=== "
            )
        );
    }

    #[test]
    fn a_trace_shape_block_head_is_its_first_token() {
        let text = "--- markdown (engine)\nStep 1\n--- answer\nSolution\n--- round-trip ok";
        assert_eq!(
            blank_blocks(text, "--- "),
            "--- markdown (engine)\n<narrated>\n--- answer\nSolution\n--- round-trip ok"
        );
    }
}
