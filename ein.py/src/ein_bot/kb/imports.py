"""Import resolution — flatten-then-load (P1.8 S1.8.A3 / A1 decision record D8).

`(import M [:as A | :symbols (S…)])` is resolved at the **form** level, before
:func:`ein_bot.kb.from_ir.load` runs: each import is replaced *in place* by the
imported module's fully-resolved form-list, with the chosen qualification
applied —

- whole-module  `(import std.macro)`        → every defined name prefixed
                                               `std.macro.` (`std.macro.forall`),
- aliased       `(import std.macro :as m)`   → prefixed `m.`            (`m.forall`),
- selective     `(import … :symbols (forall))` → the listed declarations, **flat**
                                               and unrenamed              (`forall`).

The union is then ingested by a single ``load()`` pass, so "merge" is list
concatenation and conflict detection is ``load()``'s existing duplicate-name
guards (D8). Resolution is recursive (transitive imports resolve bottom-up,
re-qualified under the outer namespace — D6) and cycle-checked. There is **no
splice-dedup**: a *qualified* diamond (A→B→D, A→C→D, all whole-module) never
collides because re-qualification gives each path its own prefix
(``B.D.x`` ≠ ``C.D.x``); a *flat* (`:symbols`) name pulled through two paths
collides into a duplicate-name error, which is the intended strict policy
(D3 — "atom re-definition → error"). Importing the same symbol twice is the
same error.

Module names are **logical**: ``std.<…>`` resolves under the packaged stdlib
root (``ein_bot/stdlib/``); any other name resolves file-relative to the
importing file. The ``.ein`` suffix is implied (D4).
"""
from __future__ import annotations

from pathlib import Path

import ein_bot
from ein_bot.ir import parse
from ein_bot.ir.types import Atom, IRNode, KwPair, SForm

from .from_ir import KBLoadError, _reserved_names

MODULE_SEP = "."
STDLIB_ALIAS = "std"
_DECLARATORS = ("rule", "hrule", "relation", "macro")


def _stdlib_root() -> Path:
    """The packaged standard-library directory (``ein_bot/stdlib/``)."""
    return Path(ein_bot.__file__).resolve().parent / "stdlib"


def _is_import(form: IRNode) -> bool:
    return (isinstance(form, SForm) and isinstance(form.head, Atom)
            and form.head.name == "import")


# ── Import spec ────────────────────────────────────────────────────


def _symbol_list(value: IRNode, module: str, loc) -> tuple[str, ...]:
    """Names inside a ``:symbols (a b …)`` list — the list lowers to an SForm
    whose head + atom args are the names."""
    if not isinstance(value, SForm):
        raise KBLoadError(f"(import {module} :symbols …) — expected a (name …) "
                          f"list at {loc}")
    names: list[str] = []
    if isinstance(value.head, Atom) and not value.head.name.startswith("@"):
        names.append(value.head.name)
    names.extend(a.name for a in value.args if isinstance(a, Atom))
    if not names:
        raise KBLoadError(f"(import {module} :symbols ()) — empty list at {loc}")
    return tuple(names)


def _import_spec(form: SForm) -> tuple[str, str | None, tuple[str, ...] | None]:
    """``(module, alias, symbols)`` for an `(import …)` form. Exactly one of
    ``alias`` / ``symbols`` is non-None, or both None (whole-module). `:as` and
    `:symbols` together is an error (A1 D3)."""
    if not form.args or not isinstance(form.args[0], Atom):
        raise KBLoadError(f"malformed (import …) — missing module name at {form.loc}")
    module = form.args[0].name
    kws = {a.key.name: a.value for a in form.args if isinstance(a, KwPair)}
    has_as, has_sym = "as" in kws, "symbols" in kws
    if has_as and has_sym:
        raise KBLoadError(
            f"(import {module}) — :as and :symbols are mutually exclusive "
            f"at {form.loc}")
    alias = None
    if has_as:
        v = kws["as"]
        if not isinstance(v, Atom):
            raise KBLoadError(
                f"(import {module} :as …) — alias must be a bare name at {form.loc}")
        alias = v.name
    symbols = _symbol_list(kws["symbols"], module, form.loc) if has_sym else None
    return module, alias, symbols


def _resolve_module_path(module: str, base_dir: Path | None, *, loc) -> Path:
    """Logical module name → file. ``std.x.y`` → ``<stdlib>/x/y.ein``; anything
    else → ``<base_dir>/<dotted→/>.ein``. Raises on a missing base / file."""
    segments = module.split(MODULE_SEP)
    if segments[0] == STDLIB_ALIAS:
        rel = segments[1:]
        if not rel:
            raise KBLoadError(f"(import {module}) — bare '{STDLIB_ALIAS}' is not "
                              f"a module at {loc}")
        root = _stdlib_root()
    else:
        if base_dir is None:
            raise KBLoadError(
                f"(import {module}) — file-relative import needs a base directory "
                f"(load from a file path) at {loc}")
        root, rel = base_dir, segments
    path = root.joinpath(*rel).with_suffix(".ein")
    if not path.is_file():
        raise KBLoadError(f"(import {module}) — module not found at {path} ({loc})")
    return path.resolve()


# ── Qualification (rename) / selection (filter) ────────────────────


def _defined_names(forms) -> set[str]:
    """Names a form-list *declares* — rule / hrule / relation / macro heads."""
    out: set[str] = set()
    for f in forms:
        if (isinstance(f, SForm) and isinstance(f.head, Atom)
                and f.head.name in _DECLARATORS
                and f.args and isinstance(f.args[0], Atom)):
            out.add(f.args[0].name)
    return out


def _rename_atoms(node: IRNode, mapping: dict[str, str]) -> IRNode:
    """Rewrite every :class:`Atom` whose name is in ``mapping`` — head, args,
    and kw-pair values alike (so references + provenance ``:rule`` refs follow
    the rename)."""
    if isinstance(node, Atom):
        return Atom(name=mapping.get(node.name, node.name), loc=node.loc)
    if isinstance(node, SForm):
        head = node.head
        if isinstance(head, Atom):
            head = Atom(name=mapping.get(head.name, head.name), loc=head.loc)
        return SForm(head=head,
                     args=tuple(_rename_atoms(a, mapping) for a in node.args),
                     loc=node.loc)
    if isinstance(node, KwPair):
        return KwPair(key=node.key, value=_rename_atoms(node.value, mapping),
                      loc=node.loc)
    return node


def _qualify(forms: list[SForm], prefix: str) -> list[SForm]:
    """Prefix every defined name (and reference to it) with ``prefix``, leaving
    reserved kernel vocabulary alone (so a module that illegally defines
    ``absent`` keeps the name and is rejected by ``load()``)."""
    mapping = {n: prefix + n
               for n in _defined_names(forms) if n not in _reserved_names()}
    if not mapping:
        return list(forms)
    return [_rename_atoms(f, mapping) for f in forms]


def _select(forms: list[SForm], symbols: tuple[str, ...],
            module: str, loc) -> list[SForm]:
    """Keep only the declarations of the listed names, **flat** (unrenamed).
    A name the module does not declare → error (no re-export of the absent)."""
    wanted = set(symbols)
    missing = wanted - _defined_names(forms)
    if missing:
        raise KBLoadError(
            f"(import {module} :symbols …) — not provided by the module: "
            f"{', '.join(sorted(missing))} at {loc}")
    return [
        f for f in forms
        if (isinstance(f, SForm) and isinstance(f.head, Atom)
            and f.head.name in _DECLARATORS
            and f.args and isinstance(f.args[0], Atom)
            and f.args[0].name in wanted)
    ]


# ── Recursive resolver ─────────────────────────────────────────────


def resolve_imports(
    forms,
    *,
    base_dir: Path | None,
    _loading: tuple[str, ...] = (),
) -> list[SForm]:
    """Return ``forms`` with every `(import …)` replaced in place by the
    imported module's resolved + qualified forms. Import-free input is returned
    unchanged (cheap pass-through — the common case). ``_loading`` is the active
    resolution stack for cycle detection."""
    out: list[SForm] = []
    for form in forms:
        if not _is_import(form):
            out.append(form)
            continue
        module, alias, symbols = _import_spec(form)
        path = _resolve_module_path(module, base_dir, loc=form.loc)
        if str(path) in _loading:
            chain = " -> ".join((*_loading, str(path)))
            raise KBLoadError(f"import cycle: {chain} (at {form.loc})")
        sub = parse(path.read_text(encoding="utf-8"), filename=str(path))
        resolved = resolve_imports(
            sub, base_dir=path.parent, _loading=(*_loading, str(path)))
        if symbols is not None:
            out.extend(_select(resolved, symbols, module, form.loc))
        else:
            prefix = (alias if alias is not None else module) + MODULE_SEP
            out.extend(_qualify(resolved, prefix))
    return out


__all__ = ["resolve_imports"]
