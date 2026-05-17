"""VersionedState: stacking + copy-on-write isolation."""
from __future__ import annotations

from ein_bot import State, VersionedState


def test_root_construction():
    v = VersionedState()
    assert v.level == 0
    assert v._base is None
    assert v.readonly is False
    assert isinstance(v.state, State)
    assert v.state.objects == {}


def test_inc_version_freezes_parent():
    parent = VersionedState()
    parent.state.rel("Bob", "is", "Person")
    child = parent.inc_version()
    assert child.level == 1
    assert child._base is parent
    assert parent.readonly is True
    assert child.readonly is False


def test_inc_version_deepcopies_state():
    parent = VersionedState()
    parent.state.rel("Bob", "is", "Person")
    child = parent.inc_version()
    # Child sees what the parent had.
    assert "Bob" in child.state.objects
    # Mutating the child's state does NOT touch parent's state (deepcopy).
    child.state.rel("Alice", "is", "Person")
    assert "Alice" in child.state.objects
    assert "Alice" not in parent.state.objects


def test_dump_threads_through_levels():
    parent = VersionedState()
    parent.state.obj("Alice")
    child = parent.inc_version()
    child.state.obj("Bob")
    out = child.dump()
    assert "# Level" in out
    assert "Alice" in out
    assert "Bob" in out


def test_dot_delegates_to_current_state():
    v = VersionedState()
    v.state.rel("Bob", "is", "Person")
    out = v.dot()
    assert "digraph G {" in out
    assert "Bob -> Person" in out
