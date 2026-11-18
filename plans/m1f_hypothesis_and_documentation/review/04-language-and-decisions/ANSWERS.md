# Your answers — The language, and three decisions' sentences

[← the topic](README.md) · [← the bundle](../README.md)

One entry an item, in the topic's order: its options as the item's page puts them, the default that stands
until you answer, and a blank. Write your answer after **Your answer:** — a letter, *default*, or your own
words — and anything else under **Notes**. An item left blank keeps its default. This file is yours: a
re-render of the bundle rewrites the READMEs and never this file.

## 08 · V-7 — a rule can conclude a fact of a relation the program does not count

[The item](08-V-7/README.md)

**Options:**

- **(a)** the registry takes a relation an activator names for a parameter that heads a conclusion (and decide
  whether a constant `:assert` head joins it too); moves R where the blind rung guessed about the name.
- **(b)** the loader refuses such an activator; the author declares the relation, which moves R where the blind rung
  then guesses facts of it.
- **(c)** a page says such a relation is declared; nothing moves, and the four lists keep disagreeing.
- **Default**: none taken; the ledger reads the user's third instruction (*any head is a relation*) as pointing at (a).

**Your answer:** What does mean "program does not count relation" ? relation is almost any atom if it is 1st in the tuple. Also for nested facts. Its enough to be the first item in the tuple to became a relation. (XXX) - is a relation fact with relation 'XXX'. Relation declaration `(relation = Formula Formula)` is optional fact that can be used to handle some random labels associated with the relation, even maybe not arguments types, not arguments count. It is just by convention is "relation declaration", that is used to reason about arguments domains. You easily can use `(rel-arg-types <relation_name> Type Type)`, `(rel-arg-types <relation_name> (list Type Type))`, `(defrel <relation_name> Type Type)` WHATEVER for the same purpose. But `relation` is already used by SOME OF stdlib modules. That is it.

**Notes:** 

---

## 09 · F21 N-15 — *everybody's thoughts are false*, with `absent`, comes out dead

[The item](09-F21-N-15/README.md)

**Options:**

- **(1) twin** of `W8/fifo-gap-root`, `x2b.ein` a `[[case.twin]]` with D2's stratum-order reading as `right` ({∅}).
- **(2) a row of its own**: the same reading, filed apart because Q-M1f.313's first half, not FIFO, picks the rule,
  and the symptom is an empty R.
- **(3) not a row**: the row's reading does not stand, and Q-M1f.313's scope note covers it.
- **Default**: none taken.

**Your answer:** This is non-monotone saturation. false inferred from absent. See followups where it belongs.
Btw, into followup: what about optimistic saturation - at first step skip non-monotone rules and get a fixedpoint without them, and then check. But again, this belongs to non-monotone programs followup. Not decide and not investigate here.
Does ein detect non-monotone program here??? We already decided to detect those cases.

**Notes:** 

---

## 16 · Q-M1f.412 — the read-out's `is-a` row is dropped: does a schema id move?

[The item](16-Q-M1f.412/README.md)

**Options:**

- **(a)** no id moves for the dropped key: `ein-summary/2`, `ein-region/1` kept, the report `ein-test-report/6` for
  the diff alone — **default**, built.
- **(b)** the report's id moves for it too, `ein-test-report/7`; the summary and the region dump keep theirs.
- **(c)** every id moves: `ein-summary/3`, `ein-region/2`, `ein-test-report/7`.

**Your answer:** How deep does this `is-a` go into the kernel? I repeat again and again THERE IS NO TYPES in ein. It's substrate, field, data plane is a pure labelled graph. Does graph nodes have types? NO! Graphs are about structure. If we want - we add types-like behaviour to the program on relations and rules level, usually (Usually! not always) following stdlib convention. If we don't need - no types.

**Notes:** 

---

## 22 · L-92 — Q-M1f.362's sentence on the fact index is behind the engine's pages

[The item](22-L-92/README.md)

**Options:**

- **(a)** re-word Q-M1f.362's *Why* to *"keys a name, and the names one level inside a nested fact, never a fact
  itself"* (the text in § 3).
- **(b)** keep it: *"does not key on a nested fact"* read as *no key's value is a fact*.
- **Default**: none taken.

**Your answer:** (a)

**Notes:** 

---

## 23 · L-93 — two stale sentences of the pages

[The item](23-L-93/README.md)

**Options:**

- **Stdlib page**: (a) link Q-M1f.185 with the line quoted, or (b) a § 1.7 on the diagnostics page and the link aimed
  at it; and the third sentence re-worded (§ 3.1) either way.
- **Reserved-strings page**: the row re-worded (§ 3.2), or the row and the constant dropped.
- Or neither, left as they are.
- **Default**: none taken.

**Your answer:** both, make docs consistent with the code and clear

**Notes:** 

---

## 24 · L-94 — Q-M1f.119's list of the ways a relation is the program's misses one

[The item](24-L-94/README.md)

**Options:**

- **L-94**: (a) re-word Q-M1f.119 to add *"writes as the relation of a fact that is an argument"* (§ 3.1); (b) keep it.
  Either way its `is-a` clause is stale since Q-M1f.436, as is `02-syntax.md`'s (§ 7).
- **D-23** (F23's, shown here): the line re-worded (§ 3.2), or as it is.
- The pages' missing activator clause (§ 3.3), with either.
- **Default**: none taken.

**Your answer:** (a)

**Notes:** 
