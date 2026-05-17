"""State: mutation + queries + carry-forward stubs."""
from __future__ import annotations

import pytest

from ein_bot import State, load_into
from ein_bot.parser import parse


def canonical(state: State) -> tuple[list[str], list[tuple[str, str, str]]]:
    """Order-independent representation for equality assertions."""
    objs = sorted(state.objects)
    triples = sorted(
        (src, rel, dst)
        for rel, rel_dict in state.relations.items()
        for src, dsts in rel_dict.items()
        for dst in dsts
    )
    return objs, triples


# ---------------------------------------------------------------------------
# Construction / mutation
# ---------------------------------------------------------------------------

def test_state_starts_empty():
    s = State()
    assert s.objects == {} and s.relations == {}


def test_obj_is_idempotent():
    s = State()
    a = s.obj("Alice")
    b = s.obj("Alice")
    assert a is b
    assert s.objects == {"Alice": {}}


def test_rel_populates_both_indices():
    s = State()
    s.rel("Bob", "is", "Person")
    assert s.relations == {"is": {"Bob": {"Person"}}}
    assert s.objects["Bob"] == {"is": {"Person"}}
    # Destination gets an empty outgoing-rels dict.
    assert s.objects["Person"] == {}


def test_rel_multiple_destinations_same_source():
    s = State()
    s.rel("Bob", "is", "Person")
    s.rel("Bob", "is", "Engineer")
    assert s.objects["Bob"]["is"] == {"Person", "Engineer"}
    assert s.relations["is"]["Bob"] == {"Person", "Engineer"}


# ---------------------------------------------------------------------------
# Dump round-trip
# ---------------------------------------------------------------------------

def test_dump_round_trip(tiny_state):
    s2 = State()
    load_into(s2, tiny_state.dump().splitlines())
    assert canonical(s2) == canonical(tiny_state)


def test_load_round_trip_full(conditions_lines):
    s1, s2 = State(), State()
    load_into(s1, conditions_lines)
    load_into(s2, s1.dump().splitlines())
    assert canonical(s1) == canonical(s2)


# ---------------------------------------------------------------------------
# Queries
# ---------------------------------------------------------------------------

def test_select_rel_keeps_only_matching(conditions_lines):
    s = State()
    load_into(s, conditions_lines)
    only_is = s.select_rel("is")
    assert set(only_is.relations) == {"is"}


def test_select_rel_exclude(conditions_lines):
    s = State()
    load_into(s, conditions_lines)
    no_is = s.select_rel("is", exclude=True)
    assert "is" not in no_is.relations


def test_select_rel_preserve_objs():
    s = State()
    s.obj("LonelyObj")
    s.rel("Bob", "is", "Person")
    out = s.select_rel("is", preserve_objs=True)
    assert "LonelyObj" in out.objects


def test_ends_and_not_ends_partition(conditions_lines):
    s = State()
    load_into(s, conditions_lines)
    ends = set(s.ends("is"))
    not_ends = set(s.not_ends("is"))
    assert ends.isdisjoint(not_ends)
    assert ends | not_ends == set(s.objects)


def test_ends_on_clean_typed_subgraph():
    """In a clean typed graph (no cross-`is` edges), every destination is an end."""
    s = State()
    s.rel("House_1", "is", "House")
    s.rel("House_2", "is", "House")
    s.rel("Ivory",   "is", "Color")
    assert set(s.ends("is"))     == {"House", "Color"}
    assert set(s.not_ends("is")) == {"House_1", "House_2", "Ivory"}


def test_obj_types_returns_matching_end_types():
    s = State()
    s.rel("House_1", "is", "House")
    s.rel("Ivory", "is", "Color")
    # House_1 is typed only as "House".
    assert s.obj_types("House_1", "is") == ["House"]
    # Missing object -> [].
    assert s.obj_types("nonexistent", "is") == []


# ---------------------------------------------------------------------------
# Stubs: explicit NotImplementedError
# ---------------------------------------------------------------------------

@pytest.mark.parametrize("attr,args", [
    ("select_obj", ("foo",)),
    ("rel_types", ("is",)),
    ("verify_single_rel_constraint", ()),
])
def test_carry_forward_stubs_raise(attr, args):
    s = State()
    with pytest.raises(NotImplementedError):
        getattr(s, attr)(*args)


# ---------------------------------------------------------------------------
# Misc
# ---------------------------------------------------------------------------

def test_copy_is_independent(tiny_state):
    other = tiny_state.copy()
    other.rel("X", "is", "Y")
    assert "X" not in tiny_state.objects
    assert "X" in other.objects


def test_parse_then_state_consistent():
    """The parser-emitted tuples should drive State to the same shape."""
    lines = ["Alice", "Bob is Person"]
    s_via_parse = State()
    for tup in parse(lines):
        if len(tup) == 1:
            s_via_parse.obj(tup[0])
        else:
            s_via_parse.rel(*tup)

    s_via_load = State()
    load_into(s_via_load, lines)

    assert canonical(s_via_parse) == canonical(s_via_load)
