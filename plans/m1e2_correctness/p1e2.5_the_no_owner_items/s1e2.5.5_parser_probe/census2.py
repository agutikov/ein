#!/usr/bin/env python3
"""Per form: which keywords, holding which shapes, as the corpus writes them. Read-only."""
import collections, os, re, sys
sys.path.insert(0,'.')
ROOT=os.path.abspath(os.path.join(os.path.dirname(__file__),'../../../..'))
files=[]
for top in ('examples','tests','stdlib','corpus'):
    for d,_,fs in os.walk(os.path.join(ROOT,top)):
        for f in fs:
            if f.endswith('.ein'): files.append(os.path.join(d,f))
def toks(s):
    i=0;n=len(s)
    while i<n:
        c=s[i]
        if c.isspace() or '\x1c'<=c<='\x1f': i+=1; continue
        if c==';':
            while i<n and s[i]!='\n': i+=1
            continue
        if s.startswith('#|',i):
            j=s.find('|#',i+2); i=n if j<0 else j+2; continue
        if c in '()': yield c; i+=1; continue
        if c=='"':
            j=i+1
            while j<n and s[j]!='"': j+=2 if s[j]=='\\' else 1
            yield s[i:j+1]; i=j+1; continue
        j=i
        while j<n and not(s[j].isspace() or s[j] in '()";'): j+=1
        yield s[i:j]; i=j
def tree(ts):
    st=[[]]
    for t in ts:
        if t=='(' : st.append([])
        elif t==')':
            if len(st)==1: raise ValueError('extra )')
            x=st.pop(); st[-1].append(x)
        else: st[-1].append(t)
    if len(st)!=1: raise ValueError('unclosed')
    return st[0]
def sh(v):
    if isinstance(v,list):
        if not v: return '()'
        h=v[0]
        if isinstance(h,list): return '(LIST…)'
        if h in('and','or','not','neq','model','false','region','merged','owing','='): return f'({h}…)'
        if h.startswith('?'): return '(?v…)'
        if h=='_': return '(_…)'
        return '(N…)'
    if v.startswith('"'): return 'STR'
    if v.startswith('?'): return 'VAR'
    if v.startswith(':'): return 'KW'
    if v=='_': return '_'
    if re.fullmatch(r'-?[0-9]+',v): return 'INT'
    if re.fullmatch(r'[0-9]+\.\.([0-9]+|\*)',v): return 'RANGE'
    return 'NAME'
DECL={'relation','rule','hrule','query','solve','config','trace','macro','import'}
forms=collections.Counter(); kw=collections.defaultdict(lambda: collections.defaultdict(collections.Counter))
pos=collections.defaultdict(collections.Counter)
factkw=collections.Counter(); facthead=collections.Counter()
bad=0
def args(form,start):
    i=start;pairs=[];pos_=[]
    while i<len(form):
        x=form[i]
        if isinstance(x,str) and x.startswith(':'):
            pairs.append((x,form[i+1] if i+1<len(form) else None)); i+=2
        else: pos_.append(x); i+=1
    return pos_,pairs
for p in files:
    try: t=tree(toks(open(p,encoding='utf-8').read()))
    except Exception: bad+=1; continue
    for f in t:
        if not isinstance(f,list) or not f or isinstance(f[0],list):
            forms['<other>']+=1; continue
        h=f[0]; name=h if h in DECL else ('not' if h=='not' else '<fact>')
        forms[name]+=1
        p_,pairs=args(f,1)
        for x in p_: pos[name][sh(x)]+=1
        for k,v in pairs:
            kw[name][k][sh(v) if v is not None else 'MISSING']+=1
print('unparsed by a plain reader',bad,'of',len(files))
print(forms.most_common())
for n in sorted(pos): print('POS',n,pos[n].most_common())
for n in sorted(kw):
    print('==',n)
    for k,c in sorted(kw[n].items(),key=lambda kv:-sum(kv[1].values())):
        print('  ',k,sum(c.values()),c.most_common(8))
