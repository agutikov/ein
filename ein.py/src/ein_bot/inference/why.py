"""``:why`` template substitution — S1.3.1 T1.3.1.6.

Q31 (resolved 2026-05-20) picked ``{?var}`` notation — the ``?`` is
part of the reference, identical to the var name as it appears in
``:match`` / ``:assert``. ``{x}`` *without* a leading ``?`` is
treated as a literal (left in the output unchanged).

Example:

    >>> render_why("{?rel} is transitive: {?a} →{?rel}→ {?b}",
    ...            {"rel": "co-located", "a": "Norwegian", "b": "House_1"})
    'co-located is transitive: Norwegian →co-located→ House_1'

Unbound vars leave their reference text in place. A future
strictness flag could promote this to an error, but the trace
renderer (P1.6) prefers a partial-render over a hard fail.
"""
from __future__ import annotations

import re
from typing import Any

# Match `{?<var-name>}` where var-name follows the IR's VAR regex
# (an identifier with letters / digits / `_` / `-`, leading letter).
_TEMPLATE_REF = re.compile(r"\{\?([A-Za-z][A-Za-z0-9_-]*)\}")


def render_why(template: str, bindings: dict[str, Any]) -> str:
    """Substitute ``{?var}`` references against `bindings`.

    Unbound vars are left as literal ``{?name}``. Bare ``{x}`` (no
    leading ``?``) is also left as-is — a deliberate non-substitution
    so authors can include literal braces in trace messages.
    """
    def _sub(m: re.Match[str]) -> str:
        name = m.group(1)
        if name in bindings:
            return str(bindings[name])
        return m.group(0)

    return _TEMPLATE_REF.sub(_sub, template)


__all__ = ["render_why"]
