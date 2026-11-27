#!/usr/bin/env python3
"""One sentence through one of amrlib's parsers, printed as PENMAN: `python parse_amr.py [--xfm|--spring] SENTENCE …`
(default --spring), in the main venv with tools/env.sh sourced. Beam 4, as run_amrlib.py; the same compatibility
setting for SPRING's generation config. Prints each sentence's graph and, on stderr, torch's peak of GPU memory.
"""

import os
import sys

MODELS = {"--xfm": "parse_xfm_bart_large-v0_1_0", "--spring": "parse_spring-v0_1_0"}


def main():
    flags = [a for a in sys.argv[1:] if a in MODELS]
    sents = [a for a in sys.argv[1:] if a not in MODELS]
    if not sents:
        sys.exit(__doc__)
    import amrlib
    import torch

    model = MODELS[flags[-1] if flags else "--spring"]
    device = "cuda:0" if torch.cuda.is_available() else "cpu"
    stog = amrlib.load_stog_model(model_dir=os.path.join(os.environ["NLP"], "models", "amrlib", model),
                                  device=device, batch_size=1, num_beams=4)
    gen = getattr(getattr(stog, "model", None), "generation_config", None)
    if gen is not None:
        gen.return_dict_in_generate = False
        gen.output_attentions = False
    for g in stog.parse_sents(sents, add_metadata=True):
        print(g)
        print()
    if device != "cpu":
        print(f"{model}: torch peak reserved {torch.cuda.max_memory_reserved() / 2**20:.0f} MiB", file=sys.stderr)


if __name__ == "__main__":
    main()
