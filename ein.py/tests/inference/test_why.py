"""`:why` template substitution tests — S1.3.1 T1.3.1.9."""
from __future__ import annotations

from ein_bot.inference.why import render_why


def test_single_substitution():
    assert render_why("{?rel} is transitive", {"rel": "co-located"}) == \
        "co-located is transitive"


def test_multiple_substitutions():
    out = render_why(
        "{?rel} is transitive: {?a} →{?rel}→ {?b} →{?rel}→ {?c}.",
        {"rel": "co-located", "a": "Norwegian", "b": "House-1", "c": "Red"},
    )
    assert out == \
        "co-located is transitive: Norwegian →co-located→ House-1 →co-located→ Red."


def test_bare_braces_left_literal():
    """`{x}` without a leading `?` is NOT a substitution reference (Q31)."""
    assert render_why("{x} stays literal", {"x": "should-not-replace"}) == \
        "{x} stays literal"


def test_unbound_var_left_as_is():
    """A `{?name}` with no binding is left as the literal `{?name}`."""
    out = render_why("{?known}+{?unknown}", {"known": "OK"})
    assert out == "OK+{?unknown}"


def test_integer_binding_str_converted():
    out = render_why("priority={?n}", {"n": 7})
    assert out == "priority=7"


def test_hyphen_in_var_name():
    """Var names admit hyphens (the IR VAR regex allows them)."""
    out = render_why("{?some-var}", {"some-var": "x"})
    assert out == "x"


def test_empty_template_returns_empty():
    assert render_why("", {}) == ""


def test_no_refs_pass_through():
    assert render_why("no refs here", {"a": "X"}) == "no refs here"
