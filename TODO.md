; Scratchpad for raw thoughts. When an item is embedded into plans/ or
; docs/, prune it from here. The trail lives in commit history.

;; Processed 2026-05-18:
;;   - rule demo problems            → plans/m1_core_graph_reasoning/p1.3_inference_rules/s1.3.2_ten_core_rules.md (T1.3.2.12)
;;   - puzzles & benchmarks catalog  → docs/ideas/09-puzzles-beyond-zebra.md, docs/index/12-llm-and-reasoning-benchmarks.md
;;   - types/relations raw           → plans/followups/f4_cross_cutting.md (Q34 expanded, Q37+Q38 added),
;;                                     plans/m1_core_graph_reasoning/open_questions.md (Q22..Q25),
;;                                     plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/s1.2.1_data_model.md,
;;                                     plans/m1_core_graph_reasoning/README.md (rules-of-thumb)
;;   plans/raw/ was a working scratchpad — gone after the embedding above.


# some generalization thoughts
continue improving ein graph model and language


core: graph + rules
graph: nodes + links
graph: objects + relations

instance of a type is an object
instance of a relation is a fact
relation is a type
fact is an object
relation is an object
type is an object
instance is a relation and instance is a fact
instance is an instance fact of instance relation
instance is an instance of instance



relation declaration and definition
Compare:
    - rules as relation implementation/body
        (relation is-a (A B) (transitive, asymetric, exclusive))
    - apply rules to relation - produce rule-relation facts
        (relation is-a (A B))
        (transitive is-a)
        (asymetric is-a)
        (exclusive is-a)
Semantic does not change, but then what the difference? What form is better and why?





(Name)  - free node
(Name1 Name2) - already a relation  (Name1) <-1- (Name1 Name2) -2-> (Name2)
We instantiate relations as nodes
(relation) -> (relation is-a (T T)) -> (is-a A B)
Reserved node names:
rule - rule definition
relation - relation declaration (or definition?)
Facts are relations instances


(rel1 objA objB) - 3-hrel

(T) <-1- (T T) -2-> (T) - two links from (T T) to the same (T)

IMPORTANT note: no copies, nodes are unique, they either
    - named obejcts with a single unique name
    - or relations with ordered list of links


So, finally

Unique name - global unique object, except ?variables and :keys
?Variables - local unique object
:Keys - alternative for positional semantic for: reordering, optional:
    (rule rule-name rule-args rule-match rule-assert rule-why rule-priority)
    (rule rule-name rule-args :match rule-match :assert rule-assert :priority rule-priority)

parantheses () - graph node
name usage - link/pointer/reference to (name) node
() - node without name
should we use it as placeholder or global singletone False or _|_ ?
or opposite - use it as top node? or just forget or postpone it?

So (A B C) - is:
    - a node
    - an object

What are types?
Types are nodes that hold relations common for multiple instance nodes.
So instance of a type, an object, inherit relations of a type.
For example:
    - humans can drink drinks - relation connecting types: humans, drinks and can-drink
        - (can-drink Human Drink)
    - coffee is a drink - instantiation of drink type
        - (is-a Coffee Drink)
    - Jack is a human - instantiation of human type
        - (is-a Jack Human)
    - Jack can drink drinks - inherited relation
        - (can-drink Jack Drink)
    - Jack can drink coffee - instantiation of inherited relation
        - (can-drink Jack Coffee)
    - Jack drinks coffee - application of some other rule
        - (drink Jack Coffee)
btw Make a markdown file with Jack drinking coffee example with rules and all statements in a language and diagram forms.
Why relations are types?
Because there are instances - facts:
    (co-located Color Pet) ;; not explicitely stated in zebra, inferred from (relation co-located T T)
    ;; how to decide if arguments has to be in () or just tail
    ;; it's simple - if there is nothing except args there - use tail (head arg1 arg2)
    ;; if there is something else, some body - then pack args into single node
    ;; (co-located)    <-\
    ;;            (co-located Color Pet)
    ;; (Color) <----------------/     \--------> (Pet)
    ;; relation declaration "enables co-location of color and pet"
    (symmetric co-located) (transitive co-located) ;; assign rules - implement semantic
    ;; this creates relations between co-located relation and rules
    ;; (symmetric) <- (symmetric co-located) -> (co-located)
    ;; if we create instance of co-located relation - a fact
    ;; we have to use also objects instances of types from relation declaration: Color and Pet
    (co-located Green Cat)
    ;; this is how is-instance and is-subtype differ
    ;; is-subtype inherit relations
    ;; is-instance instantiate types



We have to setup clear definitions:
- node
- arrow (link/arc/pointer/reference)
- object - just a node that is pointed by arrow
    - indeed objects usually not point to any other nodes, other nodes point to them
- relation
    - node contains 2+ links
    - relation fact
    - relation type

How we would live with many different meanings of `relation`?
- any multi-name node (A B C)
- relation as type for relation facts or relation instances
- relation instance



---

# Ontology

Ontology is a context for facts
Need to differentiate facts head and facts as set of fact objects, which may be placed in the ontology also
In NL problem statement ontology is implicit
For IR ontology has to be deduced "by common sense" from facts. 
For example Norvegian as noun could be a Human, a Nationality, a Language. "Norvegian lives in yellow house" languages and nationalities do not live in houses, but human do. 
So Norvegian here is a Human.


---

# Type systems induced by relations

Type system is based on:
- inheritance rule
- substitution rule
Relation is-a produce a type hierarchy or classification.

There could be different hyerarchy or another structure produces by another relation.
It wouldn't be so popular as `type`, but could be:
    - not less interesting
    - more important for specific problem
    - maybe convertible to type hierarchy







---

# glossary

add glossary to docs/ for some complex terms exmplanation and references
- homoiconic
- reflexive
- ...

---


# Self-modification ideas

F2: self modifiable language via GBNF modification
F5: operate IR rules as data, as graph nodes - objects and relations
F6: read, understand and modify own harness code

docs/ideas/10-generic-self-modification.md - refer to 3 followups


