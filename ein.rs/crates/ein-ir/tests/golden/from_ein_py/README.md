# `from_ein_py/` — the other implementation's own bytes

`zebra.golden` and `zebra2.golden` are `dump_canonical(parse(f))` as **ein.py**
wrote it: 293 lines of deep nesting, long `:why` templates and non-ASCII,
checked in years before the port and carried across by `git mv` in
[S1a.10.2](../../../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
rather than regenerated.

Read by `ein-ir/tests/dump_goldens.rs`. The rule and the reasoning are the same
as for the fifteen renderings in
[`ein-render/tests/golden/from_ein_py/`](../../../../ein-render/tests/golden/from_ein_py/README.md):
**never re-bless a file here** — a golden regenerated from ein.rs says "ein.rs
reproduces itself", and what makes these worth keeping is that they say
something else.
