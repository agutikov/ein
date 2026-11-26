//! **Root as the kernel builds it, and who wrote H**: a program loaded as
//! `ein solve` loads it, root saturated once under the call's bounds, and
//! the hypothesis set's generator run at root's fixpoint, as `ein show hyps`
//! runs it (`ein_infer::structure::hypotheses_with`). What comes back is the
//! **writer of H** the kernel reports, H's size, whether root is dead and
//! whether the static check passes; and root's own narration, every firing
//! and every obligation owed at root, each with its rule as the program
//! names it, its activator and its premises, read off the `--events`
//! protocol's `fire` and `owe` lines as values.
//!
//! No question is asked and no node above root is read. It is what the
//! contract's check reads a selection's activity and a program's mechanism
//! off.

use std::time::Instant;

use ein_infer::events::{Buffer, Events, Level};
use serde::{Deserialize, Serialize};

use super::feedback::{Finding, finding};
use super::program::{self, Loaded};
use super::{Bounds, Source};

/// **Root of one program, and H.**
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Root {
    /// The writer of H, as `ein show hyps` names it: `obligations`,
    /// `hypothesis-rules`, `union`, `no-search`, `blind`, `stuck` or
    /// `declined`; `null` where no root was built.
    pub writer: Option<String>,
    /// How many candidates H holds.
    pub candidates: usize,
    /// Root's own fixpoint holds a contradiction.
    pub root_dead: bool,
    /// The static check's verdict over root and this H.
    pub monotone: bool,
    /// Every firing at root, in the order the narration gives it.
    pub fired: Vec<Fired>,
    /// Every obligation instance root owes, in the narration's order.
    pub owed: Vec<Fired>,
    /// The program was refused at the read, the parse or the load: the
    /// refusal's findings, the kernel's diagnostics. Empty where it loaded.
    pub refused: Vec<Finding>,
    /// A bound stopped root's saturation or the generator: the kernel's
    /// word for it. `null` where nothing stopped it.
    pub stopped: Option<String>,
}

/// One `fire` or `owe` line of root's narration.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fired {
    /// The rule, as the program names it: its short name where the file
    /// lists it, its full name otherwise.
    pub rule: String,
    /// The activator's values, `[]` for a rule of no parameter.
    pub activator: Vec<String>,
    /// The facts the firing matched, each as its canonical s-expression; `[]`
    /// for an obligation owed.
    pub premises: Vec<String>,
}

/// **Build root and H of `source`**, as `ein show hyps` builds them, under
/// `bounds`: the clock from the moment of the call, and the cap on one
/// saturation's firings. A refused program is its findings.
pub fn root(source: Source<'_>, bounds: Bounds) -> Root {
    let t0 = Instant::now();
    let loaded: Loaded = match source {
        Source::File(path) => program::load_file(path, bounds, t0),
        Source::Text {
            text,
            name,
            base_dir,
        } => program::load_text(text, name, base_dir, bounds, t0),
    };
    let program = match loaded.program {
        Ok(p) => p,
        Err(refusal) => {
            return Root {
                refused: refusal.findings.iter().map(finding).collect(),
                stopped: refusal.findings.is_empty().then(|| refusal.message.clone()),
                ..Root::default()
            };
        }
    };
    let mut copy = program.clone();
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Verbose);
    let bound = ein_infer::saturator::Bound {
        deadline: bounds.clock.map(|c| t0 + c),
        max_steps: bounds.max_steps,
    };
    let built = ein_infer::saturator::bounded(bound, || {
        ein_infer::structure::hypotheses_with(&mut copy.kb, &mut copy.terms, &copy.ast, &mut events)
    });
    drop(events);
    let (fired, owed) = narration(&buffer.to_string_lossy());
    match built {
        Ok(h) => Root {
            writer: Some(h.rung.as_str().to_string()),
            candidates: h.candidates.len(),
            root_dead: h.root_dead,
            monotone: h.monotone,
            fired,
            owed,
            refused: Vec::new(),
            stopped: None,
        },
        Err(e) => match e.kind() {
            Some(ein_core::Kind::Deadline) | None => Root {
                fired,
                owed,
                stopped: Some(e.to_string()),
                ..Root::default()
            },
            Some(kind) => Root {
                fired,
                owed,
                refused: vec![finding(&ein_core::Diagnostic::error(kind, e.to_string()))],
                ..Root::default()
            },
        },
    }
}

/// The `fire` and `owe` lines of a narration, as values.
fn narration(text: &str) -> (Vec<Fired>, Vec<Fired>) {
    let mut fired = Vec::new();
    let mut owed = Vec::new();
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        let strings = |key: &str| -> Vec<String> {
            v.get(key)
                .and_then(|a| a.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|s| s.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default()
        };
        let one = Fired {
            rule: v
                .get("rule")
                .and_then(|r| r.as_str())
                .unwrap_or("")
                .to_string(),
            activator: strings("activator"),
            premises: strings("premises"),
        };
        match v.get("e").and_then(|e| e.as_str()) {
            Some("fire") => fired.push(one),
            Some("owe") => owed.push(one),
            _ => {}
        }
    }
    (fired, owed)
}
