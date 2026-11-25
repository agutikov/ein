# Q-M1f.644 — A debug build checks the compile walk's shortcut by the rules' facts counted by relation

`Engine::compile_all` skips its walk of every rule against its activators where the count of rule-application facts
has not moved since the last walk. A build with debug assertions checks each skip, by a count apart from the one the
skip reads: the facts of the rules' relations, one `Kb::n_facts_of` probe a rule. Where that count is the one the
last walk read, the walk would read the same activators and compile nothing new, since a knowledge base only grows
and a fork holds its parent's facts; only where it moved is the full walk made, and the assertion fails if that walk
would add a plan. No skip the full walk would refuse passes, as long as `Kb::n_facts_of`'s counts and the
rule-application index agree, which `Kb::index_fact` keeps together, writing both for every fact.

**Why.** The full walk at every skip made the build the tests run in pay a walk of every rule at every step of every
saturation. Counted by relation, the check stays independent of the counter it checks, and costs a probe a rule.

**Held by.** `ein_infer::Engine::compile_all`, its `would_not_grow` over `rule_facts`; `ein_core::Kb::index_fact`;
`ein_core::Kb::check_extent_counts`, which holds the counts by relation to a walk of the layers; the `gate` profile of
`ein.rs/Cargo.toml`, the profile the tests are built in, which keeps debug assertions on.
