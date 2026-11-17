# `import-module-directory-module.ein/`

A directory, on purpose: the module [`../import-module-directory.ein`](../import-module-directory.ein) imports,
the witness of **W74** (`corpus/wrong_answers.toml`). `(import import-module-directory-module)` resolves to this
path, which exists and is no file, so the import cannot read it.

An imported module is read through the one read every command makes of the program it is given (`ein_ir::read`),
so this one is refused with that read's line, naming the module's own path, and never reported as missing.

It holds this file alone, so that git keeps the directory. No `.ein` is under it: the corpus walks into a
directory, so a directory named `*.ein` is no corpus file, and `ein-corpus`'s `corpus_files` lists nothing here.
