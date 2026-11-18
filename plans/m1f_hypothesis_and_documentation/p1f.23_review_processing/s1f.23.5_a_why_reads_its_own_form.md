# S1f.23.5 — a `:why` reads the form it is written on, `{?n}` its n-th arrow

**Phase:** [P1f.23](README.md)
**Status:** planned 2026-11-18 on the user's word. **Task 1, the change shown on real programs, runs now and stops
for the user's look** (the user's standing rule: a syntax shown on real corpus programs, its output computed, before
it is built). Task 2, the build, runs on the user's word after that look, and after the phase's other stages are
merged (it touches the standard library, the examples and the tests they touch).

## The ruling

The user, 2026-11-18, verbatim:

> also in this p1f.23 phase - universal renderings of NL, :why "{1}" - {1} is the same as 1-arrow of this fact, {2}
> 2nd-arrow, etc... So (relation owns Person Pet :why "a {?3} {?2} a {?4}") produce "a Person owns a Pet"

and, asked how a fact of the relation then reads and which spelling a placeholder takes, the user's choices:

- **"Declaration only"**: a `:why` reads only the form it is written on. A fact of `owns`, `(owns Ann Dog)`, has no
  template from the declaration, and reads by the fallback (`owns(Ann, Dog)` or what the fallback ladder gives),
  unless something else gives it one.
- **"{?n}, renumbered"**: today's spelling `{?n}`, counted from the relation as 1, so every relation's `:why` of the
  standard library and the corpus is renumbered.

So: `(relation owns Person Pet :why "a {?3} {?2} a {?4}")` is the fact `(relation owns Person Pet)`, its arrows
`relation` (1), `owns` (2), `Person` (3), `Pet` (4), and reads *a Person owns a Pet*. Today the same `:why` is the
template of every fact of `owns`, `{?1}` its first argument
([`14-natural-language.md` § 1](../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md#1-the-four-texts)).

## Task 1 — the change on real programs (stops for the user)

In `s1f.23.5_examples.md` beside this file, with the programs it runs:

1. **What reads a `:why` today**, each surface with a line of it: the trace's sentence after a derived fact, the
   read-out's, `ein render`'s labels if any, `--json-summary`, the narration, `utils/nl_census.py`; and the fallback
   ladder (§ 3) as it is.
2. **Real programs, today and after**: `examples/zebra2.ein` (a step of the trace and a state's reading), a
   calculator program, a beliefs program (a fact under an arrow), a standard-library module's relations
   (`std.algebra`, `std.slots`): for each relation, its `:why` today, its `:why` renumbered, what the declaration
   reads as after, and what one fact of it read as today and reads as after (the fallback). The *after* is computed
   by hand from the rule and marked as such, since nothing is built.
3. **What the ruling leaves open**, each with a default: what a rule's, an hrule's and a macro's `:why` read (their
   forms are no facts; their variables `{?x}` as today); whether a fact may carry a `:why` of its own (*"unless
   something else gives it one"*); whether `:why` stays required on a relation; what `--no-require-why` becomes; the
   fallback's form; how a `{?n}` past the form's arrows prints; the renumbering as a generated, checked rewrite of
   every relation's `:why`.
4. **What the build would touch**: the code (`git grep -l 'why' -- ein.rs/crates/*/src`), the standard library's
   relations (counted), the corpus's, the tests (`natural_language_text.rs`, `natural_language_page.rs`, the trace's
   goldens), the pages (`14-natural-language.md`, the walkthrough, the read-out), the decisions (Q-M1f.58, Q-M1f.112,
   Q-M1f.159 and every one that states `{?1}` is a first argument).

## Task 2 — the build

Ruled 2026-11-18: every open point of task 1 as its default; runs after the phase's other stages are merged.

## Record

- Task 1 done, `11a113cf`, waiting for the user's look: [`s1f.23.5_examples.md`](s1f.23.5_examples.md).
- **2026-11-18, the user on facts**, asked whether a fact must carry a `:why` or a `:source` (it need not, today): *"leave
  facts as they are"*. So a fact carries no `:why`, and `:source` stays optional: the examples' open point 2 is
  ruled as its default, (a).
- **2026-11-18, the user**: *"S1f.23.5: defaults for the rest, build after merges"*. Every open point of
  [`s1f.23.5_examples.md`](s1f.23.5_examples.md) is ruled as its default; **task 2, the build, runs on that word once
  the phase's other stages are merged** into master (S1f.23.1, S1f.23.2 with S1f.23.3, S1f.23.4).

### Task 2 — the build

Run 2026-11-18 in the worktree `build/wt-s1f235`, branch `s1f.23.5-why`, on `02efd4e7` (every other stage of the phase
merged). Commits: `3047bbae` (L-105), `fbe2ba66` (L-102), `bd21f0bb` (the build), and this record.

**The engine.** `ein_render::sentence::statement` reads a declaration alone: a fact `(relation R L1 … Ln)` whose tuple
is the declaration the program wrote, by the declaration's `:why`, `{?n}` its n-th arrow (`{?1}` `relation`, `{?2}`
`R`, `{?k+2}` `Lk`); `None` for every other fact, the companion `(relation R)` of a typed declaration among them, and an
untyped declaration its own form. A `TraceStep` carries each premise's statement (`premise_statements`), and the
premises line prints it after the premise, `` `relation(owns, Person, Pet)` — a Person owns a Pet ``, the premises'
sources still once after the last premise; the *Derives* line prints a conclusion as written, a statement only after a
declaration. `ein-core/src/why.rs`' text and unit test follow. With L-102: `ein_render::dump::serialise` deleted whole
(`facts_to_ein_text`, `kb_to_ein_text`, `fact_line`, `provenance_comment`, `firing_to_json`, `fact_summary`), and with
it `sentence.rs`' `fact_sentence`, `FactSentence`, `Origin`, `as_comment`, which only that writer read; the shape
golden's `states` op prints each fact and its first origin (`source "…"`, `rule r`, `hypothesis N`), no English.

**The renumbering**, `{?k}` → `{?k+2}` in every declaration's `:why`, by
[`s1f.23.5_examples/renumber.py`](s1f.23.5_examples/renumber.py) (a declaration found in a `.ein`, a Rust string, a
Python string or a page; a rule's, an hrule's and a macro's `:why` untouched), counted on `02efd4e7`'s blobs:

| root | files | declarations | references |
|---|---:|---:|---:|
| `stdlib/` | 16 | 359 | 933 |
| `examples/` | 450 | 1 389 | 2 066 |
| `tests/` | 95 | 257 | 477 |
| `corpus/fuzz_findings/` | 6 | 19 | 27 |
| `corpus/regressions.toml` (a state file's printed line) | 1 | 2 | 2 |
| `ein.rs/crates/*/src/` (unit tests' programs) | 12 | 148 | 216 |
| `ein.rs/crates/*/tests/` (programs, fixtures, goldens) | 70 | 1 456 | 1 960 |
| `ein.rs/crates/ein-infer/examples/` | 1 | 9 | 9 |
| `docs/ein/` | 38 | 247 | 466 |
| `docs/decisions/` (Q27's example) | 1 | 1 | 2 |
| `utils/` (by the script) | 6 | 12 | 18 |

and by hand, the generators whose texts sit in a table or are computed: `gen_arith_tables.py` (11 texts),
`gen_num9.py` (19 texts and the two computed digit runs, `range(3, w + 3)` and `range(3, w + 4)`), `guard_split.py`
(`why_of`'s `{?j + 3}` and three texts). Every generator's `--check` then agreed with the renumbered files
(`check_generated.py`: 9 generators), the zebra2 variants regenerated for the comment edit, `stdlib_manifest.py
--write`. Leftovers the script lists (a `{?digit}` outside a declaration's `:why`) were each read and rewritten by
hand: `zebra.ein`'s and `zebra2.ein`'s comments, three tests' constants and a `.replace` pattern
(`unicode_only_in_a_string.rs`, which would have matched nothing and silently dropped the character from its `:why`;
it now asserts the replace took), `why.rs`, `template_wellformedness.rs`, `sentence.rs`.

**Four lines, today** (`build/review-bin/ein-head-b5fb8d70`, from task 1) **and as built**:

| program, step | today | as built |
|---|---|---|
| `02-zebra2.ein`, root step 231 `std.bijection.domain-elimination` | ``Premises: `std.algebra.functional(nation-loc)`, `std.algebra.total(nation-loc)`, `relation(nation-loc, Nationality, House)`, `is-a(Norwegian, Nationality)`, `is-a(House-1, House)` `` | ``Premises: `std.algebra.functional(nation-loc)`, `std.algebra.total(nation-loc)`, `relation(nation-loc, Nationality, House)` — the Nationality lives in House, `is-a(Norwegian, Nationality)`, `is-a(House-1, House)` `` |
| the same step | ``Derives `nation-loc(Norwegian, House-1)` — the Norwegian lives in House-1`` | ``Derives `nation-loc(Norwegian, House-1)`.`` |
| `03-sum.ein`, step 24 `std.math.positional.turn-end` | ``Derives `std.math.decimal-arith.done(std.math.decimal-arith.sum(apples, pears), n(n(z, 2), 2))` — the answer for (std.math.decimal-arith.sum apples pears) is (n (n z 2) 2); `std.math.decimal.valid(n(n(z, 2), 2))` — (n (n z 2) 2) is a decimal number`` | ``Derives `std.math.decimal-arith.done(std.math.decimal-arith.sum(apples, pears), n(n(z, 2), 2))`; `std.math.decimal.valid(n(n(z, 2), 2))`.`` |
| `04-packing.ein`, step 1 `witnessed` | ``Derives `knows(Ann, likes(Dan, Soup))` — Ann knows that (likes Dan Soup)`` | ``Derives `knows(Ann, likes(Dan, Soup))`.`` |

and the user's own example, `(relation owns Person Pet :why "a {?3} {?2} a {?4}")` read by a rule:
``Premises: `relation(owns, Person, Pet)` — a Person owns a Pet, `owns(Bob, Cat)` ``.

**The shape golden**, re-blessed twice with `EIN_BLESS=1 EIN_CORPUS_SLOW=1` (29 027 lines each time): with L-102,
454 `states` lines moved and nothing else; with the build, measured against that golden, 2 510 `ir` lines
(`ir[resolve]` 738, `ir[expand]` 738, `ir[parse]` 517, `ir[dump-compact]` 517), 644 `load` lines (the `REL … why=`
texts) and 463 `region-trace` lines moved, and **no `read-out`, `states`, `oracle`, `cores`, `saturate`, `plan`,
`match`, `naf`, `explain`, `dot` or `region-lattice` line**: the answer is the same, by the invariance of the two
goldens.

**Tests and checks.** `cargo test --profile gate --workspace --no-fail-fast` once, on the build's tree: 162 test
binaries, 1 349 tests, none failed, none ignored, nothing left in the run's `TMPDIR` (`check_test_scratch.py`).
`cargo fmt --check`, `cargo clippy --workspace --all-targets -D warnings`, `cargo doc --no-deps` with `-D warnings`:
clean. `doc_audit.py --links --check --tracked`: findings only under `review/` (the bundle's links to its ignored
outputs, as known); every decision id resolves. `check_doc_blocks.py`, `check_release_voice.py`,
`check_gate_steps.py`, `check_tool_mirrors.py`, `check_generated.py --bin ein.rs/target/gate/ein` (9 generators) and
`wrong_answers.py` (16 open rows, each still answered wrong): green. Not run here, by the brief: `./run_tests.sh`,
the nightly, `utils/ci_container.sh`.

**Readings, each a default** (filed as Q-M1f.475 to .482, `open_questions.md`):

1. **The premises line** prints a declaration's statement right after it, ` — <statement>`, and the premises'
   sources stay once after the last premise, ` — from <source>`: a line can carry two ` — `, the second the sources'
   of the line (`14-natural-language.md` § 3 says so). The examples page's open point 3 (a) showed this form.
2. **A declaration a rule concludes** reads by its `:why` on the *Derives* line (open point 10, *however a run
   reaches it*); no corpus program derives one, and the new test does not either.
3. **The wellformedness bound** for a declaration is `{?2}` to `{?n+2}` and `{?1}` refused, with an untyped
   declaration bounded to `{?2}` (before, an untyped declaration admitted any `{?n}`); no corpus text is past it. The
   engine still prints a `{?1}` as `relation` and a reference past the arrows as written (open point 7 (a)).
4. **L-102's reach**: `fact_sentence`, `FactSentence`, `Origin` and `as_comment` went with the six functions the
   ledger names, since nothing else read them; the `states` op digests the origin words with no English, not the
   rule's filled `:why` (the trace's ops hold that).
5. **L-105's reach**: with the answer half, `nl_census.py`'s `KERNEL` copy went, so `check_tool_mirrors.py` drops its
   census mirror; § 4's registers are still held to `ein_core::RESERVED` by `natural_language_page.rs`.
6. **The decisions**: Q-M1f.58 rewritten as the ruling (its title extended, slug kept); Q-M1f.61's title now *…each
   conjunct reads as written* (slug kept); Q-M1f.60, Q-M1f.214, Q-M1f.12 and Q31 restated to the rule; Q27's
   example renumbered. Q-M1f.62, .57, .63 stand (open points 4, 5).
7. **The pages**: `14-natural-language.md` rewritten (the intro, § 1's row, § 2's house style restated as the
   sentence a declaration reads as, with the stdlib's and examples' texts said to read stiffly, § 3's ladder without
   the relation's rung and its block moved from `zebra.ein`, whose trace reads no declaration, to `zebra2.ein`, § 4's
   column *who can give its declaration a text*, § 5, § 6, § 7); the trace blocks of `04-ontology.md`, `05-rules.md`,
   `07-properties-and-operators.md`, `10-theory.md`, `11-macros.md` and the guide's `05-jack-drinks-coffee.md`
   re-taken from their programs; the prose of `01-objects.md`, `02-relations.md`, `03-facts.md`, `09-hypotheses.md`,
   `02-syntax.md`, `07-diagnostics.md`, `12-rendering.md`, the guide's `01-objects-and-relations.md` and the section
   README; AGENTS.md's standing rule on `:why`.
8. **The words are not rewritten** (open point 9): only the page examples that show the rule use the user's pattern,
   `a {?3} {?2} a {?4}`.

**Findings** (filed as R-34 to R-38, [the phase's findings](findings.md)):

- **N-a.** The *In words* sections of `01-objects.md`, `02-relations.md` and `03-facts.md` (two) quoted a state
  file's facts with their English in comments, which no state file has held since a saved state is its program and
  its choices (Q-M1f.392): stale before this stage; rewritten here with the trace's lines.
- **N-b.** `ein-ir/tests/kb_semantics.rs` named a flag `--print-final-facts` no command has, and
  `ein-cli/tests/events_reference.rs` an `emit` in `dump/serialise.rs` that file never had; both comments fixed in
  the L-102 commit.
- **N-c.** A test that rewrites a program by a `.replace` on its text silently stops testing when the text moves
  (`unicode_only_in_a_string.rs`); the one found asserts its replace now. Others of the kind were not searched for.
- **N-d.** Q-M1f.12 lists `--events` and the JSON summary among the surfaces that carry no text, while both carry an
  obligation's `:why` (task 1's finding, still open; outside this stage).
- **N-e.** The counts moved since task 1 with the merged stages: `examples/` holds 1 389 renumbered texts (task 1:
  1 401), in 591 files (597).
