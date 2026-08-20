**Autoformalization (автоформализация)** — это автоматический перевод неформального описания задачи, утверждения или рассуждения в **формальный язык с точно определённой семантикой**, так чтобы результат можно было проверить машиной.

Исторически термин почти всегда относился к математике:

$$
\text{natural-language mathematics}
\longrightarrow
\text{Lean / Isabelle / Coq / HOL / Metamath}
$$

Но сейчас понятие расширяется до NL → logic/specification/knowledge representation вообще. В свежей работе AAAI 2026 прямо предлагается рассматривать все эти направления в единой рамке autoformalization. ([AAAI Publications][1])

### Простейший пример

Неформально:

> Every human is mortal. Socrates is a human. Therefore Socrates is mortal.

Формализация, условно:

```text
∀x. Human(x) → Mortal(x)
Human(Socrates)
⊢ Mortal(Socrates)
```

В Lean это уже будет что-то вроде типов/предикатов и theorem. В твоей терминологии это может стать графом фактов и правил:

```text
Socrates is Human
Human is Mortal
```

плюс выбранная семантика `is`, например транзитивность:

$$
x;is;y \land y;is;z \Rightarrow x;is;z.
$$

И вот **выбор того, что именно означает `is`, какие сущности существуют и какие правила нужны**, — как раз одна из самых трудных частей autoformalization.

## Важно разделить три задачи

Autoformalization часто смешивают с theorem proving, хотя это разные стадии:

$$
\boxed{\text{NL problem}}
\xrightarrow{\text{autoformalization}}
\boxed{\text{formal problem}}
\xrightarrow{\text{ATP}}
\boxed{\text{formal proof}}
\xrightarrow{\text{checker}}
\boxed{\text{verified}}
$$

Например:

> Find all integers (x) such that (x^2=4).

Autoformalizer должен получить не просто syntactically valid Lean, а **семантически эквивалентное утверждение** вроде

```lean
theorem problem (x : ℤ) (h : x^2 = 4) :
    x = 2 ∨ x = -2 := by
    ...
```

Дальше уже начинается **automated theorem proving** — заполнение `by ...`.

Это различие принципиально: Lean может подтвердить, что формула доказана, но не может сам по себе подтвердить, что формула **правильно передаёт исходный английский текст**. Это называют semantic correctness / faithfulness problem.

## Почему LLM сильно изменили область

До LLM NL → formal language был очень тяжёлой semantic parsing задачей. LLM неожиданно оказались достаточно хорошими в сопоставлении математического языка с формальным кодом.

Одна из ключевых ранних работ — Wu et al., *Autoformalization with Large Language Models* (2022). Они получили полностью правильную формализацию 25.3% соревновательных математических задач в Isabelle/HOL и показали, что сгенерированные формализации можно затем использовать для улучшения theorem prover. ([arXiv][2])

С тех пор типичная архитектура стала выглядеть скорее как цикл:

```text
natural language
      ↓
     LLM
      ↓
candidate formalization
      ↓
parser / type checker / compiler
      ↓
 errors / constraints
      ↓
     LLM
      ↓
revised formalization
      ↓
 theorem prover
      ↓
 proof / counterexample / failure
      ↺
```

То есть это уже не просто translation model.

Например, Process-Driven Autoformalization использует детальный feedback Lean 4 как supervision signal. ([arXiv][3]) А более новые pipelines используют несколько кандидатов, compiler feedback, backtranslation и дополнительные проверки семантического соответствия. Lean Workbook, например, комбинирует Lean compilation, backtranslation, NLI и human diagnostics. ([Proceedings.com][4])

## Где здесь действительно сложная проблема

Синтаксис — относительно лёгкая часть.

Пусть имеется:

> Alice sits immediately to the left of Bob.

Можно легко получить:

$$
Left(Alice,Bob).
$$

Но если задача типа Zebra puzzle, нужно понять скрытую **теорию мира**:

* есть конечный набор houses;
* дома линейно упорядочены;
* `immediately left` означает соседство + порядок;
* каждый человек находится ровно в одном доме;
* возможно, каждый дом содержит ровно одного человека;
* цвета/животные/напитки образуют bijections с домами.

Большая часть этого может вообще **не быть явно написана**.

Поэтому более реалистично:

$$
NL
\rightarrow
\underbrace{\text{ontology}}*{\text{objects/relations}}
+
\underbrace{\text{theory}}*{\text{axioms/rules}}
+
\underbrace{\text{instance}}*{\text{facts}}
+
\underbrace{\text{goal}}*{\text{query}}
$$

Именно здесь autoformalization начинает очень сильно пересекаться с тем, что ты обсуждаешь применительно к Ein.

## В контексте Ein

Для Ein я бы вообще рассматривал autoformalization не как

$$
NL\rightarrow Ein\ code
$$

а как более содержательный pipeline:

$$
NL
\rightarrow
\text{semantic model}
\rightarrow
\text{theory selection/specialization}
\rightarrow
\text{Ein theory + instance}
\rightarrow
\text{saturation/search}
\rightarrow
\text{feedback}
\rightarrow \cdots
$$

Например N-Queens:

> Place 8 queens on a chessboard so that no two attack each other.

LLM может извлечь поверхностные сущности:

```text
Queen
Square
on(Queen, Square)
attacks(Square, Square)
```

Но дальше появляется **theory synthesis / theory retrieval**:

```text
exactly one square per queen
exactly one queen per selected square

same_row(a,b)    -> attacks(a,b)
same_column(a,b) -> attacks(a,b)
same_diagonal(a,b) -> attacks(a,b)

on(q1,a) ∧ on(q2,b) ∧ attacks(a,b)
    -> contradiction
```

А если заменить queens на knights, большая часть теории остаётся:

$$
T_{\text{finite placement}}
+
T_{\text{chessboard}}
+
T_{\text{queen attack}}
$$

превращается в

$$
T_{\text{finite placement}}
+
T_{\text{chessboard}}
+
T_{\text{knight attack}}.
$$

То есть autoformalization оказывается одновременно задачей **semantic parsing + ontology alignment + theory selection + theory specialization + constraint synthesis**.

Это уже гораздо интереснее простого NL→Lean.

### И ещё один важный слой

Можно разделить процесс на уровни:

$$
\begin{aligned}
\text{NL}
&\rightarrow \text{concepts}\
&\rightarrow \text{relations}\
&\rightarrow \text{candidate theories}\
&\rightarrow \text{specialized theory}\
&\rightarrow \text{formal instance}\
&\rightarrow \text{deductive closure/search}.
\end{aligned}
$$

Тогда symbolic engine даёт LLM очень богатый feedback:

```text
syntax error
type mismatch
unknown relation

theory inconsistent
constraint violated
goal underconstrained
multiple models remain

relation R never used
rule r cannot fire
required property absent
candidate model is counterexample
...
```

Это заметно сильнее обычного compiler feedback Lean: reasoning engine становится частью **итеративного semantic compiler**.

## Где сейчас frontier

Свежий survey 2025 определяет autoformalization как преобразование informal mathematical propositions в verifiable formal representations и рассматривает уже весь end-to-end pipeline, datasets, model design и verification. ([arXiv][5]) А работа AAAI 2026 идёт ещё дальше и прямо говорит, что NL→formal representations для reasoning, planning и knowledge representation по существу является той же задачей, хотя исторически эти сообщества использовали разные термины. ([AAAI Publications][1])

Это особенно существенно для Ein: **тебе не обязательно ограничивать термин autoformalization математикой или Lean**. NL → Ein theory вполне естественно называть autoformalization, если Ein имеет строгую формальную семантику.

Для ориентира полезен также [miniF2F](https://github.com/facebookresearch/minif2f?utm_source=chatgpt.com) — benchmark формализованных математических задач для Lean/Isabelle/Metamath/HOL Light. Он хорошо показывает классическую постановку задачи: один informal statement ↔ formal statement. ([GitHub][6])

И я бы отдельно выделил одну концептуальную границу для Ein:

$$
\boxed{
\text{autoformalization}
\neq
\text{translation}
}
$$

В сильной постановке это скорее

$$
\boxed{
\text{NL}
\rightarrow
\text{discover/select/transform theory}
\rightarrow
\text{instantiate theory}
\rightarrow
\text{machine-checkable problem}
}
$$

И это практически один-в-один связывает autoformalization с нашим предыдущим обсуждением **контекстуальной специализации теорий**: формализатору недостаточно распознать факты — ему нужно определить, *какая теория применима к данному контексту и какое её преобразование даёт теорию конкретной задачи*.

[1]: https://ojs.aaai.org/index.php/AAAI/article/view/42132?utm_source=chatgpt.com "Towards a Common Framework for Autoformalization | Proceedings of the AAAI Conference on Artificial Intelligence"
[2]: https://arxiv.org/abs/2205.12615?utm_source=chatgpt.com "Autoformalization with Large Language Models"
[3]: https://arxiv.org/abs/2406.01940?utm_source=chatgpt.com "Process-Driven Autoformalization in Lean 4"
[4]: https://www.proceedings.com/content/079/079017-3357open.pdf?utm_source=chatgpt.com "Lean Workbook: A large-scale Lean problem set formalized from natural language math problems"
[5]: https://arxiv.org/abs/2505.23486?utm_source=chatgpt.com "Autoformalization in the Era of Large Language Models: A Survey"
[6]: https://github.com/facebookresearch/minif2f?utm_source=chatgpt.com "GitHub - facebookresearch/miniF2F: An updated version of miniF2F with lots of fixes and informal statements / solutions. · GitHub"
