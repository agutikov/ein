#!/usr/bin/env python3

import sys
import re
import urllib
import commonmark

# Gravizo not works with Github
# https://gist.github.com/svenevs/ce05761128e240e27883e3372ccd4ecd


parser = commonmark.Parser()
with open(sys.argv[1], 'r') as f:
    ast = parser.parse(f.read())

#print(commonmark.dumpJSON(ast))

#commonmark.dumpAST(ast)

def iteritems(ast):
    node = ast.first_child
    while node is not None:
        yield node
        node = node.nxt

def is_html_comment(s):
    return s[0:4] == '<!--' and s[-3:] == '-->'

ed = re.compile('digraph .*\}', flags=re.DOTALL)

def extract_digraph(s):
    m = ed.search(s)
    if m is None:
        return
    return str(m.group())

for node in iteritems(ast):
    if node.t == 'html_block' and is_html_comment(node.literal):
        raw = extract_digraph(node.literal)
        if raw is None:
            continue
        print(raw)
        print()
        print(urllib.parse.quote(raw.encode('utf-8')))
        print()
