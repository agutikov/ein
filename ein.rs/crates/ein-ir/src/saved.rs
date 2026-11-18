//! **A saved file**: a program, every module it imports and the choices
//! made, each as written, in one file that reads no other
//! (Q-M1f.400, Q-M1f.401, Q-M1f.402).
//!
//! ```text
//! ;;; ein-saved/1 — …                  one fixed line, FIRST_LINE
//! (program
//! <the program file's text, as read>
//! )
//! (module NAME
//! <that module's text, as read>
//! )                                     one per module the load read, sorted by name
//! (commit
//!   F
//!   F)                                  the candidates of H a node commits; none at root
//! ```
//!
//! [`write()`] writes one from a program's [`Sources`] and a commitment.
//! [`load_text`] reads a file's text, a program or a saved file, by the
//! first two tokens ([`crate::grammar::opens_saved`]): a saved file is parsed
//! whole, its wrappers read in order, and the program it carries is parsed
//! again as the file it was, under its own name
//! ([`program_name`]) and with its own positions, its imports resolved
//! against the modules the file carries and never the disk
//! ([`Resolver::carrying`]). What a load read, a program's or a saved file's,
//! is [`ein_core::Program::sources`], and the choices
//! [`ein_core::Program::commit`]: the engine commits them after root's
//! fixpoint, as a node commits its commitment (`ein_infer::region`), which is
//! why this crate does not intern them. A load handed a saved file's parsed
//! forms rather than its text, its first form its `(program …)`, reads it
//! the same way (`load_forms`, from `crate::from_ir`'s `load_query_by`), so
//! every load of a saved file is of the node it names. The parse takes a
//! saved file one level past the nesting limit, its wrappers' own, and each
//! text it carries is parsed again alone under the limit (Q-M1f.185).

use std::path::Path;
use std::sync::Arc;

use ein_core::{Kb, Sources, Terms};

use crate::ast::{Ast, Node, NodeId, loc_repr};
use crate::dump::dump_compact;
use crate::from_ir::{KbLoadError, LoadOptions, load_query_by};
use crate::grammar::{opens_saved, parse};
use crate::imports::Resolver;

/// The format's name, which its first line names.
pub const FORMAT: &str = "ein-saved/1";

/// A saved file's first line, the same in every one: what it is and that it
/// reads no other file (Q-M1f.402). What a run says of the state it saved
/// is `index.json`'s.
pub const FIRST_LINE: &str = ";;; ein-saved/1 — the program and every module it imports, each as \
     written, and the choices made: this file reads no other.";

/// A saved file of the program `sources` holds, at the node of `commit`:
/// each candidate's text in H's order, none at root. A load of it reads the
/// same texts and the same choices, so a second save of a load is the same
/// bytes.
pub fn write(sources: &Sources, commit: &[&str]) -> String {
    let mut out = String::with_capacity(
        FIRST_LINE.len()
            + sources.program.len()
            + sources
                .modules
                .iter()
                .map(|(_, t)| t.len() + 32)
                .sum::<usize>()
            + 64,
    );
    out.push_str(FIRST_LINE);
    out.push('\n');
    out.push_str("(program\n");
    out.push_str(&sources.program);
    out.push_str("\n)\n");
    for (name, text) in &sources.modules {
        out.push_str("(module ");
        out.push_str(name);
        out.push('\n');
        out.push_str(text);
        out.push_str("\n)\n");
    }
    if let Some((last, rest)) = commit.split_last() {
        out.push_str("(commit\n");
        for f in rest {
            out.push_str("  ");
            out.push_str(f);
            out.push('\n');
        }
        out.push_str("  ");
        out.push_str(last);
        out.push_str(")\n");
    }
    out
}

/// What a saved file carries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Saved {
    /// The program's text.
    pub program: String,
    /// The modules, `(name, text)`, in the file's order.
    pub modules: Vec<(String, String)>,
    /// The choices, each a fact's text as the printer writes it; `None`
    /// where the file has no `(commit …)`.
    pub commit: Option<Vec<String>>,
}

/// Why a file's text did not load: a parse error, printed as the parser
/// prints it, or a load error, which a command prints after `kb load error: `.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextError {
    Parse(String),
    Load(KbLoadError),
}

impl std::fmt::Display for TextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextError::Parse(e) => f.write_str(e),
            TextError::Load(e) => write!(f, "kb load error: {e}"),
        }
    }
}

/// The name a saved file's program is read under, which its positions name:
/// `<program saved in FILE>`, as a container's stored program is
/// `<program stored in FILE>`.
pub fn program_name(file: Option<&str>) -> String {
    format!("<program saved in {}>", file.unwrap_or("<string>"))
}

/// The parts of a saved file, or `None` where `text` does not open with
/// `(program`. A saved file is one `(program …)`, then its `(module …)`s,
/// each name once, then at most one `(commit …)`, and nothing else.
pub fn split(text: &str, name: Option<&str>) -> Result<Option<Saved>, TextError> {
    if !opens_saved(text) {
        return Ok(None);
    }
    let mut ast = Ast::new();
    let forms = parse(&mut ast, text, name).map_err(|e| TextError::Parse(e.to_string()))?;
    split_forms(&ast, &forms).map(Some)
}

/// A saved file's parts, read off its parsed top-level forms ([`split`]).
fn split_forms(ast: &Ast, forms: &[NodeId]) -> Result<Saved, TextError> {
    let shape = |at: NodeId, what: &str| {
        TextError::Load(KbLoadError(format!(
            "{what} at {} — a saved file is one (program …), then the (module …)s it carries, \
             each once, then at most one (commit …), and nothing else",
            loc_repr(ast, ast.loc(at))
        )))
    };
    let mut saved = Saved {
        program: String::new(),
        modules: Vec::new(),
        commit: None,
    };
    for (i, &form) in forms.iter().enumerate() {
        match (ast.head_name(form), i) {
            (Some("program"), 0) => saved.program = carried_text(ast, form).to_string(),
            (Some("program"), _) => return Err(shape(form, "a second (program …)")),
            (Some("module"), _) if saved.commit.is_none() => {
                let args = ast.form_args(form);
                let module = ast.atom_name(args[0]).unwrap_or("").to_string();
                if saved.modules.iter().any(|(n, _)| *n == module) {
                    return Err(shape(form, &format!("a second (module {module} …)")));
                }
                saved
                    .modules
                    .push((module, carried_text(ast, form).to_string()));
            }
            (Some("module"), _) => return Err(shape(form, "a (module …) after the (commit …)")),
            (Some("commit"), _) if saved.commit.is_none() => {
                let facts: Vec<String> = ast
                    .form_args(form)
                    .iter()
                    .map(|&f| dump_compact(ast, f))
                    .collect();
                saved.commit = Some(facts);
            }
            (Some("commit"), _) => return Err(shape(form, "a second (commit …)")),
            _ => {
                return Err(shape(
                    form,
                    "a form that is no (program …), (module …) or (commit …)",
                ));
            }
        }
    }
    Ok(saved)
}

/// The text a wrapper carries: its last argument, the string the grammar's
/// builder made of its bytes.
fn carried_text(ast: &Ast, form: NodeId) -> &str {
    match ast.form_args(form).last().map(|&a| ast.node(a)) {
        Some(Node::Str(s)) => ast.sym(s),
        _ => unreachable!("a wrapper carries its text (ein.y's Saved)"),
    }
}

/// A file's text loaded: its program's KB and the program's forms as
/// written.
pub struct Loaded {
    pub kb: Kb,
    pub forms: Vec<NodeId>,
    /// What the parse took, a saved file's two parses together; the rest of
    /// the call is the load's.
    pub parse: std::time::Duration,
}

/// Load a file's `text`, about query number `query`: a program, against
/// `base_dir`, or a saved file, its program against the modules it carries
/// ([`split`]). Either way the KB's [`ein_core::Program::sources`] is what
/// the load read, and a saved file's [`ein_core::Program::commit`] its
/// choices.
///
/// A saved file is loaded from what it carries and compares nothing: a
/// module of the standard library it carries is loaded as carried, whatever
/// this binary's library holds or whether it has one (Q-M1f.401).
pub fn load_text(
    ast: &mut Ast,
    terms: &mut Terms,
    text: &str,
    name: Option<&str>,
    base_dir: Option<&Path>,
    query: usize,
    opts: &LoadOptions,
) -> Result<Loaded, TextError> {
    let t = std::time::Instant::now();
    let Some(saved) = split(text, name)? else {
        let forms = parse(ast, text, name).map_err(|e| TextError::Parse(e.to_string()))?;
        let parse_time = t.elapsed();
        let resolver = Resolver::new();
        let mut kb = load_query_by(&resolver, ast, terms, &forms, base_dir, query, opts)
            .map_err(TextError::Load)?;
        kb.program_mut().sources = Some(Arc::new(Sources {
            program: text.to_string(),
            modules: resolver.modules_read(),
        }));
        return Ok(Loaded {
            kb,
            forms,
            parse: parse_time,
        });
    };
    // The file's own parse, and then its program's.
    let split = t.elapsed();
    let mut loaded = load_saved(ast, terms, saved, name, query, opts)?;
    loaded.parse += split;
    Ok(loaded)
}

/// A saved file's forms, parsed, loaded as [`load_text`] loads its text: what
/// a load handed a file's forms rather than its text reads
/// (`crate::from_ir::load_query_by`, where the first form is a `(program …)`),
/// so every load of a saved file is the node it names.
pub(crate) fn load_forms(
    ast: &mut Ast,
    terms: &mut Terms,
    forms: &[NodeId],
    query: usize,
    opts: &LoadOptions,
) -> Result<Kb, KbLoadError> {
    let into = |e: TextError| match e {
        TextError::Parse(e) => KbLoadError(e),
        TextError::Load(e) => e,
    };
    let saved = split_forms(ast, forms).map_err(into)?;
    let name = forms
        .first()
        .and_then(|&f| ast.loc(f))
        .map(|l| ast.file(l.file).to_string());
    load_saved(ast, terms, saved, name.as_deref(), query, opts)
        .map(|l| l.kb)
        .map_err(into)
}

/// The program a saved file carries, parsed under its own name and loaded
/// against the modules the file carries, with its sources and its choices.
/// Nothing is compared with this binary's standard library: the file is
/// self-contained, and what it carries is what it loads (Q-M1f.401).
fn load_saved(
    ast: &mut Ast,
    terms: &mut Terms,
    saved: Saved,
    name: Option<&str>,
    query: usize,
    opts: &LoadOptions,
) -> Result<Loaded, TextError> {
    let t = std::time::Instant::now();
    let file = name.unwrap_or("<string>");
    let forms = parse(ast, &saved.program, Some(&program_name(name)))
        .map_err(|e| TextError::Parse(e.to_string()))?;
    let parse_time = t.elapsed();
    let resolver = Resolver::carrying(file, &saved.modules);
    let mut kb =
        load_query_by(&resolver, ast, terms, &forms, None, query, opts).map_err(TextError::Load)?;
    let modules = resolver.modules_read();
    let p = kb.program_mut();
    p.sources = Some(Arc::new(Sources {
        program: saved.program,
        modules,
    }));
    p.commit = saved.commit.map(Vec::into_boxed_slice);
    Ok(Loaded {
        kb,
        forms,
        parse: parse_time,
    })
}

/// The forms a file's `text` writes, parsed and not loaded: a program's, or
/// the program a saved file carries, under [`program_name`]'s name. What a
/// command that draws a program as written reads (`ein render program`).
pub fn parse_text(ast: &mut Ast, text: &str, name: Option<&str>) -> Result<Vec<NodeId>, TextError> {
    match split(text, name)? {
        None => parse(ast, text, name).map_err(|e| TextError::Parse(e.to_string())),
        Some(saved) => parse(ast, &saved.program, Some(&program_name(name)))
            .map_err(|e| TextError::Parse(e.to_string())),
    }
}
