/*
 * zebra_module.c — the Zebra puzzle as a problem module: a grid size and one
 * function.
 *
 * The half of `build/zebra-blackbox` that knows what the puzzle is. It
 * exports `PROBLEM` and nothing else, so the solver in `blackbox.c` sees a
 * `5 x 5` grid and an `int (*)(const int *)` and has no way to learn that
 * row 0 is a colour, that there are fourteen conditions, or that condition
 * (9) reads one row and could have been tested first.
 *
 * All fifteen conditions live in **one** function. Not fourteen small ones
 * behind a table, as in `zebra_oracles.c` — one, because that is what an
 * interface with a single predicate in it gets you, and because it is the
 * shape a compiled or generated rule set has: the conditions are still all
 * there, but from outside they are one indivisible yes/no.
 *
 * That is the whole of what a module supplies, and the whole of what the
 * solver is entitled to.
 */

#include <stdlib.h>

#include "problem.h"

enum { N = 5, M = 5 };

/* Rows, in the order the module chooses to lay them out. The solver never
 * learns what these mean; it only uses `m` and the label tables. */
enum { R_COLOUR, R_NATION, R_DRINK, R_SMOKE, R_PET };

enum { RED, GREEN, IVORY, YELLOW, BLUE };
enum { ENGLISHMAN, SPANIARD, UKRAINIAN, NORWEGIAN, JAPANESE };
enum { COFFEE, TEA, MILK, JUICE, WATER };
enum { OLD_GOLD, KOOLS, CHESTERFIELDS, LUCKY_STRIKE, PARLIAMENT };
enum { DOG, SNAIL, FOX, HORSE, ZEBRA };

static const char *const ROW_NAME[M] = {"colour", "nationality", "drink",
                                        "smoke", "pet"};
static const char *const COLOUR_NAME[N] = {"Red", "Green", "Ivory", "Yellow",
                                           "Blue"};
static const char *const NATION_NAME[N] = {"Englishman", "Spaniard",
                                           "Ukrainian", "Norwegian",
                                           "Japanese"};
static const char *const DRINK_NAME[N] = {"Coffee", "Tea", "Milk", "Juice",
                                          "Water"};
static const char *const SMOKE_NAME[N] = {"Old_Gold", "Kools", "Chesterfields",
                                          "Lucky_Strike", "Parliament"};
static const char *const PET_NAME[N] = {"Dog", "Snail", "Fox", "Horse",
                                        "Zebra"};
static const char *const *const VALUE_NAME[M] = {
    COLOUR_NAME, NATION_NAME, DRINK_NAME, SMOKE_NAME, PET_NAME};

/* Which cell of `row` holds `value`. Every row is a permutation, so there is
 * exactly one. */
static int cell_of(const int *grid, int row, int value)
{
    const int *r = grid + row * N;
    for (int c = 0; c < N; c++)
        if (r[c] == value)
            return c;
    return -1;
}

#define SAME(ra, x, rb, y) (cell_of(grid, ra, x) == cell_of(grid, rb, y))
#define AT(ra, x, c) (cell_of(grid, ra, x) == (c))
#define RIGHT_OF(ra, x, rb, y) (cell_of(grid, ra, x) == cell_of(grid, rb, y) + 1)
#define NEXT_TO(ra, x, rb, y) \
    (abs(cell_of(grid, ra, x) - cell_of(grid, rb, y)) == 1)

/* The one predicate. Condition (1) — five houses in a row — is the grid, and
 * "each value appears exactly once" is the row being a permutation, which the
 * solver enforces for its own reasons rather than for the puzzle's. The other
 * fourteen are here, in the order the puzzle states them, and `&&` gives them
 * the same short-circuit an array of oracles would have had. */
static int zebra_holds(const int *grid)
{
    return SAME(R_NATION, ENGLISHMAN, R_COLOUR, RED)                 /* (2)  */
           && SAME(R_NATION, SPANIARD, R_PET, DOG)                   /* (3)  */
           && SAME(R_DRINK, COFFEE, R_COLOUR, GREEN)                 /* (4)  */
           && SAME(R_NATION, UKRAINIAN, R_DRINK, TEA)                /* (5)  */
           && RIGHT_OF(R_COLOUR, GREEN, R_COLOUR, IVORY)             /* (6)  */
           && SAME(R_SMOKE, OLD_GOLD, R_PET, SNAIL)                  /* (7)  */
           && SAME(R_SMOKE, KOOLS, R_COLOUR, YELLOW)                 /* (8)  */
           && AT(R_DRINK, MILK, 2)                                   /* (9)  */
           && AT(R_NATION, NORWEGIAN, 0)                             /* (10) */
           && NEXT_TO(R_SMOKE, CHESTERFIELDS, R_PET, FOX)            /* (11) */
           && NEXT_TO(R_SMOKE, KOOLS, R_PET, HORSE)                  /* (12) */
           && SAME(R_SMOKE, LUCKY_STRIKE, R_DRINK, JUICE)            /* (13) */
           && SAME(R_NATION, JAPANESE, R_SMOKE, PARLIAMENT)          /* (14) */
           && NEXT_TO(R_NATION, NORWEGIAN, R_COLOUR, BLUE);          /* (15) */
}

const struct problem PROBLEM = {
    .name = "Zebra (Einstein's puzzle) — who drinks water, who owns the zebra",
    .n = N,
    .m = M,
    .row_name = ROW_NAME,
    .value_name = VALUE_NAME,
    .holds = zebra_holds,
};
