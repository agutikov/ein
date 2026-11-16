# Q-M1f.327 — Identity and absorbing elements are derived one-sided too

Beside `identity-of` and `absorbing-of`, which hold on both sides, `std.bool.props` derives `left-identity-of`,
`right-identity-of`, `left-absorbing-of` and `right-absorbing-of`: `Andn`, *a and not b*, has a right identity, 0,
and no left one.

**Why.** Several of the sixteen have an identity or an absorbing element on one side only, of which a two-sided
fact says nothing.

**Held by.** `ein-infer/tests/bool_theory.rs`: `the_one_sided_forms_are_derived`;
`tests/stdlib/bool/props/05_identity_and_absorbing.ein`.
