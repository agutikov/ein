/*
 * zebra.c — the Zebra puzzle, solved by brute-force enumeration in plain C11.
 *
 *     ./build.sh && build/zebra
 *
 * This is the *baseline*, deliberately: the same puzzle `examples/zebra.ein`
 * encodes, with the same value names, but with the fifteen clues hardcoded as
 * C predicates over five arrays instead of stated as facts a solver reasons
 * from. It is here to be read next to `ein solve examples/zebra.ein` — same
 * answer, and the two get there in completely different ways.
 *
 * The model is as simple as it can be. Five houses in a row, numbered 0..4
 * left to right, and five arrays indexed by house:
 *
 *     colour[h]  nation[h]  drink[h]  smoke[h]  pet[h]
 *
 * each holding one value of its own enum. Every attribute is a permutation of
 * its five values across the five houses, so the whole search space is
 * 120^5 = 24 883 200 000 assignments — which is why the loops prune between
 * levels rather than at the bottom. Each clue is tested at the first level
 * where all the attributes it names are bound; see the `(n)` comments, which
 * are the puzzle's own numbering and match the `:source "condition (n)"`
 * annotations in examples/zebra.ein.
 *
 * The question: who drinks water, and who owns the zebra?
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum { N = 5, NPERM = 120 };

/* The values. Each enum is independent and each numbers its members 0..4, so
 * a permutation of {0..4} is exactly an assignment of one attribute to the
 * five houses. */
enum Colour { RED, GREEN, IVORY, YELLOW, BLUE };
enum Nation { ENGLISHMAN, SPANIARD, UKRAINIAN, NORWEGIAN, JAPANESE };
enum Drink { COFFEE, TEA, MILK, JUICE, WATER };
enum Smoke { OLD_GOLD, KOOLS, CHESTERFIELDS, LUCKY_STRIKE, PARLIAMENT };
enum Pet { DOG, SNAIL, FOX, HORSE, ZEBRA };

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

/* ── The permutations ──────────────────────────────────────────────── */

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

/* ── The predicates ────────────────────────────────────────────────── */

/* Which house holds `value` of this attribute. Every attribute is a
 * permutation, so this always finds exactly one. */
static int house_of(const int a[N], int value)
{
    for (int h = 0; h < N; h++)
        if (a[h] == value)
            return h;
    return -1; /* unreachable while every array is a permutation */
}

/* "X and Y are in the same house."   */
#define SAME(A, x, B, y) (house_of(A, x) == house_of(B, y))
/* "X is in house h."                 */
#define AT(A, x, h) (house_of(A, x) == (h))
/* "X's house is immediately right of Y's." */
#define RIGHT_OF(A, x, B, y) (house_of(A, x) == house_of(B, y) + 1)
/* "X's house is next to Y's."        */
#define NEXT_TO(A, x, B, y) (abs(house_of(A, x) - house_of(B, y)) == 1)

/* ── The search ────────────────────────────────────────────────────── */

static int colour[N], nation[N], drink[N], smoke[N], pet[N];

/* The solution, kept separately: the five arrays above are the loops' own
 * state and hold the *last* assignment tried once the search is over. */
static int s_colour[N], s_nation[N], s_drink[N], s_smoke[N], s_pet[N];

/* Per level: assignments tried, and assignments that survived this level's
 * clues. The pair is the whole story of why 24.9 billion is not the cost. */
static long long tried[N], kept[N];
static int n_solutions;

#define ROW "  %-5s  %-6s  %-11s  %-6s  %-13s  %s\n"

static void print_solution(void)
{
    printf(ROW, "house", "colour", "nationality", "drink", "smoke", "pet");
    for (int h = 0; h < N; h++) {
        char house[8];
        snprintf(house, sizeof house, "%d", h + 1);
        printf(ROW, house, COLOUR_NAME[s_colour[h]], NATION_NAME[s_nation[h]],
               DRINK_NAME[s_drink[h]], SMOKE_NAME[s_smoke[h]],
               PET_NAME[s_pet[h]]);
    }
}

int main(void)
{
    int scratch[N];
    gen_perms(scratch, 0, 0);

    for (int c = 0; c < n_perm; c++) {
        memcpy(colour, perm[c], sizeof colour);
        tried[0]++;
        /* (6) the green house is immediately right of the ivory house */
        if (!RIGHT_OF(colour, GREEN, colour, IVORY))
            continue;
        kept[0]++;

        for (int n = 0; n < n_perm; n++) {
            memcpy(nation, perm[n], sizeof nation);
            tried[1]++;
            /* (10) the Norwegian lives in the first house */
            if (!AT(nation, NORWEGIAN, 0))
                continue;
            /* (2) the Englishman lives in the red house */
            if (!SAME(nation, ENGLISHMAN, colour, RED))
                continue;
            /* (15) the Norwegian lives next to the blue house */
            if (!NEXT_TO(nation, NORWEGIAN, colour, BLUE))
                continue;
            kept[1]++;

            for (int d = 0; d < n_perm; d++) {
                memcpy(drink, perm[d], sizeof drink);
                tried[2]++;
                /* (9) milk is drunk in the middle house */
                if (!AT(drink, MILK, 2))
                    continue;
                /* (4) coffee is drunk in the green house */
                if (!SAME(drink, COFFEE, colour, GREEN))
                    continue;
                /* (5) the Ukrainian drinks tea */
                if (!SAME(nation, UKRAINIAN, drink, TEA))
                    continue;
                kept[2]++;

                for (int s = 0; s < n_perm; s++) {
                    memcpy(smoke, perm[s], sizeof smoke);
                    tried[3]++;
                    /* (8) Kools are smoked in the yellow house */
                    if (!SAME(smoke, KOOLS, colour, YELLOW))
                        continue;
                    /* (13) the Lucky Strike smoker drinks juice */
                    if (!SAME(smoke, LUCKY_STRIKE, drink, JUICE))
                        continue;
                    /* (14) the Japanese smokes Parliaments */
                    if (!SAME(nation, JAPANESE, smoke, PARLIAMENT))
                        continue;
                    kept[3]++;

                    for (int p = 0; p < n_perm; p++) {
                        memcpy(pet, perm[p], sizeof pet);
                        tried[4]++;
                        /* (3) the Spaniard owns the dog */
                        if (!SAME(nation, SPANIARD, pet, DOG))
                            continue;
                        /* (7) the Old Gold smoker owns snails */
                        if (!SAME(smoke, OLD_GOLD, pet, SNAIL))
                            continue;
                        /* (11) Chesterfields are smoked next to the fox */
                        if (!NEXT_TO(smoke, CHESTERFIELDS, pet, FOX))
                            continue;
                        /* (12) Kools are smoked next to the horse */
                        if (!NEXT_TO(smoke, KOOLS, pet, HORSE))
                            continue;
                        kept[4]++;

                        /* Clue (1) — five houses in a row — is the model
                         * itself, and the "each value appears exactly once"
                         * conditions are the permutations. Nothing is left to
                         * check: this is a solution. */
                        n_solutions++;
                        memcpy(s_colour, colour, sizeof colour);
                        memcpy(s_nation, nation, sizeof nation);
                        memcpy(s_drink, drink, sizeof drink);
                        memcpy(s_smoke, smoke, sizeof smoke);
                        memcpy(s_pet, pet, sizeof pet);
                        printf("solution %d\n", n_solutions);
                        print_solution();
                        printf("\n");
                    }
                }
            }
        }
    }

    /* What the enumeration cost, level by level. */
    static const char *const LEVEL[N] = {"colour", "nation", "drink", "smoke",
                                         "pet"};
    long long total = 0;
    printf("enumeration\n");
    printf("  %-8s %14s %14s\n", "level", "tried", "kept");
    for (int i = 0; i < N; i++) {
        printf("  %-8s %14lld %14lld\n", LEVEL[i], tried[i], kept[i]);
        total += tried[i];
    }
    printf("  %-8s %14lld\n", "total", total);
    printf("  (the space without pruning is 120^5 = 24883200000)\n\n");

    if (n_solutions != 1) {
        fflush(stdout);
        fprintf(stderr, "expected exactly one solution, found %d\n",
                n_solutions);
        return 1;
    }

    int water = house_of(s_drink, WATER), zebra = house_of(s_pet, ZEBRA);
    printf("%s drinks water   (house %d)\n", NATION_NAME[s_nation[water]],
           water + 1);
    printf("%s owns the zebra (house %d)\n", NATION_NAME[s_nation[zebra]],
           zebra + 1);

    /* The known answer, so this doubles as its own smoke test. */
    if (s_nation[water] != NORWEGIAN || s_nation[zebra] != JAPANESE) {
        fflush(stdout);
        fprintf(stderr, "that is not the answer to this puzzle\n");
        return 1;
    }
    return 0;
}
