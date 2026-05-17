





completely rewrite reasoning.py into implementation

- fine-grained code decomposition - many small files preferred, folders
- implement all TODOs from code
- implement:
    - docs/ideas/02-graph-as-formal-substrate.md
    - docs/ideas/05-zebra-puzzle-graph-reasoner.md
    - docs/ideas/06-inference-rules-completeness.md
    - docs/ideas/08-human-style-deductive-trace.md


write plans into plans/
planning items hierarhy:
    Milestones -> Phases -> Stages -> Tasks
        Tasks are complete small implementation of some feature, investigation, experiment


3 Milestones + Followups:
- core graph reasoning module
    - the one described in PoC
    - takes simple Problem graph description
        - in IR Lisp-style language
    - builds internal representation
    - does reasoning - solve a problem
    - can output human-readable reasoning steps in the same Lisp-style IR
    - can output dot diagrams of
        - rules, constaraints
        - state
        - state transition
        - search tree
    - can output markdown log/trace for reasoning seaquence with
        - dot graph states and transitions
        - search tree with path
    - major components:
        - IR model and language
        - reasoning engine
            - with dynamic, maybe editable
                - rules
                - constraints (part of problem definition)
                - inference alogythm
            - first hardcode everythin that not yet in conditions.txt
                - make a list
                - later decide what is dynamic and what is hardcoded
        - rendering
        - what else??
- NL part
    - convert human-readable text into IR, How?
    - Shoul use link-grammar or go already to LLM?
    - Want to utilize local LLMs via llama.cpp
        - look into /home/user/work/acva for reference approach, for llama.cpp (docker, client, etc...)
    - Does it make sense to use GBNF? Looks promising
        - Maybe docs/ideas/01-self-modifying-constraint-language.md
    - Maybe make sense to feed into LLM the output of link-grammar? With all ambiguities. To enrich input? Does it make sense?
- SMT solver utilization
    - docs/ideas/04-nlp-to-graph-to-solver-pipeline.md
    - conversion from IR into SMT - how?
    - How to select theory?
    - How to select types mapping?
- Followups
    - docs/ideas/07-categorical-formulation.md
    - docs/ideas/03-three-task-classes.md
    - ????


