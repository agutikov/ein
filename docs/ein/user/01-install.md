# Installing `ein`

**There are two channels:** a binary from a release, or `cargo install` from a checkout.

`ein` is one self-contained executable. The `std.*` standard library is compiled into it
([the standard library](04-reference/04-stdlib.md)), so a downloaded binary solves `zebra2.ein` on a machine with nothing
else on it. What can differ between a binary and a checkout beside it is *which* standard library a run reads, and
`ein --version` answers that in one line ([§ Verify what you got](#verify-what-you-got)).

## Download a binary

A release carries one executable per platform, and `SHA256SUMS`:

| file | platform | notes |
|---|---|---|
| `ein-linux-x86_64` | Linux x86_64 | glibc ≥ 2.35 (built on Ubuntu 22.04) |
| `ein-linux-aarch64` | Linux aarch64 | the same |
| `ein-macos-universal2` | macOS | one file, both architectures |
| `ein-windows-x86_64.exe` | Windows x86_64 | |
| `ein-linux-x86_64-musl` | Linux x86_64 | **statically linked**, best effort; see below |

```sh
curl -LO https://github.com/agutikov/ein/releases/latest/download/ein-linux-x86_64
curl -LO https://github.com/agutikov/ein/releases/latest/download/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
chmod +x ein-linux-x86_64 && ./ein-linux-x86_64 --version
```

If the repository's releases page lists no release, install from a checkout: the next section.

**The musl build is a fallback, not the default.** It is for a system whose glibc is too old to run the others at
all. It is built with `--no-default-features`, since the bundled allocator needs a C++ toolchain for musl: it uses
the system allocator, and has no `.einb` container and no `--jobs` fan-out. It answers every program the same as the
others, and can be slower. Take the glibc binary unless glibc is the problem.

## `cargo install`

The Rust workspace lives in `ein.rs/`, one directory below the repository's root, so there is no manifest where
`cargo install --git` looks for one. Clone, then install by path:

```sh
git clone https://github.com/agutikov/ein
cargo install --path ein/ein.rs/crates/ein-cli --locked
ein --version
```

This builds with the default features, which needs **`cmake` and a C++ compiler** for the bundled allocator. Without
them:

```sh
cargo install --path ein/ein.rs/crates/ein-cli --locked --no-default-features
```

That build uses the system allocator and also leaves out `ein kb` and `--jobs`; `ein --version`'s `features` line
says which build you have. Neither choice changes an answer.

To run `ein` from a checkout without installing it, build it there:

```sh
cargo build --release --manifest-path ein.rs/Cargo.toml -p ein-cli
ein.rs/target/release/ein --version
```

The toolchain is pinned by [`ein.rs/rust-toolchain.toml`](../../../ein.rs/rust-toolchain.toml).

## Verify what you got

`ein --version` reports four things a version number does not:

```text
$ ein --version
ein 0.1.0
protocol   ein-events/1
container  einb/2.0
features   einb, parallel, snmalloc
stdlib     sha256:7592561c…60c5a3  checkout /path/to/ein/stdlib
```

| line | what it answers |
|---|---|
| `ein <semver>` | the engine's version: one number for the whole workspace |
| `protocol` | the schema a `--events` file declares on its first line |
| `container` | the `.einb` format, major.minor, this build reads and writes, or `none` |
| `features` | what was compiled in. **`--jobs N` is accepted and inert without `parallel`**, and `ein kb` is not a command without `einb`; neither absence has an error message, so this line is where they show |
| `stdlib` | the SHA-256 of the `std.*` manifest **this binary will load**, and which of the three places it was found in |

The `stdlib` line is the one worth reading twice. `std.*` is resolved at run time in three steps: `$EIN_STDLIB`, then
a `stdlib/` directory found by walking up from the executable, then the copy compiled into the binary. So the *same*
binary can load different modules depending on where it runs from, and the line says which: `$EIN_STDLIB <path>`,
`checkout <path>` or `embedded`. The digest is over the resolved `stdlib/MANIFEST.sha256`, printed whole, so checking
a binary against a checkout is a command you already have:

```sh
ein --version | grep stdlib          # sha256:7592561c…  checkout /path/to/ein/stdlib
sha256sum /path/to/ein/stdlib/MANIFEST.sha256
```

Equal digests mean the binary and that checkout agree about the standard library. A binary that reports `embedded`
while you edit a checkout's `stdlib/` is reading neither what you edited nor an error message; this line ends that
confusion in one command.

## Point it at a different stdlib

```sh
EIN_STDLIB=/path/to/stdlib ein solve puzzle.ein
```

`$EIN_STDLIB` wins over both other steps, silently, which is why `ein --version` names it rather than printing a bare
path.

**A directory is the standard library only if it holds `MANIFEST.sha256`**, for the walk up from the executable,
which passes over a `stdlib/` without one, and for `$EIN_STDLIB`. Point it at a directory without the marker and the
first `(import std.…)` says so, naming the variable:

```text
kb load error: (import std.algebra) — $EIN_STDLIB names /tmp/x, which has no MANIFEST.sha256 — a directory is the stdlib only if it carries the marker, which is the same test the checkout walk applies (unset $EIN_STDLIB to use the checkout or the embedded copy, …) (None)
```

The check is made at the **first `std.*` import**, so a program that imports nothing from the standard library is
not refused for a variable it never reads. `ein --version` does not refuse either: it prints
`stdlib     unreadable  $EIN_STDLIB <path>` and goes on.

**The walk prefers whatever checkout it lands under.** A binary copied under an unrelated tree that holds a
`stdlib/MANIFEST.sha256` reads *that* standard library. No warning is given when the resolved manifest differs from
the copy compiled into the binary, because that difference is the normal state of editing the standard library: a
checkout's edited module takes effect with no rebuild, and a warning there would fire on the working case. The
instrument for the hazard is `ein --version`, which names the step and the path.

`$EIN_STDLIB` is the only environment variable a normal install needs. Every `EIN_*` name, every `(config …)` field and
every option, each with whether it can change an answer, is [the configuration reference](04-reference/06-config.md).

## There is no Python package

Ein has no binding for another language
([Q-M1a.23](../../decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md)), and publishes no Python
package. To drive Ein from Python, or from any language, run the binary and read its output:
`ein solve … --json-summary out.json` for the answer's blocks and counts
([the read-out](04-reference/08-read-out.md)), `--events e.jsonl` for the step-by-step narration
([the protocol](../ein.rs/05-events.md)), `--dump-answer DIR` for the answer as files
([the answer dump](04-reference/10-answer-dump.md)). To embed Ein in a Rust program, link the crates
([the Rust API](../ein.rs/04-api.md)).

## See also

- [`README.md`](../../../README.md): what Ein is, and the quickstart.
- [The guide](02-guide/README.md): learn Ein by solving the Zebra puzzle.
- [The standard library](04-reference/04-stdlib.md): what `std.*` holds.
