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
 * its five values across the five houses, so the whole space is
 * 120^5 = 24 883 200 000 assignments — which is why the search prunes between
 * levels rather than at the bottom.
 *
 * The clues are an **array of function pointers**: one `int (*)(void)` per
 * numbered condition, each reading the five arrays directly, tagged with the
 * level at which every attribute it names is finally bound. `search()` then
 * has nothing puzzle-specific left in it — it assigns a permutation, runs the
 * clues due at that level, and recurses. The `(n)` numbering is the puzzle's
 * own, and matches the `:source "condition (n)"` annotations in
 * examples/zebra.ein.
 *
 * That is as far towards ein's shape as C gets you for free, and the gap is
 * the point: the clues are *data* here, but they are still compiled code that
 * only answers this puzzle. Nothing derives a new one, nothing explains why a
 * house got its colour, and a sixteenth condition is an edit and a rebuild.
 *
 * The question: who drinks water, and who owns the zebra?
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum { N = 5, NPERM = 120 };

/* ── The values ────────────────────────────────────────────────────── */

/* Each enum is independent and numbers its members 0..4, so a permutation of
 * {0..4} is exactly an assignment of one attribute to the five houses. */
enum Colour { RED, GREEN, IVORY, YELLOW, BLUE };
enum Nation { ENGLISHMAN, SPANIARD, UKRAINIAN, NORWEGIAN, JAPANESE };
enum Drink { COFFEE, TEA, MILK, JUICE, WATER };
enum Smoke { OLD_GOLD, KOOLS, CHESTERFIELDS, LUCKY_STRIKE, PARLIAMENT };
enum Pet { DOG, SNAIL, FOX, HORSE, ZEBRA };

/* The five arrays: `colour[h]` is the colour of house h, and so on. */
static int colour[N], nation[N], drink[N], smoke[N], pet[N];

/* One level per attribute, in the order the search binds them. The order is
 * a tuning choice and nothing else — it decides how early each clue can
 * prune, and the per-clue table at the end of a run is what would price a
 * different one. */
enum Level { LV_COLOUR, LV_NATION, LV_DRINK, LV_SMOKE, LV_PET, N_LEVELS };

static int *const SLOT[N_LEVELS] = {colour, nation, drink, smoke, pet};

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

static const char *const *const NAME[N_LEVELS] = {
    COLOUR_NAME, NATION_NAME, DRINK_NAME, SMOKE_NAME, PET_NAME};
static const char *const LEVEL_NAME[N_LEVELS] = {"colour", "nationality",
                                                 "drink", "smoke", "pet"};
static const int LEVEL_WIDTH[N_LEVELS] = {6, 11, 6, 13, 5};

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

/* "X and Y are in the same house."         */
#define SAME(A, x, B, y) (house_of(A, x) == house_of(B, y))
/* "X is in house h."                       */
#define AT(A, x, h) (house_of(A, x) == (h))
/* "X's house is immediately right of Y's." */
#define RIGHT_OF(A, x, B, y) (house_of(A, x) == house_of(B, y) + 1)
/* "X's house is next to Y's."              */
#define NEXT_TO(A, x, B, y) (abs(house_of(A, x) - house_of(B, y)) == 1)

/* Condition (1) — five houses in a row — is the model itself, and the "each
 * value appears exactly once" conditions are the permutations. The other
 * fourteen are here, one function each. */

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

struct clue {
    int level;           /* the first level at which every value it names is bound */
    const char *text;    /* the condition, as the puzzle states it */
    int (*holds)(void);  /* does the current assignment satisfy it? */
};

/* Grouped by level, and within a level cheapest-and-most-selective first —
 * `clues_hold` stops at the first failure, so the order decides which clue
 * gets the credit in the rejection table, and a clue that never rejects
 * anything is one the level order has made redundant. */
static const struct clue CLUES[] = {
    {LV_COLOUR, "(6)  the green house is immediately right of the ivory house", c06},

    {LV_NATION, "(10) the Norwegian lives in the first house", c10},
    {LV_NATION, "(2)  the Englishman lives in the red house", c02},
    {LV_NATION, "(15) the Norwegian lives next to the blue house", c15},

    {LV_DRINK, "(9)  milk is drunk in the middle house", c09},
    {LV_DRINK, "(4)  coffee is drunk in the green house", c04},
    {LV_DRINK, "(5)  the Ukrainian drinks tea", c05},

    {LV_SMOKE, "(8)  Kools are smoked in the yellow house", c08},
    {LV_SMOKE, "(13) the Lucky Strike smoker drinks juice", c13},
    {LV_SMOKE, "(14) the Japanese smokes Parliaments", c14},

    {LV_PET, "(3)  the Spaniard owns the dog", c03},
    {LV_PET, "(7)  the Old Gold smoker owns snails", c07},
    {LV_PET, "(11) Chesterfields are smoked next to the fox", c11},
    {LV_PET, "(12) Kools are smoked next to the horse", c12},
};

enum { N_CLUES = (int)(sizeof CLUES / sizeof CLUES[0]) };

/* ── The search ────────────────────────────────────────────────────── */

/* Per level: assignments tried, and assignments that survived. Per clue: how
 * many assignments it was the first to reject. Together they are the whole
 * story of why 24.9 billion is not the cost. */
static long long tried[N_LEVELS], kept[N_LEVELS], rejected[N_CLUES];

static int n_solutions;
static int solution[N_LEVELS][N];

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

/* Every clue due at this level, in table order, stopping at the first that
 * fails — which is the pruning. */
static int clues_hold(int level)
{
    for (int i = 0; i < N_CLUES; i++) {
        if (CLUES[i].level != level)
            continue;
        if (!CLUES[i].holds()) {
            rejected[i]++;
            return 0;
        }
    }
    return 1;
}

static void print_solution(void);

static void search(int level)
{
    if (level == N_LEVELS) {
        n_solutions++;
        for (int lv = 0; lv < N_LEVELS; lv++)
            memcpy(solution[lv], SLOT[lv], sizeof(int) * N);
        printf("solution %d\n", n_solutions);
        print_solution();
        printf("\n");
        return;
    }
    for (int p = 0; p < n_perm; p++) {
        memcpy(SLOT[level], perm[p], sizeof(int) * N);
        tried[level]++;
        if (!clues_hold(level))
            continue;
        kept[level]++;
        search(level + 1);
    }
}

/* ── Output ────────────────────────────────────────────────────────── */

static void print_solution(void)
{
    /* The last column is not padded, so no line carries trailing blanks. */
    printf("  %-5s", "house");
    for (int lv = 0; lv < N_LEVELS; lv++)
        printf("  %-*s", lv == N_LEVELS - 1 ? 0 : LEVEL_WIDTH[lv],
               LEVEL_NAME[lv]);
    printf("\n");
    for (int h = 0; h < N; h++) {
        printf("  %-5d", h + 1);
        for (int lv = 0; lv < N_LEVELS; lv++)
            printf("  %-*s", lv == N_LEVELS - 1 ? 0 : LEVEL_WIDTH[lv],
                   NAME[lv][solution[lv][h]]);
        printf("\n");
    }
}

static void print_cost(void)
{
    long long total = 0;
    printf("enumeration\n");
    printf("  %-12s %14s %14s\n", "level", "tried", "kept");
    for (int lv = 0; lv < N_LEVELS; lv++) {
        printf("  %-12s %14lld %14lld\n", LEVEL_NAME[lv], tried[lv], kept[lv]);
        total += tried[lv];
    }
    printf("  %-12s %14lld\n", "total", total);
    printf("  (the space without pruning is 120^5 = 24883200000)\n\n");

    printf("clues, by what each one was the first to reject\n");
    for (int i = 0; i < N_CLUES; i++)
        printf("  %14lld  %s\n", rejected[i], CLUES[i].text);
    printf("\n");
}

int main(void)
{
    int scratch[N];
    gen_perms(scratch, 0, 0);

    search(LV_COLOUR);
    print_cost();

    if (n_solutions != 1) {
        fflush(stdout);
        fprintf(stderr, "expected exactly one solution, found %d\n",
                n_solutions);
        return 1;
    }

    const int *who = solution[LV_NATION];
    int water = house_of(solution[LV_DRINK], WATER);
    int zebra = house_of(solution[LV_PET], ZEBRA);
    printf("%s drinks water   (house %d)\n", NATION_NAME[who[water]],
           water + 1);
    printf("%s owns the zebra (house %d)\n", NATION_NAME[who[zebra]],
           zebra + 1);

    /* The known answer, so this doubles as its own smoke test. */
    if (who[water] != NORWEGIAN || who[zebra] != JAPANESE) {
        fflush(stdout);
        fprintf(stderr, "that is not the answer to this puzzle\n");
        return 1;
    }
    return 0;
}
