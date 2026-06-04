"""S1.5.8c T1.5.8c.4 — M1-blocking stdlib block.

Four parameterised rules: typecheck-arg-{0,1}, domain-elimination,
no-room-left. Each takes activator args that carry type info
(in lieu of arg-type reflection — grammar reserves `relation`).
"""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.saturator import Saturator
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO_ROOT = Path(__file__).resolve().parents[3]
DEMO = REPO_ROOT / "examples" / "features" / "05_stdlib_domain_elim.ein"


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _saturate(kb: KnowledgeBase) -> list:
    return list(Saturator(kb).saturate(max_steps=10000))


STDLIB = """
;; inline std.macro sugar (until ein-lang imports land — P1.8 S1.8.A1-A5)
(macro forall (?b ?G ?B)
  (absent (and ?G (absent ?B))))
(rule typecheck-arg-0 (?R ?Dom)
  :match  (and (?R ?a ?b) (absent (is-a ?a ?Dom)))
  :assert (false) :priority 110 :why "t0")
(rule typecheck-arg-1 (?R ?Ran)
  :match  (and (?R ?a ?b) (absent (is-a ?b ?Ran)))
  :assert (false) :priority 110 :why "t1")
(rule domain-elimination (?R ?OT ?VT)
  :match  (and (functional ?R 0 1) (total ?R 0)
               (is-a ?a ?OT) (is-a ?v ?VT)
               (forall ?v_other
                 (and (is-a ?v_other ?VT) (neq ?v_other ?v))
                 (not (?R ?a ?v_other))))
  :assert (?R ?a ?v) :priority 400 :why "de")
(rule no-room-left (?R ?OT ?VT)
  :match  (and (functional ?R 0 1) (total ?R 0)
               (is-a ?a ?OT)
               (forall ?v (is-a ?v ?VT) (not (?R ?a ?v))))
  :assert (false) :priority 110 :why "nrl")
"""


def test_demo_file_solves_to_color_of_house2_blue():
    """The bundled demo file fires exactly one domain-elimination
    deriving (color-of House-2 Blue) and nothing else."""
    kb = _kb(DEMO.read_text(encoding="utf-8"))
    firings = _saturate(kb)
    productive = [f for f in firings if not f.redundant]
    assert len(productive) == 1
    f = productive[0]
    assert f.rule == "domain-elimination"
    assert f.derived.relation_name == "color-of"
    assert f.derived.args == ("House-2", "Blue")


def test_domain_elim_forces_unique_survivor():
    kb = _kb(STDLIB + """
    (relation color-of House Color) (relation is-a T T)
    (functional color-of 0 1) (total color-of 0)
    (domain-elimination color-of House Color)
    (is-a House T) (is-a Color T)
    (is-a H1 House)
    (is-a Red Color) (is-a Blue Color) (is-a Green Color)
    (not (color-of H1 Red) :source "(a)")
    (not (color-of H1 Blue) :source "(b)")
    """)
    firings = _saturate(kb)
    derived = [
        f.derived for f in firings
        if not f.redundant and f.rule == "domain-elimination"
    ]
    assert len(derived) == 1
    assert derived[0].args == ("H1", "Green")


def test_no_room_left_fires_when_every_value_excluded():
    kb = _kb(STDLIB + """
    (relation color-of House Color) (relation is-a T T)
    (functional color-of 0 1) (total color-of 0)
    (no-room-left color-of House Color)
    (is-a House T) (is-a Color T)
    (is-a H1 House)
    (is-a Red Color) (is-a Blue Color)
    (not (color-of H1 Red) :source "(a)")
    (not (color-of H1 Blue) :source "(b)")
    """)
    firings = _saturate(kb)
    falses = [
        f for f in firings
        if not f.redundant and f.rule == "no-room-left"
    ]
    assert len(falses) == 1


def test_no_room_left_silent_when_one_value_remains():
    """Mirror of no-room-left: one survivor → domain-elim fires,
    no-room-left silent."""
    kb = _kb(STDLIB + """
    (relation color-of House Color) (relation is-a T T)
    (functional color-of 0 1) (total color-of 0)
    (no-room-left color-of House Color)
    (is-a House T) (is-a Color T)
    (is-a H1 House)
    (is-a Red Color) (is-a Blue Color)
    (not (color-of H1 Red) :source "(a)")
    """)
    firings = _saturate(kb)
    nrl = [f for f in firings if not f.redundant and f.rule == "no-room-left"]
    assert nrl == []


def test_typecheck_fires_on_mistyped_arg():
    """Wrong-typed arg → typecheck fires (false)."""
    kb = _kb(STDLIB + """
    (relation color-of House Color) (relation is-a T T)
    (typecheck-arg-0 color-of House)
    (is-a House T) (is-a Color T) (is-a Person T)
    (is-a Englishman Person)
    (is-a H1 House) (is-a Red Color)
    (color-of Englishman Red :source "(bad)")
    """)
    firings = _saturate(kb)
    tc = [f for f in firings if not f.redundant and f.rule == "typecheck-arg-0"]
    assert len(tc) == 1


def test_typecheck_silent_on_well_typed_facts():
    kb = _kb(STDLIB + """
    (relation color-of House Color) (relation is-a T T)
    (typecheck-arg-0 color-of House)
    (typecheck-arg-1 color-of Color)
    (is-a House T) (is-a Color T)
    (is-a H1 House) (is-a Red Color)
    (color-of H1 Red :source "(ok)")
    """)
    firings = _saturate(kb)
    tc = [
        f for f in firings if not f.redundant
        and f.rule in ("typecheck-arg-0", "typecheck-arg-1")
    ]
    assert tc == []


def test_domain_elim_silent_without_total_or_functional():
    """If the property facts aren't declared, domain-elim doesn't fire."""
    kb = _kb(STDLIB + """
    (relation color-of House Color) (relation is-a T T)
    (domain-elimination color-of House Color)
    (is-a House T) (is-a Color T)
    (is-a H1 House)
    (is-a Red Color) (is-a Blue Color)
    (not (color-of H1 Red) :source "(a)")
    """)
    firings = _saturate(kb)
    de = [
        f for f in firings if not f.redundant
        and f.rule == "domain-elimination"
    ]
    assert de == []
