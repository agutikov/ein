"""Core :class:`State` — a directed multigraph of named objects + relations.

This module is a *minimal-fix* refactor of the 2021 PoC ``reasoning.py``.
Bug-fixing was limited to typos that prevented import or tests (attribute
names ``_objects`` / ``_relations``, missing ``.items()`` on dict
iteration, ``dot()`` referencing a global ``s``). Two methods whose
original implementation was incomplete or referenced undefined symbols
are explicitly gated behind :class:`NotImplementedError` and deferred to
the full rewrite (see ``docs/ideas/06-inference-rules-completeness.md``).
"""
from __future__ import annotations

from copy import deepcopy

from .patterns import Pattern, compile_predicate
from .rendering import hash_color


class State:
    """A directed multigraph: relations form a 3-level dict-of-dict-of-set.

    Two parallel indices are maintained for fast lookup either way:

    - ``relations[rel][src] = {dst, ...}``
    - ``objects[obj][rel]   = {dst, ...}``  (only outgoing edges)

    The ``objects`` index additionally records every object that has been
    seen on either end of a relation (with an empty dict if it's only ever
    been a destination).
    """

    def __init__(self) -> None:
        self.relations: dict[str, dict[str, set[str]]] = {}
        self.objects:   dict[str, dict[str, set[str]]] = {}

    # ------------------------------------------------------------------
    # Mutation
    # ------------------------------------------------------------------

    def obj(self, name: str) -> dict[str, set[str]]:
        """Add an isolated object (idempotent). Returns its outgoing-rels dict."""
        return self.objects.setdefault(name, {})

    def rel(self, src: str, rel: str, dst: str
            ) -> tuple[dict[str, set[str]], dict[str, set[str]]]:
        """Add a directed relation ``src --rel--> dst``.

        Returns ``(src_obj_dict, rel_dict)``.
        """
        r = self.relations.setdefault(rel, {})
        r.setdefault(src, set()).add(dst)
        s = self.objects.setdefault(src, {})
        s.setdefault(rel, set()).add(dst)
        self.objects.setdefault(dst, {})
        return s, r

    # ------------------------------------------------------------------
    # Inspection
    # ------------------------------------------------------------------

    def is_consistent(self) -> bool:
        """Debug invariant check across the two indices. TODO."""
        return True

    def dump(self) -> str:
        """Serialise back into the same textual format ``parse`` consumes."""
        lines: list[str] = list(self.objects)
        for rel_name, rel in self.relations.items():
            for src_name, dst_set in rel.items():
                for dst_name in dst_set:
                    lines.append(f"{src_name} {rel_name} {dst_name}")
        return "\n".join(lines) + "\n"

    def dot(self, colorfull: bool = True) -> str:
        """Render as a Graphviz ``digraph``. Edges are coloured per relation."""
        out = "digraph G {\n"
        for obj_name in self.objects:
            out += f"  {obj_name};\n"

        colors = {rel_name: hash_color(rel_name) for rel_name in self.relations}

        for rel_name, rel in self.relations.items():
            for src_name, dst_set in rel.items():
                for dst_name in dst_set:
                    if colorfull:
                        color = f'"{colors[rel_name]}"'
                        label = f"<<font color={color}>{rel_name}</font>>"
                        out += (f"  {src_name} -> {dst_name} "
                                f"[label={label} color={color}];\n")
                    else:
                        out += (f"  {src_name} -> {dst_name} "
                                f'[label="{rel_name}"];\n')
        out += "}\n"
        return out

    def show(self) -> None:
        """Interactive visualisation. TODO: networkx."""
        return None

    # ------------------------------------------------------------------
    # Queries
    # ------------------------------------------------------------------

    def select_obj(self, obj_pattern: Pattern, exclude: bool = False) -> State:
        """Select a sub-state by object name. TODO."""
        raise NotImplementedError(
            "select_obj: deferred to the full rewrite "
            "(see docs/ideas/02-graph-as-formal-substrate.md)."
        )

    def select_rel(self, rel_pattern: Pattern, exclude: bool = False,
                   preserve_objs: bool = False) -> State:
        """Return a new ``State`` keeping only relations matching *rel_pattern*."""
        comp = compile_predicate(rel_pattern)
        out = State()
        for rel_name, rel in self.relations.items():
            if comp(rel_name) != exclude:
                for src_name, dst_set in rel.items():
                    for dst_name in dst_set:
                        out.rel(src_name, rel_name, dst_name)
        if preserve_objs:
            for obj_name in self.objects:
                out.obj(obj_name)
        return out

    def ends(self, rel_selector: Pattern) -> list[str]:
        """Object names with *no outgoing* relations under the selector."""
        sub = self.select_rel(rel_selector, preserve_objs=True)
        return [name for name, rels in sub.objects.items() if not rels]

    def not_ends(self, rel_selector: Pattern) -> list[str]:
        """Object names with at least one outgoing relation under the selector."""
        sub = self.select_rel(rel_selector, preserve_objs=True)
        return [name for name, rels in sub.objects.items() if rels]

    def obj_types(self, obj_name: str, rel_pattern: Pattern) -> list[str]:
        """Return the *end* objects reachable from *obj_name* via *rel_pattern*.

        The 2021 PoC used this to enumerate the *types* of an object under
        a typing relation (e.g. ``Houses → House`` via ``is``).
        """
        comp = compile_predicate(rel_pattern)
        ends = set(self.ends(rel_pattern))
        obj = self.objects.get(obj_name)
        if obj is None:
            return []
        for rel_name, rel in obj.items():
            if comp(rel_name):
                return list(ends.intersection(rel))
        return []

    def rel_types(self, type_rel_name: str) -> dict[str, set[tuple[str, str]]]:
        """For each non-typing relation, list (src_type, dst_type) pairs.

        Original PoC code referenced an undefined ``obj_type`` (singular);
        the intended behaviour is ambiguous, so completion is deferred to
        the full rewrite.
        """
        raise NotImplementedError(
            "rel_types: original PoC referenced an undefined singular obj_type; "
            "deferred to the full rewrite."
        )

    # ------------------------------------------------------------------
    # Constraints
    # ------------------------------------------------------------------

    def verify_single_rel_constraint(self, rel_pattern: Pattern = None):
        """Single-type / single-attribute constraints. TODO (PoC §Constraints).

        The 2021 source had an unfinished comprehension on this method;
        completion is deferred to the full rewrite — see
        ``docs/ideas/06-inference-rules-completeness.md``.
        """
        raise NotImplementedError(
            "verify_single_rel_constraint: completion deferred to the full rewrite "
            "(see docs/ideas/06-inference-rules-completeness.md)."
        )

    # ------------------------------------------------------------------
    # Misc
    # ------------------------------------------------------------------

    def copy(self) -> State:
        """Deep copy."""
        return deepcopy(self)
