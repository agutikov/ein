# Q-M1f.340 — Every name is a JSON string

Where Ein's JSON writes a name, it writes a JSON string, whatever its characters: a goal binding of `007` is
`"007"`, and of `7` is `"7"`.

**Why.** A run of digits is a name ([Q-M1e2.24](../language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)).
Written as JSON numbers, `007` and `7`, two names, would read back as one.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_goal_binding_of_digits_is_a_json_string`, the bindings `8`, `-7`
and `99999999999999999999999`, wider than any integer type, each written as a string.
