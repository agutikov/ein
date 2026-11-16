#!/usr/bin/env python3
"""The census of questions: every id of every question ledger, and its class.

Each question the repo has asked is in one of three classes:

- **decision** — it has an answer that stands: a ruling, or a default nobody
  revised. Its final answer is the last word of its row, after following any
  *superseded by*, *re-ruled* or *absorbed* chain.
- **open** — no answer and no default. It stays with the plan that owns it.
- **not carried** — superseded, moot, reversed, withdrawn, moved to another
  id, or a copy of a question another ledger carries. Only the one that
  stands is a decision.

    python3 utils/decision_census.py                 # every id, one line each
    python3 utils/decision_census.py --counts        # per ledger and class
    python3 utils/decision_census.py --subjects      # the proposed layout
    python3 utils/decision_census.py --local-ids     # local number -> its id
    python3 utils/decision_census.py --files         # the decisions against docs/decisions/
    python3 utils/decision_census.py --cited         # which ids code and docs cite
    python3 utils/decision_census.py --slug TITLE    # a decision file's name
    python3 utils/decision_census.py --check         # exit 1 on a gap

The ledgers read are every milestone's `open_questions.md` (under `plans/` and
`docs/history/`), `plans/open_questions.md`, the M1e review's ten, the Q rows
of `plans/followups/`, P1f.6's numbered questions with their replies, and the
stage files that number questions of their own (`STAGE_LOCAL`). Where two
ledgers use one number for two questions, the later one's key is qualified,
`Q23@M2`; a local number's key is `P1f.6#5` or `S1f.5.4#Q1`. A milestone's
row moved into `plans/open_questions.md`, id kept (Q-M1e.19), is read there
and not under `docs/history/`.

**Ids given to rulings no ledger holds** (S1f.20.8's T3: the standing rules
`AGENTS.md` states, the documentation rulings, P1f.6's `design.md`) have no
ledger row: their hand row, flagged `given`, is their record, its note naming
where the ruling is written. **A local number's id** is fixed in its hand row,
a note that opens `as Q-M1f.316;`, as the user ruled the map (Q-M1e.23 to .26,
Q-M1e2.29, Q-M1f.315 to .340); a local decision about the process takes none.

**Carried** is a decision whose folder is not `process` or `gate` and which is
not `pending`: the process's decisions stay with their plans, and a decision
not built stays with the plan that builds it. `--check` holds the census to
`docs/decisions/` both ways: every carried decision has exactly one file
`<id>-<slug>.md`, in the folder its subject names, and every file there is a
carried decision's.

The rows are read from the ledgers as they stand. A row is classed by its
state's first words; where those do not decide it, or where the row names a
chain (*superseded*, *re-ruled*, *absorbed*, *overruled*, *reopened*, …), the
class is **banked by hand** in `utils/decision_census.txt`, which also banks
each decision's proposed subject under `docs/decisions/`. `--check` fails on a
row with no class, a chain with no hand row, a decision with no subject, a
subject that is not one of `SUBJECTS`, or a hand row whose id no ledger has,
so the census is re-taken by running it again after a ledger moves; and on
any break of the cross-check with `docs/decisions/` above.
"""

from __future__ import annotations

import argparse
import re
import sys
from collections import Counter, OrderedDict, defaultdict
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HAND = ROOT / "utils" / "decision_census.txt"
DECISIONS = ROOT / "docs" / "decisions"

# The proposed layout of `docs/decisions/`: one folder a subject.
SUBJECTS = OrderedDict(
    [
        ("language", "the language: grammar, names, values, keywords, rules, macros, what the loader refuses"),
        ("imports", "modules and imports: full and short names, what an import brings, where a module is found"),
        ("diagnostics", "the line a refusal prints: its place, its wording, its note"),
        ("answer", "the answer: the region R, cores, claims, the questions about R, what no lever may change"),
        ("hypotheses", "the hypothesis space: hrules, obligations, the generation ladder, the mechanisms, H"),
        ("read-out", "what `ein solve` and `ein test` print: rows, words, JSON, dumps, the trace, English text"),
        ("cli", "commands, options, exit codes, views, `.einb` as a command reads it"),
        ("engine", "`ein.rs` inside: the parser, the compiler's invariants, the `.einb` format, determinism"),
        ("stdlib", "the stdlib as a whole: its layout, and the modules outside `math` and `bool`"),
        ("stdlib/math", "the calculator's shared parts: tables, `positional`, `arith`, `convert`, the call interface"),
        ("stdlib/math/decimal", "`std.math.decimal` and `std.math.decimal-arith`: the numeral and its operations"),
        ("stdlib/math/int", "`std.math.int`: integers"),
        ("stdlib/math/rat", "`std.math.rat`: rationals"),
        ("stdlib/math/gcd", "`std.math.gcd`: gcd and lcm"),
        ("stdlib/math/binary", "`std.math.binary` and `std.math.binary-arith`"),
        ("stdlib/math/num9", "`std.math.num9`: nine digits by wide rules"),
        ("stdlib/bool", "Boolean functions and circuits: `std.bool.*`"),
        ("corpus", "the corpus, the wrong-answer register, fixtures, examples, the stdlib's test programs"),
        ("gate", "the gate and the nightly: what runs, on what budget, under which profile (not carried)"),
        ("docs", "the documentation: what a page may claim, how a claim is warranted"),
        ("process", "plans and scheduling only: phase order, ownership, who runs what (not carried)"),
    ]
)
#: The subjects whose decisions are the process's and stay with their plans.
NOT_CARRIED_SUBJECTS = ("gate", "process")

CLASSES = ("decision", "open", "not-carried")
FLAGS = ("pending", "verify", "conflict", "home", "given")
#: A hand row's note that fixes a local number's id: `as Q-M1f.316; …`.
GIVEN_ID = re.compile(r"^as (Q-M\w+?\.\d+)\s*;\s*")
DECISION_FILE = re.compile(r"^(Q-M\d+(?:[a-z]\d*)?\.\d+|Q\d+)-[a-z0-9][a-z0-9-]*\.md$")


@dataclass
class Rec:
    key: str  # the census key: the id, qualified where two ledgers share it
    ledger: str  # path, repo-relative
    family: str  # history | milestone | global | phase | followup
    title: str
    state: str  # the ledger's state text, links and bold removed
    mech: str = ""  # the class read off the state, or "" when it cannot be
    chain: bool = False  # the state names a chain: a hand row must confirm it
    cls: str = ""  # the final class
    by_hand: bool = False
    subject: str = ""
    flags: list = field(default_factory=list)
    note: str = ""
    answer: str = ""
    local: bool = False  # a local number, to be given a milestone id
    milestone: str = ""  # the milestone that asked it, for a local number
    given_id: str = ""  # a local number's fixed id, from its hand row

    @property
    def carried(self) -> bool:
        """A decision `docs/decisions/` holds: not the process's, and built."""
        return self.cls == "decision" and self.subject not in NOT_CARRIED_SUBJECTS and "pending" not in self.flags

    @property
    def qid(self) -> str:
        """The id the decision's file carries: a local number's given id, else
        the key, which for a carried decision is never qualified."""
        return self.given_id or (self.key if not self.local else "")


# --------------------------------------------------------------------------
# Reading markdown


def strip_md(s: str) -> str:
    s = re.sub(r"\]\([^)]*\)", "]", s)  # link targets
    s = re.sub(r"\[([^\]]*)\]", r"\1", s)  # link brackets
    s = s.replace("**", "").replace("~~", "")
    s = re.sub(r"<a id=\"[^\"]*\"></a>", "", s)
    return re.sub(r"\s+", " ", s).strip()


def cells(line: str) -> list[str]:
    """A table row's cells; a `|` inside backticks or escaped is no separator."""
    out, cur, tick = [], [], False
    body = line.strip()
    if body.startswith("|"):
        body = body[1:]
    if body.endswith("|"):
        body = body[:-1]
    i = 0
    while i < len(body):
        c = body[i]
        if c == "\\" and i + 1 < len(body) and body[i + 1] == "|":
            cur.append("|")
            i += 2
            continue
        if c == "`":
            tick = not tick
        if c == "|" and not tick:
            out.append("".join(cur).strip())
            cur = []
        else:
            cur.append(c)
        i += 1
    out.append("".join(cur).strip())
    return out


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


STATUS_WORDS = (
    r"ruled|re-ruled|defaulted|accepted|decided|answered|resolved|closed|confirmed|overruled|"
    r"kept|left|open|deferred|moved|superseded|absorbed|reopened|owned|done|withdrawn|"
    r"working answer|update|resolution|decision"
)


def section_status(text: str) -> str:
    """The status sentences of a `## Q…` section with no index row."""
    out = []
    for line in text.splitlines():
        m = re.match(r"^\s*\*\*(" + STATUS_WORDS + r")\b", line, re.I)
        if m:
            out.append(line.strip())
    return strip_md(" ".join(out))


def split_sections(text: str, head_re: str) -> dict:
    """{id: (title, body)} for every heading that matches `head_re`."""
    out = OrderedDict()
    lines = text.splitlines()
    starts = []
    for i, line in enumerate(lines):
        m = re.match(head_re, line)
        if m:
            starts.append((i, m.group("id"), strip_md(m.group("title")), len(line) - len(line.lstrip("#"))))
    for k, (i, qid, title, _lvl) in enumerate(starts):
        end = starts[k + 1][0] if k + 1 < len(starts) else len(lines)
        # stop at a heading of a higher level than this one
        lvl = len(lines[i]) - len(lines[i].lstrip("#"))
        for j in range(i + 1, end):
            if lines[j].startswith("#"):
                l2 = len(lines[j]) - len(lines[j].lstrip("#"))
                if l2 < lvl:
                    end = j
                    break
        if qid not in out:
            out[qid] = (title, "\n".join(lines[i + 1 : end]))
    return out


# --------------------------------------------------------------------------
# The ledgers

MILESTONE_LEDGERS = [
    ("docs/history/m1a_rust/open_questions.md", "Q-M1a", "history"),
    ("docs/history/m1c_external_validation/open_questions.md", "Q-M1c", "history"),
    ("docs/history/m1d_satisfiability/open_questions.md", "Q-M1d", "history"),
    ("docs/history/m1e_review_processing/open_questions.md", "Q-M1e", "history"),
    ("plans/m1e2_correctness/open_questions.md", "Q-M1e2", "milestone"),
    ("plans/m1f_hypothesis_and_documentation/open_questions.md", "Q-M1f", "milestone"),
    ("plans/m1g_logic/open_questions.md", "Q-M1g", "milestone"),
    ("plans/m1h_beyond_word_problems/open_questions.md", "Q-M1h", "milestone"),
    ("plans/m10_external_benchmarks/open_questions.md", "Q-M10", "milestone"),
]


def milestone_ledger(rel: str, prefix: str, family: str) -> list[Rec]:
    text = read(rel)
    pid = re.escape(prefix) + r"\.\d+"
    recs = OrderedDict()
    for line in text.splitlines():
        m = re.match(r"^\|\s*(?:~~)?\[?(?:\*\*)?(" + pid + r")\b", line)
        if not m or m.group(1) in recs:
            continue
        c = cells(line)
        recs[m.group(1)] = Rec(m.group(1), rel, family, strip_md(c[1]), strip_md(c[-1]))
    secs = split_sections(text, r"^#{2,4} *(?P<id>" + pid + r") *[—–-]+ *(?P<title>.*)$")
    for qid, (title, body) in secs.items():
        if qid not in recs:
            recs[qid] = Rec(qid, rel, family, title, section_status(body))
    return sorted(recs.values(), key=lambda r: int(r.key.rsplit(".", 1)[1]))


def global_ledger() -> list[Rec]:
    """`plans/open_questions.md`: Q1–Q42, the cross-milestone pool and M1's.

    An index row whose *Lives in* names another ledger is a pointer: the
    question is read where it lives (M2, F1, F2, F4)."""
    rel = "plans/open_questions.md"
    text = read(rel)
    pointers = {}
    for line in text.splitlines():
        m = re.match(r"^\|\s*(Q\d+)\s*\|", line)
        if m:
            c = cells(line)
            where = c[2]
            if re.search(r"\[(M2|followups F\d+)\]", where):
                pointers[m.group(1)] = strip_md(where)
    out = []
    secs = split_sections(text, r"^## *(?P<id>Q\d+) *[—–-]+ *(?P<title>.*)$")
    for qid, (title, body) in secs.items():
        if qid not in pointers:
            out.append(Rec(qid, rel, "global", title, section_status(body)))
    # A milestone's row moved here, id kept, when its ledger left with
    # `docs/history/`: read here, the copy there dropped (`all_records`).
    moved = split_sections(text, r"^## *(?P<id>Q-M\w+?\.\d+) *[—–-]+ *(?P<title>.*)$")
    for qid, (title, body) in moved.items():
        out.append(Rec(qid, rel, "global", title, section_status(body) or "open"))
    return out


def m2_ledger() -> list[Rec]:
    rel = "plans/m2_nl_to_ir/open_questions.md"
    text = read(rel)
    out = []
    for line in text.splitlines():
        m = re.match(r"^\|\s*\[(Q\d+|Q-M2\.\d+)\]", line)
        if not m:
            continue
        c = cells(line)
        qid = m.group(1)
        # Q23–Q25 here are not the global Q23–Q25 (M1's, absorbed): the same
        # number was assigned twice.
        key = qid + "@M2" if qid in ("Q23", "Q24", "Q25") else qid
        out.append(Rec(key, rel, "milestone", strip_md(c[1]), "to be decided in: " + strip_md(c[-1])))
    return out


def f4_ledger() -> list[Rec]:
    rel = "plans/followups/f4_cross_cutting.md"
    out = []
    for line in read(rel).splitlines():
        m = re.match(r"^\|\s*(Q\d+)\s*\|", line)
        if not m:
            continue
        c = cells(line)
        qid = m.group(1)
        # F4's Q30–Q33, Q39 and Q40 are not the global ones of those numbers.
        key = qid + "@F4" if qid in ("Q30", "Q31", "Q32", "Q33", "Q39", "Q40") else qid
        out.append(Rec(key, rel, "followup", strip_md(c[1]), "parked"))
    return out


def bullet_ledger(rel: str, suffix: str, family: str) -> list[Rec]:
    out = []
    for line in read(rel).splitlines():
        m = re.match(r"^- (Q\d+): (.*)$", line)
        if m:
            out.append(Rec(m.group(1) + suffix, rel, family, strip_md(m.group(2)), "promoted, parked"))
    return out


FOLLOWUP_HOMES = {
    "F1": "plans/followups/f1_categorical_formulation.md",
    "F2": "plans/followups/f2_self_modifying_language.md",
}


def followup_homes() -> list[Rec]:
    """The global pool's questions that F1 and F2 own by name, with no table
    of their own: the title is the global index's, the ledger the followup."""
    out = []
    for line in read("plans/open_questions.md").splitlines():
        m = re.match(r"^\|\s*(Q\d+)\s*\|", line)
        if not m:
            continue
        c = cells(line)
        f = re.search(r"followups (F\d+)\]", c[2])
        if f and f.group(1) in FOLLOWUP_HOMES:
            out.append(Rec(m.group(1), FOLLOWUP_HOMES[f.group(1)], "followup", strip_md(c[1]), "parked"))
    return out


def review_ledger() -> list[Rec]:
    """The M1e review's Q1–Q10: text in `review/open-questions.md`, answers
    in the M1e README's table."""
    rel = "docs/history/m1e_review_processing/review/open-questions.md"
    secs = split_sections(read(rel), r"^## *(?P<id>Q\d+) *[—–-]+ *(?P<title>.*)$")
    states = {}
    for line in read("docs/history/m1e_review_processing/README.md").splitlines():
        m = re.match(r"^\|\s*`(Q\d+)`\s*\|", line)
        if m:
            states[m.group(1)] = strip_md(cells(line)[-1])
    out = []
    for qid, (title, _body) in secs.items():
        r = Rec(qid + "@M1e-review", rel, "history", title, states.get(qid, ""))
        r.local, r.milestone = True, "M1e"
        out.append(r)
    return out


P1F6 = "plans/m1f_hypothesis_and_documentation/p1f.6_calculator/"


def p1f6_ledger() -> list[Rec]:
    """P1f.6's local numbers: `questions.md`'s 1–73 and round 2's layout item,
    each with what `replies_2026-11-12.md` says it became."""
    q = read(P1F6 + "questions.md")
    titles = OrderedDict()
    for m in re.finditer(r"^### (\d+)\. (.*)$", q, re.M):
        titles[int(m.group(1))] = strip_md(m.group(2))
    for line in q.splitlines():
        m = re.match(r'^\|\s*<a id="q(\d+)"></a>(\d+)\s*\|', line)
        if m:
            c = cells(line)
            titles[int(m.group(2))] = strip_md(c[1] + ": " + c[2])
    replies = {}
    for line in read(P1F6 + "replies_2026-11-12.md").splitlines():
        m = re.match(r"^\|\s*\[?(\d+)\]?(?:\([^)]*\))?\s*\|", line)
        if not m:
            continue
        n = int(m.group(1))
        c = cells(line)
        if len(c) >= 3 and n not in replies:
            replies[n] = strip_md(" — ".join(c[1:]))
    out = []
    for n in sorted(titles):
        r = Rec(f"P1f.6#{n}", P1F6 + "questions.md", "phase", titles[n], "reply: " + replies.get(n, ""))
        r.local, r.milestone = True, "M1f"
        out.append(r)
    r = Rec(
        "P1f.6#layout",
        P1F6 + "questions_round2.md",
        "phase",
        "the stdlib layout: std.math and what goes where",
        "reply: the proposed layout with std.bool.*; 'Confirm others'",
    )
    r.local, r.milestone = True, "M1f"
    out.append(r)
    return out


# Stage files that number their own questions. Most are copies of a question a
# ledger carries; the hand file says which.
STAGE_LOCAL = [
    # (path, key prefix, milestone, heading regex)
    (
        "plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/"
        "s1e2.3.4_the_inherited_questions_and_the_record.md",
        "S1e2.3.4",
        "M1e2",
        r"^> \*\*(?P<id>Q\d+)\. (?P<title>.*?)(?:\*\*|$)",
    ),
    (
        "plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md",
        "S1f.5.4",
        "M1f",
        r"^#### (?P<id>Q\d+) — (?P<title>.*)$",
    ),
    (
        "plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_paper_grammar.md",
        "S1f.5.5",
        "M1f",
        r"^### (?P<id>Q\d+) — (?P<title>.*)$",
    ),
    (
        "plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md",
        "S1f.5.16",
        "M1f",
        r"^## (?P<id>Q\d+) — (?P<title>.*)$",
    ),
    (
        P1F6 + "malformed_numbers.md",
        "P1f.6-malformed",
        "M1f",
        r"^\*\*(?P<id>Q\d+)\. (?P<title>.*?)\*\*",
    ),
    (
        "docs/history/m1e_review_processing/search_soundness_probes/README.md",
        "S1e.1.1",
        "M1e",
        r"^\*\*(?P<id>Q\d+) (?:— )?(?P<title>.*?)\*\*",
    ),
]


def stage_local() -> list[Rec]:
    out = []
    for rel, pre, ms, head in STAGE_LOCAL:
        seen = set()
        for line in read(rel).splitlines():
            m = re.match(head, line)
            if not m or m.group("id") in seen:
                continue
            seen.add(m.group("id"))
            fam = "history" if rel.startswith("docs/history") else "phase"
            r = Rec(f"{pre}#{m.group('id')}", rel, fam, strip_md(m.group("title")), "stage-local")
            r.local, r.milestone = True, ms
            out.append(r)
    return out


def all_records() -> list[Rec]:
    recs = []
    for rel, prefix, fam in MILESTONE_LEDGERS:
        recs += milestone_ledger(rel, prefix, fam)
    recs += global_ledger()
    recs += m2_ledger()
    recs += followup_homes()
    recs += f4_ledger()
    recs += bullet_ledger("plans/followups/f3_three_task_classes_first_class.md", "@F3", "followup")
    recs += review_ledger()
    recs += p1f6_ledger()
    recs += stage_local()
    moved = {r.key for r in recs if r.ledger == "plans/open_questions.md" and r.key.startswith("Q-M")}
    return [r for r in recs if not (r.key in moved and r.ledger.startswith("docs/history/"))]


# --------------------------------------------------------------------------
# Classifying

NOT_CARRIED = re.compile(
    r"^(moved\b|closed moot|moot\b|superseded\b|absorbed\b|withdrawn\b|merged\b|reversed\b|closed as superseded)", re.I
)
OPEN = re.compile(r"^(open\b|deferred\b|parked\b|promoted, parked)", re.I)
DECISION = re.compile(
    r"^(ruled|re-ruled|defaulted|accepted|decided|answered|resolved|closed|confirmed|overruled|kept\b|"
    r"left as|reading given|the user's instructions|\d{4}-\d\d-\d\d|stands)",
    re.I,
)
CHAIN = re.compile(
    r"supersed|absorbed|revers|overrid|overtaken|re-ruled|reopen|overruled|withdrawn|moot|"
    r"(?<!\w)open\b|deferred|outlives|moved",
    re.I,
)


def mechanical(r: Rec) -> None:
    """The class the state's first words give, and whether it names a chain.
    A local number and a parked idea are read by hand; so is M2's ledger,
    whose last column names the stage that will decide, not a state."""
    s = r.state.strip()
    if r.local or r.family == "followup":
        r.mech = ""
    elif NOT_CARRIED.match(s):
        r.mech = "not-carried"
    elif OPEN.match(s):
        r.mech = "open"
    elif DECISION.match(s):
        r.mech = "decision"
    else:
        r.mech = ""
    # a row that names a chain, past its first word, is read by hand
    rest = s[12:] if r.mech else s
    r.chain = bool(r.mech) and bool(CHAIN.search(rest))


def load_hand() -> dict:
    """`decision_census.txt`: `key | class | subject | flags | note`, one row an
    id; an empty class keeps the one read off the state."""
    hand = OrderedDict()
    for n, line in enumerate(HAND.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        parts = [p.strip() for p in line.split("|", 4)]  # the note may hold a `|`
        if len(parts) < 5:
            raise SystemExit(f"{HAND.name}:{n}: fewer than five fields")
        key, cls, subject, flags, note = parts
        if key in hand:
            raise SystemExit(f"{HAND.name}:{n}: {key} twice")
        flags = [f.strip() for f in flags.split(",") if f.strip()]
        for f in flags:
            if f not in FLAGS:
                raise SystemExit(f"{HAND.name}:{n}: flag {f!r} is not one of {FLAGS}")
        hand[key] = dict(cls=cls, subject=subject, flags=flags, note=note, answer="", line=n)
    return hand


STATUS_HEAD = re.compile(
    r"^(?:ruled|re-ruled|defaulted|accepted|decided|answered|resolved|closed|confirmed|overruled|"
    r"kept as built|left as it is(?: in round 2)?|reading given|the user's instructions,?)\b",
    re.I,
)
# What may stand between a status word and the answer: a date, who and where
# in brackets, a separator.
HEAD_NOISE = re.compile(
    r"^\s*(?:\d{4}-\d\d-\d\d(?: and -\d\d)?|by the user|the same day|the user's to revise:?|as recommended|"
    r"put to the user[^:]*:|"
    r"\((?=[^)]*(?:user|S1|T1|stage|repl|default|instruction|\d{4}|round 2|`))[^)]*\)|"
    r"(?:\((?:[a-z]|i{1,3}|iv|v|[A-E]|\d)\)){1,2}(?:,? and (?:\((?:[a-z]|i{1,3}|iv|v|\d)\)){1,2})*,(?= )|"
    r"[,:;—–.]|→)\s*"
)


def answer_of(r: Rec) -> str:
    """The standing answer as the row writes it, with its status word, its
    date and its history cut: the text under `Before:` and a default a later
    ruling replaced. A summary to find the row by, not the decision's text."""
    def clean(seg: str) -> str:
        a = STATUS_HEAD.sub("", seg, count=1)
        prev = None
        while prev != a:
            prev, a = a, HEAD_NOISE.sub("", a, count=1)
        return re.sub(r"(;\s*)?S1[a-z0-9.]+(?:'s [A-Z0-9.]+)?\s*$", "", a).strip(" ;,")

    s = re.split(r"\bBefore:", r.state)[0].strip()
    segs = re.split(r"(?<=[.;]) (?=(?:defaulted|before, defaulted)\b)", s, flags=re.I)
    first = segs[0]
    if len(segs) > 1 and re.match(
        r"^(\d{4}-\d\d-\d\d: stands|confirmed|kept as built|left as it is|accepted|answered in round 2)", first, re.I
    ):
        return clean(segs[1])
    a = clean(first)
    # a ruling that only names its option, "(a)", is spelt out by the default
    # it confirmed, which the row writes next
    if len(a) < 40 and len(segs) > 1:
        rest = re.split(r"\)\s*:", segs[1], maxsplit=1)
        a = (a + " — " + clean(rest[-1])).strip(" —")
    return a


def classify(recs: list[Rec], hand: dict) -> list[str]:
    errs = []
    keys = {r.key for r in recs}
    dup = [k for k, c in Counter(r.key for r in recs).items() if c > 1]
    for k in dup:
        errs.append(f"{k}: read from two ledgers")
    for r in recs:
        mechanical(r)
        h = hand.get(r.key)
        if h:
            r.subject, r.flags, r.note = h["subject"], h["flags"], h["note"]
            m = GIVEN_ID.match(r.note)
            if m:
                r.given_id, r.note = m.group(1), r.note[m.end():]
                if not r.local:
                    errs.append(f"{r.key}: `as {r.given_id}` names an id for an id")
            if h["cls"]:
                r.cls = h["cls"]
                r.by_hand = True
            else:
                r.cls = r.mech
            r.answer = h["answer"]
        else:
            r.cls = r.mech
        if not r.cls:
            errs.append(f"{r.key} ({r.ledger}): no class: the state does not decide it and no hand row does")
        elif r.cls not in CLASSES:
            errs.append(f"{r.key}: class {r.cls!r} is not one of {CLASSES}")
        if r.chain and not (h and h["cls"]):
            errs.append(f"{r.key}: the state names a chain ({r.state[:80]}…); a hand row must class it")
        if r.cls == "decision":
            # a local number's answer is its reply, which the hand row sums up
            if not r.answer:
                r.answer = r.note if (r.local and r.note) else answer_of(r)
                if not r.answer:
                    r.answer = r.note
            if not r.subject:
                errs.append(f"{r.key}: a decision with no subject")
            elif r.subject not in SUBJECTS:
                errs.append(f"{r.key}: subject {r.subject!r} is not one of SUBJECTS")
        elif r.subject and r.subject not in SUBJECTS:
            errs.append(f"{r.key}: subject {r.subject!r} is not one of SUBJECTS")
    for k, h in hand.items():
        if k not in keys:
            errs.append(f"{HAND.name}:{h['line']}: {k} is in no ledger")
    # An id is one question's: a local number's given id, and a given row's,
    # is no ledger's id and no other local number's.
    ledger_ids = {r.key for r in recs if r.family != "given"}
    owner = {}
    for r in recs:
        qid = r.given_id or (r.key if r.family == "given" else "")
        if not qid:
            continue
        if qid in ledger_ids:
            errs.append(f"{r.key}: its id {qid} is a ledger's id too")
        if qid in owner:
            errs.append(f"{r.key}: its id {qid} is {owner[qid]}'s too")
        owner[qid] = r.key
        if r.family == "given" and r.cls != "decision":
            errs.append(f"{r.key}: a given id is a decision's, not {r.cls!r}")
    return errs


def given_records(hand: dict) -> list[Rec]:
    """The rulings no ledger holds, each a hand row flagged `given`: the row
    is the record, its note where the ruling is written."""
    out = []
    for k, h in hand.items():
        if "given" in h["flags"]:
            r = Rec(k, HAND.relative_to(ROOT).as_posix(), "given", h["note"], "given")
            r.milestone = re.match(r"^Q-(M\w+?)\.", k).group(1) if k.startswith("Q-M") else ""
            out.append(r)
    return out


def decision_files() -> dict[str, list[Path]]:
    """Every file under `docs/decisions/` but the folders' `README.md`, by the
    id its name opens with (`""` for a name that is not `<id>-<slug>.md`)."""
    out = defaultdict(list)
    for path in sorted(DECISIONS.rglob("*.md")) if DECISIONS.is_dir() else []:
        if path.name == "README.md":
            continue
        m = DECISION_FILE.match(path.name)
        out[m.group(1) if m else ""].append(path)
    return out


def files_check(recs: list[Rec]) -> list[str]:
    """The census against `docs/decisions/`, both ways: every carried decision
    has exactly one file, in its subject's folder, and every file is a carried
    decision's."""
    errs = []
    files = decision_files()
    for path in files.pop("", []):
        errs.append(f"{path.relative_to(ROOT)}: not `<id>-<slug>.md`")
    carried = {}
    for r in recs:
        if not r.carried:
            continue
        if not r.qid:
            errs.append(f"{r.key}: a carried local number with no id (`as Q-…;` in its hand row)")
            continue
        carried[r.qid] = r
        paths = files.get(r.qid, [])
        if len(paths) != 1:
            errs.append(f"{r.key}: {len(paths)} files for {r.qid} under docs/decisions/")
            continue
        folder = paths[0].parent.relative_to(DECISIONS).as_posix()
        if folder != r.subject:
            errs.append(f"{r.key}: its file is in {folder}/, its subject is {r.subject}")
    for qid, paths in sorted(files.items()):
        if qid not in carried:
            for path in paths:
                errs.append(f"{path.relative_to(ROOT)}: {qid} is no carried decision of the census")
    return errs


# --------------------------------------------------------------------------
# Local numbers: the id the user fixed for each


def local_ids(recs: list[Rec]) -> list[tuple[str, str, str]]:
    """(local key, its id, subject) for every local-numbered decision: the id
    its hand row fixes (`as Q-…;`), as the user ruled the map, or `—` for one
    about the process, which is not carried and takes none."""
    return [
        (r.key, r.given_id or "—", r.subject + (" (pending)" if "pending" in r.flags else ""))
        for r in recs
        if r.local and r.cls == "decision"
    ]


# --------------------------------------------------------------------------
# Reports


def ledger_group(rel: str) -> str:
    if rel.endswith("open_questions.md") or rel.endswith("open-questions.md"):
        return rel
    if rel.startswith("plans/followups/"):
        return "plans/followups/ (Q rows)"
    if rel.startswith(P1F6) and Path(rel).name in ("questions.md", "questions_round2.md"):
        return P1F6 + "questions.md + round 2"
    return "stage files' local questions"


def report_counts(recs: list[Rec]) -> str:
    groups = OrderedDict()
    for r in recs:
        groups.setdefault(ledger_group(r.ledger), Counter())[r.cls] += 1
    lines = ["| ledger | decision | open | not carried | all |", "|---|---:|---:|---:|---:|"]
    tot = Counter()
    for g, c in groups.items():
        tot.update(c)
        lines.append(f"| `{g}` | {c['decision']} | {c['open']} | {c['not-carried']} | {sum(c.values())} |")
    lines.append(f"| **all** | **{tot['decision']}** | **{tot['open']}** | **{tot['not-carried']}** | **{sum(tot.values())}** |")
    fam = OrderedDict()
    for r in recs:
        fam.setdefault(r.family, Counter())[r.cls] += 1
    lines += ["", "| family | decision | open | not carried | all |", "|---|---:|---:|---:|---:|"]
    for f, c in fam.items():
        lines.append(f"| {f} | {c['decision']} | {c['open']} | {c['not-carried']} | {sum(c.values())} |")
    hand = sum(1 for r in recs if r.by_hand)
    mech = sum(1 for r in recs if not r.by_hand)
    lines += ["", f"classed by hand: {hand}; read off the state: {mech}"]
    return "\n".join(lines)


def report_subjects(recs: list[Rec]) -> str:
    dec = [r for r in recs if r.cls == "decision"]
    c = Counter(r.subject for r in dec)
    pend = Counter(r.subject for r in dec if "pending" in r.flags)
    lines = ["| folder | decisions | of which pending | what |", "|---|---:|---:|---|"]
    for s, what in SUBJECTS.items():
        lines.append(f"| `{s}/` | {c[s]} | {pend[s]} | {what} |")
    lines.append(f"| **all** | **{len(dec)}** | **{sum(pend.values())}** | |")
    return "\n".join(lines)


def report_rows(recs: list[Rec]) -> str:
    """One line an id, tab-separated: key, class, subject and flags, ledger,
    the answer (a decision) or the state (the rest), and the hand row's note."""
    out = []
    for r in recs:
        tail = r.answer if r.cls == "decision" else r.state
        flags = (" [" + ",".join(r.flags) + "]") if r.flags else ""
        hand = " (hand)" if r.by_hand else ""
        note = "" if (r.local and r.cls == "decision") else r.note
        out.append(f"{r.key}\t{r.cls}{hand}\t{r.subject}{flags}\t{r.ledger}\t{tail}\t{note}")
    return "\n".join(out)


SLUG_MAX = 48
SLUG_TAIL = {"and", "or", "a", "an", "the", "of", "to", "in", "by", "its"}


def slug(title: str) -> str:
    """The proposed file-name slug of a decision's title: lower case, an
    apostrophe dropped, every other run outside `[a-z0-9]` one `-`, cut at the
    last whole word within `SLUG_MAX` characters, a trailing connective
    dropped. The file is `<id>-<slug>.md`, so the id alone keeps it unique."""
    s = title.lower().replace("'", "").replace("’", "")
    words = [w for w in re.split(r"[^a-z0-9]+", s) if w]
    out = []
    for w in words:
        if len("-".join(out + [w])) > SLUG_MAX:
            break
        out.append(w)
    while len(out) > 1 and out[-1] in SLUG_TAIL:
        out.pop()
    return "-".join(out)


CITED_SKIP = ("plans", "docs/history", "build", "ein.rs/target", "ein.rs/vendor", ".git", ".claude", "node_modules")
CITED_EXT = (".md", ".rs", ".ein", ".toml", ".py", ".sh", ".yml", ".txt", ".l", ".y")


def cited(recs: list[Rec]) -> dict:
    """{id: [files]} for every `Q-M…` id cited outside `plans/` and
    `docs/history/` (a bare `Qn` is not looked for: too many share a number)."""
    ids = {r.key for r in recs if r.key.startswith("Q-M")}
    pat = re.compile(r"Q-M[0-9a-z]+\.\d+")
    out = defaultdict(list)
    for path in sorted(ROOT.rglob("*")):
        rel = path.relative_to(ROOT).as_posix()
        if not path.is_file() or rel.startswith(CITED_SKIP) or not rel.endswith(CITED_EXT):
            continue
        if rel in ("utils/decision_census.txt", "utils/decision_census.py"):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for m in sorted(set(pat.findall(text))):
            if m in ids:
                out[m].append(rel)
    return out


def report_cited(recs: list[Rec]) -> str:
    c = cited(recs)
    by = {r.key: r for r in recs}
    lines = ["| class | ids cited outside `plans/` and `docs/history/` | citing files |", "|---|---:|---:|"]
    for cls in CLASSES:
        ks = [k for k in c if by[k].cls == cls]
        files = {f for k in ks for f in c[k]}
        lines.append(f"| {cls} | {len(ks)} | {len(files)} |")
    lines.append("")
    for k in sorted(c, key=lambda k: (by[k].cls, k)):
        if by[k].cls != "decision":
            lines.append(f"- `{k}` ({by[k].cls}): " + ", ".join(f"`{f}`" for f in c[k][:4])
                         + (f" and {len(c[k]) - 4} more" if len(c[k]) > 4 else ""))
    return "\n".join(lines)


def report_files(recs: list[Rec], ferrs: list[str]) -> str:
    """The decisions by where they are: carried (a file each, by folder), and
    the rest of the class with why each stays out."""
    dec = [r for r in recs if r.cls == "decision"]
    carried = [r for r in dec if r.carried]
    lines = ["| folder | carried decisions |", "|---|---:|"]
    c = Counter(r.subject for r in carried)
    for s in SUBJECTS:
        if c[s]:
            lines.append(f"| `{s}/` | {c[s]} |")
    lines.append(f"| **all** | **{len(carried)}** |")
    out = Counter()
    for r in dec:
        if not r.carried:
            out["pending: decided, not built" if "pending" in r.flags else f"{r.subject}: the process's"] += 1
    lines += ["", "| decisions not carried | count |", "|---|---:|"]
    lines += [f"| {k} | {v} |" for k, v in sorted(out.items())]
    given = [r for r in recs if r.family == "given"]
    local = [r for r in recs if r.given_id]
    lines += ["", f"ids given to rulings no ledger holds: {len(given)}; local numbers with an id: {len(local)}"]
    lines += ["", f"mismatches with docs/decisions/: {len(ferrs)}"] + [f"- {e}" for e in ferrs]
    return "\n".join(lines)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--cited", action="store_true", help="which ids are cited outside plans/ and docs/history/")
    ap.add_argument("--slug", metavar="TITLE", help="print the file-name slug of a decision's title, and exit")
    ap.add_argument("--counts", action="store_true", help="counts per ledger, family and class")
    ap.add_argument("--subjects", action="store_true", help="decisions per proposed folder")
    ap.add_argument("--local-ids", action="store_true", help="the id of each local-numbered decision")
    ap.add_argument("--files", action="store_true", help="the carried decisions against docs/decisions/")
    ap.add_argument("--class", dest="only", choices=CLASSES, help="list only this class")
    ap.add_argument("--subject", help="list only the decisions of this folder")
    ap.add_argument("--unclassed", action="store_true", help="list the rows a hand row must class, with their state")
    ap.add_argument("--check", action="store_true", help="exit 1 on any gap in the banked data")
    a = ap.parse_args()
    if a.slug is not None:
        print(slug(a.slug))
        return 0
    recs = all_records()
    hand = load_hand()
    recs += given_records(hand)
    errs = classify(recs, hand)
    ferrs = files_check(recs) if not errs else []
    if a.unclassed:
        for r in recs:
            if not r.cls or (r.chain and not (hand.get(r.key) or {}).get("cls")):
                print(f"{r.key}\t{r.mech or '?'}\t{r.ledger}\t{r.title}\t{r.state}")
        return 0
    if a.check:
        for e in errs + ferrs:
            print(e, file=sys.stderr)
        n = sum(1 for r in recs if r.carried)
        print(f"{len(recs)} ids; {len(errs)} gaps", file=sys.stderr)
        if not errs:
            print(f"{n} carried decisions, {len(decision_files())} files under docs/decisions/; "
                  f"{len(ferrs)} mismatches", file=sys.stderr)
        return 1 if errs or ferrs else 0
    if errs:
        print(f"note: {len(errs)} gaps; run --check", file=sys.stderr)
    if a.counts:
        print(report_counts(recs))
    elif a.subjects:
        print(report_subjects(recs))
    elif a.cited:
        print(report_cited(recs))
    elif a.files:
        print(report_files(recs, ferrs))
    elif a.local_ids:
        for old, new, subject in local_ids(recs):
            print(f"{old}\t{new}\t{subject}")
    else:
        sel = [r for r in recs if (not a.only or r.cls == a.only) and (not a.subject or r.subject == a.subject)]
        print(report_rows(sel))
    return 0


if __name__ == "__main__":
    sys.exit(main())
