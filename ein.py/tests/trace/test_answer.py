"""English answer rendering — S1.7.5 who-vs-where (`trace.answer`).

The query ``:goal`` carries an *anchor* conjunct ``(drink-loc Water ?h)`` and
a *projection* conjunct ``(nation-loc ?who ?h)`` joined on the house; the
renderer reports "the <who> <verb>s the <anchor>". These exercise the
projection on a hand-built solved KB (fast — no full solve), so the answer
path has coverage outside the slow ``acceptance/`` run.
"""
from __future__ import annotations

from ein_bot.inference.verdict import Solution
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase
from ein_bot.trace import render_answer


def _solution(text: str) -> Solution:
    kb = KnowledgeBase.from_ir(parse(text))
    return Solution(kb=kb, trace=())


# A complete model encoded directly as facts (no solving needed) + the
# zebra2-shaped who-vs-where goal.
_ZEBRA_SHAPED = """
(relation drink-loc  Drink       House)
(relation nation-loc Nationality House)
(relation pet-loc    Pet         House)
(drink-loc  Water     House-1 :layer fact)
(nation-loc Norwegian House-1 :layer fact)
(pet-loc    Zebra     House-5 :layer fact)
(nation-loc Japanese  House-5 :layer fact)
(query
  :mode solve
  :goal (and (drink-loc  Water      ?h_water)
             (nation-loc ?who_water ?h_water)
             (pet-loc    Zebra      ?h_zebra)
             (nation-loc ?who_zebra ?h_zebra)))
"""


def test_answer_projects_who_from_goal():
    """The bound ?who_* (Norwegian / Japanese) is read off the goal, not
    re-scanned — both questions answered as 'who', not 'where'."""
    ans = render_answer(_solution(_ZEBRA_SHAPED)).lower()
    assert "norwegian drinks the water" in ans
    assert "japanese keeps the zebra" in ans
    # The house never leaks into the sentence when a who is projected.
    assert "house-1" not in ans and "house-5" not in ans


def test_projection_relation_not_hardcoded():
    """The 'who' relation is whatever the query names — here `owner-loc`,
    not `nation-loc`. Regression against the old hardcoded `_nation_at`."""
    text = """
    (relation drink-loc Drink House)
    (relation owner-loc Owner House)
    (drink-loc Water House-1 :layer fact)
    (owner-loc Zaphod House-1 :layer fact)
    (query
      :mode solve
      :goal (and (drink-loc Water ?h) (owner-loc ?who ?h)))
    """
    ans = render_answer(_solution(text)).lower()
    assert "zaphod drinks the water" in ans


def test_where_fallback_without_projection():
    """A goal with only an anchor (no projection conjunct) degrades to the
    'where' phrasing rather than dropping the answer."""
    text = """
    (relation drink-loc Drink House)
    (drink-loc Water House-1 :layer fact)
    (query :mode solve :goal (drink-loc Water ?h))
    """
    ans = render_answer(_solution(text)).lower()
    assert "water is in house-1" in ans


def test_no_goal_is_graceful():
    text = """(relation drink-loc Drink House)
(drink-loc Water House-1 :layer fact)"""
    ans = render_answer(_solution(text))
    assert "Solved" in ans
