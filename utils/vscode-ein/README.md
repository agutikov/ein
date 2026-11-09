# ein — VSCode syntax highlighting

A lightweight **TextMate grammar** for `.ein` files (the Ein surface
language). No language server, no build step — just regex-based
structural highlighting that any TextMate-grammar consumer can load:
VSCode, Sublime Text, GitHub `linguist`, `bat`, etc.

Built for S1.7c.8;
highlights the **flat** surface (post-S1.7c.4:
no `(ontology …)` / `(facts …)` / `(reasoning …)` / `(rules …)` block
wrappers).

## What it colours

| Element | Example | Scope |
|---------|---------|-------|
| Declarator heads (head position only) | `relation` `rule` `hrule` `macro` `query` `solve` `config` `trace` `import` | `keyword.control.declarator` |
| Declared name | the `symmetric` in `(rule symmetric …)`, the `color-loc` in `(relation color-loc …)` | `entity.name.function` / `entity.name.type` |
| Rule-body / ⊥ primitives (head only) | `not` `and` `or` `absent` `false` `open` `forall` | `keyword.control.primitive` |
| Computed predicates (head only) | `eq` `neq` | `keyword.operator.predicate` |
| Equality head | `=` in `(= a b)`, a name since M1f S1f.5.5's swap, coloured by convention | `keyword.operator.equality` |
| `:keyword`s | `:match` `:assert` `:why` `:source` `:rule` `:using` `:goal` `:hrules` … (`:` and a run of name characters; each form takes a fixed set, and the parser refuses any other) | `entity.other.attribute-name` |
| Variables | `?rel` `?h_other` `?R*` `?_x` `?a.b` (`?` and a run of name characters) | `variable.parameter` |
| A bare `_` or `?_` | `_`, `?_`: no production takes either since M1f S1f.5.5's swap (S1f.5.1) | `invalid.illegal.underscore` (was the wildcard's `variable.language.wildcard` until the swap) |
| Strings (+ `{?var}` interpolation) | `"{?rel} is symmetric."` | `string.quoted.double` (+ `variable.other.interpolation`) |
| Numbers | none: a run of digits is a name since M1f S1f.5.5's swap (S1f.5.4's name rule), and Ein has no numbers | (`constant.numeric.integer` until the swap; no range since M1f S1f.5.3 retired the token) |
| Comments | `; line`, `#\| block \|#` | `comment.line` / `comment.block` |

Declarators / primitives / predicates are highlighted **only in head
position** (first atom after `(`), so a fact or relation that merely
*contains* one of these words is not miscoloured — e.g. `(co-located …)`,
`(relationship …)`, `(orange …)` stay plain. The removed wrapper heads
`ontology` / `facts` / `reasoning` / `rules` are **deliberately not**
keywords — they parse as ordinary fact heads now, and render as plain
atoms.

**Names follow the name rule** since M1f S1f.5.5's swap (2026-11-09,
S1f.5.4's ruling): a name or a variable is a run of ASCII name characters,
letters, digits and `` _ . - $ % ^ * + = < > ! & @ ~ / ' ` [ ] { } ``, so a
head is a reserved word only where a space or a paren follows it, as the
lexer reads it: `(rule-x …)`, `(not-a …)` and `(neq-1 …)` are facts, and
their heads stay plain. Until T1f.5.5.7 a head ended at a word boundary, `\b`, so the
`rule` of `(rule-x …)` was coloured.

## It's a highlighter, not a parser

This grammar is *regex* highlighting. It will happily colour
syntactically-invalid ein (a malformed `(query)`, a stray reserved word).
Shape correctness is the loader's job (`ein_ir::from_ir`); this only paints
the lexical surface.

## Source of truth (don't hand-edit the lists in two places)

The three closed name sets mirror the kernel's single source of truth:

- **declarators** — [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
- **primitives** — `ein_core::STRUCTURAL`, plus the verdict atom `open`
- **predicates** — `ein-infer`'s predicate registry (`ein_infer::predicates::names()`)

`ein-cli/tests/cli_semantics.rs` re-derives each list straight out
of `ein.tmLanguage.json` and asserts it equals the authoritative set, so
the grammar can't silently drift from the kernel. If you add a kernel
primitive / declarator, update the matching `begin` alternation here and
the test will keep you honest.

### Known, intentional omissions

- **`trace` events** (`step`) — engine-emitted internals, not part of the
  authoring surface; `(trace …)` itself is highlighted, its body is left
  plain. A trace holds `step` events alone since M1f S1f.5.5's swap, which
  refused `branch-open`, `contradiction` and the rest.

## Install

No marketplace publish — install it locally one of two ways.

### A. Symlink the folder (no build)

VSCode auto-loads any extension folder under `~/.vscode/extensions/`:

```sh
# from the repo root
ln -s "$PWD/utils/vscode-ein" ~/.vscode/extensions/ein-lang-0.1.0
# then: Command Palette → "Developer: Reload Window"
```

Forks use a different directory — VSCodium: `~/.vscode-oss/extensions/`,
Cursor: `~/.cursor/extensions/`, WSL/remote: the *server's* extensions
dir. A symlink stays live as you edit the grammar (reload window to
re-tokenize).

### B. Build a `.vsix` and install it

Package with the official tool ([`@vscode/vsce`](https://github.com/microsoft/vscode-vsce)):

```sh
cd utils/vscode-ein
# --baseContentUrl makes the README's relative links resolve on GitHub;
# needed only because this extension lives in a repo subfolder.
npx --yes @vscode/vsce package \
  --baseContentUrl https://github.com/agutikov/ein/blob/master/utils/vscode-ein/
# → ein-lang-0.1.0.vsix
```

Then install the `.vsix`:

```sh
code --install-extension ein-lang-0.1.0.vsix     # codium / cursor for the forks
```

…or from the GUI: Extensions view → `⋯` menu → **Install from VSIX…**.
The `.vsix` is self-contained (grammar + config + LICENSE), so it also
installs on a machine without this repo checked out.

### Verify it loaded

Open `examples/zebra2.ein`: the language indicator (bottom-right status
bar) should read **ein**, and declarator heads / `?vars` / `:keywords`
should be coloured. To inspect exact scopes, run **Developer: Inspect
Editor Tokens and Scopes** and click a token — you should see
`source.ein` and e.g. `keyword.control.declarator.ein` on a `rule` head.

For other TextMate consumers (Sublime, `bat`, …) point them at
`ein.tmLanguage.json` / `source.ein` per their own grammar-install
mechanism.
