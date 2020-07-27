#!/usr/bin/env python3

import sys
from pprint import pprint



class state:
    def __init__(self):

        # relation name -> dict of src object names -> set of dst object names
        self.relations = {}

        # object name -> dict of relation name -> set of object names
        self.objects = {}

    def dump(self):
        s = ''
        for obj_name in self.objects:
            s += obj_name + '\n'

        for rel_name, rel in self.relations.items():
            for src_name, dst_set in rel.items():
                for dst_name in dst_set:
                    s += src_name + ' ' + rel_name + ' ' + dst_name + '\n'
        return s

    def dot(self):
        pass #TODO: dump state in dot

    def load(self, lines):
        for line in lines:
            v = line.strip().split()
            if len(v) == 0:
                continue
            if len(v) == 1:
                self.obj(line)
            elif len(v) >= 3:
                self.rel(v[0], ' '.join(v[1:-1]), v[-1])
            else:
                raise Exception(f'Invalid line: "{line}"')

    def obj(self, name):
        return self.objects.setdefault(name, {})

    def rel(self, src, rel, dst):
        r = self.relations.setdefault(rel, {})
        r.setdefault(src, set()).add(dst)
        
        s = self.objects.setdefault(src, {})
        s.setdefault(rel, set()).add(dst)
        
        d = self.objects.setdefault(dst, {})

        return s, r, d



with open(sys.argv[1], 'r') as f:
    lines = f.readlines()

s = state()

s.load(lines)

print(s.dump())

pprint(s.objects)
pprint(s.relations)


#TODO:
# 1) obtain types (end nodes)
# 2) obtain types of relations (src type + dst type)
# 3) constraint verification 
# 4) inference
# 5) hypothesis: generate, test, verify, multilevel
