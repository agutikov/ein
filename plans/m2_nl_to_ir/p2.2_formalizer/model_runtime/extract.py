#!/usr/bin/env python3
"""Take runs out of the probe's record for the reference: `python3 extract.py probe.jsonl OUT_DIR ID...`
writes, for each run, OUT_DIR/ID.json (its request) and appends its recorded call to OUT_DIR/record.jsonl,
in the order of the ids. Standard library only."""
import gzip, json, os, sys

path, out, ids = sys.argv[1], sys.argv[2], sys.argv[3:]
op = gzip.open if path.endswith(".gz") else open
lines = {}
with op(path, "rt") as f:
    for l in f:
        if l.strip():
            r = json.loads(l)
            lines[r["planned"]["id"]] = r
os.makedirs(out, exist_ok=True)
with open(os.path.join(out, "record.jsonl"), "a") as rec:
    for i in ids:
        r = lines[i]
        json.dump(r["planned"]["request"], open(os.path.join(out, i + ".json"), "w"))
        rec.write(json.dumps(r["recorded"]) + "\n")
