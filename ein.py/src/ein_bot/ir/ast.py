"""Lark Tree → typed IRNode lowering — S1.1.2 T1.1.2.2.

A `lark.Transformer` walks the parse tree produced by `grammar.lark`
and emits a tuple of typed `SForm`s. Source positions are captured
from Lark tokens (`propagate_positions=True` on the parser).

Every top-level form (`ontology`, `facts`, `rules`, `query`, `trace`)
becomes a single `SForm(head=Atom("ontology"), …)` etc. — the rule
heads are stripped from the grammar's quoted-string keywords and
re-introduced as Atom heads so the AST is uniform.
"""
from __future__ import annotations

from lark import Token, Transformer

from .types import (
    Atom, IRNode, Int, Keyword, KwPair, Loc, Range, SForm, String, Var,
    Wildcard,
)


def _loc(tok: Token, filename: str | None) -> Loc:
    return Loc(file=filename or "<string>", line=tok.line, col=tok.column)


class _ToAST(Transformer):
    """Lark Transformer mapping each grammar rule to a typed node.

    Methods named after grammar rules receive that rule's children.
    Terminal-handling methods (uppercase names) receive a Token.
    """

    def __init__(self, filename: str | None = None) -> None:
        super().__init__()
        self._file = filename

    # ── Terminals ──────────────────────────────────────────────
    def SYMBOL(self, tok: Token) -> Atom:
        return Atom(name=str(tok), loc=_loc(tok, self._file))

    def VAR(self, tok: Token) -> Var:
        return Var(name=str(tok)[1:], loc=_loc(tok, self._file))

    def KEYWORD(self, tok: Token) -> Keyword:
        return Keyword(name=str(tok)[1:], loc=_loc(tok, self._file))

    def WILDCARD(self, tok: Token) -> Wildcard:
        return Wildcard(loc=_loc(tok, self._file))

    def EQ(self, tok: Token) -> Atom:
        # `=` is a named terminal so it survives Lark's anonymous-token
        # filtering and reaches us both as a list head (pattern interior)
        # and as the eq_fact marker.
        return Atom(name="=", loc=_loc(tok, self._file))

    def INT(self, tok: Token) -> Int:
        return Int(value=int(tok), loc=_loc(tok, self._file))

    def RANGE(self, tok: Token) -> Range:
        text = str(tok)
        low_s, high_s = text.split("..", 1)
        high: int | None = None if high_s == "*" else int(high_s)
        return Range(low=int(low_s), high=high, loc=_loc(tok, self._file))

    def STRING(self, tok: Token) -> String:
        # Strip surrounding quotes and unescape.
        body = str(tok)[1:-1]
        # Minimal unescape: \" \\ \n \t \r.
        out: list[str] = []
        i = 0
        while i < len(body):
            ch = body[i]
            if ch == "\\" and i + 1 < len(body):
                nxt = body[i + 1]
                out.append({"n": "\n", "t": "\t", "r": "\r"}.get(nxt, nxt))
                i += 2
            else:
                out.append(ch)
                i += 1
        return String(value="".join(out), loc=_loc(tok, self._file))

    # ── kw_pair ────────────────────────────────────────────────
    def kw_pair(self, items: list) -> KwPair:
        key, value = items
        assert isinstance(key, Keyword)
        return KwPair(key=key, value=value, loc=key.loc)

    # ── generic_list ───────────────────────────────────────────
    def generic_list(self, items: list) -> SForm:
        # `()` empty case — synthetic empty-named head; dump emits "()".
        if not items:
            return SForm(head=Atom(name="@empty"), args=())
        head, *rest = items
        assert isinstance(head, (Atom, Var, Wildcard))
        return SForm(head=head, args=tuple(rest), loc=head.loc)

    # ── Top-level forms ────────────────────────────────────────
    def _topform(self, head_name: str, items: list) -> SForm:
        return SForm(head=Atom(name=head_name), args=tuple(items))

    def ontology_form(self, items: list) -> SForm:
        return self._topform("ontology", items)

    def facts_form(self, items: list) -> SForm:
        return self._topform("facts", items)

    def reasoning_form(self, items: list) -> SForm:
        return self._topform("reasoning", items)

    def rules_form(self, items: list) -> SForm:
        return self._topform("rules", items)

    def query_form(self, items: list) -> SForm:
        return self._topform("query", items)

    def trace_form(self, items: list) -> SForm:
        return self._topform("trace", items)

    def config_form(self, items: list) -> SForm:
        return self._topform("config", items)

    # ── Ontology declarations ──────────────────────────────────
    def type_decl(self, items: list) -> SForm:
        # Grammar: `"(" "type" SYMBOL [SYMBOL] ")"`. The optional parent
        # SYMBOL is `None` when omitted; drop it so the dump round-trip
        # emits `(type Person)` vs `(type Engineer Person)` correctly.
        kept = tuple(x for x in items if x is not None)
        return SForm(head=Atom(name="type"), args=kept)

    def relation_decl(self, items: list) -> SForm:
        # Grammar: `"(" "relation" SYMBOL SYMBOL+ kw_pair* ")"`.
        # Args are flat (post-R10): (name, T1, T2, …, *kws). No inner
        # @sig SForm; kernel docs `01-ein-graph/03_ein_model.md` §7.2.
        return SForm(head=Atom(name="relation"), args=tuple(items))

    def apriori_decl(self, items: list) -> SForm:
        # Grammar: `"(" "a-priori" SYMBOL SYMBOL+ kw_pair* ")"`. Same
        # flat-args shape as relation_decl (R10).
        return SForm(head=Atom(name="a-priori"), args=tuple(items))

    # ── Facts ──────────────────────────────────────────────────
    def eq_fact(self, items: list) -> SForm:
        # items[0] is the EQ token (now a named terminal); drop it.
        eq_head, *rest = items
        assert isinstance(eq_head, Atom) and eq_head.name == "="
        return SForm(head=eq_head, args=tuple(rest))

    def generic_fact(self, items: list) -> SForm:
        head, *rest = items
        assert isinstance(head, Atom)
        return SForm(head=head, args=tuple(rest))

    # ── Kernel meta-primitives (shape-pinned) ─────────────────
    # Each reserved-word form gets a synthetic Atom head matching the
    # literal. The grammar guarantees arity / arg shape; the validator
    # (deferred) will check arg types and contextual constraints.
    def instance_form(self, items: list) -> SForm:
        return SForm(head=Atom(name="instance"), args=tuple(items))

    def not_form(self, items: list) -> SForm:
        return SForm(head=Atom(name="not"), args=tuple(items))

    def neq_form(self, items: list) -> SForm:
        return SForm(head=Atom(name="neq"), args=tuple(items))

    def and_form(self, items: list) -> SForm:
        return SForm(head=Atom(name="and"), args=tuple(items))

    def or_form(self, items: list) -> SForm:
        return SForm(head=Atom(name="or"), args=tuple(items))

    # ── Rules ──────────────────────────────────────────────────
    def rule_decl(self, items: list) -> SForm:
        name, params, *kws = items
        return SForm(head=Atom(name="rule"), args=(name, params, *kws))

    def hrule_decl(self, items: list) -> SForm:
        # S1.5.6b — a hypothesis rule: same shape as `rule`, but
        # the loader routes it to kb.hrules instead of kb.rules.
        name, params, *kws = items
        return SForm(head=Atom(name="hrule"), args=(name, params, *kws))

    def rule_params(self, items: list) -> SForm:
        return SForm(head=Atom(name="@params"), args=tuple(items))

    # ── Trace events ───────────────────────────────────────────
    def step_decl(self, items: list) -> SForm:
        return SForm(head=Atom(name="step"), args=tuple(items))

    def branch_open(self, items: list) -> SForm:
        return SForm(head=Atom(name="branch-open"), args=tuple(items))

    def branch_close(self, items: list) -> SForm:
        return SForm(head=Atom(name="branch-close"), args=tuple(items))

    def branch_ref(self, items: list) -> SForm:
        return SForm(head=Atom(name="branch-ref"), args=tuple(items))

    def contradiction_decl(self, items: list) -> SForm:
        return SForm(head=Atom(name="contradiction"), args=tuple(items))

    def symmetry_decl(self, items: list) -> SForm:
        return SForm(head=Atom(name="symmetry-class"), args=tuple(items))

    # ── Top ────────────────────────────────────────────────────
    def start(self, items: list) -> tuple[SForm, ...]:
        return tuple(items)


def to_ast(tree, filename: str | None = None) -> tuple[SForm, ...]:
    """Lower a Lark parse tree to a tuple of typed top-level `SForm`s."""
    result = _ToAST(filename=filename).transform(tree)
    assert isinstance(result, tuple)
    return result


__all__ = ["to_ast"]
