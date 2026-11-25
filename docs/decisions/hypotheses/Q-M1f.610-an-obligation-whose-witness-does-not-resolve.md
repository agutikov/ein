# Q-M1f.610 — An obligation whose witness does not resolve under its activator proposes nothing, and nothing proposes in its place

An obligation's payments are read off its guard with the witness step skipped
([Q-M1d.2](Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md)): the one step of the guard over the relation it
owes that binds a variable of the guard's own. The loader refuses an obligation whose witness step cannot be found for
any activator; what it cannot refuse depends on the activator, as two parameters bound to one relation, which leaves
two such steps where the text had one. Where an activated obligation's witness step does not resolve so, **the
obligations' call is declined**: no `(open F)` is written for what would pay any obligation activated at the state, and
the blind enumerator does not run in its place, since an obligation is activated
([Q-M1f.576](Q-M1f.576-the-blind-enumerator-writes-only-where-root.md)). What the state owes is counted and written,
`(open R)`, as at any state, and a rule that concludes a hypothesis writes as it does beside any obligation. The
`rung` event of `--events` says `declined`, with the reason.

**Why.** An obligation's payments are a branch that loses no model only while they are read off one witness: from
two, a choice between them would be a guess presented as every way to pay. Proposing nothing says that no payment was
read, and the blind enumerator, which a program that states an obligation is not second-guessed by, is no substitute.

**Held by.** `ein_infer::oblgen`: `project`, which finds the witness step, and `generate` and `payments`, which
decline the call on one that does not resolve (`Mode::Declined`); `ein_infer::hypgen`, which returns after the
obligations' call, whatever its mode; the `rung` event's modes in [the events protocol](../../ein/ein.rs/05-events.md).
