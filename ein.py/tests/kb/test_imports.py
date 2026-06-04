"""Loader behaviour for `(import …)` + the reserved-name guard (P1.8 S1.8.A2).

S1.8.A2 lands the *grammar* for imports; resolution is S1.8.A3. So at load
time a present `(import …)` is an explicit "not yet" error, never a silent
no-op. This file also pins the D3 reserved-name guard, now shared across all
declarators (macro / rule / relation) — a name that shadows kernel vocabulary
is rejected, while a *fact* may still carry a reserved head (a stored
`(not X)` octagon).
"""
from __future__ import annotations

import pytest

from ein_bot.ir import parse
from ein_bot.kb.from_ir import KBLoadError
from ein_bot.kb.store import KnowledgeBase


def _load(src: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(src))


# ── Imports: recognised, resolution pending (A3) ───────────────────


@pytest.mark.parametrize("src", [
    "(import std.macro)",
    "(import std.macro :as m)",
    "(import std.macro :symbols (forall open))",
])
def test_import_load_is_pending_not_a_fact(src):
    """An import is routed (not bucketed as a fact named `import`) and fails
    loudly until A3 implements resolution."""
    with pytest.raises(KBLoadError, match=r"import resolution is not implemented"):
        _load(src)


def test_import_error_names_the_module():
    with pytest.raises(KBLoadError, match=r"\(import std\.macro\)"):
        _load("(import std.macro :symbols (forall))")


# ── Reserved-name guard (D3), shared across declarators ────────────


@pytest.mark.parametrize("name", ["absent", "false", "eq", "relation"])
def test_reserved_rule_name_rejected(name):
    with pytest.raises(KBLoadError, match=r"shadows a reserved kernel name"):
        _load(f'(rule {name} () :match (x ?a) :assert (y ?a) :why "w")')


@pytest.mark.parametrize("name", ["absent", "false", "eq", "relation"])
def test_reserved_relation_name_rejected(name):
    with pytest.raises(KBLoadError, match=r"shadows a reserved kernel name"):
        _load(f"(relation {name} T T)")


def test_reserved_hrule_name_rejected():
    with pytest.raises(KBLoadError, match=r"hrule 'absent' shadows"):
        _load('(hrule absent () :match (x ?a) :assert (y ?a) :why "w")')


def test_non_reserved_names_still_load():
    """A name merely *containing* a reserved word (exact-match only) loads."""
    kb = _load("""
    (rule eq-elim () :match (x ?a) :assert (y ?a) :why "ok")
    (relation absent-of T T)
    (relation x T T) (relation y T T)
    """)
    assert "eq-elim" in kb.rules
    assert "absent-of" in kb.relations


def test_fact_may_have_a_reserved_head():
    """The guard is about *binding* a name, not fact heads — a stored
    `(not X)` octagon (a reserved head) must still load."""
    kb = _load('(not (likes A B) :source "(1)") (relation likes T T)')
    assert any(f.relation_name == "not" for f in kb.facts)
