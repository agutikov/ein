#!/usr/bin/env python3
"""llama_steps.py — test 3 on the step grammars: each step's grammar, rendered
for each text of the contract's steps by hand, and the part that step wrote by
hand, read by llama.cpp's grammar sampler with the model's vocabulary and by
EinAf's acceptor (`einaf grammar llama`); a JSON line a part into OUT.

    python3 llama_steps.py MODEL OUT [--einaf einaf/target/release/einaf]

Step 2's grammar is rendered on the catalogue and step 1's program, steps 2b and
3's on the program of steps 1 and 2, step 1's on the text's sentences, as the
test renders them. A step's part is the forms of its `stepN.ein` that the step before's program
does not hold, compared with their white space folded, in order, one a line;
step 3's with a marker, `;; missing: ` and the rule's name, before each rule;
step 2b's the rules of step 3's part, each after its marker. As `einaf/tests/grammar.rs` reads them.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
TEXTS = ["youngest", "zebra", "seating", "apples", "together", "more"]


def spans(text: str) -> list[str]:
    out, depth, start, i = [], 0, 0, 0
    while i < len(text):
        c = text[i]
        if c == ";":
            while i < len(text) and text[i] != "\n":
                i += 1
            continue
        if text.startswith("#|", i):
            j = text.index("|#", i + 2)
            i = j + 2
            continue
        if c == '"':
            i += 1
            while text[i] != '"':
                i += 2 if text[i] == "\\" else 1
        elif c == "(":
            if depth == 0:
                start = i
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                out.append(text[start:i + 1])
        i += 1
    return out


def fold(form: str) -> str:
    return re.sub(r"\s+", " ", form).replace("( ", "(").replace(" )", ")")


def part(d: Path, step: int, markers: bool) -> str:
    before = set() if step == 1 else {fold(f) for f in spans((d / f"step{step - 1}.ein").read_text())}
    out = []
    for f in spans((d / f"step{step}.ein").read_text()):
        if fold(f) in before:
            continue
        if markers and f.startswith("(rule"):
            out.append(";; missing: " + f.split()[1])
        out.append(f)
    return "\n".join(out)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("model")
    ap.add_argument("out")
    ap.add_argument("--einaf", default=str(ROOT / "einaf/target/release/einaf"))
    a = ap.parse_args()
    lines = []
    with tempfile.TemporaryDirectory() as tmp:
        for t in TEXTS:
            d = ROOT / "einaf/contract" / t
            for step in ["1", "2", "2b", "3"]:
                if step == "1":
                    text = part(d, 1, False)
                elif step == "2":
                    text = part(d, 2, False)
                elif step == "3":
                    text = part(d, 3, True)
                else:
                    text = "\n".join(";; missing: " + f.split()[1] + "\n" + f
                                      for f in spans(part(d, 3, False)) if f.startswith("(rule"))
                if not text:
                    continue
                args = [a.einaf, "grammar", "render", step, "--text", str(d / "text.txt")]
                if step == "2":
                    args += ["--program", str(d / "step1.ein")]
                if step in ("2b", "3"):
                    args += ["--program", str(d / "step2.ein")]
                g = subprocess.run(args, check=True, capture_output=True, text=True).stdout
                gp, pp = Path(tmp) / f"{t}.{step}.gbnf", Path(tmp) / f"{t}.{step}.ein"
                gp.write_text(g)
                pp.write_text(text)
                r = subprocess.run([a.einaf, "grammar", "llama", "--model", a.model, "--grammar", str(gp),
                                    "--root", "root", str(pp)], capture_output=True, text=True)
                row = json.loads(r.stdout.strip().splitlines()[-1])
                row["file"] = f"{t}, step {step}"
                row["grammar_sha256"] = __import__("hashlib").sha256(g.encode()).hexdigest()
                lines.append(row)
                print(row["file"], "agree" if row["agree"] else "DIFFER", row["llama"].get("tokens"), "tokens")
    Path(a.out).write_text("".join(json.dumps(l) + "\n" for l in lines))


if __name__ == "__main__":
    main()
