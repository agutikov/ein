#lang racket
;; nltk2smt.rkt — NLTK's trees of the Zebra puzzle's sentences, to SMT-LIB code.
;;
;;   racket nltk2smt.rkt [--readings] VOCABULARY [MODEL ...] < trees
;;
;; stdin       for each sentence a line "line N: <the sentence>", then each tree NLTK's
;;             chart parser found for it, as NLTK prints a tree, under a line
;;             "tree K of M:"; or one line "no tree: <why>"
;; VOCABULARY  vocabulary.rktd
;; MODEL       a solution the solver has printed, ((constant value) ...): it is ruled out
;; stdout      the SMT-LIB script; under --readings, what each tree of each sentence says
;;
;; lg2smt.rkt's twin for the other parser: it reads a phrase-structure tree where that
;; program reads links, and says the same things. A sentence is read off the first of its
;; trees that has a shape known here. The command line, the vocabulary and the script are
;; script.rkt's, which the two share; what is here is how a tree is read.

(require "script.rkt")

;; ---- NLTK's output
;;
;; A tree is (label part ...), and NLTK's text of one is an S-expression. Under a part of
;; speech the parts are the words, one or, for a name of several words, several:
;; (N house), (N Old Gold), (P next to). Under a phrase they are trees.

(define (read-trees lines)      ; the lines under a sentence's header: its trees, and why it has none
  (values (for/list ([tree (headed lines #px"^tree \\d+ of \\d+:$")])
            (read (open-input-string (string-join (rest tree) "\n"))))
          (for/first ([line lines] #:when (string-prefix? line "no tree: "))
            (substring line (string-length "no tree: ")))))

(define (word t) (string-join (map ~a (rest t))))                      ; (N Old Gold) → "Old Gold"
(define (word? t . words) (and (member (string-downcase (word t)) words) #t))
(define (words t)               ; every word under the tree, as one string
  (string-join (let leaves ([t t]) (if (pair? t) (append-map leaves (rest t)) (list (~a t))))))

(define (nom np)                ; the noun phrase without its determiner
  (match np
    [`(NP (Det . ,_) ,(and nom `(Nom . ,_))) nom]
    [`(NP ,(and nom `(Nom . ,_))) nom]
    [_ (fail "~s: not a noun phrase" (words np))]))

(define (noun nom)              ; the noun, and what stands beside it, the nearest first
  (match nom
    [`(Nom ,(and n `(N . ,_))) (values n '())]
    [`(Nom ,(and inner `(Nom . ,_)) ,after)                             ; … with the fox, … who smokes Kools
     (define-values (n beside) (noun inner))
     (values n (append beside (list after)))]
    [`(Nom ,before ,(and inner `(Nom . ,_)))                            ; red …, five …, Kools …
     (define-values (n beside) (noun inner))
     (values n (append beside (list before)))]
    [_ (fail "~s: no noun" (words nom))]))

(define (head np) (define-values (n _) (noun (nom np))) (word n))       ; the noun phrase's noun, a word

;; ---- what a tree says: a noun phrase, or a preposition with its noun phrase, says which
;; house, in script.rkt's terms (at x), (near x), (right x), (rightward x)

(define (thing np)              ; the noun phrase
  (define-values (n beside) (noun (nom np)))
  (cond
    [(known 'value (word n))                                 ; the dog, coffee, the Englishman
     => (λ (value)
          (unless (null? beside) (fail "~s beside ~s" (words (first beside)) (word n)))
          `(at ,(constant value)))]
    [(known 'noun (word n))                                  ; house, smoker, man
     => (λ (kind)
          (match beside
            [(list (and m `(,(or 'Adj 'N) . ,_)))            ; the red house, the first house, the Kools smoker
             (define value (known 'value (word m)))
             (cond [(known 'position (word m)) => (λ (position) `(at ,(car position)))]
                   [(and value (eq? (car value) (car kind))) `(at ,(constant value))]
                   [else (fail "~s does not say which ~a" (word m) (word n))])]
            [(list (and pp `(PP . ,_))) (place pp)]          ; the man with the fox, the house next to …
            [(list `(RC (Rel . ,_) ,vp))                     ; the man who smokes Kools
             (one "what the clause says" (says vp (word n)))]
            [(list `(RC (RelAdv . ,_) ,clause)) (where clause)]   ; the house where the horse is kept
            [_ (fail "which ~a?" (word n))]))]
    [else (fail "~s names no house" (word n))]))

(define (where clause)          ; where the horse is kept: the house of the clause's subject
  (match clause
    [`(S ,(and np `(NP . ,_)) ,vp)
     (unless (null? (says vp (head np))) (fail "where says too much"))
     (thing np)]
    [_ (fail "where ~s?" (words clause))]))

(define (place pp)              ; the preposition with its noun phrase
  (define (from np how)         ; next to x, to the right of x: x has to be a house itself
    (match (thing np) [`(at ,x) (list how x)] [_ (fail "a house beside a house beside a house")]))
  (define (right-of np)         ; the right of x: x's noun phrase
    (define-values (n beside) (noun (nom np)))
    (unless (word? n "right") (fail "\"to\": not a place"))
    (match beside
      [(list `(PP ,(? (λ (p) (word? p "of"))) ,of)) of]
      [_ (fail "what follows right: ~a found" (length beside))]))
  (match pp
    [`(PP ,(and p `(P . ,_)) ,np)
     (cond [(word? p "in")
            (unless (string-ci=? (head np) "house") (fail "in ~s?" (head np)))
            (thing np)]
           [(word? p "with") (thing np)]
           [(word? p "next to") (from np 'near)]                        ; next to x
           [(word? p "to") (from (right-of np) 'rightward)]             ; to the right of x
           [else (fail "~s: not a place" (word p))])]
    [`(PP ,(and a `(Adv . ,_)) (PP ,(and p `(P . ,_)) ,np))             ; immediately to the right of x
     (unless (and (word? a "immediately") (word? p "to")) (fail "~s: not a place" (words pp)))
     (from (right-of np) 'right)]
    [_ (fail "~s: not a place" (words pp))]))

(define (says vp subject)       ; the verb phrase: the houses it puts its subject, a word, in
  (define (verb v) (or (known 'verb (word v)) (fail "~s: not a verb" (word v))))
  (match vp
    [`(VP ,(and inner `(VP . ,_)) ,pp)                       ; … in the red house, … next to the fox
     (append (says inner subject) (list (place pp)))]
    [`(VP ,(and v `(V . ,_)) ,objects ...)                   ; owns the dog, lives
     (for/list ([o objects]) (fits (verb v) (head o)) (thing o))]
    [`(VP (Be . ,_) ,(and p `(Part . ,_)))                   ; is drunk, are smoked, is kept: the passive
     (fits (verb p) subject)
     '()]
    [`(VP (Be . ,_) ,(and pp `(PP . ,_))) (list (place pp))] ; is to the right of …, is next to …
    [_ (fail "~s: no verb phrase known here" (words vp))]))

(define (reading tree)          ; the tree's meaning: (houses n), (fact term) or (ask tag term)
  (define (said vp subject) (one "what is said of the subject" (says vp subject)))
  (match tree
    [`(S (Ex . ,_) (Be . ,_) ,np)                            ; There are five houses.
     (define-values (n beside) (noun (nom np)))
     (define number (match beside [(list (and m `(Num . ,_))) (known 'number (word m))] [_ #f]))
     (unless (and number (word? n "houses")) (fail "there are how many houses?"))
     `(houses ,(car number))]
    [`(S ,(and who `(Wh . ,_)) ,vp)                          ; Who …?
     (define asked (or (known 'asks (word who)) (fail "~s asks for nothing" (word who))))
     (asks (car asked) (said vp (word who)))]
    [`(S ,(and np `(NP . ,_)) ,vp)                           ; every statement
     (define house (said vp (head np)))
     `(fact ,(together (thing np) house))]
    [_ (fail "~s: no sentence known here" (words tree))]))

;; ---- from stdin to stdout

(convert #:program "nltk2smt.rkt" #:from "NLTK's trees"
         #:parse "tree" #:first "its first tree"
         #:parses read-trees
         #:reading reading)
