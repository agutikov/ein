# S1f.10.4 — Exactly one: owed groups and grids

**Phase:** [P1f.10](README.md), track S (the structure)
**Status:** re-planned 2026-11-15. Not started.
**Depends on:** [S1f.10.2](s1f.10.2_groups.md), the cover and what each program declares beside it.
**Read by:** [S1f.10.12](s1f.10.12_the_structure_shown.md); of track R,
[S1f.10.14](s1f.10.14_exactly_one_spent.md), which spends what this stage finds.

## Context

The instruction's second half:

> Next step is somehow functionally find that there are bounded hyp groups, e.g. permutations of pets,
> permutations of nationalities.

A group says *at most one*. A **bounded** group says *exactly one*, and that is a different and stronger claim:
it is what makes a set of alternatives jointly exhaustive, which is what L5 needs to branch on them alone
([`tree.rs`](../../../ein.rs/crates/ein-infer/src/tree.rs), § L5).

*At most one* is a negative and comes free from the dead pairs. *Exactly one* is a positive: some member
**must** hold, and no amount of pairwise refutation implies that. In the engine as it stands the *must* has one
source, an **obligation**: a state that owes an instance is not in R until a witness arrives, and
`oblgen::alternatives` reads at root the facts that can witness each instance and whether L5's premise holds of
it.

So this stage is a reading of three things against each other, per program:

| | what | from |
|---|---|---|
| **owed** | the instances root owes, each with its alternatives, licensed or not, and the reason where not | `oblgen::Alternatives` |
| **excluding** | the groups | S1f.10.2 |
| **grid** | two families of groups over one member set, every member in one group of each: rows and columns | computed here |

and the objects it names:

- an **owed group**: an owed instance whose alternatives are pairwise excluding. *Exactly one.*
- an **owed set that is not a group**: an instance whose alternatives do not exclude each other. *At least
  one.*
- a **group nothing owes**: *at most one*, and the program does not say one must hold.
- a **grid**, and whether each of its rows and columns is owed: a bijection when all are.

**The instances already disagree in instructive ways.** I-Z2 declares five bijections and root owes 36
instances of two kinds, a value owed a house and a house owed a value. I-Z1 gets its debts from `std.slots`, and the tree licenses the
`slot-owed-room` instances and not the `slot-owed-fill` ones, whose witnesses are the symmetric
`(co-located House-n v)` that the hrule never proposes: half of each grid is owed and unspendable. I-B06
declares `functional` and `sibling-exclusive` as plain rules and owes nothing: its groups are *at most one*
with no *must* anywhere.

## Tasks

1. **T1f.10.4.1 — the table of the three**, for every corpus program that searches: owed groups, owed sets
   that are not groups, groups nothing owes, grids and how much of each is owed. For the instances, the lists.
2. **T1f.10.4.2 — every bijection, found or declared.** A program that states `(bijective R)` and a program
   whose groups form a fully owed grid over `R`'s candidates are compared: found and declared; declared and
   not found, with what at root broke the grid; found and declared nowhere. I-Z2 is the control and must be
   all *found and declared*.
3. **T1f.10.4.3 — where the *must* is missing, what would supply it.** Three routes, as analysis and not as
   something this stage builds:

   | route | the claim | where the *must* comes from |
   |---|---|---|
   | **counting** | a grid of *n* rows and *n* columns over *n* objects | the pigeonhole, which needs the object list to be complete: nothing may introduce another value. The alive-set invariant gives that of names ([`invariant.rs`](../../../ein.rs/crates/ein-infer/src/invariant.rs)); whether it gives it of a *type's* members is the question |
   | **what is declared, read more widely** | `functional` and `total` together, without the word `bijective`; [`stdlib/closure.ein`](../../../stdlib/closure.ein) concludes `(__closed__ R)` from that pair | a declaration the program already carries under another name |
   | **the obligation** | the group is what an obligation rule would owe, were it stated | zero new machinery; the program has to say it |

   For each instance with a group nothing owes: which routes apply, and for the third, the lines the program
   would have to add, written out and run by `ein oracle` on a copy to show the R it gives.
4. **T1f.10.4.4 — the unlicensed half.** For every owed instance L5 does not license, the reason in the
   program's terms. For I-Z1's `slot-owed-fill`: each witness is in the closure of a candidate (its symmetric
   twin), which S1f.10.1's *implies* shows. Whether a witness that a candidate implies can stand for the
   candidate is an argument for S1f.10.14; this task supplies the census: how many unlicensed instances in the
   corpus have every witness implied by a candidate.

## Acceptance

- T1's table and T2's comparison are recorded here for the corpus, and for the instances in
  `build/structure/<program>/`.
- **On I-Z2 every row and every column of five grids is an owed group.** On I-Z1 the stage says exactly which
  halves are owed, which are licensed, and what is missing for the rest.
- **Where exactly one cannot be derived, the stage says so and says what the program would have to state.** A
  written *not without a declaration* is a full result.
- Nothing is synthesised: no rule, no obligation, no closure. The stage reads.
- No answer, route or surface changes.

## What it must not do

- **Not conclude *exactly one* from *at most one* and a count** without the premise written and checked. *An
  invariant that is the warrant for a skip and is enforced nowhere* is the defect this repo has already had
  once (the alive-set invariant before `invariant.rs` evaluated it).
- **Not read `is-a`, `bijective` or any stdlib name inside the engine** to recognise a declaration. The
  comparison with what is declared is the census's, by the mechanisms the engine already reads.
