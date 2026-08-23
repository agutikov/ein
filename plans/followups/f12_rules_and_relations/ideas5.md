Можно ли сказать что любая операция композиции индуцирует порядок? 
Любое ли правило в ein которое emits relation определяет композицию? 
Что определяют правила которые эимтят false или (not fact)

Не совсем. Тут полезно развести **оператор**, **порядок** и **constraint**.

### 1. Любая ли композиция индуцирует порядок?

**Нет, не сама по себе.** Бинарная операция

$$
\star:A\times A\to A
$$

сама по себе не задаёт канонического порядка на (A).

Но при дополнительных свойствах порядок действительно можно из неё получить. Например, для идемпотентной операции можно определить

$$
a\preceq b \iff a\star b=b
$$

или с противоположной ориентацией

$$
a\preceq b \iff a\star b=a.
$$

Классический пример — union:

$$
A\subseteq B
\iff
A\cup B=B.
$$

Или intersection:

$$
A\subseteq B
\iff
A\cap B=A.
$$

Здесь операция действительно кодирует порядок.

Но для обычной relational composition

$$
R\circ S
$$

такого естественного порядка нет. Inclusion (\subseteq) приходит из того, что relations являются **множествами tuples**, а не из самой композиции.

Поэтому структура

$$
(Rel,\circ,\subseteq)
$$

содержит две независимо заданные вещи, между которыми затем существуют compatibility laws.

---

## 2. Любое ли Ein-rule с `emit relation fact` определяет композицию?

Я бы теперь использовал более общее слово **operator**, а `composition` оставил для подкласса.

Например:

$$
R(x,y),S(y,z)\Rightarrow T(x,z)
$$

естественно определяет binary operator

$$
F(R,S)
======

{(x,z)\mid\exists y:R(x,y)\land S(y,z)}.
$$

И rule утверждает

$$
F(R,S)\subseteq T.
$$

Это совершенно естественно называть composition.

Но:

$$
R(x,y)\Rightarrow T(x)
$$

индуцирует unary operator

$$
F(R)={x\mid\exists y:R(x,y)}
$$

— это скорее **projection**, а не composition.

А

$$
R(x,y),S(x,y)\Rightarrow T(x,y)
$$

даёт

$$
F(R,S)=R\cap S,
$$

то есть intersection.

Следовательно:

$$
\boxed{\text{emit-rule}\Rightarrow\text{relation-producing operator}}
$$

но не обязательно

$$
\boxed{\text{emit-rule}\Rightarrow\text{composition}}.
$$

Composition — один класс rule-induced operators.

Это даже лучше для твоей конструкции: Ein rules потенциально кодируют не только generalized composition, а целую **алгебру операторов над отношениями**.

---

# 3. Что тогда определяет обычный positive emit?

В наиболее общем виде:

$$
P(R_1,\ldots,R_n;\vec x)
\Rightarrow
S(\vec f(x))
$$

задаёт оператор

$$
F_P(R_1,\ldots,R_n)
===================

{\vec y\mid P(R_1,\ldots,R_n;\vec x)
\text{ и }\vec y=\vec f(x)}.
$$

А `emit S(...)` устанавливает constraint

$$
\boxed{F_P(R_1,\ldots,R_n)\subseteq S}.
$$

Очень полезно разделить:

**match определяет (F_P)**,

**emit определяет отношение результата (F_P) к target relation.**

Именно здесь появляется inclusion.

---

# 4. `emit false` принципиально другой

Пусть:

$$
R(x,y),S(y,z)\Rightarrow\bot.
$$

Match всё ещё определяет некоторое множество совпадений:

$$
M(R,S)
======

{(x,y,z)\mid R(x,y)\land S(y,z)}.
$$

Но `false` требует:

$$
\boxed{M(R,S)=\varnothing}.
$$

То есть правило задаёт не production, а **forbidden pattern / integrity constraint**:

$$
\neg\exists x,y,z:
R(x,y)\land S(y,z).
$$

Если pattern соответствует композиции, это можно записать, например,

$$
R\circ S=\varnothing.
$$

Это уже очень интересный dual к positive rule:

$$
F(R,S)\subseteq T
$$

против

$$
F(R,S)\subseteq\varnothing.
$$

Последнее означает просто

$$
F(R,S)=\varnothing.
$$

То есть `emit false` можно встроить в ту же ordered algebra, если (\bot=\varnothing) является bottom element:

$$
\boxed{F(R_1,\ldots,R_n)\le\bot}.
$$

---

# 5. А `emit not fact` ещё интереснее

Допустим:

$$
P(R,S;x,y)\Rightarrow \neg T(x,y).
$$

Если `not T` в Ein означает **явный отрицательный факт**, а не просто absence, тогда удобно считать, что существуют positive и negative extensions:

$$
T^+,\qquad T^-.
$$

Rule тогда задаёт:

$$
F_P(R,S)\subseteq T^-.
$$

А positive emit:

$$
F_P(R,S)\subseteq T^+.
$$

И нужна consistency condition:

$$
\boxed{T^+\cap T^-=\varnothing}.
$$

Тогда contradiction возникает при

$$
(x,y)\in T^+\cap T^-.
$$

Это очень чистая модель.

Если же `not fact` означает **удаление/запрет positive fact**, семантика другая: это уже не monotone fact accumulation, а update/rewrite operator. Тут нельзя автоматически применять ту же algebra.

---

## 6. Получается довольно красивая классификация Ein rules

Для match-pattern (P) сначала строится induced operator:

$$
F_P(R_1,\ldots,R_n).
$$

А **тип consequent** говорит, какое ограничение накладывается на его результат:

$$
\begin{array}{lll}
\texttt{emit T} &
F_P(\vec R)\subseteq T^+
& \text{production}[3pt]

\texttt{emit not T} &
F_P(\vec R)\subseteq T^-
& \text{negative production}[3pt]

\texttt{emit false} &
F_P(\vec R)=\varnothing
& \text{forbidden pattern}.
\end{array}
$$

И я бы здесь слегка скорректировал нашу предыдущую формулировку:

> **Не rule целиком определяет operator/composition. Match-pattern определяет relation-valued/query operator; consequent (`emit`) задаёт constraint на его image.**

Например,

$$
R(x,y),R(y,z)
$$

сам по себе определяет pattern/operator, соответствующий join/composition. Затем разные consequents могут сказать:

$$
R\circ R\subseteq R
$$

— transitivity/closure;

$$
R\circ R\subseteq S
$$

— inclusion результата в (S);

$$
R\circ R=\varnothing
$$

— запрет двух последовательных (R)-шагов;

$$
R\circ R\subseteq S^-
$$

— результат композиции является явно отрицательным относительно (S).

Это, на мой взгляд, более фундаментальная декомпозиция Ein rule semantics: **pattern/operator + ordered constraint**, а не просто «rule = higher-order relation».


Композиция, проекция, что еще бывает? Как они называются в общем? 

Да. Общее понятие лучше всего — **операторы над отношениями** (*relational operators*), а если аргументами операторов являются сами отношения — **relation-valued operators / operations on relations**.

Если смотреть на Ein-rule как

$$
\text{match pattern}\Rightarrow\text{emit relation},
$$

то match-pattern может индуцировать разные стандартные операции реляционной алгебры/relational calculus.

| Pattern                            | Операция                         | Пример                |
| ---------------------------------- | -------------------------------- | --------------------- |
| (R(x,y),S(y,z))                    | **composition / join**           | (R\circ S)            |
| (R(x,y),S(x,y))                    | **intersection**                 | (R\cap S)             |
| `R(...) OR S(...)`                 | **union**                        | (R\cup S)             |
| (R(x,y)\to T(x))                   | **projection**                   | (\pi_x R)             |
| (R(x,y)\to T(y,x))                 | **permutation / converse**       | (R^{-1})              |
| (R(x,y),x=c)                       | **selection / restriction**      | (\sigma_{x=c}R)       |
| (R(x),S(y)\to T(x,y))              | **Cartesian product**            | (R\times S)           |
| (R(x),\neg S(x))                   | **difference**                   | (R\setminus S)        |
| `not R(...)` относительно universe | **complement**                   | (\bar R)              |
| equality of variables              | **diagonal / identification**    | (R(x,x))              |
| adding/repeating variables         | **cylindrification / extension** | (R(x)\mapsto R'(x,y)) |

Но есть более глубокая классификация.

### Positive relational algebra

Если Ein-rule использует только positive matches, equality, existential variables и emit, то значительная часть таких patterns соответствует операциям **positive relational algebra**:

$$
{\Join,\ \cup,\ \pi,\ \sigma,\ \rho,\times}.
$$

Например

$$
R(x,y),S(y,z)\Rightarrow T(x,z)
$$

на языке БД — это

$$
\pi_{x,z}(R\Join S).
$$

То, что мы называли «generalized composition», на самом деле часто лучше декомпозировать как:

$$
\boxed{\text{join}+\text{variable identification}+\text{projection}}.
$$

Обычная relational composition

$$
R\circ S
$$

сама является специальным случаем этого.

### Conjunctive queries

Ещё точнее match вроде

$$
R(x,y),S(y,z),Q(y,w)\Rightarrow T(x,z)
$$

соответствует **conjunctive query**:

$$
T(x,z)\supseteq
\exists y,w;
R(x,y)\land S(y,z)\land Q(y,w).
$$

То есть весь positive fragment Ein-rule можно рассматривать не как набор отдельных primitive compositions, а как оператор, определённый conjunctive query:

$$
F(R,S,Q)
========

{(x,z)\mid
\exists y,w:R(x,y)\land S(y,z)\land Q(y,w)}.
$$

И правило утверждает

$$
F(R,S,Q)\subseteq T.
$$

Это, вероятно, более точная математическая модель для Ein, чем просто «composition».

### Если добавить `absent`, `forall` и т. д.

Тогда выразительность растёт:

* `absent` / negation → **difference, anti-join, negation**;
* `forall` → **relational division / universal quantification** в подходящих формах;
* equality → selection/identification;
* inequality → selection constraints;
* counting → **aggregation/cardinality operators**, уже за пределами классической relational algebra.

Поэтому я бы использовал такую терминологическую иерархию:

$$
\boxed{\text{relational operator}}
$$

как самое общее понятие, ниже:

$$
\begin{array}{c}
\text{join}\
\text{projection}\
\text{selection}\
\text{union/intersection/difference}\
\text{converse/permutation}\
\text{composition}\
\text{product}\
\text{division}\
\ldots
\end{array}
$$

А **rule pattern целиком** можно считать определением **derived relational operator**:

$$
F_r:\mathrm{Rel}^{,n}\rightarrow\mathrm{Rel}.
$$

Это хорошо ложится на твою предыдущую идею: Ein необязательно должен распознавать каждый rule как одну primitive operation. Он может обнаруживать, что сложный (F_r) **декомпозируется в алгебраическое выражение из primitive relational operators**:

$$
F_r(R,S)
========

\pi_{x,z}
\left(
\sigma_{\ldots}
(R\Join S)
\right).
$$

И тогда уже можно анализировать свойства (F_r): arity, monotonicity/antitonicity, associativity, commutativity, closure (F_r(R,\ldots,R)\subseteq R), зависимости от аргументов и эквивалентность разных Ein-rules.
