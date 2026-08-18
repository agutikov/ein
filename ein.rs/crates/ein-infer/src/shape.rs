//! Every plan a KB compiles, as one deterministic text — the S1a.3.1 diff.
//!
//! A `JoinPlan` has no CLI surface, so `ein-conformance` cannot see one: it
//! compares two `ein` binaries, and nothing either of them prints exposes a
//! step sequence, a guard's scope, or a `watched` set. So the compiler is
//! compared the way the loader was at
//! [S1a.2.3](../../../../plans/m1a_rust/p1a.2_kb_core/s1a.2.3_loader_and_provenance.md):
//! both implementations render the same text and the texts are diffed
//! (`utils/ir_oracle.py`'s `plan-shape` op is the other half).
//!
//! Two rules make the text comparable, and they are the same two the KB shape
//! settled on:
//!
//! - **Values are rendered with `repr`**, so the atom `7` prints as `'7'` and
//!   the integer `7` as `7`, and a slot that changed shape cannot hide.
//! - **Sets are rendered sorted** — `watched`, `scope` and a step's shared
//!   variables are `frozenset`s in ein.py, whose order is not reproducible
//!   even run to run.
//!
//! What is deliberately *not* in the text: registers and probes. They are the
//! port's own metadata, ein.py has nothing to compare them against, and the
//! claim that they change no behaviour is checked where it is made — by the
//! matcher's debug assertion that its probe choice is `_candidates`' choice
//! (S1a.3.2).

use ein_core::entities::Rule;
use ein_core::pyrepr::{PyValue, repr, repr_str};
use ein_core::{FactId, Kb, Symbol, Terms};
use ein_ir::{Ast, node_repr};
use rustc_hash::FxHashMap;

use crate::compile::{
    CompileError, activators_for, asserted_relation, naf_relation_refs, negated_relation, plan_key,
};
use crate::match_::{Match, Matcher};
use crate::plan::{GuardArgKind, NafGuard, Plan, Slot, Span, Step};

/// Compile every `(rule, activator)` pair in `Engine.compile_all` order and
/// render the lot.
///
/// The order is `kb.rules` in registry (insertion) order × that rule's
/// activators in `rule_apps_by_rule` order, which is the order the compile
/// cache is built in — and the cache's iteration order is observable through
/// `_enqueue_pass`'s full pass, so it is part of what this diff checks.
pub fn plan_shape(ast: &Ast, terms: &mut Terms, kb: &Kb) -> Result<String, CompileError> {
    plan_shape_with(ast, terms, kb, true)
}

/// [`plan_shape`], optionally without `activators_for`'s S1.22.0 **arity**
/// filter.
///
/// Nothing in the engine compiles an unfiltered pair — both drivers filter
/// first, which is exactly why `compile_rule`'s arity error is otherwise
/// unreachable. `filter_activators: false` is how its fixture reaches it, on
/// both sides (`plan-shape` takes the same flag).
pub fn plan_shape_with(
    ast: &Ast,
    terms: &mut Terms,
    kb: &Kb,
    filter_activators: bool,
) -> Result<String, CompileError> {
    let rules: Vec<Rule> = kb.program().rules.values().cloned().collect();
    let mut out = String::new();
    for rule in &rules {
        let activators = if filter_activators {
            activators_for(kb, terms, rule)
        } else if rule.params.is_empty() {
            vec![None]
        } else {
            kb.rule_apps_by_rule(rule.name).map(Some).collect()
        };
        for activator in activators {
            let key = plan_key(terms, rule, activator);
            let plan = crate::compile::compile_rule(ast, terms, rule, activator)?;
            render_plan(&mut out, ast, terms, &plan, &key.activator);
        }
    }
    // Lines are `"\n".join`ed on the Python side, so there is no trailing one.
    if out.ends_with('\n') {
        out.pop();
    }
    Ok(out)
}

/// Every match every plan produces over `kb` — the S1a.3.2 diff.
///
/// Two sweeps per plan, because the matcher has two entry shapes and they owe
/// each other an identity: the full run, and a `run_seeded` at **every fact in
/// the KB**, which is what forces the premise-order contract — a seeded match's
/// provenance must read exactly like a full run's, seeded fact at its own
/// step's position.
///
/// Bindings go out in bind order (the trail's order, which is
/// `Provenance.bindings`') and premises as fact **positions**, so an order or
/// identity difference names itself rather than showing up as a wall of
/// re-rendered facts.
pub fn match_shape(ast: &Ast, terms: &mut Terms, kb: &Kb) -> Result<String, CompileError> {
    let rules: Vec<Rule> = kb.program().rules.values().cloned().collect();
    let facts: Vec<FactId> = kb.facts().collect();
    let at: FxHashMap<FactId, usize> = facts.iter().enumerate().map(|(i, &f)| (f, i)).collect();
    let mut matcher = Matcher::new();
    let mut out = String::new();
    for rule in &rules {
        for activator in activators_for(kb, terms, rule) {
            let key = plan_key(terms, rule, activator);
            let plan = crate::compile::compile_rule(ast, terms, rule, activator)?;
            let key_repr = repr(&PyValue::Tuple(
                key.activator
                    .iter()
                    .map(|&s| PyValue::Str(terms.sym(s).to_string()))
                    .collect(),
            ));
            out.push_str(&format!("PLAN {} key={key_repr}\n", terms.sym(plan.rule)));
            for d in 0..plan.disjuncts.len() {
                matcher.run_one(kb, terms, ast, &plan, d, &mut |m| {
                    out.push_str(&format!("  RUN D{d} {}\n", match_text(terms, &at, m)));
                    std::ops::ControlFlow::Continue(())
                });
            }
            for (j, &fact) in facts.iter().enumerate() {
                matcher.run_seeded(kb, terms, ast, &plan, fact, &mut |m| {
                    out.push_str(&format!(
                        "  SEED {j} D{} {}\n",
                        m.disjunct,
                        match_text(terms, &at, m)
                    ));
                    std::ops::ControlFlow::Continue(())
                });
            }
        }
    }
    if out.ends_with('\n') {
        out.pop();
    }
    Ok(out)
}

/// The `--events` log of a root saturation, plus the counters — the S1a.3.3
/// diff.
///
/// The protocol itself, at `verbose`, so a redundant firing is emitted rather
/// than only counted — a dropped one is exactly the kind of difference a port
/// introduces, which is why T2 runs at that level. The trailing `SUMMARY` line
/// carries what is engine state rather than an event: the NAF counters the
/// phase gates on, and the compile-cache size Win A is measured in.
pub fn saturate_events(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
) -> Result<String, crate::saturator::SaturateError> {
    let buffer = crate::events::Buffer::new();
    let mut events =
        crate::events::Events::to(Box::new(buffer.clone()), crate::events::Level::Verbose);
    let mut s = crate::saturator::Session {
        kb,
        terms,
        ast,
        events: &mut events,
    };
    let mut sat = crate::saturator::Saturator::new(&mut s)?;
    sat.saturate(&mut s, None, &mut |_| {})?;
    // S1.21.8 negative provenance: what each firing depended on *not* holding.
    // Neither the event stream nor the KB shape carries it, so without this
    // line `absent_premises` would be the one thing the boundary produces that
    // nothing compares — and it is where a scope projection goes wrong.
    let mut absents = String::new();
    for (i, fact) in s.kb.facts().enumerate() {
        let mut provs: Vec<(&str, ein_core::ProvId)> = Vec::new();
        if let Some(p) = s.kb.primary(fact) {
            provs.push(("primary", p));
        }
        provs.extend(s.kb.alternatives(fact).iter().map(|p| ("alt", *p)));
        for (label, id) in provs {
            let prov = s.terms.provs.get(id);
            if prov.absent.is_empty() {
                continue;
            }
            let refs: Vec<String> = prov.absent.iter().map(|r| naf_repr(s.terms, r)).collect();
            absents.push_str(&format!("ABSENT {i} {label} [{}]\n", refs.join(", ")));
        }
    }

    // The detector's own output, on the saturated KB: direct ⊥ first, then
    // pairs in extent order — an order that reaches the unsat core, so it is
    // compared rather than assumed.
    let clashes: String = crate::contradiction::detect(s.kb, s.terms)
        .iter()
        .map(|c| {
            format!(
                "CLASH {} {} {}\n",
                c.kind.as_str(),
                c.positive
                    .map(|f| crate::events::sexpr(s.terms, f))
                    .unwrap_or_else(|| "-".to_string()),
                crate::events::sexpr(s.terms, c.negative),
            )
        })
        .collect();
    let summary = format!(
        "SUMMARY facts={} rounds={} admitted={} retired={} dropped={} \
         fired={} seen={} plans={}",
        s.kb.n_facts(),
        sat.naf_rounds,
        sat.naf_admitted,
        sat.naf_retired,
        sat.naf_dropped,
        sat.engine.fired.len(),
        sat.n_seen(),
        sat.engine.len(),
    );
    Ok(buffer.to_string_lossy() + &absents + &clashes + &summary)
}

/// Every hypgen candidate, its verdict, and the stats — the S1a.4.1 diff.
///
/// Three phases, and the split is what keeps the stream readable: saturate
/// with events off, generate with them on at `verbose`, then ask the two
/// generator-backed predicates with them off again. Each [`generate`] call
/// builds its own [`crate::Lookahead`], which compiles every plan and emits a
/// `compile` event per pair, so running the tail with the log open would
/// triple the file for no signal.
///
/// The event stream **is** the observable: candidate order decides `layer_1`'s
/// singleton order and therefore the whole traversal, and which counter a drop
/// lands in is a T1 observable in its own right. So there is no second
/// rendering of the candidate list that would have to agree with this one.
/// What the trailing block adds is what the events do not carry — the
/// `--hyp-stats` report, its `raw == emitted + sum(filtered)` invariant, and
/// two facts about the predicates built on the generator. `COMPLETE`'s `raw=`
/// is the third line's point: the S1.9.E16 short-circuit is invisible in the
/// boolean and visible in how many candidates were built to reach it.
///
/// `closed` runs the auto-closure pass first — the *other* regime the
/// generator is asked in, and the one `--hyp-stats` and the JSON summary use.
/// Without it what this sees is the `(__closed__ R)` facts a puzzle authored
/// or `std.closure` derived, which is also what `solve` sees: both ein.py call
/// sites run `emit_closed` on a **fork**.
pub fn hyp_shape(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Result<String, String> {
    hyp_shape_with(ast, terms, kb, false)
}

/// [`hyp_shape`], optionally after the auto-closure pass.
pub fn hyp_shape_with(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    closed: bool,
) -> Result<String, String> {
    let mut off = crate::events::Events::off();
    let mut newly: Vec<String> = Vec::new();
    if closed {
        let mut s = crate::saturator::Session {
            kb,
            terms,
            ast,
            events: &mut off,
        };
        newly = crate::closed::emit_closed(&mut s)
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|n| repr_str(s.terms.sym(n)))
            .collect();
    }
    {
        let mut s = crate::saturator::Session {
            kb,
            terms,
            ast,
            events: &mut off,
        };
        let mut sat = crate::saturator::Saturator::new(&mut s).map_err(|e| e.to_string())?;
        sat.saturate(&mut s, None, &mut |_| {})
            .map_err(|e| e.to_string())?;
    }

    let buffer = crate::events::Buffer::new();
    let mut events =
        crate::events::Events::to(Box::new(buffer.clone()), crate::events::Level::Verbose);
    let mut stats = crate::hypgen::HypGenStats::new();
    {
        let mut s = crate::saturator::Session {
            kb,
            terms,
            ast,
            events: &mut events,
        };
        crate::hypgen::generate(&mut s, &mut stats, &mut |_| {
            std::ops::ControlFlow::Continue(())
        })
        .map_err(|e| e.to_string())?;
    }

    let mut short = crate::hypgen::HypGenStats::new();
    let mut s = crate::saturator::Session {
        kb,
        terms,
        ast,
        events: &mut off,
    };
    let is_complete =
        crate::hypgen::complete_counted(&mut s, &mut short).map_err(|e| e.to_string())?;
    let open = crate::hypgen::open_hypotheses(&mut s).map_err(|e| e.to_string())?;

    let mut lines = Vec::new();
    if closed {
        lines.push(format!("CLOSED [{}]", newly.join(", ")));
    }
    lines.push("STATS".to_string());
    lines.extend(stats.report_lines());
    lines.push(format!("BALANCE {}", py_bool(stats.balances())));
    lines.push(format!(
        "COMPLETE {} raw={}",
        py_bool(is_complete),
        short.raw
    ));
    lines.push(format!("OPEN {}", open.len()));
    Ok(buffer.to_string_lossy() + &lines.join("\n"))
}

/// The static NAF dependency map over a **saturated** cache — the S1a.4.2 diff.
///
/// Saturating first is not a convenience: most NAF-bearing rules in the Zebra
/// family are activated by facts a rule derives, so their plan does not exist
/// until the enqueue pass has refreshed the cache, and a map taken at load
/// time silently omits exactly the rules the analysis is about.
///
/// The warning texts go out verbatim rather than counted — ein.py raises them
/// through `warnings`, the suite runs under `filterwarnings=["error"]`, and a
/// caller therefore reads the string.
pub fn naf_map(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
) -> Result<String, crate::saturator::SaturateError> {
    let mut events = crate::events::Events::off();
    let mut s = crate::saturator::Session {
        kb,
        terms,
        ast,
        events: &mut events,
    };
    let mut sat = crate::saturator::Saturator::new(&mut s)?;
    sat.saturate(&mut s, None, &mut |_| {})?;
    let deps = crate::naf_deps::compute_naf_map(&sat.engine, s.terms);
    let mut out: Vec<String> = deps
        .iter()
        .map(|d| {
            format!(
                "NAF {} {} derived={} declared={}",
                repr_str(s.terms.sym(d.rule)),
                repr(&PyValue::Tuple(
                    d.activator
                        .iter()
                        .map(|&a| PyValue::Str(s.terms.sym(a).to_string()))
                        .collect()
                )),
                py_list(&d.derived),
                py_list(&d.declared_only),
            )
        })
        .collect();
    let warnings = crate::naf_deps::derived_naf_warnings(&sat.engine, s.terms);
    out.extend(
        warnings
            .iter()
            .map(|w| format!("WARN DerivedNafWarning {w}")),
    );
    out.push(format!(
        "SUMMARY plans={} deps={} warnings={}",
        sat.engine.len(),
        deps.len(),
        warnings.len()
    ));
    Ok(out.join("\n"))
}

/// The Apriori join, the ordering modes and the no-good store — the S1a.4.3
/// diff.
///
/// Pure set arithmetic over a **real** alive set: the open hypotheses of a
/// saturated root, capped at the first 12 by content order so the layer sizes
/// stay bounded (`zebra2`'s 56 alive would make layer 3 about 27 000 sets).
/// The cap costs nothing this is for — `apriori` never inspects a KB, so what
/// is under test is the join, the comparator, the filter and the store, and 12
/// elements exercise all four.
///
/// The no-good workload is a fixed recipe rather than a random one, because
/// the point is that two implementations run *the same* one: every 7th layer-3
/// set, then every 5th layer-2 set, then every 3rd singleton, then the layer-3
/// slice again. That order makes each of the three outcomes happen — a plain
/// insert, an insert that removes stored supersets, and a clause that is
/// itself subsumed and dropped. On `zebra2` it is 15 removals and 32
/// subsumed-drops.
pub fn lattice_shape(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
) -> Result<String, crate::saturator::SaturateError> {
    use crate::apriori::{generate_layer, layer_1, order_candidates};
    use rustc_hash::FxHashSet;

    let mut off = crate::events::Events::off();
    let (n_alive, alive) = {
        let mut s = crate::saturator::Session {
            kb,
            terms,
            ast,
            events: &mut off,
        };
        let mut sat = crate::saturator::Saturator::new(&mut s)?;
        sat.saturate(&mut s, None, &mut |_| {})?;
        let all = crate::hypgen::open_hypotheses(&mut s)
            .map_err(crate::saturator::SaturateError::Compile)?;
        let mut capped: Vec<FactId> = all.iter().copied().collect();
        // determinism-ok: sorted by content immediately, as `sorted(alive)` is.
        capped.sort_by(|&a, &b| s.terms.cmp_fact_semantic(a, b));
        capped.truncate(12);
        (all.len(), capped.into_iter().collect::<FxHashSet<FactId>>())
    };

    let show = |terms: &Terms, sets: &[Vec<FactId>]| -> String {
        let rendered: Vec<String> = sets
            .iter()
            .map(|s| {
                let items: Vec<String> =
                    s.iter().map(|&f| crate::events::sexpr(terms, f)).collect();
                format!("{{{}}}", items.join(" "))
            })
            .collect();
        format!("[{}]", rendered.join(", "))
    };

    let store = kb.nogoods().clone();
    let l1 = layer_1(terms, &alive);
    let l2 = generate_layer(terms, &l1, &alive, &store.read().expect("store"));
    let l3 = generate_layer(terms, &l2, &alive, &store.read().expect("store"));
    let lex = order_candidates(kb, terms, &l2, "lex").expect("lex never errors");
    let scored = order_candidates(kb, terms, &l2, "score-sum")
        .map_err(|e| crate::saturator::SaturateError::Compile(CompileError(e.to_string())))?;
    let mut out = vec![
        format!("ALIVE {n_alive} capped {}", alive.len()),
        format!("LAYER1 {}", show(terms, &l1)),
        format!("LAYER2 {}", show(terms, &l2)),
        format!("LAYER3 {}", show(terms, &l3)),
        format!("ORDER lex {}", show(terms, &lex)),
        format!("ORDER score-sum {}", show(terms, &scored)),
    ];

    let buffer = crate::events::Buffer::new();
    let mut events =
        crate::events::Events::to(Box::new(buffer.clone()), crate::events::Level::Verbose);
    for batch in [&l3, &l2, &l1]
        .into_iter()
        .zip([7usize, 5, 3])
        .map(|(v, step)| v.iter().step_by(step).cloned().collect::<Vec<_>>())
        .chain(std::iter::once(
            l3.iter().step_by(7).cloned().collect::<Vec<_>>(),
        ))
    {
        for c in &batch {
            crate::nogoods::emit_nogood(kb, terms, &mut events, c, 1);
        }
    }

    let mut clauses: Vec<Vec<String>> = store
        .read()
        .expect("store")
        .iter()
        .map(|c| crate::nogoods::clause_repr(terms, c))
        .collect();
    // determinism-ok: the store is a set on both sides and this is its only
    // rendering, sorted here exactly as ein.py sorts it at the same point.
    clauses.sort();
    out.push(format!("STORE {}", clauses.len()));
    out.extend(
        clauses
            .iter()
            .map(|c| format!("  CLAUSE {{{}}}", c.join(" "))),
    );
    let filtered = generate_layer(terms, &l2, &alive, &store.read().expect("store"));
    out.push(format!("FILTERED {}", show(terms, &filtered)));
    Ok(buffer.to_string_lossy() + &out.join("\n"))
}

/// Unsat cores and the ATMS label search — the S1a.4.6 diff.
///
/// Contradictions first — but most corpus files have none, so an op that only
/// explained contradictions would be empty on nearly all of them. It therefore
/// also explains a deterministic sample of *derived* facts (every 5th by
/// content order, capped at 12), which is where the label propagation actually
/// gets exercised, and repeats the sample under a deliberately tight budget to
/// pin where the caps cut.
///
/// `rounds` and `facts_considered` go out alongside the frontier because they
/// say what the search *did*: a port can return the right frontier the wrong
/// way, and this is the module where that is most likely.
///
/// Frontiers go out **sorted**. ein.py's is a `frozenset`, so its own
/// iteration order is not reproducible even run to run, and every display site
/// sorts.
pub fn explain_shape(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    alts: bool,
) -> Result<String, crate::saturator::SaturateError> {
    use crate::explain::{Explanation, ExplanationBudget, explain};

    let mut cfg = kb.program().config.clone().unwrap_or_default();
    cfg.record_alternative_justifications = alts;
    kb.program_mut().config = Some(cfg);

    let mut events = crate::events::Events::off();
    let mut s = crate::saturator::Session {
        kb,
        terms,
        ast,
        events: &mut events,
    };
    let mut sat = crate::saturator::Saturator::new(&mut s)?;
    sat.saturate(&mut s, None, &mut |_| {})?;
    let (kb, terms) = (&*s.kb, &*s.terms);

    let show = |facts: &[FactId]| -> String {
        let mut items: Vec<String> = facts
            .iter()
            .map(|&f| crate::events::sexpr(terms, f))
            .collect();
        items.sort();
        format!("[{}]", items.join(" "))
    };
    let line = |tag: &str, e: &Explanation| -> String {
        let target = e
            .target
            .map_or_else(|| "-".to_string(), |t| crate::events::sexpr(terms, t));
        format!(
            "{tag} {} target={target} exhausted={} rounds={} considered={} {}",
            e.len(),
            py_bool(e.exhausted),
            e.rounds,
            e.facts_considered,
            show(&e.frontier)
        )
    };

    let witnesses: Vec<FactId> = crate::contradiction::detect(kb, terms)
        .iter()
        .map(|c| c.witness())
        .collect();
    let mut core = ein_core::walks::unsat_core(
        kb,
        terms,
        &witnesses,
        ein_core::walks::Justifications::Primary,
    );
    core.sort_unstable();
    core.dedup();
    let scf = crate::explain::smallest_contradiction_frontier(kb, terms, Some(&witnesses));
    let budget = ExplanationBudget::default();
    let tight = ExplanationBudget {
        max_environments: 1,
        max_rounds: 2,
        max_env_size: Some(1),
        max_facts: 10,
    };
    let mut out = vec![
        format!(
            "ALTS {} witnesses={}",
            py_bool(kb.has_alternative_justifications()),
            witnesses.len()
        ),
        format!("CORE {} {}", core.len(), show(&core)),
        format!("SCF {} {}", scf.len(), show(&scf)),
        line("CONTRA", &explain(kb, terms, &witnesses, &budget)),
        // The **multi-target** budget cut, which is the only way to reach
        // `recorded_fallback`'s tie-break: with one target its key never has
        // to break a tie, and `zebra2-bad` offers 126 witnesses.
        line("CONTRA-TIGHT", &explain(kb, terms, &witnesses, &tight)),
        // The fallback's tie-break, reached on purpose. It only decides when
        // two targets tie on core *size*, and on this corpus the smallest tie
        // is won by the same witness whichever way it is broken —
        // `zebra2-bad` has four size-1 cores whose repr-smallest is also the
        // first the detector found. Reversing the witness list separates the
        // two, so dropping the `" ".join(sorted(repr(f)))` half of the key
        // becomes visible instead of being a comment nobody can check.
        line("FALLBACK-REV", &{
            let mut rev = witnesses.clone();
            rev.reverse();
            crate::explain::recorded_fallback(kb, terms, &rev, 0, 0)
        }),
    ];

    // `sorted(…, key=repr)` over the `Fact` dataclass repr, then `[::11][:8]`.
    let mut derived: Vec<(String, FactId)> = kb
        .facts()
        .filter(|&f| {
            kb.primary(f)
                .is_some_and(|p| terms.provs.get(p).kind == ein_core::ProvKind::Rule)
        })
        .map(|f| (repr(&terms.py_fact(f)), f))
        .collect();
    derived.sort();
    let sample: Vec<FactId> = derived
        .iter()
        .step_by(5)
        .take(12)
        .map(|(_, f)| *f)
        .collect();
    for &f in &sample {
        let sexpr = crate::events::sexpr(terms, f);
        out.push(line(
            &format!("EXPLAIN {sexpr}"),
            &explain(kb, terms, &[f], &budget),
        ));
        out.push(line(
            &format!("TIGHT   {sexpr}"),
            &explain(kb, terms, &[f], &tight),
        ));
    }
    out.push(format!(
        "SUMMARY facts={} derived={}",
        kb.n_facts(),
        sample.len()
    ));
    Ok(out.join("\n"))
}

/// `try_commitment_set` over real candidates — the S1a.4.4 diff.
///
/// Bounded the same way [`lattice_shape`] is, and for the same reason: the
/// primitive is what is under test, not the size of the search. Layer-1
/// singletons plus the first six layer-2 sets of an alive frontier capped at 8.
///
/// The last two lines are the **purity** claims, checked rather than asserted:
/// `REPEAT` re-enters the first commitment and compares the whole result
/// against the first call, and `ROOT` reports root's fact count, which every
/// entering must leave alone.
pub fn commit_shape(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    fail_fast: bool,
) -> Result<String, crate::saturator::SaturateError> {
    use crate::apriori::{generate_layer, layer_1};
    use rustc_hash::FxHashSet;

    let mut cfg = kb.program().config.clone().unwrap_or_default();
    cfg.enable_fail_fast_fork = fail_fast;
    kb.program_mut().config = Some(cfg);

    let mut off = crate::events::Events::off();
    let (n_alive, alive) = {
        let mut s = crate::saturator::Session {
            kb,
            terms,
            ast,
            events: &mut off,
        };
        let mut sat = crate::saturator::Saturator::new(&mut s)?;
        sat.saturate(&mut s, None, &mut |_| {})?;
        let all = crate::hypgen::open_hypotheses(&mut s)
            .map_err(crate::saturator::SaturateError::Compile)?;
        let mut capped: Vec<FactId> = all.iter().copied().collect();
        // determinism-ok: sorted by content immediately, as `sorted(alive)` is.
        capped.sort_by(|&a, &b| s.terms.cmp_fact_semantic(a, b));
        capped.truncate(8);
        (all.len(), capped.into_iter().collect::<FxHashSet<FactId>>())
    };
    let l1 = layer_1(terms, &alive);
    let store = kb.nogoods().clone();
    let mut commitments = l1.clone();
    commitments.extend(
        generate_layer(terms, &l1, &alive, &store.read().expect("store"))
            .into_iter()
            .take(6),
    );

    let mut out = vec![format!("ALIVE {n_alive} capped {}", alive.len())];
    let mut first: Option<String> = None;
    for c in &commitments {
        let r = crate::commitment::try_commitment_set(kb, terms, ast, &mut off, c, None)?;
        let text = enter_line(terms, c, &r);
        first.get_or_insert_with(|| text.clone());
        out.push(text);
    }
    if let Some(first) = first {
        let r =
            crate::commitment::try_commitment_set(kb, terms, ast, &mut off, &commitments[0], None)?;
        let again = enter_line(terms, &commitments[0], &r);
        out.push(format!("REPEAT {}", py_bool(again == first)));
    }
    out.push(format!(
        "ROOT facts={} commitments={}",
        kb.n_facts(),
        commitments.len()
    ));
    Ok(out.join("\n"))
}

fn enter_line(terms: &Terms, c: &[FactId], r: &crate::commitment::CommitmentSetResult) -> String {
    let sorted_sexprs = |facts: &[FactId]| -> String {
        let mut items: Vec<String> = facts
            .iter()
            .map(|&f| crate::events::sexpr(terms, f))
            .collect();
        items.sort();
        format!("[{}]", items.join(" "))
    };
    let cs: Vec<String> = c.iter().map(|&f| crate::events::sexpr(terms, f)).collect();
    // The hypothesis writes' provenance, because `branch=0` is a contract and
    // not a placeholder: it is per-commitment context the lattice search does
    // not use, and changing it changes provenance output.
    let prov: Vec<String> = r
        .hypothesis_facts
        .iter()
        .map(|&f| match r.kb.primary(f) {
            None => "-".to_string(),
            Some(p) => {
                let p = terms.provs.get(p);
                format!(
                    "{}:{}",
                    p.kind.as_str(),
                    p.branch.map_or("None".to_string(), |b| b.to_string())
                )
            }
        })
        .collect();
    format!(
        "ENTER {{{}}} kind={} firings={} facts={} core={} hyps={} prov=[{}]",
        cs.join(" "),
        r.kind.as_str(),
        r.firings.len(),
        r.kb.n_facts(),
        sorted_sexprs(&r.unsat_core),
        sorted_sexprs(&r.hypothesis_facts),
        prov.join(" ")
    )
}

/// One whole solve — the phase's own gate — the S1a.4.5 diff.
///
/// The verdict, `k`, `exhausted`, every counter, the proof's shape and the
/// search layer's **event stream**. Both regimes cap `max_enterings` and take
/// `OnBudget::Verdict`, so no file can run away and the abort path is compared
/// rather than avoided.
///
/// The log is written at `verbose` and then **filtered** to `enter` /
/// `nogood` / `writeback` / `warn`. Generating the rest is unavoidable — the
/// writer has a level, not a kind filter — but comparing it is not: a
/// `stop_after = 1` `zebra` solve emits 58 000 events of which 19 are the
/// search layer's, and the other 58 000 are the saturator's, which
/// `saturate_events` already compares one layer down.
///
/// The `n` field is **kept**, and it is the strongest line here: it counts
/// every event including the filtered ones, so two implementations agreeing on
/// it agree on the whole stream, not merely on the part that is printed.
pub fn solve_shape(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    mode: &str,
) -> Result<String, String> {
    use crate::solve::{NoDumper, OnBudget, SolveOptions, solve};

    let exhaustive = mode == "exhaustive";
    if mode == "shuffled" {
        // Traversal-order probe: the verdict is shuffle-invariant, so what
        // this compares is the *traversal*.
        let mut cfg = kb.program().config.clone().unwrap_or_default();
        cfg.lattice_order_seed = Some(7);
        kb.program_mut().config = Some(cfg);
    }
    let opts = SolveOptions {
        stop_after: if exhaustive { None } else { Some(1) },
        max_set_size: 5,
        max_enterings: Some(if exhaustive { 60 } else { 300 }),
        on_budget: OnBudget::Verdict,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let buffer = crate::events::Buffer::new();
    let mut events =
        crate::events::Events::to(Box::new(buffer.clone()), crate::events::Level::Verbose);
    let solved =
        solve(kb, terms, ast, &mut events, &mut NoDumper, &opts).map_err(|e| e.to_string())?;

    let kinds = ["\"enter\"", "\"nogood\"", "\"writeback\"", "\"warn\""];
    let log = buffer.to_string_lossy();
    let log: Vec<&str> = log
        .lines()
        .filter(|l| {
            let head = &l[..l.len().min(24)];
            kinds.iter().any(|k| head.contains(k))
        })
        .collect();

    let s = &solved.stats;
    let mut out = vec![format!(
        "VERDICT {} k={} exhausted={}",
        solved.answer.as_str(),
        s.solution_nodes,
        py_bool(s.exhausted)
    )];
    if let crate::verdict::Answer::Aborted { reason } = &solved.answer {
        out.push(format!("ABORT {reason}"));
    }
    if let crate::verdict::Answer::Verdict(crate::verdict::Verdict::Contradiction { unsat_core }) =
        &solved.answer
    {
        let mut core: Vec<String> = unsat_core
            .iter()
            .map(|&f| crate::events::sexpr(terms, f))
            .collect();
        core.sort();
        out.push(format!("CORE {} [{}]", core.len(), core.join(" ")));
    }
    let b = &s.base;
    out.push(format!(
        "STATS enterings={} alive={} dead_pre={} dead_post={} merged={} \
         forced={} saturate={} layers={} nogoods={}/{}",
        b.enterings_total,
        b.enterings_alive,
        b.enterings_dead_pre,
        b.enterings_dead_post,
        b.facts_merged,
        b.forced_positives,
        b.saturate_count,
        b.layers_explored,
        b.nogoods_emitted,
        b.nogoods_subsumed
    ));
    if let Some(p) = &solved.proof {
        out.push(format!(
            "PROOF solutions={} dead={} alive_at_end={} nogoods={}",
            p.solutions.len(),
            p.dead_commitments.len(),
            p.alive_at_end.len(),
            p.learned_nogoods.len()
        ));
        // The proof's *content*, not only its shape: a solution's commitment
        // (which `record_node`'s lex-smallest rule decides), and each dead
        // commitment with the core that refuted it.
        for r in &p.solutions {
            let cs: Vec<String> = r
                .commitment
                .iter()
                .map(|&f| crate::events::sexpr(terms, f))
                .collect();
            out.push(format!(
                "  SOLUTION {{{}}} layer={} firings={}",
                cs.join(" "),
                r.layer,
                r.firings.len()
            ));
        }
        for d in &p.dead_commitments {
            let cs: Vec<String> = d
                .commitment
                .iter()
                .map(|&f| crate::events::sexpr(terms, f))
                .collect();
            let mut core: Vec<String> = d
                .unsat_core
                .iter()
                .map(|&f| crate::events::sexpr(terms, f))
                .collect();
            core.sort();
            out.push(format!(
                "  DEAD {{{}}} layer={} kind={} core=[{}]",
                cs.join(" "),
                d.layer,
                d.kind.as_str(),
                core.join(" ")
            ));
        }
        let mut clauses: Vec<Vec<String>> = p
            .learned_nogoods
            .iter()
            .map(|c| crate::nogoods::clause_repr(terms, c))
            .collect();
        clauses.sort();
        out.extend(
            clauses
                .iter()
                .map(|c| format!("  CLAUSE {{{}}}", c.join(" "))),
        );
    }
    out.push(format!("EVENTS {}", log.len()));
    out.extend(log.iter().map(|l| format!("  {l}")));
    out.push(format!("ROOT facts={}", kb.n_facts()));
    Ok(out.join("\n"))
}

/// `repr(list_of_str)` — `PyValue` has no list shape, and one is needed in
/// exactly this one place.
fn py_list(items: &[String]) -> String {
    let rendered: Vec<String> = items.iter().map(|s| repr_str(s)).collect();
    format!("[{}]", rendered.join(", "))
}

/// One [`NafRef`] as `repr((relation, args))` — the tuple `world._ground`
/// builds, with `None` where the query ranged free.
fn naf_repr(terms: &Terms, r: &ein_core::NafRef) -> String {
    format!(
        "({}, {})",
        repr_str(terms.sym(r.rel)),
        naf_args_repr(terms, &r.args)
    )
}

fn naf_args_repr(terms: &Terms, args: &[ein_core::NafArg]) -> String {
    let items: Vec<String> = args.iter().map(|a| naf_arg_repr(terms, a)).collect();
    // Python's one-tuple comma: `('x',)` is a tuple, `('x')` is a string.
    if items.len() == 1 {
        format!("({},)", items[0])
    } else {
        format!("({})", items.join(", "))
    }
}

fn naf_arg_repr(terms: &Terms, arg: &ein_core::NafArg) -> String {
    match arg {
        ein_core::NafArg::Free => "None".to_string(),
        ein_core::NafArg::Value(v) => repr(&terms.py_value(*v)),
        ein_core::NafArg::Nested { rel, args } => format!(
            "({}, {})",
            repr_str(terms.sym(*rel)),
            naf_args_repr(terms, args)
        ),
    }
}

fn match_text(terms: &Terms, at: &FxHashMap<FactId, usize>, m: &Match<'_>) -> String {
    let bindings: Vec<String> = m
        .bindings()
        .map(|(name, value)| {
            format!(
                "({}, {})",
                repr_str(terms.sym(name)),
                repr(&terms.py_value(value))
            )
        })
        .collect();
    let premises: Vec<String> = m.premises().iter().map(|f| at[f].to_string()).collect();
    format!("b=[{}] p=[{}]", bindings.join(", "), premises.join(", "))
}

fn render_plan(out: &mut String, ast: &Ast, terms: &Terms, plan: &Plan, key: &[Symbol]) {
    let key_repr = repr(&PyValue::Tuple(
        key.iter()
            .map(|&s| PyValue::Str(terms.sym(s).to_string()))
            .collect(),
    ));
    let args_repr = repr(&PyValue::Tuple(
        plan.activator_args
            .iter()
            .map(|&s| PyValue::Str(terms.sym(s).to_string()))
            .collect(),
    ));
    let why = plan.why.map(|s| terms.sym(s)).unwrap_or("");
    out.push_str(&format!(
        "PLAN {} key={key_repr} args={args_repr} why={}\n",
        terms.sym(plan.rule),
        repr_str(why),
    ));
    for (name, value) in plan.seed.iter() {
        out.push_str(&format!(
            "  SEED {} {}\n",
            terms.sym(*name),
            repr(&terms.py_value(*value))
        ));
    }
    for (i, d) in plan.disjuncts.iter().enumerate() {
        out.push_str(&format!("  D{i} STEPS {}\n", d.steps.len()));
        render_steps(out, ast, terms, plan, &plan.reg_names, d.steps, 2);
        for (j, g) in plan.guards(d.guards).iter().enumerate() {
            render_guard(out, ast, terms, plan, g, i, j);
        }
    }
    for (i, t) in plan.asserts.iter().enumerate() {
        out.push_str(&format!(
            "  ASSERT {i} {}\n",
            slot_text(ast, terms, plan, &plan.reg_names, t)
        ));
    }
    out.push_str(&format!(
        "  ASSERTED {} NEGATED {}\n",
        opt_name(terms, asserted_relation(plan, terms)),
        opt_name(terms, negated_relation(plan, terms)),
    ));
    let refs = naf_relation_refs(plan, terms);
    if !refs.is_empty() {
        let rendered: Vec<String> = refs
            .iter()
            .map(|(r, neg)| format!("({}, {})", repr_str(terms.sym(*r)), py_bool(*neg)))
            .collect();
        out.push_str(&format!("  NAFREFS [{}]\n", rendered.join(", ")));
    }
}

fn render_guard(
    out: &mut String,
    ast: &Ast,
    terms: &Terms,
    plan: &Plan,
    g: &NafGuard,
    disjunct: usize,
    index: usize,
) {
    out.push_str(&format!(
        "  D{disjunct} GUARD {index} scope=({}) watched=({}) monotone={}\n",
        names(terms, &g.scope),
        names(terms, &g.watched),
        py_bool(g.monotone),
    ));
    render_steps(out, ast, terms, plan, &g.reg_names, g.sub, 2);
}

fn render_steps(
    out: &mut String,
    ast: &Ast,
    terms: &Terms,
    plan: &Plan,
    regs: &[Symbol],
    span: Span,
    depth: usize,
) {
    let pad = "  ".repeat(depth);
    for step in plan.steps(span) {
        match step {
            Step::Rel(r) => {
                let slots: Vec<String> = plan
                    .slots(r.slots)
                    .iter()
                    .map(|s| slot_text(ast, terms, plan, regs, s))
                    .collect();
                let kind = if r.join { "JOIN" } else { "SCAN" };
                let shared = if r.join {
                    format!(" shared=({})", names(terms, plan.shared(r.shared)))
                } else {
                    String::new()
                };
                out.push_str(&format!(
                    "{pad}{kind} {}{shared} [{}]\n",
                    terms.sym(r.rel),
                    slots.join(" ")
                ));
            }
            Step::Guard { pred, args } => {
                let rendered: Vec<String> = plan
                    .guard_args(*args)
                    .iter()
                    .map(|a| node_repr(ast, a.node))
                    .collect();
                out.push_str(&format!(
                    "{pad}GUARD {} [{}]\n",
                    pred.as_str(),
                    rendered.join(", ")
                ));
            }
            Step::Absent { sub } => {
                out.push_str(&format!("{pad}ABSENT {}\n", sub.len()));
                render_steps(out, ast, terms, plan, regs, *sub, depth + 1);
            }
        }
    }
}

/// One compiled slot, in the vocabulary the Python renderer uses.
fn slot_text(ast: &Ast, terms: &Terms, plan: &Plan, regs: &[Symbol], slot: &Slot) -> String {
    match slot {
        Slot::Reg(r) => format!("?{}", terms.sym(regs[*r as usize])),
        Slot::Const(v) => repr(&terms.py_value(*v)),
        Slot::Nested { rel, slots } => {
            let mut s = format!("({}", terms.sym(*rel));
            for inner in plan.slots(*slots) {
                s.push(' ');
                s.push_str(&slot_text(ast, terms, plan, regs, inner));
            }
            s.push(')');
            s
        }
        Slot::Opaque(node) => node_repr(ast, *node),
    }
}

fn names(terms: &Terms, syms: &[Symbol]) -> String {
    syms.iter()
        .map(|&s| terms.sym(s))
        .collect::<Vec<_>>()
        .join(" ")
}

fn opt_name(terms: &Terms, sym: Option<Symbol>) -> String {
    match sym {
        Some(s) => repr_str(terms.sym(s)),
        None => "None".to_string(),
    }
}

fn py_bool(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

/// A register's name, for the unbound-`:assert`-var message and for tests.
pub fn reg_name<'a>(terms: &'a Terms, plan: &Plan, reg: crate::plan::Reg) -> &'a str {
    terms.sym(plan.reg_names[reg as usize])
}

/// A guard argument as ein.py stores it — the raw IR node.
pub fn guard_arg_text(ast: &Ast, node: ein_ir::NodeId, _kind: GuardArgKind) -> String {
    node_repr(ast, node)
}
