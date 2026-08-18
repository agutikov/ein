//! `ein saturate` — wall-clock benchmark + state dump for the Saturator.
//!
//! The Rust half of `ein/cli/saturate.py`. The delegated subcommand: its own
//! parser, its own `prog`, reached before the top-level one runs.
//!
//! Its output is dense and entirely mechanical — an entity census, a
//! before/after snapshot with Δ columns, a firing breakdown and an optional
//! whole-KB dump — which makes it the phase's easiest T3 target and its most
//! unforgiving one: every column position below is ein.py's.

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Instant;

use ein_core::pyfmt::format_spec;
use ein_core::{Kb, NameCategory, ProvKind, SolverConfig, Terms};
use ein_infer::SharedMemo;
use ein_infer::events::{Events, Level};
use ein_infer::firing::Firing;
use ein_infer::saturator::{Saturator, Session};
use ein_infer::{Engine, events};
use ein_ir::{Ast, Node, NodeId};

// ── Snapshot ────────────────────────────────────────────────────

/// Every countable property of a KB + Engine, flat so before/after is
/// subtractable.
#[derive(Default)]
struct Snap {
    scalars: Vec<(&'static str, i64)>,
    prov_kinds: BTreeMap<String, i64>,
    arity_hist: BTreeMap<usize, i64>,
    by_relation: BTreeMap<String, i64>,
}

impl Snap {
    fn get(&self, key: &str) -> i64 {
        self.scalars
            .iter()
            .find(|(k, _)| *k == key)
            .map_or(0, |(_, v)| *v)
    }
}

fn snapshot(terms: &Terms, kb: &Kb, eng: &Engine) -> Snap {
    let p = kb.program();
    let declared = p.relations.values().filter(|r| r.declared).count() as i64;

    let mut names_objects = 0i64;
    let mut names_relations = 0i64;
    let mut names_rules = 0i64;
    let (mut head_total, mut arg_total, mut head_only, mut arg_only) = (0i64, 0i64, 0i64, 0i64);
    let names = kb.names();
    for &n in &names {
        match kb.category(terms, n) {
            NameCategory::Object => names_objects += 1,
            NameCategory::Relation => names_relations += 1,
            NameCategory::Rule => names_rules += 1,
        }
        let (h, a) = kb.name_entry(n);
        head_total += h as i64;
        arg_total += a as i64;
        if h > 0 && a == 0 {
            head_only += 1;
        }
        if a > 0 && h == 0 {
            arg_only += 1;
        }
    }

    let not = terms.syms.get("not");
    let (mut derived, mut given, mut negated, mut nested) = (0i64, 0i64, 0i64, 0i64);
    let mut prov_kinds: BTreeMap<String, i64> = BTreeMap::new();
    let mut arity_hist: BTreeMap<usize, i64> = BTreeMap::new();
    let mut by_relation: BTreeMap<String, i64> = BTreeMap::new();
    let mut total = 0i64;
    for f in kb.facts() {
        total += 1;
        let (rel, args) = terms.facts.get(f);
        match kb.primary(f) {
            None => *prov_kinds.entry("<none>".to_string()).or_default() += 1,
            Some(id) => {
                let prov = terms.provs.get(id);
                *prov_kinds
                    .entry(prov.kind.as_str().to_string())
                    .or_default() += 1;
                if prov.kind == ProvKind::Source {
                    if prov.source.is_some() {
                        given += 1;
                    }
                } else {
                    derived += 1;
                }
            }
        }
        *arity_hist.entry(args.len()).or_default() += 1;
        if args.iter().any(|a| a.as_fact().is_some()) {
            nested += 1;
        }
        if Some(rel) == not {
            negated += 1;
        }
        *by_relation.entry(terms.sym(rel).to_string()).or_default() += 1;
    }
    let idx = kb.index_sizes();

    Snap {
        scalars: vec![
            ("relations", p.relations.len() as i64),
            ("relations_declared", declared),
            ("relations_open_world", p.relations.len() as i64 - declared),
            ("rules", p.rules.len() as i64),
            ("names_total", names.len() as i64),
            ("names_objects", names_objects),
            ("names_relations", names_relations),
            ("names_rules", names_rules),
            ("names_as_head_total", head_total),
            ("names_as_arg_total", arg_total),
            ("names_head_only", head_only),
            ("names_arg_only", arg_only),
            ("facts_total", total),
            ("facts_background", total - derived - given),
            ("facts_given", given),
            ("facts_derived", derived),
            ("facts_negated", negated),
            ("facts_with_nested_args", nested),
            ("index_facts_by_relation", idx[0] as i64),
            ("index_facts_by_rel_slot_val", idx[1] as i64),
            ("index_rule_apps_by_rule", idx[2] as i64),
            ("index_rule_apps_on_relation", idx[3] as i64),
            ("engine_cache_size", eng.len() as i64),
            ("engine_fired", eng.fired.len() as i64),
        ],
        prov_kinds,
        arity_hist,
        by_relation,
    }
}

// ── Pretty printing ─────────────────────────────────────────────

fn fmt_int(n: i64) -> String {
    format!("{n:>6}")
}

/// Note the asymmetry, which is ein.py's: a positive delta is `+` followed by
/// a 5-wide number (6 chars), a negative one is a 5-wide number (5 chars),
/// and zero is six blanks.
fn fmt_delta(d: i64) -> String {
    if d == 0 {
        "      ".to_string()
    } else if d > 0 {
        format!("+{d:>5}")
    } else {
        format!("{d:>5}")
    }
}

/// `SCALAR_KEYS`. `None` is a separator; `types` / `instances` head the list
/// in ein.py under keys the snapshot never sets, so they never print and are
/// not carried here.
const SCALAR_KEYS: [Option<(&str, &str)>; 27] = [
    Some(("relations", "relations (total)")),
    Some(("relations_declared", "  declared")),
    Some(("relations_open_world", "  open-world (auto-vivified)")),
    Some(("rules", "rules")),
    None,
    Some(("names_total", "names (global, encoding-agnostic)")),
    Some(("names_objects", "  category = object")),
    Some(("names_relations", "  category = relation")),
    Some(("names_rules", "  category = rule")),
    Some(("names_as_head_total", "  total head-participations")),
    Some(("names_as_arg_total", "  total arg-participations")),
    Some(("names_head_only", "  appearing only as head")),
    Some(("names_arg_only", "  appearing only as arg")),
    None,
    Some(("facts_total", "facts (total)")),
    Some(("facts_background", "  origin = BACKGROUND")),
    Some(("facts_given", "  origin = GIVEN")),
    Some(("facts_derived", "  origin = DERIVED")),
    None,
    Some(("facts_negated", "facts whose head is `not`")),
    Some((
        "facts_with_nested_args",
        "facts with nested-Fact args (Q40)",
    )),
    None,
    Some((
        "index_facts_by_relation",
        "index entries: facts_by_relation",
    )),
    Some((
        "index_facts_by_rel_slot_val",
        "index entries: facts_by_rel_slot_val",
    )),
    Some((
        "index_rule_apps_by_rule",
        "index entries: rule_apps_by_rule",
    )),
    Some((
        "index_rule_apps_on_relation",
        "index entries: rule_apps_on_relation",
    )),
    None,
];

/// The engine pair closes the list; kept out of the const so the two arrays
/// read as the sections they are.
const ENGINE_KEYS: [(&str, &str); 2] = [
    ("engine_cache_size", "engine cache: (rule, activator) plans"),
    ("engine_fired", "engine cache: bindings fired"),
];

fn print_snapshot(before: Option<&Snap>, after: &Snap, title: &str) {
    println!();
    println!("── {title} ──");
    let rows: Vec<Option<(&str, &str)>> = SCALAR_KEYS
        .iter()
        .copied()
        .chain(ENGINE_KEYS.iter().map(|&kv| Some(kv)))
        .collect();
    match before {
        None => {
            for row in &rows {
                match row {
                    None => println!(),
                    Some((key, label)) => {
                        println!("  {label:<42}  {}", fmt_int(after.get(key)))
                    }
                }
            }
        }
        Some(b) => {
            println!("  {:<42}  {:>6}  {:>6}  {:>6}", "", "before", "after", "Δ");
            for row in &rows {
                match row {
                    None => println!(),
                    Some((key, label)) => {
                        let (x, y) = (b.get(key), after.get(key));
                        println!(
                            "  {label:<42}  {}  {}  {}",
                            fmt_int(x),
                            fmt_int(y),
                            fmt_delta(y - x)
                        );
                    }
                }
            }
        }
    }

    println!();
    dict_breakdown(
        "provenance kinds",
        before.map(|b| &b.prov_kinds),
        &after.prov_kinds,
    );
    let key_str = |k: &usize| k.to_string();
    dict_breakdown_by(
        "fact arities (arity → count)",
        before.map(|b| &b.arity_hist),
        &after.arity_hist,
        key_str,
    );
    dict_breakdown(
        "facts by relation",
        before.map(|b| &b.by_relation),
        &after.by_relation,
    );
}

fn dict_breakdown(
    label: &str,
    before: Option<&BTreeMap<String, i64>>,
    after: &BTreeMap<String, i64>,
) {
    dict_breakdown_by(label, before, after, |k: &String| k.clone())
}

/// `_print_dict_breakdown` — keys are the *union* of both sides, sorted, so a
/// relation that vanished still shows with its `after` zero.
fn dict_breakdown_by<K: Ord + Clone>(
    label: &str,
    before: Option<&BTreeMap<K, i64>>,
    after: &BTreeMap<K, i64>,
    show: impl Fn(&K) -> String,
) {
    println!("  {label}:");
    let mut keys: Vec<&K> = after.keys().collect();
    if let Some(b) = before {
        for k in b.keys() {
            if !after.contains_key(k) {
                keys.push(k);
            }
        }
    }
    keys.sort();
    for k in keys {
        let a = after.get(k).copied().unwrap_or(0);
        match before {
            Some(b) => {
                let x = b.get(k).copied().unwrap_or(0);
                println!(
                    "    {:<38}  {}  {}  {}",
                    show(k),
                    fmt_int(x),
                    fmt_int(a),
                    fmt_delta(a - x)
                );
            }
            None => println!("    {:<38}  {}", show(k), fmt_int(a)),
        }
    }
}

// ── Firing analysis ─────────────────────────────────────────────

/// A `Counter` whose iteration order is insertion order, which is what the
/// `sorted(…, key=lambda r: -count[r])` tie-break resolves to.
struct Tally {
    order: Vec<String>,
    counts: Vec<i64>,
}

impl Tally {
    fn new() -> Tally {
        Tally {
            order: Vec::new(),
            counts: Vec::new(),
        }
    }

    fn bump(&mut self, key: &str) {
        match self.order.iter().position(|k| k == key) {
            Some(i) => self.counts[i] += 1,
            None => {
                self.order.push(key.to_string());
                self.counts.push(1);
            }
        }
    }

    fn get(&self, key: &str) -> i64 {
        self.order
            .iter()
            .position(|k| k == key)
            .map_or(0, |i| self.counts[i])
    }

    /// Descending by count, ties in insertion order — `sort_by` is stable.
    fn descending(&self) -> Vec<(&str, i64)> {
        let mut rows: Vec<(&str, i64)> = self
            .order
            .iter()
            .map(String::as_str)
            .zip(self.counts.iter().copied())
            .collect();
        rows.sort_by_key(|r| std::cmp::Reverse(r.1));
        rows
    }
}

fn print_firings(terms: &Terms, firings: &[Firing]) {
    let total = firings.len() as i64;
    let productive = firings.iter().filter(|f| !f.redundant).count() as i64;
    let redundant = total - productive;

    let mut per_rule_total = Tally::new();
    let mut per_rule_productive = Tally::new();
    let mut per_relation = Tally::new();
    for f in firings {
        let rule = terms.sym(f.rule);
        per_rule_total.bump(rule);
        if !f.redundant {
            per_rule_productive.bump(rule);
            for &d in f.derived.iter() {
                let rel = terms.facts.get(d).0;
                per_relation.bump(terms.sym(rel));
            }
        }
    }

    println!();
    println!("── saturation: firing breakdown ──");
    println!("  total firings              {}", fmt_int(total));
    println!("    productive (new fact)    {}", fmt_int(productive));
    println!("    redundant (already in KB){}", fmt_int(redundant));

    println!();
    println!("  per-rule firings (rule → productive / total):");
    for (rule, tot) in per_rule_total.descending() {
        println!(
            "    {:<38}  {} / {}",
            rule,
            fmt_int(per_rule_productive.get(rule)),
            fmt_int(tot)
        );
    }

    println!();
    println!("  derived facts by relation:");
    for (rel, n) in per_relation.descending() {
        println!("    {:<38}  {}", rel, fmt_int(n));
    }
}

// ── Saturated KB dump (--dump) ──────────────────────────────────

/// `_fact_text` — `(rel arg1 arg2 …)`, nested facts recursing.
fn fact_text(terms: &Terms, id: ein_core::FactId) -> String {
    events::sexpr(terms, id)
}

/// `_fact_text_with_provenance` — the compact form with its annotation.
fn fact_text_with_provenance(terms: &Terms, kb: &Kb, id: ein_core::FactId) -> String {
    let base = fact_text(terms, id);
    let base = &base[1..base.len() - 1];
    let Some(pid) = kb.primary(id) else {
        return format!("({base})");
    };
    let p = terms.provs.get(pid);
    match p.kind {
        ProvKind::Source => match p.source {
            Some(s) => format!("({base} :source \"{}\")", terms.sym(s)),
            None => format!("({base})"),
        },
        ProvKind::Rule => match p.rule {
            Some(r) => format!("({base} :rule {})", terms.sym(r)),
            None => format!("({base})"),
        },
        ProvKind::Hypothesis => format!("({base} :hypothesis {})", branch(p)),
        ProvKind::Rejected => format!("({base} :rejected {})", branch(p)),
    }
}

/// `p.branch` — `None` prints as Python's `None`.
fn branch(p: &ein_core::Prov) -> String {
    match p.branch {
        Some(b) => b.to_string(),
        None => "None".to_string(),
    }
}

/// Q41's priority bands.
fn band_label(priority: Option<i64>) -> &'static str {
    match priority {
        None => "unbanded",
        Some(p) if p < 200 => "propagate",
        Some(p) if p < 300 => "derive",
        Some(p) if p < 900 => "eliminate",
        Some(_) => "hypothesis",
    }
}

fn dump_kb(ast: &Ast, terms: &Terms, kb: &Kb) {
    println!();
    println!("{}", "=".repeat(70));
    println!("=  SATURATED KB DUMP");
    println!("{}", "=".repeat(70));

    let p = kb.program();
    if !p.relations.is_empty() {
        let declared: Vec<_> = p.relations.values().filter(|r| r.declared).collect();
        let open_w: Vec<_> = p.relations.values().filter(|r| !r.declared).collect();
        if !declared.is_empty() {
            println!();
            println!(";; Relations — declared ({})", declared.len());
            for r in &declared {
                let sig: Vec<&str> = r.signature.iter().map(|&s| terms.sym(s)).collect();
                let sig = sig.join(" ");
                let tail = if sig.is_empty() {
                    String::new()
                } else {
                    format!(" {sig}")
                };
                println!("(relation {}{tail})", terms.sym(r.name));
            }
        }
        if !open_w.is_empty() {
            println!();
            println!(";; Relations — auto-vivified ({})", open_w.len());
            for r in &open_w {
                println!(";;   {}", terms.sym(r.name));
            }
        }
    }

    if !p.rules.is_empty() {
        println!();
        println!(";; Rules ({})", p.rules.len());
        for rule in p.rules.values() {
            let prio = rule.priority.and_then(|id| terms.ints.value(id));
            let shown = match rule.priority {
                Some(id) => terms.ints.text(id).to_string(),
                None => "None".to_string(),
            };
            let params: Vec<String> = rule
                .params
                .iter()
                .map(|&s| format!("?{}", terms.sym(s)))
                .collect();
            println!(
                ";;   {}  :priority {shown} ({})  :params ({})",
                terms.sym(rule.name),
                band_label(prio),
                params.join(" ")
            );
        }
    }

    // Facts grouped by origin, then by relation within a bucket, insertion
    // order preserved inside each group.
    let mut buckets: [Vec<ein_core::FactId>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for f in kb.facts() {
        let slot = match kb.primary(f) {
            None => 0,
            Some(id) => {
                let prov = terms.provs.get(id);
                if prov.kind != ProvKind::Source {
                    2
                } else if prov.source.is_some() {
                    1
                } else {
                    0
                }
            }
        };
        buckets[slot].push(f);
    }
    for (slot, label) in [(0, "BACKGROUND"), (1, "GIVEN"), (2, "DERIVED")] {
        let facts = &buckets[slot];
        if facts.is_empty() {
            continue;
        }
        println!();
        println!(";; ── {label} ({} facts) ──", facts.len());
        let mut groups: Vec<(String, Vec<ein_core::FactId>)> = Vec::new();
        for &f in facts {
            let rel = terms.sym(terms.facts.get(f).0).to_string();
            match groups.iter_mut().find(|(k, _)| *k == rel) {
                Some((_, v)) => v.push(f),
                None => groups.push((rel, vec![f])),
            }
        }
        groups.sort_by(|a, b| a.0.cmp(&b.0));
        for (rel, facts_for_rel) in &groups {
            if facts_for_rel.len() > 1 {
                println!(";;   {rel} ({})", facts_for_rel.len());
            }
            for &f in facts_for_rel {
                println!("  {}", fact_text_with_provenance(terms, kb, f));
            }
        }
    }

    if let Some(query) = p.query.as_ref() {
        println!();
        println!(";; Query");
        for &pair in query.kw_pairs.iter() {
            let Node::KwPair { key, value } = ast.node(NodeId(pair.0)) else {
                continue;
            };
            let Node::Keyword(name) = ast.node(key) else {
                continue;
            };
            println!(
                ";;   :{}  {}",
                ast.sym(name),
                ein_ir::dump_compact(ast, value)
            );
        }
    }

    println!();
    println!("{}", "=".repeat(70));
}

/// The per-rule tally's key: a rule name and whether the firing was
/// redundant, which ein.py keys with a `(str, bool)` tuple.
fn tally_key(rule: &str, redundant: bool) -> String {
    format!("{rule}\u{0}{redundant}")
}

/// `SaturatorStepLimitError`'s message, whose tail is `repr(self._last_firing)`
/// — a dataclass repr all the way down, which is why it goes through
/// [`ein_core::pyrepr`] rather than Rust's `{:?}`.
fn step_limit_message(max_steps: i64, terms: &Terms, last: Option<&Firing>) -> String {
    let shown = match last {
        None => "None".to_string(),
        Some(f) => repr_firing(terms, f),
    };
    format!(
        "saturator hit max_steps={max_steps} without reaching fixed point — \
         last firing was {shown}; see Saturator._last_firing for the runaway \
         candidate."
    )
}

fn repr_firing(terms: &Terms, f: &Firing) -> String {
    use ein_core::pyrepr::{PyValue, repr, repr_str};
    let tuple = |vs: Vec<PyValue>| repr(&PyValue::Tuple(vs));
    let activator = tuple(
        f.activator
            .iter()
            .map(|&s| PyValue::Str(terms.sym(s).to_string()))
            .collect(),
    );
    let bindings: Vec<String> = f
        .bindings
        .iter()
        .map(|(k, v)| format!("{}: {}", repr_str(terms.sym(*k)), repr(&terms.py_value(*v))))
        .collect();
    let facts = |ids: &[ein_core::FactId]| tuple(ids.iter().map(|&id| terms.py_fact(id)).collect());
    format!(
        "Firing(rule={}, activator={activator}, bindings={{{}}}, derived={}, \
         premises={}, redundant={})",
        repr_str(terms.sym(f.rule)),
        bindings.join(", "),
        facts(&f.derived),
        facts(&f.premises),
        if f.redundant { "True" } else { "False" },
    )
}

// ── Main ────────────────────────────────────────────────────────

fn ms(t: Instant) -> String {
    format_spec(t.elapsed().as_secs_f64() * 1000.0, "8.2f")
}

#[allow(clippy::too_many_arguments)]
fn bench(
    path: &Path,
    dump: bool,
    max_steps: Option<i64>,
    progress_every: i64,
    open_log: impl FnOnce(Option<&SolverConfig>) -> Events,
) -> Result<(), String> {
    let src = crate::common::read_text_or_crash(path);
    println!("input:   {}", path.display());
    println!("         {} chars", src.chars().count());

    let mut ast = Ast::new();
    let t = Instant::now();
    let forms = ein_ir::parse(&mut ast, &src, path.to_str()).map_err(|e| e.to_string())?;
    println!("parse:    {} ms  ({} top-level forms)", ms(t), forms.len());

    // No `base_dir`: `saturate` calls `from_ir(forms)` bare, so a
    // file-relative `(import …)` resolves against the *working directory*
    // here where `solve` resolves it against the puzzle's own.
    let mut terms = Terms::new();
    let t = Instant::now();
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None)
        .map_err(|e| format!("kb load error: {e}"))?;
    println!("kb load:  {} ms", ms(t));

    // The log opens here — after the kb exists, before anything is compiled or
    // fired, and carrying the KB's *own* config, since `saturate` has no CLI
    // overrides to resolve. It has no verdict either, so the stream is the
    // deductive layer on its own.
    let mut events_owned = open_log(kb.program().config.as_ref());
    let events = &mut events_owned;
    crate::solve::events_load(events, &terms, &kb);

    // Compiled with the log **off**: ein.py hands this engine to the
    // `Saturator`, so its enqueue pass finds the cache warm and emits nothing;
    // here the saturator builds its own and compiles again, and recording both
    // passes would double every `compile` event.
    let mut eng = Engine::new();
    let mut quiet = Events::off();
    let t = Instant::now();
    eng.compile_all(&ast, &mut terms, &kb, &mut quiet)
        .map_err(crate::common::compile_error_line)?;
    println!("compile:  {} ms", ms(t));

    let before = snapshot(&terms, &kb, &eng);
    print_snapshot(None, &before, "state BEFORE saturation");

    println!();
    match max_steps {
        Some(n) => {
            println!("saturate: running with max_steps={n}, progress every {progress_every} steps")
        }
        None => println!("saturate: running unbounded"),
    }

    let mut firings: Vec<Firing> = Vec::new();
    let mut per_rule: Tally = Tally::new();
    let t_start = Instant::now();
    let mut t_mark = t_start;
    let mut limit_msg: Option<String> = None;
    // ein.py hands its pre-compiled `Engine` to the `Saturator`, so the two
    // snapshots read the *same* object: 6 plans before, 32 and 880 fired
    // after. Here the saturator builds its own, so the after-snapshot reads
    // that one back out.
    let sat_engine: Option<Engine>;
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events,
            memo: SharedMemo::default(),
        };
        // `step` is driven directly rather than through `saturate`, because
        // the progress line reads the KB's fact count *at that step* and the
        // firing sink has no handle on it.
        let mut sat = Saturator::new(&mut s).map_err(|e| crate::common::saturate_error_line(&e))?;
        let mut i: usize = 0;
        loop {
            if max_steps.is_some_and(|m| i as i64 >= m) {
                limit_msg = Some(step_limit_message(
                    max_steps.expect("checked"),
                    s.terms,
                    sat.last_firing(),
                ));
                break;
            }
            let firing = match sat
                .step(&mut s)
                .map_err(|e| crate::common::saturate_error_line(&e))?
            {
                None => break,
                Some(f) => f,
            };
            i += 1;
            if progress_every > 0 && i as i64 % progress_every == 0 {
                let now = Instant::now();
                println!(
                    "  step {:>6}  Δ={} ms  facts={:>6}  last={}{}",
                    i,
                    format_spec(now.duration_since(t_mark).as_secs_f64() * 1000.0, "8.2f"),
                    s.kb.n_facts(),
                    ein_core::pyrepr::repr_str(s.terms.sym(firing.rule)),
                    if firing.redundant { " [redundant]" } else { "" }
                );
                t_mark = now;
            }
            per_rule.bump(&tally_key(s.terms.sym(firing.rule), firing.redundant));
            firings.push(firing.clone());
            sat.set_last_firing(firing);
        }
        sat_engine = Some(std::mem::replace(&mut sat.engine, Engine::new()));
    }
    if let Some(msg) = &limit_msg {
        println!();
        println!("!! saturator step limit hit: {msg}");
    }
    println!();
    println!("saturate: {} ms  ({} firings)", ms(t_start), firings.len());

    let after = snapshot(&terms, &kb, sat_engine.as_ref().unwrap_or(&eng));
    print_snapshot(Some(&before), &after, "state AFTER saturation");
    print_firings(&terms, &firings);

    if limit_msg.is_some() {
        println!();
        println!("── per-rule firing breakdown at limit ──");
        for (key, n) in per_rule.descending() {
            let (rule, redundant) = key.split_once('\u{0}').unwrap_or((key, "false"));
            let tag = if redundant == "true" {
                "redundant"
            } else {
                "productive"
            };
            println!("  {rule:30} [{tag:10}] {n:>6}");
        }
    }

    if dump {
        dump_kb(&ast, &terms, &kb);
    }
    Ok(())
}

pub fn main(argv: &[String]) -> i32 {
    let m = crate::cmdline::saturate_command()
        .get_matches_from(std::iter::once("ein saturate".to_string()).chain(argv.iter().cloned()));
    let file = m.get_one::<String>("file").expect("required").clone();
    let target = Path::new(&file);
    if !target.exists() {
        eprintln!("error: {file} not found");
        return 1;
    }
    let events_path = m.get_one::<String>("events").cloned();
    let level = match m.get_one::<String>("events-level").map(String::as_str) {
        Some("verbose") => Level::Verbose,
        _ => Level::Normal,
    };
    let file_for_log = file.clone();
    let open_log = move |cfg: Option<&SolverConfig>| match events_path {
        None => Events::off(),
        Some(path) => match std::fs::File::create(&path) {
            Ok(f) => {
                let argv_all: Vec<String> = std::env::args().skip(1).collect();
                let cfg_json = cfg.map(crate::solve::config_json);
                Events::to_with(Box::new(std::io::BufWriter::new(f)), level, |l| {
                    l.str("impl", "ein.rs");
                    l.str("file", &file_for_log);
                    l.owned_strs("argv", argv_all);
                    if let Some(c) = cfg_json.as_ref() {
                        l.obj_strs("config", c);
                    }
                })
            }
            Err(e) => {
                eprintln!("{e}");
                Events::off()
            }
        },
    };
    match bench(
        target,
        m.get_flag("dump"),
        m.get_one::<i64>("max-steps").copied(),
        *m.get_one::<i64>("progress-every").unwrap_or(&500),
        open_log,
    ) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}
