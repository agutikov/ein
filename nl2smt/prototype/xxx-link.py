#!/usr/bin/env python3.6


from linkgrammar import Sentence, ParseOptions, Dictionary, Clinkgrammar as clg
from pprint import pprint
import nltk


text = """
    The man who drinks milk lives in the middle house.
    The Kools smoker owns snails.
    The man who smokes Chesterfields lives in the house next to the man with the fox.

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


    The man who smokes Chesterfields lives in the house next to the man with the fox.

    Man smokes cigarette.

    Who drinks water? Who owns the zebra?

"""


sentences = text.split('.')

statements = []
questions = []

def uncapitalize(s):
    return s[0].lower() + s[1:]

for s in sentences:
    if '?' in s:
        a = [ss.strip() for ss in s.split('?')]
        for q in a[:-1]:
            questions.append(q)
        if len(a[-1]) > 0:
            statements.append(uncapitalize(a[-1]))
    else:
        statements.append(uncapitalize(s.strip()))


#x = 'coffee is drunk in the green house'
#x = 'the green house is immediately to the right of the ivory house'


po = ParseOptions(verbosity=0, display_morphology=True)

en_dir = Dictionary()

def word(w):
    if '.' in w:
        a = w.split('.')
        p = a[1]
        w = a[0]
        if p in 'vn':
            ww = nltk.stem.WordNetLemmatizer().lemmatize(w, p)
            return '({} {} {})'.format(p, ww, w)
        else:
            return '({} {})'.format(p, w)
    else:
        return w

def short_word(w):
    return w.split('.')[0]

def link(lnk):
    return '(({} {}) ({} {}))'.format(
        lnk.left_label, word(lnk.left_word),
        lnk.right_label, word(lnk.right_word))

def short_link(lnk):
    return '({} {} {})'.format(
        lnk.left_label[0],
        short_word(lnk.left_word),
        short_word(lnk.right_word))

def linkage(lkg):
    a = []
    for lnk in lkg.links():
        if lnk.left_word.lower() == 'the' or lnk.left_word.lower() == 'a':
            continue
        if lnk.left_word == 'LEFT-WALL' or lnk.right_word == 'RIGHT-WALL':
            continue
        a.append(link(lnk))
    return '\n'.join(a)



"""
sent = Sentence(x, en_dir, po)
linkages = sent.parse()

d2 = set()

for lkg in linkages:
    #print(lkg.diagram())
    print(lkg.constituent_tree())
    l2 = linkage(lkg)
    #print(l2)
    d2.add(l2)

print()
for d in d2:
    print(d)

quit()
"""



def parse_stmt(stmt):
    print("="*80)
    print(s)
    sent = Sentence(s, en_dir, po)
    linkages = sent.parse()
    for lkg in sorted(linkages, key=lambda x: x.link_cost()):
        print()
        print(lkg.link_cost())
        print(lkg.disjunct_cost())
        print(lkg.diagram())
        print(lkg.constituent_tree())
        print(linkage(lkg))
        print()
        print([w for w in lkg.words()])
        break




s = 'the man who smokes Chesterfields lives in the house next to the man with the fox'
#parse_stmt(s)

#exit(0)

for s in statements:
    parse_stmt(s)


'''
    Очистить и получить дерево основных связей что-то между diagram и constituent_tree
     - инверсия (is drunk)
     - существительное = субъект+действие (smoker)

    Построить semantic net из двух частей:
     - модель домена
        man can smoke,drink,live,own; live in house; house can be green,red
        - классы объектов
        - свойства объектов
        - классы свойств
        - субъект-действие-объект
          - без объекта
          - без субъекта
        - свойство действия
        - классы действий
        - место действия
          - локальность действия
        - действие внутри-снаружи объекта
        - относительное статическое расположение
          - одна координата
        - планы на будущее
          - несколько координат
            - расстояние
          - часть-целое
          - группа объектов
          - количество
          - события вместо действий
            - причина-следствие
          - время
            - время как последовательность событий (состояний)
            - время как координата
     - модель ситуации
        first man lives in green house, second man owns snails

    По модели домена определить SMT теорию
    По модели ситуации сгенерить условия

    Преобразование ошибки в запрос на уточнение
     - ошибка link-linkgrammar
     - ошибка семантической модели
        - противоречие
     - ошибка SMT решателя

    Запрос дополнительной информации: Is Norwegian a house?


'''
