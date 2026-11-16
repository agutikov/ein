# Q-M1f.184 — `std.math.rat`'s harness exhausts 28 fractions, in programs of bounded size

`std.math.rat` is held to exact fractions on a small exhausting set: the 28 fractions with a numerator from −3 to 3
over a denominator from 1 to 4, every pair under each of the five operations, asked 250 requests to a saturation;
and, past it, a sample of longer fractions in every combination of signs. Its cost is held in batches of 100, 400 and
1 200 requests, where the work per request at a larger batch may exceed the smallest batch's by 15 %.

**Why.** A stored fact of a rational request costs memory, so a program of thousands of such requests is large;
bounding a saturation's requests keeps every program of the harness small. A batch's claim is a proportion: a rule
that joined two facts on a number would multiply the work per request by the batches' ratio, 4 and 12, which 15 %
still catches, while the length of a fraction's cross products varies from request to request more than a sum's
digits do.

**Held by.** `ein-infer/tests/rat_theory.rs`: `every_pair_of_small_fractions_is_right`, `longer_fractions_are_right`,
its constants `NUM`, `DEN` and `CHUNK`; `ein-infer/tests/arith_batch.rs`:
`a_batch_of_rational_requests_costs_in_proportion_to_its_count`, `SIZES_RAT` and `TOLERANCE_RAT`.
