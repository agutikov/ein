/*
 * blackbox.c — a solver that knows a grid size and one yes/no function.
 *
 *     ./build.sh && build/zebra-blackbox
 *
 * The third of the three C implementations, and the one with nothing left in
 * it. `zebra_levels.c` knows every clue and the level at which each becomes
 * testable. `zebra_oracles.c` knows there are fourteen clues and can ask them
 * one at a time, so it can stop at the first that says no. This one knows
 * `n`, `m` and a function pointer.
 *
 * Everything puzzle-shaped is behind `struct problem` (problem.h) in another
 * translation unit, which is not a stylistic choice: it is what makes the
 * claim checkable. Nothing here can name a colour, count the conditions, or
 * discover that condition (9) only reads one row.
 *
 * So the search is the only one available: fill every row with a permutation,
 * call `holds` once on the complete grid, and count. `(n!)^m` grids — for the
 * Zebra module's 5x5 that is 120^5 = 24 883 200 000, the same number
 * `zebra_oracles.c` reaches, arrived at without knowing anything at all.
 *
 * The grid size is dynamic, which is where the rest of the time goes: `n` and
 * `m` are read from the module at run time, so the row loop is a recursion
 * rather than a nest the compiler can unroll, the permutations are generated
 * cell by cell instead of copied from a table, and the predicate is an
 * indirect call through a pointer the optimiser cannot see past. That is the
 * price of generality and it is worth paying attention to: it buys the
 * ability to load a 7x4 problem without recompiling the solver, and the
 * solver is the part that would not have to change.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "problem.h"

static int *grid;              /* m * n, row-major */
static int *solution;          /* the last complete grid that held */
static long long leaves;
static long long calls;
static long long *assigned;    /* per row: permutations written into it */
static int n_solutions;

static double total_leaves;    /* (n!)^m, for the progress line */
static clock_t t0;

#define PROGRESS_MASK ((1LL << 28) - 1)

static void progress(void)
{
    double secs = (double)(clock() - t0) / CLOCKS_PER_SEC;
    double frac = (double)leaves / total_leaves;
    fprintf(stderr, "\r  %5.1f%%  %.2f G grids  %.0f M/s  eta %.0f s      ",
            100.0 * frac, (double)leaves / 1e9,
            secs > 0 ? (double)leaves / secs / 1e6 : 0.0,
            frac > 0 ? secs / frac - secs : 0.0);
    fflush(stderr);
}

/* Fill cell `cell` of row `row`; `used` is the bitmask of values already
 * placed in this row, which is how "every row is a permutation" is enforced
 * without ever building a permutation table. */
static void fill(int row, int cell, unsigned used)
{
    const int n = PROBLEM.n, m = PROBLEM.m;

    if (cell == n) {
        /* A row just got a whole permutation. Counting it here rather than at
         * the leaf puts this program on the same axis as the other two: it is
         * `tried` in zebra_levels.c and `assigned` in zebra_oracles.c. */
        assigned[row]++;
        if (row + 1 == m) {
            leaves++;
            if ((leaves & PROGRESS_MASK) == 0)
                progress();
            calls++;
            if (PROBLEM.holds(grid)) {
                n_solutions++;
                memcpy(solution, grid, sizeof(int) * (size_t)(m * n));
            }
            return;
        }
        fill(row + 1, 0, 0u);
        return;
    }
    for (int v = 0; v < n; v++) {
        if (used & (1u << v))
            continue;
        grid[row * n + cell] = v;
        fill(row, cell + 1, used | (1u << v));
    }
}

static void print_solution(void)
{
    const int n = PROBLEM.n, m = PROBLEM.m;

    /* One column per row of the grid, each as wide as its widest label. */
    int *w = calloc((size_t)m, sizeof *w);
    if (!w)
        return;
    for (int r = 0; r < m; r++) {
        w[r] = (int)strlen(PROBLEM.row_name[r]);
        for (int v = 0; v < n; v++) {
            int len = (int)strlen(PROBLEM.value_name[r][v]);
            if (len > w[r])
                w[r] = len;
        }
    }
    printf("  %-5s", "cell");
    for (int r = 0; r < m; r++)
        printf("  %-*s", r == m - 1 ? 0 : w[r], PROBLEM.row_name[r]);
    printf("\n");
    for (int c = 0; c < n; c++) {
        printf("  %-5d", c + 1);
        for (int r = 0; r < m; r++)
            printf("  %-*s", r == m - 1 ? 0 : w[r],
                   PROBLEM.value_name[r][solution[r * n + c]]);
        printf("\n");
    }
    free(w);
}

int main(void)
{
    const int n = PROBLEM.n, m = PROBLEM.m;

    if (n < 1 || n > 20 || m < 1) {
        fprintf(stderr, "blackbox: refusing a %dx%d grid\n", n, m);
        return 2;
    }
    grid = calloc((size_t)(m * n), sizeof *grid);
    solution = calloc((size_t)(m * n), sizeof *solution);
    assigned = calloc((size_t)m, sizeof *assigned);
    if (!grid || !solution || !assigned) {
        fprintf(stderr, "blackbox: out of memory\n");
        return 2;
    }

    /* (n!)^m, as a double, for the progress line only. */
    double fact = 1.0;
    for (int i = 2; i <= n; i++)
        fact *= i;
    total_leaves = 1.0;
    for (int i = 0; i < m; i++)
        total_leaves *= fact;

    printf("problem  %s\n", PROBLEM.name);
    printf("grid     %d x %d — %.0f grids to try, one predicate, no other\n",
           n, m, total_leaves);
    printf("         information about it\n\n");

    t0 = clock();
    fill(0, 0, 0u);
    if (leaves > PROGRESS_MASK)
        fprintf(stderr, "\r%*s\r", 64, "");
    double secs = (double)(clock() - t0) / CLOCKS_PER_SEC;

    if (n_solutions != 1) {
        fflush(stdout);
        fprintf(stderr, "expected exactly one solution, found %d\n",
                n_solutions);
        return 1;
    }
    printf("solution 1\n");
    print_solution();
    printf("\n");
    long long total = 0;
    for (int r = 0; r < m; r++)
        total += assigned[r];
    printf("enumeration\n");
    printf("  %-16s %16lld\n", "assignments", total);
    printf("  %-16s %16lld\n", "grids", leaves);
    printf("  %-16s %16lld\n", "predicate calls", calls);
    printf("  %-16s %16.1f\n", "seconds", secs);
    /* No comparison line here. This file had one — "zebra-levels does the
     * same puzzle in 6840 assignments" — until a 4x3 test module printed it
     * too, which is exactly the bug the whole arrangement exists to make
     * visible: a solver that knows nothing about the problem cannot say
     * anything about it either. The comparison lives in c/README.md. */

    free(grid);
    free(solution);
    free(assigned);
    return 0;
}
