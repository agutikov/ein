//! Executing one (entry, run) cell against one implementation.
//!
//! The harness **shells out**. It links neither implementation, on purpose:
//! a harness that links the thing it tests can only find the bugs it does not
//! share, and half the point of the port is that the two engines are separate
//! programs with separate CLIs.
//!
//! Everything an implementation can be observed to do is captured here —
//! exit code, stdout, stderr, and every file it wrote under the run's own
//! output directory (the markdown trace, the DOT, the `--dump-states` tree,
//! the JSON summary). Comparison is [`crate::tier`]'s job.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::normalise;

/// One implementation under test: a command prefix plus env overrides.
#[derive(Debug, Clone)]
pub struct Impl {
    /// `a` / `b` — used for output paths and report columns.
    pub side: String,
    /// The command and its fixed leading arguments, e.g.
    /// `["python3", "-m", "ein.cli"]`. Split on whitespace; no shell quoting.
    pub prefix: Vec<String>,
    /// Environment overrides applied on top of the harness's own environment.
    /// This is how the determinism sweep is expressed: one implementation,
    /// two `PYTHONHASHSEED`s.
    pub env: Vec<(String, String)>,
}

impl Impl {
    pub fn parse(side: &str, cmd: &str) -> Result<Impl, String> {
        let prefix: Vec<String> = cmd.split_whitespace().map(str::to_string).collect();
        if prefix.is_empty() {
            return Err(format!("--impl-{side}: empty command"));
        }
        Ok(Impl {
            side: side.to_string(),
            prefix,
            env: Vec::new(),
        })
    }
}

/// What one run of one implementation produced.
#[derive(Debug)]
pub struct Capture {
    pub code: i32,
    /// Normalised.
    pub stdout: String,
    /// Normalised.
    pub stderr: String,
    /// Path (relative to the run's output dir) → normalised content.
    pub files: BTreeMap<String, String>,
    /// Raw text of `summary.json`, if the run produced one.
    pub summary: Option<String>,
    pub wall: Duration,
    pub timed_out: bool,
}

/// Where a cell's captured streams land inside its output directory. Excluded
/// from the produced-file comparison, since they are compared as streams.
pub const STDOUT_FILE: &str = "stdout.txt";
pub const STDERR_FILE: &str = "stderr.txt";

/// Run `imp` on `argv` with cwd `repo`, writing artefacts under `out`.
///
/// **stdout and stderr are redirected to files, not pipes.** A pipe holds
/// ~64 KB; `render lattice` writes more DOT than that, and a child that fills
/// its pipe blocks on `write` — so the timeout poll below would spin until it
/// gave up on a program that had merely been talkative. (Found the first time
/// the harness ran the whole corpus: two 0.3 s cells hung for two minutes.)
/// `wait_with_output` cannot fix it either — the deadlock happens *before* the
/// wait. Files have no such limit, cost nothing here, and leave the two sides'
/// output on disk where a hand investigation wants it.
pub fn execute(
    imp: &Impl,
    argv: &[String],
    repo: &Path,
    out: &Path,
    timeout: Duration,
) -> Result<Capture, String> {
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let out_path = out.join(STDOUT_FILE);
    let err_path = out.join(STDERR_FILE);
    let mk = |p: &Path| std::fs::File::create(p).map_err(|e| format!("{}: {e}", p.display()));
    let mut cmd = Command::new(&imp.prefix[0]);
    cmd.args(&imp.prefix[1..])
        .args(argv)
        .current_dir(repo)
        .stdin(Stdio::null())
        .stdout(Stdio::from(mk(&out_path)?))
        .stderr(Stdio::from(mk(&err_path)?));
    for (k, v) in &imp.env {
        cmd.env(k, v);
    }
    let t0 = Instant::now();
    let mut child = cmd.spawn().map_err(|e| format!("{}: {e}", imp.prefix[0]))?;

    // std has no timed wait; poll, then kill. The harness runs many of these
    // in parallel, so the poll interval is a compromise between wasted wakeups
    // and latency on short runs.
    let mut timed_out = false;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {
                if t0.elapsed() > timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    timed_out = true;
                    break None;
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            Err(e) => return Err(format!("wait: {e}")),
        }
    };
    let wall = t0.elapsed();

    let repo_s = repo.display().to_string();
    let out_s = out.display().to_string();
    let read = |p: &Path| {
        let raw = std::fs::read(p).unwrap_or_default();
        normalise::normalise_run(&String::from_utf8_lossy(&raw), &repo_s, &out_s)
    };
    let stdout = read(&out_path);
    let stderr = read(&err_path);
    let summary = std::fs::read_to_string(out.join("summary.json")).ok();
    let files = collect(out, &repo_s, &out_s)?;

    Ok(Capture {
        code: status.and_then(|s| s.code()).unwrap_or(-1),
        stdout,
        stderr,
        files,
        summary,
        wall,
        timed_out,
    })
}

/// Every file under `root`, normalised, keyed by relative path.
fn collect(root: &Path, repo: &str, out_dir: &str) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("{}: {e}", dir.display()))?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let rel = path
                .strip_prefix(root)
                .map_err(|e| e.to_string())?
                .display()
                .to_string();
            // The captured streams are compared as streams, not as files.
            if rel == STDOUT_FILE || rel == STDERR_FILE {
                continue;
            }
            let raw = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let text = String::from_utf8_lossy(&raw);
            let value = if path.file_name().is_some_and(|n| n == "state_hash.txt") {
                // A `PYTHONHASHSEED`-salted digest — shape, never value.
                normalise::digest_shape(&text)
            } else {
                normalise::normalise_run(&text, repo, out_dir)
            };
            out.insert(rel, value);
        }
    }
    Ok(out)
}
