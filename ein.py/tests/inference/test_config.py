"""SolverConfig `(config …)` coercion — P1.7b S1.7b.3.

Regression coverage for F-KER-4: ``_coerce`` used identity checks
(``field.type is int``) that are dead under ``from __future__ import
annotations`` (the annotation is a *string*), and unwrapped IR values via
``.name`` only — so **every numeric flag** was unsettable through the IR
``(config …)`` block:

- an ``int | None`` field (``lattice-order-seed``) raised
  ``unsupported type 'int | None'``;
- a plain ``int`` field (``candidate-order-seed``) raised because the IR
  ``Int`` node carries ``.value``, not ``.name``.

Both were latent because the test-suite sets seeds via ``dataclasses.replace``
on a constructed ``SolverConfig``, never through ``from_kw_pairs``.
"""
import pytest

from ein.inference.config import SolverConfig
from ein.ir import parse


def _cfg(text: str) -> SolverConfig:
    """Build a SolverConfig from a single ``(config …)`` IR form."""
    return SolverConfig.from_kw_pairs(parse(text)[0].args)


# ── The regression: numeric flags via IR ───────────────────────────


def test_int_or_none_flag_loads_via_ir():
    """`int | None` field — the F-KER-4 crash."""
    assert _cfg("(config :lattice-order-seed 7)").lattice_order_seed == 7


def test_plain_int_flag_loads_via_ir():
    """`int` field — the `.value`-vs-`.name` unwrap crash."""
    assert _cfg("(config :candidate-order-seed 5)").candidate_order_seed == 5


def test_optional_flag_default_is_none():
    assert SolverConfig().lattice_order_seed is None


# ── The other primitive dispatches still work ──────────────────────


def test_bool_flag_via_symbol():
    assert _cfg("(config :print-alive true)").print_alive is True
    assert _cfg("(config :print-alive false)").print_alive is False


def test_str_flag_via_symbol():
    assert _cfg("(config :lattice-order lex)").lattice_order == "lex"


def test_float_flag_accepts_int_literal():
    # The IR grammar has no float token; a float flag takes an int
    # literal or a quoted string.
    assert _cfg("(config :hypgen-rel-weight 2)").hypgen_rel_weight == 2.0


def test_float_flag_accepts_quoted_string():
    assert _cfg('(config :hypgen-rel-weight "1.5")').hypgen_rel_weight == 1.5


# ── Error paths are preserved ──────────────────────────────────────


def test_unknown_flag_raises():
    with pytest.raises(ValueError, match="unknown config flag :nope"):
        _cfg("(config :nope 1)")


def test_int_flag_rejects_non_integer():
    with pytest.raises(ValueError, match="expects an integer"):
        _cfg("(config :candidate-order-seed foo)")


def test_bool_flag_rejects_non_boolean():
    with pytest.raises(ValueError, match="expects true/false"):
        _cfg("(config :print-alive 7)")


def test_unsupported_type_message_names_the_flag():
    # A synthetic field whose annotation is outside {bool,int,float,str}.
    from dataclasses import dataclass, fields

    from ein.inference.config import _coerce

    @dataclass
    class _Fake:
        weird_flag: "complex" = 0j

    (fld,) = fields(_Fake)
    with pytest.raises(ValueError, match=r"weird-flag.*unsupported type"):
        _coerce(fld, 1)
