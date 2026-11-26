# Q-M2.26 — Does `zebra3.1.ein` quote, in each membership of a thing a sentence names, that sentence?

**Status:** open

**Raised 2026-11-26 by [S2.2.1](../../s2.2.1_contract.md)**, working the Zebra puzzle through the three steps by hand
to the target shape (its T2.2.1.8; [the contract's § 7.2](../../../../../docs/einaf/04-contract.md#72-the-zebra-puzzle-step-by-step)).

**The question.** The contract makes a membership source-derived where it quotes the sentence that names the thing,
and otherwise a generated assumption, an event ([Q-M2.22](Q-M2.22-is-a-membership-no-sentence-states-a-faithfulness-event.md)'s
default). [`examples/zebra3.1.ein`](../../../../../examples/zebra3.1.ein), the target shape, was written before the
contract: of its thirty-two memberships, the twenty-five of values a sentence names (*Red*, named by *The Englishman
lives in the red house.*; *Water*, named by *Who drinks water?*) quote no sentence, and under the contract they are
twenty-five events beside the eleven no sentence can carry (seven kinds, four facts of the row): thirty-six. The
steps by hand write each of the twenty-five with the first sentence that names the thing, so their step 3,
`einaf/contract/zebra/step3.ein`, is `zebra3.1.ein` form for form less its `(config …)` and with twenty-five
`:source`s more, and has eleven events. Should `zebra3.1.ein` quote them too?

**The default taken**: no. `zebra3.1.ein` is Ein's example and stays as S2.2.0a wrote it; the contract's reading is
`einaf/contract/zebra/step3.ein`, the program a prompt quotes as the target shape, and
`the_zebra_steps_end_at_the_target_shape` holds the two to the same forms but the `:source`s and the `(config …)`,
and to the same answer under every question.

**What the default costs.** Two programs of the target shape, which a reader has to tell apart; the corpus's copy
counts thirty-six events where the contract's has eleven.

**The alternatives.**

1. **No** (the default).
2. **Yes**: `examples/zebra3.1.ein` gains the twenty-five `:source`s, an edit of an Ein example whose shape digests
   are re-banked (`corpus_shapes.md5`); the test then compares the two less the `(config …)` alone.
3. **Moot**: Q-M2.22 rules a membership no event (its alternative 2), and neither program carries one.
