#!/usr/bin/env python3

import sys
from copy import deepcopy
from pprint import pprint
import re
import numpy as np
import zlib


def random_dot_color():
    color = list(np.random.choice(range(256), size=3))
    return '#' + ''.join(['%02X' % c for c in color])


def hash_color(s):
    crc = zlib.crc32(s.encode('utf-8'))
    return '#%06X' % (crc & 0xFFFFFF)


def str_comparator(pattern):
    if pattern is None:
        return lambda x: True
    elif isinstance(pattern, str):
        r = re.compile(pattern)
        return lambda x: r.search(x) is not None
    elif isinstance(pattern, list):
        str_set = set(pattern)
        return lambda x: x in str_set
    else:
        raise Exception(f'Ivalid string comparator pattern: "{pattern}"')


class state:

    def __init__(self):

        # relation name -> dict of src object names -> set of dst object names
        self.relations = {}

        # src object name -> dict of relation name -> set of dst object names
        self.objects = {}


    #TODO: Replace set of dst object names with single object name -> automatic constraint checking during adding of new relation
    def is_consistent(self):
        # Debug method.
        # Verify that relations and objects are consistent.
        pass #TODO


    def dump(self):
        s = ''
        for obj_name in self.objects:
            s += obj_name + '\n'

        for rel_name, rel in self.relations.items():
            for src_name, dst_set in rel.items():
                for dst_name in dst_set:
                    s += src_name + ' ' + rel_name + ' ' + dst_name + '\n'
        return s


    def dot(self, colorfull=True):
        dot = 'digraph G {\n'
        for obj_name in self.objects:
            dot += f'  {obj_name};\n'

        colors = {rel_name: hash_color(rel_name) for rel_name in self.relations}

        for rel_name, rel in self.relations.items():
            for src_name, dst_set in rel.items():
                for dst_name in dst_set:
                    if colorfull:
                        color = f'"{colors[rel_name]}"'
                        label = f'<<font color={color}>{rel_name}</font>>'
                        dot += f'  {src_name} -> {dst_name} [label={label} color={color}];\n'
                    else:
                        dot += f'  {src_name} -> {dst_name} [label="{rel_name}"];\n'
        dot += '}\n'
        return dot


    def show(self):
        pass #TODO: networkx visualisation


    def loadf(self, filename):
        with open(filename, 'r') as f:
            lines = f.readlines()
        self.load(lines)


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


    def select_obj(self, obj_pattern, exclude=False):
        #TODO: select objects with relations
        pass


    def select_rel(self, rel_pattern, exclude=False, preserve_objs=False):
        comp = str_comparator(rel_pattern)
        s = state()

        for rel_name, rel in self.relations.items():
            if comp(rel_name) != exclude:
                for src_name, dst_set in rel.items():
                    for dst_name in dst_set:
                        s.rel(src_name, rel_name, dst_name)

        if preserve_objs:
            for obj in self.objects:
                s.obj(obj)

        return s


    def ends(self, rel_selector):
        s = self.select_rel(rel_selector)
        return [obj_name for obj_name, obj in s.objects.items() if len(obj) == 0]

    
    def obj_type(self, obj_name, type_rel_name):
        types = self.objects[obj_name].get(type_rel_name, set())
        if len(types) == 1:
            return next(iter(types))
        return None


    def rel_types(self, type_rel_name):
        types = {}
        for rel_name, rel in self.relations.items():
            if rel_name != type_rel_name:
                for src_name, dst_set in rel.items():
                    for dst_name in dst_set:
                        src_type = self.obj_type(src_name, type_rel_name)
                        dst_type = self.obj_type(dst_name, type_rel_name)
                        if dst_type is not None and src_type is not None:
                            rt = types.setdefault(rel_name, set())
                            rt.add((src_type, dst_type))
        return types


    def verify_single_rel_constraint(self, rel_pattern=None):
        # Verify single relation constraint.
        # Return dict of object -> relations that violate the rule.
        comp = str_comparator(rel_pattern)
        violations = {}
        for obj_name, obj in self.objects.items():
            for rel_name, rel in obj.items():
                if comp(rel_name) and len(rel) > 1:
                    violations.setdefault(obj_name, {})[rel_name] = deepcopy(rel)
        return violations





s = state()

s.loadf(sys.argv[1])

#print(s.dump())
#pprint(s.objects)
#pprint(s.relations)
#print(s.select_rel(" in| in", False, False).dot())
#print(s.dot())

#print(s.ends(['is']))
#print(s.ends(['to the left of']))
#print(s.ends(['to the right of']))
#print(s.ends(['to the right of', 'to the left of']))

#pprint(s.rel_types('is'))
#pprint(s.verify_single_rel_constraint('next'))




#TODO:
# 4) state versioning
# 5) inference
# 6) hypothesis: generate, test, verify, multilevel
