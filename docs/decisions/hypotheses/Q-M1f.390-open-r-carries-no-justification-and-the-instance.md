# Q-M1f.390 — `(open R)` carries no justification, and the instance is the engine's record

What an obligation writes is `(open R)`, the relation it owes and nothing more, and a hypothesis is `(open F)`, the
fact supposed and nothing more: no fact of `open` names the rule, the activator or the bindings it came from. Two
instances over one relation write one fact, held once by the state that owes them
([Q-M1f.566](Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md)), and the count of instances is kept beside
it, as before. Where each instance was written, its rule, activator, relation, bindings and `:why`, is the engine's
own record, which every report of a state that owes prints and no rule reads. `absent` reads a fact of `open` as it
reads any other.

**Why.** Inside Ein there is one thing to say, that a relation is owed or that a fact may be supposed; the engine
sees where each was written anyway and can record what it wants of it. A justification in the fact would put the
engine's bookkeeping into the knowledge base, and make two instances over one relation two facts where the program
said one thing.

**Held by.** `ein_infer::open`, which writes `(open R)` once a relation;
`ein_infer::obligations`, whose `owe` event carries each instance
([`05-events.md`](../../ein/ein.rs/05-events.md)); `ein-infer/tests/open_facts.rs`:
`the_seats_hold_what_they_owe_where_they_owe_it`, root holding `(open seats)` once for its four instances; the
read-out's `owed` rows ([`08-read-out.md`](../../ein/user/04-reference/08-read-out.md)).
