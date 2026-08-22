/*
 * problem.h — the interface between a *problem module* and a solver that
 * knows nothing about it.
 *
 * A module says how big the grid is and hands over **one** function. That is
 * the whole contract: `n` cells per row, `m` rows, a name for each row and
 * each value so a solution can be printed, and a predicate that takes a
 * completely filled grid and answers yes or no.
 *
 * What is deliberately not here:
 *
 *   - no list of constraints — the module's conditions are inside its one
 *     predicate, and the solver cannot count them, name them or order them;
 *   - no dependency information — nothing says which rows a condition reads,
 *     so nothing can be tested before every row is bound;
 *   - no partial-grid contract — `holds` is called on complete grids only,
 *     because a predicate that could judge a partial one would have to be
 *     told which rows are real, and that is the information this interface
 *     exists to withhold.
 *
 * `c/blackbox.c` is the solver, `c/zebra_module.c` is the module, and they
 * are separate translation units on purpose: the solver's object file has no
 * symbol from the puzzle in it beyond `PROBLEM`.
 */

#ifndef EIN_C_PROBLEM_H
#define EIN_C_PROBLEM_H

struct problem {
    /* What the module is called, for the banner. */
    const char *name;
    /* Cells per row. Every row is a permutation of the values 0..n-1, so a
     * grid is m rows of n distinct values and the space is (n!)^m. */
    int n;
    /* Rows. */
    int m;
    /* `m` row labels, and `m` arrays of `n` value labels. Output only. */
    const char *const *row_name;
    const char *const *const *value_name;
    /* The one predicate. `grid` is row-major, `grid[row * n + cell]`, and
     * every cell is filled. Returns non-zero when the grid satisfies every
     * condition the module cares about. */
    int (*holds)(const int *grid);
};

/* The module a build links in. Exactly one translation unit defines it. */
extern const struct problem PROBLEM;

#endif /* EIN_C_PROBLEM_H */
