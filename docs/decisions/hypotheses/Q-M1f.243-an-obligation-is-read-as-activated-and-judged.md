# Q-M1f.243 — An obligation is read as activated, and judged by the open facts root holds

Wherever Ein reads which obligations a program has, an obligation is a plan the state's activators compile, and one
with no parameters always: activated whether or not the state owes anything under it. Activated at root, it keeps
the blind enumerator off ([Q-M1f.576](Q-M1f.576-the-blind-enumerator-writes-only-where-root.md)). The lists
and a closure are judged against the `open` facts root holds
([Q-M1f.578](Q-M1f.578-the-two-lists-and-a-closure-configure-the-blind.md)): an obligation that owes nothing at
root writes none, and disagrees with nothing.

**Why.** A declaration says what the program means, and an `open` fact what a state owes or supposes; the lists
configure a generator that does not run beside them, so it is what they say of root's `open` facts that can be
false.

**Held by.** `ein_infer::mechanisms::check` and `obligation_plans`; `ein-cli/tests/mechanism_conflicts.rs`.
