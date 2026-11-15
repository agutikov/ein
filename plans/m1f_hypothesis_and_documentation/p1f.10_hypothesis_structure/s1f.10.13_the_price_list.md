# S1f.10.13 — The price list

**Phase:** [P1f.10](README.md), track R (the search)
**Status:** planned 2026-11-15, at the re-plan. Not started. **Re-scoped 2026-11-15 on the user's correction**
([README § The two ideas](README.md#the-two-ideas)): track R is required and depends on track S's results. So
this stage runs after track S's stages and not beside them; it prices each object of the structure (the theory's
exclusions, root's exclusions, the groups, the owed groups and grids, the coupling) and not the excluded pairs
alone; and a *no* is a verdict on one way of spending for one question, which sends the track to the next way and
does not close it. Where the text below says otherwise, this line stands.
**Depends on:** [S1f.10.1](s1f.10.1_exclusion_census.md), [S1f.10.10](s1f.10.10_the_theorys_share.md),
[S1f.10.2](s1f.10.2_groups.md), [S1f.10.4](s1f.10.4_bounded_groups.md) and [S1f.10.11](s1f.10.11_coupling.md).
**Read by:** [S1f.10.3](s1f.10.3_the_restricted_join.md), [S1f.10.5](s1f.10.5_ordering.md) and
[S1f.10.14](s1f.10.14_exactly_one_spent.md), none of which builds anything this stage calls a *no*; and the
phase's stop, [S1f.10.12](s1f.10.12_the_structure_shown.md), where the user sees the table.

## Context

The user's second idea is a question with a *maybe* in it: *maybe hyp structure investigation is cheap and can
improve search of R*. This stage answers both halves with numbers before any route changes, and a *no* is a
full result.

**What the re-plan's probe found**, 2026-11-15, the release binary at `dbedc535`, on AC
([README § What was measured at the re-plan](README.md#what-was-measured-at-the-re-plan)): between 0 % and 95 %
of a run's saturations are deaths that hold a pair of one relation agreeing in exactly one argument. The probe
was syntactic. Its script, kept so the table can be re-taken until this stage replaces it:

```python
# probe.py NODES_JSON…  — over `ein solve FILE [MODE] --dump-lattice DIR`'s DIR/nodes.json
import json, sys, itertools, collections
def parse(f): return tuple(f.strip("()").split())
def structural(a, b):
    A, B = parse(a), parse(b)
    if A[0] != B[0] or len(A) != 3 or len(B) != 3: return False
    return (A[1] == B[1]) != (A[2] == B[2])
for path in sys.argv[1:]:
    d = json.load(open(path)); nodes = d["nodes"]
    alive = [(frozenset(n["commitment"]), frozenset(n.get("closure") or n["commitment"]))
             for n in nodes if n["kind"] != "dead"]
    dead = [n for n in nodes if n["kind"] == "dead"]; hit = 0
    for n in dead:
        C = frozenset(n["commitment"]); cl = set(C)
        for c, k in alive:
            if c < C: cl |= k
        hit += any(structural(a, b) for a, b in itertools.combinations(sorted(cl), 2))
    print(d["source"], d["route"], len(nodes), collections.Counter(n["kind"] for n in nodes), hit)
```

## What is priced

**The work, per program and per question.** Every corpus program that searches; the questions `ein solve`
asks: describe, `--count`, `--single`, `-n 1`, `--minimal`, `--maximal`. Per cell: saturations and facts beyond
root, and the saturations split four ways:

| part | what would remove it |
|---|---|
| dead, and the commitment with the closures under it holds an **excluded pair** of S1f.10.1 | *at most one*, [S1f.10.3](s1f.10.3_the_restricted_join.md) |
| dead otherwise | nothing in this phase; a conflict of three or more, [S1f.10.11](s1f.10.11_coupling.md)'s subject |
| alive and owing | *exactly one*, [S1f.10.14](s1f.10.14_exactly_one_spent.md), or an order that reaches a leaf sooner, [S1f.10.5](s1f.10.5_ordering.md) |
| in R | nothing: it is the answer |

**The cost of having the pairs**, by each way of getting them:

| source | what it costs | when it is paid |
|---|---|---|
| **the pair layer whole** | S1f.10.1's saturations for the program | before the first node of any question |
| **the pair layer by need** | one saturation per pair first asked about: the pair `{a, b}` is saturated when a route is about to extend a state holding `a` by `b` | during the run; never more than the layer whole |
| **the pair of a death** | when a set dies, the pair in it that is dead, found from the death's support with no saturation, or by saturating the pairs of the last candidate added with the members of the state | at each death |
| **the small KB** | [S1f.10.10](s1f.10.10_the_theorys_share.md)'s check, a saturation of a smaller KB per pair | before the run, and reusable across puzzles over one ontology |

**Why not a saturation rule instead.**
[`domain_elim_vs_hypothesis.md`](../../../docs/kernel/inference/domain_elim_vs_hypothesis.md) measured the
same structure written as rules (`domain-elimination`, `no-room-left`). The stage restates that comparison in
today's terms: what the rules derive at a node is already in every number above, so what is priced here is only
what remains after them.

## The rule for *go*, written before the table is taken

For a question and a source:

- **go** when, over the corpus programs that search, the total work of that question with the source's cost
  added is lower, **and no program's is higher by more than one tenth**;
- **no** otherwise, with the programs that lose named.

The second clause is the user's ruling that each mode does the least work its question needs: a 45-saturation
`--single` may not be handed a 1 500-saturation preamble because describe gains from it. A source may be *go*
for one question and *no* for another, and then a route takes it only for the first. The tenth is this stage's
number and the user's to revise at the stop.

## Tasks

1. **T1f.10.13.1 — the work table**, by an example in the crate beside S1f.10.1's, reading each route's
   visited nodes in process (what `--dump-lattice` writes) against S1f.10.1's pairs. Within the default budget;
   a cell the budget stops is recorded as stopped, with what it had spent.
2. **T1f.10.13.2 — the cost table**, each source simulated on the same runs: how many saturations it would
   have added and how many removed, counted from the visited nodes, with no route changed.
3. **T1f.10.13.3 — the verdicts**, by the rule above, per question and source; and the two shares the other
   stages need: the *alive and owing* share per question (S1f.10.14), and for `--single` and `-n 1` the
   saturations a best order would have needed, read off the leaves found (S1f.10.5's ceiling).
4. **T1f.10.13.4 — the record**, in this file, with commands and the date, and the page of it
   S1f.10.12's bundle shows.

## Acceptance

- The rule for *go* above is committed before T1's table is.
- Every corpus program that searches has its cells, or is named as stopped.
- Each source has a verdict per question, and each *no* names its losers.
- **A table of *no* everywhere closes track R's spending of pairs**: S1f.10.3 is then recorded as not built,
  with this table as the reason, and track S is untouched.
- No answer, route or surface changes. The stage measures.

## What it must not do

- **Not change a route to measure it.** The simulation reads visited nodes.
- **Not count a saved saturation without the cost that bought it.**
- **Not run a cell past the default budget.**
