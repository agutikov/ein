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
ZEBRA2 = REPO / "examples" / "zebra2.ein"
EXAMPLE_FILES = [ZEBRA, ZEBRA2]


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
        "(ontology (relation r A B :cardinality 1..* :priority 5))"
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


def test_relation_sig_flat():
    """Post-R10: relation args are flat — no @sig SForm wrapper."""
    (form,) = parse(
        "(ontology (relation lives-in Person House :cardinality 1..1))"
    )
    rel = form.args[0]
    # rel.args == (name, T1, T2, KwPair(:cardinality 1..1))
    assert rel.args[0] == Atom("lives-in")
    assert rel.args[1] == Atom("Person")
    assert rel.args[2] == Atom("House")
    assert isinstance(rel.args[3], KwPair)
    assert rel.args[3].key.name == "cardinality"


def test_relation_wrapped_form_rejected():
    """Post-R10: the wrapped form `(relation R (T1 T2))` is a parse error."""
    with pytest.raises(IRParseError):
        parse("(ontology (relation lives-in (Person House)))")


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
    # ── Original baseline cases ──
    "(ontology)",
    "(ontology (type Person) (type Engineer Person))",
    "(facts (= (color House_1) Red))",
    "(facts (lives-in Norwegian House_1 :source \"condition (10)\"))",
    "(ontology (symmetric co-located) (implies right-of next-to))",
    "(rules (rule symmetric (?rel) :match (?rel ?a ?b) :assert (?rel ?b ?a)"
    " :why \"sym\"))",
    "(rules (rule t () :match (and (?r ?a ?b) (?r ?b ?c)"
    " :where (transitive ?r)) :assert (?r ?a ?c) :why \"tr\"))",
    "(query :mode solve :goal (drinks Water ?h))",
    "(trace (step s1 :rule from-condition :using (c10)"
    " :derives (lives-in Norwegian House_1)))",
    "(trace (branch-open s3 :on (lives-in Englishman ?h)"
    " :choices (a b c)))",
    # ── Edge cases (S1.1.3 T1.1.3.3, 2026-05-18) ──
    # String escapes
    '(facts (lives-in a b :source "tab\\there"))',
    '(facts (lives-in a b :source "newline\\nhere"))',
    '(facts (lives-in a b :source "quote\\"inside"))',
    '(facts (lives-in a b :source "back\\\\slash"))',
    '(facts (lives-in a b :source ""))',           # empty string
    '(facts (lives-in a b :source "unicode é·»→"))',
    # Range edge cases
    "(ontology (relation r A B :cardinality 0..0))",
    "(ontology (relation r A B :cardinality 0..*))",
    "(ontology (relation r A B :cardinality 9999..*))",
    # Deep nesting
    "(rules (rule deep () :match (and (and (and (and (rel ?a ?b)))))"
    " :assert ?a :why \"d\"))",
    # Variable / wildcard heads
    "(rules (rule var-head (?r) :match (?r ?a ?b) :assert ?a :why \"v\"))",
    "(rules (rule wild () :match (_ ?a ?b) :assert ?a :why \"w\"))",
    "(rules (rule mixed (?r ?s) :match (?r ?a (?s ?b ?c)) :assert ?a"
    " :why \"m\"))",
    # Empty bodies
    "(facts)",
    "(rules)",
    "(reasoning)",
    "(trace)",
    "(ontology (type T))",
    "(rules (rule x () :match a :assert b :why \"x\"))",
    # Kw-pair ordering — same form, kw-pairs swapped
    "(rules (rule p () :match a :assert b :priority 1 :why \"p\"))",
    "(rules (rule p () :match a :priority 1 :assert b :why \"p\"))",
    "(rules (rule p () :why \"p\" :match a :assert b :priority 1))",
    # Equality patterns / facts
    "(facts (= a b))",
    "(facts (= (color House_1) Red))",
    "(rules (rule eq () :match (= ?a ?b) :assert ?a :why \"e\"))",
    # Negation
    "(facts (not (lives-in Spaniard Coffee)))",
    "(facts (not (not (lives-in a b))))",
    "(rules (rule n () :match (rel ?a ?b) :assert (not (rel ?b ?a))"
    " :why \"n\"))",
    "(reasoning (not (co-located N H) :rule type-exclusivity"
    " :using (s1)))",
    # Mixed kernel primitives
    "(rules (rule mix () :match (and (or (rel ?a ?b) (rel ?b ?a))"
    " (not (rel ?a ?a)) :where (neq ?a ?b)) :assert ?a :why \"mx\"))",
    # Reasoning provenance
    "(reasoning (co-located Blue House_2 :rule square-fwd"
    " :using (c10 c15)))",
    # Numbers (positive, negative, zero)
    "(ontology (relation r A B :priority 0))",
    "(ontology (relation r A B :priority -7))",
    "(ontology (relation r A B :priority 9999))",
    # Instance as fact + as pattern
    "(ontology (instance Norwegian Nationality))",
    "(rules (rule i () :match (instance ?a ?T) :assert ?a :why \"i\"))",
]


@pytest.mark.parametrize("src", ROUNDTRIP_CASES)
def test_roundtrip(src: str) -> None:
    ast1 = parse(src)
    text = dump_canonical(ast1)
    ast2 = parse(text)
    assert ast1 == ast2, f"roundtrip diverged:\n--src--\n{src}\n--dump--\n{text}"


@pytest.mark.parametrize("path", EXAMPLE_FILES, ids=lambda p: p.name)
def test_roundtrip_example(path):
    """Each bundled example survives dump∘parse without loss."""
    src = path.read_text(encoding="utf-8")
    ast1 = parse(src, filename=str(path))
    text = dump_canonical(ast1)
    ast2 = parse(text)
    assert ast1 == ast2


@pytest.mark.parametrize("path", EXAMPLE_FILES, ids=lambda p: p.name)
def test_dump_stable(path):
    """dump∘parse∘dump∘parse == dump∘parse — printer is a fixed point."""
    src = path.read_text(encoding="utf-8")
    t1 = dump_canonical(parse(src))
    t2 = dump_canonical(parse(t1))
    assert t1 == t2


@pytest.mark.parametrize("path", EXAMPLE_FILES, ids=lambda p: p.name)
def test_golden_example(path):
    """Snapshot: dump_canonical(parse(<example>)) matches tests/golden/<example>.golden.

    Catches IR drift across phases (a change in grammar, AST shape,
    or dumper output trips this; refresh the golden if the change is
    intentional with:
        python3 -c "from pathlib import Path; from ein_bot.ir import \\
            parse, dump_canonical; \\
            stem = 'zebra'  # or 'zebra2' \\
            Path(f'tests/golden/{stem}.golden').write_text( \\
                dump_canonical(parse(Path(f'examples/{stem}.ein').read_text())))"
    """
    src = path.read_text(encoding="utf-8")
    golden = REPO / "tests" / "golden" / f"{path.stem}.golden"
    got = dump_canonical(parse(src))
    expected = golden.read_text(encoding="utf-8")
    if got != expected:
        import difflib
        diff = "".join(difflib.unified_diff(
            expected.splitlines(keepends=True),
            got.splitlines(keepends=True),
            fromfile=golden.name,
            tofile=f"dump_canonical(parse({path.name}))",
        ))
        raise AssertionError(f"golden snapshot drift:\n{diff}")


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
    ("(facts (instance Norwegian))", "1:"),      # instance arity 1
    ("(step s1)", "1:"),                         # step at top level
])
def test_error_has_location(src: str, marker: str) -> None:
    with pytest.raises(IRParseError) as ei:
        parse(src, filename="<demo>")
    msg = str(ei.value)
    assert "<demo>:" in msg
    assert marker in msg
