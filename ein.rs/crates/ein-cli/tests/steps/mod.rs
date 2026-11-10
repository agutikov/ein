//! What `gate_steps.rs` and `nightly_steps.rs` share: reading a step list out
//! of `run_tests.sh` (the gate's, `--nightly`'s and `--dev-host`'s), and the
//! one normalisation both sides of each diff go through. Not a test target: cargo builds only `tests/*.rs`, and each of the
//! two files includes this with `mod steps;`.

/// The steps of `run_tests.sh` that a `banner "…"` line announces, in order.
///
/// `banner` is `step` for the per-commit gate, `nightly_step` for
/// `--nightly` and `dev_host_step` for `--dev-host` (M1f S1f.5.27). A line is
/// a banner when it *starts* with `banner "`, so the
/// one-line definition `nightly_step() { step "$@"; }` is not one: the parse
/// is by line, and that line starts with `nightly_step()`.
///
/// The command is the statement after the banner — one logical line, which a
/// trailing backslash may spread over several physical ones — with what a
/// shell needs and a workflow does not taken off: `if ! ` … `; then`,
/// `--manifest-path "${MANIFEST}"`, `-q`, the forwarded `${ARGS[@]…}`, and the
/// `"${SCRIPT_DIR}/…"` quoting of a path.
pub fn shell_steps(text: &str, banner: &str) -> Vec<String> {
    let open = format!("{banner} \"");
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.trim_start().starts_with(&open) {
            continue;
        }
        let mut j = i + 1;
        let mut cmd = String::new();
        while j < lines.len() {
            let piece = lines[j].trim();
            cmd.push_str(piece.strip_suffix('\\').unwrap_or(piece));
            if !piece.ends_with('\\') {
                break;
            }
            cmd.push(' ');
            j += 1;
        }
        let cmd = cmd
            .trim_start_matches("if ! ")
            .trim_end_matches("; then")
            .replace("--manifest-path \"${MANIFEST}\" ", "")
            .replace(" -q ", " ")
            .replace("${ARGS[@]+\"${ARGS[@]}\"}", "");
        // `"${SCRIPT_DIR}/utils/x.py"` is `utils/x.py` with the quoting a
        // shell needs and a workflow does not. Rewritten per token, so the
        // quotes that are *semantic* — `RUSTDOCFLAGS="-D warnings"` — survive
        // and are still compared.
        out.push(normalise(
            cmd.split_whitespace()
                .map(|t| {
                    t.strip_prefix("\"${SCRIPT_DIR}/")
                        .and_then(|t| t.strip_suffix('"'))
                        .unwrap_or(t)
                })
                .collect::<Vec<_>>()
                .join(" "),
        ));
    }
    out
}

/// A workflow step's `env: { K: "V", … }` as the shell's `K="V" … ` prefix.
pub fn env_prefix(line: &str) -> Option<String> {
    let body = line.trim().strip_prefix("env: { ")?.strip_suffix(" }")?;
    Some(
        body.split(", ")
            .filter_map(|kv| kv.split_once(": "))
            .map(|(k, v)| format!("{k}={v} "))
            .collect(),
    )
}

/// One space between words, nothing at the ends.
pub fn normalise(cmd: impl AsRef<str>) -> String {
    cmd.as_ref()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
