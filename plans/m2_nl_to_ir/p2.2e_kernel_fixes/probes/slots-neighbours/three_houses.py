#!/usr/bin/env python3
"""`three-houses.ein` by brute force: every assignment of each kind to the
houses, filtered by its three clues; the cat's owner tallied."""
import itertools, collections
H = [0, 1, 2]                      # First, Middle, Third
nat = ["Englishman", "Spaniard", "Norwegian"]; drink = ["Tea", "Milk", "Water"]; pet = ["Dog", "Cat", "Fish"]
tally = collections.Counter(); n = 0
for N in itertools.permutations(H):
    for D in itertools.permutations(H):
        for P in itertools.permutations(H):
            hn = dict(zip(nat, N)); hd = dict(zip(drink, D)); hp = dict(zip(pet, P))
            if abs(hn["Englishman"] - hp["Dog"]) != 1: continue   # next to
            if hd["Milk"] != 1: continue                           # middle
            if hn["Spaniard"] != hd["Tea"]: continue
            n += 1
            tally[[k for k in nat if hn[k] == hp["Cat"]][0]] += 1
print(n, "states;", dict(sorted(tally.items())))
