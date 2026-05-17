"""Rendering helpers + State.dot()."""
from __future__ import annotations

import re

from ein_bot import State, load_into
from ein_bot.rendering import hash_color, random_dot_color

HEX_RE = re.compile(r"^#[0-9A-F]{6}$")


# ---------------------------------------------------------------------------
# Colour helpers
# ---------------------------------------------------------------------------

def test_hash_color_format():
    assert HEX_RE.fullmatch(hash_color("anything"))


def test_hash_color_is_deterministic():
    assert hash_color("foo") == hash_color("foo")
    assert hash_color("foo") != hash_color("bar")


def test_random_dot_color_format():
    for _ in range(20):
        assert HEX_RE.fullmatch(random_dot_color())


# ---------------------------------------------------------------------------
# State.dot()
# ---------------------------------------------------------------------------

def test_dot_well_formed(tiny_state):
    out = tiny_state.dot()
    assert out.startswith("digraph G {\n")
    assert out.rstrip().endswith("}")


def test_dot_includes_every_object_and_edge(tiny_state):
    out = tiny_state.dot()
    for name in tiny_state.objects:
        assert f"  {name};" in out
    edge_count = sum(
        len(dsts)
        for rel in tiny_state.relations.values()
        for dsts in rel.values()
    )
    assert out.count("->") == edge_count


def test_dot_colorfull_false_uses_plain_labels():
    s = State()
    load_into(s, ["Bob is Person"])
    out = s.dot(colorfull=False)
    assert '[label="is"]' in out
    assert "<font" not in out


def test_dot_colorfull_true_uses_font_tags():
    s = State()
    load_into(s, ["Bob is Person"])
    out = s.dot(colorfull=True)
    assert "<font color=" in out
