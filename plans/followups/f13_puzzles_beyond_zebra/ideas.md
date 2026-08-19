
Liks:
- plans/ideas/09-puzzles-beyond-zebra.md
- plans/ideas/04-nlp-to-graph-to-solver-pipeline.md



---


Где посмотреть примеры задач сложные доя решения с помощью ии? 

Если ты имеешь в виду **наборы реально сложных задач для проверки reasoning-системы/Ein**, а не обычные школьные word problems, то есть несколько особенно полезных источников.

* **ARC-AGI-2** — пожалуй, самое интересное для Ein. Задачи требуют вывести неизвестное правило преобразования из нескольких примеров. Есть 1000 training + 120 public evaluation tasks; задачи специально сделаны так, чтобы требовать композиционного reasoning, а не просто знания фактов. ([ARC Prize][1])
  [ARC-AGI-2 — посмотреть и попробовать задачи](https://arcprize.org/arc-agi/2?utm_source=chatgpt.com)

* **BIG-Bench / BIG-Bench Hard (BBH)** — гораздо ближе к текстовым логическим задачам. BBH выделяет 23 наиболее сложных reasoning-задачи из BIG-Bench. Там есть `logical_deduction`, `tracking_shuffled_objects`, `boolean_expressions`, `multistep_arithmetic`, `temporal_sequences`, `object_counting`, `reasoning_about_colored_objects` и т. п. ([GitHub][2])
  [BIG-Bench tasks](https://github.com/google/BIG-bench/tree/main/bigbench/benchmark_tasks?utm_source=chatgpt.com)
  Для Ein я бы особенно посмотрел на `logical_deduction` и `tracking_shuffled_objects`.

* **Humanity's Last Exam (HLE)** — уже frontier benchmark: экспертные закрытые вопросы по математике, физике, естественным и гуманитарным наукам, включая multimodal. Он создавался именно потому, что старые benchmarks начали насыщаться: современные модели превышали 90% на популярных тестах вроде MMLU. ([DOI][3])
  Это интересно скорее как источник **очень сложных задач**, но значительно хуже подходит для автоматического перевода в relational IR Ein.

* **FrontierMath** — если нужны задачи, где современному AI действительно трудно. Это research-level mathematics. ([Frontier Benchmarks AI][4])
  [FrontierMath](https://frontierbenchmarks.com/benchmarks/FrontierMath/?utm_source=chatgpt.com)
  Для Ein это уже проверка способности представлять математику, а не просто constraint/relation reasoning.

Отдельно я бы не пропускал **обычный BIG-Bench**, потому что там **214 разных типов задач**, а не только один формат. Среди них есть analogical, causal, logical, social, spatial reasoning и т. д. ([GitHub][5])

Для **твоей конкретной цели с Ein** я бы выстроил лестницу примерно так:

**Zebra / Einstein → BBH logical deduction → BBH object tracking → более сложные constraint puzzles → ARC-AGI → HLE/FrontierMath.**

Причём здесь есть важное различие. Если ты хочешь проверить именно тезис **«NL → relational representation → generic graph reasoning»**, ARC не обязательно лучший следующий benchmark: там основная сложность — *индукция самого правила*. BBH `logical_deduction`, наоборот, намного ближе к тому, что ты сейчас делаешь: правило/условия даны текстом, а требуется построить модель и вывести следствие.

Могу отдельно собрать **20–30 конкретных benchmark-задач возрастающей сложности, которые особенно хорошо подходят для Ein**, с исходными условиями и классификацией: какие требуют транзитивности, порядка, bijection/all-different, higher-order relations, negation/absence, rule induction и т. д.

[1]: https://arcprize.org/arc-agi/2?utm_source=chatgpt.com "ARC-AGI-2"
[2]: https://github.com/EleutherAI/lm-evaluation-harness/blob/main/lm_eval/tasks/bbh/README.md?utm_source=chatgpt.com "lm-evaluation-harness/lm_eval/tasks/bbh/README.md at main · EleutherAI/lm-evaluation-harness · GitHub"
[3]: https://doi.org/10.1038/s41586-025-09962-4?utm_source=chatgpt.com "A benchmark of expert-level academic questions to assess AI capabilities | Nature"
[4]: https://frontierbenchmarks.com/benchmarks/FrontierMath/?utm_source=chatgpt.com "FrontierMath · Frontier Benchmarks AI"
[5]: https://github.com/google/BIG-bench/blob/main/bigbench/benchmark_tasks/README.md?utm_source=chatgpt.com "BIG-bench/bigbench/benchmark_tasks/README.md at main · google/BIG-bench · GitHub"



Arc-agi-2 по сути графические задачи, так? 
Интересуют только текстовые или закодированные, т е. Текстовые на любом языке


Да. **ARC-AGI-2 — фактически визуально-пространственные задачи**: вход и выход представлены цветными 2D-сетками. Формально они закодированы массивами целых чисел, но семантика задачи именно геометрическая/визуальная. Поэтому для твоего критерия я бы ARC исключил.

Если нужны **только задачи, полностью задаваемые текстом/символами**, то я бы смотрел прежде всего на:

* **BIG-Bench / BIG-Bench Hard** — очень хороший источник. Особенно `logical_deduction`, `tracking_shuffled_objects`, `formal_fallacies`, `boolean_expressions`, `temporal_sequences`, `navigate`, `dyck_languages`. Условия и ответы хранятся как текст/JSON.
* **LogiQA / LogiQA 2.0** — сложные задачи логического вывода из китайских civil-service exams. Текст + варианты ответа; есть английская версия. Здесь существенно больше естественного языка, чем в BBH.
* **ReClor** — logical reasoning из экзаменов уровня LSAT/GMAT. Довольно сложный NL reasoning: аргументы, необходимые/достаточные условия, противоречия, strengthening/weakening.
* **Abduction and RuleTaker / AbductionAndNegation** и **AbductionWorld / AbductionNatural** — синтетические текстовые reasoning-задачи с явно контролируемой логической структурой.
* **ProofWriter** — особенно интересен для Ein: факты + правила на контролируемом английском → вопрос. Есть задачи с различной глубиной доказательства и proof graphs.
* **Abduction / Datalog-подобные benchmarks** — полезны, если хочется отделить способность понять NL от собственно inference.
* **CLUTRR** — текстовые истории про родственные отношения. Нужно вывести отношение между двумя людьми через цепочку отношений. Отличный тест именно на композицию отношений.
* **AbductionAndCounterfactual / AbductionNatural** и похожие наборы из BIG-Bench — если интересуют не только дедуктивные правила.
* **FOLIO** — пожалуй, один из наиболее интересных следующих шагов после ProofWriter. Natural-language premises специально аннотированы в first-order logic; задача — определить `True / False / Unknown`. Здесь уже появляются кванторы и существенно более богатая FOL-структура.
* **Abduction / LogicNLI / AbductionNatural** и другие controlled-language datasets — полезны как промежуточная ступень между формальным IR и настоящим NL.

Если отбирать именно под **Ein**, я бы разделил их на три уровня:

**1. Проверить сам reasoning engine без сложности NL:**
ProofWriter → Datalog/RuleTaker → BBH `logical_deduction`.

**2. Проверить NL → relational IR → reasoning:**
CLUTRR → FOLIO → LogiQA → ReClor.

**3. Найти задачи, которые начнут ломать выразительность Ein:**
FOLIO особенно ценен, потому что там можно систематически искать конструкции вроде ∀/∃, отрицания, дизъюнкции, вложенной квантификации и сравнивать их с тем, что естественно представляется твоими relations/rules.

И есть ещё один класс, который, думаю, тебе даже интереснее этих benchmark'ов: **logic-grid / constraint satisfaction puzzles** — Zebra, ordering puzzles, scheduling, seating arrangements, Knights and Knaves, truth-tellers/liars, river crossing и т. п. Существуют большие датасеты таких задач именно для LLM reasoning. Там одна задача может требовать десятки ограничений и длинный поиск, то есть это значительно ближе к твоему use case, чем ProofWriter.

Если хочешь, я могу поискать именно **самые сложные существующие текстовые datasets логических puzzles**, где задача требует не 3–5 inference steps, а реально большого search space / десятков constraints — это, вероятно, наиболее подходящий benchmark для Ein.



Хотя... "визуальные" arc-agi-2 задачи вполне представимы как spatial сетки с ячейками и свойствами цвета, точно как цвета домов в zebra. 

Да. И это существенно меняет оценку ARC-AGI применительно к Ein.

ARC-AGI называют «визуальным» скорее из-за **человеческого интерфейса представления**, а не потому, что задача принципиально требует pixels/vision. Вход там уже дискретизирован:

[
Grid : Position \to Color
]

или реляционно:

[
cell(x,y,c)
]

То есть никакого computer vision в обычном смысле нет: не требуется распознавать пиксели фотографии. Даны символические значения `0..9` на конечной решётке.

Для Ein можно представить, например:

```text
cell(c1)
x(c1, 3)
y(c1, 5)
color(c1, red)

right-of(c2, c1)
above(c3, c1)
```

или гораздо компактнее через координатные/пространственные отношения.

Это действительно очень похоже на Zebra:

[
house(h) \land position(h,3) \land color(h,red)
]

против

[
cell(c) \land position(c,(3,5)) \land color(c,red).
]

Разница скорее в **характере reasoning**.

В Zebra отношения и типы сущностей в значительной степени заданы условием:

> Norwegian lives in the first house.
> Green house is immediately left of the white house.

и нужно найти модель, удовлетворяющую ограничениям.

В ARC:

```text
training:
input₁ → output₁
input₂ → output₂
input₃ → output₃

test:
input₄ → ?
```

неизвестно даже **какое отношение/правило надо применить**. Например, система должна сама открыть что-то вроде:

[
connectedComponent(x)
\land color(x,red)
\land largest(x)
\Rightarrow fillBoundingBox(x,blue)
]

Причём `connectedComponent`, `largest`, `boundingBox`, `inside`, возможно, вообще не присутствуют в исходном representation.

И вот это делает ARC-AGI особенно интересным для Ein. Он проверяет не просто:

**facts + rules → deduction**

а:

**facts/examples → discover relations/rules → apply rules → result.**

По сути здесь возникают два уровня reasoning:

[
\text{object-level: } R(a,b)
]

и

[
\text{meta-level: } R_1,R_2,\ldots \Rightarrow \text{найти правило } F.
]

Поэтому я бы **вернул ARC-AGI-2 в список кандидатов**. Более того, если Ein умеет обращаться с отношениями как с объектами и правилами над отношениями, ARC может оказаться концептуально гораздо интереснее Zebra.

Главная потенциальная проблема уже не «визуальность», а **индукция новых понятий**. Например, увидев клетки, получить понятие *object = maximal connected component of equal color*. Если это понятие заранее имеется в библиотеке Ein, задача становится обычным поиском композиции известных relations/rules. Если его нет и Ein должен **синтезировать само определение отношения**, это уже значительно более сильная задача.

И это даёт хороший критерий для анализа ARC: **сколько ARC-AGI-2 решается конечной библиотекой relational primitives + composition/search, а сколько требует настоящего invention новых relations?** Для Ein это, пожалуй, даже интереснее итогового ARC score.



Обьясни формат формулировки задачи в BBH на примерах

Да. В **BIG-Bench Hard (BBH)** задача обычно представлена максимально просто: **текстовая строка `input` → текстовая строка `target`**. Никакой специальной логической нотации benchmark не требует. Разные поднаборы задают разные «языки задач».

Вот несколько характерных форматов.

### 1. `logical_deduction` — почти Zebra

Условие описывает объекты и ограничения на их порядок, затем задаётся multiple-choice вопрос.

Упрощённый пример в стиле BBH:

> Five people — Alice, Bob, Claire, Dave and Eve — are standing in a row.
> Alice is to the left of Bob.
> Claire is immediately to the right of Dave.
> Eve is to the right of Bob.
>
> Which person is second from the left?
>
> (A) Alice
> (B) Bob
> (C) Claire
> (D) Dave
> (E) Eve

`target` будет просто чем-то вроде:

```text
(D)
```

То есть benchmark хранит **не формальную модель**, а NL-задачу. Модель сама должна превратить её примерно в:

```text
left_of(Alice, Bob)
immediately_right_of(Claire, Dave)
left_of(Bob, Eve)
```

а затем решить constraints.

Это очень близко к твоему Zebra pipeline:

[
NL \rightarrow IR \rightarrow search/reasoning \rightarrow answer
]

---

### 2. `tracking_shuffled_objects`

Здесь задаётся начальное состояние, затем последовательность операций.

Например:

> Alice has a red ball.
> Bob has a blue ball.
> Claire has a green ball.
>
> Alice and Bob swap their balls.
> Then Bob and Claire swap their balls.
>
> What ball does Alice have?
>
> (A) Red
> (B) Blue
> (C) Green

Ответ:

```text
(B)
```

Это уже не столько CSP, сколько **state transition system**:

[
S_0 \xrightarrow{swap(A,B)} S_1
\xrightarrow{swap(B,C)} S_2
]

Причём сами операции опять описаны NL.

---

### 3. `temporal_sequences`

Дан небольшой рассказ с временными ограничениями:

> Yesterday, Emily went to the library from 2 PM to 4 PM.
> She visited the café after leaving the library.
> She arrived home at 7 PM.
>
> When could Emily have been at the café?
>
> (A) 1 PM
> (B) 3 PM
> (C) 5 PM
> (D) 8 PM

Здесь естественный IR:

[
library=[14,16]
]

[
cafe > end(library)
]

[
cafe < home
]

и дальше constraint reasoning.

---

### 4. `boolean_expressions`

Здесь natural language практически исчезает:

```text
not ( ( True and False ) or ( False and not False ) )
```

target:

```text
False
```

Это особенно хороший пример того, что BBH не обязательно является benchmark'ом понимания языка. Некоторые его subsets — практически **закодированные формальные задачи**.

Для Ein это может проверять непосредственно parsing + rewriting/evaluation.

---

### 5. `dyck_languages`

Ещё более искусственный язык. Например:

```text
[ { ( ) } ]
```

или незавершённая последовательность скобок, которую надо правильно продолжить.

Концептуально задача проверяет вложенную структуру:

[
open([)
\rightarrow open({)
\rightarrow open(()
\rightarrow close())
\rightarrow close(})
\rightarrow close(])
]

Это уже скорее parsing/stack reasoning, чем relational puzzle.

---

### 6. `navigate`

Здесь дан текстовый «язык команд»:

> Always face forward.
> Take 2 steps forward.
> Turn right.
> Take 3 steps forward.
> Turn left.
> Take 1 step forward.
>
> What is your final position relative to the starting point?

Ответ выбирается из направлений/позиций.

Это можно представить:

[
state=(x,y,direction)
]

и каждое предложение задаёт transformation:

[
move : State \rightarrow State
]

[
turnRight : State \rightarrow State
]

То есть опять композиция:

[
S_n =
move_1 \circ turnLeft \circ move_3
\circ turnRight \circ move_2(S_0)
]

---

### 7. `formal_fallacies`

Здесь формат уже ближе к настоящему логическому тексту:

> If someone is a doctor, then they are educated.
> John is educated.
> Therefore, John is a doctor.
>
> Is the argument valid?

Нужно распознать ошибку affirming the consequent:

[
doctor(x)\Rightarrow educated(x)
]

[
educated(John)
]

из чего **не следует**

[
doctor(John).
]

Это особенно интересно для prover-like части Ein, потому что надо отличать `R → S` от возможности применять правило обратно.

---

## Что фактически лежит в dataset

BBH не даёт тебе AST вроде:

```json
{
  "facts": [...],
  "constraints": [...],
  "query": ...
}
```

Типичная запись концептуально выглядит примерно так:

```json
{
  "input": "Five people ... Which person is second ... (A)...",
  "target": "(D)"
}
```

То есть **вся задача находится внутри текста `input`**.

При этом BBH неоднороден. Есть subsets, где задача практически уже является программой (`boolean_expressions`), есть controlled natural language (`tracking_shuffled_objects`, `logical_deduction`), а есть задачи с существенно большей NL-составляющей.

Для Ein я бы особенно выделил **`logical_deduction`**: это наиболее прямой следующий шаг после Zebra. Причём у BBH есть варианты `logical_deduction_three_objects`, `..._five_objects`, `..._seven_objects`, что удобно для измерения scaling: одна и та же семантика отношений, но растёт размер задачи и search space.



Мне кажется bbh идеально подходит для моей задачи для ein: семантический анализ + построение программы на ein + поиск/вывод набора правил для решения. 


Да. Причём BBH хорошо подходит не просто как benchmark для Ein, а как **benchmark всей предполагаемой архитектуры Ein end-to-end**.

Я бы формализовал pipeline так:

[
\boxed{
\text{BBH text}
\xrightarrow{\text{semantic analysis}}
\text{Ein program}
\xrightarrow{\text{rule synthesis/selection}}
\text{Ein reasoning}
\xrightarrow{}
\text{answer}
}
]

Здесь полезно разделить два разных вида синтеза.

**1. Синтез instance/program.** Из конкретного текста получаем сущности, факты, отношения и query:

```text
Alice is left of Bob.
Bob is left of Carol.
Who is leftmost?
```

↓

```text
left-of Alice Bob
left-of Bob Carol

query leftmost ?
```

Это NL → Ein IR.

**2. Синтез/выбор theory — правил решения.** Из самой структуры задачи нужно понять, какие свойства отношений нужны. Например:

[
leftOf(a,b)\land leftOf(b,c)
\Rightarrow leftOf(a,c)
]

то есть обнаружить необходимость транзитивности `left-of`.

В другом BBH task понадобятся совсем другие rules:

```text
swap(A, B)
```

↓

правила изменения ownership/state.

Таким образом, benchmark можно разложить на три независимо измеряемых компонента:

[
\text{semantic parsing}
\quad|\quad
\text{theory construction}
\quad|\quad
\text{reasoning/search}
]

Это очень ценно экспериментально. Можно сделать **oracle ablations**:

* **NL + known theory → answer** — проверяет semantic parsing + solver.
* **Ein IR + unknown theory → answer** — проверяет rule discovery.
* **Ein IR + known theory → answer** — проверяет чистый kernel/search.
* **NL + unknown theory → answer** — полный Ein pipeline.

И тогда результат вида «Ein решил 82% BBH» становится гораздо информативнее: можно определить, **где именно теряются остальные 18%**.

Ещё интереснее организовать rules не per-instance, а **per task family**. Например, дать системе несколько training examples `logical_deduction`, позволить вывести theory (T), после чего заморозить её:

[
examples_{train}
\rightarrow T_{\text{logical-deduction}}
]

а затем:

[
NL_i \rightarrow P_i
]

[
T_{\text{logical-deduction}} + P_i
\rightarrow answer_i
]

на unseen instances.

Это уже проверяет очень сильную гипотезу Ein: **много различных текстов являются всего лишь разными инстанциациями небольшой relational theory**.

И BBH особенно удобен именно потому, что task families уже фактически дают тебе такую естественную группировку. `logical_deduction_3/5/7` вообще почти готовый controlled experiment на generalization/scaling.

Я бы поэтому начал не со всего BBH, а с **одной family — `logical_deduction` — и построил полный benchmark harness: BBH JSON → NL→Ein → inferred/selected rules → search → answer → trace → error classification.** После этого `tracking_shuffled_objects` даст принципиально другой класс — state transitions — и покажет, насколько подход действительно generic, а не специализирован под Zebra/CSP.



Пока что я не планирую training examples, и вообще training llms. 
Суть системы ein в том что formal reasoning часть (которая уже есть) очень быстро решает или проверяет решаемость уже сформулированной задачи, выдает инфу о противоречиях, скрытых в рулах зависимостях между отношениями и тд. А neural часть используется для семантического анализа текста и neural-guided выбора теори/теорий, синтеза новых правил. И это все в цикле до достижения fixed point или лимита по времени. 


Да, тогда это уже не схема «LLM решает BBH», а скорее **нейро-символьный цикл синтеза теории с формальным верификатором**.

Я бы описал Ein так:

[
\text{text}
\to
\text{semantic candidates}
\to
(P,T)
\to
\text{formal solve/check}
\to
\text{diagnostics}
\to
\text{neural revision}
\to \cdots
]

где (P) — формализованный instance задачи, а (T) — выбранная или синтезированная theory, то есть набор правил и свойств отношений.

Ключевое отличие от обычного agentic LLM-loop в том, что formal reasoning у тебя не просто «ещё один инструмент». Он даёт **жёсткий feedback signal**: satisfiable/unsatisfiable, найденные следствия, противоречия, недостающие связи, возможно redundant rules, implicit dependencies между relations, размер search space, наличие/отсутствие ответа. Neural часть после этого не обязана угадывать с нуля — она получает структурированную диагностику того, почему текущая формализация не работает.

Тогда один iteration можно мыслить примерно так:

[
(P_i,T_i)
\xrightarrow{\text{Ein}}
D_i
]

[
(text,P_i,T_i,D_i)
\xrightarrow{\text{LLM}}
(P_{i+1},T_{i+1})
]

до

[
(P_{i+1},T_{i+1})=(P_i,T_i)
]

в некотором смысле fixed point, либо пока не получен достаточный formal result.

Причём fixed point здесь, вероятно, лучше определять не как буквальное равенство текста программы, а как **семантическую стабилизацию**. Например, дальнейшие изменения не меняют closure теории:

[
Cl(T_{i+1},P_{i+1}) = Cl(T_i,P_i)
]

или не меняют множество допустимых моделей / ответ на query. Иначе LLM может бесконечно переписывать эквивалентные rules синтаксически.

Для BBH это действительно выглядит очень естественно. Например, `logical_deduction`:

1. LLM извлекает `left-of`, `immediately-right-of`, позиции и query.
2. Ein сообщает: задача underconstrained или query не выводится.
3. Neural layer замечает, что `left-of` должен быть транзитивным, а `immediately-right-of` связан с `right-of`.
4. Добавляет theory:
   [
   immediateRight(x,y)\Rightarrow right(x,y)
   ]
   [
   right(x,y)\land right(y,z)\Rightarrow right(x,z)
   ]
5. Ein снова насыщает граф и проверяет задачу.
6. Если появляются две несовместимые позиции одного объекта, это уже сигнал, что semantic parse или rule synthesis неверен.

Это довольно сильная архитектурная идея, потому что **LLM не должен быть надёжным reasoner**. Он должен быть хорошим generator/proposer в пространстве формализаций, а Ein быстро отбрасывает плохие гипотезы.

Я бы ещё различал три типа neural actions внутри цикла:

* **reinterpretation** — изменить отображение текста в relations/facts;
* **theory selection** — подключить известные properties/rule modules;
* **theory synthesis** — породить действительно новое правило или relation definition.

Последнее особенно важно отделить от второго. Если система постоянно «синтезирует» транзитивность заново, это хуже, чем распознать:

[
leftOf \in TransitiveRelations.
]

И здесь твоя идея отношений между отношениями начинает играть центральную роль: neural layer может не генерировать низкоуровневый rule, а утверждать higher-order fact вроде:

[
transitive(leftOf)
]

или

[
inverse(leftOf,rightOf)
]

а Ein уже разворачивает это в конкретные rewrite rules.

Тогда архитектура становится ещё чище:

[
\text{NL}
\to
\text{object-level facts}
+
\text{meta-level facts about relations}
]

а дальше kernel сам инстанцирует нужную theory.

Поэтому BBH полезен не только как набор задач. Он может проверять, насколько далеко можно зайти с гипотезой:

> **большая часть “reasoning” — это правильное выделение relations и выбор небольшой теории их свойств; после этого формальный вывод дешёв.**

И если Ein начнёт хорошо решать разные BBH families с одним и тем же kernel, но разными автоматически выбираемыми relation theories, это будет уже довольно содержательный эксперимент, а не просто ещё один benchmark score.



