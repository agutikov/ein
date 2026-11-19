#lang racket
;; script.rkt — what the two converters share: the vocabulary, what a sentence means, and
;; the SMT-LIB script written from the meanings.
;;
;;   racket CONVERTER [--readings] VOCABULARY [MODEL ...] < the parser's output
;;
;; stdin       for each sentence a line "line N: <the sentence>", then what the parser gave
;;             for it: the converter says how that is read
;; VOCABULARY  vocabulary.rktd
;; MODEL       a solution the solver has printed, ((constant value) ...): it is ruled out
;; stdout      the SMT-LIB script; under --readings, what each parse of each sentence says
;;
;; A converter, lg2smt.rkt or nltk2smt.rkt, reads its parser's output and says what one
;; parse of a sentence means. A meaning is one of
;;
;;   (houses n)       the sentence counts the houses
;;   (fact term)      it states the term, over the things' constants and the houses' numbers
;;   (ask tag term)   it asks which things of the tag make the term true, `who` standing for one
;;
;; The script is written from the meanings and from nothing else. A thing has a constant
;; only where a sentence names it, the houses have a count only where a line counts them,
;; and a question is asked of the things the sentences name. What the vocabulary adds is
;; what a kind is: two things of one kind are in two houses.

(provide vocabulary spelling known constant fail one fits together asks headed convert)

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

;; ---- what a sentence says, in the terms both converters build
;;
;; A noun, or a preposition with its noun, says which house:
;;   (at x)         the house of x, a constant or a house's number
;;   (near x)       a house next to it
;;   (right x)      the house immediately to its right
;;   (rightward x)  a house somewhere to its right

(define (fail . message) (raise (apply format message)))
(define (one what found) (match found [(list it) it] [_ (fail "~a: ~a found" what (length found))]))

(define (fits verb noun)       ; owns wants a pet, drinks a drink, smokes a brand
  (define value (known 'value noun))
  (unless (and value (eq? (car value) (car verb)))
    (fail "~s wants a ~a, which ~s is not" (cdr verb) (car verb) noun)))

(define (together a b)          ; a subject's house and the house it is put in: an SMT term
  (match* (a b)
    [(`(at ,x) `(at ,y)) `(= ,x ,y)]
    [(`(at ,x) `(near ,y)) `(near ,x ,y)]
    [(`(near ,x) `(at ,y)) `(near ,y ,x)]
    [(`(at ,x) `(right ,y)) `(order ,y ,x)]
    [(`(at ,x) `(rightward ,y)) `(< ,y ,x)]
    [(_ _) (fail "a house beside a house beside a house")]))

(define (asks tag said)         ; Who …?: which things of the tag are in the house that is said
  `(ask ,tag ,(together '(at who) said)))

;; ---- the parser's output, by sentence

(struct sentence (line text parses note))   ; note: why the parser gave nothing, or #f

(define (headed lines rx)       ; the lines in blocks, each begun by a line that matches rx
  (for/fold ([blocks '()] #:result (reverse (map reverse blocks))) ([line lines])
    (cond [(regexp-match? rx line) (cons (list line) blocks)]
          [(null? blocks) blocks]
          [else (cons (cons line (car blocks)) (cdr blocks))])))

(define (stop s . message)
  (eprintf "line ~a: ~a\n" (sentence-line s) (apply format message))
  (exit 1))

;; ---- the script

(define (script taken program from parse)   ; taken: (sentence rank meaning), in the input's order
  (define (title t) (format "line ~a, ~a ~a of ~a: ~a" (sentence-line (first t)) parse (second t)
                            (length (sentence-parses (first t))) (sentence-text (first t))))
  (define (those kind) (filter (λ (t) (eq? (first (third t)) kind)) taken))
  (define said (flatten (map (compose rest third) (append (those 'fact) (those 'ask)))))
  (define tags (remove-duplicates (map second (forms 'value))))
  (define (named tag)           ; the tag's constants that a sentence names, in the vocabulary's order
    (filter (λ (c) (memq c said)) (constants tag)))
  (define-values (houses counted)
    (match (those 'houses)
      ['() (values #f "no line says how many houses there are: a house's number is 1 or more")]
      [(list t) (values (second (third t)) (title t))]
      [(list _ t _ ...) (stop (first t) "the houses are counted twice")]))
  (define (numbered term t)     ; the middle house has a number once the houses are counted
    (cond [(eq? term 'middle)
           (cond [(not houses) (stop (first t) "which house is the middle one? No line says how many there are")]
                 [(odd? houses) (quotient (+ houses 1) 2)]
                 [else (stop (first t) "~a houses have no middle one" houses)])]
          [(list? term) (map (λ (part) (numbered part t)) term)]
          [else term]))
  (define (of term c)           ; a question's term, asked of one thing
    (cond [(eq? term 'who) c] [(list? term) (map (λ (part) (of part c)) term)] [else term]))

  (printf "; written by ~a from ~a of the sentences\n" program from)
  (printf "(set-option :produce-models true)\n(set-logic QF_LIA)\n")
  (printf "(define-fun order ((x Int) (y Int)) Bool (= (+ x 1) y))\n")
  (printf "(define-fun near ((x Int) (y Int)) Bool (or (= (+ x 1) y) (= (- x 1) y)))\n")
  (printf "\n; ~a\n" counted)
  (when houses (printf "(define-fun houses () Int ~a)\n" houses))
  (for ([tag tags] #:unless (null? (named tag)))
    (printf "\n; each ~a the sentences name is in a house, and no two in one\n" tag)
    (for ([c (named tag)]) (printf "(declare-fun ~a () Int)\n" c))
    (for ([c (named tag)]) (printf (if houses "(assert (<= 1 ~a houses))\n" "(assert (<= 1 ~a))\n") c))
    (when (pair? (rest (named tag)))
      (printf "(assert (distinct ~a))\n" (string-join (map ~a (named tag))))))
  (for ([t (those 'fact)])
    (printf "\n; ~a\n(assert ~a)\n" (title t) (numbered (second (third t)) t)))
  (for ([model models])
    (printf "\n; not this solution, which is already found\n(assert ~a)\n"
            (if (null? model)
                "false"
                (format "(not (and~a))"
                        (string-append* (for/list ([pair model]) (format " (= ~a ~a)" (first pair) (second pair))))))))
  (printf "\n(check-sat)\n")
  (unless (null? (append-map named tags))
    (printf "(get-value (~a))\n" (string-join (map ~a (append-map named tags)))))
  (for ([t (those 'ask)])       ; the answer is each thing whose term is true
    (match-define (list _ tag term) (third t))
    (printf "\n; ~a\n(echo \"question ~a\")\n" (title t)
            (string-join (map ~a (cons (sentence-line (first t)) (named tag)))))
    (unless (null? (named tag))
      (printf "(get-value (~a))\n"
              (string-join (for/list ([c (named tag)]) (~a (numbered (of term c) t))))))))

;; ---- from stdin to stdout
;;
;; program      the converter's file name, for the script's first line
;; from         whose output it reads: "link grammar's linkages"
;; parse        what one parse is called: "linkage", "tree"
;; first-parse  what the first one is called: "its cheapest linkage"
;; parses       the lines under a sentence's header → its parses, and why there is none or #f
;; reading      a parse → its meaning; it raises a string where it has none

(define (convert #:program program #:from from #:parse parse #:first first-parse
                 #:parses parses #:reading reading)
  (define sentences
    (for/list ([block (headed (port->lines) #px"^line \\d+: ")])
      (match-define (list _ line text) (regexp-match #px"^line (\\d+): (.*)$" (first block)))
      (define-values (found note) (parses (rest block)))
      (sentence (string->number line) text found note)))
  (define (readings s)          ; each parse's meaning, or as a string why it has none
    (for/list ([p (sentence-parses s)]) (with-handlers ([string? values]) (reading p))))
  (define (none s) (format "no ~a~a" parse (if (sentence-note s) (string-append ", " (sentence-note s)) "")))
  (cond
    [readings?
     (for ([s sentences])
       (printf "line ~a: ~a\n" (sentence-line s) (sentence-text s))
       (when (null? (sentence-parses s)) (printf "  ~a 0: not read, ~a\n" parse (none s)))
       (for ([r (readings s)] [rank (in-naturals 1)])
         (printf "  ~a ~a: ~a\n" parse rank (if (string? r) (string-append "not read, " r) r))))]
    [else
     (script
      (for/list ([s sentences])
        (define all (readings s))
        (define rank (index-where all list?))
        (cond [rank (list s (+ rank 1) (list-ref all rank))]
              [(null? all) (stop s "cannot read ~s: ~a" (sentence-text s) (none s))]
              [else (stop s "cannot read ~s: ~a (~a; none of the ~a is read)"
                          (sentence-text s) (first all) first-parse (length all))]))
      program from parse)]))
