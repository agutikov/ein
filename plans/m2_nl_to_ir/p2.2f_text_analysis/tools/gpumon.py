#!/usr/bin/env python3
"""Run a command and record the GPU memory it takes: `python3 gpumon.py OUT.json -- COMMAND …`.

Reads `nvidia-smi` once before the command (the card's state, every process on it), then samples the card's used
memory and the compute processes' memory every 100 ms while the command runs. OUT.json holds the state before, the
peak of the card's used memory, the baseline (the card's used memory before), the peak less the baseline, the peak
of the command's own processes, the wall time and the exit code. Standard library only (runs under 3.8 on).
"""

import json
import os
import subprocess
import sys
import threading
import time


def smi(*args):
    try:
        return subprocess.run(["nvidia-smi", *args], capture_output=True, text=True, timeout=20).stdout
    except Exception as e:  # noqa: BLE001
        return f"nvidia-smi failed: {e}"


def card_used():
    out = smi("--query-gpu=memory.used,memory.total", "--format=csv,noheader,nounits")
    try:
        used, total = (int(x) for x in out.strip().splitlines()[0].split(","))
        return used, total
    except Exception:  # noqa: BLE001
        return None, None


def apps():
    out = smi("--query-compute-apps=pid,used_memory,process_name", "--format=csv,noheader,nounits")
    rows = []
    for line in out.strip().splitlines():
        parts = [p.strip() for p in line.split(",")]
        if len(parts) >= 2 and parts[0].isdigit():
            rows.append({"pid": int(parts[0]), "mib": int(parts[1]) if parts[1].isdigit() else None,
                         "name": parts[2] if len(parts) > 2 else ""})
    return rows


def descendants(pid):
    """The pids of pid's process tree, read from /proc."""
    children = {}
    for d in os.listdir("/proc"):
        if not d.isdigit():
            continue
        try:
            with open(f"/proc/{d}/stat") as f:
                ppid = int(f.read().rsplit(")", 1)[1].split()[1])
            children.setdefault(ppid, []).append(int(d))
        except Exception:  # noqa: BLE001
            pass
    out, todo = {pid}, [pid]
    while todo:
        for c in children.get(todo.pop(), []):
            if c not in out:
                out.add(c)
                todo.append(c)
    return out


def main():
    if "--" not in sys.argv:
        sys.exit(__doc__)
    i = sys.argv.index("--")
    out_path, cmd = sys.argv[1], sys.argv[i + 1:]
    before_text = smi()
    base, total = card_used()
    peak = {"card": base or 0, "own": 0}
    stop = threading.Event()
    proc = subprocess.Popen(cmd)

    def sample():
        while not stop.is_set():
            used, _ = card_used()
            if used is not None:
                peak["card"] = max(peak["card"], used)
            tree = descendants(proc.pid)
            own = sum(a["mib"] or 0 for a in apps() if a["pid"] in tree)
            peak["own"] = max(peak["own"], own)
            stop.wait(0.1)

    t = threading.Thread(target=sample, daemon=True)
    t0 = time.time()
    t.start()
    code = proc.wait()
    stop.set()
    t.join()
    rec = {
        "command": cmd,
        "exit": code,
        "wall_s": round(time.time() - t0, 1),
        "card_total_mib": total,
        "card_used_before_mib": base,
        "card_peak_mib": peak["card"],
        "card_peak_less_before_mib": (peak["card"] - base) if base is not None else None,
        "own_processes_peak_mib": peak["own"],
        "nvidia_smi_before": before_text,
    }
    os.makedirs(os.path.dirname(os.path.abspath(out_path)), exist_ok=True)
    with open(out_path, "w") as f:
        json.dump(rec, f, indent=1)
    print(f"gpumon: exit {code}, {rec['wall_s']} s, card peak {peak['card']} MiB "
          f"(before {base}), own processes' peak {peak['own']} MiB -> {out_path}", file=sys.stderr)
    sys.exit(code)


if __name__ == "__main__":
    main()
