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


def snapshot(s):
    return state(s)


class state:

    def __init__(self, base=None):
        self._base = base

        # relation name -> dict of src object names -> set of dst object names
        self._relations = {}

        # src object name -> dict of relation name -> set of dst object names
        self._objects = {}


    def objects(self):
        if self._base is not None:
            yield from self._base.objects()

        for obj_name, obj in self._objects.items():
            yield obj_name, obj


    def relations(self):
        if self._base is not None:
            yield from self._base.relations()

        for rel_name, rel in self._relations.items():
            yield rel_name, rel


    def get_obj(self, name):
        s = self
        while s is not None:
            obj = s._objects.get(name, None)
            if obj is not None:
                return obj
            s = s._base


    def get_rel(self, name):
        s = self
        while s is not None:
            rel = s._relations.get(name, None)
            if rel is not None:
                return rel
            s = s._base


    def flatten(self):
        s = state()

        for obj_name, _ in self.objects():
            s.obj(obj_name)

        for rel_name, rel in self.relations():
            for src_name, dst_set in rel.items():
                for dst_name in dst_set:
                    s.rel(src_name, rel_name, dst_name)

        return s


    #TODO: Replace set of dst object names with single object name -> automatic constraint checking during adding of new relation
    def is_consistent(self):
        # Debug method.
        # Verify that relations and objects are consistent.
        pass #TODO


    def dump(self, level=0):
        s = ''

        if self._base is not None:
            s = self._base.dump(level-1)

        s += f'\n# Level {level}\n\n'

        for obj_name in self._objects:
            s += obj_name + '\n'

        for rel_name, rel in self._relations.items():
            for src_name, dst_set in rel.items():
                for dst_name in dst_set:
                    s += src_name + ' ' + rel_name + ' ' + dst_name + '\n'
        return s


    def dot(self, colorfull=True):
        s = self.flatten()

        dot = 'digraph G {\n'
        for obj_name in s._objects:
            dot += f'  {obj_name};\n'

        colors = {rel_name: hash_color(rel_name) for rel_name in s._relations}

        for rel_name, rel in s._relations.items():
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
        # Add/Insert new Object

        return self._objects.setdefault(name, {})


    def rel(self, src, rel, dst):
        # Add new Relation

        r = self._relations.setdefault(rel, {})
        r.setdefault(src, set()).add(dst)
        
        s = self._objects.setdefault(src, {})
        s.setdefault(rel, set()).add(dst)
        
        d = self._objects.setdefault(dst, {})

        return s, r, d


    def select_obj(self, obj_pattern, exclude=False):
        #TODO: select objects with relations
        pass


    def select_rel(self, rel_pattern, exclude=False, preserve_objs=False):
        comp = str_comparator(rel_pattern)
        s = state()

        for rel_name, rel in self.relations():
            if comp(rel_name) != exclude:
                for src_name, dst_set in rel.items():
                    for dst_name in dst_set:
                        s.rel(src_name, rel_name, dst_name)

        if preserve_objs:
            for obj_name, _ in self.objects():
                s.obj(obj_name)

        return s


    def ends(self, rel_selector):
        s = self.select_rel(rel_selector)
        return [obj_name for obj_name, obj in s._objects.items() if len(obj) == 0]


    def obj_type(self, obj_name, type_rel_name):
        obj = self.get_obj(obj_name)
        if obj is None:
            return None
        types = obj.get(type_rel_name, set())
        if len(types) == 1:
            return next(iter(types))
        return None


    def rel_types(self, type_rel_name):
        types = {}
        for rel_name, rel in self.relations():
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
        for obj_name, obj in self.flatten()._objects.items():
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


s = snapshot(s)

s.rel('Snail', 'is', 'Tea')


print(s.dot())




#TODO:
# 3) generate pics in README from text files
# 4) next step in README - replace different relation types with just 'is' - look at object 'Snail' as 'House with Snail' - triangle rule become trivial
# 4.1) reformulate constraints with End objects and 'is' relation
# 4.2) restrict adding of any relation to End object - keep them ends
# 4.3) all operations are with 'is' relations
# 4.4) do smoething with left|right and next relation and square rule
# 5) TESTS (with versioning)
# 6) inference
# 7) hypothesis: generate, test, verify, multilevel
# 8) delete obj or relation and versioning
