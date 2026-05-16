# Diagramming & Visualization Libraries

Practical engines for interactive diagrams, node editors, graph
visualisations, and whiteboard-style canvases. Five fairly different
classes — pick by *type of interaction*, not by "another graph lib".

---

## 1. General-purpose graph / node visualisation (Web / JS)

### Cytoscape.js
Strong graph-theory engine; force / dagre / cola layouts; CSS-like
styling; scales well to large graphs.
- <https://js.cytoscape.org/>

### React Flow
Currently the "strongest overall" for node-editor-style web UIs:
custom nodes, nested groups, arbitrary handles/ports, custom edges,
drag/drop, zoom/pan, background layers, node resize. Conversation-3
called it "almost a diagram operating system".
- <https://reactflow.dev/>

### JointJS
Enterprise-grade diagramming library; mature interaction model,
UML/BPMN flavour, strong orthogonal edge routing.
- <https://www.jointjs.com/>

### AntV X6
Ant Group's graph editor framework; very flexible — custom node
rendering, ports, nesting, snaplines, stencil palettes.
- <https://x6.antv.antgroup.com/>

### vis-network
Quick-start network visualisation library; lower customisation ceiling.
- <https://visjs.org/>

### Sigma.js
Faster than Cytoscape on huge graphs (WebGL).
- <https://www.sigmajs.org/>

### Apache ECharts
General-purpose visualisation library; surprisingly strong graph mode.
- <https://echarts.apache.org/>

---

## 2. Node editors / visual programming (Web / JS)

### Rete.js
Visual-programming framework; sockets, connections, execution model.
- <https://retejs.org/>

### Drawflow
Lightweight; quick MVP of a flow editor.
- <https://github.com/jerosoler/Drawflow>

### LiteGraph.js
Fast node-graph editor library, good for local tools.
- <https://github.com/jagenjo/litegraph.js>

---

## 3. Native / C++ diagram editors

### ImNodes
Node-editor extension for Dear ImGui; minimal, real-time.
- <https://github.com/Nelarius/imnodes>

### imgui-node-editor
More powerful Dear ImGui node editor; production-grade.
- <https://github.com/thedmd/imgui-node-editor>

### Dear ImGui
Immediate-mode GUI library underlying both.
- <https://github.com/ocornut/imgui>

### Graphviz
Excellent layout engine (dot/neato/twopi/circo/fdp/sfdp); weak as an
interactive editor.
- <https://graphviz.org/>

### OGDF — Open Graph Drawing Framework
Academic-grade C++ graph layout library; handles huge graphs.
- <https://ogdf.uos.de/>

---

## 4. Technical / CAD-style diagram editors

### GoJS
Commercial high-end diagramming library; enterprise diagram editors.
- <https://gojs.net/>

### mxGraph
Battle-tested JavaScript diagramming library; substrate for the
draw.io / diagrams.net ecosystem (now in maintenance).
- <https://github.com/jgraph/mxgraph>

### draw.io / diagrams.net
End-user diagram editor; reference application for the mxGraph stack.
- <https://www.drawio.com/>

### Excalidraw
Whiteboard / hand-drawn aesthetic.
- <https://excalidraw.com/>

---

## 5. Low-level canvas / drawing libraries

For "absolute freedom of drawing" — useful when standard node-graph
abstractions are too narrow (e.g. custom annotations, free-draw layers,
diagram-builder primitives).

### Konva
HTML5 canvas framework: shapes, transforms, layers, hit detection.
- <https://konvajs.org/>

### Fabric.js
Higher-level canvas library; whiteboard / object-editing style
(closer to a "Figma-lite").
- <https://fabricjs.com/>

---

## 6. Layout engines

### ELK — Eclipse Layout Kernel
High-quality automatic layout (especially for technical diagrams);
commonly paired with React Flow.
- <https://www.eclipse.org/elk/>

### Dagre
Directed-graph hierarchical layout; widely used inside Cytoscape /
React Flow / others.
- <https://github.com/dagrejs/dagre>

### Cola
Constraint-based layout library.
- <https://ialab.it.monash.edu/webcola/>

---

## 7. Workflow / pipeline applications (reference points)

Cited as "what to learn from" architecturally:

### n8n
Workflow automation; node-based editor.
- <https://n8n.io/>

### Node-RED
Flow-based programming environment.
- <https://nodered.org/>

### LangFlow
LLM workflow builder.
- <https://www.langflow.org/>

### Unreal Engine Blueprint
Visual scripting in Unreal — frequently cited as state-of-the-art node
editor UX.

### Grafana
Dashboarding and visualisation; not a graph editor but a reference for
configurable visual interaction.
- <https://grafana.com/>

---

## 8. Architectural axis: DOM/SVG vs Canvas/WebGL

| substrate | pros | cons |
|---|---|---|
| DOM / SVG | easy customisation, accessible | slow on huge graphs |
| Canvas / WebGL | high performance | harder interaction model |

Cytoscape and Sigma sit on the canvas/WebGL side; React Flow sits on
the SVG/DOM side; many production systems combine the two
(e.g. *React Flow for topology + Konva overlay for arbitrary
drawing* — today's pragmatic max-flexibility stack).

---

## 9. Recommendations

For "not large graphs, max human-centric flexibility, custom nodes,
groups, arrows, drawing":

| criterion | best |
|---|---|
| easiest + powerful | React Flow |
| enterprise diagrams | JointJS |
| maximal freedom | AntV X6 |
| custom graphics / free-draw | Konva |

For a 2026 *web* stack overall: **React Flow + ELK layout + custom
canvas rendering**. For *native* C++: **imgui-node-editor**.

For *workflow / control plane / observability* scenarios:

| task | best choice |
|---|---|
| realtime native C++ | ImGui node editor |
| web control plane | React Flow |
| huge dependency graph | Cytoscape / Sigma |
| infra architecture editor | JointJS / X6 |
| AI pipeline editor | Rete |

## Cross-references

- Diagram languages for category theory:
  [05-category-theory.md](05-category-theory.md)
- Underlying graph theory:
  [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- LLM workflow tooling parallels (LangFlow):
  [09-cognitive-architectures-neurosymbolic.md](09-cognitive-architectures-neurosymbolic.md)
