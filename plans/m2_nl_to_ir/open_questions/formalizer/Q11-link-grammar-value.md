# Q11 — Link-grammar value

**Status:** ruled by the user, 2026-11-27

**Ruled by the user, 2026-11-27:** *"Q11 - link-grammar will not be used, any strictly pre-defined grammars will
not be used."* Said with the instruction that made [P2.2f](../../p2.2f_text_analysis/README.md): *"add new phase for
investigation of this approach, especially AMR; everything that fits 8GB VRAM is ok, for now speed is not main
question for EinAF, main question is feasibility"*.

**Read as** (the coordinator's reading, which the user may correct):

- **No grammar of English written in advance feeds the formalizer**: not link grammar's dictionaries, not NLTK's
  grammars, not a hand-written one. The link-grammar arm of
  [S2.6.4](../../p2.6_ablations/s2.6.4_representation_ablations.md) is dropped with its runner, its serialiser and
  its pre-registered rule, and nothing of it is built.
- **The input side is asked again of trained networks**: an analysis of the text by a parser that learned it (AMR
  first) is [P2.2f](../../p2.2f_text_analysis/README.md)'s question, of feasibility; what it finds feasible is
  S2.6.4's input-side arm in place of link grammar.
- **The grammars of Ein's output stay**: the GBNF EinAf writes from `ein.l` and `ein.y`, which constrains what the
  model writes, is a grammar of Ein and not of English, and is not touched by this ruling
  ([Q-M2.6](Q-M2.6-one-gbnf-or-many.md) is its question).
- **`nl2smt/` stays as it is**: outside Ein and EinAf, wired into nothing; it is no input of EinAf's.
  [P2.8](../../p2.8_representations/README.md) names it as a second route from the sentences to SMT with no model,
  a comparison, which its own planning reads against this ruling.

The question as it stood, a measured arm with link grammar off unless it won, is in
`git show 4087c457:plans/m2_nl_to_ir/open_questions/formalizer/Q11-link-grammar-value.md`.
