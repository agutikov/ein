#!/usr/bin/env python3
"""T1f.5.5.1's probe matrix: each shape written at each position of the
grammar, run through today's parser and loader and through the final grammar
(the snapshot's `ein` with EIN_PARSER=grm), with what each does: refuses at
the parse, refuses at load, refuses at the run, or loads and reads what.

  python3 matrix.py OUT.json        (S1F555=work dir, default build/s1f.5.5)
"""
import json, os, re, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
# The work directory holds `ein` (today's, built from the snapshot), `ein-grm`
# (the snapshot's ein with the final grammar behind EIN_PARSER=grm), the
# snapshot itself and a `tmp/`: build/s1f.5.5 by default.
ROOT = os.environ.get('S1F555', os.path.join(HERE, '../../../../../build/s1f.5.5'))
TODAY = [os.path.join(ROOT, 'ein')]
GRM = [os.path.join(ROOT, 'ein-grm')]
ENV = dict(os.environ, EIN_STDLIB=os.path.join(ROOT, 'snapshot/stdlib'))

SHAPES = {
    'name': 'a', 'digits': '7', '-digits': '-7', '1a': '1a', '$x': '$x',
    'string': '"s"', 'range': '1..5', '_': '_', '?x': '?x', '?_': '?_',
    'a?x': 'a?x', 'form': '(f a)', '()': '()', ':k v': ':k v',
    '(and)': '(and (q a))', '(not)': '(not (q a))', '(neq)': '(neq a b)', '(eq)': '(eq a b)',
}
BASE = '(relation p T) (relation q T) (relation hit T) (relation r T T) (p a) (p b) (q a)\n'
# position: (template, how to read the effect). `{S}` is the hole.
POS = {
    # facts
    'fact argument':            ('(p {S})', 'facts p'),
    'nested fact argument':     ('(r (f {S}) c)', 'facts r'),
    'fact relation':            ('({S} c)', 'facts *'),
    'stored negation':          ('(not {S})', 'facts not'),
    'fact :source':             ('(q c :source {S})', 'facts q'),
    'fact :rule':               ('(q c :rule {S})', 'facts q'),
    'fact :using':              ('(q c :rule x :using {S})', 'facts q'),
    'activator argument':       ('(rule act (?y) :match (p ?y) :assert (hit ?y)) (act {S})', 'derived hit'),
    # declarations
    'relation name':            ('(relation {S} T)', 'relations'),
    'relation signature':       ('(relation r2 {S})', 'relations'),
    'relation :why':            ('(relation r2 T :why {S})', 'relations'),
    'rule name':                ('(rule {S} () :match (p ?x) :assert (hit ?x))', 'derived hit'),
    'rule parameter':           ('(rule rr ({S}) :match (p ?x) :assert (hit ?x))', 'derived hit'),
    # premises
    ':match':                   ('(rule rr () :match {S} :assert (hit yes))', 'derived hit'),
    'premise relation':         ('(rule rr () :match ({S} a) :assert (hit yes))', 'derived hit'),
    'premise argument':         ('(rule rr () :match (p {S}) :assert (hit yes))', 'derived hit'),
    'premise nested argument':  ('(r (f a) c) (rule rr () :match (r (f {S}) c) :assert (hit yes))', 'derived hit'),
    'conjunct':                 ('(rule rr () :match (and (p ?x) {S}) :assert (hit ?x))', 'derived hit'),
    'absent argument':          ('(rule rr () :match (and (p ?x) (absent {S})) :assert (hit ?x))', 'derived hit'),
    'guard argument':           ('(rule rr () :match (and (p ?x) (neq ?x {S})) :assert (hit ?x))', 'derived hit'),
    # conclusions
    ':assert':                  ('(rule rr () :match (p ?x) :assert {S})', 'derived *'),
    'conclusion argument':      ('(rule rr () :match (p ?x) :assert (hit {S}))', 'derived hit'),
    'conclusion relation':      ('(rule rr () :match (p ?x) :assert ({S} ?x))', 'derived *'),
    'hrule conclusion argument': ('(hrule h () :match (p ?x) :assert (q {S})) (solve :hrules (h))', 'solve'),
    # rule keywords
    'rule :why':                ('(rule rr () :match (p ?x) :assert (hit ?x) :why {S})', 'derived hit'),
    'rule :priority':           ('(rule rr () :match (p ?x) :assert (hit ?x) :priority {S})', 'derived hit'),
    # queries
    ':goal':                    ('(query :goal {S})', 'query'),
    'goal argument':            ('(query :goal (p {S}))', 'query'),
    ':goal-text':               ('(query :goal (p ?x) :goal-text {S})', 'query'),
    ':over':                    ('(query :goal (p ?x) :over {S})', 'query'),
    'claim fact argument':      ('(query :goal (p ?x) :expect (model (p a) (p {S})))', 'test'),
    # the lattice, config, imports, macros, trace
    ':no-hypothesis item':      ('(solve :no-hypothesis ({S}))', 'solve'),
    ':hrules item':             ('(hrule h () :match (p ?x) :assert (q ?x)) (solve :hrules (h {S}))', 'solve'),
    'config value':             ('(config :record-alternative-justifications {S})', 'facts p'),
    'import module':            ('(import {S})', 'facts p'),
    'import :symbols item':     ('(import std.macro :symbols ({S}))', 'facts p'),
    'import :as':               ('(import std.macro :as {S})', 'facts p'),
    'macro body':               ('(macro mm (?x) {S})', 'facts p'),
    'trace :derives':           ('(trace (step s1 :rule x :derives {S}))', 'facts p'),
}
# a keyword this form does not take, at each place a keyword can be written
UNKNOWN_KW = {
    'relation':        '(relation r2 T :zz 1)',
    'rule':            '(rule rr () :match (p ?x) :assert (hit ?x) :zz 1)',
    'hrule':           '(hrule h () :match (p ?x) :assert (q ?x) :zz 1)',
    'fact':            '(q c :zz 1)',
    'nested fact':     '(r (f a :zz 1) c)',
    'stored negation': '(not (q c) :zz 1)',
    'query':           '(query :goal (p ?x) :zz 1)',
    'solve':           '(solve :zz 1)',
    'config':          '(config :zz 1)',
    'import':          '(import std.macro :zz 1)',
    'trace step':      '(trace (step s1 :rule x :zz 1))',
    'pattern in :match': '(rule rr () :match (p ?x :zz 1) :assert (hit ?x))',
    'and in :match':   '(rule rr () :match (and (p ?x) :zz 1) :assert (hit ?x))',
    'pattern in :assert': '(rule rr () :match (p ?x) :assert (hit ?x :zz 1))',
    'and in :assert':  '(rule rr () :match (p ?x) :assert (and (hit ?x) :zz 1))',
    ':goal pattern':   '(query :goal (p ?x :zz 1))',
    ':using of a fact': '(q c :rule x :using (u (r a b :zz 1)))',
    'query :mode':     '(query :goal (p ?x) :mode x)',
    'relation :cardinality': '(relation r2 T :cardinality 1..1)',
    'fact :why':       '(q c :why "text")',
    'rule :where':     '(rule rr () :match (and (p ?x) :where (neq ?x a)) :assert (hit ?x))',
}

def run(cmd, path, mode, grm):
    env = dict(ENV)
    if grm: env['EIN_PARSER'] = 'grm'
    args = {'facts': ['saturate', path, '--dump'], 'derived': ['saturate', path, '--dump'],
            'relations': ['saturate', path, '--dump'], 'query': ['solve', path],
            'solve': ['solve', path], 'test': ['test', path]}[mode]
    try:
        r = subprocess.run(cmd + args, capture_output=True, text=True, timeout=30, env=env)
    except subprocess.TimeoutExpired:
        return 'timeout', ''
    return r.returncode, r.stdout + r.stderr

def classify(code, out, how):
    mode, _, rel = how.partition(' ')
    if re.search(r':-?\d+:-?\d+: unexpected input', out):
        return 'parse error'
    m = re.search(r'kb load error: (.*)', out)
    if m: return 'load error: ' + m.group(1)[:200]
    for pat in (r'saturate failed[^\n]*\n?([^\n]*)', r'^error: ([^\n]*)', r'Error: ([^\n]*)', r'(KeyError[^\n]*)', r'(CompileError[^\n]*)', r'compile error: ([^\n]*)'):
        m = re.search(pat, out, re.M)
        if m and code not in (0,):
            return 'run error: ' + m.group(1)[:200]
    if code not in (0,) and mode != 'test':
        tail = [l for l in out.splitlines() if l.strip()][-1:] or ['']
        return f'exit {code}: ' + tail[0][:200]
    if mode in ('facts', 'derived', 'relations'):
        dump = out.split('SATURATED KB DUMP', 1)[-1]
        if mode == 'relations':
            decl = re.findall(r'^\(relation [^\n]*\)$', dump, re.M)
            viv = re.search(r';; Relations — auto-vivified \(\d+\)\n((?:;;   [^\n]*\n)*)', dump)
            return 'loads: declares ' + ' '.join(d for d in decl if 'r2' in d or not re.match(r'\(relation (p|q|hit|r) ', d)) + (' · vivifies ' + viv.group(1).replace(';;   ', '').strip() if viv else '')
        facts = re.findall(r'^  (\(.*\))$', dump, re.M)
        if mode == 'facts':
            base = {'(p a)', '(p b)', '(q a)'}
            sel = [f for f in facts if not f.startswith('(relation') and f not in base and (rel == '*' or f.startswith('(' + rel + ' ') or f.startswith('(' + rel + ')'))]
            return 'loads: stores ' + (' '.join(sel) if sel else 'nothing new')
        sec = dump.split('── DERIVED', 1)
        der = re.findall(r'^  (\(.*\))$', sec[1], re.M) if len(sec) > 1 else []
        der = [d for d in der if rel == '*' or d.startswith('(' + rel)]
        return 'loads: derives ' + (' '.join(der) if der else 'nothing')
    if mode in ('query', 'solve'):
        lines = out.splitlines()
        keep = [l.strip() for l in lines if l.strip().startswith(('as :expect', '?', 'states (k_s)', 'the goal', 'universe', 'note:'))]
        return 'loads: ' + ' · '.join(keep)[:220]
    if mode == 'test':
        first = [l for l in out.splitlines() if l.strip()][:1]
        return f'exit {code}: ' + (first[0][:200] if first else '')
    return f'exit {code}'

def main():
    rows = []
    with tempfile.TemporaryDirectory(dir=os.path.join(ROOT, 'tmp')) as td:
        def probe(kind, pos, shape, text, how):
            path = os.path.join(td, 'p.ein')
            open(path, 'w').write(BASE + text + '\n')
            res = {}
            for nm, cmd, g in (('today', TODAY, False), ('grammar', GRM, True)):
                code, out = run(cmd, path, how.split()[0], g)
                res[nm] = classify(code, out, how)
            rows.append({'kind': kind, 'position': pos, 'shape': shape, 'text': text, 'template': tmpl_now[0], **res})
        tmpl_now = ['']
        for pos, (tmpl, how) in POS.items():
            tmpl_now[0] = tmpl
            for sh, s in SHAPES.items():
                probe('position', pos, sh, tmpl.replace('{S}', s), how)
        for form, text in UNKNOWN_KW.items():
            tmpl_now[0] = text
            probe('keyword', form, ':zz', text, 'facts *' if 'query' not in form else 'query')
    json.dump(rows, open(sys.argv[1], 'w'), indent=1, ensure_ascii=False)
    print(len(rows), 'probes')

main()
