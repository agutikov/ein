# `import-module-directory-module.ein/`

A directory, on purpose: the module
[`../import-module-directory.ein`](../import-module-directory.ein) imports, M1f
**W74** (`corpus/wrong_answers.toml`), filed by S1f.5.22 (T1f.5.22.2,
2026-11-10). `(import import-module-directory-module)` resolves to this path,
which exists and is no file, so the import cannot read it.

Until S1f.5.22 every command printed *`kb load error: (import
import-module-directory-module) — module not found at
…/import-module-directory-module.ein (None)`* for it: a path that exists
reported as missing, with a Rust `None` for a position. Since then an imported
module is read through the one read every command makes of the program it is
given (`ein_ir::read`), so this one is refused with that read's line, naming
the module's own path.

It holds this file alone, so that git keeps the directory. No `.ein` is
under it: the corpus walks into a directory, so a directory named `*.ein` is
no corpus file, and `ein-corpus`'s `corpus_files` lists nothing here.
