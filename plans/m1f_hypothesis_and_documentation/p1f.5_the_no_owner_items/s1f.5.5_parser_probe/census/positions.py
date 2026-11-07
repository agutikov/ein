#!/usr/bin/env python3
"""T1f.5.5.1's corpus census: every position of the grammar, and what the corpus
writes there. Read-only.

A token is read as today's lexer reads it (the corpus has no token whose reading
the name rule changes: census.py, 2026-11-07), and each node is labelled with the
position the AST gives it: which form, which keyword, which slot. Prints one line
per (position, shape) with a count and up to three files.

  python3 positions.py ROOT [--roots examples,tests,stdlib,corpus] [--json OUT]
"""
import collections, json, os, re, sys

ROOT = sys.argv[1]
roots = 'examples,tests,stdlib,corpus'
out_json = None
args = sys.argv[2:]
while args:
    a = args.pop(0)
    if a == '--roots': roots = args.pop(0)
    elif a == '--json': out_json = args.pop(0)
files = []
for top in roots.split(','):
    for d, _, fs in os.walk(os.path.join(ROOT, top)):
        for f in fs:
            if f.endswith('.ein'):
                files.append(os.path.join(d, f))
files.sort()

# ── a reader: today's token classes, a tree of (kind, text) leaves and lists ──
SYM = re.compile(r'(__)?[A-Za-z][A-Za-z0-9_*.\-]*$')
VAR = re.compile(r'\?[A-Za-z][A-Za-z0-9_*\-]*$')
KW = re.compile(r':[a-z][A-Za-z0-9_\-]*$')
INT = re.compile(r'-?[0-9]+$')
RNG = re.compile(r'[0-9]+\.\.([0-9]+|\*)$')

def toks(s):
    i, n = 0, len(s)
    while i < n:
        c = s[i]
        if c.isspace() or '\x1c' <= c <= '\x1f': i += 1; continue
        if c == ';':
            while i < n and s[i] != '\n': i += 1
            continue
        if s.startswith('#|', i):
            j = s.find('|#', i + 2); i = n if j < 0 else j + 2; continue
        if c in '()': yield (c, c); i += 1; continue
        if c == '"':
            j = i + 1
            while j < n and s[j] != '"': j += 2 if s[j] == '\\' else 1
            yield ('STR', s[i:j + 1]); i = j + 1; continue
        j = i
        while j < n and not (s[j].isspace() or s[j] in '()";' or '\x1c' <= s[j] <= '\x1f'): j += 1
        t = s[i:j]; i = j
        if t == '_': k = '_'
        elif t == '=': k = 'NAME'
        elif SYM.match(t): k = 'NAME'
        elif VAR.match(t): k = 'VAR'
        elif KW.match(t): k = 'KW'
        elif RNG.match(t): k = 'RANGE'
        elif INT.match(t): k = 'NEGINT' if t.startswith('-') else 'DIGITS'
        else: k = 'OTHER'
        yield (k, t)

def tree(s):
    st = [[]]
    for k, t in toks(s):
        if k == '(': st.append([])
        elif k == ')':
            if len(st) == 1: raise ValueError('extra )')
            x = st.pop(); st[-1].append(x)
        else: st[-1].append((k, t))
    if len(st) != 1: raise ValueError('unclosed')
    return st[0]

CONNECTIVES = {'and', 'or', 'not', 'neq', 'eq', 'absent', 'forall', 'unknown'}
def shape(v):
    """The shape of a node, as the census reports it."""
    if isinstance(v, list):
        if not v: return '()'
        h = v[0]
        if isinstance(h, list): return '(LIST …)'
        hk, ht = h
        if hk == 'NAME' and ht in CONNECTIVES | {'model', 'false', 'region', 'merged', 'owing', 'open', '=', 'relation'}:
            return f'({ht} …)'
        if hk == 'VAR': return '(?var …)'
        if hk == '_': return '(_ …)'
        if hk == 'NAME': return '(name …)'
        return f'({hk} …)'
    k, t = v
    return {'NAME': 'name', 'VAR': '?var', 'KW': ':kw', 'STR': 'string', '_': '_',
            'DIGITS': 'digits', 'NEGINT': '-digits', 'RANGE': 'range', 'OTHER': 'other'}[k]

def split(items):
    """Positional items and keyword pairs of a list's tail, as the parser pairs them."""
    pos, pairs, i = [], [], 0
    while i < len(items):
        x = items[i]
        if not isinstance(x, list) and x[0] == 'KW':
            pairs.append((x[1], items[i + 1] if i + 1 < len(items) else None)); i += 2
        else:
            pos.append(x); i += 1
    return pos, pairs

C = collections.defaultdict(collections.Counter)
EX = collections.defaultdict(lambda: collections.defaultdict(list))
cur = ['']
def note(pos, v, s=None):
    s = s or shape(v)
    C[pos][s] += 1
    ex = EX[pos][s]
    if cur[0] not in ex and len(ex) < 3: ex.append(cur[0])

def head_of(v):
    return v[0] if isinstance(v, list) and v and not isinstance(v[0], list) else None

# ── the walk, position by position ──
def fact_arg(v, where):
    note(where, v)
    if isinstance(v, list) and v and not isinstance(v[0], list):
        h = v[0]
        note(where + '/nested.head', h)
        pos, pairs = split(v[1:])
        for a in pos: fact_arg(a, where + '/nested.arg')
        for k, val in pairs: note(where + '/nested.kwpair', val, f'{k} {shape(val) if val is not None else "MISSING"}')

def fact(f, kind='fact'):
    pos, pairs = split(f[1:])
    if kind == 'fact': note('fact.head', f[0])
    for a in pos: fact_arg(a, f'{kind}.arg')
    for k, v in pairs:
        note(f'{kind}.keyword', None, k)
        note(f'{kind}.{k}', v)
        if k == ':using' and isinstance(v, list):
            for a in v: note(f'{kind}.:using/item', a)

def pattern(v, where, depth=0):
    """A pattern: a relation applied to arguments."""
    h = v[0]
    note(where + '.head', h if not isinstance(h, list) else h)
    pos, pairs = split(v[1:])
    for a in pos:
        note(where + ('.arg' if depth == 0 else '.nested.arg'), a)
        if isinstance(a, list) and a: pattern(a, where, depth + 1)
    for k, val in pairs:
        note(where + ('.kwpair' if depth == 0 else '.nested.kwpair'), None, k)

def premise(v, where):
    """A :match premise or a :goal."""
    note(where, v)
    h = head_of(v)
    if h is None: return
    name = h[1] if h[0] == 'NAME' else None
    pos, pairs = split(v[1:])
    for k, val in pairs: note(where + '.kwpair-in-connective' if name in CONNECTIVES else where + '/pattern.kwpair', None, k)
    if name in ('and', 'or'):
        for a in pos: premise(a, where)
    elif name == 'not':
        note(where + '/not.arg', pos[0] if pos else None, None if pos else 'MISSING')
        for a in pos:
            if isinstance(a, list) and a: pattern(a, where + '/not.pattern')
    elif name == 'absent':
        note(where + '/absent.arity', None, str(len(pos)))
        for a in pos: premise(a, where + '/absent')
    elif name in ('eq', 'neq'):
        note(where + f'/guard.arity', None, str(len(pos)))
        for a in pos: note(where + '/guard.arg', a)
    elif name == 'forall':
        note(where + '/forall.arity', None, str(len(pos)))
        for a in pos[1:]: premise(a, where + '/forall')
    elif name == 'unknown':
        for a in pos: premise(a, where + '/unknown')
    else:
        pattern(v, where + '/pattern')

def conclusion(v, where):
    note(where, v)
    h = head_of(v)
    if h is None: return
    name = h[1] if h[0] == 'NAME' else None
    pos, pairs = split(v[1:])
    if name == 'and':
        for k, val in pairs: note(where + '.kwpair-in-and', None, k)
        for a in pos: conclusion(a, where)
    elif name == 'not':
        note(where + '/not.arg', pos[0] if pos else None, None if pos else 'MISSING')
        for a in pos:
            if isinstance(a, list) and a: pattern(a, where + '/not.pattern')
    else:
        pattern(v, where + '/pattern')

def claim_model(m, where):
    pos, pairs = split(m[1:])
    for a in pos:
        note(where + '/model.item', a)
        if isinstance(a, list) and a: pattern(a, where + '/model.fact')
    for k, _ in pairs: note(where + '/model.kwpair', None, k)

def query_claim(v, where):
    note(where, v)
    h = head_of(v)
    if h is None: return
    if h[1] == 'model': claim_model(v, where)
    elif h[1] == 'or':
        for a in v[1:]:
            note(where + '/or.item', a)
            if head_of(a) and head_of(a)[1] == 'model': claim_model(a, where + '/or')

def region_claim(v, where):
    note(where, v)
    h = head_of(v)
    if h is None or h[1] != 'region': return
    for it in v[1:]:
        note(where + '/region.item', it)
        hh = head_of(it)
        if not hh: continue
        if hh[1] == 'model': claim_model(it, where + '/region')
        elif hh[1] == 'merged':
            for x in it[1:]: note(where + '/merged.item', x)
        elif hh[1] == 'owing':
            for x in it[1:]: note(where + '/owing.item', x)

def names_list(v, where):
    note(where, v)
    if isinstance(v, list):
        for x in v: note(where + '/item', x)

for path in files:
    cur[0] = os.path.relpath(path, ROOT)
    try:
        t = tree(open(path, encoding='utf-8').read())
    except Exception as e:
        note('<file does not read as balanced lists>', None, 'unbalanced'); continue
    for f in t:
        if not isinstance(f, list):
            note('top-level', f); continue
        if not f:
            note('top-level', f); continue
        h = f[0]
        if isinstance(h, list):
            note('top-level', f); continue
        hk, ht = h
        if hk != 'NAME' or ht not in {'relation', 'rule', 'hrule', 'query', 'solve', 'config', 'trace', 'macro', 'import', 'not'}:
            note('form', None, 'fact'); fact(f); continue
        note('form', None, ht)
        pos, pairs = split(f[1:])
        if ht == 'relation':
            if pos: note('relation.name', pos[0])
            for a in pos[1:]: note('relation.signature', a)
            for k, v in pairs: note('relation.keyword', None, k); note(f'relation.{k}', v)
        elif ht in ('rule', 'hrule'):
            if pos: note(f'{ht}.name', pos[0])
            if len(pos) > 1:
                note(f'{ht}.params', pos[1])
                if isinstance(pos[1], list):
                    for p in pos[1]: note(f'{ht}.params/item', p)
            for a in pos[2:]: note(f'{ht}.extra-positional', a)
            for k, v in pairs:
                note(f'{ht}.keyword', None, k)
                if k == ':match': premise(v, f'{ht}.:match')
                elif k == ':assert': conclusion(v, f'{ht}.:assert')
                else: note(f'{ht}.{k}', v)
        elif ht == 'query':
            for a in pos: note('query.positional', a)
            for k, v in pairs:
                note('query.keyword', None, k)
                if k == ':goal': premise(v, 'query.:goal')
                elif k == ':expect': query_claim(v, 'query.:expect')
                else: note(f'query.{k}', v)
        elif ht == 'solve':
            for a in pos: note('solve.positional', a)
            for k, v in pairs:
                note('solve.keyword', None, k)
                if k == ':expect': region_claim(v, 'solve.:expect')
                elif k == ':hrules':
                    note('solve.:hrules', v)
                    if isinstance(v, list):
                        for x in v: note('solve.:hrules/item', x)
                else: names_list(v, f'solve.{k}')
        elif ht == 'config':
            for a in pos: note('config.positional', a)
            for k, v in pairs: note('config.keyword', None, k); note('config.value', v)
        elif ht == 'trace':
            for e in pos:
                note('trace.event', e)
                if head_of(e):
                    note('trace.event.word', None, head_of(e)[1])
                    epos, epairs = split(e[1:])
                    for a in epos: note('trace.step.positional', a)
                    for k, v in epairs:
                        note('trace.step.keyword', None, k); note(f'trace.step.{k}', v)
        elif ht == 'macro':
            if pos: note('macro.name', pos[0])
            if len(pos) > 1: names_list(pos[1], 'macro.params')
            if len(pos) > 2: premise(pos[2], 'macro.body')
            for k, v in pairs: note('macro.keyword', None, k)
        elif ht == 'import':
            if pos: note('import.module', pos[0])
            for a in pos[1:]: note('import.positional', a)
            for k, v in pairs:
                note('import.keyword', None, k)
                if k == ':symbols': names_list(v, 'import.:symbols')
                else: note(f'import.{k}', v)
        elif ht == 'not':
            note('form', None, 'not (stored negation)')
            fact(f, 'notfact')

print(f'# {len(files)} files under {roots}')
for pos in sorted(C):
    tot = sum(C[pos].values())
    print(f'{pos}  [{tot}]')
    for s, n in C[pos].most_common():
        ex = ', '.join(EX[pos][s])
        print(f'    {s:28} {n:6}   {ex}')
if out_json:
    json.dump({p: {s: {'n': n, 'ex': EX[p][s]} for s, n in C[p].items()} for p in C}, open(out_json, 'w'), indent=1)
