"""``:why`` template substitution — S1.3.1 T1.3.1.6.

Q31 (resolved 2026-05-20) picked ``{?var}`` notation — the ``?`` is
part of the reference, identical to the var name as it appears in
``:match`` / ``:assert``. ``{x}`` *without* a leading ``?`` is
treated as a literal (left in the output unchanged).

Example:

    >>> render_why("{?rel} is transitive: {?a} →{?rel}→ {?b}",
    ...            {"rel": "co-located", "a": "Norwegian", "b": "House-1"})
    'co-located is transitive: Norwegian →co-located→ House-1'

The same machinery renders ``(relation …)`` ``:why`` templates, which
reference argument *slots positionally* — ``{?1}`` is the first arg,
``{?2}`` the second — so a leading digit is admitted in a reference
(the rule ``:why`` case never uses one; bound var names always start
with a letter):

    >>> render_why("{?1} is drunk in {?2}", {"1": "Water", "2": "House-1"})
    'Water is drunk in House-1'

Unbound vars leave their reference text in place. A future
strictness flag could promote this to an error, but the trace
renderer (P1.6) prefers a partial-render over a hard fail.
"""
from __future__ import annotations

import re
from typing import Any

# Match `{?<ref>}`. A ref is a rule/goal var name (letter-led, per the IR's
# VAR regex) OR a positional slot index (`{?1}`, `{?2}` …) used by
# `(relation …)` `:why` templates — hence the leading char admits a digit.
_TEMPLATE_REF = re.compile(r"\{\?([A-Za-z0-9][A-Za-z0-9_-]*)\}")


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
