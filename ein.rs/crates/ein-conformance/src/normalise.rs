//! The normalisation list — `plans/m1a_rust/design/01_parity_contract.md` §5.
//!
//! Some outputs cannot be identical between two implementations and must not
//! be pretended to be. Each is dealt with here, and **the list is closed**:
//! adding to it requires an entry in `plans/m1a_rust/open_questions.md`.
//!
//! The rule for a wall-clock number is "matched by field presence and format,
//! values elided". *Format* here means the field's total width and its decimal
//! places, both of which stay comparable — a column that drifts by one
//! character is a real T3 difference (Q-M1a.15) even when the value in it is
//! not. *Value* means everything else, including how many digits it took to
//! write: `10.23` and `9.87` in a `{:8.2f}` field both become `    #.##`, so
//! two honest runs of the same engine agree.
//!
//! Every rule below names the site that produces the text it masks. A rule
//! with no producing site is a rule nobody can check.

/// Mask every ASCII digit in `s`, leaving `.`/`,`/`-` in place.
fn mask(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_digit() { '#' } else { c })
        .collect()
}

/// Is `line` inside the `--timing` table? The table's numeric column carries
/// no unit (its header does), so it needs block context rather than a suffix.
///
/// Produced by `cli/solve.py::_print_timing`, opened by the literal
/// `timing (ms)` and terminated by the end of output.
fn timing_header(line: &str) -> bool {
    line.trim_end() == "timing (ms)"
}

/// Rewrite a right-aligned numeric field to a canonical, value-free form of
/// the same width.
///
/// Masking digit-for-digit is not enough for a padded field: `{:8.2f}` prints
/// `   10.23` and `    9.87`, whose *digit counts* differ, so `##.##` and
/// `#.##` land at different offsets and a legitimate parity pass reads as a
/// diff. The format — total width and decimal places — is what has to survive,
/// so the whole field (its padding included) becomes `#.##`, right-aligned in
/// the original width. A different width or a different precision still shows
/// up; a different *value* no longer does.
fn canon_field(field: &str) -> String {
    let width = field.chars().count();
    let digits = field.split_once('.').map_or(0, |(_, frac)| {
        frac.chars().take_while(char::is_ascii_digit).count()
    });
    let core = if digits > 0 {
        format!("#.{}", "#".repeat(digits))
    } else {
        "#".to_string()
    };
    let pad = width.saturating_sub(core.chars().count());
    format!("{}{core}", " ".repeat(pad))
}

/// Canonicalise the numeric field that immediately precedes ` ms`.
///
/// Produced by `cli/saturate.py` (`parse:` / `kb load:` / `compile:` /
/// `saturate:`) and `cli/solve.py::_print_stats` (`wall`).
fn mask_ms(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let b: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < b.len() {
        // Look ahead for " ms" and walk back over the padded number before it.
        if b[i] == ' '
            && b.get(i + 1) == Some(&'m')
            && b.get(i + 2) == Some(&'s')
            && b.get(i + 3).is_none_or(|c| !c.is_ascii_alphanumeric())
        {
            let keep = out.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
            if keep.len() < out.len() {
                // …and over its leading padding, which is part of the field.
                let field_start = keep.trim_end_matches(' ');
                let canon = canon_field(&out[field_start.len()..]);
                out.truncate(field_start.len());
                out.push_str(&canon);
            }
        }
        out.push(b[i]);
        i += 1;
    }
    out
}

/// Elide a wall-clock value in a JSON object: `"ts_ms": 1.124` → `"ts_ms": #`.
///
/// Collapsed to a single token rather than masked digit-for-digit, because
/// `json.dumps(round(x, 3))` drops trailing zeros — so `1.12` and `1.124` are
/// the *same* format printed at different values, and masking would report
/// `#.##` vs `#.###`. For a JSON number the whole format is "a number".
///
/// Produced by `inference/monotonic/_serialise.py` — `ts_ms` in
/// `00_timeline.jsonl`, `elapsed_seconds` in `summary.json`.
fn mask_json_time(line: &str) -> String {
    const KEYS: [&str; 2] = ["\"ts_ms\": ", "\"elapsed_seconds\": "];
    let mut out = line.to_string();
    for key in KEYS {
        while let Some(at) = out.find(key) {
            let start = at + key.len();
            let n = out[start..]
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                .count();
            if n == 0 {
                break;
            }
            let masked = "#".to_string();
            // Replace the key too, so the loop cannot find it again.
            out = format!(
                "{}{}{masked}{}",
                &out[..at],
                key.replace('"', "\u{1}"),
                &out[start + n..]
            );
        }
        out = out.replace('\u{1}', "\"");
    }
    out
}

/// Mask the elapsed marker `(   12s)` that ends every `--verbose` line.
///
/// Produced by `inference/monotonic/state_dump.py::ProgressDumper._el`.
fn mask_elapsed(line: &str) -> String {
    let Some(open) = line.rfind('(') else {
        return line.to_string();
    };
    let tail = &line[open..];
    if !tail.ends_with("s)") {
        return line.to_string();
    }
    let inner = &tail[1..tail.len() - 2];
    if inner.is_empty() || !inner.chars().all(|c| c.is_ascii_digit() || c == ' ') {
        return line.to_string();
    }
    format!("{}({}s)", &line[..open], mask(inner))
}

/// Canonicalise the **first** numeric field on a line — used inside the
/// `--timing` table, whose columns are
/// `  <label>  <duration:9.2f>    (<deterministic counts>)`.
///
/// Only the first, deliberately: the parenthesised tail carries form counts,
/// fact counts, entering counts — every one of them a real observable that a
/// blanket mask would throw away.
fn canon_first_field(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let Some(start) = chars.iter().position(char::is_ascii_digit) else {
        return line.to_string();
    };
    let mut end = start;
    while end < chars.len() && (chars[end].is_ascii_digit() || chars[end] == '.') {
        end += 1;
    }
    let mut field_start = start;
    while field_start > 0 && chars[field_start - 1] == ' ' {
        field_start -= 1;
    }
    let head: String = chars[..field_start].iter().collect();
    let field: String = chars[field_start..end].iter().collect();
    let tail: String = chars[end..].iter().collect();
    format!("{head}{}{tail}", canon_field(&field))
}

/// Mask every decimal number on a line — used only for a `state_digest`.
fn mask_decimals(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut num = String::new();
    for c in line.chars() {
        if c.is_ascii_digit() || (c == '.' && !num.is_empty()) {
            num.push(c);
        } else {
            if !num.is_empty() {
                out.push_str(&mask(&num));
                num.clear();
            }
            out.push(c);
        }
    }
    out.push_str(&mask(&num));
    out
}

/// Mask a `state_digest` value.
///
/// `canon.state_digest` is `hash(tuple)`, salted by `PYTHONHASHSEED`, so
/// ein.py is not even self-stable here (design/02 §8). Compared for shape.
fn mask_state_digest(line: &str) -> String {
    let Some(at) = line.find("state_digest") else {
        return line.to_string();
    };
    let (head, tail) = line.split_at(at);
    format!("{head}{}", mask_decimals(tail))
}

/// Apply the whole list to one captured stream.
///
/// `repo` is the absolute repo root; every occurrence becomes `{REPO}` so a
/// message that names a path is comparable across checkouts. `out_dir` is the
/// *side's own* artefact directory (`…/a`, `…/b`), which becomes `{OUT}` —
/// without that, a run that echoes where it wrote (`solve --trace` prints
/// `wrote <path>`) would differ on the one part of the path the harness chose
/// rather than the implementation.
pub fn normalise_run(text: &str, repo: &str, out_dir: &str) -> String {
    normalise(&text.replace(out_dir, "{OUT}"), repo)
}

pub fn normalise(text: &str, repo: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_timing = false;
    for line in text.lines() {
        // A `--verbose` root-saturation heartbeat fires only when root
        // saturation runs longer than a second, so its very presence is
        // timing-dependent. Elided, not masked.
        if line.contains("saturating root:") {
            continue;
        }
        if timing_header(line) {
            in_timing = true;
        }
        let mut l = line.replace(repo, "{REPO}");
        l = mask_ms(&l);
        l = mask_elapsed(&l);
        l = mask_json_time(&l);
        if line.contains("state_digest") {
            l = mask_state_digest(&l);
        }
        if in_timing && !timing_header(line) {
            l = canon_first_field(&l);
        }
        out.push_str(&l);
        out.push('\n');
    }
    out
}

/// `state_hash.txt` holds a `PYTHONHASHSEED`-salted digest. Compared for
/// shape — same length, same character class — never for value.
pub fn digest_shape(text: &str) -> String {
    let t = text.trim();
    let class = if t
        .strip_prefix('-')
        .unwrap_or(t)
        .chars()
        .all(|c| c.is_ascii_digit())
    {
        "dec"
    } else if t.chars().all(|c| c.is_ascii_hexdigit()) {
        "hex"
    } else {
        "other"
    };
    format!("<{class}:{}>", t.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ms_fields_lose_their_value_and_keep_their_width() {
        // The point: two runs whose durations have different digit counts
        // must normalise to the *same* string. Masking digit-for-digit does
        // not, because the padding moves.
        let a = mask_ms("kb load:     10.23 ms");
        let b = mask_ms("kb load:      9.87 ms");
        assert_eq!(a, b);
        assert_eq!(a, "kb load:      #.## ms");
        assert_eq!(a.len(), "kb load:     10.23 ms".len());
        // A different precision is a format change and still shows up.
        assert_ne!(mask_ms("  wall  12.3 ms"), mask_ms("  wall  12.34 ms"));
    }

    #[test]
    fn json_time_fields_are_masked() {
        // `round(x, 3)` drops trailing zeros, so the *decimal count* is
        // value-dependent: both of these must land on the same string.
        assert_eq!(
            mask_json_time("{\"seq\": 0, \"ts_ms\": 1.124, \"event\": \"root\"}"),
            "{\"seq\": 0, \"ts_ms\": #, \"event\": \"root\"}"
        );
        assert_eq!(
            mask_json_time("{\"seq\": 0, \"ts_ms\": 1.12, \"event\": \"root\"}"),
            "{\"seq\": 0, \"ts_ms\": #, \"event\": \"root\"}"
        );
        assert_eq!(
            mask_json_time("\"elapsed_seconds\": 12.5"),
            "\"elapsed_seconds\": #"
        );
        // `seq` is a counter, not a clock — untouched.
        assert_eq!(mask_json_time("{\"seq\": 41}"), "{\"seq\": 41}");
    }

    #[test]
    fn the_runs_own_output_dir_is_rewritten() {
        // `solve --trace` echoes where it wrote; the harness chose that path,
        // so the two sides differ on `/a` vs `/b` for no engine reason.
        assert_eq!(
            normalise_run("wrote /o/a/trace.md (3 steps)\n", "/repo", "/o/a"),
            "wrote {OUT}/trace.md (3 steps)\n"
        );
    }

    #[test]
    fn a_count_that_is_not_a_duration_survives() {
        assert_eq!(
            mask_ms("saturate: 0.09 ms  (3 firings)"),
            "saturate: #.## ms  (3 firings)"
        );
        assert_eq!(
            mask_ms("  enterings        11 (alive=9)"),
            "  enterings        11 (alive=9)"
        );
    }

    #[test]
    fn elapsed_markers_are_masked() {
        assert_eq!(
            mask_elapsed("  layer 1: alive=5 root_facts=42  (   3s)"),
            "  layer 1: alive=5 root_facts=42  (   #s)"
        );
        // Not an elapsed marker: a parenthesised count at end of line.
        assert_eq!(
            mask_elapsed("root saturated: 42 facts"),
            "root saturated: 42 facts"
        );
    }

    #[test]
    fn the_timing_table_masks_its_unitless_column() {
        let a = normalise(
            "timing (ms)\n  parse                  23.46    (20 forms)\n",
            "/r",
        );
        let b = normalise(
            "timing (ms)\n  parse                   9.87    (20 forms)\n",
            "/r",
        );
        assert_eq!(a, b);
        assert_eq!(
            a,
            "timing (ms)\n  parse                   #.##    (20 forms)\n"
        );
    }

    #[test]
    fn hyp_stats_percentages_are_not_masked() {
        // Derived from counts, so deterministic — comparing them is the point.
        let out = normalise("  co-located  120  ( 42.9%)\n", "/repo");
        assert_eq!(out, "  co-located  120  ( 42.9%)\n");
    }

    #[test]
    fn the_repo_root_is_rewritten() {
        assert_eq!(
            normalise("read /home/u/ein/examples/x.ein\n", "/home/u/ein"),
            "read {REPO}/examples/x.ein\n"
        );
    }

    #[test]
    fn the_verbose_heartbeat_is_elided() {
        let out = normalise("a\n  saturating root: 900 firings  (   1s)\nb\n", "/r");
        assert_eq!(out, "a\nb\n");
    }

    #[test]
    fn digest_shapes_ignore_the_value() {
        assert_eq!(
            digest_shape("1234567890123456789"),
            digest_shape("9876543210987654321")
        );
        assert_ne!(digest_shape("123"), digest_shape("1234"));
    }
}
