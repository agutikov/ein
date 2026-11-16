# Q-M1f.234 — One short name bound to two full names in one file is refused

In one file, a short name names one declaration. A short name bound to two full names, by two imports that list it
or by an import that lists it and the file's own declaration of it, is refused at load, on every command, by one
line placed where the second binding is written: `'done' is bound to two full names in the program, m.done and
n.done — one short name names one declaration in a file: list it from one module, or write the other by its full
name`. One alias prefixing two modules in one file is refused the same way. One short name bound to one full name
twice, by two imports of one module, is not an error.

**Why.** A short name that could mean either of two declarations is not correct Ein, as in any language with
namespaces: the file cannot say which one it writes. Two modules that declare one name are still imported together,
each written by its full name.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `the_refusals`, both bindings, the alias, and the name bound
twice to one declaration that loads; `ein-cli/tests/imports_by_full_name.rs`: `a_refusal_is_one_line`, one
`kb load error:` line at each command's load-error exit.
