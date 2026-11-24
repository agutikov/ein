#!/usr/bin/env python3
"""The key's and the filter's shares of the P-core instructions, read off each report `key_profile.sh` writes.

Scratch: an A/B for the coordinator, not a tool of the repository.

    key_profile_figures.py REPORT...  > key_profile.tsv

*state_key*: `ein_infer::canon::state_key` with what it calls (children). *filter*: the vector `node_in_region` built
of the key's facts root does not hold (self). *read_above*: the vectors `Snapshot::new_facts_of` builds of the facts
above root's mark (children; both its calls, a resumed state's delta and the key). *sort*: the sort of `FactId`s
(self). The instructions are the report's P-core event count.
"""
import re, sys
from pathlib import Path


def core(path):
    lines = open(path).read().splitlines()
    out, on = [], False
    for l in lines:
        if "of event 'cpu_core" in l:
            on = True
            continue
        if on and l.startswith("# Samples"):
            break
        if on and l.strip() and not l.startswith("#"):
            m = re.match(r"\s*([\d.]+)%\s+([\d.]+)%\s+\[\.\]\s+(.*?)\s+-\s+-\s*$", l)
            if m:
                out.append((float(m.group(1)), float(m.group(2)), m.group(3)))
    return out



def total(path):
    on = False
    for l in open(path):
        if "of event 'cpu_core" in l:
            on = True
            continue
        if on and l.startswith("# Event count"):
            return int(l.split(":")[1])
    return None


def figures(path):
    rows = core(path)
    key = sum(c for c, s, sym in rows if sym == "ein_infer::canon::state_key")
    filt = sum(s for c, s, sym in rows
               if sym.startswith("<alloc::vec::Vec<ein_core::facts::FactId> as alloc::vec::spec_from_iter_nested")
               and "Filter<core::iter::adapters::copied::Copied<core::slice::iter::Iter<ein_core::facts::FactId>>" in sym
               and "node_in_region" in sym)
    above = sum(c for c, s, sym in rows
                if sym.startswith("<alloc::vec::Vec<ein_core::facts::FactId> as alloc::vec::spec_from_iter_nested")
                and "Filter<core::iter::adapters::flatten::FlatMap" in sym)
    sort = sum(s for c, s, sym in rows if "core::slice::sort" in sym and "ein_core::facts::FactId" in sym)
    node = sum(c for c, s, sym in rows if sym == "ein_infer::region::node_in_region")
    return total(path), key, filt, above, sort, node


print("report\tinstructions\tstate_key\tfilter\tread_above\tsort_factid_self\tnode_in_region")
for p in sys.argv[1:]:
    t, k, f, a, s, n = figures(p)
    print(f"{Path(p).name}\t{t}\t{k:.2f}\t{f:.2f}\t{a:.2f}\t{s:.2f}\t{n:.2f}")
