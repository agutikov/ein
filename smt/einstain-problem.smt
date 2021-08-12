

; (set house (red white blue yellow green))
; (set nation (england spain ukrain japan norway))
; (set pet (snail dog horse zebra fox))
; (set drink (milk water tee coffee juice))
; (set smoke (parlament luckystrike chesterfield kool oldgold))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(set-logic QF_LIA)

(define-fun != ((x Int) (y Int)) Bool (not (= x y)))
(define-fun order ((x Int) (y Int)) Bool (= (+ x 1) y))
(define-fun near ((x Int) (y Int)) Bool (or (= (+ x 1) y) (= (- x 1) y)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(declare-fun house-red () Int)
(declare-fun house-white () Int)
(declare-fun house-blue () Int)
(declare-fun house-yellow () Int)
(declare-fun house-green () Int)

(declare-fun nation-england () Int)
(declare-fun nation-spain () Int)
(declare-fun nation-ukrain () Int)
(declare-fun nation-japan () Int)
(declare-fun nation-norway () Int)

(declare-fun pet-snail () Int)
(declare-fun pet-dog () Int)
(declare-fun pet-horse () Int)
(declare-fun pet-zebra () Int)
(declare-fun pet-fox () Int)

(declare-fun drink-milk () Int)
(declare-fun drink-water () Int)
(declare-fun drink-tee () Int)
(declare-fun drink-coffee () Int)
(declare-fun drink-juice () Int)

(declare-fun smoke-parlament () Int)
(declare-fun smoke-luckystrike () Int)
(declare-fun smoke-chesterfield () Int)
(declare-fun smoke-kool () Int)
(declare-fun smoke-oldgold () Int)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;


(assert (>= house-red 1))
(assert (>= house-white 1))
(assert (>= house-blue 1))
(assert (>= house-yellow 1))
(assert (>= house-green 1))

(assert (>= nation-england 1))
(assert (>= nation-spain 1))
(assert (>= nation-ukrain 1))
(assert (>= nation-japan 1))
(assert (>= nation-norway 1))

(assert (>= pet-snail 1))
(assert (>= pet-dog 1))
(assert (>= pet-horse 1))
(assert (>= pet-zebra 1))
(assert (>= pet-fox 1))

(assert (>= drink-milk 1))
(assert (>= drink-water 1))
(assert (>= drink-tee 1))
(assert (>= drink-coffee 1))
(assert (>= drink-juice 1))

(assert (>= smoke-parlament 1))
(assert (>= smoke-luckystrike 1))
(assert (>= smoke-chesterfield 1))
(assert (>= smoke-kool 1))
(assert (>= smoke-oldgold 1))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assert (<= house-red 5))
(assert (<= house-white 5))
(assert (<= house-blue 5))
(assert (<= house-yellow 5))
(assert (<= house-green 5))

(assert (<= nation-england 5))
(assert (<= nation-spain 5))
(assert (<= nation-ukrain 5))
(assert (<= nation-japan 5))
(assert (<= nation-norway 5))

(assert (<= pet-snail 5))
(assert (<= pet-dog 5))
(assert (<= pet-horse 5))
(assert (<= pet-zebra 5))
(assert (<= pet-fox 5))

(assert (<= drink-milk 5))
(assert (<= drink-water 5))
(assert (<= drink-tee 5))
(assert (<= drink-coffee 5))
(assert (<= drink-juice 5))

(assert (<= smoke-parlament 5))
(assert (<= smoke-luckystrike 5))
(assert (<= smoke-chesterfield 5))
(assert (<= smoke-kool 5))
(assert (<= smoke-oldgold 5))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assert (!= house-red house-white))
(assert (!= house-red house-blue))
(assert (!= house-red house-yellow))
(assert (!= house-red house-green))
(assert (!= house-white house-blue))
(assert (!= house-white house-yellow))
(assert (!= house-white house-green))
(assert (!= house-blue house-yellow))
(assert (!= house-blue house-green))
(assert (!= house-yellow house-green))

(assert (!= nation-england nation-spain))
(assert (!= nation-england nation-ukrain))
(assert (!= nation-england nation-japan))
(assert (!= nation-england nation-norway))
(assert (!= nation-spain nation-ukrain))
(assert (!= nation-spain nation-japan))
(assert (!= nation-spain nation-norway))
(assert (!= nation-ukrain nation-japan))
(assert (!= nation-ukrain nation-norway))
(assert (!= nation-japan nation-norway))

(assert (!= pet-snail pet-dog))
(assert (!= pet-snail pet-horse))
(assert (!= pet-snail pet-zebra))
(assert (!= pet-snail pet-fox))
(assert (!= pet-dog pet-horse))
(assert (!= pet-dog pet-zebra))
(assert (!= pet-dog pet-fox))
(assert (!= pet-horse pet-zebra))
(assert (!= pet-horse pet-fox))
(assert (!= pet-zebra pet-fox))

(assert (!= drink-milk drink-water))
(assert (!= drink-milk drink-tee))
(assert (!= drink-milk drink-coffee))
(assert (!= drink-milk drink-juice))
(assert (!= drink-water drink-tee))
(assert (!= drink-water drink-coffee))
(assert (!= drink-water drink-juice))
(assert (!= drink-tee drink-coffee))
(assert (!= drink-tee drink-juice))
(assert (!= drink-coffee drink-juice))

(assert (!= smoke-parlament smoke-luckystrike))
(assert (!= smoke-parlament smoke-chesterfield))
(assert (!= smoke-parlament smoke-kool))
(assert (!= smoke-parlament smoke-oldgold))
(assert (!= smoke-luckystrike smoke-chesterfield))
(assert (!= smoke-luckystrike smoke-kool))
(assert (!= smoke-luckystrike smoke-oldgold))
(assert (!= smoke-chesterfield smoke-kool))
(assert (!= smoke-chesterfield smoke-oldgold))
(assert (!= smoke-kool smoke-oldgold))


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assert (= house-red nation-england))
(assert (= nation-spain pet-dog))
(assert (= house-green drink-coffee))
(assert (= nation-ukrain drink-tee))
(assert (order house-white house-green))
(assert (= smoke-oldgold pet-snail))
(assert (= house-yellow smoke-kool))
(assert (= drink-milk 3))
(assert (= nation-norway 1))
(assert (near smoke-chesterfield pet-fox))
(assert (near pet-horse smoke-kool))
(assert (= smoke-luckystrike drink-juice))
(assert (= nation-japan smoke-parlament))
(assert (near nation-norway house-blue))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(check-sat)

(get-value (pet-zebra drink-water))
(get-value (house-red house-white house-green house-blue house-yellow))
