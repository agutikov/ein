//! Per-entering audit records — the T1a.6.9.2 verification instrument.
//!
//! Compiled only under `fork-delta`, because it exists to check one thing:
//! that resuming the root's saturation in a fork
//! ([`crate::saturator::Snapshot`]) reaches the *same fork* by a shorter
//! narration. Its output is deliberately **not** a firing list — the firing
//! list is what the change is expected to move — but the artefacts
//! S1a.6.9 § What is *not* at risk claims are invariant:
//!
//! - the fork's fact set at quiescence, fact by fact (`state_key`);
//! - every recorded justification of every fact, primary and alternatives,
//!   rendered so two runs compare without sharing an arena;
//! - the entering's `kind` and its unsat core.
//!
//! One JSON-Lines record per entering to `$EIN_FORK_AUDIT`, in entering
//! order. Both arms of the diff are the same binary — `EIN_FORK_DELTA=1`
//! throws the switch — so a difference in this file is a difference the
//! resumed saturator made and nothing else.
//!
//! Facts and provenance go out as canonical s-expressions, for the reason
//! [`crate::events`] gives: nothing in the stream may depend on either run's
//! interning.

use ein_core::{FactId, NafArg, NafRef, ProvId, Terms};
use std::io::Write;
use std::sync::{Mutex, OnceLock};

use crate::commitment::CommitmentSetResult;
use crate::events::{sexpr, sexpr_value};

fn sink() -> &'static Mutex<Option<std::fs::File>> {
    static SINK: OnceLock<Mutex<Option<std::fs::File>>> = OnceLock::new();
    SINK.get_or_init(|| {
        Mutex::new(
            std::env::var_os("EIN_FORK_AUDIT")
                .and_then(|p| std::fs::File::create(p).ok()),
        )
    })
}

/// One derivation, rendered. `kind` and `rule` plus the premises in plan-step
/// order and the negative premises in query order — the whole record except
/// the display bindings, which an *alternative* deliberately leaves empty.
fn prov_repr(terms: &Terms, p: ProvId) -> String {
    let r = terms.provs.get(p);
    let mut out = String::from(r.kind.as_str());
    out.push('|');
    if let Some(rule) = r.rule {
        out.push_str(terms.sym(rule));
    }
    out.push('|');
    if let Some(src) = r.source {
        out.push_str(terms.sym(src));
    }
    out.push('|');
    for (i, &f) in r.premises.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&sexpr(terms, f));
    }
    out.push('|');
    for (i, n) in r.absent.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        naf_repr(terms, n, &mut out);
    }
    out
}

fn naf_repr(terms: &Terms, n: &NafRef, out: &mut String) {
    out.push('(');
    out.push_str(terms.sym(n.rel));
    for a in n.args.iter() {
        out.push(' ');
        match a {
            NafArg::Free => out.push('_'),
            NafArg::Value(v) => out.push_str(&sexpr_value(terms, *v)),
            NafArg::Nested { rel, args } => naf_repr(
                terms,
                &NafRef {
                    rel: *rel,
                    args: args.clone(),
                },
                out,
            ),
        }
    }
    out.push(')');
}

fn json_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn json_list(items: impl Iterator<Item = String>, out: &mut String) {
    out.push('[');
    for (i, s) in items.enumerate() {
        if i > 0 {
            out.push(',');
        }
        json_str(&s, out);
    }
    out.push(']');
}

/// Append one entering's record. A no-op when `$EIN_FORK_AUDIT` is unset.
pub fn record(terms: &Terms, r: &CommitmentSetResult) {
    let mut guard = sink().lock().expect("no writer panicked");
    let Some(file) = guard.as_mut() else {
        return;
    };
    // Sorted by the **rendered** s-expression, not by `FactId`.
    // [`crate::canon::state_key`] sorts by id because within one run any total
    // order gives exact set equality; across two *processes* the intern order
    // is not the same order, and a record that sorts by it reports a
    // difference where there is only a different arena.
    let mut facts: Vec<(String, FactId)> =
        r.kb.facts().map(|f| (sexpr(terms, f), f)).collect();
    facts.sort_unstable();
    let mut commitment: Vec<String> = r.commitment.iter().map(|&f| sexpr(terms, f)).collect();
    commitment.sort();
    let mut core: Vec<String> = r.unsat_core.iter().map(|&f| sexpr(terms, f)).collect();
    core.sort();

    let mut out = String::with_capacity(4096);
    out.push_str("{\"commitment\":");
    json_list(commitment.into_iter(), &mut out);
    out.push_str(",\"kind\":");
    json_str(r.kind.as_str(), &mut out);
    out.push_str(",\"n_facts\":");
    out.push_str(&facts.len().to_string());
    out.push_str(",\"core\":");
    json_list(core.into_iter(), &mut out);
    out.push_str(",\"state\":");
    json_list(facts.iter().map(|(t, _)| t.clone()), &mut out);
    // The AND/OR graph: primary first, then the alternatives in stored order,
    // which `record_justification` keeps sorted by premise count.
    out.push_str(",\"just\":[");
    for (i, (text, f)) in facts.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let js = r.kb.justifications(*f);
        out.push('[');
        json_str(text, &mut out);
        out.push(',');
        json_list(js.into_iter().map(|p| prov_repr(terms, p)), &mut out);
        out.push(']');
    }
    out.push_str("]}\n");
    let _ = file.write_all(out.as_bytes());
    let _ = file.flush();
}
