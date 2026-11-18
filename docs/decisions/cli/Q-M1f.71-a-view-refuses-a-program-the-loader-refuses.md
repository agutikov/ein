# Q-M1f.71 — A view refuses a program the loader refuses

`ein render program`, `render rules` and `render rule` read the program through the loader first, refuse what it
refuses with the same line and exit 1, and only then draw the program as written, its imports unresolved; `render
constraints` and `render kb` draw a knowledge base the program loads to, and refuse what the loader refuses by
loading it. Each takes `--no-require-why`, as every other command that loads a program does, so a program with no
`:why` draws where it loads.

**Why.** No two commands may disagree, and a view that exits 0 on a program every other command refuses disagrees
about whether it is a program at all.

**Held by.** `ein-cli/tests/render_doors.rs`: `every_view_refuses_what_the_loader_refuses`;
`ein-cli/tests/config_reference.rs`: `require_why_decides_loading_only`, the option on every command that loads.
