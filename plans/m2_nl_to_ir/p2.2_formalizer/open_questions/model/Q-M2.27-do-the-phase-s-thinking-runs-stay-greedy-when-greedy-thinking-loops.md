# Q-M2.27 — Do the phase's runs with thinking on stay greedy, when greedy thinking loops?

**Status:** open

**Raised 2026-11-26 by [S2.2.2](../../s2.2.2_model_runtime.md)**, from its first experiment and its probe
([the probe's report](../../model_runtime/README.md)).

**The question.** The phase's own runs are greedy (the phase's default 7), so that one run is the whole
distribution. With thinking on, greedy decoding of `qwen3.5:4b` falls into a repetition loop: the first experiment's
thinking settled its answer (copy the example) by its 2 500th token, then repeated one cycle of 163 tokens from its
6 154th to the budget of 55 000, and wrote no program; the model alone on the Zebra text solved the puzzle in its
thinking by about its 5 900th token, then looped from its 6 459th to the budget of 65 000 and wrote no answer; and
**every one of the probe's 34 other runs with thinking on** (the library and theory-given conditions, step 1) ended
the same way, a cycle of 11 to 911 tokens begun between its 249th and its 8 901st, repeated to the budget of 16 384
(one, a block whose heading it numbers): no program in 36 runs. The model's card asks for sampling with thinking on,
and warns against greedy decoding for this reason. Does the phase keep greedy
decoding for its thinking-on runs?

**The default taken**: yes. Greedy stays the phase's, as its default 7 says, and **the record names every loop** (the
token it began at, its period) and counts the runs the budget stopped; no generation is cut short by EinAf. The
arms' chain stays [S2.5.2](../../../p2.5_harness/s2.5.2_baselines.md)'s, whose default is the model's own, and the one
shot runs under it unchanged; a thinking arm is not compared with a non-thinking one under greedy decoding.

**What the default costs.** A thinking-on run is a run the budget stops: what it measures is where the model loops,
not what it writes, and a thinking arm under greedy decoding writes nothing. Each loop costs its budget, five minutes
for 16 384 tokens.

**The alternatives.**

1. **Greedy, the loop recorded** (the default).
2. **The model's own chain for every thinking-on run** (presence penalty 1.5, top-k 20, top-p 0.95, temperature 1),
   seeded, several seeds a run: the distribution, at the price of several runs where one stood.
3. **Greedy with a stop on a loop**: a cycle of the same tokens repeated a fixed number of times ends the generation
   with a stop of its own, `loop`, which changes no token before it, so a run is greedy up to its loop and costs no
   more than its loop's start.
4. **Greedy with a repetition or presence penalty** in the chain, which changes the tokens before the loop too.

**What bears on it.** [S2.5.2](../../../p2.5_harness/s2.5.2_baselines.md), which owns the arms' chain; P2.6's
thinking ablation; [Q-M2.14](../../../open_questions/model/Q-M2.14-what-runs-the-model.md)'s control of the sampler
chain, which the binding gives (any of the four is a request's `sampler` field).
