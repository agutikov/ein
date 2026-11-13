//! Import resolution — flatten-then-load (A1 D8).
//!
//! `(import M [:as A | :symbols (S…)])` is resolved at the **form** level,
//! *before* the loader runs: each import is replaced in place by the module's
//! resolved form list, qualified per tier —
//!
//! | form | effect |
//! |---|---|
//! | `(import std.macro)` | every defined name prefixed `std.macro.` |
//! | `(import std.macro :as m)` | prefixed `m.` |
//! | `(import std.macro :symbols (forall))` | the listed declarations and their closure, flat and unrenamed, and the module's facts of every relation the closure names |
//!
//! **A flat import brings a module's facts** since M1f S1f.6.11 (W88, the
//! user's ruling of 2026-11-11, *"importing facts is mandatory"*): a listed
//! name selects the module's declaration of it, if any, and every top-level
//! fact whose relation it is, unrenamed and in the module's order
//! (`select`). Until then a selection kept declarations alone, so a library
//! of facts, an ontology, arrived empty.
//!
//! **No import brings a module's `(query …)`, `(solve …)`, `(config …)` or
//! `(trace …)`** since M1f S1f.6.25 (E-8, W90): those are a *program's*, what
//! it asks and how it is solved and configured, and a module is imported for
//! its declarations and its facts. A flat import never brought one; a
//! qualified one kept every form, so the importer was solved under the
//! module's lattice, configured by its flags and asked its queries, and two
//! qualified imports of a module with a `(solve …)` were refused, *2 (solve …)
//! forms*.
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
//! does ([`crate::stdlib`]). A module on disk is read through
//! [`crate::read::read_program`], the read every command makes of the program
//! it is given (M1f S1f.5.22): a module that exists and cannot be read as text
//! is refused with that read's line, naming the module's path, where until
//! then it was `module not found at … (None)` (the close review's finding 2,
//! W74).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::ast::{Ast, Node, NodeId, loc_repr};
use crate::grammar::{ParseError, parse};
use crate::read;
use crate::stdlib;

const MODULE_SEP: char = '.';
const STDLIB_ALIAS: &str = "std";

/// The four heads that *bind* a name.
const DECLARATORS: [&str; 4] = ["rule", "hrule", "relation", "macro"];

/// The heads of the top-level forms that are neither a declaration nor a
/// fact: what a program asks, how it is solved and configured, an engine's
/// trace, and an import. The loader's own dispatch (`from_ir`'s
/// `load_query_with`) reads every other head as a fact's relation.
const DIRECTIVES: [&str; 5] = ["query", "solve", "config", "trace", "import"];

/// **A selection that brought a relation's declaration and no rule that
/// derives it** — M1f S1f.6.25, E-20 of P1f.6's `findings.md`. A diagnostic,
/// never a refusal: the closure rule is unchanged, and a selection is a
/// selection.
///
/// The case it is for: a theory built on a theory. `upper` imports `lower`
/// by `lower`'s entry and a program imports `upper` by `upper`'s; the closure
/// of that selection follows the names `upper`'s rules write, so it reaches
/// the declarations of `lower`'s relations and, unless `upper`'s entry names
/// `lower`'s, none of `lower`'s rules. No request is then answered and the
/// lower theory's relations are open, far from the cause.
///
/// **The condition** (chosen on the census in the stage's record): the
/// relation's declaration came through the selection's *closure*, its name
/// not listed (a listed relation is asked for on purpose); the module, with
/// what it imports, holds a rule or an hrule whose `:assert` writes the
/// relation; the selection keeps none of them; and the program as resolved
/// holds no rule that writes it either, by another import or its own. A
/// library of facts has no such rule and is never named.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftBehind {
    /// `<file>:<line>:<col>` of the import form, or empty where the parser
    /// kept no position for it.
    pub at: String,
    /// The import form, as written.
    pub import: String,
    /// The module the form names.
    pub module: String,
    /// The relations, in the module's order.
    pub relations: Vec<String>,
}

impl LeftBehind {
    /// The one line a loader prints for it.
    pub fn line(&self) -> String {
        let at = if self.at.is_empty() {
            String::new()
        } else {
            format!("{}: ", self.at)
        };
        let (first, more) = (&self.relations[0], self.relations.len() - 1);
        let (what, them) = match more {
            0 => (format!("the declaration of {first}"), "it"),
            1 => (
                format!("the declarations of {first} and {}", self.relations[1]),
                "either",
            ),
            n => (
                format!("the declarations of {first} and {n} more relations"),
                "any of them",
            ),
        };
        format!(
            "{at}{} brings {what} and no rule that derives {them}: {} has such a rule, its own \
             or one it imports, and the selection names none — a selection brings the names it \
             lists and what their declarations write, so list the rule, or the entry that names \
             it, if {first} is to be derived",
            self.import, self.module
        )
    }
}

/// What one resolution carries from import to import: the chain of modules
/// being loaded (the cycle check), the modules parsed so far, and what the
/// selections left behind ([`LeftBehind`]).
#[derive(Default)]
struct Walk {
    loading: Vec<String>,
    cache: ModuleCache,
    left: Vec<LeftBehind>,
}

/// A load-time failure. The message is output, byte for byte — the
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
    /// loader lets it through rather than wrapping it.
    fn from(e: ParseError) -> Self {
        LoadError(e.to_string())
    }
}

/// Parsed module forms, by resolved path — [T1a.6.5.3](../../../../docs/history/m1a_rust/README.md#s1a65--frontend-and-load-path).
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

/// Why [`Resolver::read`] has no text for a module.
enum Unread {
    /// No file is there: `module not found at …`.
    Missing,
    /// A file is there and cannot be read as text: the line
    /// [`read::read_program`] gives it, naming its path (M1f S1f.5.22).
    Refused(String),
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
    /// Empty when `std.macro` is unreadable: degrade to no check.
    pub fn stdlib_macro_names(&self) -> Vec<String> {
        let Ok(text) = self.read(&self.stdlib_root(), &["macro"]).1 else {
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

    /// `(root/rel….ein, its text)`, or why there is none: [`Unread::Missing`]
    /// where no file is there, and [`Unread::Refused`] with the line the read
    /// of a program gives where one is and cannot be read as text.
    ///
    /// The display path is what "module not found at …" names, and what the
    /// read's line names, so how it is built is observable: empty segments
    /// dropped, then the final component's extension replaced with `.ein`.
    ///
    /// A file on disk is read through [`crate::read::read_program`], the read
    /// every command makes of the program it is given, since M1f S1f.5.22: a
    /// module that is not UTF-8 is refused at its first such byte, one that
    /// cannot be read or is a directory with the OS's reason, and a `.einb`
    /// container as one ([`crate::read::container_module_line`], Q-M1f.91).
    /// Until then it was `read_to_string(…).ok()` behind `is_file()`, so each
    /// was *not found* (the close review's finding 2, W74).
    fn read(&self, root: &Root, rel: &[&str]) -> (String, Result<String, Unread>) {
        match root {
            Root::Dir(dir) => {
                let mut path = dir.clone();
                for seg in rel.iter().filter(|s| !s.is_empty()) {
                    path.push(seg);
                }
                path.set_extension("ein");
                let display = path.display().to_string();
                let text = match read::read_program(&path) {
                    Ok(read::Held::Text(text)) => Ok(text),
                    Ok(read::Held::Container(_)) => {
                        Err(Unread::Refused(read::container_module_line(&path)))
                    }
                    Err(e) if e.not_found() => Err(Unread::Missing),
                    Err(e) => Err(Unread::Refused(e.line)),
                };
                (display, text)
            }
            Root::Embedded => {
                let joined: Vec<&str> = rel.iter().copied().filter(|s| !s.is_empty()).collect();
                let name = format!("{}.ein", joined.join("/"));
                let text = self.stdlib.read(&name).ok_or(Unread::Missing);
                (format!("<embedded>/{name}"), text)
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
        self.resolve_imports_noting(ast, forms, base_dir)
            .map(|(forms, _)| forms)
    }

    /// [`Self::resolve_imports`], and what its selections left behind
    /// ([`LeftBehind`], M1f S1f.6.25): each selection of the program, or of
    /// a module it imports, that brought a relation's declaration and no rule
    /// that derives it, where the resolved program holds none either.
    pub fn resolve_imports_noting(
        &self,
        ast: &mut Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
    ) -> Result<(Vec<NodeId>, Vec<LeftBehind>), LoadError> {
        let mut walk = Walk::default();
        let out = self.resolve(ast, forms, base_dir, false, &mut walk)?;
        let mut left = walk.left;
        // What the program as resolved declares, and what its rules write.
        let mut declared: BTreeSet<String> = BTreeSet::new();
        let mut written: BTreeSet<String> = BTreeSet::new();
        for &f in &out {
            match ast.head_name(f) {
                Some("relation") => declared.extend(decl_name(ast, f)),
                Some("rule") | Some("hrule") => written.extend(concluded(ast, f)),
                _ => {}
            }
        }
        for l in &mut left {
            l.relations
                .retain(|r| declared.contains(r) && !written.contains(r));
        }
        left.retain(|l| !l.relations.is_empty());
        left.dedup();
        Ok((out, left))
    }

    /// `within_stdlib` is what makes the three resolution tiers agree — M1e
    /// `CO-M5`. A `std.*` module's own `base_dir` is its directory under a
    /// checkout or an override and **`None`** under the embedded root, because
    /// `<embedded>/std/x.ein` is not a path `canonicalize` can resolve. So a
    /// stdlib module with a file-relative import would resolve in a checkout
    /// and fail only in an installed binary — the one configuration no test
    /// exercised. It is refused at load instead, in every tier, with a message
    /// that says why.
    fn resolve(
        &self,
        ast: &mut Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
        within_stdlib: bool,
        walk: &mut Walk,
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
            let is_std = spec.module.split(MODULE_SEP).next() == Some(STDLIB_ALIAS);
            if within_stdlib && !is_std {
                return Err(LoadError(format!(
                    "(import {}) — a std.* module may import only std.* modules; \
                     a file-relative import there resolves under a checkout and \
                     not under the embedded copy an installed binary carries, at {loc}",
                    spec.module
                )));
            }
            let (key, text, dir) = self.locate(&spec.module, base_dir, &loc)?;
            if walk.loading.contains(&key) {
                let mut chain = walk.loading.clone();
                chain.push(key);
                return Err(LoadError(format!(
                    "import cycle: {} (at {loc})",
                    chain.join(" -> ")
                )));
            }
            // Parsed once per resolution, not once per edge.
            let sub = match walk.cache.get(&key) {
                Some(sub) => sub.clone(),
                None => {
                    let sub = parse(ast, &text, Some(&key))?;
                    walk.cache.insert(key.clone(), sub.clone());
                    sub
                }
            };
            walk.loading.push(key);
            let inner = walk.left.len();
            let resolved =
                self.resolve(ast, &sub, dir.as_deref(), within_stdlib || is_std, walk)?;
            walk.loading.pop();
            match &spec.symbols {
                Some(symbols) => {
                    let kept = select(ast, &resolved, symbols, &spec.module, &loc)?;
                    let relations = left_behind(ast, &resolved, &kept, symbols);
                    if !relations.is_empty() {
                        walk.left.push(LeftBehind {
                            at: first_loc(ast, form)
                                .map_or(String::new(), |l| loc_repr(ast, Some(l))),
                            import: crate::dump::dump_compact(ast, form),
                            module: spec.module.clone(),
                            relations,
                        });
                    }
                    out.extend(kept);
                }
                None => {
                    let prefix = format!(
                        "{}{MODULE_SEP}",
                        spec.alias.as_deref().unwrap_or(&spec.module)
                    );
                    // What a module asks, how it is solved and configured and
                    // an engine's trace are a *program's*: an import of any
                    // kind brings declarations and facts and none of
                    // [`DIRECTIVES`] (M1f S1f.6.25, E-8, W90). A selection
                    // never brought one ([`select`]).
                    let brought: Vec<NodeId> = resolved
                        .iter()
                        .copied()
                        .filter(|f| !ast.head_name(*f).is_some_and(|h| DIRECTIVES.contains(&h)))
                        .collect();
                    // What the module's own selections left behind is
                    // renamed as its declarations are.
                    let renamed = defined_names(ast, &brought);
                    for l in &mut walk.left[inner..] {
                        for r in &mut l.relations {
                            if renamed.contains(r) && !ein_core::is_reserved(r) {
                                *r = format!("{prefix}{r}");
                            }
                        }
                    }
                    out.extend(qualify(ast, &brought, &prefix));
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
            // **The override proves itself here** — M1e S1e.3.5, `EH-M2`.
            // Checked at the first `std.*` import rather than at
            // `Resolver::new`, because a program that imports nothing from the
            // stdlib has no business being refused for the shape of a variable
            // it never reads. One error, naming the variable and what is
            // missing, in place of a *module not found* that named neither.
            if let Some(why) = self.stdlib.problem() {
                return Err(LoadError(format!("(import {module}) — {why} ({loc})")));
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
        let text = match text {
            Ok(text) => text,
            Err(Unread::Missing) => {
                return Err(LoadError(format!(
                    "(import {module}) — module not found at {display} ({loc})"
                )));
            }
            Err(Unread::Refused(line)) => return Err(LoadError(line)),
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
    /// **And a name an import's selection lists is live** (M1f S1f.6.25, E-11,
    /// W91): `(import lib :symbols (entry))` names `entry`, and a rule with an
    /// empty `:match`, which fires once in any program that holds it, is
    /// reached by neither relation above. Until that stage such a rule, and a
    /// relation only it names, were dropped, so the program a `--dump-answer`
    /// state file re-writes was not the program that ran, and `std.arith`
    /// wrote its closing rule's name as a fact, `(arith-rule arith)`, for the
    /// name reference to follow. A selection's closure is what is reachable
    /// from its listed names, so everything a top-level flat import brings is
    /// kept; what is shaken is what a qualified import brings.
    ///
    /// Without the activation pass this would silently drop an entire
    /// activator-driven library and leave a file that no longer solves. The
    /// surviving set is observable through the compile cache's size and
    /// through firing order, so it is an observable surface, not a cosmetic
    /// one.
    pub fn resolve_and_minimize(
        &self,
        ast: &mut Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
    ) -> Result<Vec<NodeId>, LoadError> {
        // `(form, is_imported)` — the puzzle's own forms are false, everything
        // an import brings in (transitively) is true.
        let mut tagged: Vec<(NodeId, bool)> = Vec::new();
        // One cache for the whole program; each import its own chain.
        let mut walk = Walk::default();
        // The names the program's own imports list (M1f S1f.6.25, E-11).
        let mut listed: Vec<String> = Vec::new();
        for &form in forms {
            if ast.head_name(form) == Some("import") {
                listed.extend(import_spec(ast, form)?.symbols.unwrap_or_default());
                let resolved = self.resolve(ast, &[form], base_dir, false, &mut walk)?;
                for f in resolved {
                    tagged.push((f, true));
                }
            } else {
                tagged.push((form, false));
            }
        }

        // Imported declarations: first position wins, last body wins.
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
        // A name a selection lists is live: the program asked for it by name.
        let mut work: Vec<String> = listed;
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
    // An import is a module's name, then `:symbols` or `:as` (`ein.y`'s
    // `Import`). Until M1f S1f.5.5's T5 four shapes were refused here, each
    // the grammar's since the swap: no module name, an `:as` that is no name,
    // a `:symbols` that is no list, and `:symbols ()`.
    let module = ast
        .atom_name(args[0])
        .expect("an import's first argument is a module's name (ein.y's Import)")
        .to_string();
    // Last wins on a repeated key.
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
        // An alias binds a name, so it may not be a reserved one (M1f
        // S1f.5.5's T1f.5.5.11, Q-M1f.7 (b)): `ein_core::RESERVED`, the
        // kernel's dunders among them.
        Some(v) => match ast
            .atom_name(v)
            .expect(":as takes a name (ein.y's ImportClause)")
        {
            name if ein_core::is_reserved(name) => {
                return Err(LoadError(format!(
                    "(import {module} :as {name}) — alias '{name}' shadows a reserved kernel \
                     name at {loc}"
                )));
            }
            name => Some(name.to_string()),
        },
        None => None,
    };
    let symbols = symbols_value.map(|v| symbol_list(ast, v));
    Ok(ImportSpec {
        module,
        alias,
        symbols,
    })
}

/// Names inside a `:symbols (a b …)` list — the list lowers to a form whose
/// head and arguments are the names, one or more (`ein.y`'s `NameListPlus`).
fn symbol_list(ast: &Ast, value: NodeId) -> Vec<String> {
    let Node::SForm { head, args } = ast.node(value) else {
        unreachable!(":symbols takes a list of names (ein.y's ImportClause)");
    };
    std::iter::once(head)
        .chain(ast.args(args).iter().copied())
        .map(|n| {
            ast.atom_name(n)
                .expect("a :symbols list holds names")
                .to_string()
        })
        .collect()
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

/// The relation a top-level fact is of, or `None` for a form that is no fact:
/// a declaration, or one of [`DIRECTIVES`].
///
/// A stored negation, `(not (likes Milk Tea))`, is a fact of the relation it
/// negates, so a selection that names `likes` brings it (M1f S1f.6.11).
fn fact_relation(ast: &Ast, form: NodeId) -> Option<String> {
    let mut form = form;
    loop {
        let head = ast.head_name(form)?;
        if DECLARATORS.contains(&head) || DIRECTIVES.contains(&head) {
            return None;
        }
        if head != "not" {
            return Some(head.to_string());
        }
        form = *ast.form_args(form).first()?;
    }
}

/// Names a form list *declares*.
fn defined_names(ast: &Ast, forms: &[NodeId]) -> BTreeSet<String> {
    forms.iter().filter_map(|f| decl_name(ast, *f)).collect()
}

/// Prefix every defined name — and every reference to it — leaving reserved
/// kernel vocabulary alone, so a module that illegally defines `absent` keeps
/// the name and is rejected by the loader rather than silently renamed.
///
/// **What "reserved" means here is [`ein_core::RESERVED`], and that is the fix
/// for `CO-H2`** (M1e S1e.2.1). This function used to filter against a
/// hand-maintained copy of the list, eight names where `ein-core` had nine:
/// M1d S1d.2.3 added the verdict atom `open` to one of the two and not to the
/// other, and the sentence above stopped being true for it. A module declaring
/// `open` — under any of `rule` / `hrule` / `relation` / `macro` — was
/// silently renamed to `mod.open` and the program loaded with exit 0, while
/// the same declaration written directly, or imported flat via
/// `:symbols (open)`, was refused. Same name, different guard, depending on
/// the import tier — and `:as` was a fourth route with the same hole, which
/// the finding did not name because it takes this same path with a different
/// prefix. Thirty-two cells of that matrix are
/// `reserved_names_are_reserved_through_every_import_route` in
/// `tests/imports_semantics.rs`.
fn qualify(ast: &mut Ast, forms: &[NodeId], prefix: &str) -> Vec<NodeId> {
    let mapping: BTreeMap<String, String> = defined_names(ast, forms)
        .into_iter()
        .filter(|n| !ein_core::is_reserved(n))
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
/// unrenamed, and the module's **facts** of every relation the closure names.
///
/// Auto-closure (S1.8a.f20): a listed declaration drags in every *other*
/// declaration of this module it references, so importing an entry rule pulls
/// the machinery it asserts and matches without the importer enumerating it.
/// Names referenced but not declared here — cross-module deps, kernel
/// primitives — are left for the importer's other imports, so a module need
/// not be self-contained.
///
/// **Facts** (M1f S1f.6.11, W88; point (a) of the stage, design.md § G's a3).
/// A name in the closure, listed or named by a kept declaration, selects every
/// top-level fact of the module whose relation it is ([`fact_relation`]), as
/// written and in the module's order: `(import onto :symbols (is-a))` brings
/// the `is-a` facts, and listing a rule that reads `is-a` brings them too. The
/// closure follows the names a kept *declaration* writes, as it always did,
/// and not the names inside a kept fact: `(likes Tea Milk)` brings no
/// declaration of `Tea` (Q-M1f.141). A relation needs no declaration, so a
/// listed name may be one the module only states facts of. Until this stage a
/// selection kept declarations alone, and a module's facts were dropped in
/// silence.
///
/// A listed name the module neither declares nor states a fact of is an
/// error: there is no re-export of the absent. A rule nothing names is still
/// dropped (point (f)): a selection is a selection.
fn select(
    ast: &Ast,
    forms: &[NodeId],
    symbols: &[String],
    module: &str,
    loc: &str,
) -> Result<Vec<NodeId>, LoadError> {
    let mut decls: BTreeMap<String, NodeId> = BTreeMap::new();
    let mut stated: BTreeSet<String> = BTreeSet::new();
    for &f in forms {
        if let Some(n) = decl_name(ast, f) {
            decls.insert(n, f); // last wins
        } else if let Some(r) = fact_relation(ast, f) {
            stated.insert(r);
        }
    }
    let missing: Vec<&String> = symbols
        .iter()
        .filter(|s| !decls.contains_key(*s) && !stated.contains(*s))
        .collect();
    if !missing.is_empty() {
        let mut names: Vec<&str> = missing.iter().map(|s| s.as_str()).collect();
        names.sort_unstable();
        names.dedup();
        return Err(LoadError(format!(
            "(import {module} :symbols …) — not provided by the module: {} at {loc}",
            names.join(", ")
        )));
    }
    // Every name the closure reaches: the listed ones, and the ones a kept
    // declaration writes. A declaration is kept when its name is here, and so
    // is a fact when its relation is.
    let mut keep: BTreeSet<String> = BTreeSet::new();
    let mut work: Vec<String> = symbols.to_vec();
    // A repeated symbol is one item; the closure makes that immaterial, but
    // the stack starts deduplicated.
    work.sort();
    work.dedup();
    while let Some(n) = work.pop() {
        if keep.contains(&n) {
            continue;
        }
        if let Some(&f) = decls.get(&n) {
            let mut names = BTreeSet::new();
            referenced_names(ast, f, &mut names);
            work.extend(names);
        }
        keep.insert(n);
    }
    Ok(forms
        .iter()
        .copied()
        .filter(|f| {
            decl_name(ast, *f)
                .or_else(|| fact_relation(ast, *f))
                .is_some_and(|n| keep.contains(&n))
        })
        .collect())
}

/// The relations a selection brought the declaration of and no rule for
/// ([`LeftBehind`]'s first three conditions): declared among `forms`, kept in
/// `kept`, not in `symbols`, written by the `:assert` of a rule or an hrule
/// of `forms`, and by none of `kept`. In the module's order.
fn left_behind(ast: &Ast, forms: &[NodeId], kept: &[NodeId], symbols: &[String]) -> Vec<String> {
    let writers = |fs: &[NodeId]| -> BTreeSet<String> {
        fs.iter()
            .filter(|f| matches!(ast.head_name(**f), Some("rule") | Some("hrule")))
            .flat_map(|f| concluded(ast, *f))
            .collect()
    };
    let (in_module, in_kept) = (writers(forms), writers(kept));
    let mut out: Vec<String> = Vec::new();
    for &f in kept {
        if ast.head_name(f) != Some("relation") {
            continue;
        }
        let Some(name) = decl_name(ast, f) else {
            continue;
        };
        if !symbols.contains(&name)
            && in_module.contains(&name)
            && !in_kept.contains(&name)
            && !out.contains(&name)
        {
            out.push(name);
        }
    }
    out
}

/// The relations a rule's or an hrule's `:assert` concludes a fact of: the
/// head of each conclusion, under `(and …)` and `(not …)`. A relation written
/// as a variable is none, and neither is the head of a fact *inside* a
/// conclusion, which is a value ([`kw_heads`] collects those too, for the
/// tree-shake, which wants every name).
fn concluded(ast: &Ast, rule: NodeId) -> BTreeSet<String> {
    fn walk(ast: &Ast, node: NodeId, out: &mut BTreeSet<String>) {
        match ast.head_name(node) {
            Some("and") | Some("not") => {
                for &a in ast.form_args(node) {
                    walk(ast, a, out);
                }
            }
            Some(name) => {
                out.insert(name.to_string());
            }
            None => {}
        }
    }
    let mut out = BTreeSet::new();
    for &a in ast.form_args(rule) {
        if let Node::KwPair { key, value } = ast.node(a)
            && let Node::Keyword(k) = ast.node(key)
            && ast.sym(k) == "assert"
        {
            walk(ast, value, &mut out);
        }
    }
    out
}

/// Where a form is written: its own position, or its head's, or its first
/// argument's that has one. The parser keeps none for a top-level form.
fn first_loc(ast: &Ast, node: NodeId) -> Option<crate::ast::Loc> {
    ast.loc(node).or_else(|| match ast.node(node) {
        Node::SForm { head, .. } => first_loc(ast, head)
            .or_else(|| ast.form_args(node).iter().find_map(|&a| first_loc(ast, a))),
        _ => None,
    })
}

/// Drop repeated **identical** declarations, keeping the first.
///
/// Import is idempotent: a module and its importer — or two modules — may both
/// pull the same shared dependency, and the diamond must collapse rather than
/// trip a duplicate-name error. A second declaration of the same name *and
/// kind* with a different body is a genuine conflict.
///
/// **A fact that arrives twice is kept once** since M1f S1f.6.11, when a flat
/// import began to bring facts: a diamond of flat imports of one facts library
/// yields each of its facts once, in the place of its first arrival.
///
/// The key is `(kind, name)`, not the name alone: a `(rule undefeated …)` and
/// a `(relation undefeated …)` share a name but are distinct declarations — a
/// rule that produces a same-named relation is idiomatic.
fn dedup_declarations(ast: &Ast, forms: &[NodeId]) -> Result<Vec<NodeId>, LoadError> {
    let mut seen: BTreeMap<(String, String), NodeId> = BTreeMap::new();
    let mut facts: BTreeSet<NodeId> = BTreeSet::new();
    let mut out: Vec<NodeId> = Vec::new();
    for &f in forms {
        let Some(name) = decl_name(ast, f) else {
            // One statement of one module, arriving by two paths, is one
            // statement (M1f S1f.6.11): a module is parsed once per resolution
            // ([`ModuleCache`]) and a flat import does not rebuild a fact, so
            // the second arrival is the same node. A fact the program writes
            // twice is two nodes, and is left to the loader, as it was.
            if fact_relation(ast, f).is_none() || facts.insert(f) {
                out.push(f);
            }
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
