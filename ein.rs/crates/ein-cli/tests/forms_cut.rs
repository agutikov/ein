//! M1e2 S1e2.3.3 — **the forms after the read-out share its 100 lines.**
//!
//! The user's ruling (a) of 2026-11-03: the six forms (`--models key|grid`,
//! `-p`, `-P`, `-f`, `--diff-models`, `-H`) are listings, and share stdout's
//! 100 lines with the read-out's. Each is drawn from the states stdout
//! showed, prints its items while what the read-out left lasts, at least
//! one, and then where the rest is; `--dump-answer DIR` writes each uncut,
//! one file per form (`DIR/forms/<form>.txt`, named in `index.json`), and
//! `answer.txt` stays the read-out uncut.
//!
//! `pets.ein` is the witness: R is 11 339 states, the read-out lists 38 of
//! them, and before the stage `--models grid` printed 34 143 lines.

use std::path::Path;
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

const PETS: &str = "examples/saturation/type-exclusivity/pets.ein";
/// Seven states, every form whole: nothing is cut.
const THREE: &str = "examples/lattice/02_genuine_3set_death.ein";

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn scratch(name: &str) -> Scratch {
    Scratch::new(&format!("ein-forms-cut-{name}"))
}

/// The lines after the read-out: what `extra` added to stdout. The header
/// names the flags, so the read-out is compared below it.
fn added<'a>(bare: &str, with: &'a str) -> Vec<&'a str> {
    let body = |s: &str| {
        s.split_once('\n')
            .map_or(String::new(), |x| x.1.to_string())
    };
    let (b, w) = (body(bare), body(with));
    assert!(w.starts_with(&b), "the read-out moved:\n{with}");
    let n = bare.lines().count();
    with.lines().skip(n).collect()
}

/// The forms, their files and their flags.
const FORMS: [(&str, &[&str]); 6] = [
    ("hyp-stats", &["-H"]),
    ("grid", &["--models", "grid"]),
    ("key", &["--models", "key"]),
    ("print-final-state", &["-p"]),
    ("print-final-positive", &["-P"]),
    ("print-final-hfacts", &["-f"]),
];

fn dump(file: &str, flags: &[&str], dir: &Path) -> Run {
    let d = dir.to_string_lossy().to_string();
    let mut a = vec!["solve", file];
    a.extend(flags);
    a.extend(["--dump-answer", &d]);
    ein(&a)
}

/// **On `pets`, every form prints one item after the read-out's cut and
/// names its file**, the file holds the form uncut — one item per listed
/// state, stdout's first among them byte for byte — and `index.json` names
/// it. The read-out, the summary and `answer.txt` are the same with the form
/// and without, and stdout is the same with `--dump-answer` and without.
#[test]
fn on_pets_each_form_prints_what_fits_and_the_dump_holds_it_uncut() {
    let dir = scratch("pets");
    let bare_dir = dir.join("bare");
    let bare = dump(PETS, &[], &bare_dir);
    assert_eq!(bare.code, 0, "{}", bare.err);
    assert!(
        bare.out.contains("states (k_s)      11 339"),
        "the witness moved:\n{}",
        bare.out
    );
    let answer_txt = std::fs::read_to_string(bare_dir.join("answer.txt")).expect("answer.txt");
    let answer_json = std::fs::read_to_string(bare_dir.join("answer.json")).expect("answer.json");
    for (name, flags) in FORMS {
        let d = dir.join(name);
        let r = dump(PETS, flags, &d);
        assert_eq!(r.code, 0, "{flags:?}: {}", r.err);
        let lines = added(&bare.out, &r.out);
        // The read-out spent the 100 lines: one item, the form's head, and
        // the line that says where the rest is. 20 lines is the key's head
        // (six), H's (four), a state of `-p` (nine) with room to spare,
        // against 34 143 for `--models grid` before the stage.
        assert!(
            lines.len() <= 20,
            "{flags:?} printed {} lines after the read-out:\n{}",
            lines.len(),
            lines.join("\n")
        );
        let pointer = format!("writes the form uncut to DIR/forms/{name}.txt");
        assert!(
            lines
                .last()
                .is_some_and(|l| l.starts_with("  ;; not shown: ") && l.ends_with(&pointer)),
            "{flags:?}: the cut does not say where the rest is:\n{}",
            lines.join("\n")
        );
        let file = std::fs::read_to_string(d.join("forms").join(format!("{name}.txt")))
            .unwrap_or_else(|e| panic!("{flags:?}: no forms/{name}.txt: {e}"));
        let index: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(d.join("index.json")).unwrap()).unwrap();
        assert_eq!(
            index["forms"],
            serde_json::json!([{ "form": name, "path": format!("forms/{name}.txt") }]),
            "{flags:?}"
        );
        // What the form adds is nothing an answer is: the uncut read-out,
        // below the header that names the flags, and the summary are the
        // bare run's.
        let below = |t: &str| {
            t.split_once('\n')
                .map_or(String::new(), |x| x.1.to_string())
        };
        assert_eq!(
            below(&std::fs::read_to_string(d.join("answer.txt")).unwrap()),
            below(&answer_txt),
            "{flags:?} moved answer.txt"
        );
        assert_eq!(
            std::fs::read_to_string(d.join("answer.json")).unwrap(),
            answer_json,
            "{flags:?} moved answer.json"
        );
        // The flag is additive: stdout is the same without the directory.
        let mut a = vec!["solve", PETS];
        a.extend(flags);
        assert_eq!(ein(&a).out, r.out, "{flags:?}: --dump-answer moved stdout");
        // Stdout's items are the file's first, byte for byte, where the
        // form is drawn state by state; each listed state is an item.
        let printed: Vec<&str> = lines[..lines.len() - 1].to_vec();
        match name {
            "grid" | "print-final-state" | "print-final-positive" | "print-final-hfacts" => {
                let state = if name == "grid" {
                    "  ── state "
                } else {
                    "── state "
                };
                assert_eq!(
                    file.lines().filter(|l| l.starts_with(state)).count(),
                    11_339,
                    "{name}: one item per listed state"
                );
                // The head names the set it was drawn from: the shown states
                // on stdout, every state of R in the file.
                // Stdout's one item is the file's first, line for line: from
                // the first state's line to the end of the block.
                let first = |lines: &[&str]| -> Vec<String> {
                    lines
                        .iter()
                        .skip_while(|l| !l.starts_with(state))
                        .take_while(|l| !l.is_empty())
                        .map(|l| l.to_string())
                        .collect()
                };
                let file_lines: Vec<&str> = file.lines().collect();
                assert_eq!(
                    first(&printed),
                    first(&file_lines),
                    "{name}: stdout's item is not the file's first"
                );
                if name == "grid" {
                    // The head names the set it was drawn from: the states
                    // stdout showed, and in the file every state of R.
                    assert!(
                        printed
                            .join("\n")
                            .contains("models — the 38 states shown above, of R's 11 339"),
                        "{}",
                        printed.join("\n")
                    );
                    assert!(file.contains("models — every state of R, each as a grid"));
                }
            }
            "hyp-stats" => {
                // A relation's line is never cut: stdout names every relation.
                for l in file
                    .lines()
                    .filter(|l| l.starts_with("  ") && !l.starts_with("   "))
                {
                    assert!(printed.contains(&l), "{name}: `{l}` was cut");
                }
                assert_eq!(file.lines().filter(|l| l.starts_with("    (")).count(), 15);
            }
            "key" => {
                assert!(
                    printed.join("\n").contains(
                        "models — the determining key of the 38 states shown above, of R's 11 339"
                    ),
                    "{}",
                    printed.join("\n")
                );
                assert!(
                    file.contains("models — the determining key of every state of R"),
                    "{file}"
                );
            }
            _ => unreachable!(),
        }
    }
}

/// **Where nothing is cut, the form on stdout is its file**: seven states,
/// every form whole, no `not shown` line, and `forms/<form>.txt` is exactly
/// what stdout printed after the read-out. `--diff-models` is a form too.
#[test]
fn where_nothing_is_cut_stdout_is_the_file() {
    let dir = scratch("three");
    let want = dir.join("want.txt");
    std::fs::write(
        &want,
        "closes a-prop b-prop c-prop\nmodel\n(a-prop X)\n(b-prop X)\nmodel\n(a-prop X)\n(c-prop X)\n",
    )
    .unwrap();
    let w = want.to_string_lossy().to_string();
    let bare = ein(&["solve", THREE]);
    let mut forms: Vec<(&str, Vec<&str>)> = FORMS.iter().map(|(n, f)| (*n, f.to_vec())).collect();
    forms.push(("diff-models", vec!["--diff-models", &w]));
    for (name, flags) in forms {
        let d = dir.join(name);
        let r = dump(THREE, &flags, &d);
        assert_eq!(r.code, 0, "{flags:?}: {}", r.err);
        let lines = added(&bare.out, &r.out);
        assert!(
            !lines.iter().any(|l| l.contains(";; not shown")),
            "{flags:?} was cut on seven states:\n{}",
            r.out
        );
        let file = std::fs::read_to_string(d.join("forms").join(format!("{name}.txt"))).unwrap();
        assert_eq!(
            file,
            format!("{}\n", lines.join("\n")),
            "{flags:?}: stdout's form is not its file"
        );
    }
}

/// **`ein oracle` writes no answer directory**, so its forms' cut names the
/// command that does, and a second dump into one directory leaves no form
/// of the first: `forms/` is the writer's, cleared as `models/` is.
#[test]
fn the_oracle_points_at_solve_and_a_second_dump_leaves_no_stale_form() {
    let r = ein(&[
        "oracle",
        "examples/saturation/type-exclusivity/colors.ein",
        "-p",
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(
        r.out.contains(
            "— ein solve FILE --dump-answer DIR writes the form uncut to \
             DIR/forms/print-final-state.txt"
        ),
        "{}",
        r.out
    );
    let dir = scratch("twice");
    let d = dir.join("d");
    assert_eq!(dump(THREE, &["-p", "-H"], &d).code, 0);
    assert!(d.join("forms/print-final-state.txt").exists());
    assert_eq!(dump(THREE, &["--models", "grid"], &d).code, 0);
    let left: Vec<String> = std::fs::read_dir(d.join("forms"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(left, ["grid.txt"], "a stale form survived");
    // `--count` lists no state and refuses every form (M1e2 T1e2.1c.4.6),
    // and its directory names no form.
    assert_eq!(ein(&["solve", THREE, "--count", "-p"]).code, 2);
    assert_eq!(dump(THREE, &["--count"], &d).code, 0);
    assert!(!d.join("forms").exists(), "--count's dump kept forms/");
}
