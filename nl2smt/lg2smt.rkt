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
;; off the first of its linkages that has a shape known here.

(define-values (readings? vocabulary-file models)
  (match (vector->list (current-command-line-arguments))
    [(list "--readings" file) (values #t file '())]
    [(list file models ...) (values #f file (map (λ (m) (read (open-input-string m))) models))]))

;; ---- the vocabulary: forms (kind tag word ...), a name of several words a list

(define vocabulary (file->list vocabulary-file))
(define (forms kind) (filter (λ (form) (eq? (first form) kind)) vocabulary))
(define (spelling name) (string-join (map ~a (flatten name))))         ; (Old Gold) → "Old Gold"

(define (known kind word)       ; a word of this kind: (tag . the vocabulary's spelling), or #f
  (for*/first ([form (forms kind)] [name (cddr form)]
               #:when (and word (string-ci=? (spelling name) word)))
    (cons (second form) (spelling name))))

(define (constant value)        ; (smoke . "Old Gold") → smoke-Old-Gold, the SMT constant
  (string->symbol (format "~a-~a" (car value) (string-replace (cdr value) " " "-"))))
(define (constants tag)         ; the constants of a tag's values, in the vocabulary's order
  (for*/list ([form (forms 'value)] #:when (eq? (second form) tag) [name (cddr form)])
    (constant (cons tag (spelling name)))))

;; ---- link-parser's output

(struct sentence (line text linkages))
(struct linkage (words unlinked links))    ; words by position; links (left right label)

(define (headed lines rx)       ; the lines in blocks, each begun by a line that matches rx
  (for/fold ([blocks '()] #:result (reverse (map reverse blocks))) ([line lines])
    (cond [(regexp-match? rx line) (cons (list line) blocks)]
          [(null? blocks) blocks]
          [else (cons (cons line (car blocks)) (cdr blocks))])))

(define (read-sentences lines)
  (for/list ([block (headed lines #px"^line \\d+: ")])
    (match-define (list _ line text) (regexp-match #px"^line (\\d+): (.*)$" (first block)))
    (sentence (string->number line) text
              (filter-map read-linkage (headed (rest block) #px"^\\s*Linkage \\d+")))))

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
(define (fail . message) (raise (apply format message)))
(define (word i) (and (< -1 i (vector-length (linkage-words (here)))) (vector-ref (linkage-words (here)) i)))
(define (word? i . words) (and (word i) (member (string-downcase (word i)) words) #t))

(define (of-type? label type)   ; a label is a type, then subscripts: Ss*s is an S, SIpx is not
  (regexp-match? (pregexp (string-append "^" type "([^A-Z]|$)")) label))
(define (right i type)          ; the words that the word at i is linked to on its right
  (for/list ([l (linkage-links (here))] #:when (and (= (first l) i) (of-type? (third l) type))) (second l)))
(define (left i type)
  (for/list ([l (linkage-links (here))] #:when (and (= (second l) i) (of-type? (third l) type))) (first l)))
(define (linked? i j) (for/or ([l (linkage-links (here))]) (and (= (first l) i) (= (second l) j))))
(define (one what found) (match found [(list it) it] [_ (fail "~a: ~a found" what (length found))]))

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
        (fits (or (known 'verb (word p)) (fail "~s: not a verb" (word p))) subject)
        (places p)]
       [('() (list p)) (list (place p))]                     ; is to the right of …, is next to …
       [(_ _) (fail "after ~s, no passive and no place" (word v))])]
    [(known 'verb (word v))                                  ; owns the dog, lives in …
     => (λ (verb)
          (append (for/list ([o (right v "O")]) (fits verb o) (thing o))
                  (places v)))]
    [else (fail "~s: not a verb" (word v))]))

(define (places v)              ; the prepositions that modify the verb at v, and no noun
  (for/list ([p (right v "MV")] #:when (null? (left p "M"))) (place p)))

(define (fits verb n)           ; owns wants a pet, drinks a drink, smokes a brand
  (define value (known 'value (word n)))
  (unless (and value (eq? (car value) (car verb)))
    (fail "~s wants a ~a, which ~s is not" (cdr verb) (car verb) (word n))))

(define (together a b)          ; a subject's house and the house it is put in: an SMT term
  (match* (a b)
    [(`(at ,x) `(at ,y)) `(= ,x ,y)]
    [(`(at ,x) `(near ,y)) `(near ,x ,y)]
    [(`(near ,x) `(at ,y)) `(near ,y ,x)]
    [(`(at ,x) `(right ,y)) `(order ,y ,x)]
    [(`(at ,x) `(rightward ,y)) `(< ,y ,x)]
    [(_ _) (fail "a house beside a house beside a house")]))

(define (reading)               ; the linkage's meaning: (houses n), (fact term) or (ask (constant term) ...)
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
       [(cons tag _) `(ask ,@(for/list ([c (constants tag)]) (list c (together `(at ,c) said))))]   ; Who …?
       [#f `(fact ,(together (thing subject) said))])]))

(define (readings s)            ; each linkage's meaning, or as a string why it has none
  (for/list ([l (sentence-linkages s)])
    (with-handlers ([string? values]) (parameterize ([here l]) (reading)))))

;; ---- the script

(define (stop s . message)
  (eprintf "line ~a: ~a\n" (sentence-line s) (apply format message))
  (exit 1))

(define (script taken)          ; taken: (sentence rank meaning) for each sentence, in the input's order
  (define (title t) (format "line ~a, linkage ~a of ~a: ~a" (sentence-line (first t)) (second t)
                            (length (sentence-linkages (first t))) (sentence-text (first t))))
  (define (those kind) (filter (λ (t) (eq? (first (third t)) kind)) taken))
  (define tags (remove-duplicates (map second (forms 'value))))
  (define everything (append-map constants tags))
  (define-values (houses counted)
    (match (those 'houses)
      ['() (values (apply max (map (λ (tag) (length (constants tag))) tags)) "no line says how many: one for each value")]
      [(list t) (values (second (third t)) (title t))]
      [(list _ t _ ...) (stop (first t) "the houses are counted twice")]))
  (define (numbered term t)     ; the middle house has a number once the houses are counted
    (cond [(eq? term 'middle)
           (if (odd? houses) (quotient (+ houses 1) 2) (stop (first t) "~a houses have no middle one" houses))]
          [(list? term) (map (λ (part) (numbered part t)) term)]
          [else term]))

  (printf "; written by lg2smt.rkt from link grammar's linkages of the sentences\n")
  (printf "(set-option :produce-models true)\n(set-logic QF_LIA)\n")
  (printf "(define-fun order ((x Int) (y Int)) Bool (= (+ x 1) y))\n")
  (printf "(define-fun near ((x Int) (y Int)) Bool (or (= (+ x 1) y) (= (- x 1) y)))\n")
  (printf "\n; ~a\n(define-fun houses () Int ~a)\n" counted houses)
  (for ([tag tags])
    (printf "\n; from the vocabulary: each ~a is in a house, and no two in one\n" tag)
    (for ([c (constants tag)]) (printf "(declare-fun ~a () Int)\n" c))
    (for ([c (constants tag)]) (printf "(assert (<= 1 ~a houses))\n" c))
    (printf "(assert (distinct ~a))\n" (string-join (map ~a (constants tag)))))
  (for ([t (those 'fact)])
    (printf "\n; ~a\n(assert ~a)\n" (title t) (numbered (second (third t)) t)))
  (for ([model models])
    (printf "\n; not this solution, which is already found\n(assert (not (and~a)))\n"
            (string-append* (for/list ([pair model]) (format " (= ~a ~a)" (first pair) (second pair))))))
  (printf "\n(check-sat)\n(get-value (~a))\n" (string-join (map ~a everything)))
  (for ([t (those 'ask)])       ; the answer is each constant whose term is true
    (define asked (rest (third t)))
    (printf "\n; ~a\n(echo \"question ~a ~a\")\n(get-value (~a))\n" (title t) (sentence-line (first t))
            (string-join (map (compose ~a first) asked))
            (string-join (for/list ([a asked]) (~a (numbered (second a) t)))))))

;; ---- from stdin to stdout

(define sentences (read-sentences (port->lines)))

(cond
  [readings?
   (for ([s sentences])
     (printf "line ~a: ~a\n" (sentence-line s) (sentence-text s))
     (for ([r (readings s)] [rank (in-naturals 1)])
       (printf "  linkage ~a: ~a\n" rank (if (string? r) (string-append "not read, " r) r))))]
  [else
   (script
    (for/list ([s sentences])
      (define all (readings s))
      (define rank (index-where all list?))
      (cond [rank (list s (+ rank 1) (list-ref all rank))]
            [(null? all) (stop s "the parser has no linkage of ~s" (sentence-text s))]
            [else (stop s "cannot read ~s: ~a (its cheapest linkage; none of the ~a is read)"
                        (sentence-text s) (first all) (length all))])))])
