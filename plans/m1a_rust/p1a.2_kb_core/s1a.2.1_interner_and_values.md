# S1a.2.1 — Interner, `Value`, `FactId`, the fact store

**Phase:** P1a.2 (KB core)
**Estimate:** 3 days
**Depends on:** [P1a.1](../p1a.1_ir_frontend/README.md)
**Implements design:** [design/03](../design/03_data_model.md) §§2–4

## Context

Turn every atom, literal and proposition into a `u32`. This is the single
change that makes the matcher cheap
([design/05](../design/05_matcher.md)), the negation index a bitset, and
`_fact_by_id` O(1) instead of an O(deg) scan.

The subtle part is not the encoding, it is the **separation of interning
from belief**: `intern(rel, args) -> FactId` assigns a number; whether the
proposition holds is a per-KB bit. That separation is what lets a fork
intern freely without leaking into its parent, and it is what
[S1a.2.2](s1a.2.2_store_and_indexes.md)'s O(1) fork rests on.

## Acceptance

- `Value` is 4 bytes; `Row` is 12; a fact's args are contiguous.
- Interning is injective and stable: re-interning the same
  `(rel, args)` in a fork returns the parent's `FactId`.
- The int pool canonicalises exactly as `Int(value=int(tok))` does
  (`007` ≡ `7`, `-0` ≡ `0`) and handles arbitrary-precision literals
  without overflow.
- `Atom("foo")`, `String("foo")` and a `Var`→`"?x"` arg collapse to the
  same `Value` shape the Python `_atomic_value` flattening produces.
- The rank table gives lexicographic order matching Python's
  `sorted(strings)` on a generated Unicode corpus.
- Memory: 381 interned facts + args ≤ 10 KB.

## Tasks

### Task T1a.2.1.1 — Interner

Text arena + span table + `FxHashMap` lookup. `Symbol(u32)`. Lazily-built
`rank: Vec<u32>` (Symbol → lexicographic position), invalidated on
growth. Document that ids are assignment-ordered and **must never be used
as an observable sort key** ([design/08](../design/08_parallelism.md) §1).

### Task T1a.2.1.2 — `Value`

2-bit tag + 30-bit payload: `Sym` / `Int` / `Fact`. Two comparators: a
raw `Ord` for identity containers and `cmp_semantic(&Interner)` for
observable sorts. A lint or a newtype wrapper that makes using the wrong
one hard.

### Task T1a.2.1.3 — Int pool

Canonical decimal text + `Option<i64>` fast value. Equality is pool-id
equality. Rendering (`str(v)` in provenance bindings, `_compact` in the
dumper) goes through the canonical text.

### Task T1a.2.1.4 — Fact store

`rows: Vec<Row>`, `args: Vec<Value>`, `lookup: FxHashMap<RowKey, FactId>`
with hashing/equality over `(rel, args)`. `intern`, `get(FactId) ->
(Symbol, &[Value])`, and a `probe` that computes the key without
materialising, so hypgen can reject a candidate before creating anything
([design/07](../design/07_search_layer.md) §2).

### Task T1a.2.1.5 — Nested facts

A `(not X)` fact stores `X` as a `Value::fact(FactId)`. Verify the
recursive identity ein.py has (`Fact.__eq__` cascading into nested
facts) is preserved, and that a nested fact interned *only* as an arg
(ein.py's "unregistered" nested `Fact`) is distinguishable from a
believed one — belief is the presence bit, not the id.

## Notes

- `Value` at 4 bytes assumes ≤ 2³⁰ symbols / ints / facts. Add a
  saturating check that turns overflow into a clean error rather than a
  silent wrap; a puzzle that hits it is a research finding, not a crash.
- Keep the interner and fact store `Send + Sync`-ready from the start
  (sharded locks behind a trait), even though P1a.2–6 are
  single-threaded — retrofitting them under
  [P1a.7](../p1a.7_parallelism/README.md) would touch every call site.
