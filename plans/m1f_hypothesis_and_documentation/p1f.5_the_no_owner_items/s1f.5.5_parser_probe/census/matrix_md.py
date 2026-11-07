#!/usr/bin/env python3
"""Render matrix.json as markdown: one table per position, a row per shape."""
import json, sys, collections, re
rows = json.load(open(sys.argv[1]))
def cell(s):
    s = s.replace('|', '\\|')
    s = re.sub(r' — (a|an|the) .*$', '', s)  # keep the refusal's first clause
    return s if len(s) < 150 else s[:147] + '…'
by = collections.OrderedDict()
for r in rows:
    by.setdefault((r['kind'], r['position']), []).append(r)
out = []
for (kind, pos), rs in by.items():
    if kind == 'keyword': continue
    out.append(f"\n#### {pos}\n\n`{rs[0]['template']}`\n")
    out.append('| shape | today | the final grammar |\n|---|---|---|')
    for r in rs:
        out.append(f"| `{r['shape']}` | {cell(r['today'])} | {cell(r['grammar'])} |")
out.append('\n#### A keyword the form does not take\n\n| where | written | today | the final grammar |\n|---|---|---|---|')
for (kind, pos), rs in by.items():
    if kind != 'keyword': continue
    r = rs[0]
    out.append(f"| {pos} | `{r['text']}` | {cell(r['today'])} | {cell(r['grammar'])} |")
print('\n'.join(out))
