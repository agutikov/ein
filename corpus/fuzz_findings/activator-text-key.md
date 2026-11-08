# activator-text-key

**Fixed 2026-11-04** by M1e2
[S1e2.2.16](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.16_activator_text_key.md)
(T1e2.2.16.2): the compile key is the activator's values, not their text, so
the int and the string key two plans, and both files derive the four `noted`
facts in a release build and a debug one. Promoted the same day to
[`examples/ein-bugs/activator-text-int-and-string.ein`](../../examples/ein-bugs/activator-text-int-and-string.ein)
and [`activator-text-string-then-int.ein`](../../examples/ein-bugs/activator-text-string-then-int.ein),
beside the single-activator controls; the two files here stay as frontend
fuzzer seeds, their text unchanged. The note below is the finding as it
stood.

**Unwritable since M1f S1f.5.5's swap (T1f.5.5.4, 2026-11-08).** A run of
digits is a name (S1f.5.4's ruling) and a string is natural-language text,
written only after `:why`, `:source` or `:goal-text` (S1f.5.2's), so an int
and a string of one text no longer exist, and `(holds "1")` is refused at the
parse. The two seeds here stay seeds whose recorded outcome is that refusal
(`ein-ir/tests/golden/fuzz_seeds.txt`); the two witnesses stay the refusal's
witnesses, their rows restated to it; and the two single-activator controls,
`activator-text-int-only.ein` and `activator-text-string-only.ein`, retired,
their case being unwritable (the user's ruling of 2026-11-08 on S1f.5.5's
review stop, item 2).

- found: 2026-11-04, by hand — M1e2
  [S1e2.2.7](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.7_binding_key_layout.md)
  (T1e2.2.7.1), reading the compile key for W7, and parked in its § Notes;
  filed as W16 by
  [S1e2.2.16](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.16_activator_text_key.md)
- property: **no-crash** in a debug build; a **lost derivation** in release
- banked here: 2026-11-04, M1e2 S1e2.2.16 (T1e2.2.16.1), as the witnesses of
  register rows `W16/int-and-string` and `W16/string-then-int`
  ([`corpus/wrong_answers.toml`](../wrong_answers.toml)):
  [`activator-text-key.ein`](activator-text-key.ein) and
  [`activator-text-key-swapped.ein`](activator-text-key-swapped.ein), the
  two activator lines in the two orders

```
(rule note (?R ?f)
  :match  (and (?R ?a ?b) (holds ?f))
  :assert (noted ?a ?f))
(edge 1 2) (edge 2 3)
(holds 1)  (holds "1")
(note edge 1)
(note edge "1")
```

## What happens

The loader holds `1` and `"1"` as two values, an `Int` and a `Sym` whose text
is `1`, so the KB has two `holds` facts, and each activator alone derives its
own two `noted` facts (the controls `examples/ein-bugs/activator-text-int-only.ein`
and `activator-text-string-only.ein`, retired at S1f.5.5's swap). `compile::plan_key` keyed a plan by its
activator's arguments **as text**: a symbol is its own text, and anything else
was rendered and the text interned. So both activators gave one key, and the
second compiled no plan:

```
$ ein saturate --dump corpus/fuzz_findings/activator-text-key.ein          # release, 6a775f3d
;;   noted (2)
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
$ ein saturate --dump corpus/fuzz_findings/activator-text-key-swapped.ein
;;   noted (2)
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
```

The printed lines are the same in both orders, and they are not the same
facts: the first file holds the int's two and the second the string's two. The
fact printer drops a string's quotes (P1e2.3's N5), so the dump, `--events`
and a claim cannot tell `1` from `"1"`; the count can (`noted (2)` where the
rule has four conclusions), and so does the summary's
`root.facts_by_relation`. `--events` shows one `compile`, and every enqueue
and fire the first activator's plan's. In a build with debug assertions both
files abort in `Engine::compile_all`, whose debug-only check that its shortcut
skipped no activator counts two activators and one plan:

```
thread 'main' panicked at crates/ein-infer/src/engine.rs:134:13:
the rule-application counter said nothing changed and a full walk disagreed
```

`compile.rs` documented the collision as deliberate (*the integer `7` and the
atom `7` both stringify to `"7"` and therefore share a key*). It predates
S1e2.2.7: it reproduces at `16a05caf`.

Unlike W7's pair, both orders are wrong: whichever activator is written first
keeps its conclusions, and the other's are lost.
