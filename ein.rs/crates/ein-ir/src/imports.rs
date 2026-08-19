//! Import resolution — `kb/imports.py`, flatten-then-load (A1 D8).
//!
//! `(import M [:as A | :symbols (S…)])` is resolved at the **form** level,
//! *before* the loader runs: each import is replaced in place by the module's
//! resolved form list, qualified per tier —
//!
//! | form | effect |
//! |---|---|
//! | `(import std.macro)` | every defined name prefixed `std.macro.` |
//! | `(import std.macro :as m)` | prefixed `m.` |
//! | `(import std.macro :symbols (forall))` | the listed declarations, flat and unrenamed |
//!
//! so "merge" is list concatenation and conflict detection is the loader's
//! existing duplicate-name guard. Resolution is recursive, bottom-up and
//! re-qualified under the outer namespace (D6), with cycle detection: a
//! *qualified* diamond never collides because re-qualification gives each path
//! its own prefix (`B.D.x` ≠ `C.D.x`), while a *flat* one collides into a
//! duplicate-name error — the intended strict policy (D3).
//!
//! This module is the engine's **only** filesystem access, which is what keeps
//! any later policy on what the engine may read a single seam rather than an
//! audit: everything goes through [`Resolver`], and the stdlib half already
//! does ([`crate::stdlib`]).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::ast::{Ast, Node, NodeId, loc_repr};
use crate::parse::{ParseError, parse};
use crate::stdlib;

const MODULE_SEP: char = '.';
const STDLIB_ALIAS: &str = "std";

/// The four heads that *bind* a name.
const DECLARATORS: [&str; 4] = ["rule", "hrule", "relation", "macro"];

/// Names a declaration may not bind — kernel vocabulary
/// (`from_ir._reserved_names()`: the structural primitives `absent`/`and`/
/// `false`/`not`/`or`, the computed predicates `eq`/`neq`, and `relation`,
/// which stays a plain `SYMBOL` so rules can match `(relation ?R ?A ?B)`).
/// `open` / `forall` are deliberately **not** here — they migrated into
/// `std.macro` (S1.5.9).
///
/// A literal list because the registries it mirrors are Python objects;
/// [P1a.3](../../../../plans/m1a_rust/p1a.3_deductive_core/README.md) brings
/// the primitive and predicate registries over and this becomes a query
/// against them.
const RESERVED_NAMES: [&str; 8] = [
    "absent", "and", "eq", "false", "neq", "not", "or", "relation",
];

/// A load-time failure. The message is `KBLoadError`'s, byte for byte — the
/// `examples/broken/load/*.expected` fixtures are the gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadError(pub String);

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for LoadError {}

impl From<ParseError> for LoadError {
    /// A module that does not parse surfaces as its own parse error; the
    /// loader lets `IRParseError` through rather than wrapping it.
    fn from(e: ParseError) -> Self {
        LoadError(e.to_string())
    }
}

/// Parsed module forms, by resolved path — [T1a.6.5.3](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.5_frontend.md).
///
/// A resolution is a *tree*, and the corpus's trees are diamonds: `zebra2`
/// imports `std.algebra` and `std.bijection`, `std.bijection` imports
/// `std.algebra`, and all three import `std.macro`. Parsing each module once
/// per *edge* meant a load parsed **3.3× the bytes on disk** (`parse_bytes`
/// against the file's own length, [`ein_core::counters`]).
///
/// The cache lives for one resolution and holds [`NodeId`]s, which are indices
/// into the `Ast` that resolution is building — so it is threaded through the
/// recursion rather than kept on the [`Resolver`], and cannot outlive the arena
/// its contents name.
///
/// Reusing the forms is sound because nothing downstream mutates a node:
/// `qualify` rewrites by *building* (`rename_atoms`), `select` filters, and
/// `dedup_declarations` compares with [`Ast::eq_nodes`], which is structural.
/// Two importers already shared subtrees before this — `rename_atoms` returns
/// the node it was given when no name in it is renamed.
type ModuleCache = BTreeMap<String, Vec<NodeId>>;

/// Where a module's text may come from. The stdlib half can be compiled into
/// the binary; the file-relative half never is.
enum Root {
    Dir(PathBuf),
    Embedded,
}

/// Import resolution with an explicit stdlib source.
///
/// Carrying the source rather than resolving it per call is what lets a test
/// point at a fixture tree, and what will let `--sandbox` refuse the
/// filesystem in one place.
pub struct Resolver {
    stdlib: stdlib::Source,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            stdlib: stdlib::resolve_default(),
        }
    }
}

impl Resolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_stdlib(stdlib: stdlib::Source) -> Self {
        Resolver { stdlib }
    }

    fn stdlib_root(&self) -> Root {
        match &self.stdlib {
            stdlib::Source::Override(p) | stdlib::Source::Checkout(p) => Root::Dir(p.clone()),
            stdlib::Source::Embedded => Root::Embedded,
        }
    }

    /// The pattern-macro names `std.macro` exports (`forall` / `open`), read
    /// from the module through the resolution chain rather than hardcoded —
    /// so the S1.8a.f20 guard consults the library that is actually loaded.
    ///
    /// Empty when `std.macro` is unreadable: degrade to no check, as ein.py
    /// does.
    pub fn stdlib_macro_names(&self) -> Vec<String> {
        let Some(text) = self.read(&self.stdlib_root(), &["macro"]).1 else {
            return Vec::new();
        };
        let mut ast = Ast::new();
        let Ok(forms) = parse(&mut ast, &text, Some("macro.ein")) else {
            return Vec::new();
        };
        let mut out: Vec<String> = forms
            .iter()
            .filter(|f| ast.head_name(**f) == Some("macro"))
            .filter_map(|f| ast.form_args(*f).first().and_then(|a| ast.atom_name(*a)))
            .map(str::to_string)
            .collect();
        out.sort();
        out.dedup();
        out
    }

    /// `(root.joinpath(*rel).with_suffix(".ein"), its text if it is a file)`.
    ///
    /// The display path is what "module not found at …" names, so it is built
    /// the way `pathlib` builds it: empty segments dropped, then the final
    /// component's extension replaced.
    fn read(&self, root: &Root, rel: &[&str]) -> (String, Option<String>) {
        match root {
            Root::Dir(dir) => {
                let mut path = dir.clone();
                for seg in rel.iter().filter(|s| !s.is_empty()) {
                    path.push(seg);
                }
                path.set_extension("ein");
                let display = path.display().to_string();
                let text = path
                    .is_file()
                    .then(|| std::fs::read_to_string(&path).ok())
                    .flatten();
                (display, text)
            }
            Root::Embedded => {
                let joined: Vec<&str> = rel.iter().copied().filter(|s| !s.is_empty()).collect();
                let name = format!("{}.ein", joined.join("/"));
                // ein.py has no embedded case, so this path's *message* has no
                // oracle; the harness always sets `$EIN_STDLIB`.
                (format!("<embedded>/{name}"), self.stdlib.read(&name))
            }
        }
    }

    /// Return `forms` with every `(import …)` replaced in place by the
    /// imported module's resolved, qualified forms. Import-free input is
    /// returned unchanged — the common case, and the one where `load()`'s own
    /// inline-duplicate detection must still see the original list.
    pub fn resolve_imports(
        &self,
        ast: &mut Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
    ) -> Result<Vec<NodeId>, LoadError> {
        self.resolve(ast, forms, base_dir, &mut Vec::new(), &mut ModuleCache::new())
    }

    fn resolve(
        &self,
        ast: &mut Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
        loading: &mut Vec<String>,
        cache: &mut ModuleCache,
    ) -> Result<Vec<NodeId>, LoadError> {
        let mut out: Vec<NodeId> = Vec::new();
        let mut had_import = false;
        for &form in forms {
            if ast.head_name(form) != Some("import") {
                out.push(form);
                continue;
            }
            had_import = true;
            let spec = import_spec(ast, form)?;
            let loc = loc_repr(ast, ast.loc(form));
            let (key, text, dir) = self.locate(&spec.module, base_dir, &loc)?;
            if loading.contains(&key) {
                let mut chain = loading.clone();
                chain.push(key);
                return Err(LoadError(format!(
                    "import cycle: {} (at {loc})",
                    chain.join(" -> ")
                )));
            }
            // Parsed once per resolution, not once per edge.
            let sub = match cache.get(&key) {
                Some(sub) => sub.clone(),
                None => {
                    let sub = parse(ast, &text, Some(&key))?;
                    cache.insert(key.clone(), sub.clone());
                    sub
                }
            };
            loading.push(key);
            let resolved = self.resolve(ast, &sub, dir.as_deref(), loading, cache)?;
            loading.pop();
            match &spec.symbols {
                Some(symbols) => out.extend(select(ast, &resolved, symbols, &spec.module, &loc)?),
                None => {
                    let prefix = format!(
                        "{}{MODULE_SEP}",
                        spec.alias.as_deref().unwrap_or(&spec.module)
                    );
                    out.extend(qualify(ast, &resolved, &prefix));
                }
            }
        }
        // Collapse diamonds only when imports were actually spliced.
        if had_import {
            dedup_declarations(ast, &out)
        } else {
            Ok(out)
        }
    }

    /// Logical module name → `(identity, text, base_dir for its own imports)`.
    ///
    /// `std.x.y` resolves under the stdlib root; anything else file-relative
    /// to the importing file. The identity is the **resolved** path, because
    /// that is what the cycle stack compares and what the imported forms'
    /// `Loc`s name.
    fn locate(
        &self,
        module: &str,
        base_dir: Option<&Path>,
        loc: &str,
    ) -> Result<(String, String, Option<PathBuf>), LoadError> {
        let segments: Vec<&str> = module.split(MODULE_SEP).collect();
        let (root, rel) = if segments[0] == STDLIB_ALIAS {
            let rel = &segments[1..];
            if rel.is_empty() {
                return Err(LoadError(format!(
                    "(import {module}) — bare '{STDLIB_ALIAS}' is not a module at {loc}"
                )));
            }
            (self.stdlib_root(), rel.to_vec())
        } else {
            let Some(dir) = base_dir else {
                return Err(LoadError(format!(
                    "(import {module}) — file-relative import needs a base directory \
                     (load from a file path) at {loc}"
                )));
            };
            (Root::Dir(dir.to_path_buf()), segments.clone())
        };
        let (display, text) = self.read(&root, &rel);
        let Some(text) = text else {
            return Err(LoadError(format!(
                "(import {module}) — module not found at {display} ({loc})"
            )));
        };
        // `Path.resolve()` — the identity, so two spellings of one file are
        // one node in the cycle graph.
        let resolved = std::fs::canonicalize(&display).ok();
        let key = resolved
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| display.clone());
        let dir = resolved.and_then(|p| p.parent().map(Path::to_path_buf));
        Ok((key, text, dir))
    }

    /// Resolve every import inline, then **tree-shake**: drop any imported
    /// *declaration* nothing references (A1 D9).
    ///
    /// Reachability seeds from the puzzle's own forms and closes over two
    /// coupled relations — **name reference** (a kept form mentioning a
    /// declaration keeps it, transitively) and **activation** (an imported
    /// rule whose `:match` references a *live* relation is kept even when its
    /// own name is referenced nowhere, because the `*-setup` glue rules are
    /// fired by their match pattern rather than by name).
    ///
    /// Without the activation pass this would silently drop an entire
    /// activator-driven library and leave a file that no longer solves. The
    /// surviving set is observable through `len(engine.cache)` and through
    /// firing order, so it is a T1/T2 surface, not a cosmetic one.
    pub fn resolve_and_minimize(
        &self,
        ast: &mut Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
    ) -> Result<Vec<NodeId>, LoadError> {
        // `(form, is_imported)` — the puzzle's own forms are false, everything
        // an import brings in (transitively) is true.
        let mut tagged: Vec<(NodeId, bool)> = Vec::new();
        let mut cache = ModuleCache::new();
        for &form in forms {
            if ast.head_name(form) == Some("import") {
                let resolved =
                    self.resolve(ast, &[form], base_dir, &mut Vec::new(), &mut cache)?;
                for f in resolved {
                    tagged.push((f, true));
                }
            } else {
                tagged.push((form, false));
            }
        }

        // Imported declarations, first position wins, last body wins — a
        // Python dict comprehension's assignment semantics.
        let mut imported_decls: Vec<(String, NodeId)> = Vec::new();
        let mut index: BTreeMap<String, usize> = BTreeMap::new();
        for &(f, imp) in &tagged {
            if !imp {
                continue;
            }
            let Some(name) = decl_name(ast, f) else {
                continue;
            };
            match index.get(&name) {
                Some(&i) => imported_decls[i].1 = f,
                None => {
                    index.insert(name.clone(), imported_decls.len());
                    imported_decls.push((name, f));
                }
            }
        }

        // Each imported rule's match / assert relation heads, for activation.
        let mut match_heads: Vec<(String, BTreeSet<String>)> = Vec::new();
        let mut assert_heads: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (name, f) in &imported_decls {
            if !matches!(ast.head_name(*f), Some("rule") | Some("hrule")) {
                continue;
            }
            match_heads.push((name.clone(), kw_heads(ast, *f, "match")));
            assert_heads.insert(name.clone(), kw_heads(ast, *f, "assert"));
        }

        // Live relations: heads of every kept fact, and the asserts of the
        // puzzle's own (always-kept) rules — what can exist in the saturated
        // KB and so trigger an activator-driven imported rule.
        let mut live: BTreeSet<String> = BTreeSet::new();
        for &(f, imp) in &tagged {
            if decl_name(ast, f).is_none() {
                sform_head_names(ast, f, &mut live);
            } else if !imp && matches!(ast.head_name(f), Some("rule") | Some("hrule")) {
                live.extend(kw_heads(ast, f, "assert"));
            }
        }

        let mut reachable: BTreeSet<String> = BTreeSet::new();
        let mut work: Vec<String> = Vec::new();
        for &(f, imp) in &tagged {
            if !imp || decl_name(ast, f).is_none() {
                let mut names = BTreeSet::new();
                referenced_names(ast, f, &mut names);
                work.extend(names);
            }
        }

        let mut changed = true;
        while changed {
            changed = false;
            while let Some(n) = work.pop() {
                if reachable.contains(&n) {
                    continue;
                }
                let Some(&i) = index.get(&n) else { continue };
                reachable.insert(n.clone());
                changed = true;
                let mut names = BTreeSet::new();
                referenced_names(ast, imported_decls[i].1, &mut names);
                work.extend(names);
                if let Some(heads) = assert_heads.get(&n) {
                    live.extend(heads.iter().cloned());
                }
            }
            for (name, heads) in &match_heads {
                if !reachable.contains(name) && heads.iter().any(|h| live.contains(h)) {
                    work.push(name.clone());
                    changed = true;
                }
            }
        }

        let kept: Vec<NodeId> = tagged
            .iter()
            .filter(|&&(f, imp)| !imp || decl_name(ast, f).is_none_or(|n| reachable.contains(&n)))
            .map(|&(f, _)| f)
            .collect();
        dedup_declarations(ast, &kept)
    }
}

// ── Import spec ────────────────────────────────────────────────────

struct ImportSpec {
    module: String,
    alias: Option<String>,
    symbols: Option<Vec<String>>,
}

/// `(module, alias, symbols)` for an `(import …)` form. Exactly one of
/// `alias` / `symbols` is set, or neither (whole-module).
fn import_spec(ast: &Ast, form: NodeId) -> Result<ImportSpec, LoadError> {
    let loc = loc_repr(ast, ast.loc(form));
    let args = ast.form_args(form).to_vec();
    let Some(module) = args
        .first()
        .and_then(|a| ast.atom_name(*a))
        .map(str::to_string)
    else {
        return Err(LoadError(format!(
            "malformed (import …) — missing module name at {loc}"
        )));
    };
    // Last wins on a repeated key — a Python dict comprehension's semantics.
    let mut kws: BTreeMap<String, NodeId> = BTreeMap::new();
    for &a in &args {
        if let Node::KwPair { key, value } = ast.node(a)
            && let Node::Keyword(s) = ast.node(key)
        {
            kws.insert(ast.sym(s).to_string(), value);
        }
    }
    let as_value = kws.get("as").copied();
    let symbols_value = kws.get("symbols").copied();
    if as_value.is_some() && symbols_value.is_some() {
        return Err(LoadError(format!(
            "(import {module}) — :as and :symbols are mutually exclusive at {loc}"
        )));
    }
    let alias = match as_value {
        Some(v) => match ast.atom_name(v) {
            Some(name) => Some(name.to_string()),
            None => {
                return Err(LoadError(format!(
                    "(import {module} :as …) — alias must be a bare name at {loc}"
                )));
            }
        },
        None => None,
    };
    let symbols = match symbols_value {
        Some(v) => Some(symbol_list(ast, v, &module, &loc)?),
        None => None,
    };
    Ok(ImportSpec {
        module,
        alias,
        symbols,
    })
}

/// Names inside a `:symbols (a b …)` list — the list lowers to a form whose
/// head and atom args are the names.
fn symbol_list(
    ast: &Ast,
    value: NodeId,
    module: &str,
    loc: &str,
) -> Result<Vec<String>, LoadError> {
    let Node::SForm { head, args } = ast.node(value) else {
        return Err(LoadError(format!(
            "(import {module} :symbols …) — expected a (name …) list at {loc}"
        )));
    };
    let mut names: Vec<String> = Vec::new();
    if let Some(name) = ast.atom_name(head)
        && !name.starts_with('@')
    {
        names.push(name.to_string());
    }
    names.extend(
        ast.args(args)
            .iter()
            .filter_map(|a| ast.atom_name(*a))
            .map(str::to_string),
    );
    if names.is_empty() {
        return Err(LoadError(format!(
            "(import {module} :symbols ()) — empty list at {loc}"
        )));
    }
    Ok(names)
}

// ── Qualification and selection ────────────────────────────────────

/// The name a declarator form binds, or `None` for a fact.
fn decl_name(ast: &Ast, form: NodeId) -> Option<String> {
    let head = ast.head_name(form)?;
    if !DECLARATORS.contains(&head) {
        return None;
    }
    ast.form_args(form)
        .first()
        .and_then(|a| ast.atom_name(*a))
        .map(str::to_string)
}

/// Names a form list *declares*.
fn defined_names(ast: &Ast, forms: &[NodeId]) -> BTreeSet<String> {
    forms.iter().filter_map(|f| decl_name(ast, *f)).collect()
}

/// Prefix every defined name — and every reference to it — leaving reserved
/// kernel vocabulary alone, so a module that illegally defines `absent` keeps
/// the name and is rejected by the loader rather than silently renamed.
fn qualify(ast: &mut Ast, forms: &[NodeId], prefix: &str) -> Vec<NodeId> {
    let mapping: BTreeMap<String, String> = defined_names(ast, forms)
        .into_iter()
        .filter(|n| !RESERVED_NAMES.contains(&n.as_str()))
        .map(|n| (format!("{prefix}{n}"), n))
        .map(|(v, k)| (k, v))
        .collect();
    if mapping.is_empty() {
        return forms.to_vec();
    }
    forms
        .iter()
        .map(|f| rename_atoms(ast, *f, &mapping))
        .collect()
}

/// Rewrite every `Atom` whose name is in `mapping` — head, args and kw-pair
/// values alike, so references and `:rule` provenance refs follow the rename.
fn rename_atoms(ast: &mut Ast, node: NodeId, mapping: &BTreeMap<String, String>) -> NodeId {
    match ast.node(node) {
        Node::Atom(s) => match mapping.get(ast.sym(s)) {
            Some(renamed) => {
                let renamed = renamed.clone();
                let loc = ast.loc(node);
                ast.atom(&renamed, loc)
            }
            None => node,
        },
        Node::SForm { head, args } => {
            let head = rename_atoms(ast, head, mapping);
            let args: Vec<NodeId> = ast.args(args).to_vec();
            let new_args: Vec<NodeId> = args
                .into_iter()
                .map(|a| rename_atoms(ast, a, mapping))
                .collect();
            let loc = ast.loc(node);
            ast.sform(head, &new_args, loc)
        }
        Node::KwPair { key, value } => {
            let value = rename_atoms(ast, value, mapping);
            let loc = ast.loc(node);
            ast.push(Node::KwPair { key, value }, loc)
        }
        _ => node,
    }
}

/// Keep the listed names **plus their dependency closure**, flat and
/// unrenamed.
///
/// Auto-closure (S1.8a.f20): a listed declaration drags in every *other*
/// declaration of this module it references, so importing an entry rule pulls
/// the machinery it asserts and matches without the importer enumerating it.
/// Names referenced but not declared here — cross-module deps, kernel
/// primitives — are left for the importer's other imports, so a module need
/// not be self-contained. A listed name the module does not declare is an
/// error: there is no re-export of the absent.
fn select(
    ast: &Ast,
    forms: &[NodeId],
    symbols: &[String],
    module: &str,
    loc: &str,
) -> Result<Vec<NodeId>, LoadError> {
    let mut decls: BTreeMap<String, NodeId> = BTreeMap::new();
    for &f in forms {
        if let Some(n) = decl_name(ast, f) {
            decls.insert(n, f); // last wins
        }
    }
    let missing: Vec<&String> = symbols.iter().filter(|s| !decls.contains_key(*s)).collect();
    if !missing.is_empty() {
        let mut names: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
        names.sort_unstable();
        names.dedup();
        return Err(LoadError(format!(
            "(import {module} :symbols …) — not provided by the module: {} at {loc}",
            names.join(", ")
        )));
    }
    let mut keep: BTreeSet<String> = BTreeSet::new();
    let mut work: Vec<String> = symbols.to_vec();
    // `wanted` is a set in ein.py, so a repeated symbol is one item; the
    // closure makes that immaterial, but the stack starts the same way.
    work.sort();
    work.dedup();
    while let Some(n) = work.pop() {
        if keep.contains(&n) {
            continue;
        }
        let Some(&f) = decls.get(&n) else { continue };
        keep.insert(n);
        let mut names = BTreeSet::new();
        referenced_names(ast, f, &mut names);
        work.extend(names);
    }
    Ok(forms
        .iter()
        .copied()
        .filter(|f| decl_name(ast, *f).is_some_and(|n| keep.contains(&n)))
        .collect())
}

/// Drop repeated **identical** declarations, keeping the first.
///
/// Import is idempotent: a module and its importer — or two modules — may both
/// pull the same shared dependency, and the diamond must collapse rather than
/// trip a duplicate-name error. A second declaration of the same name *and
/// kind* with a different body is a genuine conflict.
///
/// The key is `(kind, name)`, not the name alone: a `(rule undefeated …)` and
/// a `(relation undefeated …)` share a name but are distinct declarations — a
/// rule that produces a same-named relation is idiomatic.
fn dedup_declarations(ast: &Ast, forms: &[NodeId]) -> Result<Vec<NodeId>, LoadError> {
    let mut seen: BTreeMap<(String, String), NodeId> = BTreeMap::new();
    let mut out: Vec<NodeId> = Vec::new();
    for &f in forms {
        let Some(name) = decl_name(ast, f) else {
            out.push(f);
            continue;
        };
        let kind = ast
            .head_name(f)
            .expect("a declarator has an atom head")
            .to_string();
        match seen.get(&(kind.clone(), name.clone())) {
            None => {
                seen.insert((kind, name), f);
                out.push(f);
            }
            Some(&prev) if ast.eq_nodes(prev, f) => {} // identical re-import
            Some(_) => {
                return Err(LoadError(format!(
                    "conflicting definitions of '{name}': same name, different body (at {})",
                    loc_repr(ast, ast.loc(f))
                )));
            }
        }
    }
    Ok(out)
}

// ── Reachability helpers ───────────────────────────────────────────

/// Every `Atom` name reachable from `node` — heads (so a macro invocation
/// `(forall …)` counts as referencing `forall`), args and kw-pair values.
fn referenced_names(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
    match ast.node(node) {
        Node::Atom(s) => {
            out.insert(ast.sym(s).to_string());
        }
        Node::SForm { head, args } => {
            if let Some(name) = ast.atom_name(head) {
                out.insert(name.to_string());
            }
            for a in ast.args(args) {
                referenced_names(ast, *a, out);
            }
        }
        Node::KwPair { value, .. } => referenced_names(ast, value, out),
        _ => {}
    }
}

/// The `Atom` head-name of every form reachable from `node` — relation heads
/// and logical connectives alike. Variable heads contribute nothing; callers
/// intersect against the sets they care about, so stray connective names are
/// harmless.
fn sform_head_names(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
    if let Node::SForm { head, args } = ast.node(node) {
        if let Some(name) = ast.atom_name(head) {
            out.insert(name.to_string());
        }
        for a in ast.args(args) {
            sform_head_names(ast, *a, out);
        }
    }
}

/// The form heads inside a rule's `:match` / `:assert` value.
fn kw_heads(ast: &Ast, form: NodeId, key: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for &a in ast.form_args(form) {
        if let Node::KwPair { key: k, value } = ast.node(a)
            && let Node::Keyword(s) = ast.node(k)
            && ast.sym(s) == key
        {
            sform_head_names(ast, value, &mut out);
            return out;
        }
    }
    out
}

// ── Convenience wrappers ───────────────────────────────────────────

/// [`Resolver::resolve_imports`] with the default stdlib source.
pub fn resolve_imports(
    ast: &mut Ast,
    forms: &[NodeId],
    base_dir: Option<&Path>,
) -> Result<Vec<NodeId>, LoadError> {
    Resolver::new().resolve_imports(ast, forms, base_dir)
}

/// [`Resolver::resolve_and_minimize`] with the default stdlib source.
pub fn resolve_and_minimize(
    ast: &mut Ast,
    forms: &[NodeId],
    base_dir: Option<&Path>,
) -> Result<Vec<NodeId>, LoadError> {
    Resolver::new().resolve_and_minimize(ast, forms, base_dir)
}
