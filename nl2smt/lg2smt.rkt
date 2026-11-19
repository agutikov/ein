#lang racket
;; lg2smt.rkt — link grammar's linkages of the Zebra puzzle's sentences, to SMT-LIB code.
;;
;;   racket lg2smt.rkt [--readings] VOCABULARY [MODEL ...] < linkages
;;
;; stdin       for each sentence a line "line N: <the sentence>", then what link-parser
;;             printed for it under -postscript=1: its linkages, the cheapest first
;; VOCABULARY  vocabulary.rktd
;; MODEL       a solution the solver has printed, ((constant value) ...): it is ruled out
;; stdout      the SMT-LIB script; under --readings, what each linkage of each sentence says
;;
;; One direction, and no more English than the puzzle's clues need. A sentence is read
;; off the first of its linkages that has a shape known here. The command line, the
;; vocabulary and the script are script.rkt's, which nltk2smt.rkt shares; what is here is
;; how a linkage is read.

(require "script.rkt")

;; ---- link-parser's output

(struct linkage (words unlinked links))    ; words by position; links (left right label)

(define (read-linkages lines)   ; the lines under a sentence's header: its linkages
  (filter-map read-linkage (headed lines #px"^\\s*Linkage \\d+")))

(define (read-linkage block)    ; [(word)(word)...][[left right height (label)]...][0]
  (match (regexp-match #rx"\\[\\((.*)\\)\\](\\[\\[.*\\]\\])\\[" (string-append* (rest block)))
    [(list _ words links)
     (define tokens (string-split words ")("))
     (define names (list->vector (map bare tokens)))
     (join-names! names)
     (linkage names
              (for/list ([token tokens] [i (in-naturals)]      ; [Old]: a word linked to nothing
                         #:when (and (string-prefix? token "[") (vector-ref names i)))
                i)
              (for/list ([link (read (open-input-string links))]
                         #:when (and (vector-ref names (first link)) (vector-ref names (second link))))
                (list (first link) (second link) (~a (first (fourth link))))))]
    [_ #f]))

(define (bare token)            ; lives.v → lives, Englishman[!] → Englishman, [Old] → Old
  (match (regexp-match #rx"[^].[]+" token) [(list word) word] [_ token]))

(define (join-names! words)     ; a name of several words becomes one word, where its last stood
  (for* ([form vocabulary] [name (cddr form)] #:when (list? name)
         [start (in-range (- (vector-length words) (length name) -1))]
         #:when (for/and ([part name] [i (in-naturals start)])
                  (and (vector-ref words i) (string-ci=? (~a part) (vector-ref words i)))))
    (define end (+ start (length name) -1))
    (for ([i (in-range start end)]) (vector-set! words i #f))
    (vector-set! words end (spelling name))))

;; ---- what a linkage says

(define here (make-parameter #f))                   ; the linkage being read
(define (word i) (and (< -1 i (vector-length (linkage-words (here)))) (vector-ref (linkage-words (here)) i)))
(define (word? i . words) (and (word i) (member (string-downcase (word i)) words) #t))

(define (of-type? label type)   ; a label is a type, then subscripts: Ss*s is an S, SIpx is not
  (regexp-match? (pregexp (string-append "^" type "([^A-Z]|$)")) label))
(define (right i type)          ; the words that the word at i is linked to on its right
  (for/list ([l (linkage-links (here))] #:when (and (= (first l) i) (of-type? (third l) type))) (second l)))
(define (left i type)
  (for/list ([l (linkage-links (here))] #:when (and (= (second l) i) (of-type? (third l) type))) (first l)))
(define (linked? i j) (for/or ([l (linkage-links (here))]) (and (= (first l) i) (= (second l) j))))

;; A noun, or a preposition with its noun, says which house:
;;   (at x)         the house of x, a constant or a house's number
;;   (near x)       a house next to it
;;   (right x)      the house immediately to its right
;;   (rightward x)  a house somewhere to its right

(define (thing n)               ; the noun at n
  (define beside (append (left n "A") (left n "AN")                             ; red house, Kools smoker
                         (append-map (λ (the) (right the "L")) (left n "D"))))  ; the first house: the –L– first
  (cond
    [(known 'value (word n))                                 ; the dog, coffee, the Englishman
     => (λ (value)
          (unless (null? beside) (fail "~s beside ~s" (word (first beside)) (word n)))
          `(at ,(constant value)))]
    [(known 'noun (word n))                                  ; house, smoker, man
     => (λ (noun)
          (match* (beside (right n "M") (right n "B"))
            [((list m) '() '())                              ; the red house, the first house
             (define value (known 'value (word m)))
             (cond [(known 'position (word m)) => (λ (position) `(at ,(car position)))]
                   [(and value (eq? (car value) (car noun))) `(at ,(constant value))]
                   [else (fail "~s does not say which ~a" (word m) (word n))])]
            [('() (list p) '()) (place p)]                   ; the man with the fox, the house next to …
            [('() '() (list v)) (one "what the clause says" (says v n))]   ; the man who smokes Kools
            [(_ _ _) (fail "which ~a?" (word n))]))]
    [else (fail "~s names no house" (word n))]))

(define (place p)               ; the preposition at p, with the noun it governs
  (define (governed q) (one (format "what ~s governs" (word q)) (right q "J")))
  (define (from q how)          ; next to x, to the right of x: x has to be a house itself
    (match (thing (governed q)) [`(at ,x) (list how x)] [_ (fail "a house beside a house beside a house")]))
  (cond
    [(word? p "in")
     (unless (word? (governed p) "house") (fail "in ~s?" (word (governed p))))
     (thing (governed p))]
    [(word? p "with") (thing (governed p))]
    [(and (word? p "to") (word? (- p 1) "next") (linked? (- p 1) p)) (from p 'near)]     ; next to x
    [(and (word? p "to") (word? (governed p) "right"))                                 ; to the right of x
     (from (one "what follows right" (right (governed p) "M"))
           (if (and (word? (- p 1) "immediately") (pair? (left (- p 1) "EB"))) 'right 'rightward))]
    [(word? p "where")                                       ; where the horse is kept
     (define subject (one "the subject after where" (right p "C")))
     (unless (null? (says (one "its verb" (right subject "S")) subject)) (fail "where says too much"))
     (thing subject)]
    [else (fail "~s: not a place" (word p))]))

(define (says v subject)        ; the verb at v: the houses it puts its subject in
  (cond
    [(word? v "is" "are")
     (match* ((right v "Pv") (right v "Pp"))
       [((list p) '())                                       ; is drunk, are smoked, is kept: the passive
        (fits (or (known 'verb (word p)) (fail "~s: not a verb" (word p))) (word subject))
        (places p)]
       [('() (list p)) (list (place p))]                     ; is to the right of …, is next to …
       [(_ _) (fail "after ~s, no passive and no place" (word v))])]
    [(known 'verb (word v))                                  ; owns the dog, lives in …
     => (λ (verb)
          (append (for/list ([o (right v "O")]) (fits verb (word o)) (thing o))
                  (places v)))]
    [else (fail "~s: not a verb" (word v))]))

(define (places v)              ; the prepositions that modify the verb at v, and no noun
  (for/list ([p (right v "MV")] #:when (null? (left p "M"))) (place p)))

(define (reading)               ; the linkage's meaning: (houses n), (fact term) or (ask tag term)
  (for ([i (linkage-unlinked (here))]) (fail "the parser links ~s to nothing" (word i)))
  (define subject (one "the subject" (right 0 "W")))
  (cond
    [(word? subject "there")                                 ; There are five houses.
     (define houses (one "what there is" (right (one "there's verb" (right subject "PF")) "SI")))
     (define number (known 'number (word (one "how many" (left houses "D")))))
     (unless (and number (word? houses "houses")) (fail "there are how many houses?"))
     `(houses ,(car number))]
    [else
     (define said (one "what is said of the subject" (says (one "the subject's verb" (right subject "S")) subject)))
     (match (known 'asks (word subject))
       [(cons tag _) (asks tag said)]                        ; Who …?
       [#f `(fact ,(together (thing subject) said))])]))

;; ---- from stdin to stdout

(convert #:program "lg2smt.rkt" #:from "link grammar's linkages"
         #:parse "linkage" #:first "its cheapest linkage"
         #:parses (λ (lines) (values (read-linkages lines) #f))
         #:reading (λ (l) (parameterize ([here l]) (reading))))
