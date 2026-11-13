;; vocabulary.rktd — every word the pipeline accepts, and what kind of word it is.
;;
;; A form is (kind tag word ...): the first two say what the words are, the rest
;; are the words. A name of several words is a list. Both programs read this one
;; file: nl2smt.py refuses a sentence with a word that is not here, and
;; lg2smt.rkt reads the kinds and the tags.

;; The puzzle's things. Each becomes an SMT constant, tag-word, whose value is
;; the number of its house; the things of one tag are in five different houses.
(value nation  Englishman Spaniard Ukrainian Norwegian Japanese)
(value colour  red green ivory yellow blue)
(value pet     dog snails fox horse zebra)
(value drink   coffee tea milk (orange juice) water)
(value smoke   Kools Chesterfields (Old Gold) (Lucky Strike) Parliaments)

;; A verb, and the kind of thing it is said of: one owns a pet, drinks a drink.
(verb pet    owns kept)
(verb drink  drinks drunk)
(verb smoke  smokes smoked)
(verb house  lives)

;; A noun that a value beside it names: the red house, the Kools smoker.
(noun colour house)
(noun smoke  smoker)
(noun nation man)

;; A house by its position, the tag its number; the middle one's depends on
;; how many there are.
(position 1      first)
(position middle middle)

;; How many houses. The puzzle says five; four and six are not its words, and
;; are here so that the line that counts the houses can be changed.
(number 5 five)
(number 4 four)
(number 6 six)

;; A question word, and the kind of thing it asks for.
(asks nation who)

;; The words the sentence shapes are made of.
(grammar - the is are there houses in with next to immediately right of where)
