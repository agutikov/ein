# Q-M1f.26 — Every member of N carries its core, and a renderer refuses one without

Every surface that prints N, `--dump-answer`'s `nogoods.json` and `--trace`, prints each member with its core, past
`--region-limit` too: a member whose core the route did not take is given it after the route, the core the route would
have taken. There is no status for a member kept without a core. A library caller may describe R without asking for
N's cores (`region::Cores::nogoods` off), and a renderer handed such a member refuses it, naming the option the run
was not asked: an engine defect, never a line of output.

**Why.** A dead set printed without why it is dead tells a reader nothing the set does not; with every member's core
taken after the route, a status for the missing core would have no writer.

**Held by.** `ein-infer/tests/explore.rs`: `a_run_that_prints_no_core_takes_none_and_answers_alike`, every member
of N given the eager run's core; the refusals in `ein_render::dump::region_answer` and `ein_render::trace::region`.
`ein-infer/tests/corpus_cores.rs`: `every_core_names_its_commitment_and_derives_its_contradiction`.
