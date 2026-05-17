"""ein-bot — graph-based Zebra-puzzle reasoner (PoC refactor).

This package is the cleaned-up successor to the 2021 single-file
``reasoning.py`` (archived under ``docs/PoC/``). The deep redesign is
tracked separately in ``TODO.md`` and the ``docs/ideas/`` files.
"""
from .parser import load_file, load_into, parse
from .state import State
from .versioned import VersionedState

__all__ = ["State", "VersionedState", "load_file", "load_into", "parse"]
__version__ = "0.0.1"
