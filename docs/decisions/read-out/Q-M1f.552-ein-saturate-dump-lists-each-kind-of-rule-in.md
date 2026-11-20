# Q-M1f.552 — `ein saturate --dump` lists each kind of rule in a section of its own

The saturated dump lists every rule the program loads, each in the section of its kind: `;; Rules — the saturation
agenda's (N)` for the rules saturation fires, a rule that states `:ein inconsistent` among them, `;; Obligation
rules (N)` and `;; Hypothesis rules (N)` for the two kinds no agenda holds. A rule stands in one section and no rule
in none. Beside an obligation rule and a hypothesis rule with parameters the line says how many activators bind
them, and `no activator` for one the program loads and does not activate, so that a module imported for another of
its names does not read as active.

**Why.** A dump that listed the agenda's rules alone left the rules that judge a state and those that propose
candidates in no list, though the program loads them and the answer reads them; and a module brings its rules whole,
so a rule listed with no word of its activation would read as one the program uses.

**Held by.** `ein-cli/tests/saturate_dump_rules.rs`: `an_obligation_rule_is_listed_in_a_section_of_its_own`,
`an_obligation_rule_nothing_activates_says_so`, `a_hypothesis_rule_is_listed_with_its_activators`,
`every_rule_stands_in_one_section`. The page is [the CLI reference](../../ein/user/04-reference/05-cli.md).
