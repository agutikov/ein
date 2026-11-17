# Q-M1f.220 — What root owes is a block of at most four `owed` rows under `∅`

Where the `∅` row reads *not in R — root owes n*, describe, `ein oracle` and `--no-search` print under it one `owed`
row per instance, each through its obligation rule's `:why`, in the order the rules report them, at most four; then
one line at the values' column that counts the rest and names `--dump-answer DIR`, whose `answer.txt` has every one.
The block is outside the listings' 100 lines, which are R's. `--count` prints the `∅` row and no `owed` row, and a
mode prints neither; the summary's `region.root_owed` holds every instance on every description.

**Why.** A root's debts are not R's states, so they do not take the lines R's listing needs, and a root that owes a
hundred instances cuts its own block, not the answer.

**Held by.** `ein-cli/tests/read_out.rs`: `the_read_out_names_what_root_owes`; the row:
[08-read-out.md](../../ein/user/04-reference/08-read-out.md#2-describe-a-bare-ein-solve---count-and-ein-oracle) § 2.
