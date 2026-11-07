
#### fact argument

`(p {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores nothing new | loads: stores nothing new |
| `digits` | loads: stores (p 7) | loads: stores (p "7") |
| `-digits` | loads: stores (p -7) | loads: stores (p "-7") |
| `1a` | loads: stores (p 1 a) | loads: stores (p "1a") |
| `$x` | parse error | loads: stores (p "$x") |
| `string` | loads: stores (p s) | parse error |
| `range` | loads: stores (p "1..5") | loads: stores (p "1..5") |
| `_` | loads: stores (p) | parse error |
| `?x` | loads: stores (p "?x") | parse error |
| `?_` | parse error | parse error |
| `a?x` | loads: stores (p a "?x") | parse error |
| `form` | loads: stores (p (f a)) | loads: stores (p (f a)) |
| `()` | loads: stores (p ()) | parse error |
| `:k v` | loads: stores (p) | parse error |
| `(and)` | loads: stores (p (and (q a))) | parse error |
| `(not)` | loads: stores (p (not (q a))) | loads: stores (p (not (q a))) |
| `(neq)` | loads: stores (p (neq a b)) | parse error |
| `(eq)` | loads: stores (p (eq a b)) | loads: stores (p (eq a b)) |

#### nested fact argument

`(r (f {S}) c)`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores (r (f a) c) | loads: stores (r (f a) c) |
| `digits` | loads: stores (r (f 7) c) | loads: stores (r (f "7") c) |
| `-digits` | loads: stores (r (f -7) c) | loads: stores (r (f "-7") c) |
| `1a` | loads: stores (r (f 1 a) c) | loads: stores (r (f "1a") c) |
| `$x` | parse error | loads: stores (r (f "$x") c) |
| `string` | loads: stores (r (f s) c) | parse error |
| `range` | loads: stores (r (f "1..5") c) | loads: stores (r (f "1..5") c) |
| `_` | loads: stores (r (f) c) | parse error |
| `?x` | loads: stores (r (f "?x") c) | parse error |
| `?_` | parse error | parse error |
| `a?x` | loads: stores (r (f a "?x") c) | parse error |
| `form` | loads: stores (r (f (f a)) c) | loads: stores (r (f (f a)) c) |
| `()` | loads: stores (r (f ()) c) | parse error |
| `:k v` | loads: stores (r (f) c) | parse error |
| `(and)` | loads: stores (r (f (and (q a))) c) | parse error |
| `(not)` | loads: stores (r (f (not (q a))) c) | loads: stores (r (f (not (q a))) c) |
| `(neq)` | loads: stores (r (f (neq a b)) c) | parse error |
| `(eq)` | loads: stores (r (f (eq a b)) c) | loads: stores (r (f (eq a b)) c) |

#### fact relation

`({S} c)`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores (a c) | loads: stores (a c) |
| `digits` | parse error | loads: stores ("7" c) |
| `-digits` | parse error | loads: stores ("-7" c) |
| `1a` | parse error | loads: stores ("1a" c) |
| `$x` | parse error | loads: stores ("$x" c) |
| `string` | parse error | parse error |
| `range` | parse error | loads: stores ("1..5" c) |
| `_` | parse error | parse error |
| `?x` | parse error | parse error |
| `?_` | parse error | parse error |
| `a?x` | loads: stores (a "?x" c) | parse error |
| `form` | parse error | parse error |
| `()` | parse error | parse error |
| `:k v` | parse error | parse error |
| `(and)` | parse error | parse error |
| `(not)` | parse error | parse error |
| `(neq)` | parse error | parse error |
| `(eq)` | parse error | parse error |

#### stored negation

`(not {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores (not a) | parse error |
| `digits` | loads: stores (not 7) | parse error |
| `-digits` | loads: stores (not -7) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: stores (not s) | parse error |
| `range` | loads: stores (not "1..5") | parse error |
| `_` | loads: stores (not) | parse error |
| `?x` | loads: stores (not "?x") | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: stores (not (f a)) | loads: stores (not (f a)) |
| `()` | loads: stores (not ()) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: stores (not (and (q a))) | parse error |
| `(not)` | loads: stores (not (not (q a))) | parse error |
| `(neq)` | loads: stores (not (neq a b)) | parse error |
| `(eq)` | loads: stores (not (eq a b)) | loads: stores (not (eq a b)) |

#### fact :source

`(q c :source {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores (q c) | parse error |
| `digits` | loads: stores (q c) | parse error |
| `-digits` | loads: stores (q c) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: stores (q c :source "s") | loads: stores (q c :source "s") |
| `range` | loads: stores (q c) | parse error |
| `_` | loads: stores (q c) | parse error |
| `?x` | loads: stores (q c) | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: stores (q c) | parse error |
| `()` | loads: stores (q c) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: stores (q c) | parse error |
| `(not)` | loads: stores (q c) | parse error |
| `(neq)` | loads: stores (q c) | parse error |
| `(eq)` | loads: stores (q c) | parse error |

#### fact :rule

`(q c :rule {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores (q c :rule a) | loads: stores (q c :rule a) |
| `digits` | loads: stores (q c) | loads: stores (q c :rule "7") |
| `-digits` | loads: stores (q c) | loads: stores (q c :rule "-7") |
| `1a` | parse error | loads: stores (q c :rule "1a") |
| `$x` | parse error | loads: stores (q c :rule "$x") |
| `string` | loads: stores (q c) | parse error |
| `range` | loads: stores (q c) | loads: stores (q c :rule "1..5") |
| `_` | loads: stores (q c) | parse error |
| `?x` | loads: stores (q c) | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: stores (q c) | parse error |
| `()` | loads: stores (q c) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: stores (q c) | parse error |
| `(not)` | loads: stores (q c) | parse error |
| `(neq)` | loads: stores (q c) | parse error |
| `(eq)` | loads: stores (q c) | parse error |

#### fact :using

`(q c :rule x :using {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores (q c :rule x) | parse error |
| `digits` | loads: stores (q c :rule x) | parse error |
| `-digits` | loads: stores (q c :rule x) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: stores (q c :rule x) | parse error |
| `range` | loads: stores (q c :rule x) | parse error |
| `_` | loads: stores (q c :rule x) | parse error |
| `?x` | loads: stores (q c :rule x) | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: stores (q c :rule x) | parse error |
| `()` | loads: stores (q c :rule x) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: stores (q c :rule x) | parse error |
| `(not)` | loads: stores (q c :rule x) | parse error |
| `(neq)` | loads: stores (q c :rule x) | parse error |
| `(eq)` | loads: stores (q c :rule x) | parse error |

#### activator argument

`(rule act (?y) :match (p ?y) :assert (hit ?y)) (act {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (hit a :rule act) | loads: derives (hit a :rule act) |
| `digits` | loads: derives nothing | loads: derives nothing |
| `-digits` | loads: derives nothing | loads: derives nothing |
| `1a` | loads: derives nothing | loads: derives nothing |
| `$x` | parse error | loads: derives nothing |
| `string` | loads: derives nothing | parse error |
| `range` | loads: derives nothing | loads: derives nothing |
| `_` | loads: derives nothing | parse error |
| `?x` | loads: derives nothing | parse error |
| `?_` | parse error | parse error |
| `a?x` | loads: derives nothing | parse error |
| `form` | loads: derives (hit a :rule act) (hit b :rule act) | loads: derives (hit a :rule act) (hit b :rule act) |
| `()` | loads: derives (hit a :rule act) (hit b :rule act) | parse error |
| `:k v` | loads: derives nothing | parse error |
| `(and)` | loads: derives (hit a :rule act) (hit b :rule act) | parse error |
| `(not)` | loads: derives (hit a :rule act) (hit b :rule act) | loads: derives (hit a :rule act) (hit b :rule act) |
| `(neq)` | loads: derives (hit a :rule act) (hit b :rule act) | parse error |
| `(eq)` | loads: derives (hit a :rule act) (hit b :rule act) | loads: derives (hit a :rule act) (hit b :rule act) |

#### relation name

`(relation {S} T)`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: declares (relation a T) | loads: declares (relation a T) |
| `digits` | load error: malformed (relation) at None | loads: declares (relation 7 T) |
| `-digits` | load error: malformed (relation) at None | loads: declares (relation -7 T) |
| `1a` | load error: malformed (relation) at None | loads: declares (relation 1a T) |
| `$x` | parse error | loads: declares (relation $x T) |
| `string` | load error: malformed (relation) at None | parse error |
| `range` | load error: malformed (relation) at None | loads: declares (relation 1..5 T) |
| `_` | load error: malformed (relation) at None | parse error |
| `?x` | load error: malformed (relation) at None | parse error |
| `?_` | parse error | parse error |
| `a?x` | load error: malformed (relation) at None | parse error |
| `form` | load error: malformed (relation) at None | parse error |
| `()` | load error: malformed (relation) at None | parse error |
| `:k v` | parse error | parse error |
| `(and)` | load error: malformed (relation) at None | parse error |
| `(not)` | load error: malformed (relation) at None | parse error |
| `(neq)` | load error: malformed (relation) at None | parse error |
| `(eq)` | load error: malformed (relation) at None | parse error |

#### relation signature

`(relation r2 {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: declares (relation r2 a) | loads: declares (relation r2 a) |
| `digits` | load error: malformed (relation) at None | loads: declares (relation r2 7) |
| `-digits` | load error: malformed (relation) at None | loads: declares (relation r2 -7) |
| `1a` | load error: malformed (relation) at None | loads: declares (relation r2 1a) |
| `$x` | parse error | loads: declares (relation r2 $x) |
| `string` | load error: malformed (relation) at None | parse error |
| `range` | load error: malformed (relation) at None | loads: declares (relation r2 1..5) |
| `_` | load error: malformed (relation) at None | parse error |
| `?x` | load error: malformed (relation) at None | parse error |
| `?_` | parse error | parse error |
| `a?x` | load error: malformed (relation) at None | parse error |
| `form` | load error: malformed (relation) at None | parse error |
| `()` | load error: malformed (relation) at None | parse error |
| `:k v` | loads: declares (relation r2) | parse error |
| `(and)` | load error: malformed (relation) at None | parse error |
| `(not)` | load error: malformed (relation) at None | parse error |
| `(neq)` | load error: malformed (relation) at None | parse error |
| `(eq)` | load error: malformed (relation) at None | parse error |

#### relation :why

`(relation r2 T :why {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: declares (relation r2 T) | parse error |
| `digits` | loads: declares (relation r2 T) | parse error |
| `-digits` | loads: declares (relation r2 T) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: declares (relation r2 T) | loads: declares (relation r2 T) |
| `range` | loads: declares (relation r2 T) | parse error |
| `_` | loads: declares (relation r2 T) | parse error |
| `?x` | loads: declares (relation r2 T) | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: declares (relation r2 T) | parse error |
| `()` | loads: declares (relation r2 T) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: declares (relation r2 T) | parse error |
| `(not)` | loads: declares (relation r2 T) | parse error |
| `(neq)` | loads: declares (relation r2 T) | parse error |
| `(eq)` | loads: declares (relation r2 T) | parse error |

#### rule name

`(rule {S} () :match (p ?x) :assert (hit ?x))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (hit a :rule a) (hit b :rule a) | loads: derives (hit a :rule a) (hit b :rule a) |
| `digits` | parse error | loads: derives (hit a :rule "7") (hit b :rule "7") |
| `-digits` | parse error | loads: derives (hit a :rule "-7") (hit b :rule "-7") |
| `1a` | parse error | loads: derives (hit a :rule "1a") (hit b :rule "1a") |
| `$x` | parse error | loads: derives (hit a :rule "$x") (hit b :rule "$x") |
| `string` | parse error | parse error |
| `range` | parse error | loads: derives (hit a :rule "1..5") (hit b :rule "1..5") |
| `_` | parse error | parse error |
| `?x` | parse error | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | parse error | parse error |
| `()` | parse error | parse error |
| `:k v` | parse error | parse error |
| `(and)` | parse error | parse error |
| `(not)` | parse error | parse error |
| `(neq)` | parse error | parse error |
| `(eq)` | parse error | parse error |

#### rule parameter

`(rule rr ({S}) :match (p ?x) :assert (hit ?x))`

| shape | today | the final grammar |
|---|---|---|
| `name` | parse error | parse error |
| `digits` | parse error | parse error |
| `-digits` | parse error | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | parse error | parse error |
| `range` | parse error | parse error |
| `_` | parse error | parse error |
| `?x` | loads: derives nothing | loads: derives nothing |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | parse error | parse error |
| `()` | parse error | parse error |
| `:k v` | parse error | parse error |
| `(and)` | parse error | parse error |
| `(not)` | parse error | parse error |
| `(neq)` | parse error | parse error |
| `(eq)` | parse error | parse error |

#### :match

`(rule rr () :match {S} :assert (hit yes))`

| shape | today | the final grammar |
|---|---|---|
| `name` | load error: rule 'rr': its :match is a, and a is no premise | parse error |
| `digits` | load error: rule 'rr': its :match is 7, and 7 is no premise | parse error |
| `-digits` | load error: rule 'rr': its :match is -7, and -7 is no premise | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | load error: rule 'rr': its :match is "s", and "s" is no premise | parse error |
| `range` | load error: rule 'rr': its :match is 1..5, and no fact can hold it | parse error |
| `_` | load error: rule 'rr': its :match is _, and _ is no premise | parse error |
| `?x` | load error: rule 'rr': its :match is ?x, and ?x is no premise | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: derives nothing | loads: derives nothing |
| `()` | loads: derives nothing | loads: derives nothing |
| `:k v` | parse error | parse error |
| `(and)` | loads: derives (hit yes :rule rr) | loads: derives (hit yes :rule rr) |
| `(not)` | loads: derives nothing | loads: derives nothing |
| `(neq)` | loads: derives (hit yes :rule rr) | loads: derives (hit yes :rule rr) |
| `(eq)` | loads: derives nothing | loads: derives nothing |

#### premise relation

`(rule rr () :match ({S} a) :assert (hit yes))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives nothing | loads: derives nothing |
| `digits` | parse error | loads: derives nothing |
| `-digits` | parse error | loads: derives nothing |
| `1a` | parse error | loads: derives nothing |
| `$x` | parse error | loads: derives nothing |
| `string` | parse error | parse error |
| `range` | parse error | loads: derives nothing |
| `_` | load error: rule 'rr': its :match writes (_ a) with the wildcard _ as its relation | parse error |
| `?x` | load error: rule 'rr': its :match writes (?x a) with ?x as its relation, which no parameter binds | load error: rule 'rr': its :match writes (?x a) with ?x as its relation, which no parameter binds |
| `?_` | parse error | parse error |
| `a?x` | loads: derives nothing | parse error |
| `form` | parse error | parse error |
| `()` | parse error | parse error |
| `:k v` | parse error | parse error |
| `(and)` | parse error | parse error |
| `(not)` | parse error | parse error |
| `(neq)` | parse error | parse error |
| `(eq)` | parse error | parse error |

#### premise argument

`(rule rr () :match (p {S}) :assert (hit yes))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (hit yes :rule rr) | loads: derives (hit yes :rule rr) |
| `digits` | loads: derives nothing | loads: derives nothing |
| `-digits` | loads: derives nothing | loads: derives nothing |
| `1a` | loads: derives nothing | loads: derives nothing |
| `$x` | parse error | loads: derives nothing |
| `string` | loads: derives nothing | parse error |
| `range` | load error: rule 'rr': its :match writes 1..5 as an argument of p, and no fact can hold it | loads: derives nothing |
| `_` | loads: derives (hit yes :rule rr) | parse error |
| `?x` | loads: derives (hit yes :rule rr) | loads: derives (hit yes :rule rr) |
| `?_` | parse error | parse error |
| `a?x` | loads: derives nothing | parse error |
| `form` | loads: derives nothing | loads: derives nothing |
| `()` | loads: derives nothing | parse error |
| `:k v` | loads: derives nothing | parse error |
| `(and)` | loads: derives nothing | loads: derives nothing |
| `(not)` | loads: derives nothing | loads: derives nothing |
| `(neq)` | loads: derives nothing | loads: derives nothing |
| `(eq)` | loads: derives nothing | loads: derives nothing |

#### premise nested argument

`(r (f a) c) (rule rr () :match (r (f {S}) c) :assert (hit yes))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (hit yes :rule rr) | loads: derives (hit yes :rule rr) |
| `digits` | loads: derives nothing | loads: derives nothing |
| `-digits` | loads: derives nothing | loads: derives nothing |
| `1a` | loads: derives nothing | loads: derives nothing |
| `$x` | parse error | loads: derives nothing |
| `string` | loads: derives nothing | parse error |
| `range` | load error: rule 'rr': its :match writes 1..5 as an argument of f, and no fact can hold it | loads: derives nothing |
| `_` | loads: derives (hit yes :rule rr) | parse error |
| `?x` | loads: derives (hit yes :rule rr) | loads: derives (hit yes :rule rr) |
| `?_` | parse error | parse error |
| `a?x` | loads: derives nothing | parse error |
| `form` | loads: derives nothing | loads: derives nothing |
| `()` | loads: derives nothing | parse error |
| `:k v` | load error: rule 'rr': its :match writes :k v as an argument of f, and no fact can hold it | parse error |
| `(and)` | loads: derives nothing | loads: derives nothing |
| `(not)` | loads: derives nothing | loads: derives nothing |
| `(neq)` | loads: derives nothing | loads: derives nothing |
| `(eq)` | loads: derives nothing | loads: derives nothing |

#### conjunct

`(rule rr () :match (and (p ?x) {S}) :assert (hit ?x))`

| shape | today | the final grammar |
|---|---|---|
| `name` | load error: rule 'rr': its :match writes a as a premise of and, and a is no premise | parse error |
| `digits` | load error: rule 'rr': its :match writes 7 as a premise of and, and 7 is no premise | parse error |
| `-digits` | load error: rule 'rr': its :match writes -7 as a premise of and, and -7 is no premise | parse error |
| `1a` | load error: rule 'rr': its :match writes 1 as a premise of and, and 1 is no premise | parse error |
| `$x` | parse error | parse error |
| `string` | load error: rule 'rr': its :match writes "s" as a premise of and, and "s" is no premise | parse error |
| `range` | load error: rule 'rr': its :match writes 1..5 as an argument of and, and no fact can hold it | parse error |
| `_` | load error: rule 'rr': its :match writes _ as a premise of and, and _ is no premise | parse error |
| `?x` | load error: rule 'rr': its :match writes ?x as a premise of and, and ?x is no premise | parse error |
| `?_` | parse error | parse error |
| `a?x` | load error: rule 'rr': its :match writes a as a premise of and, and a is no premise | parse error |
| `form` | loads: derives nothing | loads: derives nothing |
| `()` | loads: derives nothing | parse error |
| `:k v` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `(and)` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `(not)` | loads: derives nothing | loads: derives nothing |
| `(neq)` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `(eq)` | loads: derives nothing | loads: derives nothing |

#### absent argument

`(rule rr () :match (and (p ?x) (absent {S})) :assert (hit ?x))`

| shape | today | the final grammar |
|---|---|---|
| `name` | load error: rule 'rr': its :match writes a as a premise of absent, and a is no premise | parse error |
| `digits` | load error: rule 'rr': its :match writes 7 as a premise of absent, and 7 is no premise | parse error |
| `-digits` | load error: rule 'rr': its :match writes -7 as a premise of absent, and -7 is no premise | parse error |
| `1a` | load error: rule 'rr': its :match writes 1 as a premise of absent, and 1 is no premise | parse error |
| `$x` | parse error | parse error |
| `string` | load error: rule 'rr': its :match writes "s" as a premise of absent, and "s" is no premise | parse error |
| `range` | load error: rule 'rr': its :match writes 1..5 as an argument of absent, and no fact can hold it | parse error |
| `_` | load error: rule 'rr': its :match writes _ as a premise of absent, and _ is no premise | parse error |
| `?x` | load error: rule 'rr': its :match writes ?x as a premise of absent, and ?x is no premise | parse error |
| `?_` | parse error | parse error |
| `a?x` | load error: rule 'rr': its :match writes a as a premise of absent, and a is no premise | parse error |
| `form` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `()` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `:k v` | run error: `(absent …)` sub-plan compiled to no steps in SForm(head=Atom(name='absent'), args=(KwPair(key=Keyword(name='k'), value=Atom(name='v')),)) | parse error |
| `(and)` | loads: derives nothing | loads: derives nothing |
| `(not)` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `(neq)` | loads: derives nothing | loads: derives nothing |
| `(eq)` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |

#### guard argument

`(rule rr () :match (and (p ?x) (neq ?x {S})) :assert (hit ?x))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (hit b :rule rr) | loads: derives (hit b :rule rr) |
| `digits` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `-digits` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `1a` | parse error | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `$x` | parse error | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `string` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `range` | load error: rule 'rr': its :match writes 1..5 as an argument of neq, and no fact can hold it | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `_` | load error: rule 'rr': its :match writes (neq ?x _), and _ names no value | parse error |
| `?x` | loads: derives nothing | loads: derives nothing |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | load error: rule 'rr': its :match writes (neq ?x (f a)), and the form (f a) is no value | load error: rule 'rr': its :match writes (neq ?x (f a)), and the form (f a) is no value |
| `()` | load error: rule 'rr': its :match writes (neq ?x ()), and the form () is no value | parse error |
| `:k v` | parse error | parse error |
| `(and)` | load error: rule 'rr': its :match writes (neq ?x (and (q a))), and the form (and (q a)) is no value | parse error |
| `(not)` | load error: rule 'rr': its :match writes (neq ?x (not (q a))), and the form (not (q a)) is no value | load error: rule 'rr': its :match writes (neq ?x (not (q a))), and the form (not (q a)) is no value |
| `(neq)` | load error: rule 'rr': its :match writes (neq ?x (neq a b)), and the form (neq a b) is no value | parse error |
| `(eq)` | load error: rule 'rr': its :match writes (neq ?x (eq a b)), and the form (eq a b) is no value | load error: rule 'rr': its :match writes (neq ?x (eq a b)), and the form (eq a b) is no value |

#### :assert

`(rule rr () :match (p ?x) :assert {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | exit 1: rule 'rr': its :assert concludes a, which is not a fact | parse error |
| `digits` | exit 1: rule 'rr': its :assert concludes 7, which is not a fact | parse error |
| `-digits` | exit 1: rule 'rr': its :assert concludes -7, which is not a fact | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | exit 1: rule 'rr': its :assert concludes s, which is not a fact | parse error |
| `range` | load error: rule 'rr': its :assert is 1..5, and no fact can hold it | parse error |
| `_` | exit 1: rule 'rr': its :assert concludes _, which is not a fact | parse error |
| `?x` | exit 1: rule 'rr': its :assert concludes ?x, which is not a fact | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: derives (f a :rule rr) | loads: derives (f a :rule rr) |
| `()` | loads: derives ( :rule rr) | loads: derives ( :rule rr) |
| `:k v` | parse error | parse error |
| `(and)` | loads: derives nothing | loads: derives nothing |
| `(not)` | loads: derives (not (q a) :rule rr) | loads: derives (not (q a) :rule rr) |
| `(neq)` | loads: derives (neq a b :rule rr) | loads: derives (neq a b :rule rr) |
| `(eq)` | loads: derives (eq a b :rule rr) | loads: derives (eq a b :rule rr) |

#### conclusion argument

`(rule rr () :match (p ?x) :assert (hit {S}))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (hit a :rule rr) | loads: derives (hit a :rule rr) |
| `digits` | loads: derives (hit 7 :rule rr) | loads: derives (hit "7" :rule rr) |
| `-digits` | loads: derives (hit -7 :rule rr) | loads: derives (hit "-7" :rule rr) |
| `1a` | loads: derives (hit 1 a :rule rr) | loads: derives (hit "1a" :rule rr) |
| `$x` | parse error | loads: derives (hit "$x" :rule rr) |
| `string` | loads: derives (hit s :rule rr) | parse error |
| `range` | load error: rule 'rr': its :assert writes 1..5 as an argument of hit, and no fact can hold it | loads: derives (hit "1..5" :rule rr) |
| `_` | exit 1: rule 'rr': its :assert writes _ as an argument of hit, and no fact can hold it | parse error |
| `?x` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `?_` | parse error | parse error |
| `a?x` | loads: derives (hit a a :rule rr) (hit a b :rule rr) | parse error |
| `form` | loads: derives (hit (f a) :rule rr) | loads: derives (hit (f a) :rule rr) |
| `()` | loads: derives (hit () :rule rr) | parse error |
| `:k v` | exit 1: rule 'rr': its :assert writes :k v as an argument of hit, and no fact can hold it | parse error |
| `(and)` | loads: derives (hit (and (q a)) :rule rr) | parse error |
| `(not)` | loads: derives (hit (not (q a)) :rule rr) | loads: derives (hit (not (q a)) :rule rr) |
| `(neq)` | loads: derives (hit (neq a b) :rule rr) | parse error |
| `(eq)` | loads: derives (hit (eq a b) :rule rr) | loads: derives (hit (eq a b) :rule rr) |

#### conclusion relation

`(rule rr () :match (p ?x) :assert ({S} ?x))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (a a :rule rr) (a b :rule rr) | loads: derives (a a :rule rr) (a b :rule rr) |
| `digits` | parse error | loads: derives ("7" a :rule rr) ("7" b :rule rr) |
| `-digits` | parse error | loads: derives ("-7" a :rule rr) ("-7" b :rule rr) |
| `1a` | parse error | loads: derives ("1a" a :rule rr) ("1a" b :rule rr) |
| `$x` | parse error | loads: derives ("$x" a :rule rr) ("$x" b :rule rr) |
| `string` | parse error | parse error |
| `range` | parse error | loads: derives ("1..5" a :rule rr) ("1..5" b :rule rr) |
| `_` | load error: rule 'rr': its :assert writes (_ ?x) with the wildcard _ as its relation | parse error |
| `?x` | load error: rule 'rr': its :assert writes (?x ?x) with ?x as its relation, which no parameter binds | load error: rule 'rr': its :assert writes (?x ?x) with ?x as its relation, which no parameter binds |
| `?_` | parse error | parse error |
| `a?x` | loads: derives (a a a :rule rr) (a b b :rule rr) | parse error |
| `form` | parse error | parse error |
| `()` | parse error | parse error |
| `:k v` | parse error | parse error |
| `(and)` | parse error | parse error |
| `(not)` | parse error | parse error |
| `(neq)` | parse error | parse error |
| `(eq)` | parse error | parse error |

#### hrule conclusion argument

`(hrule h () :match (p ?x) :assert (q {S})) (solve :hrules (h))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: states (k_s)      1 · universe          no candidate: R is {∅} or empty, decided by root alone | loads: states (k_s)      1 · universe          no candidate: R is {∅} or empty, decided by root alone |
| `digits` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `-digits` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `1a` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `$x` | parse error | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `string` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `range` | load error: hrule 'h': its :assert writes 1..5 as an argument of q, and no fact can hold it | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `_` | loads: states (k_s)      1 · universe          no candidate: R is {∅} or empty, decided by root alone | parse error |
| `?x` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `?_` | parse error | parse error |
| `a?x` | loads: states (k_s)      4 · universe          2 candidates, 0 dead alone | parse error |
| `form` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `()` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `:k v` | loads: states (k_s)      1 · universe          no candidate: R is {∅} or empty, decided by root alone | parse error |
| `(and)` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `(not)` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `(neq)` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `(eq)` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |

#### rule :why

`(rule rr () :match (p ?x) :assert (hit ?x) :why {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `digits` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `-digits` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `range` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `_` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `?x` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `()` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `(not)` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `(neq)` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `(eq)` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |

#### rule :priority

`(rule rr () :match (p ?x) :assert (hit ?x) :priority {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `digits` | loads: derives (hit a :rule rr) (hit b :rule rr) | loads: derives (hit a :rule rr) (hit b :rule rr) |
| `-digits` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `range` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `_` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `?x` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `()` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `(not)` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `(neq)` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |
| `(eq)` | loads: derives (hit a :rule rr) (hit b :rule rr) | parse error |

#### :goal

`(query :goal {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model) | parse error |
| `digits` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model) | parse error |
| `-digits` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model) | parse error |
| `range` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model) | parse error |
| `_` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model) | parse error |
| `?x` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model) | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `()` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model (q a)) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model (q a)) |
| `(not)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `(neq)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model (neq a b)) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model (neq a b)) |
| `(eq)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |

#### goal argument

`(query :goal (p {S}))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model (p a)) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model (p a)) |
| `digits` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `-digits` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `1a` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `$x` | parse error | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `string` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | parse error |
| `range` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `_` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · the goal holds: it has no variable · as :expect        (model (p _)) | parse error |
| `?x` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) |
| `?_` | parse error | parse error |
| `a?x` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | parse error |
| `form` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `()` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | parse error |
| `:k v` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | parse error |
| `(and)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `(not)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `(neq)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |
| `(eq)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · as :expect        (model) |

#### :goal-text

`(query :goal (p ?x) :goal-text {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `digits` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `-digits` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) |
| `range` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `_` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `?x` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `()` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `(not)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `(neq)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| `(eq)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |

#### :over

`(query :goal (p ?x) :over {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | load error: (query …): :over a — one of all, minimal, maximal | load error: (query …): :over a — one of all, minimal, maximal |
| `digits` | load error: (query …): :over 7 — one of all, minimal, maximal | load error: (query …): :over 7 — one of all, minimal, maximal |
| `-digits` | load error: (query …): :over -7 — one of all, minimal, maximal | load error: (query …): :over -7 — one of all, minimal, maximal |
| `1a` | parse error | load error: (query …): :over 1a — one of all, minimal, maximal |
| `$x` | parse error | load error: (query …): :over $x — one of all, minimal, maximal |
| `string` | load error: (query …): :over s — one of all, minimal, maximal | parse error |
| `range` | load error: (query …): :over "1..5" — one of all, minimal, maximal | load error: (query …): :over 1..5 — one of all, minimal, maximal |
| `_` | load error: (query …): :over _ — one of all, minimal, maximal | parse error |
| `?x` | load error: (query …): :over ?x — one of all, minimal, maximal | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | load error: (query …): :over (f a) — one of all, minimal, maximal | parse error |
| `()` | load error: (query …): :over () — one of all, minimal, maximal | parse error |
| `:k v` | parse error | parse error |
| `(and)` | load error: (query …): :over (and (q a)) — one of all, minimal, maximal | parse error |
| `(not)` | load error: (query …): :over (not (q a)) — one of all, minimal, maximal | parse error |
| `(neq)` | load error: (query …): :over (neq a b) — one of all, minimal, maximal | parse error |
| `(eq)` | load error: (query …): :over (eq a b) — one of all, minimal, maximal | parse error |

#### claim fact argument

`(query :goal (p ?x) :expect (model (p a) (p {S})))`

| shape | today | the final grammar |
|---|---|---|
| `name` | exit 1: FAILED       p.ein | exit 1: FAILED       p.ein |
| `digits` | exit 1: FAILED       p.ein | exit 1: FAILED       p.ein |
| `-digits` | exit 1: FAILED       p.ein | exit 1: FAILED       p.ein |
| `1a` | load error: :expect lists (p 1 a), which no atom of the query's :goal matches | exit 1: FAILED       p.ein |
| `$x` | parse error | exit 1: FAILED       p.ein |
| `string` | exit 1: FAILED       p.ein | parse error |
| `range` | load error: :expect — `"1..5"` is not a fact argument | exit 1: FAILED       p.ein |
| `_` | load error: :expect — `_` is a wildcard; an expectation is an answer, not a pattern | parse error |
| `?x` | load error: :expect — `?x` is a variable; an expectation is an answer, not a pattern | parse error |
| `?_` | parse error | parse error |
| `a?x` | load error: :expect — `?x` is a variable; an expectation is an answer, not a pattern | parse error |
| `form` | exit 1: FAILED       p.ein | exit 1: FAILED       p.ein |
| `()` | exit 1: FAILED       p.ein | parse error |
| `:k v` | load error: :expect — `:k v` is not a fact argument | parse error |
| `(and)` | exit 1: FAILED       p.ein | parse error |
| `(not)` | exit 1: FAILED       p.ein | exit 1: FAILED       p.ein |
| `(neq)` | exit 1: FAILED       p.ein | parse error |
| `(eq)` | exit 1: FAILED       p.ein | exit 1: FAILED       p.ein |

#### :no-hypothesis item

`(solve :no-hypothesis ({S}))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone |
| `digits` | parse error | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone |
| `-digits` | parse error | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone |
| `1a` | parse error | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone |
| `$x` | parse error | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone |
| `string` | parse error | parse error |
| `range` | parse error | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone |
| `_` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone | parse error |
| `?x` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone | parse error |
| `?_` | parse error | parse error |
| `a?x` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone | parse error |
| `form` | parse error | parse error |
| `()` | parse error | parse error |
| `:k v` | parse error | parse error |
| `(and)` | parse error | parse error |
| `(not)` | parse error | parse error |
| `(neq)` | parse error | parse error |
| `(eq)` | parse error | parse error |

#### :hrules item

`(hrule h () :match (p ?x) :assert (q ?x)) (solve :hrules (h {S}))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `digits` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `-digits` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `1a` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `$x` | parse error | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `string` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `range` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `_` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `?x` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `?_` | parse error | parse error |
| `a?x` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `form` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |
| `()` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `:k v` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `(and)` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `(not)` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `(neq)` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | parse error |
| `(eq)` | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone | loads: states (k_s)      2 · universe          1 candidate, 0 dead alone |

#### config value

`(config :record-alternative-justifications {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | load error: (config …): config flag :record-alternative-justifications expects true/false, got Atom(name='a') | parse error |
| `digits` | load error: (config …): config flag :record-alternative-justifications expects true/false, got Int(value=7) | parse error |
| `-digits` | load error: (config …): config flag :record-alternative-justifications expects true/false, got Int(value=-7) | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | load error: (config …): config flag :record-alternative-justifications expects true/false, got String(value='s') | parse error |
| `range` | load error: (config …): config flag :record-alternative-justifications expects true/false, got Range(low=1, high=5) | parse error |
| `_` | load error: (config …): config flag :record-alternative-justifications expects true/false, got Wildcard() | parse error |
| `?x` | load error: (config …): config flag :record-alternative-justifications expects true/false, got Var(name='x') | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | load error: (config …): config flag :record-alternative-justifications expects true/false, got SForm(head=Atom(name='f'), args=(Atom(name='a'),)) | parse error |
| `()` | load error: (config …): config flag :record-alternative-justifications expects true/false, got SForm(head=Atom(name='@empty'), args=()) | parse error |
| `:k v` | parse error | parse error |
| `(and)` | load error: (config …): config flag :record-alternative-justifications expects true/false, got SForm(head=Atom(name='and'), args=(SForm(head=Atom(n… | parse error |
| `(not)` | load error: (config …): config flag :record-alternative-justifications expects true/false, got SForm(head=Atom(name='not'), args=(SForm(head=Atom(n… | parse error |
| `(neq)` | load error: (config …): config flag :record-alternative-justifications expects true/false, got SForm(head=Atom(name='neq'), args=(Atom(name='a'), A… | parse error |
| `(eq)` | load error: (config …): config flag :record-alternative-justifications expects true/false, got SForm(head=Atom(name='eq'), args=(Atom(name='a'), At… | parse error |

#### import module

`(import {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | load error: (import a) — file-relative import needs a base directory (load from a file path) at None | load error: (import a) — file-relative import needs a base directory (load from a file path) at None |
| `digits` | parse error | load error: (import 7) — file-relative import needs a base directory (load from a file path) at None |
| `-digits` | parse error | load error: (import -7) — file-relative import needs a base directory (load from a file path) at None |
| `1a` | parse error | load error: (import 1a) — file-relative import needs a base directory (load from a file path) at None |
| `$x` | parse error | load error: (import $x) — file-relative import needs a base directory (load from a file path) at None |
| `string` | parse error | parse error |
| `range` | parse error | load error: (import 1..5) — file-relative import needs a base directory (load from a file path) at None |
| `_` | parse error | parse error |
| `?x` | parse error | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | parse error | parse error |
| `()` | parse error | parse error |
| `:k v` | parse error | parse error |
| `(and)` | parse error | parse error |
| `(not)` | parse error | parse error |
| `(neq)` | parse error | parse error |
| `(eq)` | parse error | parse error |

#### import :symbols item

`(import std.macro :symbols ({S}))`

| shape | today | the final grammar |
|---|---|---|
| `name` | load error: (import std.macro :symbols …) — not provided by the module: a at None | load error: (import std.macro :symbols …) — not provided by the module: a at None |
| `digits` | parse error | load error: (import std.macro :symbols …) — not provided by the module: 7 at None |
| `-digits` | parse error | load error: (import std.macro :symbols …) — not provided by the module: -7 at None |
| `1a` | parse error | load error: (import std.macro :symbols …) — not provided by the module: 1a at None |
| `$x` | parse error | load error: (import std.macro :symbols …) — not provided by the module: $x at None |
| `string` | parse error | parse error |
| `range` | parse error | load error: (import std.macro :symbols …) — not provided by the module: 1..5 at None |
| `_` | load error: (import std.macro :symbols ()) — empty list at None | parse error |
| `?x` | load error: (import std.macro :symbols ()) — empty list at None | parse error |
| `?_` | parse error | parse error |
| `a?x` | load error: (import std.macro :symbols …) — not provided by the module: a at None | parse error |
| `form` | parse error | parse error |
| `()` | parse error | parse error |
| `:k v` | parse error | parse error |
| `(and)` | parse error | parse error |
| `(not)` | parse error | parse error |
| `(neq)` | parse error | parse error |
| `(eq)` | parse error | parse error |

#### import :as

`(import std.macro :as {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores nothing new | loads: stores nothing new |
| `digits` | load error: (import std.macro :as …) — alias must be a bare name at None | loads: stores nothing new |
| `-digits` | load error: (import std.macro :as …) — alias must be a bare name at None | loads: stores nothing new |
| `1a` | parse error | loads: stores nothing new |
| `$x` | parse error | loads: stores nothing new |
| `string` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |
| `range` | load error: (import std.macro :as …) — alias must be a bare name at None | loads: stores nothing new |
| `_` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |
| `?x` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |
| `()` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |
| `:k v` | parse error | parse error |
| `(and)` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |
| `(not)` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |
| `(neq)` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |
| `(eq)` | load error: (import std.macro :as …) — alias must be a bare name at None | parse error |

#### macro body

`(macro mm (?x) {S})`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores nothing new | parse error |
| `digits` | loads: stores nothing new | parse error |
| `-digits` | loads: stores nothing new | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: stores nothing new | parse error |
| `range` | loads: stores nothing new | parse error |
| `_` | loads: stores nothing new | parse error |
| `?x` | loads: stores nothing new | loads: stores nothing new |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: stores nothing new | loads: stores nothing new |
| `()` | loads: stores nothing new | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: stores nothing new | loads: stores nothing new |
| `(not)` | loads: stores nothing new | loads: stores nothing new |
| `(neq)` | loads: stores nothing new | loads: stores nothing new |
| `(eq)` | loads: stores nothing new | loads: stores nothing new |

#### trace :derives

`(trace (step s1 :rule x :derives {S}))`

| shape | today | the final grammar |
|---|---|---|
| `name` | loads: stores nothing new | parse error |
| `digits` | loads: stores nothing new | parse error |
| `-digits` | loads: stores nothing new | parse error |
| `1a` | parse error | parse error |
| `$x` | parse error | parse error |
| `string` | loads: stores nothing new | parse error |
| `range` | loads: stores nothing new | parse error |
| `_` | loads: stores nothing new | parse error |
| `?x` | loads: stores nothing new | parse error |
| `?_` | parse error | parse error |
| `a?x` | parse error | parse error |
| `form` | loads: stores nothing new | loads: stores nothing new |
| `()` | loads: stores nothing new | parse error |
| `:k v` | parse error | parse error |
| `(and)` | loads: stores nothing new | parse error |
| `(not)` | loads: stores nothing new | loads: stores nothing new |
| `(neq)` | loads: stores nothing new | parse error |
| `(eq)` | loads: stores nothing new | loads: stores nothing new |

#### A keyword the form does not take

| where | written | today | the final grammar |
|---|---|---|---|
| relation | `(relation r2 T :zz 1)` | loads: stores nothing new | parse error |
| rule | `(rule rr () :match (p ?x) :assert (hit ?x) :zz 1)` | loads: stores (hit a :rule rr) (hit b :rule rr) | parse error |
| hrule | `(hrule h () :match (p ?x) :assert (q ?x) :zz 1)` | loads: stores nothing new | parse error |
| fact | `(q c :zz 1)` | loads: stores (q c) | parse error |
| nested fact | `(r (f a :zz 1) c)` | loads: stores (r (f a) c) | parse error |
| stored negation | `(not (q c) :zz 1)` | loads: stores (not (q c)) | parse error |
| query | `(query :goal (p ?x) :zz 1)` | load error: (query …): unknown keyword :zz — one of :goal :goal-text :over :expect :mode | parse error |
| solve | `(solve :zz 1)` | load error: (solve …): unknown keyword :zz — one of :hrules :hypothesis-relations :no-hypothesis :expect | parse error |
| config | `(config :zz 1)` | load error: (config …): unknown config flag :zz (expected one of: lattice-sanity-check, record-alternative-justifications) | parse error |
| import | `(import std.macro :zz 1)` | loads: stores nothing new | parse error |
| trace step | `(trace (step s1 :rule x :zz 1))` | loads: stores nothing new | parse error |
| pattern in :match | `(rule rr () :match (p ?x :zz 1) :assert (hit ?x))` | loads: stores (hit a :rule rr) (hit b :rule rr) | parse error |
| and in :match | `(rule rr () :match (and (p ?x) :zz 1) :assert (hit ?x))` | loads: stores (hit a :rule rr) (hit b :rule rr) | parse error |
| pattern in :assert | `(rule rr () :match (p ?x) :assert (hit ?x :zz 1))` | exit 1: rule 'rr': its :assert writes :zz 1 as an argument of hit, and no fact can hold it | parse error |
| and in :assert | `(rule rr () :match (p ?x) :assert (and (hit ?x) :zz 1))` | loads: stores (hit a :rule rr) (hit b :rule rr) | parse error |
| :goal pattern | `(query :goal (p ?x :zz 1))` | loads: stores nothing new | parse error |
| :using of a fact | `(q c :rule x :using (u (r a b :zz 1)))` | loads: stores (q c :rule x) | parse error |
| query :mode | `(query :goal (p ?x) :mode x)` | loads: states (k_s)      32 · universe          5 candidates, 0 dead alone · ?x = a · ?x = b · as :expect        (model (p a) (p b)) | parse error |
| relation :cardinality | `(relation r2 T :cardinality 1..1)` | loads: stores nothing new | parse error |
| fact :why | `(q c :why "text")` | loads: stores (q c) | parse error |
| rule :where | `(rule rr () :match (and (p ?x) :where (neq ?x a)) :assert (hit ?x))` | loads: stores (hit a :rule rr) (hit b :rule rr) | parse error |
