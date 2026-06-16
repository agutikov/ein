"""Guided hypothesis generation — S1.5.6b.

Two ways to steer what `hypgen` enumerates:

- **T1.5.6b.1** — a `(query :hypothesis-relations …)` whitelist
  restricting which relations the blind enumerator builds
  candidates for;
- **T1.5.6b.2** — `(hrule …)` declarations: rules that *generate*
  candidate hypotheses. A hrule lives in the `(rules …)` block but
  loads into `kb.hrules` (separate from `kb.rules`); the saturator
  never fires it. A hrule may be generic — parameterised — and
  activated by `(query … :hrules …)`.

Rule, hrule, and relation names must be unique — a duplicate is a
load error.
"""
import pytest

from ein.inference.hrule import Hrules
from ein.inference.hypgen import (
    generate_hypotheses,
    generate_hypotheses_with_stats,
)
from ein.inference.saturator import Saturator
from ein.ir import dump_canonical, parse
from ein.kb.entities import Fact
from ein.kb.from_ir import KBLoadError
from ein.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# Two declared relations + two instance-like objects — the blind
# enumerator produces both `co-located` and `likes` candidates.
_TWO_REL = """
(relation co-located T T)
(relation likes      T T)
(is-a Thing T)
(is-a A Thing) (is-a B Thing)
"""

# A parameter-less hrule declared in the (rules …) block: every
# (House, Color) pair is a co-located candidate.
_HRULE = """
(hrule guess-co-located ()
  :match  (and (is-a ?o House) (is-a ?v Color))
  :assert (co-located ?o ?v)
  :why    "guess co-located")
(relation is-a       T T)
(relation co-located T T)
(is-a House T) (is-a Color T)
(is-a H1 House) (is-a H2 House)
(is-a Red Color) (is-a Blue Color)
"""

# A generic hrule — parameterised over (?rel ?type), activated by
# the `:hrules` block in (query …) with two (rel type) tuples.
_GENERIC_HRULE = """
(hrule guess (?rel ?type)
  :match  (and (is-a ?a ?type) (is-a ?house House))
  :assert (?rel ?a ?house)
  :why    "g")
(relation is-a       T T)
(relation co-located T T)
(is-a House T) (is-a Color T) (is-a Pet T)
(is-a H1 House)
(is-a Red Color)
(is-a Dog Pet)
(query :goal (co-located Red H1)
       :hrules (guess (co-located Color) (co-located Pet)))
"""

# Two hrules sharing a name — a load error.
_DUP_HRULE = """
(hrule g () :match (is-a ?o House) :assert (co-located ?o ?o) :why "a")
(hrule g () :match (is-a ?o House) :assert (co-located ?o ?o) :why "b")
(relation is-a T T) (relation co-located T T)
(is-a House T) (is-a H1 House)
"""


# ── T1.5.6b.1 — query whitelist ────────────────────────────────────


def test_query_whitelist_restricts_enumerator():
    """`(query :hypothesis-relations (co-located))` keeps the
    enumerator to `co-located`; `likes` counts as a
    `relation_not_whitelisted` pre-candidate skip."""
    kb = _kb(_TWO_REL + "(query :goal (co-located A B)"
                        "        :hypothesis-relations (co-located))")
    facts, stats = generate_hypotheses_with_stats(kb)
    assert facts
    assert all(f.relation_name == "co-located" for f in facts)
    assert stats.pre_candidate["relation_not_whitelisted"] > 0


def test_query_single_relation_whitelist():
    """A bare SYMBOL value is a one-relation whitelist."""
    kb = _kb(_TWO_REL + "(query :goal (co-located A B)"
                        "        :hypothesis-relations co-located)")
    facts, _stats = generate_hypotheses_with_stats(kb)
    assert all(f.relation_name == "co-located" for f in facts)


def test_no_whitelist_enumerates_all_relations():
    """No `:hypothesis-relations` ⇒ every declared relation runs."""
    kb = _kb(_TWO_REL)
    facts, _stats = generate_hypotheses_with_stats(kb)
    assert {"co-located", "likes"} <= {f.relation_name for f in facts}


# ── S1.9.E3 — query :no-hypothesis blacklist ───────────────────────


def test_query_no_hypothesis_excludes_relation():
    """`(query :no-hypothesis (likes))` drops `likes` from the
    enumerator while `co-located` still runs; the skip counts as a
    `no_hypothesis_relation` pre-candidate skip."""
    kb = _kb(_TWO_REL + "(query :goal (co-located A B)"
                        "        :no-hypothesis (likes))")
    facts, stats = generate_hypotheses_with_stats(kb)
    assert facts
    assert all(f.relation_name != "likes" for f in facts)
    assert any(f.relation_name == "co-located" for f in facts)
    assert stats.pre_candidate["no_hypothesis_relation"] > 0


def test_query_no_hypothesis_single_relation():
    """A bare SYMBOL value is a one-relation blacklist."""
    kb = _kb(_TWO_REL + "(query :goal (co-located A B)"
                        "        :no-hypothesis likes)")
    facts, _stats = generate_hypotheses_with_stats(kb)
    assert facts
    assert all(f.relation_name != "likes" for f in facts)


def test_no_hypothesis_overrides_whitelist():
    """The blacklist applies on top of the whitelist: a relation named
    by both `:hypothesis-relations` and `:no-hypothesis` is excluded."""
    kb = _kb(_TWO_REL + "(query :goal (co-located A B)"
                        "        :hypothesis-relations (co-located likes)"
                        "        :no-hypothesis (likes))")
    facts, _stats = generate_hypotheses_with_stats(kb)
    assert facts
    assert all(f.relation_name == "co-located" for f in facts)


def test_no_no_hypothesis_enumerates_all_relations():
    """No `:no-hypothesis` ⇒ no relation is blacklisted."""
    kb = _kb(_TWO_REL)
    _facts, stats = generate_hypotheses_with_stats(kb)
    assert stats.pre_candidate["no_hypothesis_relation"] == 0


# ── T1.5.6b.2 — hrule mechanism ────────────────────────────────────


def test_hrule_loaded_into_kb_hrules():
    """A `(hrule …)` form loads into `kb.hrules`, not `kb.rules`."""
    kb = _kb(_HRULE)
    assert "guess-co-located" in kb.hrules
    assert "guess-co-located" not in kb.rules


def test_hrule_form_round_trips():
    """The `(hrule …)` form survives dump → parse unchanged."""
    forms = parse(_HRULE)
    assert parse(dump_canonical(forms)) == forms


def test_hrule_generates_candidates():
    """`Hrules` runs the declared hrule; each match is a candidate."""
    kb = _kb(_HRULE)
    cands = list(Hrules(kb).candidates(kb))
    assert len(cands) == 4  # {H1,H2} x {Red,Blue}
    assert Fact("co-located", ("H1", "Red")) in cands


def test_hrule_not_fired_by_saturator():
    """A hrule is a generator, not a derivation rule — it lives
    outside `kb.rules`, so the saturator never fires it."""
    kb = _kb(_HRULE)
    list(Saturator(kb).saturate())
    assert not kb._facts_by_relation.get("co-located")


def test_hrule_disables_enumerator():
    """Any hrule present ⇒ generation is the hrule's output only;
    the blind enumerator does not run."""
    kb = _kb(_HRULE)
    facts = list(generate_hypotheses(kb))
    assert len(facts) == 4
    assert all(f.relation_name == "co-located" for f in facts)


def test_no_hrule_runs_enumerator():
    """No hrule ⇒ the blind enumerator runs."""
    kb = _kb(_TWO_REL)
    assert list(generate_hypotheses(kb))


def test_generic_hrule_via_query_hrules():
    """A parameterised hrule is activated by `(query … :hrules
    (NAME (args…) …))` — one plan per activator tuple."""
    kb = _kb(_GENERIC_HRULE)
    cands = list(Hrules(kb).candidates(kb))
    assert len(cands) == 2  # one per (rel type) activator
    assert Fact("co-located", ("Red", "H1")) in cands
    assert Fact("co-located", ("Dog", "H1")) in cands


# ── name uniqueness ────────────────────────────────────────────────


def test_duplicate_hrule_name_rejected():
    """Two hrules sharing a name is a load error."""
    with pytest.raises(KBLoadError):
        _kb(_DUP_HRULE)


def test_duplicate_relation_rejected():
    """Two `(relation R …)` declarations for the same R is a load
    error."""
    with pytest.raises(KBLoadError):
        _kb("(relation co-located T T) (relation co-located T T)")
