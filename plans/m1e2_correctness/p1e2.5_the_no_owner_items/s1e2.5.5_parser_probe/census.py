#!/usr/bin/env python3
"""What the corpus writes, read with the user's name rule of 2026-11-07:
a token is a maximal run of non-separator characters. Read-only."""
import collections, os, re, sys, unicodedata
ROOT=os.path.abspath(os.path.join(os.path.dirname(__file__),'../../../..'))
files=[]
for top in ('examples','tests','stdlib','corpus','plans','docs'):
    for d,_,fs in os.walk(os.path.join(ROOT,top)):
        for f in fs:
            if f.endswith('.ein'): files.append(os.path.join(d,f))
print('files',len(files))
SEP=set('()";')
def toks(s):
    i=0;n=len(s);out=[]
    while i<n:
        c=s[i]
        if c.isspace() or '\x1c'<=c<='\x1f': i+=1; continue
        if c==';':
            while i<n and s[i]!='\n': i+=1
            continue
        if s.startswith('#|',i):
            j=s.find('|#',i+2); i=n if j<0 else j+2; continue
        if c in '()': out.append(c); i+=1; continue
        if c=='"':
            j=i+1
            while j<n and s[j]!='"':
                j+=2 if s[j]=='\\' else 1
            out.append(s[i:j+1]); i=j+1
            # glue: what follows a string
            if i<n and not(s[i].isspace() or s[i] in '();'): out.append('<<GLUE>>'+s[i:i+10])
            continue
        j=i
        while j<n and not(s[j].isspace() or s[j] in SEP or '\x1c'<=s[j]<='\x1f'): j+=1
        out.append(s[i:j])
        if j<n and s[j]=='"': out.append('<<GLUEQ>>'+s[i:j+6])
        i=j
    return out
oldsym=re.compile(r'^(__)?[A-Za-z][A-Za-z0-9_*.-]*$')
oldvar=re.compile(r'^\?[A-Za-z][A-Za-z0-9_*-]*$')
oldkw=re.compile(r'^:[a-z][A-Za-z0-9_-]*$')
oldint=re.compile(r'^-?[0-9]+$')
oldrange=re.compile(r'^[0-9]+\.\.([0-9]+|\*)$')
cls=collections.Counter(); ex=collections.defaultdict(lambda: collections.Counter())
chars=collections.Counter(); inner=collections.Counter()
for p in files:
    try: s=open(p,encoding='utf-8').read()
    except Exception as e: print('unreadable',p,e); continue
    for t in toks(s):
        if t in '()' : continue
        if t.startswith('"'):
            cls['STRING']+=1; 
            continue
        if t.startswith('<<'): cls['glued to a string']+=1; ex['glued to a string'][os.path.relpath(p,ROOT)+' '+t]+=1; continue
        if t=='_': k='bare _'
        elif t=='=': k='='
        elif oldsym.match(t): k='SYMBOL today'
        elif oldvar.match(t): k='VAR today'
        elif oldkw.match(t): k='KEYWORD today'
        elif oldint.match(t): k='INT today'
        elif oldrange.match(t): k='RANGE today'
        else:
            k='other run'; ex[k][t]+=1
        cls[k]+=1
        for c in t[1:]:
            if not c.isalnum() and c not in '_-': inner[c]+=1
for k,v in cls.most_common(): print(f'{k:22} {v}')
print('non-alnum chars after the first, any token:',inner.most_common())
print('--- other runs (would change reading):')
for t,c in ex['other run'].most_common(70): print(repr(t),c)
print('--- glued to a string:')
for t,c in list(ex['glued to a string'].most_common(20)): print(t,c)
