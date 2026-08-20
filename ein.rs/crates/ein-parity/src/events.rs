//! The event stream, compared for the derivation rather than the narration.
//!
//! Two [`--events`](../../../../docs/kernel/inference/events.md) logs in,
//! a list of differences out. The tier that used to call it (T2, "the two
//! engines took the same steps") was retired with the second engine at
//! [S1a.10.3](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md);
//! what calls it now is `ein-infer/tests/event_cut_control.rs`, which mutates
//! one real stream and checks the cut still reports the mutation. That test is
//! not decoration: **a relaxation nothing exercises is a hole rather than a
//! decision**, and it is the only thing standing between §2's measured cut and
//! a comparison that quietly stopped comparing.
//!
//! # 1. Why the stream cannot simply be filtered
//!
//! [T2](../../../../plans/m1a_rust/design/01_parity_contract.md#t2--event-trace-parity)
//! was the tier that pinned *the algorithm*, and since
//! [S1a.6.9](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
//! the two engines ran a deliberately different one at the fork boundary:
//! ein.rs resumes root's saturation, ein.py re-derived it. 97 of T2's 240
//! cells reported that as a difference, and eliding the firing traffic
//! wholesale would have stopped catching the thing the tier existed for — a
//! port that silently stopped deriving something.
//!
//! What survives the boundary is *what each fork derives*, so that is what is
//! compared. The stream is split into **segments** — root's saturation, then
//! one per entering — at every `enter`, and at the **first hypgen event**,
//! which is what closes root's. That second boundary is not decoration: under
//! `--lookahead` the first entering is a probe that usually dies, so without
//! it root's whole derivation would share a segment with a `dead-post` fork
//! and be skipped along with it. Found by the negative control, on
//! `examples/branching/05_mini_zebra.ein :: solve -L`. Within a segment:
//!
//! | | compared |
//! |---|---|
//! | the **spine** (`run`, `load`, `hyp`, `hypskip`, `enter`, `nogood`, `writeback`, `warn`, `verdict`) | in order, exactly — minus `enter`'s `n_firings` and a `dead-post` `enter`'s `core` |
//! | the **derivation** — every `fire` with `redundant = false`, and every `mirror` | the **multiset of facts derived**, and the **set of rules** that derived them |
//! | the scheduling traffic — `enqueue`, `park`, `admit`, `retire`, `quiesce`, `alt`, `compile` — and every redundant `fire` | nothing; counted per side and reported |
//!
//! A **`dead-post`** segment's derivation is not compared at all.
//! `enable_fail_fast_fork` stops a dying fork at the firing that kills it, so
//! its firing list is a *prefix* by construction and two firing orders leave
//! two different prefixes. Its `kind`, its position and the fact that it died
//! are still compared exactly, and with fail-fast off the prefixes agree —
//! which is what says this is the stopping point and not a different conflict
//! ([D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)).
//!
//! # 2. Why *that* cut, measured
//!
//! Six candidates were run over the 240 captured T2 cells before one was
//! written down. Each row is the same corpus, the same logs, a different
//! definition of "the derivation":
//!
//! | the derivation is … | cells agreeing |
//! |---|---:|
//! | the whole stream (the contract before this) | 142 / 240 |
//! | the ordered non-redundant firings | 142 / 240 |
//! | … also eliding `compile` | 213 / 240 |
//! | … as an ordered `(rule, premises, derived)` | 214 / 240 |
//! | … as a **multiset** of `(rule, premises, derived)`, per segment, `dead-post` excluded | 232 / 240 |
//! | **… as a multiset of derived facts + the set of rules, per segment, `dead-post` excluded** | **239 / 240** |
//!
//! 239 / 240 is [D2](../../../../plans/m1a_rust/divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)
//! and nothing else — the same standard T3 is held to. The row above it is
//! the 7 cells where a fork records a *different one of a fact's equally valid
//! derivations* first, which D3 measures at 267 529 facts corpus-wide and
//! argues cannot be designed away: matching a fresh pass's admission order
//! requires running a fresh pass, which is the thing removed. So the cut is
//! the strongest one that reaches the standard, not the first one that went
//! green.
//!
//! `compile` is on the elided list and was not on
//! [S1a.6.10](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)'s
//! predicted list: a `compile` is emitted on a plan-memo **miss**, and how
//! many times a rule misses depends on how many enqueue passes ran, so it is
//! downstream of the boundary like everything else here. The *distinct*
//! compiles — which rules, for which activator, with what shape — are
//! identical on both sides and are what the elided-count report exposes.

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Everything a fork's *scheduling* emits. Elided, counted, reported.
pub const SCHEDULING: [&str; 7] = [
    "enqueue", "park", "admit", "retire", "quiesce", "alt", "compile",
];

/// One stretch of the stream: root's saturation, one entering, or the tail.
pub struct Segment {
    /// The closing `enter`'s `kind`. `None` for root's saturation, for the
    /// tail after the last entering, and for a run that never forks — the
    /// three cases whose derivation is always compared, because none of them
    /// is a fail-fast prefix.
    pub kind: Option<String>,
    /// The lifecycle and search-layer events, in order, already
    /// [`comparable`].
    pub spine: Vec<Value>,
    /// What the productive firings derived, as a multiset — so a *dropped*
    /// firing whose fact another firing also derives still changes the count.
    pub facts: BTreeMap<String, usize>,
    /// Which rules derived it. `__symmetric__` is the native arg-swap mirror,
    /// which the engine reports as its own event and not as a `fire`.
    pub rules: BTreeSet<String>,
    /// How many productive firings the segment performed. Reported, not
    /// compared: it is a firing count, which is narration by rule 1.
    pub productive: usize,
}

/// A whole log, split.
pub struct Split {
    pub segments: Vec<Segment>,
    /// Elided events, by kind — what each side narrated and this comparison
    /// did not read. Reported so a run still says *how much*.
    pub elided: BTreeMap<String, usize>,
}

impl Split {
    /// `enqueue 452, fire(redundant) 194, …` — the report line.
    pub fn elided_summary(&self) -> String {
        if self.elided.is_empty() {
            return "none".to_string();
        }
        self.elided
            .iter()
            .map(|(k, n)| format!("{k} {n}"))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn elided_total(&self) -> usize {
        self.elided.values().sum()
    }
}

/// Strip what differs by construction rather than by behaviour.
///
/// - `n` is a per-run counter, so it differs the moment either side emits one
///   extra event. It is reported as a *position*, not a field.
/// - the `run` event's `impl` names which implementation ran, which is the
///   whole point of the comparison, and its `argv` carries the artefact paths
///   the **caller** chose — `--events a.jsonl` against `--events b.jsonl` is
///   not a divergence. Both stay in the file, where they document the run.
///
/// These two rules predate D3 and hold under **both** contracts, so the
/// strict differ calls this too. What the relaxed one adds on top — an
/// `enter`'s `n_firings` (rule 1) and a `dead-post` `enter`'s `core`
/// (rule 3) — [`split`] applies, and a strict call site simply does not go
/// through [`split`].
pub fn comparable(e: &Value) -> Value {
    let mut e = e.clone();
    let is_run = e.get("e").and_then(Value::as_str) == Some("run");
    if let Some(obj) = e.as_object_mut() {
        obj.remove("n");
        if is_run {
            obj.remove("impl");
            obj.remove("argv");
        }
    }
    e
}

/// [`comparable`], plus what the relaxed contract elides from an `enter`.
fn comparable_relaxed(e: &Value) -> Value {
    let mut e = comparable(e);
    if e.get("e").and_then(Value::as_str) != Some("enter") {
        return e;
    }
    let dead = e.get("kind").and_then(Value::as_str) == Some("dead-post");
    if let Some(obj) = e.as_object_mut() {
        obj.remove("n_firings");
        if dead {
            obj.remove("core");
        }
    }
    e
}

/// Split a log into [`Segment`]s.
pub fn split(events: &[Value]) -> Split {
    let mut segments = Vec::new();
    let mut elided: BTreeMap<String, usize> = BTreeMap::new();
    // Hypgen only runs on a saturated world, so its first event is where
    // root's saturation ends — and the only place it can end, since nothing
    // else marks the handover to the search layer.
    let mut root_closed = false;
    let (mut spine, mut facts, mut rules, mut productive) = (
        Vec::new(),
        BTreeMap::<String, usize>::new(),
        BTreeSet::new(),
        0usize,
    );
    for e in events {
        let kind = e.get("e").and_then(Value::as_str).unwrap_or("");
        let redundant = e.get("redundant").and_then(Value::as_bool) == Some(true);
        if SCHEDULING.contains(&kind) || (kind == "fire" && redundant) {
            let bucket = if kind == "fire" {
                "fire(redundant)".to_string()
            } else {
                kind.to_string()
            };
            *elided.entry(bucket).or_insert(0) += 1;
            continue;
        }
        if kind == "fire" || kind == "mirror" {
            productive += 1;
            rules.insert(match kind {
                "mirror" => "__symmetric__".to_string(),
                _ => e
                    .get("rule")
                    .and_then(Value::as_str)
                    .unwrap_or("<no rule>")
                    .to_string(),
            });
            if let Some(ds) = e.get("derived").and_then(Value::as_array) {
                for d in ds {
                    let key = d.as_str().map_or_else(|| d.to_string(), str::to_string);
                    *facts.entry(key).or_insert(0) += 1;
                }
            }
            continue;
        }
        if !root_closed && (kind == "hyp" || kind == "hypskip") {
            root_closed = true;
            segments.push(Segment {
                kind: None,
                spine: std::mem::take(&mut spine),
                facts: std::mem::take(&mut facts),
                rules: std::mem::take(&mut rules),
                productive: std::mem::replace(&mut productive, 0),
            });
        }
        let ended = kind == "enter";
        let entering_kind = e.get("kind").and_then(Value::as_str).map(str::to_string);
        spine.push(comparable_relaxed(e));
        if ended {
            root_closed = true;
            segments.push(Segment {
                kind: entering_kind,
                spine: std::mem::take(&mut spine),
                facts: std::mem::take(&mut facts),
                rules: std::mem::take(&mut rules),
                productive: std::mem::replace(&mut productive, 0),
            });
        }
    }
    segments.push(Segment {
        kind: None,
        spine,
        facts,
        rules,
        productive,
    });
    Split { segments, elided }
}

/// Compare two logs. An empty result is agreement.
///
/// The report is built around the two questions such a failure actually
/// raises, in the order they are worth asking: *which segment*, then *what in
/// it*.
pub fn diff(a: &[Value], b: &[Value]) -> Vec<String> {
    let (sa, sb) = (split(a), split(b));
    let mut out = Vec::new();
    if sa.segments.len() != sb.segments.len() {
        out.push(format!(
            "segments: a={} b={} (an entering on one side only)",
            sa.segments.len(),
            sb.segments.len()
        ));
    }
    for (i, (x, y)) in sa.segments.iter().zip(&sb.segments).enumerate() {
        let where_ = |what: &str| match &x.kind {
            Some(k) => format!("segment {i} [{k}] {what}"),
            None => format!("segment {i} [tail] {what}"),
        };
        if x.spine != y.spine {
            out.push(where_(&spine_diff(&x.spine, &y.spine)));
            break;
        }
        // A dying fork stops at the firing that kills it, so its derivation is
        // a prefix and not a claim. Its `kind` is compared above, in the spine.
        if x.kind.as_deref() == Some("dead-post") {
            continue;
        }
        if x.facts != y.facts {
            out.push(where_(&multiset_diff(&x.facts, &y.facts)));
            break;
        }
        if x.rules != y.rules {
            let only_a: Vec<&str> = x.rules.difference(&y.rules).map(String::as_str).collect();
            let only_b: Vec<&str> = y.rules.difference(&x.rules).map(String::as_str).collect();
            out.push(where_(&format!(
                "rules: a-only {only_a:?}, b-only {only_b:?}"
            )));
            break;
        }
    }
    if !out.is_empty() {
        out.push(format!(
            "  elided as narration — a: {} · b: {}",
            sa.elided_summary(),
            sb.elided_summary()
        ));
    }
    out
}

fn spine_diff(a: &[Value], b: &[Value]) -> String {
    for (i, (x, y)) in a.iter().zip(b).enumerate() {
        if x != y {
            let (Some(ox), Some(oy)) = (x.as_object(), y.as_object()) else {
                return format!("spine event {i}: {x} vs {y}");
            };
            let mut keys: Vec<&String> = ox.keys().chain(oy.keys()).collect();
            keys.sort();
            keys.dedup();
            let fields: Vec<String> = keys
                .into_iter()
                .filter(|k| ox.get(*k) != oy.get(*k))
                .map(|k| format!("{k}: {} vs {}", terse(ox.get(k)), terse(oy.get(k))))
                .collect();
            return format!("spine event {i}: {}", fields.join(", "));
        }
    }
    format!("spine: {} events vs {}", a.len(), b.len())
}

fn multiset_diff(a: &BTreeMap<String, usize>, b: &BTreeMap<String, usize>) -> String {
    let mut keys: Vec<&String> = a.keys().chain(b.keys()).collect();
    keys.sort();
    keys.dedup();
    let diffs: Vec<String> = keys
        .into_iter()
        .filter(|k| a.get(*k) != b.get(*k))
        .take(3)
        .map(|k| {
            format!(
                "{k} ×{} vs ×{}",
                a.get(k).copied().unwrap_or(0),
                b.get(k).copied().unwrap_or(0)
            )
        })
        .collect();
    format!("derived: {}", diffs.join("; "))
}

fn terse(v: Option<&Value>) -> String {
    let s = v.map_or("<absent>".to_string(), Value::to_string);
    if s.chars().count() > 80 {
        format!("{}…", s.chars().take(79).collect::<String>())
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fire(rule: &str, premises: &[&str], derived: &[&str], redundant: bool) -> Value {
        json!({"e": "fire", "n": 0, "rule": rule, "premises": premises,
               "derived": derived, "redundant": redundant})
    }

    fn log(events: &[Value]) -> Vec<Value> {
        events.to_vec()
    }

    /// Root's saturation, one alive fork, one dying fork — the shape every
    /// assertion below mutates.
    ///
    /// A fork's firings come **before** its `enter`, because the engine emits
    /// one when `try_commitment_set` returns; root's saturation is closed by
    /// the first `hyp`. So segment 0 is root's, segment 1 the alive fork's,
    /// segment 2 the dying fork's.
    fn base() -> Vec<Value> {
        log(&[
            json!({"e": "run", "n": 0, "impl": "ein.py", "argv": ["--events", "a.jsonl"]}),
            json!({"e": "load", "n": 1, "relations": 2}),
            fire("symmetric", &["(r a b)"], &["(r b a)"], false),
            json!({"e": "enqueue", "n": 3, "rule": "transitive"}),
            fire("transitive", &["(r b a)"], &["(r a a)"], false),
            json!({"e": "quiesce", "n": 5, "round": 1}),
            // ── root's saturation ends here ──
            json!({"e": "hyp", "n": 6, "fact": "(p x)", "verdict": "emitted"}),
            fire("functional", &["(p x)"], &["(q x)"], false),
            json!({"e": "enter", "n": 8, "layer": 1, "kind": "alive",
                   "commitment": ["(p x)"], "n_firings": 12, "core": []}),
            fire("functional", &["(p y)"], &["(q y)"], false),
            json!({"e": "enter", "n": 10, "layer": 1, "kind": "dead-post",
                   "commitment": ["(p y)"], "n_firings": 4, "core": ["(p y)"]}),
            json!({"e": "verdict", "n": 11, "type": "Solution", "k": 1}),
        ])
    }

    /// What ein.rs's resumed fork produces from the same run: a quarter of the
    /// narration, a dying fork stopped somewhere else, and the same answer.
    fn resumed() -> Vec<Value> {
        let mut v = base();
        v[0] = json!({"e": "run", "n": 0, "impl": "ein.rs", "argv": ["--events", "b.jsonl"]});
        v[3] = fire("symmetric", &["(r a b)"], &["(r b a)"], true); // one fewer enqueue
        v[8] = json!({"e": "enter", "n": 8, "layer": 1, "kind": "alive",
                      "commitment": ["(p x)"], "n_firings": 3, "core": []});
        // The dying fork stops at a different clash, so its own derivation and
        // its core both move — and neither is a difference.
        v[9] = fire("injective", &["(p y)"], &["(not (q z))"], false);
        v[10] = json!({"e": "enter", "n": 10, "layer": 1, "kind": "dead-post",
                       "commitment": ["(p y)"], "n_firings": 1, "core": ["(q y)"]});
        v
    }

    #[test]
    fn the_divergence_this_was_built_for_is_not_reported() {
        assert!(
            diff(&base(), &resumed()).is_empty(),
            "{:?}",
            diff(&base(), &resumed())
        );
    }

    #[test]
    fn a_dropped_productive_firing_is_caught() {
        // The acceptance criterion: a relaxation that cannot be shown to still
        // catch the thing it was relaxed around is a hole, not a decision.
        let mut broken = resumed();
        let at = broken
            .iter()
            .position(|e| e["rule"] == "transitive")
            .expect("the fixture has one");
        broken.remove(at);
        let d = diff(&base(), &broken);
        assert!(
            d.first().is_some_and(|s| s.contains("(r a a)")),
            "the dropped derivation is not named: {d:?}"
        );
    }

    #[test]
    fn a_productive_firing_that_became_redundant_is_caught() {
        // The subtler port bug: still emitted, but no longer deriving.
        let mut broken = resumed();
        broken[7] = fire("functional", &["(p x)"], &["(q x)"], true);
        assert!(!diff(&base(), &broken).is_empty());
    }

    /// The other half of the relaxation, and the reason a `dead-post` segment
    /// is excluded rather than merely tolerated: with fail-fast on, the two
    /// engines stop a dying fork at *different firings*, so its derivation is
    /// a prefix and not a claim about anything.
    #[test]
    fn a_dying_forks_own_derivation_is_not_compared() {
        let mut other = resumed();
        other[9] = fire("range-elimination", &["(p y)"], &["(not (q w))"], false);
        assert!(diff(&base(), &other).is_empty());
        // Its `kind` and its position still are.
        other[10]["kind"] = json!("dead-pre");
        assert!(!diff(&base(), &other).is_empty());
    }

    #[test]
    fn a_rule_that_stopped_firing_is_caught() {
        let mut broken = resumed();
        let at = broken
            .iter()
            .position(|e| e["rule"] == "transitive")
            .unwrap();
        broken[at] = fire("transitive-2", &["(r b a)"], &["(r a a)"], false);
        let d = diff(&base(), &broken);
        assert!(d.first().is_some_and(|s| s.contains("rules")), "{d:?}");
    }

    /// The boundary the negative control found: without it, root's own
    /// derivation shares a segment with the first entering, and under
    /// `--lookahead` that entering is a probe that dies — so a firing lost at
    /// root would be skipped along with the fork's prefix.
    #[test]
    fn roots_saturation_is_its_own_segment_and_is_always_compared() {
        let s = split(&base());
        assert_eq!(s.segments.len(), 4, "root, alive, dead-post, tail");
        assert_eq!(s.segments[0].kind, None);
        assert!(s.segments[0].facts.contains_key("(r b a)"));
        assert_eq!(s.segments[1].kind.as_deref(), Some("alive"));
        assert_eq!(s.segments[2].kind.as_deref(), Some("dead-post"));

        // …and now make the first entering a dying one, as `-L` does.
        let mut lookahead = base();
        lookahead[8] = json!({"e": "enter", "n": 8, "layer": 1, "kind": "dead-post",
                              "commitment": ["(p x)"], "n_firings": 12, "core": []});
        let mut broken = lookahead.clone();
        broken.remove(2); // a *root* firing, before any fork
        assert!(
            !diff(&lookahead, &broken).is_empty(),
            "a firing lost at root went unseen because the first fork died"
        );
    }

    #[test]
    fn the_scheduling_traffic_is_elided_and_counted() {
        let mut quiet = resumed();
        quiet.retain(|e| !SCHEDULING.contains(&e["e"].as_str().unwrap_or("")));
        assert!(diff(&base(), &quiet).is_empty());
        let s = split(&base());
        assert_eq!(s.elided.get("enqueue"), Some(&1));
        assert_eq!(s.elided.get("quiesce"), Some(&1));
        assert_eq!(split(&resumed()).elided.get("fire(redundant)"), Some(&1));
    }

    #[test]
    fn the_search_layer_is_still_compared_exactly() {
        for (field, value) in [("verdict.type", json!("Ambiguity")), ("layer", json!(2))] {
            let mut broken = resumed();
            if field == "layer" {
                broken[8]["layer"] = value.clone();
            } else {
                let last = broken.len() - 1;
                broken[last]["type"] = value.clone();
            }
            assert!(!diff(&base(), &broken).is_empty(), "{field} went unseen");
        }
        // An **alive** entering's core is the answer, not narration.
        let mut broken = resumed();
        broken[8]["core"] = json!(["(p x)"]);
        assert!(!diff(&base(), &broken).is_empty());
    }

    #[test]
    fn a_missing_entering_is_caught() {
        let mut broken = resumed();
        broken.remove(10);
        let d = diff(&base(), &broken);
        assert!(d.first().is_some_and(|s| s.contains("segments")), "{d:?}");
    }

    #[test]
    fn the_strict_normalisation_keeps_what_the_relaxed_one_drops() {
        // `comparable` is §5 and holds under both contracts: `n` is a
        // position and `impl` / `argv` are the caller's. `n_firings` and a
        // dying fork's core are D3, so only the relaxed pass drops them.
        let strict = comparable(&base()[8]);
        assert_eq!(strict.get("n"), None);
        assert_eq!(strict.get("impl"), None);
        assert_eq!(strict.get("n_firings"), Some(&json!(12)));
        let relaxed = comparable_relaxed(&base()[8]);
        assert_eq!(relaxed.get("n_firings"), None);
        assert_eq!(relaxed.get("core"), Some(&json!([])));
        assert_eq!(comparable_relaxed(&base()[10]).get("core"), None);
    }
}
