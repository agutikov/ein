
(set-logic QF_LIA)

(define-fun != ((x Int) (y Int)) Bool (not (= x y)))

(declare-fun q1 () Int)
(declare-fun q2 () Int)
(declare-fun q3 () Int)
(declare-fun q4 () Int)

(assert (>= q1 1))
(assert (<= q1 4))
(assert (>= q2 1))
(assert (<= q2 4))
(assert (>= q3 1))
(assert (<= q3 4))
(assert (>= q4 1))
(assert (<= q4 4))

(assert (!= q1 q2))
(assert (!= q1 q3))
(assert (!= q1 q4))
(assert (!= q2 q3))
(assert (!= q2 q4))
(assert (!= q3 q4))

(assert (!= q1 (+ q2 1)))
(assert (!= q1 (+ q3 2)))
(assert (!= q1 (+ q4 3)))
(assert (!= q2 (+ q3 1)))
(assert (!= q2 (+ q4 2)))
(assert (!= q3 (+ q4 1)))

(assert (!= q1 (- q2 1)))
(assert (!= q1 (- q3 2)))
(assert (!= q1 (- q4 3)))
(assert (!= q2 (- q3 1)))
(assert (!= q2 (- q4 2)))
(assert (!= q3 (- q4 1)))

(check-sat)

(get-value (q1 q2 q3 q4))
