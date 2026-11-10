//! **Unicode only inside a string literal or a comment, and a module read as
//! the program is** — M1f S1f.5.22, on the user's ruling of 2026-11-10 on the
//! close review's finding 2: *"Unicode is not allowed anywhere else than
//! inside string literals, and the error must say it explicitely"*, and
//! Q-M1f.65 (b), *"Comments may hold Unicode"*.
//!
//! A character that is not ASCII is written only inside a string literal,
//! after `:why`, `:source` or `:goal-text`, or in a `;` or `#| … |#` comment.
//! Anywhere else the lexer stops at it (`ein.l`: no rule but a string's and
//! the skip rule's comments matches one), and the parse error names it, its
//! code point and where Unicode is allowed:
//!
//! ```text
//! x.ein:7:4: 'é' (U+00E9) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
//! ```
//!
//! Until S1f.5.22 such a character was taken into a `BAD` token, *a token that
//! is not a name*, wherever it was glued to one, and a Unicode space, a
//! no-break space, U+3000, U+2028 or U+0085, was a separator, `\s` being
//! Unicode's in the skip rule, so a program writing one loaded and was
//! answered as if it were a space (W73).
//!
//! And an imported module is read through the read every command makes of
//! the program it is given (`ein_ir::read`), so a module that is not UTF-8,
//! one that cannot be read, a directory and a `.einb` container are refused
//! with that read's line, naming the module's path, where until then each was
//! `module not found at … (None)` (the close review's finding 2, W74).
//!
//! Every place a token can stand, each on every command that reads a
//! program, and every character this file names in every place through the
//! parser; the register's witnesses; a string and a comment holding each
//! character, loaded and answered by every command and through a container;
//! and every way a module can fail to be read, on every command.

#![cfg(feature = "einb")]

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein_in(dir: &Path, args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

const HEAD: &str = "(relation a Thing :why \"{?1} is an a\")\n\
                    (relation p Thing :why \"{?1} is a p\")\n\
                    (relation q Thing :why \"{?1} is a q\")\n\
                    (is-a k Thing)\n\
                    (is-a j Thing)\n\
                    (a k)\n\
                    (rule base () :match (a ?x) :assert (q ?x) :why \"{?x} is an a, so a q\")\n";

/// A claim `ein test` checks, which holds of every program here that loads.
const CLAIM: &str = "(query :goal (is-a ?x Thing) :expect (model (is-a j Thing) (is-a k Thing)))\n";

/// The characters: a name for each, the character, and how the line quotes
/// it, itself or, where a reader cannot see it, the escape a string writes it
/// with (`ein_ir::grammar::shown_char`).
const CHARS: [(&str, char, &str); 12] = [
    ("e-acute", 'é', "é"),
    ("arrow", '→', "→"),
    ("box-rule", '─', "─"),
    ("zero-width-space", '\u{200b}', "\\u{200b}"),
    ("no-break-space", '\u{a0}', "\\u{a0}"),
    ("fullwidth-paren", '（', "（"),
    ("zebra", '🦓', "🦓"),
    ("ideographic-space", '\u{3000}', "\\u{3000}"),
    ("line-separator", '\u{2028}', "\\u{2028}"),
    ("byte-order-mark", '\u{feff}', "\\u{feff}"),
    ("next-line", '\u{85}', "\\u{85}"),
    ("right-to-left-override", '\u{202e}', "\\u{202e}"),
];

/// The line a character is refused with, after the file and its position.
fn line_for(shown: &str, c: char) -> String {
    format!(
        "'{shown}' (U+{:04X}) is not ASCII: Unicode is allowed only inside a string literal \
         (after :why, :source or :goal-text) or in a comment",
        c as u32
    )
}

/// A rule `r` over `m` and `a`.
fn rule(m: &str, a: &str) -> String {
    format!("(rule r () :match {m} :assert {a} :why \"r\")\n")
}

/// `(place, the program after HEAD with `X` where the character goes)`: every
/// place a token can stand, as the stage's census wrote them (T1f.5.22.2).
/// `@FIRST` is the file's first character, before HEAD.
fn places() -> Vec<(&'static str, String)> {
    vec![
        ("a fact's relation, glued", "(pX k)\n".into()),
        ("a fact's value, glued inside", "(p kXj)\n".into()),
        ("a fact's value, alone", "(p X)\n".into()),
        ("a fact's value, glued after", "(p kX)\n".into()),
        (
            "a declared relation's name",
            "(relation tX Thing :why \"t\")\n".into(),
        ),
        (
            "a declared relation's type",
            "(relation t ThingX :why \"t\")\n".into(),
        ),
        ("an is-a's object", "(is-a mX Thing)\n".into()),
        ("a nested fact's relation", "(p (qX k))\n".into()),
        ("a stored negation", "(not (pX j))\n".into()),
        ("after a keyword", "(relation t Thing :whyX \"t\")\n".into()),
        (
            "a rule's name",
            "(rule rX () :match (a ?x) :assert (q ?x) :why \"r\")\n".into(),
        ),
        ("a rule's variable", rule("(a ?xX)", "(q ?x)")),
        ("a premise's relation", rule("(aX ?x)", "(q ?x)")),
        ("a conclusion's value", rule("(a ?x)", "(q kX)")),
        (
            "a guard's value",
            rule("(and (a ?x) (neq ?x jX))", "(q ?x)"),
        ),
        (
            "an hrule's conclusion",
            "(hrule h () :match (a ?x) :assert (p ?xX) :why \"h\")\n".into(),
        ),
        (
            "a macro's name",
            "(macro mX (?v) (p ?v) :why \"m\")\n".into(),
        ),
        (
            "a macro's body",
            format!(
                "(macro m (?v) (pX ?v) :why \"m\")\n{}",
                rule("(and (a ?x) (m ?x))", "(q ?x)")
            ),
        ),
        ("a query's goal", "(query :goal (p ?xX))\n".into()),
        (
            "a query claim's fact",
            "(query :goal (a ?x) :expect (model (a kX)))\n".into(),
        ),
        ("a query's :over", "(query :goal (a ?x) :over aX)\n".into()),
        (
            "a region claim's fact",
            "(solve :expect (region (model (a kX))))\n".into(),
        ),
        (
            "a solve's list",
            "(solve :hypothesis-relations (pX))\n".into(),
        ),
        (
            "a config flag's value",
            "(config :lattice-sanity-check trueX)\n".into(),
        ),
        (
            "a config keyword",
            "(config :lattice-sanity-checkX true)\n".into(),
        ),
        (
            "a trace step's name",
            "(trace (step s1X :derives (a k)))\n".into(),
        ),
        ("an import's module", "(import std.macroX)\n".into()),
        ("between two forms", "(p k)X(p j)\n".into()),
        ("a line's first character", "X(p j)\n".into()),
        ("between a paren and a word", "(Xp j)\n".into()),
        (
            "glued after a :why string",
            "(relation t Thing :why \"t\"X)\n".into(),
        ),
        ("after a #", "(p k) #X\n".into()),
        ("in place of a paren", "Xp j)\n".into()),
        ("the file's first character", "@FIRST".into()),
    ]
}

/// The program a place writes with `c`, and where `c` stands in it, 1-based.
fn program(body: &str, c: char) -> (String, usize, usize) {
    let text = if body == "@FIRST" {
        format!("{c}{HEAD}{CLAIM}")
    } else {
        let b = body.replacen('X', &c.to_string(), 1);
        let claim = if b.contains("(query") { "" } else { CLAIM };
        format!("{HEAD}{b}{claim}")
    };
    let at = text.find(c).expect("the character is written");
    let before = &text[..at];
    let line = before.matches('\n').count() + 1;
    let col = before.rsplit('\n').next().unwrap_or("").chars().count() + 1;
    (text, line, col)
}

/// Every command that reads a program, and the exit it gives one it refuses.
const COMMANDS: [(&[&str], i32); 20] = [
    (&["saturate"], 1),
    (&["saturate", "--dump"], 1),
    (&["solve"], 1),
    (&["solve", "--minimal"], 1),
    (&["solve", "--maximal"], 1),
    (&["solve", "--single"], 1),
    (&["solve", "--count"], 1),
    (&["solve", "-n", "1"], 1),
    (&["solve", "--events", "{events}"], 1),
    (&["oracle"], 1),
    (&["test"], 2),
    (&["test", "--events", "{events}"], 2),
    (&["render", "lattice"], 1),
    (&["render", "kb"], 1),
    (&["render", "program"], 1),
    (&["render", "rules"], 1),
    (&["render", "rule", "--name", "base"], 1),
    (&["render", "constraints"], 1),
    (&["kb", "save"], 1),
    (&["kb", "save", "--saturate"], 1),
];

fn argv(cmd: &[&str], file: &str, einb: &str, events: &str) -> Vec<String> {
    let mut args: Vec<String> = cmd
        .iter()
        .map(|a| match *a {
            "{events}" => events.to_string(),
            a => a.to_string(),
        })
        .collect();
    args.push(file.to_string());
    if cmd[0] == "kb" {
        args.push(einb.to_string());
    }
    args
}

/// `file`, in `dir`, refused on every command with `want` as its first line,
/// no container written and nothing fired.
fn refused_everywhere(dir: &Path, file: &str, want: &str) -> Vec<String> {
    let mut bad = Vec::new();
    let einb = format!("{file}.einb");
    let events = format!("{file}.events.jsonl");
    for (cmd, code) in COMMANDS {
        let _ = std::fs::remove_file(dir.join(&events));
        let args = argv(cmd, file, &einb, &events);
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let r = ein_in(dir, &argv);
        let line = r.err.lines().next().unwrap_or("");
        if r.code != code {
            bad.push(format!(
                "ein {argv:?}: exit {}, want {code}: {line}",
                r.code
            ));
        } else if line != want {
            bad.push(format!("ein {argv:?}: {line:?}, want {want:?}"));
        }
        if dir.join(&einb).exists() {
            bad.push(format!(
                "ein {argv:?} wrote a container of a refused program"
            ));
            let _ = std::fs::remove_file(dir.join(&einb));
        }
        if std::fs::read_to_string(dir.join(&events)).is_ok_and(|t| t.contains("\"e\": \"fire\"")) {
            bad.push(format!("ein {argv:?} fired a rule"));
        }
    }
    bad
}

/// Each job on its own thread, at most `width` at a time, the failures in
/// job order.
fn in_parallel<T: Sync>(
    jobs: &[T],
    width: usize,
    f: impl Fn(&T) -> Vec<String> + Sync,
) -> Vec<String> {
    let mut out = Vec::new();
    for chunk in jobs.chunks(width.max(1)) {
        let got: Vec<Vec<String>> = std::thread::scope(|s| {
            let handles: Vec<_> = chunk.iter().map(|j| s.spawn(|| f(j))).collect();
            handles
                .into_iter()
                .map(|h| h.join().expect("a job"))
                .collect()
        });
        out.extend(got.into_iter().flatten());
    }
    out
}

fn width() -> usize {
    std::thread::available_parallelism().map_or(4, |n| n.get().min(16))
}

/// **Every character in every place, through the parser**: the file
/// refused at the character, one error, the line naming it, its code point
/// and where Unicode is allowed.
#[test]
fn every_character_in_every_place_is_refused_by_the_parser() {
    let mut bad = Vec::new();
    for (place, body) in places() {
        for (name, c, shown) in CHARS {
            let (text, line, col) = program(&body, c);
            let want = format!("x.ein:{line}:{col}: {}", line_for(shown, c));
            match ein_ir::parse(&mut ein_ir::Ast::new(), &text, Some("x.ein")) {
                Ok(_) => bad.push(format!("{place}, {name}: parses")),
                Err(e) if e.to_string() != want => bad.push(format!(
                    "{place}, {name}: {:?}, want {want:?}",
                    e.to_string()
                )),
                Err(_) => {}
            }
        }
    }
    assert_eq!(places().len(), 34, "every place the census wrote");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **Every place on every command**, each with one of the characters in
/// turn, and **every character on every command**, in one place: one first
/// line, the same on each, no container and nothing fired.
#[test]
fn a_character_that_is_not_ascii_is_refused_on_every_command() {
    let scratch = Scratch::new("ein-unicode-places");
    let dir = scratch.path();
    let mut jobs: Vec<(String, String, String)> = Vec::new();
    for (i, (place, body)) in places().into_iter().enumerate() {
        let (_, c, shown) = CHARS[i % CHARS.len()];
        let (text, line, col) = program(&body, c);
        let file = format!("place-{i:02}.ein");
        std::fs::write(dir.join(&file), text).expect("written");
        let want = format!("{file}:{line}:{col}: {}", line_for(shown, c));
        jobs.push((format!("{place}, U+{:04X}", c as u32), file, want));
    }
    for (name, c, shown) in CHARS {
        let (text, line, col) = program("(p kX)\n", c);
        let file = format!("char-{name}.ein");
        std::fs::write(dir.join(&file), text).expect("written");
        let want = format!("{file}:{line}:{col}: {}", line_for(shown, c));
        jobs.push((name.to_string(), file, want));
    }
    let bad = in_parallel(&jobs, width(), |(what, file, want)| {
        refused_everywhere(dir, file, want)
            .into_iter()
            .map(|b| format!("{what}: {b}"))
            .collect()
    });
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The register's witnesses, W73's five and W74's, each read from its file,
/// as `wrong_answers.rs` reads one command's run of each, and refused on
/// every command with the one line.
#[test]
fn the_registers_witnesses_are_refused_on_every_command() {
    let root = repo_root();
    let dir = "examples/ein-bugs/wrong-answers";
    let spaces = [
        ("unicode-space-nbsp", '\u{a0}', "\\u{a0}"),
        ("unicode-space-in-a-name", '\u{a0}', "\\u{a0}"),
        ("unicode-space-ideographic", '\u{3000}', "\\u{3000}"),
        ("unicode-space-line-separator", '\u{2028}', "\\u{2028}"),
        ("unicode-space-next-line", '\u{85}', "\\u{85}"),
    ];
    let mut jobs: Vec<(String, String)> = Vec::new();
    for (name, c, shown) in spaces {
        let file = format!("{dir}/{name}.ein");
        let text = std::fs::read_to_string(root.join(&file)).expect("the witness");
        let at = text.find(c).expect("the witness writes its character");
        let before = &text[..at];
        let line = before.matches('\n').count() + 1;
        let col = before.rsplit('\n').next().unwrap_or("").chars().count() + 1;
        jobs.push((
            file.clone(),
            format!("{file}:{line}:{col}: {}", line_for(shown, c)),
        ));
    }
    jobs.push((
        format!("{dir}/import-module-directory.ein"),
        format!(
            "kb load error: IsADirectoryError: [Errno 21] Is a directory: \
             '{dir}/import-module-directory-module.ein'"
        ),
    ));
    let bad = in_parallel(&jobs, width(), |(file, want)| {
        refused_everywhere(&root, file, want)
            .into_iter()
            .map(|b| format!("{file}: {b}"))
            .collect()
    });
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **A string literal and a comment hold any character**: each character in
/// a `:why`, a `:source` and a `:goal-text` string, a `;` comment and a `#| …
/// |#` comment, loaded and answered by every command, and by `saturate`,
/// `solve`, `oracle` and `test` through the container `kb save` writes; and
/// `ein saturate --dump` writes the `:source` string back byte for byte.
#[test]
fn a_string_literal_and_a_comment_hold_any_character() {
    let scratch = Scratch::new("ein-unicode-strings");
    let dir = scratch.path();
    let mut jobs: Vec<(String, String, Option<String>)> = Vec::new();
    for (name, c, _) in CHARS {
        let why = HEAD.replace("\"{?1} is a q\"", &format!("\"{{?1}} is a q {c}\""));
        let cases = [
            ("why", format!("{why}{CLAIM}"), None),
            (
                "source",
                format!("{HEAD}(p j :source \"s {c} t\")\n{CLAIM}"),
                Some(format!(":source \"s {c} t\"")),
            ),
            (
                "goal-text",
                format!(
                    "{HEAD}(query :goal (is-a ?x Thing) :goal-text \"{{?x}} {c}\" \
                     :expect (model (is-a j Thing) (is-a k Thing)))\n"
                ),
                None,
            ),
            ("line-comment", format!("{HEAD};; {c}\n{CLAIM}"), None),
            ("block-comment", format!("{HEAD}#| {c} |#\n{CLAIM}"), None),
        ];
        for (kind, text, dumped) in cases {
            let file = format!("ok-{kind}-{name}.ein");
            std::fs::write(dir.join(&file), text).expect("written");
            jobs.push((file, kind.to_string(), dumped));
        }
    }
    let bad = in_parallel(&jobs, width(), |(file, _, dumped)| {
        let mut bad = Vec::new();
        let einb = format!("{file}.einb");
        let events = format!("{file}.events.jsonl");
        for (cmd, _) in COMMANDS {
            let args = argv(cmd, file, &einb, &events);
            let argv: Vec<&str> = args.iter().map(String::as_str).collect();
            let r = ein_in(dir, &argv);
            if r.code != 0 {
                bad.push(format!(
                    "{file}: ein {argv:?}: exit {}: {}",
                    r.code,
                    r.err.lines().next().unwrap_or("")
                ));
            }
            if cmd == ["saturate", "--dump"]
                && let Some(d) = dumped
                && !r.out.contains(d.as_str())
            {
                bad.push(format!("{file}: saturate --dump does not write {d:?} back"));
            }
        }
        for cmd in ["saturate", "solve", "oracle", "test"] {
            let r = ein_in(dir, &[cmd, &einb]);
            if r.code != 0 {
                bad.push(format!(
                    "{file}: ein {cmd} the container: exit {}: {}",
                    r.code, r.err
                ));
            }
        }
        bad
    });
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The commands that read a program, as in [`COMMANDS`], with a rule `r` to
/// draw.
fn every_command(dir: &Path, file: &str) -> Vec<(String, Run)> {
    let einb = format!("{file}.einb");
    let events = format!("{file}.events.jsonl");
    COMMANDS
        .iter()
        .map(|(cmd, _)| {
            let cmd: Vec<&str> = cmd
                .iter()
                .map(|a| if *a == "base" { "r" } else { *a })
                .collect();
            let args = argv(&cmd, file, &einb, &events);
            let argv: Vec<&str> = args.iter().map(String::as_str).collect();
            (format!("{argv:?}"), ein_in(dir, &argv))
        })
        .collect()
}

/// **An imported module is read as the program a command is given is read**
/// — the close review's finding 2. A module that is not UTF-8, one that is a
/// directory, one that cannot be read and one that is a `.einb` container,
/// each imported by a file-relative name, on every command: one first line,
/// the line the module's own path gives as the program a command is given
/// (but the container's, which an import does not open), behind `kb load
/// error: `, naming the module's path, never `None` and never *not found*.
/// A module that is missing keeps its line, and one holding a character that
/// is not ASCII outside a string is refused at it, as the program is.
#[test]
fn an_imported_module_is_read_as_the_program_is() {
    let scratch = Scratch::new("ein-unicode-imports");
    let dir = scratch.path();
    let module = "(relation q Thing :why \"{?1} is a q\")\n";
    std::fs::write(
        dir.join("m-bytes.ein"),
        [module.as_bytes(), b";; x\n(q \xffk)\n"].concat(),
    )
    .expect("written");
    std::fs::create_dir(dir.join("m-dir.ein")).expect("a directory");
    std::fs::write(dir.join("m-src.ein"), format!("{module}(is-a z Thing)\n")).expect("written");
    let saved = ein_in(dir, &["kb", "save", "m-src.ein", "m-box.ein"]);
    assert_eq!(saved.code, 0, "kb save: {}", saved.err);
    std::fs::write(dir.join("m-nonascii.ein"), format!("{module}(q k\u{e9})\n")).expect("written");
    std::fs::write(dir.join("m-nbsp.ein"), format!("{module}(q\u{a0}k)\n")).expect("written");
    // Unreadable where the permission bits say so: a run as root reads it,
    // and then the case is left out, said so on stderr.
    std::fs::write(dir.join("m-unread.ein"), module).expect("written");
    let unreadable = {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(
            dir.join("m-unread.ein"),
            std::fs::Permissions::from_mode(0o000),
        )
        .expect("chmod");
        std::fs::read(dir.join("m-unread.ein")).is_err()
    };
    if !unreadable {
        eprintln!("m-unread.ein is readable with its mode 000 (root?): its case is not run");
    }
    let canonical = dir.canonicalize().expect("the scratch directory");
    let nonascii = format!(
        "kb load error: {}/m-nonascii.ein:2:5: {}",
        canonical.display(),
        line_for("é", '\u{e9}')
    );
    let nbsp = format!(
        "kb load error: {}/m-nbsp.ein:2:3: {}",
        canonical.display(),
        line_for("\\u{a0}", '\u{a0}')
    );
    let mut cases: Vec<(&str, String)> = vec![
        ("m-bytes", "kb load error: m-bytes.ein:3:4: byte 0xff is not UTF-8: a program is UTF-8 text, or a .einb container".into()),
        ("m-dir", "kb load error: IsADirectoryError: [Errno 21] Is a directory: 'm-dir.ein'".into()),
        ("m-box", "kb load error: m-box.ein: a .einb container, and an imported module is Ein text: import the .ein it was saved from".into()),
        ("m-none", "kb load error: (import m-none) — module not found at m-none.ein (None)".into()),
        ("m-nonascii", nonascii),
        ("m-nbsp", nbsp),
    ];
    if unreadable {
        cases.push((
            "m-unread",
            "kb load error: PermissionError: [Errno 13] Permission denied: 'm-unread.ein'".into(),
        ));
    }
    let main = format!(
        "(import {{M}} :symbols (q))\n{}{CLAIM}",
        HEAD.replace("(relation q Thing :why \"{?1} is a q\")\n", "")
            .replace("(rule base ", "(rule r ")
    );
    let mut bad = Vec::new();
    for (m, want) in &cases {
        let file = format!("main-{m}.ein");
        std::fs::write(dir.join(&file), main.replace("{M}", m)).expect("written");
        for (args, r) in every_command(dir, &file) {
            let line = r.err.lines().next().unwrap_or("");
            let code = if args.contains("\"test\"") { 2 } else { 1 };
            if r.code != code || line != want {
                bad.push(format!(
                    "{m}: ein {args}: exit {} (want {code}), {line:?}, want {want:?}",
                    r.code
                ));
            }
        }
        // The module's line is the one its path gives as the program a
        // command is given, but the container's, which a command opens, and
        // the missing one's, which no read reaches; a module's parse names it
        // by its resolved path, which is the only difference.
        if !matches!(*m, "m-box" | "m-none") {
            let r = ein_in(dir, &["solve", &format!("{m}.ein")]);
            let alone = r.err.lines().next().unwrap_or("");
            let through = want
                .strip_prefix("kb load error: ")
                .expect("the load's line")
                .replace(&format!("{}/", canonical.display()), "");
            if alone != through {
                bad.push(format!(
                    "{m}: as the program, {alone:?}; through the import, {through:?}"
                ));
            }
        }
        if *m != "m-none" && want.contains("None") {
            bad.push(format!("{m}: the line carries `None`: {want}"));
        }
    }
    {
        use std::os::unix::fs::PermissionsExt as _;
        let _ = std::fs::set_permissions(
            dir.join("m-unread.ein"),
            std::fs::Permissions::from_mode(0o644),
        );
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}
