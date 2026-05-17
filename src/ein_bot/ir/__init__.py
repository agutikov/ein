"""ein-bot IR — S-expression intermediate representation.

Designed in `plans/m1_core_graph_reasoning/p1.1_ir_language/`. The
kernel grammar lives in `grammar.lark`; `parser.parse()` produces a
raw Lark tree for now (S1.1.1). A typed `IRNode` AST and `dump()`
round-trip land in S1.1.2; a DOT renderer + reverse parser land in
S1.1.2 / P1.2 per Q21.
"""
from .parser import parse

__all__ = ["parse"]
