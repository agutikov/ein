#!/usr/bin/env python3
"""Every refusal site in from_ir.rs and compile.rs (non-test code): the call that
makes an error and its message text, one row per site. Read-only."""
import re, sys, json
def sites(path):
    src = open(path).read()
    cut = src.find('#[cfg(test)]')
    if cut > 0: src = src[:cut]
    out = []
    # error-making calls
    pat = re.compile(r'(errors\.push\(|KbLoadError\(|CompileError\(|Err\(format!|Err\("|ok_or_else\(\|\|\s*format!|ok_or_else\(\|\|\s*"|LoadErr\(|err\(format!|return Err\()')
    for m in pat.finditer(src):
        # balanced paren span from the opening paren of the call
        i = m.end() - 1
        while i < len(src) and src[i] != '(':
            i += 1
        depth = 0; j = i; instr = False; esc = False
        while j < len(src):
            c = src[j]
            if instr:
                if esc: esc = False
                elif c == '\\': esc = True
                elif c == '"': instr = False
            else:
                if c == '"': instr = True
                elif c == '(': depth += 1
                elif c == ')':
                    depth -= 1
                    if depth == 0: break
            j += 1
        body = src[i:j+1]
        lits = re.findall(r'"((?:[^"\\]|\\.)*)"', body)
        text = ''.join(lits)
        text = re.sub(r'\\\n\s*', '', text)
        line = src[:m.start()].count('\n') + 1
        if not lits: continue
        out.append((line, m.group(1), text))
    # de-duplicate nested matches (Err(CompileError(format!...))) by line
    seen = {}
    for l, k, t in out:
        if l not in seen or len(t) > len(seen[l][1]): seen[l] = (k, t)
    return sorted((l, k, t) for l, (k, t) in seen.items())
for p in sys.argv[1:]:
    rows = sites(p)
    print(f'## {p}: {len(rows)} sites')
    for l, k, t in rows:
        print(f'{l}\t{k}\t{t[:260]}')
