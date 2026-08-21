//! The dumpers' shared serialisers — `ein.py`'s
//! `inference/monotonic/_serialise.py`.
//!
//! Engine-agnostic projections of a fact, a firing or a whole KB into ein
//! source text or machine-parseable JSON, plus the `00_timeline.jsonl` writer
//! both file dumpers share.

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use ein_core::{FactId, Kb, ProvKind, Tag, Terms, Value};
use ein_infer::firing::Firing;
use ein_ir::{Ast, Node, NodeId, dump_canonical};

use super::json::{Json, dumps, dumps_indent_sorted};

/// Lower a fact argument to an IR node.
fn arg_to_node(scratch: &mut Ast, terms: &Terms, arg: Value) -> NodeId {
    match arg.tag() {
        Tag::Fact => fact_to_sform(scratch, terms, None, arg.as_fact().expect("tagged Fact")),
        Tag::Int => {
            let sym = scratch.intern(terms.int_text(arg.as_int().expect("tagged Int")));
            scratch.push(Node::Int(sym), None)
        }
        // `Atom(name=str(arg))` — every non-fact, non-int argument.
        Tag::Sym => {
            let sym = scratch.intern(terms.sym(arg.as_sym().expect("tagged Sym")));
            scratch.push(Node::Atom(sym), None)
        }
    }
}

/// A fact as `(rel arg0 arg1 … :source "…" / :rule "…" / :hypothesis N)`.
///
/// A nested fact argument is lowered recursively **without** its keywords, so
/// it reads as a bare `(rel args…)` inside the outer form — which is why `kb`
/// is an `Option` rather than a plain reference: `None` means "no keywords".
fn fact_to_sform(scratch: &mut Ast, terms: &Terms, kb: Option<&Kb>, f: FactId) -> NodeId {
    let (rel, args) = terms.fact(f);
    let args = args.to_vec();
    let mut children: Vec<NodeId> = args
        .into_iter()
        .map(|a| arg_to_node(scratch, terms, a))
        .collect();
    if let Some(kb) = kb
        && let Some(prov) = kb.primary(f).map(|p| terms.provs.get(p))
    {
        let kw = |scratch: &mut Ast, key: &str, value: NodeId| {
            let name = scratch.intern(key);
            let key = scratch.push(Node::Keyword(name), None);
            scratch.push(Node::KwPair { key, value }, None)
        };
        match prov.kind {
            // Both guarded by truthiness in ein.py, so an empty string or a
            // missing name falls through to no keyword at all.
            ProvKind::Source => {
                if let Some(s) = prov.source.map(|s| terms.sym(s)).filter(|s| !s.is_empty()) {
                    let sym = scratch.intern(s);
                    let value = scratch.push(Node::Str(sym), None);
                    let pair = kw(scratch, "source", value);
                    children.push(pair);
                }
            }
            ProvKind::Rule => {
                if let Some(r) = prov.rule.map(|r| terms.sym(r)).filter(|r| !r.is_empty()) {
                    let sym = scratch.intern(r);
                    let value = scratch.push(Node::Str(sym), None);
                    let pair = kw(scratch, "rule", value);
                    children.push(pair);
                }
            }
            ProvKind::Hypothesis => {
                let sym = scratch.intern(&prov.branch.unwrap_or(0).to_string());
                let value = scratch.push(Node::Int(sym), None);
                let pair = kw(scratch, "hypothesis", value);
                children.push(pair);
            }
            ProvKind::Rejected => {}
        }
    }
    // The provenance *is* the origin, so the dump → reload round trip is
    // exact by construction rather than by patching.
    let name = scratch.intern(terms.sym(rel));
    let head = scratch.push(Node::Atom(name), None);
    scratch.sform(head, &children, None)
}

/// A KB as a **flat** sequence of ein forms.
///
/// The block wrappers are gone: each fact is a top-level form and where it
/// came from rides on its provenance. Un-annotated facts are emitted first
/// purely for readability; the round trip is order-independent.
pub fn kb_to_ein_text(kb: &Kb, terms: &Terms) -> String {
    let mut scratch = Ast::new();
    let (mut ont, mut rest): (Vec<NodeId>, Vec<NodeId>) = (Vec::new(), Vec::new());
    for f in kb.facts() {
        let node = fact_to_sform(&mut scratch, terms, Some(kb), f);
        // `f.is_given or f.is_derived` — given is a `:source`-kind with a
        // source, derived is any non-source kind.
        let prov = kb.primary(f).map(|p| terms.provs.get(p));
        let given = prov.is_some_and(|p| p.kind == ProvKind::Source && p.source.is_some());
        let derived = prov.is_some_and(|p| p.kind != ProvKind::Source);
        if given || derived {
            &mut rest
        } else {
            &mut ont
        }
        .push(node);
    }
    ont.extend(rest);
    dump_canonical(&scratch, &ont)
}

/// A firing for JSONL output — rule, activator, bindings, the primary derived
/// fact, and the premises.
pub fn firing_to_json(terms: &Terms, firing: &Firing) -> Json {
    let bindings: Vec<(String, Json)> = firing
        .bindings
        .iter()
        .map(|(k, v)| {
            let text = match v.tag() {
                // A `Fact` binding is dropped to a marker — chase it through
                // the premises instead.
                Tag::Fact => {
                    let (rel, _) = terms.fact(v.as_fact().expect("tagged Fact"));
                    format!("<fact:{}>", terms.sym(rel))
                }
                _ => terms.display(*v),
            };
            (terms.sym(*k).to_string(), Json::Str(text))
        })
        .collect();
    let fact_json = |f: FactId| {
        let (rel, args) = terms.fact(f);
        Json::obj(vec![
            ("relation", Json::str(terms.sym(rel))),
            (
                "args",
                Json::Array(args.iter().map(|a| Json::str(terms.display(*a))).collect()),
            ),
        ])
    };
    // The derived side shows the *primary* conclusion; a nested fact argument
    // renders as its own `{relation, args}` dict rather than as text.
    let (drel, dargs) = terms.fact(firing.derived[0]);
    let derived = Json::obj(vec![
        ("relation", Json::str(terms.sym(drel))),
        (
            "args",
            Json::Array(
                dargs
                    .iter()
                    .map(|a| match a.tag() {
                        Tag::Fact => fact_json(a.as_fact().expect("tagged Fact")),
                        _ => Json::str(terms.display(*a)),
                    })
                    .collect(),
            ),
        ),
    ]);
    Json::obj(vec![
        ("rule", Json::str(terms.sym(firing.rule))),
        (
            "activator",
            Json::Array(
                firing
                    .activator
                    .iter()
                    .map(|s| Json::str(terms.sym(*s)))
                    .collect(),
            ),
        ),
        ("bindings", Json::Object(bindings)),
        ("redundant", Json::Bool(firing.redundant)),
        ("derived", derived),
        (
            "premises",
            Json::Array(firing.premises.iter().map(|p| fact_json(*p)).collect()),
        ),
    ])
}

/// A fact as a recursive `{relation, args}` dict — nested facts nest, and
/// everything else stringifies.
pub fn fact_summary(terms: &Terms, f: FactId) -> Json {
    let (rel, args) = terms.fact(f);
    Json::obj(vec![
        ("relation", Json::str(terms.sym(rel))),
        (
            "args",
            Json::Array(
                args.iter()
                    .map(|a| match a.tag() {
                        Tag::Fact => fact_summary(terms, a.as_fact().expect("tagged Fact")),
                        _ => Json::str(terms.display(*a)),
                    })
                    .collect(),
            ),
        ),
    ])
}

/// The `00_timeline.jsonl` + `summary.json` plumbing both file dumpers share.
///
/// ein.py factors it as a mixin with one behavioural knob — the timeline
/// record's `json.dumps(default=)` serialiser. That knob does not survive the
/// port and does not need to: `default=` only fires for values Python's JSON
/// cannot encode, and every value built here is already encodable.
pub struct Timeline {
    pub out_dir: Option<PathBuf>,
    file: Option<File>,
    seq: u64,
    /// Seconds since the run started, for the timestamp fields — which are on
    /// the [normalisation list](../../../../plans/m1a_rust/design/01_parity_contract.md) §5,
    /// so what has to match is that they are *there* and well-shaped.
    started_at: std::time::Instant,
}

impl Timeline {
    /// Create the directory and open the timeline. `out_dir = None` skips
    /// every filesystem write while the hooks keep firing.
    pub fn new(out_dir: Option<&Path>) -> std::io::Result<Timeline> {
        let mut t = Timeline {
            out_dir: out_dir.map(Path::to_path_buf),
            file: None,
            seq: 0,
            started_at: std::time::Instant::now(),
        };
        if let Some(dir) = &t.out_dir {
            std::fs::create_dir_all(dir)?;
            std::fs::create_dir_all(dir.join("layers"))?;
            t.file = Some(File::create(dir.join("00_timeline.jsonl"))?);
        }
        Ok(t)
    }

    pub fn elapsed(&self) -> f64 {
        self.started_at.elapsed().as_secs_f64()
    }

    /// One timeline record: `seq`, `ts_ms`, `event`, then the caller's fields
    /// in the order given.
    pub fn emit(&mut self, event: &str, fields: Vec<(&str, Json)>) {
        let Some(file) = self.file.as_mut() else {
            return;
        };
        let mut rec = vec![
            ("seq".to_string(), Json::Int(self.seq as i64)),
            (
                "ts_ms".to_string(),
                Json::Float(round3(self.started_at.elapsed().as_secs_f64() * 1000.0)),
            ),
            ("event".to_string(), Json::str(event)),
        ];
        rec.extend(fields.into_iter().map(|(k, v)| (k.to_string(), v)));
        let _ = writeln!(file, "{}", dumps(&Json::Object(rec)));
        let _ = file.flush();
        self.seq += 1;
    }

    /// Write `summary.json` and close the timeline.
    pub fn summary(&mut self, verdict: &str, stats: Vec<(&str, Json)>) {
        let elapsed = round3(self.started_at.elapsed().as_secs_f64());
        if let Some(dir) = &self.out_dir {
            let doc = Json::obj(vec![
                ("verdict", Json::str(verdict)),
                ("elapsed_seconds", Json::Float(elapsed)),
                (
                    "stats",
                    Json::Object(stats.into_iter().map(|(k, v)| (k.to_string(), v)).collect()),
                ),
            ]);
            let _ = std::fs::write(dir.join("summary.json"), dumps_indent_sorted(&doc));
        }
        self.emit(
            "summary",
            vec![
                ("verdict", Json::str(verdict)),
                ("elapsed_seconds", Json::Float(elapsed)),
            ],
        );
        self.file = None;
    }

    /// Close the timeline without writing `summary.json` — the abort path, so
    /// the log is flushed when no final summary will land. Idempotent.
    pub fn close(&mut self) {
        self.file = None;
    }
}

/// `round(x, 3)` — Python's banker's rounding at three decimals.
fn round3(x: f64) -> f64 {
    let scaled = x * 1000.0;
    let r = scaled.round();
    // `f64::round` breaks ties away from zero; Python's `round` breaks them to
    // even. The difference only shows on an exact `.5`, which a clock reading
    // essentially never is — but "essentially never" is not "never".
    let r = if (scaled - scaled.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r - scaled.signum()
    } else {
        r
    };
    r / 1000.0
}
