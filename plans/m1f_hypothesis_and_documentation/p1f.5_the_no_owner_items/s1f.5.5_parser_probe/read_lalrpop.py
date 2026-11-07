#!/usr/bin/env python3
"""Can a small reader take the grammar back out of ein.lalrpop?  It strips the actions, writes the
productions as EBNF and as GBNF, then interprets them over the corpus: its accept/reject must be the
generated parser's.  Throwaway; no dependency on lalrpop."""
import re,sys,subprocess,functools
sys.setrecursionlimit(100000)
src=open(sys.argv[1]).read(); H=sys.argv[2]
# --- the token rules
m=re.search(r'match \{(.*?)\n\}\n',src,re.S); block=m.group(1); body=src[m.end():]
tiers=[]; 
for tier in re.split(r'\n\} else \{',block):
    t=[]
    i=0
    for mm in re.finditer(r'''r#"(.*?)"#(?:\s*=>\s*(\w+|\{\}))?|r"((?:[^"\\]|\\.)*)"(?:\s*=>\s*(\w+|\{\}))?|"((?:[^"\\]|\\.)*)"''',tier):
        if mm.group(5) is not None: t.append((re.escape(mm.group(5)),mm.group(5),True))
        else:
            rx=mm.group(1) if mm.group(1) is not None else mm.group(3); nm=mm.group(2) or mm.group(4)
            t.append((rx,nm,False))
    tiers.append(t)
lit=sum(1 for t in tiers for x in t if x[2])
rules=[(re.compile(re.sub(r'\\u\{(\w+)\}',r'\\x\1',rx)),nm,rank) for rank,t in enumerate(tiers) for rx,nm,_ in t]
def lex(s):
    i=0;out=[]
    while i<len(s):
        best=None
        for rx,nm,rank in rules:
            a=rx.match(s,i)
            if a and a.end()>i and (best is None or a.end()>best[0] or (a.end()==best[0] and rank<best[1])): best=(a.end(),rank,nm)
        if best is None: return None
        if best[2]!='{}': out.append(best[2])
        i=best[0]
    return out
# --- the productions: name, params, alternatives with actions stripped
def split_alts(t):
    """Top-level alternatives of `{ a => act, b => act, c, }` or a single `a => act`."""
    t=t.strip()
    if t.startswith('{') and t.endswith('}') : t=t[1:-1]
    alts=[];cur='';d=0;i=0;q=False
    while i<len(t):
        c=t[i]
        if q:
            cur+=c
            if c=='\\': cur+=t[i+1]; i+=1
            elif c=='"': q=False
        elif c=='"': q=True; cur+=c
        elif c in '({[<': d+=1; cur+=c
        elif c in ')}]': d-=1; cur+=c
        elif c=='>':
            if t[i-1]=='=' : cur+=c        # =>
            else: d-=1; cur+=c
        elif c==',' and d==0: alts.append(cur); cur=''
        else: cur+=c
        i+=1
    if cur.strip(): alts.append(cur)
    return [a.split('=>')[0].strip() for a in alts]
prods={}
for mm in re.finditer(r'''^(pub )?(\w+)(<[\w,]*>)?:\s*[^=\n]*?=\s*(.*?)(?=;\s*$)''',body,re.M|re.S):
    name=mm.group(2); params=(mm.group(3) or '<>')[1:-1].split(',') if mm.group(3) else []
    prods[name]=(params,split_alts(mm.group(4)))
def sym(alt):
    """`<a:W<"and",()>> <v:TArg+> ")"` to a list of (kind,...)"""
    i=0;out=[];n=len(alt)
    def one(i):
        while alt[i].isspace(): i+=1
        if alt[i]=='"':
            j=i+1
            while alt[j]!='"': j+= 2 if alt[j]=='\\' else 1
            return ('t',alt[i+1:j]),j+1
        if alt[i]=='@': return None,i+2
        if alt[i]=='<':
            j=i+1
            a=re.match(r'\w+:',alt[j:])
            if a: j+=a.end()
            x,j=one(j); 
            while alt[j].isspace(): j+=1
            assert alt[j]=='>',alt[j:]
            j+=1
            return post(x,j)
        if alt[i]=='(' : return ('t','()'),i+2
        a=re.match(r'\w+',alt[i:]); nm=a.group(); j=i+a.end()
        if j<n and alt[j]=='<' and nm in prods and prods[nm][0]:
            j+=1;args=[]
            while True:
                x,j=one(j); args.append(x)
                while alt[j].isspace(): j+=1
                if alt[j]==',': j+=1; continue
                if alt[j]=='>': j+=1; break
            return post(('m',nm,tuple(args)),j)
        return post(('n',nm),j)
    def post(x,j):
        if j<n and alt[j] in '*+?': return (alt[j],x),j+1
        return x,j
    while i<n:
        if alt[i].isspace(): i+=1; continue
        x,i=one(i)
        if x: out.append(x)
    return out
G={k:(v[0],[sym(a) for a in v[1]]) for k,v in prods.items()}
# --- out: EBNF and GBNF
def show(x,env=None,g=False):
    k=x[0]
    if k=='t':
        return x[1]
    if k=='n': return x[1]
    if k=='m': return x[1]+'['+','.join(show(a) for a in x[2])+']'
    return show(x[1])+k
ebnf=[f"{n}{'['+','.join(v[0])+']' if v[0] else ''} ::= "+' | '.join(' '.join(("'"+s[1]+"'" if s[0]=='t' else show(s)) for s in a) for a in v[1]) for n,v in G.items()]
open(sys.argv[3],'w').write('\n'.join(ebnf)+'\n')
# --- interpret
def run(toks):
    @functools.lru_cache(maxsize=None)
    def p(x,i,envk):
        env=dict(envk)
        k=x[0]
        if k=='t':
            if x[1]=='()': return (i,)
            return (i+1,) if i<len(toks) and toks[i]==x[1] else ()
        if k=='n':
            if x[1] in env: return p(env[x[1]],i,())
            if x[1] in G: return prod(x[1],(),i)
            return (i+1,) if i<len(toks) and toks[i]==x[1] else ()
        if k=='m': return prod(x[1],tuple(subst(a,env) for a in x[2]),i)
        if k in '*+?':
            res=[] if k=='+' else [i]
            fr=p(x[1],i,envk) if True else ()
            seen=set(res)
            front=list(fr)
            for e in front:
                if e not in seen: seen.add(e); res.append(e)
            if k=='?': return tuple(res)
            while front:
                nx=[]
                for e in front:
                    for e2 in p(x[1],e,envk):
                        if e2 not in seen: seen.add(e2); res.append(e2); nx.append(e2)
                front=nx
            return tuple(res)
    def subst(a,env):
        if a[0]=='n' and a[1] in env: return env[a[1]]
        return a
    @functools.lru_cache(maxsize=None)
    def prod(nm,args,i):
        params,alts=G[nm]; envk=tuple(zip(params,args))
        out=[]
        for alt in alts:
            fr=[i]
            for s in alt:
                nx=[]
                for e in fr:
                    for e2 in p(s,e,envk):
                        if e2 not in nx: nx.append(e2)
                fr=nx
                if not fr: break
            for e in fr:
                if e not in out: out.append(e)
        return tuple(out)
    return len(toks) in prod('Program',(),0)
lines=subprocess.run([H,'list'],capture_output=True,text=True).stdout.splitlines()
same=diff=acc=0
for l in lines:
    want,f=l.split(' ',1); want=want=='1'
    t=lex(open(f,encoding='utf-8').read())
    got=t is not None and run(tuple(t))
    if got: acc+=1
    if got==want: same+=1
    else: diff+=1; print('DIFF',f,'parser',want,'reader',got)
print(f'productions read {len(G)}, token rules {len(rules)} ({lit} literals); files {len(lines)}: reader accepts {acc}, agrees with the parser on {same}, differs on {diff}')
