//! `help-shape` — the argument surface as one comparable text.
//!
//! What [Q-M1a.13](../../../../plans/m1a_rust/open_questions.md#q-m1a13--argparse-surface-parity)
//! owes. `--help` *layout* is on the normalisation list, so a byte diff of it
//! is gone — and a byte diff was the only thing checking that ein.rs had not
//! silently lost an option. It is replaced, not dropped: both parsers are
//! rendered as `{command → {option → short, metavar, arity, default, choices,
//! group, help}}` and the texts are diffed.
//!
//! Stronger than the byte diff where it matters: a renamed short key or a
//! changed default fails on its own line instead of somewhere inside an
//! 89-line blob. Blind where the resolution gave it up: wrapping, indentation,
//! headings, and the order options appear in — which is why the options are
//! sorted here rather than emitted in declaration order.
//!
//! `utils/ir_oracle.py`'s `help-shape` op was the other half until
//! S1a.10.4 removed it with the second engine, and it walked `argparse`'s
//! parser objects rather than re-parsing its formatted output: the parser
//! *is* the structure, and scraping it back out of the text would only
//! re-import the layout this stage exempts.

use clap::Command;

/// Every parser, as one text. The `saturate` entry is its **own** parser, not
/// the bare one registered for the help listing.
pub fn help_shape() -> String {
    let mut out = String::new();
    render(&mut out, "ein", &crate::cmdline::command());
    render(
        &mut out,
        "ein saturate",
        &crate::cmdline::saturate_command(),
    );
    out
}

fn render(out: &mut String, path: &str, cmd: &Command) {
    let about = cmd.get_about().map_or(String::new(), |a| a.to_string());
    out.push_str(&format!("COMMAND {path}\n"));
    out.push_str(&format!("  ABOUT {about}\n"));

    let mut rows: Vec<String> = Vec::new();
    for arg in cmd.get_arguments() {
        if arg.get_id() == "help" || arg.get_id() == "version" {
            continue;
        }
        // The bare `saturate` placeholder's catch-all is not an option.
        if arg.is_hide_set() {
            continue;
        }
        let id = arg.get_id().as_str();
        let group = cmd
            .get_groups()
            .find(|g| g.get_args().any(|a| a.as_str() == id))
            .map(|g| g.get_id().as_str().to_string())
            .unwrap_or_else(|| "-".to_string());
        let arity = match arg.get_num_args() {
            Some(r) => r.min_values(),
            None if arg.get_action().takes_values() => 1,
            None => 0,
        };
        let metavar = if arity == 0 {
            "-".to_string()
        } else {
            let name = arg
                .get_value_names()
                .and_then(|n| n.first())
                .map(|n| n.as_str().to_string())
                .unwrap_or_else(|| id.to_string());
            // `clap` reports the id when no value name was set, which is
            // `argparse`'s `dest.upper()` case; an *explicit* one travels
            // verbatim, so `FILE.jsonl` stays `FILE.jsonl`.
            if name == id {
                name.to_uppercase().replace('-', "_")
            } else {
                name
            }
        };
        let default = if arity == 0 {
            "False".to_string()
        } else {
            match arg.get_default_values().first() {
                Some(v) => ein_core::pyrepr::repr_str(&v.to_string_lossy()),
                None => "None".to_string(),
            }
        };
        let choices: String = {
            let vs: Vec<String> = arg
                .get_possible_values()
                .iter()
                .map(|p| p.get_name().to_string())
                .collect();
            if vs.is_empty() {
                "-".to_string()
            } else {
                vs.join("|")
            }
        };
        let help = arg
            .get_help()
            .map_or(String::new(), |h| squeeze(&h.to_string()));
        match arg.get_long() {
            None => rows.push(format!(
                "  POSITIONAL {id} required={} help={help}",
                py_bool(arg.is_required_set())
            )),
            Some(long) => rows.push(format!(
                "  OPTION --{long} -{} metavar={metavar} arity={arity} \
                 default={default} choices={choices} group={group} \
                 required={} help={help}",
                arg.get_short().map_or("-".to_string(), |c| c.to_string()),
                py_bool(arg.is_required_set()),
            )),
        }
    }
    rows.sort();
    for r in rows {
        out.push_str(&r);
        out.push('\n');
    }

    let mut subs: Vec<&Command> = cmd.get_subcommands().collect();
    subs.sort_by_key(|c| c.get_name());
    for sub in subs {
        // `saturate` is registered bare so it appears in `ein --help`; its
        // real parser is rendered separately, under its own `prog`.
        if sub.get_name() == "saturate" && path == "ein" {
            out.push_str("  SUBCOMMAND saturate (delegated)\n");
            continue;
        }
        render(out, &format!("{path} {}", sub.get_name()), sub);
    }
}

fn py_bool(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

/// Help strings are authored as wrapped source literals on both sides; the
/// content is the joined text, so runs of whitespace collapse before compare.
fn squeeze(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
