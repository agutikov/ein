# The grammars' sizes, before the narrowing and after it

| grammar | before: bytes, rules | after: bytes, rules |
|---|---|---|
| the file `einaf/grammars/ein.gbnf`, both roots | 50,131, 531 | 87,649, 910 |
| what a request for a whole program carries (`render whole`, no frame) | 50,131, 531 (the file) | 38,289, 383 (`root` and its rules) |
| the rules `root` reaches, their bytes but comments | 38,719, 454 | 34,249, 383 |
| `root`'s name tree (`tok-name…`, then `m-tok-name…`) | 175 rules, 68 of them after `<`, the writeback names' | 107 rules |
| `root`'s variable (`tok-var…`, then `m-tok-var…`) | 3 rules | 3 rules |
| the rules `program` reaches, their bytes but comments | 42,793, 528 | 42,793, 528 |

**The Zebra puzzle's grammars as a request carries them**, in the fenced frame (S2.2.3's speed requests before; rendered as its `plan.py` renders them after):

| grammar | before: bytes, rules | after: bytes, rules |
|---|---|---|
| whole | 50,183, 532 | 38,341, 384 |
| step1 | 14,980, 151 | 15,024, 151 |
| step2 | 55,609, 173 | 55,653, 173 |
| step2b | 19,401, 132 | 19,413, 132 |
| step3 | 23,833, 137 | 23,845, 137 |
