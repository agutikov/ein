#!/bin/sh
# S1f.23.7 task 1: every run the examples page quotes, from the repository's root.
#
#   sh run.sh                  # BIN defaults to the review binary of 545e4d6b
#   BIN=path/to/ein sh run.sh
#
# Writes beside this script: the outputs (*.out.txt), the census (census.md,
# census.tsv) and the slices (*.slice.dot, *.constraints.dot and their .svg).
set -eu
here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../../../.." && pwd)
BIN=${BIN:-$repo/build/review-bin/ein-head-545e4d6b}
rel=${here#"$repo"/}
cd "$repo"

# § 1 and § 2: the small program, its KB and the view's facts
"$BIN" saturate --dump "$rel/01-two-definitions.ein" 2>/dev/null \
  | sed -n '/SATURATED KB DUMP/,$p' > "$here/01-two-definitions.dump.txt"
"$BIN" render constraints "$rel/01-two-definitions.ein" > "$here/01-two-definitions.constraints.dot"

# § 4: the goals and the rule
for f in 02-goals 03-refused 04-about; do
  "$BIN" solve "$rel/$f.ein" > "$here/$f.out.txt" 2>&1 || true
done

# § 4 on zebra2: a copy of examples/zebra2.ein with one form appended, each run alone
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
z() {  # z NAME FORM…: zebra2 with the forms appended, solved
  name=$1; shift
  { cat examples/zebra2.ein; printf '\n'; for form in "$@"; do printf '%s\n' "$form"; done; } > "$tmp/$name.ein"
  printf ';; examples/zebra2.ein with, appended:\n' >> "$here/zebra2-goals.out.txt"
  for form in "$@"; do printf ';;   %s\n' "$form" >> "$here/zebra2-goals.out.txt"; done
  "$BIN" solve "$tmp/$name.ein" 2>&1 | sed -n '/^kb load error/p; /query [2-9] of/,$p' \
    | sed "s|$tmp/||g" >> "$here/zebra2-goals.out.txt" || true
  printf '\n' >> "$here/zebra2-goals.out.txt"
}
: > "$here/zebra2-goals.out.txt"
about='(rule about-1 (?F) :match (and (?F ?r) (relation ?r)) :assert (in-slice-1 ?F ?r) :why "{?F} says something of {?r}")'
z or '(query :goal (or (symmetric ?r) (bijective ?r) (transitive ?r)))'
z co '(query :goal (co-located ?r1 ?v1 ?r2 ?v2))'
z about-bijective "$about" '(about-1 bijective)' '(query :goal (in-slice-1 ?f ?r))'
z about-symmetric "$about" '(about-1 symmetric)' '(query :goal (in-slice-1 ?f ?r))'
pack='(rule about-1 (?F) :match (and (?F ?r) (relation ?r)) :assert (in-slice (?F ?r)) :why "{?F} says something of {?r}")'
z pack "$pack" '(about-1 bijective)' '(query :goal (in-slice ?f))'

# § 4.3: the class of 02-goals.ein's query 10, pasted back under it
{ sed -n '1,/^(solve :no-search)/p' "$rel/02-goals.ein"
  printf '%s\n' '(query :goal (or (and (relation ?r) (includes ?r ?s)) (and (relation ?s) (includes ?r ?s)))' \
    '       :expect (model (or (and (relation right-of) (includes right-of next-to)) (and (relation next-to) (includes right-of next-to)))))'
} > "$tmp/or-pasted.ein"
"$BIN" solve "$tmp/or-pasted.ein" 2>&1 | head -2 | sed "s|$tmp/||g" > "$here/or-pasted.out.txt" || true

# § 3: the census
python3 "$here/census.py" --bin "$BIN" > /dev/null

# § 5: a goal's slice, mocked, beside the constraint view
python3 "$here/slice.py" --bin "$BIN" --name zebra2 --program examples/zebra2.ein \
  --goal '(and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))' \
  > "$here/slices.out.txt"
python3 "$here/slice.py" --bin "$BIN" --name zebra2-bijective --program examples/zebra2.ein \
  --goal '(and (bijective ?r) (relation ?r ?a ?b))' >> "$here/slices.out.txt"
python3 "$here/slice.py" --bin "$BIN" --name beliefs --program examples/beliefs/01_common_belief.ein \
  --goal '(believes ?who (likes Dan ?food))' >> "$here/slices.out.txt"
python3 "$here/slice.py" --bin "$BIN" --name add --program examples/arith/add.ein \
  --goal '(sum-of apples pears ?fruit)' >> "$here/slices.out.txt"
rm -f "$here/zebra2-bijective.constraints.dot" "$here/zebra2-bijective.constraints.svg"

# § 5: a query is about R, and zebra2's root is not in it
{ cat examples/zebra2.ein; printf '\n(query :goal (and (drink-loc Water ?h) (nation-loc ?who ?h)))\n'; } > "$tmp/root.ein"
"$BIN" solve "$tmp/root.ein" --no-search 2>&1 | sed -n '/query 2 of/,$p' | sed "s|$tmp/||g" > "$here/zebra2-root-query.out.txt"
