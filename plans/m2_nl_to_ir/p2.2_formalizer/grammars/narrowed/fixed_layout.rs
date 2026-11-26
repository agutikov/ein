//! Q-M2.30's measurement: the narrowed root with a fixed layout, written and not adopted. An EinAf test, run once
//! as `einaf/tests/zz_qm230_tmp.rs` on 23d582aa and removed:
//!
//!     QM230_OUT=fixed-layout.gbnf cargo test --manifest-path einaf/Cargo.toml --profile gate \
//!         --test zz_qm230_tmp -- --nocapture
//!
//! `ein.y`'s rules reachable from the contract's forms compiled as the root's are (`Phrase::compile_from`, each
//! rule named `m-…`), but in `Flavor::Model`, its `sp` one space and its `sp0` none; the forms a line break apart;
//! no comment; the root's name, variable and string (`root_lexical`'s `m-tok-…`). It prints the sizes beside the
//! narrowed root's as a request carries it (`einaf::grammar::root_text`), and writes the grammar to `$QM230_OUT`.
use einaf::grammar::gbnf::{Compiled, Expr, Grammar, Rule};
use einaf::grammar::whole::{self, Ar, Atom, Flavor, Kind, Phrase};

fn sub(e: &Expr) -> Expr {
    match e {
        Expr::Ref(n) if n == "m-sp" => Expr::lit(" "),
        Expr::Ref(n) if n == "m-sp0" => Expr::Empty,
        Expr::Seq(v) => Expr::seq(v.iter().map(sub).collect()),
        Expr::Alt(v) => Expr::alt(v.iter().map(sub).collect()),
        Expr::Star(x) => Expr::star(sub(x)),
        Expr::Plus(x) => Expr::plus(sub(x)),
        Expr::Opt(x) => Expr::opt(sub(x)),
        Expr::Times { expr, min, max } => Expr::times(sub(expr), *min, *max),
        o => o.clone(),
    }
}

#[test]
fn measure() {
    let lx = whole::lexical(whole::EIN_L).unwrap();
    let ph = Phrase::read(whole::EIN_Y, &lx.literals).unwrap();
    let forms = ["relation", "rule", "query", "solve", "import", "not-fact", "fact", "open-fact"];
    let contract = Ar::plus(Ar::Alt(
        forms.iter().map(|f| Ar::Atom(Atom::Rule(f.to_string(), Kind::Paren, Kind::Paren))).collect(),
    ));
    let mut g = Grammar::default();
    g.push(Rule::new(
        "root",
        Expr::seq(vec![
            Expr::r("m-form"),
            Expr::star(Expr::seq(vec![Expr::lit("\n"), Expr::r("m-form")])),
        ]),
    ));
    g.push(Rule::new(
        "m-form",
        Expr::alt(forms.iter().map(|f| Expr::r(&format!("m-{f}"))).collect()),
    ));
    for mut r in ph.compile_from(std::slice::from_ref(&contract), &Flavor::Model) {
        r.name = format!("m-{}", r.name);
        r.body = sub(&r.body.renamed(&|n| format!("m-{n}")));
        r.doc.clear();
        g.push(r);
    }
    for r in whole::root_lexical(&lx).unwrap() {
        if r.name.starts_with("m-tok-") {
            let mut r = r;
            r.doc.clear();
            g.push(r);
        }
    }
    let fixed = g.reachable("root", Vec::new()).text();
    let now = einaf::grammar::root_text();
    let strip = |t: &str| t.lines().filter(|l| !l.starts_with('#')).collect::<Vec<_>>().join("\n");
    let (cf, cn) = (Compiled::parse(&fixed).unwrap(), Compiled::parse(&now).unwrap());
    let count = |t: &str| t.lines().filter(|l| l.contains(" ::= ")).count();
    println!(
        "fixed layout: {} bytes ({} without comments), {} rules, llama's {} rules, {} elements",
        fixed.len(), strip(&fixed).len(), count(&fixed), cf.rules(), cf.elements()
    );
    println!(
        "the root now: {} bytes ({} without comments), {} rules, llama's {} rules, {} elements",
        now.len(), strip(&now).len(), count(&now), cn.rules(), cn.elements()
    );
    let ok = |t: &str| cf.accepts("root", t).unwrap().accepted();
    assert!(ok("(relation p T :why \"{?3} is p\")\n(p a b)"));
    std::fs::write(std::env::var("QM230_OUT").unwrap(), &fixed).unwrap();
}
