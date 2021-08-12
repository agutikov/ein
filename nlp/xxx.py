#!/usr/bin/env python3

import nltk
from nltk.corpus import treebank
from nltk.draw.util import CanvasFrame
from nltk.draw import TreeWidget

from pprint import pprint





text = """
    На улице стоят пять домов.
    Англичанин живёт в красном доме.
    У испанца есть собака.
    В зелёном доме пьют кофе.
    Украинец пьёт чай.
    Зелёный дом стоит сразу справа от белого дома.
    Тот, кто курит Old Gold, разводит улиток.
    В жёлтом доме курят Kool.
    В центральном доме пьют молоко.
    Норвежец живёт в первом доме.
    Сосед того, кто курит Chesterfield, держит лису.
    В доме по соседству с тем, в котором держат лошадь, курят Kool.
    Тот, кто курит Lucky Strike, пьёт апельсиновый сок.
    Японец курит Parliament.
    Норвежец живёт рядом с синим домом.

    Кто пьёт воду? Кто держит зебру?
"""


text = """
    There are five houses.
    The Englishman lives in the red house.
    The Spaniard owns the dog.
    Coffee is drunk in the green house.
    The Ukrainian drinks tea.
    The green house is immediately to the right of the ivory house.
    The Old Gold smoker owns snails.
    Kools are smoked in the yellow house.
    Milk is drunk in the middle house.
    The Norwegian lives in the first house.
    The man who smokes Chesterfields lives in the house next to the man with the fox.
    Kools are smoked in the house next to the house where the horse is kept.
    The Lucky Strike smoker drinks orange juice.
    The Japanese smokes Parliaments.
    The Norwegian lives next to the blue house.

    Man smokes cigarette.

    Who drinks water? Who owns the zebra?

"""



sentences = text.split('.')

statements = []
questions = []

for s in sentences:
    if '?' in s:
        a = [ss.strip() for ss in s.split('?')]
        for q in a[:-1]:
            questions.append(q)
        if len(a[-1]) > 0:
            statements.append(a[-1])
    else:
        statements.append(s.strip())


g1 = nltk.CFG.fromstring("""
    S -> NP VP
    PP -> P NP
    NP -> Det N | Det N PP | 'I'
    VP -> V NP | VP PP
    Det -> 'an' | 'my'
    N -> 'elephant' | 'pajamas'
    V -> 'shot'
    P -> 'in'
""")

sent = ['I', 'shot', 'an', 'elephant', 'in', 'my', 'pajamas']
parser = nltk.ChartParser(g1)

for tree in parser.parse(sent):
    print(tree)

nltk.app.rdparser()

exit()



for s in statements:
    tokens = nltk.word_tokenize(s)
    pprint(tokens)
    tagged = nltk.pos_tag(tokens)
    pprint(tagged)
    tree = nltk.chunk.ne_chunk(tagged)
    pprint(tree)
    #tree.draw()
    """
    cf = CanvasFrame()
    tc = TreeWidget(cf.canvas(), tree)
    cf.add_widget(tc,10,10) # (10,10) offsets
    cf.print_to_file(s.lower().replace(' ', '_') + '.ps')
    cf.destroy()
    """

for q in questions:
    tokens = nltk.word_tokenize(q)

