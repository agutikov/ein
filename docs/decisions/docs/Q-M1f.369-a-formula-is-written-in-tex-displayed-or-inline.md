# Q-M1f.369 — A formula is written in TeX, displayed or inline

A formula on a page of `docs/ein/` is TeX. A displayed formula stands between two `$$` lines, an inline one
between two `$`, and a name of more than one letter is set with `\operatorname`:

$$
\operatorname{discharged}(S) \iff \operatorname{owes}(S) = 0
$$

No formula is written in a fenced block, of `text` or of any other language, and none is spelt in Unicode operators
in place of TeX: `\iff`, `\models` and `\exists`, not `⟺`, `⊨` and `∃`. A symbol the engine prints, the read-out's
`∅`, `⊤` and `→`, is quoted as printed where a page speaks of the output, in inline code or in the block that shows
the run.

**A displayed formula states one thing a row.** A block that states several equations or definitions, or whose one
statement is longer than a line of the page, is written one statement a source line and one a rendered row, inside
`\begin{aligned} … \end{aligned}`, aligned on the main relation symbol, with a side condition in a column on the
right; a single long statement breaks at its main connective. Statements that are independent, each with a sentence
of its own, are separate blocks. No block chains statements with `\qquad`, and an inline formula that grows into a
chain of statements becomes a displayed one.

**Why.** A formula in a `text` block is a picture of a formula: it is not typeset, its operators are whatever a font
has, and it cannot be told from program output, which is what a `text` block holds. TeX is what the renderers of
these pages typeset and what a reader of mathematics expects, and one notation for every formula lets a definition
on one page be compared with its use on another. A row that holds several statements runs past the page's width
where it is rendered and hides where one statement ends; one statement a row is read, and compared, a statement at a
time.

**Held by.** The pages: the *In mathematics* sections of `docs/ein/user/03-syntax-and-semantics/` and the
definitions of `docs/ein/reasoning/`, each a `$$` block.
