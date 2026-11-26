#!/usr/bin/env python3
"""plan.py — S2.2.3's runs with the model in the fenced frame, written before
the first of them.

    python3 plan.py [--einaf einaf/target/release/einaf]

Writes, beside this script:

- `fenced/plan.jsonl.gz`: the recorded generations under a grammar, a line a
  run in the probe's plan's form (`einaf::probe::Planned`), so that
  `einaf probe run` makes each call and judges its answer through the kernel
  as the probe did. Each is a run of S2.2.2's probe plan
  (`../model_runtime/plan.jsonl`), its prompt, sampler (greedy) and seed
  unchanged, with a grammar added in the fenced frame (the prompt asks for one
  fenced block) and a new id: `step1-fenced.*`, step 1's grammar rendered on
  the text's sentences; `whole-fenced.*`, the whole grammar's root `root`;
  `step1-fenced.on.youngest`, the probe's thinking-on step 1 of `youngest`
  with step 1's grammar after the thinking; and `demo-fenced.on.youngest`, a
  short prompt whose thinking ends, step 1's grammar after it. A grammar after
  the thinking begins at the model's token `</think>`.
- `speed/requests.jsonl.gz`: the speed's requests, one a grammar and a check,
  each the probe's `step1.off.zebra` or `step1.off.apples` request with that
  grammar in the fenced frame, 256 tokens at most, a line `{"name",
  "request"}` each; `speed/order.txt`, the names in the order they run, each
  three times.

Both are gzip with no time stamp, so that the same plan is the same bytes.
Every grammar is written by `einaf grammar render … --fence --request`, held
to by the chain's choice first. The first plan, `first/plan.jsonl.gz`, the
grammars from the first token with no fence, was written by this script's
first version (`git log -- first/plan.jsonl.gz`).
"""

from __future__ import annotations

import argparse
import gzip
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
PROBE = HERE.parent / "model_runtime" / "plan.jsonl"
CONTRACT = "einaf/contract"
ITEMS = ["zebra", "youngest", "seating", "apples", "together", "more"]
REPEATS = 3
SPEED_TOKENS = 256


def probe() -> dict[str, dict]:
    out = {}
    for line in PROBE.read_text().splitlines():
        if line.strip():
            e = json.loads(line)
            out[e["id"]] = e
    return out


def grammar(einaf: str, which: str, *, text: str | None = None, program: str | None = None,
            after: bool = False, check: str = "chosen-first") -> dict:
    args = [einaf, "grammar", "render", which, "--request", "--fence"]
    if text:
        args += ["--text", text]
    if program:
        args += ["--program", program]
    if after:
        args.append("--after-thinking")
    g = json.loads(subprocess.run(args, check=True, capture_output=True, text=True, cwd=ROOT).stdout)
    if check == "every-token":
        g.pop("check", None)
    else:
        g["check"] = check
    return g


def with_grammar(entry: dict, new_id: str, g: dict | None) -> dict:
    e = json.loads(json.dumps(entry))
    e["id"] = new_id
    e["request"]["grammar"] = g
    return e


def write_gz(path: Path, text: str) -> None:
    with open(path, "wb") as raw, gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as f:
        f.write(text.encode())


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--einaf", default=str(ROOT / "einaf/target/release/einaf"))
    a = ap.parse_args()
    p = probe()
    runs = []
    # Step 1 under its grammar, thinking off, in the fence.
    for item in ITEMS:
        g = grammar(a.einaf, "1", text=f"{CONTRACT}/{item}/text.txt")
        runs.append(with_grammar(p[f"step1.off.{item}"], f"step1-fenced.off.{item}", g))
    # A whole program over the library, under the whole grammar's root; and
    # the first experiment under it.
    for pid in sorted(k for k in p if k.startswith("library.off.")):
        runs.append(with_grammar(p[pid], "whole-fenced.off." + pid.split(".", 2)[2],
                                 grammar(a.einaf, "whole")))
    runs.append(with_grammar(p["first-experiment.off.zebra"], "whole-fenced.off.first-experiment",
                             grammar(a.einaf, "whole")))
    # Thinking on, the grammar begun after the thinking at the token
    # `</think>`: the probe's step 1 of `youngest`, whose thinking loops
    # before it ends (the same tokens under any grammar begun after it), so
    # that what a grammar waiting for its token costs is seen; and a short
    # prompt whose thinking ends.
    runs.append(with_grammar(p["step1.on.youngest"], "step1-fenced.on.youngest",
                             grammar(a.einaf, "1", text=f"{CONTRACT}/youngest/text.txt", after=True)))
    demo = with_grammar(p["step1.on.youngest"], "demo-fenced.on.youngest",
                        grammar(a.einaf, "1", text=f"{CONTRACT}/youngest/text.txt", after=True))
    paragraph = (ROOT / "einaf/prompts/ein-paragraph.v1.md").read_text()
    demo["request"]["prompt"]["messages"] = [
        {"role": "system", "content": paragraph},
        {"role": "user", "content": (
            "Text: Ann is older than Bob. Bob is older than Cy. Who is the youngest?\n\n"
            "Write step 1 of its formalization in Ein: each statement as a fact with its sentence as "
            "`:source`, the question as a query with a `:goal` and a `:goal-text`, then the relations "
            "declared, each with a `:why`. Reply with the program alone, in one fenced block.")},
    ]
    demo["request"]["max_tokens"] = 4096
    demo["examples"] = []
    runs.append(demo)
    (HERE / "fenced").mkdir(exist_ok=True)
    write_gz(HERE / "fenced" / "plan.jsonl.gz",
             "".join(json.dumps(r, ensure_ascii=False) + "\n" for r in runs))
    # The speed's requests.
    lines = []
    order = []
    for item in ["zebra", "apples"]:
        base = p[f"step1.off.{item}"]["request"]
        grammars = [
            ("none", None),
            ("whole", ("whole", {})),
            ("step1", ("1", {"text": f"{CONTRACT}/{item}/text.txt"})),
            ("step2", ("2", {})),
            ("step2b", ("2b", {"program": f"{CONTRACT}/{item}/step2.ein"})),
            ("step3", ("3", {"program": f"{CONTRACT}/{item}/step2.ein"})),
        ]
        for name, spec in grammars:
            checks = ["none"] if spec is None else ["every-token", "chosen-first"]
            for check in checks:
                req = json.loads(json.dumps(base))
                req["max_tokens"] = SPEED_TOKENS
                req["grammar"] = None if spec is None else grammar(a.einaf, spec[0], check=check, **spec[1])
                stem = f"{item}.{name}" + ("" if spec is None else f".{check}")
                lines.append(json.dumps({"name": stem, "request": req}, ensure_ascii=False) + "\n")
                order += [stem] * REPEATS
    (HERE / "speed").mkdir(exist_ok=True)
    write_gz(HERE / "speed" / "requests.jsonl.gz", "".join(lines))
    (HERE / "speed" / "order.txt").write_text("\n".join(order) + "\n")
    print(f"plan.py: {len(runs)} runs in fenced/plan.jsonl.gz, {len(order)} calls in speed/order.txt")


if __name__ == "__main__":
    main()
