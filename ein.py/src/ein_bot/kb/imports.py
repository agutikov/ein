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
    """Keep the listed names **plus their dependency closure**, flat (unrenamed).

    Auto-closure (S1.8a.f20): a listed declaration drags in every *other*
    declaration of this module it references by name — so importing an entry
    rule (`bijective-setup`) pulls the machinery it asserts/matches
    (`domain-elimination`, …) without the importer enumerating all of it. Names
    referenced but not declared here — cross-module deps (`total` from
    std.algebra), kernel primitives (`relation`, `forall`) — are left for the
    importer's *other* imports to provide, so a module need not be self-contained.

    A listed name the module does not declare → error (no re-export of the
    absent); the closure only follows names this module actually declares.
    """
    decls: dict[str, SForm] = {
        nm: f for f in forms if (nm := _decl_name(f)) is not None
    }
    wanted = set(symbols)
    missing = wanted - set(decls)
    if missing:
        raise KBLoadError(
            f"(import {module} :symbols …) — not provided by the module: "
            f"{', '.join(sorted(missing))} at {loc}")
    keep: set[str] = set()
    work = list(wanted)
    while work:
        n = work.pop()
        if n in keep or n not in decls:
            continue
        keep.add(n)
        work.extend(_referenced_names(decls[n]))
    return [f for f in forms if _decl_name(f) in keep]


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


# ── Resolved + minimized dump (A1 D9) ──────────────────────────────


def _decl_name(form: IRNode) -> str | None:
    """The name a single declarator form binds (rule/hrule/relation/macro), or
    None for a fact / non-declarator."""
    if (isinstance(form, SForm) and isinstance(form.head, Atom)
            and form.head.name in _DECLARATORS
            and form.args and isinstance(form.args[0], Atom)):
        return form.args[0].name
    return None


def _referenced_names(node: IRNode, out: set[str] | None = None) -> set[str]:
    """Every :class:`Atom` name reachable from ``node`` — heads (so a macro
    invocation `(forall …)` counts as referencing `forall`), args, and kw-pair
    values. Object names + reserved words are collected too; the caller only
    follows those that name an imported declaration."""
    if out is None:
        out = set()
    if isinstance(node, Atom):
        out.add(node.name)
    elif isinstance(node, SForm):
        if isinstance(node.head, Atom):
            out.add(node.head.name)
        for a in node.args:
            _referenced_names(a, out)
    elif isinstance(node, KwPair):
        _referenced_names(node.value, out)
    return out


def _resolve_tagged(forms, base_dir: Path | None) -> list[tuple[SForm, bool]]:
    """Resolve imports, tagging each result form ``is_imported``: the puzzle's
    own (non-import) forms are ``False``; everything an import brings in
    (transitively) is ``True``."""
    out: list[tuple[SForm, bool]] = []
    for form in forms:
        if _is_import(form):
            for f in resolve_imports([form], base_dir=base_dir):
                out.append((f, True))
        else:
            out.append((form, False))
    return out


def _is_rule_form(form: IRNode) -> bool:
    return (isinstance(form, SForm) and isinstance(form.head, Atom)
            and form.head.name in ("rule", "hrule"))


def _kw_value(form: SForm, key: str) -> IRNode | None:
    """The value of a rule/hrule keyword arg (e.g. `:match` / `:assert`)."""
    for a in form.args:
        if isinstance(a, KwPair) and a.key.name == key:
            return a.value
    return None


def _sform_head_names(node: IRNode, out: set[str] | None = None) -> set[str]:
    """The Atom head-name of every SForm reachable from ``node`` — relation
    heads and logical connectives (`and`/`not`/…) alike. Variable heads
    (`(?R ?a ?b)`) contribute nothing. Callers intersect against the sets they
    care about, so the stray connective names are harmless."""
    if out is None:
        out = set()
    if isinstance(node, SForm):
        if isinstance(node.head, Atom):
            out.add(node.head.name)
        for a in node.args:
            _sform_head_names(a, out)
    return out


def resolve_and_minimize(forms, *, base_dir: Path | None = None) -> list[SForm]:
    """Resolve every `(import …)` inline, then **tree-shake**: drop any imported
    *declaration* (rule/hrule/relation/macro) nothing references (A1 D9).

    Reachability seeds from the puzzle's own forms (and any imported facts,
    kept conservatively), then closes over two coupled relations:

    - **name reference** — a kept form mentioning a declaration's name keeps it
      (and, transitively, what its body names); this reaches a parameterised
      rule once something asserts its activator fact, since the activator's head
      *is* the rule name;
    - **activation** — an imported rule whose `:match` references a *live*
      relation is kept too, even when its own name is referenced nowhere (the
      None-activator glue rules — `bijective-setup`, the `*-setup` fan-outs — are
      fired by their match pattern, not by name). A relation is live if it heads
      a kept fact or is asserted by a kept rule; a freshly-kept rule's
      `:assert` heads join the live set, so activation cascades.

    Without the activation pass, `--resolve` would silently drop an entire
    activator-driven rule library (e.g. `std.bijection`) and leave a standalone
    file that no longer solves. The result is import-free; `dump_canonical` of it
    is a self-contained `.ein` equivalent to the original. Import-free input
    passes through unchanged.
    """
    tagged = _resolve_tagged(list(forms), base_dir)
    imported_decls: dict[str, SForm] = {
        nm: f for f, imp in tagged
        if imp and (nm := _decl_name(f)) is not None
    }
    # match / assert relation heads of each imported rule (for activation).
    imp_match_heads: dict[str, set[str]] = {}
    imp_assert_heads: dict[str, set[str]] = {}
    for nm, f in imported_decls.items():
        if _is_rule_form(f):
            m, a = _kw_value(f, "match"), _kw_value(f, "assert")
            imp_match_heads[nm] = _sform_head_names(m) if m is not None else set()
            imp_assert_heads[nm] = _sform_head_names(a) if a is not None else set()

    # live relations: heads of every kept fact (puzzle + imported facts) and the
    # asserts of the puzzle's own (always-kept) rules — the relations that can
    # exist in the saturated KB and so trigger activator-driven imported rules.
    live: set[str] = set()
    for f, imp in tagged:
        if _decl_name(f) is None:                       # a fact / non-declarator
            _sform_head_names(f, live)
        elif not imp and _is_rule_form(f):              # puzzle's own rule
            a = _kw_value(f, "assert")
            if a is not None:
                _sform_head_names(a, live)

    reachable: set[str] = set()
    work: list[str] = []
    for f, imp in tagged:                    # roots: puzzle forms + imported facts
        if not imp or _decl_name(f) is None:
            work.extend(_referenced_names(f))

    changed = True
    while changed:
        changed = False
        while work:                          # name-reference closure
            n = work.pop()
            if n in reachable or n not in imported_decls:
                continue
            reachable.add(n)
            changed = True
            work.extend(_referenced_names(imported_decls[n]))
            live |= imp_assert_heads.get(n, set())   # a kept rule's asserts go live
        for nm, heads in imp_match_heads.items():     # activation closure
            if nm not in reachable and (heads & live):
                work.append(nm)
                changed = True
    return [
        f for f, imp in tagged
        if not imp or _decl_name(f) is None or _decl_name(f) in reachable
    ]


__all__ = ["resolve_and_minimize", "resolve_imports"]
