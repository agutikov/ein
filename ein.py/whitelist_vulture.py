# Vulture whitelist — names vulture reports as dead but that ARE used in
# ways its static name-analysis cannot see. Listed in [tool.vulture] `paths`
# (and excluded from ruff) so `vulture` runs clean. Regenerate after code
# changes with:
#
#     vulture --make-whitelist src tests \
#         --min-confidence 60 --ignore-names 'test_*,Test*' \
#         --ignore-decorators '@pytest.fixture,@fixture' > whitelist_vulture.py
#
# This file is data for vulture, not runnable code (`_` is an undefined
# sentinel) — hence the ruff exclude. Two categories:
#
# 1. Field-iteration / dataclass-eq / documentation reads vulture misses:
#    - forced_positives  — _BaseStats counter read via `for f in fields(stats)`
#                          (_serialise.py, _lattice_dump.py, _helpers.py)
#    - root_state_hash   — LatticeSnapshotV1 field; participates in dataclass
#                          `==` (shuffle-invariance tests)
#    - col               — Loc source-location field set by the parser
#    - arity/role/site   — Primitive metadata table (inline kernel docs)
#    - weird_flag        — synthetic test dataclass field read via fields()
_.forced_positives  # unused attribute (src/ein/inference/monotonic/_helpers.py:169)
forced_positives  # unused variable (src/ein/inference/monotonic/lattice.py:81)
root_state_hash  # unused variable (src/ein/inference/monotonic/snapshot.py:113)
arity  # unused variable (src/ein/inference/primitives.py:47)
role  # unused variable (src/ein/inference/primitives.py:48)
site  # unused variable (src/ein/inference/primitives.py:49)
col  # unused variable (src/ein/ir/types.py:23)
weird_flag  # unused variable (tests/inference/test_config.py:93)
# 2. Lark Transformer methods — dispatched by name on grammar terminals/rules
#    (ir/ast.py); never called directly from Python.
_.SYMBOL  # unused method (src/ein/ir/ast.py:46)
_.VAR  # unused method (src/ein/ir/ast.py:49)
_.KEYWORD  # unused method (src/ein/ir/ast.py:52)
_.WILDCARD  # unused method (src/ein/ir/ast.py:55)
_.INT  # unused method (src/ein/ir/ast.py:64)
_.query_form  # unused method (src/ein/ir/ast.py:109)
_.trace_form  # unused method (src/ein/ir/ast.py:112)
_.config_form  # unused method (src/ein/ir/ast.py:115)
_.not_form  # unused method (src/ein/ir/ast.py:153)
_.neq_form  # unused method (src/ein/ir/ast.py:156)
_.and_form  # unused method (src/ein/ir/ast.py:159)
_.or_form  # unused method (src/ein/ir/ast.py:162)
_.rule_params  # unused method (src/ein/ir/ast.py:176)
_.macro_params  # unused method (src/ein/ir/ast.py:187)
_.step_decl  # unused method (src/ein/ir/ast.py:191)
_.branch_open  # unused method (src/ein/ir/ast.py:194)
_.branch_close  # unused method (src/ein/ir/ast.py:197)
_.branch_ref  # unused method (src/ein/ir/ast.py:200)
_.contradiction_decl  # unused method (src/ein/ir/ast.py:203)
_.symmetry_decl  # unused method (src/ein/ir/ast.py:206)
_.start  # unused method (src/ein/ir/ast.py:210)
_.rule_decl  # unused method (src/ein/ir/ast.py:166)
_.macro_decl  # unused method (src/ein/ir/ast.py:183)
_.kw_pair  # unused method (src/ein/ir/ast.py:91)
_.generic_fact  # unused method (src/ein/ir/ast.py:144)
_.EQ  # unused method (src/ein/ir/ast.py:58)
_.RANGE  # unused method (src/ein/ir/ast.py:67)
_.relation_decl  # unused method (src/ein/ir/ast.py:131)
_.eq_fact  # unused method (src/ein/ir/ast.py:138)
_.hrule_decl  # unused method (src/ein/ir/ast.py:170)
_.import_form  # unused method (src/ein/ir/ast.py:118)
_.generic_list  # unused method (src/ein/ir/ast.py:97)
_.STRING  # unused method (src/ein/ir/ast.py:73)
