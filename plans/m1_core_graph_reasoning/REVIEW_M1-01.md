Я посмотрел текущий `master` именно как reasoning engine, а не просто Python-проект. Общая оценка: **архитектурно проект уже интересный и довольно цельный, но сейчас есть несколько мест, где формальные claims сильнее, чем гарантии реализации**. Это важнее обычного code-quality review.

### Что выглядит хорошо

Главное архитектурное решение — разделить систему на monotone deductive layer и non-monotone search layer — правильное. По сути сейчас Ein действительно выглядит как смесь Datalog + CSP/SAT + ATMS: rule compilation → semi-naive saturation → forked commitment search → contradiction/no-good → provenance. Это хорошо отражено и в коде, и в документации.

Особенно удачным считаю переход к **одному `solve()`**, где `Solution/Ambiguity/Contradiction` являются результатами одного процесса, а не тремя режимами. Это устраняет довольно фундаментальный semantic smell старой архитектуры. В самом solver это проводится последовательно: solution node определяется как `complete ∧ consistent`, а query используется потом как projection модели.

Хорошее решение и здесь:

```text
commitment C
    ↓
fork(root)
    ↓
write hypotheses
    ↓
saturate to fixpoint
    ↓
detect contradiction
```

То есть branch state изолирован, а shared root специально оставлен стабильным во время Phase 2. Причём в коде уже явно зафиксировано, почему старое распространение fork-derived facts в root было unsound при NAF.

Инженерная сторона тоже выглядит значительно взрослее типичного research prototype: package layout, typed AST/IR, отдельный KB, compile/match/saturate/search, provenance, trace, CLI, ruff/pytest/vulture, большое количество fixtures. `pyproject.toml` достаточно аккуратный.

---

## 1. Самая серьёзная вещь: `state_hash()` сейчас нельзя использовать как идентичность модели

Сейчас:

```python
return hash(tuple(sorted(...)))
```

а solution nodes deduplicate именно по `state_hash`.

Это означает:

```text
logical state
    ↓
Python hash : int
    ↓
identity
```

Но должно быть:

```text
logical state
    ↓
canonical representation
    ↓
identity

hash(canonical representation)
    ↓
только accelerator/index
```

Коллизия Python `hash()` сейчас теоретически способна превратить две разные модели в одну. А поскольку **число distinct models `k` определяет сам verdict**, collision — это уже не performance bug, а soundness bug:

```text
M1 != M2
hash(M1) == hash(M2)

реально: k = 2 → Ambiguity
Ein:     k = 1 → Solution
```

README прямо определяет `k` как смысл результата.

Я бы заменил `state_hash` концептуально на:

```python
StateKey = tuple[CanonicalFact, ...]
```

и именно `StateKey` использовал как dictionary/set key. Python сам его захеширует, но при hash collision dict дополнительно проверит equality tuple, поэтому корректность уже не зависит от отсутствия collisions.

Если объект получается слишком большим:

```text
digest -> bucket
         ↓
canonical equality verification
```

но **digest alone как identity использовать нельзя**.

Дополнительно я бы убрал из semantic canonicalization всё, что не является частью модели. Сейчас hash включает:

```python
f.layer.value
```

хотя фактическая identity самого `Fact` в KB в других местах определена как `(relation_name, args)` и игнорирует layer/provenance. `add_and_index_fact()` именно так dedup'ит.

Нужно явно определить:

> Что такое extensional identity модели Ein?

По текущей семантике мне кажется естественным:

[
M = {(R,args)}
]

а не:

[
M = {(layer,R,args)}.
]

Это я бы поставил **P0**.

---

## 2. Есть прямое противоречие между кодом и документацией по `unconditional_facts`

`commitment.py` до сих пор утверждает:

> derivation doesn't touch commitment ⇒ fact is provably true at root

и реально вычисляет `unconditional_facts`.

Architecture document тоже говорит, что alive branch:

> merges its unconditional consequences into root.

Но актуальный solver говорит прямо противоположное:

> **Do NOT merge unconditional facts ... extraction is UNSOUND under NAF (`absent`).**

И solver здесь прав.

Проблема фундаментальная. Для NAF зависимость существует не только по **positive provenance edges**, но также через **отсутствие факта**.

Например:

```text
h causes X
rule: absent X -> Y
```

В branch без `X`:

```text
absent X
⇒ Y
```

Но provenance `Y` может не содержать positive edge к `h`.

Поэтому обычный DAG provenance:

```text
Y
↑
positive premises
```

недостаточен для определения independence от assumptions. Нужен dependency object примерно:

[
Deps(Y)=PositiveDeps(Y)\cup NegativeDeps(Y)
]

или environment/provenance с отрицательными assumptions.

Сам solver уже сделал правильный conservative choice: **не merge**.

Я бы теперь полностью удалил:

```python
CommitmentSetResult.unconditional_facts
_is_unconditional()
```

если они больше нигде не имеют корректного применения. Не оставлял бы это как dormant API — оно концептуально опасное.

---

## 3. `minimal_unsat_core` называется сильнее, чем реально делает

Сейчас алгоритм определён как:

> smallest single-contradiction source frontier.

Это полезная вещь, но это **не обязательно minimal unsatisfiable core в стандартном смысле**.

Обычный MUS — subset-minimal unsatisfiable subset:

[
C \text{ unsat},
\qquad
\forall c\in C:\ C-{c}\text{ sat}.
]

А здесь выбирается минимальный source frontier среди существующих contradiction derivations.

Это скорее:

* minimal provenance explanation;
* smallest observed contradiction justification;
* minimal derivation frontier.

Причём KB хранит один `Fact` на `(relation,args)` и при повторном выводе возвращает уже существующий объект, не добавляя новую provenance alternative.

Следовательно:

```text
A ──┐
B ──┴→ X

C ─────→ X
```

если первым сохранился proof `A,B → X`, альтернативный `C → X` может вообще не стать частью provenance объекта `X`.

Тогда explanation зависит от порядка derivation/firing и может быть:

```text
{A,B}
```

хотя существует более короткое:

```text
{C}
```

То есть даже **smallest provenance explanation over all derivations** пока не гарантируется, если provenance не хранит alternatives.

Я бы либо переименовал API:

```python
smallest_contradiction_frontier()
```

либо расширил provenance до OR/AND proof DAG:

```text
Fact X
 ├─ justification 1: A ∧ B
 └─ justification 2: C
```

Тогда уже можно искать minimal explanation.

Это **P1**, потому что сейчас README обещает «minimal unsat core».

---

## 4. Самая интересная архитектурная проблема Ein сейчас — не RA, а точная семантика NAF

Судя по коду, именно `absent` стал местом, которое разделяет систему на:

```text
positive monotone reasoning
        +
non-monotone observations of absence
```

И из этого уже появились реальные последствия:

* fork facts нельзя безусловно backpropagate;
* provenance только positive premises недостаточен;
* deletion-based MUS minimization становится tricky;
* saturation ordering требует fire-time re-evaluation `AbsentGuard`;
* semi-naive triggering должен учитывать изменение relations внутри `AbsentGuard`.

Это всё видно в implementation — и это хороший знак: проблемы обнаружены практически, а не замаскированы.

Но теперь я бы **формализовал semantics `absent` отдельным документом**, прежде чем расширять язык.

Нужно определить хотя бы:

[
KB \models absent(P)
]

что именно означает?

### Closed-world?

[
P\notin closure(KB)
\Rightarrow absent(P)
]

### Stratified NAF?

Тогда должна существовать dependency stratification.

### Branch-relative epistemic statement?

[
KB_C \not\models P
]

где (C) — commitment environment.

Последний вариант, судя по фактическому поведению Ein, ближе всего.

Тогда `absent(P)` — фактически не ground logical atom, а **query over current saturated world/environment**.

И это очень важное различие.

---

## 5. Commitment lattice — интересная идея, но я бы перестал слишком близко называть её CDCL

В документации:

```text
no-good ≈ CDCL conflict clause
```

как аналог — нормально.

Но algorithmically это пока гораздо ближе к:

```text
ATMS environments
+
Apriori subset enumeration
+
nogood subset pruning
```

чем к CDCL.

CDCL делает:

```text
decision trail
→ implication graph
→ conflict analysis
→ learned asserting clause
→ non-chronological backjump
```

Ein:

```text
set C
→ fork
→ saturate whole C
→ dead
→ learn C/no-good
→ suppress supersets
```

То есть learned clause часто представляет basically dead environment, а не минимизированный implication-graph conflict clause.

Это не критика алгоритма — наоборот, **set-lattice search является одной из наиболее оригинальных частей Ein**. Я бы просто позиционировал её точнее:

> ATMS-style environment search with Apriori candidate generation and nogood learning.

А CDCL оставить как analog / possible optimization direction.

---

## 6. Есть хороший архитектурный seam для следующего этапа

Я бы видел ядро примерно так:

```text
                 ┌───────────────┐
                 │   ein-lang    │
                 └───────┬───────┘
                         │
                    typed IR
                         │
                 ┌───────▼───────┐
                 │      KB       │
                 │ ground atoms  │
                 └───────┬───────┘
                         │
       ┌─────────────────┴─────────────────┐
       │                                   │
┌──────▼──────┐                     ┌──────▼──────┐
│ monotone    │                     │ assumptions │
│ closure     │                     │ / worlds    │
│ Datalog-ish │                     │ lattice     │
└──────┬──────┘                     └──────┬──────┘
       │                                   │
       └───────────────┬───────────────────┘
                       │
                 complete model
                       │
                 canonical key
                       │
             models / refutations
```

А NAF должен сидеть **на границе closure/world**, а не выглядеть обычной разновидностью positive premise.

Это сделает дальнейшую SMT-интеграцию значительно понятнее.

---

# Моя итоговая оценка

Если оценивать как **research/learning prototype**, я бы дал примерно **8/10 по архитектуре**. Здесь уже нет ощущения «кучи special cases для Zebra»; присутствуют реальные abstractions: IR → KB → compiled joins → saturation → search → verdict/proof.

Если оценивать как **формально sound solver framework**, пока скорее **6/10**, главным образом из-за того, что некоторые слова — *minimal*, *canonical hash*, *CDCL*, *stratified negation*, *unconditional* — обещают более строгие свойства, чем те, которые сейчас реально гарантированы.

При этом я не вижу необходимости менять основную архитектуру. Я бы поставил следующие приоритеты:

1. **P0:** заменить `state_hash` как identity на collision-safe canonical state key.
2. **P0:** окончательно удалить/переформулировать `unconditional_facts` и синхронизировать docs с текущей NAF-safe моделью.
3. **P1:** зафиксировать формальную semantics `absent` / `forall` / completeness относительно commitment world.
4. **P1:** переименовать текущий `minimal_unsat_core` либо сделать provenance multi-justification и определить точный вид minimality.
5. **P2:** после этого уже оптимизировать lattice search — propagation, conflict minimization, variable/value selection, backjump-like behaviour — вместо дальнейшего наращивания Apriori BFS.

Самая перспективная часть Ein сейчас, на мой взгляд, — именно сочетание **typed relational graph + saturation + explicit assumption environments/provenance**. И наиболее опасная зона одновременно — граница между monotonic deduction и `absent`/closed-world reasoning. Там я бы сейчас сосредоточил формализацию и тестирование.
