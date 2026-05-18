"""Typed-AST + round-trip tests (S1.1.2).

Covers:
- Each kernel form lowers to the expected `SForm` shape.
- Terminals lift to the right node types (Atom / Var / Keyword /
  Wildcard / String / Int / Range).
- `parse(dump_canonical(parse(text))) == parse(text)` modulo `Loc`
  for every example.
- A handful of broken fixtures raise `IRParseError` with
  `file:line:col` in the message.
"""
from pathlib import Path

import pytest

from ein_bot.ir import (
    Atom, IRParseError, Int, Keyword, KwPair, Range, SForm, String, Var,
    Wildcard, dump_canonical, dump_compact, parse,
)


REPO = Path(__file__).resolve().parent.parent
ZEBRA = REPO / "examples" / "zebra.ein"


# ═══════════ Lowering: terminals ═══════════

def test_atom():
    (form,) = parse("(facts (lives-in Norwegian House_1))")
    fact = form.args[0]
    assert isinstance(fact, SForm)
    assert fact.head == Atom("lives-in")
    assert fact.args == (Atom("Norwegian"), Atom("House_1"))


def test_var_and_wildcard():
    (form,) = parse("(rules (rule any () :match (_ ?a ?b) :assert ?a))")
    rule = form.args[0]
    match = next(kp.value for kp in rule.args if isinstance(kp, KwPair)
                 and kp.key.name == "match")
    assert isinstance(match, SForm)
    assert isinstance(match.head, Wildcard)
    assert match.args == (Var("a"), Var("b"))


def test_keyword_kwpair():
    (form,) = parse("(query :mode solve :goal (lives-in _ House_1))")
    args = form.args
    assert isinstance(args[0], KwPair)
    assert args[0].key == Keyword("mode")
    assert args[0].value == Atom("solve")


def test_string_escapes():
    (form,) = parse(r'(facts (msg :source "line1\nline2 \"quoted\""))')
    fact = form.args[0]
    src = next(kp.value for kp in fact.args if isinstance(kp, KwPair))
    assert src == String('line1\nline2 "quoted"')


def test_int_and_range():
    (form,) = parse(
        "(ontology (relation r (A B) :cardinality 1..* :priority 5))"
    )
    rel = form.args[0]
    cardinality = next(kp.value for kp in rel.args if isinstance(kp, KwPair)
                       and kp.key.name == "cardinality")
    priority = next(kp.value for kp in rel.args if isinstance(kp, KwPair)
                    and kp.key.name == "priority")
    assert cardinality == Range(1, None)
    assert priority == Int(5)


# ═══════════ Lowering: top-level forms ═══════════

def test_top_level_heads():
    forms = parse("""
    (ontology (type Person))
    (facts (lives-in a b))
    (rules (rule x () :match a :assert b))
    (query :mode solve :goal X)
    (trace)
    """)
    assert tuple(f.head.name for f in forms) == (
        "ontology", "facts", "rules", "query", "trace",
    )


def test_eq_fact():
    (form,) = parse("(facts (= (color House_1) Red))")
    eq = form.args[0]
    assert eq.head == Atom("=")
    assert isinstance(eq.args[0], SForm) and eq.args[0].head == Atom("color")
    assert eq.args[1] == Atom("Red")


def test_eq_inside_pattern():
    (form,) = parse(
        "(rules (rule eq-elim () :match (= ?a ?b) :assert ?a))"
    )
    rule = form.args[0]
    match = next(kp.value for kp in rule.args if isinstance(kp, KwPair)
                 and kp.key.name == "match")
    assert match.head == Atom("=")
    assert match.args == (Var("a"), Var("b"))


def test_rule_params():
    (form,) = parse(
        "(rules (rule symmetric (?rel) :match ?rel :assert ?rel))"
    )
    rule = form.args[0]
    # rule_decl args: [name, params, *kw_pairs]
    assert rule.args[0] == Atom("symmetric")
    params = rule.args[1]
    assert isinstance(params, SForm) and params.head == Atom("@params")
    assert params.args == (Var("rel"),)


def test_empty_rule_params():
    (form,) = parse("(rules (rule x () :match a :assert b))")
    rule = form.args[0]
    params = rule.args[1]
    assert params.head == Atom("@params") and params.args == ()


def test_relation_sig():
    (form,) = parse(
        "(ontology (relation lives-in (Person House) :cardinality 1..1))"
    )
    rel = form.args[0]
    sig = rel.args[1]
    assert sig.head == Atom("@sig")
    assert sig.args == (Atom("Person"), Atom("House"))


# ═══════════ Loc tracking ═══════════

def test_loc_recorded():
    forms = parse("(ontology\n  (type Person))", filename="x.ein")
    type_decl = forms[0].args[0]
    name = type_decl.args[0]
    assert name == Atom("Person")
    assert name.loc is not None
    assert name.loc.file == "x.ein"
    assert name.loc.line == 2  # second line


def test_loc_excluded_from_equality():
    """Two ASTs from differently-positioned sources still compare equal."""
    a = parse("(facts (lives-in a b))")
    b = parse("\n\n  (facts (lives-in a b))")
    assert a == b


# ═══════════ Round-trip ═══════════

ROUNDTRIP_CASES = [
    "(ontology)",
    "(ontology (type Person) (type Engineer Person))",
    "(facts (= (color House_1) Red))",
    "(facts (lives-in Norwegian House_1 :source \"condition (10)\"))",
    "(facts (symmetric co-located) (implies right-of next-to))",
    "(rules (rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a)"
    " :why \"sym\"))",
    "(rules (rule t () :match (and (?r ?a ?b) (?r ?b ?c)"
    " :where (transitive ?r)) :assert (?r ?a ?c) :why \"tr\"))",
    "(query :mode solve :goal (drinks Water ?h))",
    "(trace (step s1 :rule from-condition :using (c10)"
    " :derives (lives-in Norwegian House_1)))",
    "(trace (branch-open s3 :on (lives-in Englishman ?h)"
    " :choices (a b c)))",
]


@pytest.mark.parametrize("src", ROUNDTRIP_CASES)
def test_roundtrip(src: str) -> None:
    ast1 = parse(src)
    text = dump_canonical(ast1)
    ast2 = parse(text)
    assert ast1 == ast2, f"roundtrip diverged:\n--src--\n{src}\n--dump--\n{text}"


def test_roundtrip_zebra():
    """Full Zebra puzzle survives dump∘parse without loss."""
    src = ZEBRA.read_text(encoding="utf-8")
    ast1 = parse(src, filename=str(ZEBRA))
    text = dump_canonical(ast1)
    ast2 = parse(text)
    assert ast1 == ast2


def test_dump_stable():
    """dump∘parse∘dump∘parse == dump∘parse — printer is a fixed point."""
    src = ZEBRA.read_text(encoding="utf-8")
    t1 = dump_canonical(parse(src))
    t2 = dump_canonical(parse(t1))
    assert t1 == t2


def test_dump_compact_one_line():
    (form,) = parse("(facts (lives-in Norwegian House_1 :source \"x\"))")
    compact = dump_compact(form)
    assert "\n" not in compact
    assert compact.startswith("(facts ") and compact.endswith(")")


# ═══════════ Error messages ═══════════

@pytest.mark.parametrize("src,marker", [
    ("(rules (rule x () :match a", "1:"),       # unclosed paren
    ("Norwegian", "1:"),                         # top-level bare atom
    ("(query :mode :solve)", "1:"),              # keyword as value
    ("(ontology (instance N T))", "1:"),         # instance not in ontology
    ("(step s1)", "1:"),                         # step at top level
])
def test_error_has_location(src: str, marker: str) -> None:
    with pytest.raises(IRParseError) as ei:
        parse(src, filename="<demo>")
    msg = str(ei.value)
    assert "<demo>:" in msg
    assert marker in msg
