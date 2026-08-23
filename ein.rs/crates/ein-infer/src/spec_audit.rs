//! S1a.7.0 — the speculation audit: **does an entering's outcome depend on
//! the root writes that preceded it inside its own layer?**
//!
//! [design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §2 fans a
//! layer out from `R0 = Arc::clone(root_core)` — root as it stood when the
//! layer began — and then commits the results in candidate order, validating
//! each against the write set `W` that the commits before it produced. Three
//! cases, of which only the third costs anything: a candidate whose fork
//! *could* have consumed a `(not h)` that landed mid-layer has to have its
//! saturation continued with `W` as the delta before its result may stand.
//!
//! The rate at which case 3 fires is [Q-M1a.7](../../../../docs/history/m1a_rust/open_questions.md)'s
//! open half, and the phase's acceptance asks for "≤ a few percent". That
//! number is measurable **without a single thread**: run the sequential engine,
//! and beside every entering run the same entering against `R0`. Where the two
//! agree, the speculation would have been accepted as computed; where they
//! disagree, a continuation is what would have had to correct it.
//!
//! So this is the instrument that comes before the refactor. It costs one
//! extra `try_commitment_set` per entering, is compiled out without
//! `--features spec-audit`, and is inert without `$EIN_SPEC_AUDIT`.
//!
//! ### What `W` is, and why it is not assumed
//!
//! `W` here is **every fact root gained since the layer began**, computed as a
//! set difference rather than recorded at the one site that was expected to
//! produce it. The singleton `(not h)` writeback is the write
//! [design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §2 names;
//! an instrument that only counted *that* one could not report a write nobody
//! predicted, which is the failure mode a speculation scheme cannot afford.
//!
//! ### The control
//!
//! Case 1 — `W` still empty — is run through the same comparison even though
//! both arms fork the same root by construction. It cannot differ; a build in
//! which it does has a nondeterminism the audit would otherwise have blamed on
//! the writes. It is the cheapest control available and it is always on.
//!
//! One JSON-Lines record per entering to `$EIN_SPEC_AUDIT`, aggregated by
//! `utils/spec_audit.py`.

use ein_core::{FactId, Kb, Terms};
use ein_ir::Ast;
use rustc_hash::FxHashSet;
use std::io::Write;
use std::sync::{Mutex, OnceLock};

use crate::commitment::{CommitmentSetResult, try_commitment_set};
use crate::compile::SharedMemo;
use crate::events::{Events, sexpr};
use crate::saturator::Snapshot;

fn sink() -> &'static Mutex<Option<std::fs::File>> {
    static SINK: OnceLock<Mutex<Option<std::fs::File>>> = OnceLock::new();
    SINK.get_or_init(|| {
        Mutex::new(std::env::var_os("EIN_SPEC_AUDIT").and_then(|p| std::fs::File::create(p).ok()))
    })
}

/// Is the audit armed? Checked once per layer, not once per entering.
pub fn armed() -> bool {
    sink().lock().expect("no writer panicked").is_some()
}

/// One layer's speculative arm: `R0`, and the facts root had when it opened.
pub struct LayerAudit {
    /// Root as of layer start. Forking it is what a worker would have done.
    /// `try_commitment_set` writes nothing to its root argument (P1.21 R2), so
    /// this stays layer-start root for the whole layer.
    r0: Kb,
    /// `R0`'s fact set, for the `W` difference.
    r0_facts: FxHashSet<FactId>,
    layer: u32,
    index: usize,
    /// The speculative arm must not narrate: its firings are not the run's.
    off: Events,
}

impl LayerAudit {
    /// Open the layer. `None` when the audit is not armed, which is the only
    /// state a shipping build is ever in.
    pub fn start(root: &mut Kb, layer: u32) -> Option<LayerAudit> {
        if !armed() {
            return None;
        }
        let r0 = root.fork();
        let r0_facts: FxHashSet<FactId> = r0.facts().collect();
        Some(LayerAudit {
            r0,
            r0_facts,
            layer,
            index: 0,
            off: Events::off(),
        })
    }

    /// Run candidate `c` against `R0` and compare with what the sequential
    /// engine just got against the *current* root.
    #[allow(clippy::too_many_arguments)]
    pub fn check(
        &mut self,
        root: &Kb,
        terms: &mut Terms,
        ast: &Ast,
        memo: &SharedMemo,
        c: &[FactId],
        seq: &CommitmentSetResult,
        resume: Option<&Snapshot>,
    ) {
        let i = self.index;
        self.index += 1;

        // W — everything root gained since the layer opened, and the
        // hypotheses whose negation is among it.
        let seen = &self.r0_facts;
        let w: Vec<FactId> = root.facts().filter(|f| !seen.contains(f)).collect();
        let negated: FxHashSet<FactId> = w
            .iter()
            .filter_map(|&f| {
                let (rel, args) = terms.facts.get(f);
                if rel == terms.kernel.not && args.len() == 1 {
                    args[0].as_fact()
                } else {
                    None
                }
            })
            .collect();

        let case = if w.is_empty() {
            1
        } else if c.iter().any(|h| negated.contains(h)) {
            2
        } else {
            3
        };

        let spec = try_commitment_set(
            self.r0.sealed(),
            terms,
            ast,
            &mut self.off,
            memo,
            c,
            None,
            resume,
        );
        let spec = match spec {
            Ok(r) => r,
            Err(e) => {
                let mut out = String::new();
                out.push_str("{\"layer\":");
                out.push_str(&self.layer.to_string());
                out.push_str(",\"i\":");
                out.push_str(&i.to_string());
                out.push_str(",\"case\":");
                out.push_str(&case.to_string());
                out.push_str(",\"error\":");
                json_str(&e.to_string(), &mut out);
                out.push_str("}\n");
                emit(&out);
                return;
            }
        };

        // The two fact sets are from one process and one `Terms`, so `FactId`
        // is directly comparable — the reason this comparison is a set
        // difference where `fork_audit`'s, which crosses processes, has to
        // render every fact first.
        let seq_facts: FxHashSet<FactId> = seq.kb.facts().collect();
        let spec_facts: FxHashSet<FactId> = spec.kb.facts().collect();
        let seq_core: FxHashSet<FactId> = seq.unsat_core.iter().copied().collect();
        let spec_core: FxHashSet<FactId> = spec.unsat_core.iter().copied().collect();
        let same_kind = seq.kind == spec.kind;
        let same_core = seq_core == spec_core;
        let same_state = seq_facts == spec_facts;
        // A case-3 fork inherits `W` from root and the speculative one does
        // not, so *every* case-3 entering differs by at least `W` itself. The
        // number that decides whether the continuation has work to do is the
        // difference **past** `W`: a fact the sequential fork derived *from* a
        // mid-layer write.
        let w_set: FxHashSet<FactId> = w.iter().copied().collect();
        let derived_only_seq: Vec<FactId> = seq_facts
            .difference(&spec_facts)
            .copied()
            .filter(|f| !w_set.contains(f))
            .collect();
        let only_spec: Vec<FactId> = spec_facts.difference(&seq_facts).copied().collect();

        let mut out = String::with_capacity(512);
        out.push_str("{\"layer\":");
        out.push_str(&self.layer.to_string());
        out.push_str(",\"i\":");
        out.push_str(&i.to_string());
        out.push_str(",\"case\":");
        out.push_str(&case.to_string());
        out.push_str(",\"w\":");
        out.push_str(&w.len().to_string());
        out.push_str(",\"c\":");
        out.push_str(&c.len().to_string());
        out.push_str(",\"kind\":");
        json_str(seq.kind.as_str(), &mut out);
        out.push_str(",\"spec_kind\":");
        json_str(spec.kind.as_str(), &mut out);
        out.push_str(",\"n_firings\":");
        out.push_str(&seq.firings.len().to_string());
        out.push_str(",\"spec_n_firings\":");
        out.push_str(&spec.firings.len().to_string());
        out.push_str(",\"n_facts\":");
        out.push_str(&seq_facts.len().to_string());
        out.push_str(",\"spec_n_facts\":");
        out.push_str(&spec_facts.len().to_string());
        out.push_str(",\"same_kind\":");
        out.push_str(if same_kind { "true" } else { "false" });
        out.push_str(",\"same_core\":");
        out.push_str(if same_core { "true" } else { "false" });
        out.push_str(",\"same_state\":");
        out.push_str(if same_state { "true" } else { "false" });
        out.push_str(",\"n_derived_only_seq\":");
        out.push_str(&derived_only_seq.len().to_string());
        out.push_str(",\"n_only_spec\":");
        out.push_str(&only_spec.len().to_string());
        out.push_str(",\"same\":");
        out.push_str(if same_kind && same_core && same_state {
            "true"
        } else {
            "false"
        });
        if !(same_kind && same_core && same_state) {
            // Capped: a divergence is read one example at a time, and an
            // unbounded dump of a 500-fact fork helps nobody.
            out.push_str(",\"only_seq\":");
            json_facts(terms, seq_facts.difference(&spec_facts).copied(), &mut out);
            out.push_str(",\"derived_only_seq\":");
            json_facts(terms, derived_only_seq.iter().copied(), &mut out);
            out.push_str(",\"only_spec\":");
            json_facts(terms, only_spec.iter().copied(), &mut out);
            out.push_str(",\"commitment\":");
            json_facts(terms, c.iter().copied(), &mut out);
            out.push_str(",\"w_facts\":");
            json_facts(terms, w.iter().copied(), &mut out);
        }
        out.push_str("}\n");
        emit(&out);
    }
}

const CAP: usize = 12;

fn json_facts(terms: &Terms, ids: impl Iterator<Item = FactId>, out: &mut String) {
    let mut rendered: Vec<String> = ids.map(|f| sexpr(terms, f)).collect();
    rendered.sort();
    let n = rendered.len();
    rendered.truncate(CAP);
    out.push('[');
    for (i, s) in rendered.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        json_str(s, out);
    }
    if n > CAP {
        out.push(',');
        json_str(&format!("… and {} more", n - CAP), out);
    }
    out.push(']');
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

fn emit(line: &str) {
    let mut guard = sink().lock().expect("no writer panicked");
    if let Some(file) = guard.as_mut() {
        let _ = file.write_all(line.as_bytes());
        let _ = file.flush();
    }
}
