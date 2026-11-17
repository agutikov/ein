# Q-M1f.394, with examples — the words for a fact under an arrow

**Phase:** [P1f.22](README.md). Written 2026-11-17 on the user's word (*"Q-M1f.394 explain with examples in
separate file"*). The question is
[Q-M1f.394](../open_questions.md#q-m1f394--the-words-for-a-fact-under-an-arrow); it is **not ruled**. It is three
small questions about vocabulary, and nothing in the engine depends on any of them: they decide which words
S1f.22.2 writes on the pages.

One program is used throughout:

```lisp
(think Alice (= 5 (+ 2 2)))     ; t
(says  Bob   (= 5 (+ 2 2)))     ; b
(= 4 (+ 2 2))                   ; h
(not (= 5 (+ 2 2)))             ; n
```

with the facts named

| name | the fact | does the knowledge base hold it? | which held facts point at it? |
|---|---|---|---|
| $t$ | `(think Alice (= 5 (+ 2 2)))` | yes | none |
| $b$ | `(says Bob (= 5 (+ 2 2)))` | yes | none |
| $h$ | `(= 4 (+ 2 2))` | yes | none |
| $n$ | `(not (= 5 (+ 2 2)))` | yes | none |
| $e$ | `(= 5 (+ 2 2))` | **no** | $t$, $b$, and $n$ |
| $s$ | `(+ 2 2)` | **no** | $h$ directly; $t$, $b$, $n$ through $e$ |

Add the line `(= 5 (+ 2 2))` and $e$ becomes held *and* pointed at; that is the third case.

---

## Question 1 — what the three cases are called

The same fact can be (i) held and under no arrow, (ii) under an arrow and not held, (iii) both. The pages need
words for these that a reader keeps apart. Your own words in the instruction were *"in KB"* and *"in another fact
context"*, and in the last message *"global context"* and *"local/fact context"*.

**Option (a): two independent properties, *held* and *in a context*.** A fact is **held** when the knowledge base
holds it. The **context** of a fact is the set of held facts that point at it, directly or through other facts:

$$
\operatorname{ctx}_K(g) = \{\, f \in K : \text{a path of argument arrows leads from } f \text{ to } g \,\}
$$

On the example:

| fact | held? | its context | so it is |
|---|---|---|---|
| $h$ = `(= 4 (+ 2 2))` | yes | $\emptyset$ | **held, in no context** |
| $e$ = `(= 5 (+ 2 2))` | no | $\lbrace t, b, n \rbrace$ | **in a context, not held** |
| $s$ = `(+ 2 2)` | no | $\lbrace h, t, b, n \rbrace$ | **in a context, not held** |
| $e$, after adding the line `(= 5 (+ 2 2))` | yes | $\lbrace t, b, n \rbrace$ | **held, and in a context** |

Sentences the pages would then say: *"`(= 5 (+ 2 2))` is in the context of `(think Alice …)` and is not held."*
*"A premise matches a held fact; a pattern under an arrow of a premise matches a fact in that premise's context."*
*"A rule that packs gives a held fact a context; a rule that unpacks holds a fact that was only in one."*

With your newer words this reads: **held = in the global context; in a context = in a fact's (local) context.** The
two can be used side by side: *global context* for $K$, *the context of a fact* for $\operatorname{ctx}_K$.

**Option (b): the words the knowledge-base page has today, *held* and *only pointed at*.**

| fact | so it is |
|---|---|
| $h$ | held |
| $e$, $s$ | only pointed at |
| $e$ after adding the line | held (and the page has no word for *and also pointed at*) |

It is shorter and adds no new term, but the case your instruction is most about, *the same fact both in the KB and
in a fact's context*, has no name, and *pointed at by what* cannot be said without a second word anyway.

**Recommended: (a)**, with *global context* as a second name for $K$ where the contrast with a fact's context is
the point.

---

## Question 2 — is the fact a negation denies *in the context* of that negation?

$n$ = `(not (= 5 (+ 2 2)))` is a held fact whose one argument arrow points at $e$. So, by the definition above,
$n \in \operatorname{ctx}_K(e)$: the negation is one more fact that speaks about $e$, as `think` and `says` do.

The reason to ask is that the pages today say two things about this arrow, and both are true of the engine:

- `06-constraints.md`, the knowledge-base page: *"`(not P)` is a fact whose one argument is the fact it denies"*.
  The arrow is there.
- `02-syntax.md`, `07-diagnostics.md`: *"a stored negation's denied fact is no term"*. That is about `written`:
  the loader states `written` for what is nested in a written fact, and **skips the fact a negation denies**
  (it states `written` for what is nested inside *that*):

  ```text
  (not (think Alice (= 5 (+ 2 2))))     →   (written (= 5 (+ 2 2)))   (written (+ 2 2))
                                            and no (written (think Alice …))
  ```

  The knowledge-base page writes this as $\operatorname{sub}(\lnot f) = \operatorname{sub}(f)$, *"the one exception
  to the arrows"*.

**Option (a): yes, a negation is a context.** $\operatorname{ctx}$ follows every argument arrow, the negation's
too; $\operatorname{sub}$ (which is only what `written` reads) keeps its exception. Then one sentence explains the
whole of how `not` differs from `think`:

> *Every held fact that points at $g$ is a context of $g$. One relation's facts are read as saying something of
> whether $g$ is held: $\mathsf{not}$. $\lnot g \in K$ makes $g$ negated, and makes $K$ dead if $g$ is held too.
> Every other context is opaque: what `think` means is what a program's rules make of it.*

and the example is drawn as it is: $t$, $b$ and $n$ all point at $e$; $e$ is not held; the knowledge base is alive.
Alice thinks it, Bob says it, and it is denied; no contradiction.

**Option (b): no, a negation is not a context.** A context is defined through $\operatorname{sub}$, so the denied
fact is left out. Then `(not (= 5 (+ 2 2)))` gives $e$ no context, and $e$ in the program `(not (= 5 (+ 2 2)))`
alone would be *in the graph, not held, and in no context*: a fourth case, which needs its own name and its own
row in every table.

**Recommended: (a).** It keeps three cases, and it is what makes *a context gives a fact no truth* a statement
with exactly one named exception.

---

## Question 3 — *value*, *term*, *object*, *fact*: one thing under four names

The reasoning's knowledge-base page (ruled the day this phase opened) says: an **object** is an **atom** or a
**fact**; an arrow points at an object. The other pages were written before it, and call the fact under an arrow
by other names. Each line is a quotation from a page as it stands:

| the page | what it calls `(= 5 (+ 2 2))` inside `(think Alice …)` |
|---|---|
| `reasoning/01-static-ir/01-knowledge-base.md` | *"a fact … nested in it"*, *"only pointed at"*; an **object** |
| `user/03-syntax-and-semantics/03-facts.md` | *"a **value**, not a fact of the knowledge base"* |
| `user/04-reference/02-syntax.md`, `01-objects.md`, the calculator's pages | a **term**, *"a ground term over the relations"* |
| `user/03-syntax-and-semantics/04-ontology.md`, `ein.rs/03-design/03-data-model.md` | a **relational node** |
| `ein.rs/03-design/05-matcher.md` | a *fact-valued argument* |

And *value* means three more things elsewhere: the number a numeral stands for (`10-theory.md`), a name
(`08-natural-numbers.md`), a program's text (`13-glossary.md`).

The sentence that matters most is `03-facts.md`'s: *"`(knows Carol Dan)` inside the `claims` fact is a **value**,
not a fact of the knowledge base."* It is trying to say *not held*, and says instead that the thing **is not a
fact**. By the knowledge-base page it is a fact, the same node it would be if stated; it is just not held. A
reader who learns *value ≠ fact* from this page cannot then make sense of your sentence *"the same fact can be in
KB and in another fact context"*.

**Option (a): one word, *fact*, everywhere; *held* / *not held* carries the difference.**

| today | under (a) |
|---|---|
| *"`(knows Carol Dan)` inside the `claims` fact is a value, not a fact of the knowledge base: Carol claims it, and nothing says it holds"* | *"`(knows Carol Dan)` inside the `claims` fact is a fact the knowledge base does not hold: Carol claims it, and nothing says it holds"* |
| *"A fact held as a value is a node of its own"* | *"A fact under an arrow of another is a node of its own, held or not"* |
| *"a value is a name, or a fact a state could hold"* (Q-M1f.113, and the diagnostics that quote it) | **stays**: here *value* means *what may stand under an argument arrow*, i.e. an object, and it is the engine's printed text |
| *"a numeral is a term"*, *"a malformed term"* (the calculator's pages) | **stays**, defined once in the glossary: *a **term** is a fact written to stand for a thing, not to be held: a numeral, a tag* |

So *value* survives only where a decision or a diagnostic's message already uses it, read as *object*; *term*
survives in the standard library's pages for one use (`(+ 2 2)` in the example is a term in that sense: nobody
means to state that two plus two); *relational node* and *fact-valued argument* go.

**Option (b): two vocabularies, joined once.** The user's pages keep *value* for anything under an argument arrow,
the reasoning's pages say *object*, and each perspective says once that the two are one thing. `03-facts.md`'s
sentence is still re-worded, since *"not a fact"* is wrong under either option, but *value* stays the user-facing
word.

- For (b): *value* is the word the diagnostics print, so a user meets it anyway; and it is the code's word
  (`ein_core::Value`).
- Against (b): two words for one thing is what this phase was opened to remove, and *value* already has three
  other meanings on the user's pages.

**Recommended: (a).** What it costs is a pass over `03-facts.md`, `01-objects.md`, `05-rules.md`, `04-ontology.md`
and the glossary, which S1f.22.2 makes as it writes there; no message of the engine changes.

---

## The three answers on one line

| | question | recommended |
|---|---|---|
| 1 | the cases' names | **(a)** *held* and *in a context*; $K$ is also *the global context* |
| 2 | a negation's denied fact | **(a)** is in the negation's context; `not` is the one context the kernel reads |
| 3 | *value* / *term* | **(a)** *fact* everywhere; *value* only where a decision or a message has it; *term* for a fact written to stand for a thing |
