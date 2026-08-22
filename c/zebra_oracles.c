/*
 * zebra_oracles.c — the same puzzle, with the clues as **black-box oracles**.
 *
 *     ./build.sh && build/zebra-oracles          # minutes, not milliseconds
 *
 * `zebra_levels.c` tells its search, for every clue, the level at which the
 * clue becomes testable. That tag is not in the puzzle. Somebody read the
 * fourteen conditions, worked out which attributes each one names, and wrote
 * the answer into the table — and it is worth a factor of three and a half
 * million, which is what this file exists to measure.
 *
 * Here the clues arrive the way they would from a plugin or a data file: an
 * array of `int (*)(void)` in the order the puzzle states them, conditions
 * (2) through (15), and **nothing else**. No dependency set, no level, no
 * hint about which of the five arrays an oracle reads. The search cannot know
 * whether an oracle is ready to be asked, and asking it early would not fail
 * — the five arrays are always fully populated, just with a stale permutation
 * from an outer loop — it would quietly answer about an assignment nobody is
 * testing. So there is exactly one place a black box can soundly be opened:
 * the leaf, where every row is bound.
 *
 * That is the whole difference. Same model, same fourteen predicates, same
 * answer — and 120^5 = 24 883 200 000 leaves instead of 6 840 assignments.
 *
 * What survives without any knowledge of the clues:
 *
 *   - the permutation representation, which is structural rather than
 *     puzzle-specific: 120 arrangements per attribute instead of 5^5 = 3 125,
 *     so 120^5 rather than 3 125^5;
 *   - short-circuiting on the first oracle that says no, which is why the
 *     *order* of the array still matters even though nothing else about it
 *     does. This one is the problem's own numbering, which is the order a
 *     data file would arrive in and not a good one.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

enum { N = 5, NPERM = 120 };

/* ── The model — identical to zebra_levels.c ───────────────────────── */

enum Colour { RED, GREEN, IVORY, YELLOW, BLUE };
enum Nation { ENGLISHMAN, SPANIARD, UKRAINIAN, NORWEGIAN, JAPANESE };
enum Drink { COFFEE, TEA, MILK, JUICE, WATER };
enum Smoke { OLD_GOLD, KOOLS, CHESTERFIELDS, LUCKY_STRIKE, PARLIAMENT };
enum Pet { DOG, SNAIL, FOX, HORSE, ZEBRA };

static int colour[N], nation[N], drink[N], smoke[N], pet[N];

enum Row { R_COLOUR, R_NATION, R_DRINK, R_SMOKE, R_PET, N_ROWS };
static int *const ROW[N_ROWS] = {colour, nation, drink, smoke, pet};

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
static const char *const *const NAME[N_ROWS] = {
    COLOUR_NAME, NATION_NAME, DRINK_NAME, SMOKE_NAME, PET_NAME};
static const char *const ROW_NAME[N_ROWS] = {"colour", "nationality", "drink",
                                             "smoke", "pet"};
static const int ROW_WIDTH[N_ROWS] = {6, 11, 6, 13, 5};

static int house_of(const int a[N], int value)
{
    for (int h = 0; h < N; h++)
        if (a[h] == value)
            return h;
    return -1;
}

#define SAME(A, x, B, y) (house_of(A, x) == house_of(B, y))
#define AT(A, x, h) (house_of(A, x) == (h))
#define RIGHT_OF(A, x, B, y) (house_of(A, x) == house_of(B, y) + 1)
#define NEXT_TO(A, x, B, y) (abs(house_of(A, x) - house_of(B, y)) == 1)

/* ── The oracles ───────────────────────────────────────────────────── */

/* Fourteen opaque yes/no functions over the current assignment. Condition (1)
 * — five houses in a row — is the model, and "each value appears exactly
 * once" is the permutation. */

static int c02(void) { return SAME(nation, ENGLISHMAN, colour, RED); }
static int c03(void) { return SAME(nation, SPANIARD, pet, DOG); }
static int c04(void) { return SAME(drink, COFFEE, colour, GREEN); }
static int c05(void) { return SAME(nation, UKRAINIAN, drink, TEA); }
static int c06(void) { return RIGHT_OF(colour, GREEN, colour, IVORY); }
static int c07(void) { return SAME(smoke, OLD_GOLD, pet, SNAIL); }
static int c08(void) { return SAME(smoke, KOOLS, colour, YELLOW); }
static int c09(void) { return AT(drink, MILK, 2); }
static int c10(void) { return AT(nation, NORWEGIAN, 0); }
static int c11(void) { return NEXT_TO(smoke, CHESTERFIELDS, pet, FOX); }
static int c12(void) { return NEXT_TO(smoke, KOOLS, pet, HORSE); }
static int c13(void) { return SAME(smoke, LUCKY_STRIKE, drink, JUICE); }
static int c14(void) { return SAME(nation, JAPANESE, smoke, PARLIAMENT); }
static int c15(void) { return NEXT_TO(nation, NORWEGIAN, colour, BLUE); }

/* In the order the puzzle states them, which is the order a plugin or a data
 * file would hand them over in and is the only ordering available to a search
 * that knows nothing else about them. */
static const struct {
    const char *text;
    int (*holds)(void);
} ORACLE[] = {
    {"(2)  the Englishman lives in the red house", c02},
    {"(3)  the Spaniard owns the dog", c03},
    {"(4)  coffee is drunk in the green house", c04},
    {"(5)  the Ukrainian drinks tea", c05},
    {"(6)  the green house is immediately right of the ivory house", c06},
    {"(7)  the Old Gold smoker owns snails", c07},
    {"(8)  Kools are smoked in the yellow house", c08},
    {"(9)  milk is drunk in the middle house", c09},
    {"(10) the Norwegian lives in the first house", c10},
    {"(11) Chesterfields are smoked next to the fox", c11},
    {"(12) Kools are smoked next to the horse", c12},
    {"(13) the Lucky Strike smoker drinks juice", c13},
    {"(14) the Japanese smokes Parliaments", c14},
    {"(15) the Norwegian lives next to the blue house", c15},
};

enum { N_ORACLES = (int)(sizeof ORACLE / sizeof ORACLE[0]) };

/* ── The search ────────────────────────────────────────────────────── */

/* `assigned[r]` counts permutations written into row r, which is the same
 * quantity `zebra_levels.c` reports as `tried` — so the two programs' totals
 * are on one axis and the ratio between them is not a comparison of two
 * different units. */
static long long assigned[N_ROWS];
static long long leaves, calls, rejected[N_ORACLES];
static int n_solutions, solution[N_ROWS][N];

static int perm[NPERM][N];
static int n_perm;

static void gen_perms(int *out, int depth, int used)
{
    if (depth == N) {
        memcpy(perm[n_perm++], out, sizeof(int) * N);
        return;
    }
    for (int v = 0; v < N; v++) {
        if (used & (1 << v))
            continue;
        out[depth] = v;
        gen_perms(out, depth + 1, used | (1 << v));
    }
}

static void print_solution(void)
{
    printf("  %-5s", "house");
    for (int r = 0; r < N_ROWS; r++)
        printf("  %-*s", r == N_ROWS - 1 ? 0 : ROW_WIDTH[r], ROW_NAME[r]);
    printf("\n");
    for (int h = 0; h < N; h++) {
        printf("  %-5d", h + 1);
        for (int r = 0; r < N_ROWS; r++)
            printf("  %-*s", r == N_ROWS - 1 ? 0 : ROW_WIDTH[r],
                   NAME[r][solution[r][h]]);
        printf("\n");
    }
}

/* Every 2^28 leaves, so a run that takes minutes says so rather than looking
 * hung. The denominator is exact: 120^5. */
#define PROGRESS_MASK ((1LL << 28) - 1)
static const double TOTAL_LEAVES = 24883200000.0;
static clock_t t0;

static void progress(void)
{
    double secs = (double)(clock() - t0) / CLOCKS_PER_SEC;
    double frac = (double)leaves / TOTAL_LEAVES;
    fprintf(stderr, "\r  %5.1f%%  %.2f G leaves  %.0f M/s  eta %.0f s      ",
            100.0 * frac, (double)leaves / 1e9,
            secs > 0 ? (double)leaves / secs / 1e6 : 0.0,
            frac > 0 ? secs / frac - secs : 0.0);
    fflush(stderr);
}

int main(void)
{
    int scratch[N];
    gen_perms(scratch, 0, 0);
    t0 = clock();

    for (int a = 0; a < n_perm; a++) {
        memcpy(ROW[0], perm[a], sizeof(int) * N);
        assigned[0]++;
        for (int b = 0; b < n_perm; b++) {
            memcpy(ROW[1], perm[b], sizeof(int) * N);
            assigned[1]++;
            for (int c = 0; c < n_perm; c++) {
                memcpy(ROW[2], perm[c], sizeof(int) * N);
                assigned[2]++;
                for (int d = 0; d < n_perm; d++) {
                    memcpy(ROW[3], perm[d], sizeof(int) * N);
                    assigned[3]++;
                    for (int e = 0; e < n_perm; e++) {
                        memcpy(ROW[4], perm[e], sizeof(int) * N);
                        assigned[4]++;

                        /* The only place a black box can be opened. */
                        leaves++;
                        if ((leaves & PROGRESS_MASK) == 0)
                            progress();
                        for (int i = 0; i < N_ORACLES; i++) {
                            calls++;
                            if (!ORACLE[i].holds()) {
                                rejected[i]++;
                                goto next;
                            }
                        }
                        n_solutions++;
                        for (int r = 0; r < N_ROWS; r++)
                            memcpy(solution[r], ROW[r], sizeof(int) * N);
                    next:;
                    }
                }
            }
        }
    }
    if (leaves > PROGRESS_MASK)
        fprintf(stderr, "\r%*s\r", 64, "");

    double secs = (double)(clock() - t0) / CLOCKS_PER_SEC;
    printf("solution 1\n");
    print_solution();
    printf("\n");

    long long total = 0;
    for (int r = 0; r < N_ROWS; r++)
        total += assigned[r];
    printf("enumeration — every clue is opaque, so every test is at the leaf\n");
    printf("  %-16s %16lld\n", "assignments", total);
    printf("  %-16s %16lld\n", "leaves", leaves);
    printf("  %-16s %16lld\n", "oracle calls", calls);
    printf("  %-16s %16.2f\n", "calls per leaf", (double)calls / (double)leaves);
    printf("  %-16s %16.1f\n", "seconds", secs);
    printf("  (zebra-levels does the same puzzle in 6840 assignments)\n\n");

    printf("oracles, in the order the puzzle states them, by first rejection\n");
    for (int i = 0; i < N_ORACLES; i++)
        printf("  %16lld  %s\n", rejected[i], ORACLE[i].text);
    printf("\n");

    if (n_solutions != 1) {
        fflush(stdout);
        fprintf(stderr, "expected exactly one solution, found %d\n",
                n_solutions);
        return 1;
    }
    const int *who = solution[R_NATION];
    int water = house_of(solution[R_DRINK], WATER);
    int zebra = house_of(solution[R_PET], ZEBRA);
    printf("%s drinks water   (house %d)\n", NATION_NAME[who[water]],
           water + 1);
    printf("%s owns the zebra (house %d)\n", NATION_NAME[who[zebra]],
           zebra + 1);
    if (who[water] != NORWEGIAN || who[zebra] != JAPANESE) {
        fflush(stdout);
        fprintf(stderr, "that is not the answer to this puzzle\n");
        return 1;
    }
    return 0;
}
