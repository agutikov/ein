#!/usr/bin/env python3
"""The reference: each request of the record sent once to `llama-server` of the
host's package, on the same file and the same context, greedy, with the same
seed and grammar, and set against what the binding returned.

    python3 reference.py --server http://127.0.0.1:18731 --template TEMPLATE \
        --einaf einaf/target/release/einaf --record RECORD.jsonl REQUEST.json... > OUT.jsonl

`RECORD.jsonl` is `einaf generate`'s output for the same requests, a line a
request in the same order. For each request: the prompt as EinAf renders it
(`einaf render`) against the server's `/apply-template`; the server's tokens
of it (`/tokenize`, special tokens parsed) against the binding's, by the
envelope's digest and count; and the server's greedy completion of those
tokens (`/completion`, `return_tokens`) against the binding's generated
tokens, with the first index where they part. Standard library only.
"""

import argparse
import hashlib
import json
import subprocess
import sys
import urllib.request


def post(server, path, body):
    req = urllib.request.Request(
        server + path,
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=3600) as r:
        return json.loads(r.read())


def digest(tokens):
    return "sha256:" + hashlib.sha256("".join(f"{t}\n" for t in tokens).encode()).hexdigest()


def tools_json(tools):
    out = []
    for t in tools:
        out.append({
            "type": "function",
            "function": {
                "name": t["name"],
                "description": t["description"],
                "parameters": {
                    "type": "object",
                    "properties": {p["name"]: {"type": "string", "description": p["description"]}
                                   for p in t["parameters"]},
                    "required": [p["name"] for p in t["parameters"]],
                },
            },
        })
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--server", required=True)
    ap.add_argument("--template", required=True)
    ap.add_argument("--einaf", required=True)
    ap.add_argument("--record", required=True)
    ap.add_argument("requests", nargs="+")
    a = ap.parse_args()
    records = [json.loads(l) for l in open(a.record) if l.strip()]
    assert len(records) == len(a.requests), "a record line a request"
    for path, rec in zip(a.requests, records):
        req = json.load(open(path))
        row = {"request": path}
        prompt = req["prompt"]
        if prompt["kind"] == "chat":
            ours = subprocess.run([a.einaf, "render", "--template", a.template, path],
                                  capture_output=True, text=True, check=True).stdout
            body = {"messages": prompt["messages"],
                    "chat_template_kwargs": {"enable_thinking": req["thinking"]}}
            if prompt.get("tools"):
                body["tools"] = tools_json(prompt["tools"])
            theirs = post(a.server, "/apply-template", body)["prompt"]
            row["template_equal"] = ours == theirs
            if ours != theirs:
                i = next((i for i, (x, y) in enumerate(zip(ours, theirs)) if x != y), min(len(ours), len(theirs)))
                row["template_differs_at"] = i
                row["template_ours"] = ours[max(0, i - 40):i + 80]
                row["template_theirs"] = theirs[max(0, i - 40):i + 80]
            text = theirs
        else:
            text = prompt["text"]
        tokens = post(a.server, "/tokenize", {"content": text, "add_special": False,
                                              "parse_special": True})["tokens"]
        env = (rec.get("generation") or {}).get("envelope")
        row["prompt_tokens_server"] = len(tokens)
        if env:
            row["prompt_tokens_binding"] = env["prompt"]["tokens"]
            row["prompt_tokens_equal"] = digest(tokens) == env["prompt"]["tokens_sha256"]
        body = {"prompt": tokens, "n_predict": req["max_tokens"], "temperature": 0, "seed": req["seed"],
                "cache_prompt": False, "return_tokens": True}
        if req.get("logprobs"):
            body["n_probs"] = req["logprobs"]
        g = req.get("grammar")
        if g:
            body["grammar"] = g["text"]
            if g["start"]["at"] == "patterns":
                body["grammar_lazy"] = True
                body["grammar_triggers"] = [{"type": 3, "value": p} for p in g["start"]["patterns"]]
        try:
            out = post(a.server, "/completion", body)
        except Exception as e:  # noqa: BLE001 - the record says what the server refused
            row["server_error"] = str(e)
            print(json.dumps(row))
            continue
        theirs_tokens = [t for t in out.get("tokens", []) if t not in (248044, 248046)]
        row["server_text"] = out.get("content")
        row["server_tokens"] = len(theirs_tokens)
        row["server_stop"] = out.get("stop_type")
        row["server_timings"] = out.get("timings")
        if env:
            ours_tokens = env["generation"]["tokens"]
            row["binding_tokens"] = len(ours_tokens)
            row["generation_equal"] = ours_tokens == theirs_tokens
            if ours_tokens != theirs_tokens:
                row["parts_at"] = next((i for i, (x, y) in enumerate(zip(ours_tokens, theirs_tokens)) if x != y),
                                       min(len(ours_tokens), len(theirs_tokens)))
        print(json.dumps(row))
        sys.stdout.flush()


if __name__ == "__main__":
    main()
