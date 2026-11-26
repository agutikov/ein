# The probes of the analysis

What [the analysis](../README.md) ran, kept so that every number of it can be taken again. Each script names the
binaries by `$EIN_BIN` (default `ein.rs/target/release/ein`) and `$EINAF_BIN` (default
`einaf/target/release/einaf`), and runs from the repository's root whatever directory it is started in.

| file | what it is |
|---|---|
| [`forms.py`](forms.py) | an Ein file's top-level forms, each classified by the step that writes it (statement, query, declaration, membership, structure, import, selector, rule), and the file written again in synthesis or theory-first order |
| [`measure.py`](measure.py) | the measurements: `kernel` (every encoding under every question, through `einaf feedback` and `ein solve --json-summary`), `damage` (the seven damages written and run), `order` (each encoding in both orders, compared through `einaf feedback` and `ein solve --json-summary`), `order-texts` (their read-outs, traces and event streams by `diff`), `text` (each statement against its sentence), `times` (wall clock, under `utils/bench_env.sh`) |
| [`render.py`](render.py) | the renderings of [`../renderings/`](../renderings/) and their sizes |
| [`results/`](results/) | what the measurements wrote: `kernel.json`, `damage.json`, `order.json`, `order-compare.json` and `order-texts.json`, `text.json`, `times.json` |
| [`damaged/`](damaged/) | the damaged variants, `<encoding>.<damage>.ein`, each headed by what was changed |
| [`order/`](order/) | the six encodings in synthesis order and in theory-first order, and each order's contradictory twin |
| [`order-cases/`](order-cases/) | six small programs, each with one form written before another it may need, beside the same forms in the usual order; `run.sh` compares their read-outs |
| [`language/`](language/) | a relation declared at two counts, with and without an import ([E-104](../../../evaluation/language/E-104-a-relation-declared-at-a-second-count-is-refused-where-the.md)) |
| [`zebra3.2-spatial-as-same.ein`](zebra3.2-spatial-as-same.ein), [`zebra3.2-read-through-means.ein`](zebra3.2-read-through-means.ein) | two forms of `zebra3.2` the static check flags ([E-106](../../../evaluation/answer/E-106-the-static-check-reads-a-relation-not-a-shape.md)) |
| [`zebra3.2-descriptions-in-same-house.ein`](zebra3.2-descriptions-in-same-house.ein) | the monotone form of `zebra3.2` before its descriptions were read inside `same`: every description an object of `same-house`, 110 510 facts to describe R where the example takes 60 418 (`einaf feedback FILE`, `work`) |
| [`slots-without-the-same-house-exemption.diff`](slots-without-the-same-house-exemption.diff) | `std.slots` with the guard of its two pruning rules taken out, run through `EIN_STDLIB` on a copy ([E-105](../../../evaluation/stdlib/E-105-std-slots-lets-two-values-stated-next-to-each-other-share-a.md)) |

```sh
cd plans/m2_nl_to_ir/p2.2_formalizer/zebra_formulations/probes
python3 measure.py kernel && python3 measure.py damage && python3 measure.py order
python3 measure.py order-texts && python3 measure.py text
../../../../../utils/bench_env.sh --cores PT python3 measure.py times
python3 render.py
order-cases/run.sh
```
