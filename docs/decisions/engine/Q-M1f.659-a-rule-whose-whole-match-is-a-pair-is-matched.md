# Q-M1f.659 — A rule whose whole match is a pair is matched by one probe, never seeded

A plan whose match is one disjunct of two premises and nothing else, no `(absent …)` among them, a fact `F` of
distinct variables and a fact `(W F)` holding it as its one argument, and whose firing ends a state, by stating
`:ein inconsistent` or by concluding what a rule that states it matches alone, `(false)` among them, is never seeded.
A new fact of `F`'s relation and count asks whether the state holds `(W F)`, by the fact store's own probe of that
proposition, which builds nothing, and then by `Kb::contains`; a new `(W G)` asks whether the state holds `G`. Only a
hit builds the match, the one a seeded match of the same fact builds, the same registers, bindings and premises,
queued at the place that one would take. `std.contradiction`'s `clash-N` is such a plan, and so is a program's own
rule of that shape, whichever way it ends the state. `W` is the relation the rule's shape names: no set of the facts
a state holds under `not`, or under any other name, is kept beside the fact store.

Every other plan is seeded: a rule that states `:ein inconsistent` over anything but a pair, `std.core`'s among
them, and a plan in which the pair is one of several disjuncts.

**Why.** The match of such a rule is the pair or nothing, so one lookup decides it where a seeded match walks the
held extent of `W` or of `F`'s relation, and the contradiction's rules are matched at every state of a search. The
fact store's probe asks for the proposition the rule's shape names, so the kernel reads no theory's name
([Q-M1f.436](Q-M1f.436-the-kernel-reads-no-theorys-name.md)); a set kept of the negations alone would read one. The
match a probe builds is the seeded one, queued in its place, so no firing moves and the agenda reads the pair as
before ([Q-M1f.313](Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)).

**Held by.** `ein_infer::contradiction::PairShape` and `PairShape::each_match`, read off the compiled plans by
`ein_infer::contradiction::Pairs`; `ein_infer::match_::Matcher::run_pair`; the saturator's `pair_match`, which a
build with debug assertions, the gate's, holds to a seeded match of the same fact at every probe; `contradiction.rs`'
unit tests `a_pair_alone_has_a_shape_and_its_held_step`, `a_pair_is_found_from_either_side_by_a_probe` and
`a_wrapper_of_its_own_relation_is_probed_at_both_steps`. The design pages:
[`06-saturation.md` § 3](../../ein/ein.rs/03-design/06-saturation.md#3-the-queue-the-delta-and-the-parked-set) and
[`05-matcher.md` § 6](../../ein/ein.rs/03-design/05-matcher.md#6-which-optimisations-are-legal).
