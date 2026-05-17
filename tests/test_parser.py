"""Parser-level tests: tokenisation rules + load_into / load_file."""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot import State, load_file, load_into
from ein_bot.parser import parse


def test_skips_empty_and_whitespace_lines():
    assert list(parse(["", "   ", "\n", "\t"])) == []


def test_single_token_yields_object_tuple():
    assert list(parse(["Alice"])) == [("Alice",)]


def test_three_tokens_yield_triple():
    assert list(parse(["Bob is Person"])) == [("Bob", "is", "Person")]


def test_multi_token_relation_collapses_middle():
    out = list(parse(["Carol moves toward Dan"]))
    assert out == [("Carol", "moves toward", "Dan")]


def test_two_tokens_raises():
    with pytest.raises(ValueError, match="2 tokens"):
        list(parse(["foo bar"]))


def test_load_into_populates_state():
    s = State()
    load_into(s, ["Alice", "Bob is Person"])
    assert "Alice" in s.objects
    assert "Bob" in s.objects
    assert "Person" in s.objects
    assert s.relations == {"is": {"Bob": {"Person"}}}


def test_load_file_reads_conditions(conditions_path: Path):
    s = State()
    load_file(s, conditions_path)
    # Sanity: the file references at least the five houses and several types.
    for i in range(1, 6):
        assert f"House_{i}" in s.objects
    for type_name in ("House", "Color", "Yellow", "Ivory"):
        assert type_name in s.objects
    # Spatial relations from the head of the file.
    assert "left_to" in s.relations
    assert s.relations["left_to"]["House_1"] == {"House_2"}
