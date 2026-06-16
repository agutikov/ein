"""Solve-result rendering — template-driven, no hardcoded vocabulary
(`trace.answer`).

Every word of English in the ``solve`` table comes from puzzle-authored
templates: each ``(relation R … :why "<tmpl>")`` renders one fact with
``{?1}``/``{?2}`` bound to its args positionally; the ``(query … :goal-text
"<tmpl>")`` renders the headline against the goal's own vars. A relation with
no ``:why`` renders as its raw IR s-expression; a query with no ``:goal-text``
omits the result sentence. These build solved KBs directly (no solving), so the
render path has fast coverage outside the slow ``acceptance/`` run.
"""
from __future__ import annotations

from types import SimpleNamespace

from ein.inference.verdict import Solution
from ein.ir import parse
from ein.kb.store import KnowledgeBase
from ein.trace import render_answer, render_solution_table


def _solution(text: str) -> Solution:
    kb = KnowledgeBase.from_ir(parse(text))
    return Solution(kb=kb, trace=())


def _stats(k: int = 1, exhausted: bool = False) -> SimpleNamespace:
    return SimpleNamespace(solution_nodes=k, exhausted=exhausted)


# A complete model encoded directly as facts (no solving needed) + the
# zebra2-shaped who-vs-where goal, with per-relation :why templates and a
# query :goal-text — exactly the two puzzle-template sources the renderer uses.
_ZEBRA_SHAPED = """
(relation drink-loc  Drink       House :why "{?1} is drunk in {?2}")
(relation nation-loc Nationality House :why "the {?1} lives in {?2}")
(relation pet-loc    Pet         House :why "the {?1} is kept in {?2}")
(drink-loc  Water     House-1 :layer fact)
(nation-loc Norwegian House-1 :layer fact)
(pet-loc    Zebra     House-5 :layer fact)
(nation-loc Japanese  House-5 :layer fact)
(query
  :goal (and (drink-loc  Water      ?h_water)
             (nation-loc ?who_water ?h_water)
             (pet-loc    Zebra      ?h_zebra)
             (nation-loc ?who_zebra ?h_zebra))
  :goal-text "The {?who_water} drinks in {?h_water}; the {?who_zebra} owns the zebra in {?h_zebra}")
"""


# ── headline (render_answer) ───────────────────────────────────────


def test_headline_comes_from_goal_text():
    """The NL result is the query :goal-text rendered against the bound goal
    vars — the words ('drinks', 'owns the zebra') live in the puzzle, not here."""
    ans = render_answer(_solution(_ZEBRA_SHAPED))
    assert "The Norwegian drinks in House-1" in ans
    assert "the Japanese owns the zebra in House-5" in ans


def test_headline_not_exhausted_hint():
    """A non-exhaustive solve appends the certify hint; exhaustive does not."""
    sol = _solution(_ZEBRA_SHAPED)
    assert "--exhaustive" in render_answer(sol, exhausted=False)
    assert "--exhaustive" not in render_answer(sol, exhausted=True)


def test_goal_text_vars_not_hardcoded():
    """:goal-text binds whatever vars the query names — here `owner-loc` and
    `?who`, not nation-loc. Regression against any hardcoded projection."""
    text = """
    (relation drink-loc Drink House :why "{?1} drunk at {?2}")
    (relation owner-loc Owner House)
    (drink-loc Water House-1 :layer fact)
    (owner-loc Zaphod House-1 :layer fact)
    (query
      :goal (and (drink-loc Water ?h) (owner-loc ?who ?h))
      :goal-text "{?who} drinks the water")
    """
    assert "Zaphod drinks the water" in render_answer(_solution(text))


def test_no_goal_text_is_graceful():
    """No :goal-text → a neutral 'Solved.' headline (never invented prose)."""
    text = """
    (relation drink-loc Drink House :why "{?1} at {?2}")
    (drink-loc Water House-1 :layer fact)
    (query :goal (drink-loc Water ?h))
    """
    assert render_answer(_solution(text)) == "Solved."


# ── relation :why positional rendering (the facts column) ──────────


def test_table_renders_facts_via_relation_why():
    """Each goal conjunct is rendered by its relation's :why with {?1}/{?2}
    bound positionally to the fact's args."""
    table = render_solution_table(_solution(_ZEBRA_SHAPED), _stats())
    assert "Water is drunk in House-1" in table
    assert "the Norwegian lives in House-1" in table
    assert "the Zebra is kept in House-5" in table
    assert "the Japanese lives in House-5" in table


def test_table_fact_without_why_falls_back_to_ir():
    """A relation with no :why renders as its raw IR s-expression — no
    invented vocabulary. `owner-loc` here has no template."""
    text = """
    (relation drink-loc Drink House :why "{?1} drunk at {?2}")
    (relation owner-loc Owner House)
    (drink-loc Water House-1 :layer fact)
    (owner-loc Zaphod House-1 :layer fact)
    (query :goal (and (drink-loc Water ?h) (owner-loc ?who ?h)))
    """
    table = render_solution_table(_solution(text), _stats())
    assert "(owner-loc Zaphod House-1)" in table   # IR fallback, untemplated
    assert "Water drunk at House-1" in table        # templated


def test_no_templates_at_all_has_zero_invented_prose():
    """A puzzle with neither :why nor :goal-text yields a table whose every
    cell is IR / bindings — the rendered column equals the fact column and the
    result line is the explicit no-template note."""
    text = """
    (relation drink-loc Drink House)
    (drink-loc Water House-1 :layer fact)
    (query :goal (drink-loc Water ?h))
    """
    table = render_solution_table(_solution(text), _stats())
    assert "(drink-loc Water House-1)" in table
    assert "no :goal-text template" in table


# ── the five table fields ──────────────────────────────────────────


def test_table_has_all_five_fields():
    table = render_solution_table(
        _solution(_ZEBRA_SHAPED), _stats(k=1), exhausted=False, source="z.ein")
    assert "solutions (k)   1" in table          # 1. count
    assert "verdict         Solution" in table   # 2. verdict
    assert "query bindings" in table             # 3. raw query values
    assert "?who_water  = Norwegian" in table
    assert "query facts" in table and "rendered" in table   # 4. rendered facts
    assert "result" in table                     # 5. NL result
    assert "z.ein" in table                       # source echoed in the title
    assert "not certified" in table               # non-exhaustive note
