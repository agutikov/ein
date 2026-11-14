//! Import resolution — full names and short names (M1f S1f.6.35, the user's
//! model (h) of 2026-11-13 and its ruling of 2026-11-14).
//!
//! `(import M [:as A | :symbols (S…)])` is resolved at the **form** level,
//! *before* the loader runs, into one import-free list of forms:
//!
//! - **Every import brings its module whole**, with the module's own imports,
//!   all the way down, and **one copy per program**: a module read by two
//!   edges, under two aliases or by none of the program's own, is read once.
//!   The user, 2026-11-14: *"in all cases we import everything, the
//!   difference only in names"*. What a module states as a program, its
//!   `(query …)`, `(solve …)`, `(config …)` and `(trace …)` (`DIRECTIVES`),
//!   is the program's own and no import brings it (M1f S1f.6.25, E-8).
//! - **A name a module declares has one full name**, the module's canonical
//!   name and the name, `std.math.decimal-arith.add`, the same through every import path
//!   and global: any file may write it. A module's canonical name is
//!   `std.x.y` under the stdlib, and any other a path **from the program
//!   file's directory**, a dot per directory, for the imports a module writes
//!   too. The main file has no module name: what it declares keeps its name.
//!   A name no file declares (`z`, a digit, a type, a relation stated and
//!   never declared) has no full name and is one name in every file.
//! - **A short name is local to the file that binds it**: its own
//!   declarations, the names an import's `:symbols` lists, and the alias
//!   prefix `:as a` gives (`a.add`). A bare `(import M)` binds none: the file
//!   writes full names. Nothing is re-exported. One short name bound to two
//!   full names in one file is refused; a short name a file writes that it
//!   neither declares nor imports is its own, whatever a module declares under
//!   it (shadowing, Q-M1f.236).
//! - **What prints** is the main file's spelling where it has one, a listed
//!   short name before an alias's, and the full name otherwise
//!   (Q-M1f.237, Q-M1f.253): the resolved list is renamed once, full name to
//!   that spelling, before the loader sees it, so every surface prints it.
//!
//! The refusals, each a section of `defined_behaviour.md` § 4: a module's
//! name with an empty segment or a `/` (Q-M1f.252, `..` among them); one short
//! name bound to two full names in one file (Q-M1f.234); a prefixed or full
//! name that names nothing its module declares (Q-M1f.251); a full name of a
//! module no import brings (Q-M1f.238); a declaration of the main file under
//! an imported module's full name (Q-M1f.239); a module's declared name with a
//! dot (Q-M1f.241). A cycle is refused as it always was.
//!
//! This module is the engine's **only** filesystem access, which is what keeps
//! any later policy on what the engine may read a single seam rather than an
//! audit: everything goes through [`Resolver`], and the stdlib half already
//! does ([`crate::stdlib`]). A module on disk is read through
//! [`crate::read::read_program`], the read every command makes of the program
//! it is given (M1f S1f.5.22): a module that exists and cannot be read as text
//! is refused with that read's line, naming the module's path.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::ast::{Ast, Loc, Node, NodeId, loc_repr};
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

/// One file of the program: the main file (index 0, no module name) or a
/// module, each read once.
struct File {
    /// The canonical module name; `None` for the main file.
    name: Option<String>,
    /// The resolved path (or the embedded copy's), what a cycle names.
    key: String,
    /// The file's forms, as parsed.
    forms: Vec<NodeId>,
    /// `(import form, spec, the index of the module it names)`, in order.
    imports: Vec<(NodeId, ImportSpec, usize)>,
    /// The names the file declares, each with its first declaring form.
    declared: BTreeMap<String, NodeId>,
    /// The relations of the file's top-level facts, as written.
    stated: BTreeSet<String>,
    /// Whether the file is a `std.*` module.
    is_std: bool,
}

impl File {
    fn new(name: Option<String>, key: String, forms: Vec<NodeId>, ast: &Ast, is_std: bool) -> File {
        let mut declared = BTreeMap::new();
        let mut stated = BTreeSet::new();
        for &f in &forms {
            if let Some(n) = decl_name(ast, f) {
                declared.entry(n).or_insert(f);
            } else if let Some(r) = fact_relation(ast, f) {
                stated.insert(r);
            }
        }
        File {
            name,
            key,
            forms,
            imports: Vec::new(),
            declared,
            stated,
            is_std,
        }
    }

    /// The full name of a name this file declares: the module's name and the
    /// name, or the name itself in the main file. A reserved name is never
    /// renamed, so that a module that illegally declares one is refused by
    /// the loader under its own name (`CO-H2`, M1e S1e.2.1).
    fn full(&self, short: &str) -> String {
        match &self.name {
            Some(m) if !ein_core::is_reserved(short) => format!("{m}{MODULE_SEP}{short}"),
            _ => short.to_string(),
        }
    }
}

/// The per-file short names: a name to its full name, and an alias to the
/// module it prefixes.
#[derive(Default)]
struct Table {
    names: BTreeMap<String, String>,
    aliases: BTreeMap<String, usize>,
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
    /// A file on disk is read through [`crate::read::read_program`], the read
    /// every command makes of the program it is given, since M1f S1f.5.22: a
    /// module that is not UTF-8 is refused at its first such byte, one that
    /// cannot be read or is a directory with the OS's reason, and a `.einb`
    /// container as one ([`crate::read::container_module_line`], Q-M1f.91).
    fn read(&self, root: &Root, rel: &[&str]) -> (String, Result<String, Unread>) {
        match root {
            Root::Dir(dir) => {
                let mut path = dir.clone();
                for seg in rel {
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
                let name = format!("{}.ein", rel.join("/"));
                let text = self.stdlib.read(&name).ok_or(Unread::Missing);
                (format!("<embedded>/{name}"), text)
            }
        }
    }

    /// Whether a module of this canonical name exists, for a full name of a
    /// module no import brings (Q-M1f.238). Never an error.
    fn exists(&self, module: &str, base_dir: Option<&Path>) -> bool {
        let segments: Vec<&str> = module.split(MODULE_SEP).collect();
        if segments.iter().any(|s| s.is_empty() || s.contains('/')) {
            return false;
        }
        if segments[0] == STDLIB_ALIAS {
            if segments.len() < 2 || self.stdlib.problem().is_some() {
                return false;
            }
            return self.read(&self.stdlib_root(), &segments[1..]).1.is_ok();
        }
        let Some(dir) = base_dir else { return false };
        let mut path = dir.to_path_buf();
        for s in &segments {
            path.push(s);
        }
        path.set_extension("ein");
        path.is_file()
    }

    /// Return `forms` with every `(import …)` replaced by the program (h)
    /// makes of it: every module once, whole, under full names, the main
    /// file's spelling printed. A program with no import is returned as it
    /// is, but for its full names of a module no import brings (Q-M1f.238),
    /// which are refused.
    ///
    /// `base_dir` is the program file's directory: every file-relative module
    /// of the program is found from it.
    pub fn resolve_imports(
        &self,
        ast: &mut Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
    ) -> Result<Vec<NodeId>, LoadError> {
        if !forms.iter().any(|&f| ast.head_name(f) == Some("import")) {
            self.refuse_unbrought(ast, forms, base_dir)?;
            return Ok(forms.to_vec());
        }
        // Read every file of the program, once each.
        let main = File::new(None, String::new(), forms.to_vec(), ast, false);
        let mut files = vec![main];
        let mut by_name: BTreeMap<String, usize> = BTreeMap::new();
        self.walk(ast, &mut files, &mut by_name, 0, &mut Vec::new(), base_dir)?;

        // Every file's short names, its own and its imports'.
        let tables = self.tables(ast, &files, &by_name)?;

        // Every file's forms with every name it writes read as (h) reads it.
        let mut rewritten: Vec<Vec<NodeId>> = Vec::with_capacity(files.len());
        for i in 0..files.len() {
            let mapping = self.name_map(ast, &files, &by_name, &tables, i, base_dir)?;
            let forms: Vec<NodeId> = files[i]
                .forms
                .iter()
                .map(|&f| {
                    if ast.head_name(f) == Some("import") || mapping.is_empty() {
                        f
                    } else {
                        rename_atoms(ast, f, &mapping)
                    }
                })
                .collect();
            rewritten.push(forms);
        }

        // One list: the main file's forms, a module's in place of the first
        // import that reaches it, its own imports at theirs.
        let mut out = Vec::new();
        let mut done = vec![false; files.len()];
        emit(ast, &files, &rewritten, 0, &mut done, &mut out);
        let out = dedup_declarations(ast, &out)?;

        // What prints: the main file's spelling.
        let display = display_names(ast, &files, &tables[0], &out);
        if display.is_empty() {
            return Ok(out);
        }
        Ok(out
            .iter()
            .map(|&f| rename_atoms(ast, f, &display))
            .collect())
    }

    /// The short names the main file binds through its imports' `:symbols`
    /// lists, each to its full name: what a program written with every
    /// import whole would write in full (`ein-cli`'s `import_invariance.rs`,
    /// the stage's acceptance). The program's own declarations are not among
    /// them.
    pub fn short_names(
        &self,
        ast: &mut Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
    ) -> Result<BTreeMap<String, String>, LoadError> {
        let main = File::new(None, String::new(), forms.to_vec(), ast, false);
        let mut files = vec![main];
        let mut by_name: BTreeMap<String, usize> = BTreeMap::new();
        self.walk(ast, &mut files, &mut by_name, 0, &mut Vec::new(), base_dir)?;
        let tables = self.tables(ast, &files, &by_name)?;
        Ok(tables[0]
            .names
            .iter()
            .filter(|(s, _)| !files[0].declared.contains_key(*s))
            .map(|(s, f)| (s.clone(), f.clone()))
            .collect())
    }

    /// Read the modules file `i` imports, depth first, each once. `chain` is
    /// the files being read, for the cycle check.
    fn walk(
        &self,
        ast: &mut Ast,
        files: &mut Vec<File>,
        by_name: &mut BTreeMap<String, usize>,
        i: usize,
        chain: &mut Vec<usize>,
        base_dir: Option<&Path>,
    ) -> Result<(), LoadError> {
        let forms = files[i].forms.clone();
        for form in forms {
            if ast.head_name(form) != Some("import") {
                continue;
            }
            let spec = import_spec(ast, form)?;
            let loc = loc_repr(ast, ast.loc(form));
            let module = canonical(&spec.module, &loc)?;
            let is_std = module.split(MODULE_SEP).next() == Some(STDLIB_ALIAS);
            if files[i].is_std && !is_std {
                return Err(LoadError(format!(
                    "(import {}) — a std.* module may import only std.* modules; \
                     a file-relative import there resolves under a checkout and \
                     not under the embedded copy an installed binary carries, at {loc}",
                    spec.module
                )));
            }
            let j = match by_name.get(&module) {
                Some(&j) => {
                    if chain.contains(&j) {
                        let mut keys: Vec<String> =
                            chain.iter().map(|&c| files[c].key.clone()).collect();
                        keys.push(files[j].key.clone());
                        let from = keys.iter().position(|k| *k == files[j].key).unwrap_or(0);
                        return Err(LoadError(format!(
                            "import cycle: {} (at {loc})",
                            keys[from..].join(" -> ")
                        )));
                    }
                    j
                }
                None => {
                    let (key, text) = self.locate(&spec.module, &module, base_dir, &loc)?;
                    let parsed = parse(ast, &text, Some(&key))?;
                    let file = File::new(Some(module.clone()), key, parsed, ast, is_std);
                    if let Some((short, decl)) =
                        file.declared.iter().find(|(n, _)| n.contains(MODULE_SEP))
                    {
                        return Err(LoadError(format!(
                            "{}: {module} declares '{short}', a name with a dot — a module's \
                             declared name holds no dot, so that a full name such as \
                             {module}.x reads one way (Q-M1f.241)",
                            first_loc(ast, *decl)
                                .map_or_else(|| file.key.clone(), |l| loc_repr(ast, Some(l)))
                        )));
                    }
                    files.push(file);
                    let j = files.len() - 1;
                    by_name.insert(module.clone(), j);
                    chain.push(j);
                    self.walk(ast, files, by_name, j, chain, base_dir)?;
                    chain.pop();
                    j
                }
            };
            files[i].imports.push((form, spec, j));
        }
        Ok(())
    }

    /// Logical module name → `(identity, text)`.
    ///
    /// `std.x.y` resolves under the stdlib root; anything else from the
    /// program file's directory. The identity is the **resolved** path,
    /// because that is what a cycle names and what the imported forms' `Loc`s
    /// name.
    fn locate(
        &self,
        written: &str,
        module: &str,
        base_dir: Option<&Path>,
        loc: &str,
    ) -> Result<(String, String), LoadError> {
        let segments: Vec<&str> = module.split(MODULE_SEP).collect();
        let (root, rel) = if segments[0] == STDLIB_ALIAS {
            // **The override proves itself here** — M1e S1e.3.5, `EH-M2`.
            // Checked at the first `std.*` import rather than at
            // `Resolver::new`, because a program that imports nothing from the
            // stdlib has no business being refused for the shape of a variable
            // it never reads.
            if let Some(why) = self.stdlib.problem() {
                return Err(LoadError(format!("(import {written}) — {why} ({loc})")));
            }
            (self.stdlib_root(), segments[1..].to_vec())
        } else {
            let Some(dir) = base_dir else {
                return Err(LoadError(format!(
                    "(import {written}) — file-relative import needs a base directory \
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
                    "(import {written}) — module not found at {display} ({loc})"
                )));
            }
            Err(Unread::Refused(line)) => return Err(LoadError(line)),
        };
        let key = std::fs::canonicalize(&display)
            .ok()
            .map_or_else(|| display.clone(), |p| p.display().to_string());
        Ok((key, text))
    }

    /// Every file's short names. A module's are read before the files that
    /// import it, since a listed name a module states facts of is bound as
    /// that module's own text reads it (Q-M1f.250).
    fn tables(
        &self,
        ast: &Ast,
        files: &[File],
        by_name: &BTreeMap<String, usize>,
    ) -> Result<Vec<Table>, LoadError> {
        let mut tables: Vec<Option<Table>> = (0..files.len()).map(|_| None).collect();
        // Imports before importers: the program has no cycle (the walk
        // refused one), so a file's modules can each be read first.
        let mut order = Vec::with_capacity(files.len());
        let mut seen = vec![false; files.len()];
        fn post(files: &[File], i: usize, seen: &mut [bool], order: &mut Vec<usize>) {
            if std::mem::replace(&mut seen[i], true) {
                return;
            }
            for (_, _, j) in &files[i].imports {
                post(files, *j, seen, order);
            }
            order.push(i);
        }
        post(files, 0, &mut seen, &mut order);
        for i in order {
            let file = &files[i];
            let mut t = Table::default();
            for short in file.declared.keys() {
                t.names.insert(short.clone(), file.full(short));
                if file.name.is_none()
                    && let Some((prefix, _)) = short.rsplit_once(MODULE_SEP)
                    && by_name.contains_key(prefix)
                {
                    let decl = file.declared[short];
                    return Err(LoadError(format!(
                        "{}: the program declares '{short}', a full name of {prefix}, \
                         which the program imports — a declaration's full name is its own \
                         file's, and a full name of {prefix} is one {prefix} declares \
                         (Q-M1f.239)",
                        place(ast, decl)
                    )));
                }
            }
            for (form, spec, j) in &file.imports {
                let m = &files[*j];
                let m_name = m.name.clone().expect("an imported file is a module");
                match &spec.symbols {
                    Some(symbols) => {
                        let missing: BTreeSet<&str> = symbols
                            .iter()
                            .filter(|s| !m.declared.contains_key(*s) && !m.stated.contains(*s))
                            .map(String::as_str)
                            .collect();
                        if !missing.is_empty() {
                            return Err(LoadError(format!(
                                "(import {} :symbols …) — not provided by the module: {} at {}",
                                spec.module,
                                missing.into_iter().collect::<Vec<_>>().join(", "),
                                loc_repr(ast, ast.loc(*form))
                            )));
                        }
                        for s in symbols {
                            let full = if m.declared.contains_key(s) {
                                m.full(s)
                            } else {
                                // A name the module states facts of and does not
                                // declare: the relation those facts are of, as the
                                // module's own text reads it.
                                tables[*j]
                                    .as_ref()
                                    .and_then(|mt| mt.names.get(s).cloned())
                                    .unwrap_or_else(|| s.clone())
                            };
                            bind(ast, &mut t, s, full, file, *form)?;
                        }
                    }
                    None => {
                        let alias = spec.alias.clone().unwrap_or_else(|| m_name.clone());
                        match t.aliases.get(&alias) {
                            Some(&k) if k != *j => {
                                return Err(LoadError(format!(
                                    "{}: '{alias}' prefixes two modules in one file, {} and \
                                     {m_name} — one short name names one thing in a file",
                                    place(ast, *form),
                                    files[k].name.as_deref().unwrap_or("")
                                )));
                            }
                            _ => {
                                t.aliases.insert(alias, *j);
                            }
                        }
                    }
                }
            }
            tables[i] = Some(t);
        }
        Ok(tables.into_iter().map(|t| t.unwrap_or_default()).collect())
    }

    /// The renaming of file `i`'s names to their full names, refusing a
    /// prefixed or full name that names nothing (Q-M1f.251) and a full name
    /// of a module no import brings (Q-M1f.238).
    fn name_map(
        &self,
        ast: &Ast,
        files: &[File],
        by_name: &BTreeMap<String, usize>,
        tables: &[Table],
        i: usize,
        base_dir: Option<&Path>,
    ) -> Result<BTreeMap<String, String>, LoadError> {
        let t = &tables[i];
        let mut written: BTreeMap<String, NodeId> = BTreeMap::new();
        for &f in &files[i].forms {
            if ast.head_name(f) != Some("import") {
                atoms(ast, f, &mut written);
            }
        }
        // The longest alias first, so that `a.b.x` reads `a.b` before `a`.
        let mut aliases: Vec<(&String, &usize)> = t.aliases.iter().collect();
        aliases.sort_by_key(|(a, _)| std::cmp::Reverse(a.len()));
        let mut out = BTreeMap::new();
        for (n, node) in written {
            if ein_core::is_reserved(&n) {
                continue;
            }
            let full = if let Some(full) = t.names.get(&n) {
                full.clone()
            } else if n.contains(MODULE_SEP) {
                let aliased = aliases.iter().find_map(|(a, j)| {
                    n.strip_prefix(a.as_str())?
                        .strip_prefix(MODULE_SEP)
                        .map(|r| (r, **j))
                });
                if let Some((rest, j)) = aliased {
                    let m = &files[j];
                    if !m.declared.contains_key(rest) {
                        return Err(names_nothing(
                            ast,
                            node,
                            &n,
                            m.name.as_deref().unwrap_or(""),
                            rest,
                        ));
                    }
                    m.full(rest)
                } else if let Some((prefix, short)) = n.rsplit_once(MODULE_SEP) {
                    if let Some(&j) = by_name.get(prefix) {
                        if !files[j].declared.contains_key(short) {
                            return Err(names_nothing(ast, node, &n, prefix, short));
                        }
                        n.clone()
                    } else if self.exists(prefix, base_dir) {
                        return Err(LoadError(format!(
                            "{}: '{n}' is a full name of {prefix}, which no import of the \
                             program brings — a module enters a program by an import of it: \
                             write (import {prefix} …) (Q-M1f.238)",
                            place(ast, node)
                        )));
                    } else {
                        n.clone()
                    }
                } else {
                    n.clone()
                }
            } else {
                n.clone()
            };
            if full != n {
                out.insert(n, full);
            }
        }
        Ok(out)
    }

    /// A program with no import may still write a full name of a module, which
    /// no import brings (Q-M1f.238). A name it declares is its own, whatever
    /// its dots: a `--dump-answer` state file declares the full names it was
    /// loaded with and imports nothing (Q-M1f.239).
    fn refuse_unbrought(
        &self,
        ast: &Ast,
        forms: &[NodeId],
        base_dir: Option<&Path>,
    ) -> Result<(), LoadError> {
        let declared: BTreeSet<String> = forms.iter().filter_map(|f| decl_name(ast, *f)).collect();
        let mut written: BTreeMap<String, NodeId> = BTreeMap::new();
        for &f in forms {
            atoms(ast, f, &mut written);
        }
        for (n, node) in written {
            if declared.contains(&n) {
                continue;
            }
            if let Some((prefix, _)) = n.rsplit_once(MODULE_SEP)
                && self.exists(prefix, base_dir)
            {
                return Err(LoadError(format!(
                    "{}: '{n}' is a full name of {prefix}, which no import of the program \
                     brings — a module enters a program by an import of it: write (import \
                     {prefix} …) (Q-M1f.238)",
                    place(ast, node)
                )));
            }
        }
        Ok(())
    }
}

/// Bind a short name in a file's table, refusing one bound to two full names
/// (Q-M1f.234: *"if it happens in the same file - it's an error, not correct
/// ein code, just like ordinary namespaces"*).
fn bind(
    ast: &Ast,
    t: &mut Table,
    short: &str,
    full: String,
    file: &File,
    form: NodeId,
) -> Result<(), LoadError> {
    match t.names.get(short) {
        Some(prev) if *prev != full => {
            let own = file.declared.contains_key(short);
            Err(LoadError(format!(
                "{}: '{short}' is bound to two full names in {}, {prev}{} and {full} — one short \
                 name names one declaration in a file: list it from one module, or write the \
                 other by its full name",
                place(ast, form),
                file.name
                    .as_deref()
                    .map_or_else(|| "the program".to_string(), str::to_string),
                if own { " (its own declaration)" } else { "" }
            )))
        }
        _ => {
            t.names.insert(short.to_string(), full);
            Ok(())
        }
    }
}

fn names_nothing(ast: &Ast, node: NodeId, n: &str, module: &str, short: &str) -> LoadError {
    LoadError(format!(
        "{}: '{n}' names nothing: {module} declares no '{short}' — a prefixed name is a full \
         name of a declaration of the module it names (Q-M1f.251)",
        place(ast, node)
    ))
}

/// Where a node is written, or `None`.
fn place(ast: &Ast, node: NodeId) -> String {
    loc_repr(ast, first_loc(ast, node))
}

/// A module's canonical name: its segments, none empty and none holding a
/// `/` (Q-M1f.252), and not `std` alone.
fn canonical(module: &str, loc: &str) -> Result<String, LoadError> {
    let segments: Vec<&str> = module.split(MODULE_SEP).collect();
    if segments.len() == 1 && segments[0] == STDLIB_ALIAS {
        return Err(LoadError(format!(
            "(import {module}) — bare '{STDLIB_ALIAS}' is not a module at {loc}"
        )));
    }
    if segments.iter().any(|s| s.is_empty() || s.contains('/')) {
        return Err(LoadError(format!(
            "(import {module}) — a module's name is its path from the program's directory, a \
             name per directory joined by dots, none empty and none holding a '/': no '..', no \
             path (Q-M1f.252) at {loc}"
        )));
    }
    Ok(module.to_string())
}

/// Splice file `i`'s forms into `out`: a module's in place of the first
/// import that reaches it, and a module's [`DIRECTIVES`] left out.
fn emit(
    ast: &Ast,
    files: &[File],
    rewritten: &[Vec<NodeId>],
    i: usize,
    done: &mut [bool],
    out: &mut Vec<NodeId>,
) {
    done[i] = true;
    let mut next = files[i].imports.iter();
    for &f in &rewritten[i] {
        if ast.head_name(f) == Some("import") {
            let (_, _, j) = next.next().expect("one entry per import form");
            if !done[*j] {
                emit(ast, files, rewritten, *j, done, out);
            }
            continue;
        }
        if i != 0 && ast.head_name(f).is_some_and(|h| DIRECTIVES.contains(&h)) {
            continue;
        }
        out.push(f);
    }
}

/// What prints: the main file's listed short name, else its first alias's
/// spelling, else the full name (Q-M1f.237, Q-M1f.253). A spelling that is
/// already a name of the resolved program, a name a module writes and no file
/// declares, is not taken, so that two names never print as one.
fn display_names(
    ast: &Ast,
    files: &[File],
    main: &Table,
    out: &[NodeId],
) -> BTreeMap<String, String> {
    let mut d: BTreeMap<String, String> = BTreeMap::new();
    for (short, full) in &main.names {
        if short != full {
            d.entry(full.clone()).or_insert_with(|| short.clone());
        }
    }
    // Aliases in the file's order: the first one written prints.
    for &(form, ref spec, j) in &files[0].imports {
        let _ = form;
        let Some(alias) = &spec.alias else { continue };
        let m = &files[j];
        for short in m.declared.keys() {
            let full = m.full(short);
            if full != *short {
                d.entry(full)
                    .or_insert_with(|| format!("{alias}{MODULE_SEP}{short}"));
            }
        }
    }
    if d.is_empty() {
        return d;
    }
    let mut used: BTreeMap<String, NodeId> = BTreeMap::new();
    for &f in out {
        atoms(ast, f, &mut used);
    }
    d.retain(|full, spelt| full != spelt && !used.contains_key(spelt.as_str()));
    d
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

// ── Names ──────────────────────────────────────────────────────────

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
/// negates (M1f S1f.6.11).
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

/// Every `Atom` name under `node` — heads, arguments and kw-pair values —
/// with the first node that writes it.
fn atoms(ast: &Ast, node: NodeId, out: &mut BTreeMap<String, NodeId>) {
    match ast.node(node) {
        Node::Atom(s) => {
            out.entry(ast.sym(s).to_string()).or_insert(node);
        }
        Node::SForm { head, args } => {
            atoms(ast, head, out);
            for &a in ast.args(args) {
                atoms(ast, a, out);
            }
        }
        Node::KwPair { value, .. } => atoms(ast, value, out),
        _ => {}
    }
}

/// Rewrite every `Atom` whose name is in `mapping` — head, args and kw-pair
/// values alike, so references and `:rule` provenance refs follow the rename.
/// A node no name of which is renamed is returned as it is.
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
            let args: Vec<NodeId> = ast.args(args).to_vec();
            let new_head = rename_atoms(ast, head, mapping);
            let new_args: Vec<NodeId> = args
                .iter()
                .map(|&a| rename_atoms(ast, a, mapping))
                .collect();
            if new_head == head && new_args == args {
                return node;
            }
            let loc = ast.loc(node);
            ast.sform(new_head, &new_args, loc)
        }
        Node::KwPair { key, value } => {
            let new_value = rename_atoms(ast, value, mapping);
            if new_value == value {
                return node;
            }
            let loc = ast.loc(node);
            ast.push(
                Node::KwPair {
                    key,
                    value: new_value,
                },
                loc,
            )
        }
        _ => node,
    }
}

/// Where a form is written: its own position, or its head's, or its first
/// argument's that has one. The parser keeps none for a top-level form.
fn first_loc(ast: &Ast, node: NodeId) -> Option<Loc> {
    ast.loc(node).or_else(|| match ast.node(node) {
        Node::SForm { head, .. } => first_loc(ast, head)
            .or_else(|| ast.form_args(node).iter().find_map(|&a| first_loc(ast, a))),
        _ => None,
    })
}

/// Drop repeated **identical** declarations, keeping the first; a second
/// declaration of one kind and one name with another body is refused.
///
/// Every module is one copy, so two declarations of one full name come from
/// one file's text: a file that writes one declaration twice. A fact that
/// arrives twice is the same node and kept once (M1f S1f.6.11); a fact the
/// program writes twice is two nodes, and is left to the loader.
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
            Some(&prev) if ast.eq_nodes(prev, f) => {}
            Some(_) => {
                return Err(LoadError(format!(
                    "conflicting definitions of '{name}': same name, different body (at {})",
                    loc_repr(ast, first_loc(ast, f))
                )));
            }
        }
    }
    Ok(out)
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
