"""ein-bot IR — S-expression intermediate representation.

Designed in `plans/m1_core_graph_reasoning/p1.1_ir_language/`.

- `grammar.lark` — Lark grammar; source of truth (M2.P2.3 GBNF lift
  derives from it).
- `types.py` — frozen-dataclass AST (`Atom`, `Var`, `Keyword`,
  `Wildcard`, `String`, `Int`, `Range`, `KwPair`, `SForm`, `Loc`).
- `ast.py` — Lark Transformer lowering parse tree → typed AST.
- `parser.py` — `parse(text) -> tuple[SForm, ...]` plus `IRParseError`.
- `dump.py` — `dump_canonical()` / `dump_compact()`; round-trips
  with `parse` modulo `Loc`.

- `to_dot.py` — IR → DOT renderer per `docs/ir.md` §6 (S1.1.4).
"""
from .dump import dump, dump_canonical, dump_compact
from .parser import IRParseError, parse, parse_tree
from .to_dot import (
    render_facts,
    render_ontology,
    render_query,
    render_reasoning,
    render_rule,
    render_trace,
    to_dot,
)
from .types import (
    Atom,
    Int,
    IRNode,
    Keyword,
    KwPair,
    Loc,
    Range,
    SForm,
    String,
    Var,
    Wildcard,
)

__all__ = [  # noqa: RUF022  — grouped by role, not alphabetical
    # parse / dump
    "parse", "parse_tree", "IRParseError",
    "dump", "dump_canonical", "dump_compact",
    # AST nodes
    "Atom", "Var", "Keyword", "Wildcard", "String", "Int", "Range",
    "KwPair", "SForm", "Loc", "IRNode",
    # DOT renderer
    "to_dot",
    "render_ontology", "render_facts", "render_reasoning",
    "render_rule", "render_query", "render_trace",
]
