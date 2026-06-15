#!/usr/bin/env python3
"""What the `std.algebra` relation-algebra library (P1.8 S1.8.A12) can express.

Three worked examples, each saturated through the real engine so the printed
derivations are actual engine output, not assertions:

1. KINSHIP — composition + converse generate a whole relationship vocabulary
   (grandparent / child / sibling) from `parent` alone. The De Morgan point:
   term logic cannot compose relations; relation algebra can.

2. A RELATION-ALGEBRA LEMMA verified on a model — composition is monotone, so
   `parent ⊆ ancestor` + `ancestor` transitive ⟹ `parent;parent ⊆ ancestor`.
   ein is a *representable* RA: it checks equational/inclusion consequences by
   computing both sides over a concrete model.

3. SCHRÖDER negative propagation solves a constraint — a missing composite edge
   forces a missing factor edge (`¬(R;S)(a,c) ∧ R(a,b) ⟹ ¬S(b,c)`). This is the
   generic form of the Zebra engine's `adjacent-via` elimination: the algebra
   does not merely *query* relations, it *propagates* constraints, negatives
   included.

Run from the package root:  python demo/relation_algebra_examples.py
"""
from __future__ import annotations

from ein.inference.saturator import Saturator
from ein.ir import parse
from ein.kb.store import KnowledgeBase


def saturate(src: str, max_steps: int = 4000):
    """Saturate `src`, return the list of non-redundant firings."""
    kb = KnowledgeBase.from_ir(parse(src))
    return [f for f in Saturator(kb).saturate(max_steps=max_steps)
            if not f.redundant]


def show_positive(firings, rel: str) -> None:
    """Print every derived `(rel a b)` edge, sorted."""
    edges = sorted({f.derived.args for f in firings
                    if f.derived.relation_name == rel})
    for args in edges:
        print(f"    ({rel} {' '.join(args)})")


def _fact_str(fact) -> str:
    """Render a Fact / nested octagon as ein surface syntax."""
    parts = []
    for a in fact.args:
        parts.append(_fact_str(a) if hasattr(a, "relation_name") else str(a))
    return f"({fact.relation_name} {' '.join(parts)})"


def show_negative(firings) -> None:
    """Print every derived `(not (rel a b))` octagon with its provenance."""
    for f in firings:
        if f.derived.relation_name == "not":
            premises = ", ".join(_fact_str(p) for p in f.premises)
            print(f"    {_fact_str(f.derived)}")
            print(f"        ⟸ rule `{f.rule}`  from  {premises}")


def banner(n: int, title: str) -> None:
    print(f"\n{'═' * 72}\n  Example {n} — {title}\n{'═' * 72}")


# ── 1. Kinship: compose + converse build a vocabulary from `parent` ──

banner(1, "kinship — composition + converse generate relationships")

KINSHIP = """
(import std.algebra :symbols (compose converse))

; data — a fragment of a family tree (who-is-parent-of-whom)
(parent Alice Bob   :source "(1)")
(parent Bob   Carol :source "(2)")
(parent Bob   Dave  :source "(3)")

; derived relations, defined purely algebraically
(compose parent parent grandparent)      ; grandparent := parent ; parent
(converse parent child)                  ; child       := parent°
(compose child parent sibling-or-self)   ; sib-or-self := parent° ; parent
"""
fk = saturate(KINSHIP)
print("\n  grandparent := parent ; parent")
show_positive(fk, "grandparent")
print("\n  child := parent°  (converse)")
show_positive(fk, "child")
print("\n  sibling-or-self := parent° ; parent  (share a parent, incl. self)")
show_positive(fk, "sibling-or-self")
print("\n  → De Morgan (1860): syllogistic logic cannot derive these — they")
print("    require *composing* relations, which is exactly `compose`.")


# ── 2. A representable-RA lemma: composition is monotone ─────────────

banner(2, "lemma on a model — parent;parent ⊆ ancestor")

MONOTONE = """
(import std.algebra :symbols (compose imply2-fwd))

(parent Alice Bob   :source "(1)")
(parent Bob   Carol :source "(2)")

(imply2-fwd parent ancestor)             ; axiom:  parent ⊆ ancestor
(compose ancestor ancestor ancestor)     ; axiom:  ancestor transitive (A;A ⊆ A)
(compose parent parent grandparent)      ; def:    grandparent := parent ; parent
"""
fm = saturate(MONOTONE)
gp = {f.derived.args for f in fm if f.derived.relation_name == "grandparent"}
anc = {f.derived.args for f in fm if f.derived.relation_name == "ancestor"}
print("\n  grandparent edges:", sorted(gp))
print("  ancestor    edges:", sorted(anc))
holds = gp <= anc
print(f"\n  grandparent ⊆ ancestor  ?  {holds}")
print("    Each grandparent (= parent;parent) edge is also an ancestor edge —")
print("    the monotonicity-of-composition lemma, witnessed on this model.")
print("    (ein decides RA consequences on a model; it is a representable RA.)")


# ── 3. Schröder propagation: a missing composite forces a missing factor ──

banner(3, "constraint solving — Schröder negative propagation")

SCHRODER = """
(import std.algebra :symbols (compose-negative-s))

; `two-ahead := succ ; succ`, treated as CLOSED (composition is its only source)
(compose-negative-s succ succ two-ahead)

(succ p1 p2 :source "(given: p2 immediately follows p1)")
(not (two-ahead p1 p3) :source "(given: p3 is NOT two ahead of p1)")
"""
fs = saturate(SCHRODER)
print("\n  given:  (succ p1 p2)   and   ¬(two-ahead p1 p3)")
print("  Schröder (¬(R;S)(a,c) ∧ R(a,b) ⟹ ¬S(b,c)) derives:\n")
show_negative(fs)
print("\n  → p3 cannot immediately follow p2. A genuine *deduction*, not a query —")
print("    the generic form of the Zebra engine's `adjacent-via` elimination.")

print("\n" + "═" * 72)
print("  Expressivity ceiling: every rule above is ≤3 logical variables")
print("  (RA ≡ FOL³). Constraints that irreducibly coordinate ≥4 positions —")
print("  the puzzle's `adjacent-via` (value→house→house→value) — are NOT binary")
print("  RA terms and correctly stay as named multi-param rules.")
print("═" * 72)
