Да. При **Rust reasoning engine + Monaco + Cytoscape.js** я бы сейчас ставил на **Tauri 2**, а Qt убрал из shortlist почти полностью.

Тут получается очень чистая архитектура:

```text
┌───────────────────────────────────────────────┐
│                Tauri desktop app              │
│                                               │
│  WebView                                      │
│  ┌──────────────┬──────────────┬────────────┐ │
│  │ Monaco       │ Cytoscape.js │ SearchTree │ │
│  │ ein-lang     │ ein-graph    │ DAG/tree   │ │
│  └──────────────┴──────────────┴────────────┘ │
│             TypeScript / React                │
│                     │                         │
│               Tauri invoke/events             │
│                     │                         │
│              Rust application layer           │
│                     │                         │
│              ein-core / ein-engine            │
│                     │                         │
│        parser / saturator / solver / IR        │
└───────────────────────────────────────────────┘
```

Это именно тот случай, для которого Tauri хорошо подходит: UI — HTML/JS/CSS в системном WebView, application/backend layer — Rust. Tauri не тащит свой Chromium как Electron; он использует системный WebView через WRY, а окно создаётся через TAO. ([Tauri][1])

## Что такое Tauri в твоём случае

Не надо воспринимать его как «Rust-версию Electron».

Electron примерно:

```text
Node.js
+
Chromium
+
JS frontend
+
native bindings
```

Tauri:

```text
Rust process
+
OS WebView
+
JS frontend
```

То есть у тебя Rust **не sidecar и не отдельный backend server**. Сам Tauri backend — Rust. Frontend вызывает Rust-функции через Tauri commands; команды могут принимать аргументы, возвращать значения, ошибки и быть async. ([Tauri][2])

Поэтому если сейчас условно есть:

```text
ein-core/
ein-parser/
ein-search/
ein-cli/
```

ты добавляешь:

```text
ein-gui/
```

и он может зависеть непосредственно от тех же crates:

```text
ein-gui
 ├─ tauri
 ├─ ein-core
 ├─ ein-parser
 └─ ein-search
```

Никакого:

```text
Rust -> C ABI -> Qt
```

или

```text
JS -> HTTP -> Rust server
```

не требуется.

---

# Я бы сделал workspace примерно так

```text
ein/
├── crates/
│   ├── ein-core/
│   ├── ein-ir/
│   ├── ein-parser/
│   ├── ein-reasoning/
│   ├── ein-search/
│   └── ein-render/
│
├── cli/
│   └── ...
│
└── gui/
    ├── src/                    # TypeScript frontend
    │   ├── components/
    │   ├── views/
    │   │   ├── LangView.tsx
    │   │   ├── GraphView.tsx
    │   │   └── BranchView.tsx
    │   ├── stores/
    │   │   └── session.ts
    │   └── ...
    │
    ├── src-tauri/
    │   ├── src/
    │   │   ├── lib.rs
    │   │   ├── commands.rs
    │   │   └── session.rs
    │   └── Cargo.toml
    │
    └── package.json
```

Причём я бы **не делал отдельный GUI API crate на M1b**, если он пока не нужен.

Tauri layer может напрямую использовать публичный Rust API движка.

---

# Frontend stack

Я бы взял:

```text
Tauri 2
React
TypeScript
Vite

Monaco Editor
Cytoscape.js
cytoscape-fcose

Zustand
```

React здесь не принципиален. Svelte тоже отлично подходит. Но для IDE-like UI React имеет огромное количество готовых компонентов и паттернов.

### Почему Zustand, а не Redux

У тебя state достаточно концептуально простой:

```ts
Session {
    puzzle
    source
    parsedIr
    searchTree

    selectedStateId
    selectedGraphLayer
    graphMode
    layoutMode

    graphPositions
}
```

Redux здесь, скорее всего, создаст больше ceremony, чем пользы.

---

# Tauri ↔ Rust engine

Предположим, пользователь открыл:

```text
examples/zebra2.ein
```

Frontend вызывает:

```ts
const puzzle = await invoke("load_puzzle", {
    path
})
```

Rust:

```rust
#[tauri::command]
fn load_puzzle(path: PathBuf) -> Result<PuzzleSessionDto, GuiError> {
    let source = std::fs::read_to_string(&path)?;
    let ir = ein_parser::parse(&source)?;

    Ok(PuzzleSessionDto {
        source,
        ir: ir.into(),
        ...
    })
}
```

Это стандартная модель Tauri commands. ([Tauri][2])

Но важный архитектурный момент:

## Не тащи весь KB туда-сюда как JSON

Я бы не делал:

```text
Rust KB
   ↓ serialize 20 MB
JS
   ↓ edit
serialize 20 MB
   ↓
Rust
```

Вместо этого:

```text
Rust

SessionId -> Session {
    KB,
    SearchTree,
    ...
}
```

а JS получает только view models.

Например:

```ts
invoke("graph_for_state", {
    sessionId,
    stateId,
    mode: "levi"
})
```

возвращает:

```ts
{
    nodes: [...],
    edges: [...]
}
```

---

# Rust должен владеть семантикой

Это, на мой взгляд, ключевой boundary.

JS не должен знать, что значит:

```ein
f -0-> a
f -1-> b
```

на семантическом уровне.

JS знает только:

```ts
GraphNode
GraphEdge
SourceRange
StateId
RuleId
```

Rust знает:

```text
EinGraph
Relation
Fact
Rule
SearchState
Saturation
```

То есть:

```text
              semantics
                  ↓
      ┌─────────────────────┐
      │        Rust         │
      └─────────────────────┘
                  │
             presentation DTO
                  │
      ┌─────────────────────┐
      │      TypeScript     │
      └─────────────────────┘
                  ↓
             visualization
```

Так UI не превращается во второй implementation ein.

---

# View 1 — Monaco

Здесь web stack просто явно лучше Qt.

Получишь:

```text
syntax highlighting
bracket matching
diagnostics
hover
go-to-definition
selection ranges
code folding
diff view
```

И главное — очень удобно связать текст с графом.

Например Rust parser возвращает:

```rust
struct FactDto {
    id: FactId,
    span: SourceSpan,
    ...
}
```

JS получает:

```ts
{
  id: "fact:123",
  range: {
    startLine: 18,
    startColumn: 5,
    endLine: 18,
    endColumn: 20
  }
}
```

И тогда:

```text
click graph edge
       ↓
FactId
       ↓
Monaco revealRange(...)
```

И наоборот:

```text
cursor / selection
       ↓
source position
       ↓
FactId
       ↓
Cytoscape highlight
```

Это может стать одной из самых полезных возможностей GUI.

---

# View 2 — Cytoscape.js

Здесь Qt уже особенно трудно оправдать.

Cytoscape.js умеет не просто drawing, а полноценную interactive graph model: selection, events, styles, compound nodes, layout extensions и т.д. Его документация прямо рекомендует fCoSE как первый force-directed layout для рассмотрения; fCoSE поддерживает fixed-position, alignment и relative-placement constraints. ([Cytoscape.js][3])

То есть твои:

```text
compact / Levi
ontology / fact / reasoning / rules

drag
selection
manual placement
auto-layout
```

попадают прямо в его use case.

Особенно интересно, что fCoSE умеет constraints:

```text
fixed node
alignment
relative placement
```

Это может оказаться гораздо полезнее, чем просто «drag node and save XY», потому что layout можно сохранить **семантически**:

```json
{
  "fixed": {
    "person": [500, 300]
  },

  "alignVertical": [
    ["house1", "house2", "house3"]
  ],

  "relative": [
    ["ontology", "above", "facts"]
  ]
}
```

а потом перелэйаутить остальные вершины вокруг этого. fCoSE прямо поддерживает такие ограничения. ([Cytoscape.js][3])

---

# Graphviz при этом я бы сохранил

У тебя уже есть:

```text
DOT
FDP
SFDP
OSAGE
```

Не нужно заменять их fCoSE.

Лучше:

```text
Layout
 ├── dot
 ├── fdp
 ├── sfdp
 ├── osage
 └── fcose
```

Но есть два варианта реализации.

### Старые Graphviz layouts

Rust запускает:

```text
dot -Tjson
fdp -Tjson
...
```

и отдаёт координаты frontend.

### fCoSE

JS/Cytoscape считает его непосредственно внутри WebView.

Это нормальное разделение.

---

# View 3 — SearchTree

И здесь web stack тоже оказывается удобным.

Я бы, кстати, **не использовал Cytoscape для folders mode**.

Сделал бы два renderer одного SearchTree DTO:

```text
SearchTree
    │
    ├── GitGraphView
    │
    └── TreeView
```

Git mode можно сделать SVG/React или graph library.

Folders mode — обычный virtualized tree.

Главное — один ID:

```rust
StateId
```

И вся синхронизация идёт через него.

```text
click branch node

StateId(472)

     ├───────────► Monaco
     │             to_ir(state 472)
     │
     └───────────► Cytoscape
                   graph(state 472)
```

---

# А где реально находится Session?

Вот здесь я бы изменил рекомендацию относительно предыдущего ответа.

**Source of truth должен быть Rust Session**, не JS store.

```text
              ┌───────────────┐
              │ Rust Session  │
              │               │
              │ Source        │
              │ IR            │
              │ KB            │
              │ SearchTree    │
              │ Edit history  │
              └───────┬───────┘
                      │
                 projection
                      │
              ┌───────▼───────┐
              │ frontend store│
              └───────────────┘
```

Frontend store — только projection/cache/UI state.

Это важно, потому что parser/reasoning/editor semantics у тебя Rust.

---

# Editing

Например пользователь в graph view рисует:

```text
Alice ─likes→ Bob
```

Cytoscape **не меняет KB**.

Он посылает intent:

```ts
invoke("add_relation", {
    sessionId,
    relation: {
        type: "likes",
        args: ["Alice", "Bob"]
    }
})
```

Rust:

```text
apply edit
    ↓
modify IR
    ↓
to_ir()
    ↓
parse again
    ↓
validate
    ↓
new Session revision
```

и возвращает patch:

```ts
{
    revision: 43,
    sourcePatch: ...,
    graphPatch: ...
}
```

Это очень хорошо соответствует твоему acceptance:

> round-trip through the IR parser to keep the file authoritative.

---

# Я бы даже формализовал GUI API как Commands + Events

### Commands

Frontend → Rust:

```text
open_puzzle
save_puzzle

select_state
get_state_ir
get_state_graph

edit_source
add_fact
remove_fact
add_relation
remove_relation

run_layout
save_layout

undo
redo
```

### Events

Rust → frontend:

```text
session-changed
parse-error
state-changed
search-tree-changed
```

Tauri поддерживает как вызов Rust из frontend через commands, так и обратное взаимодействие через events/channels. ([Tauri][2])

Для bulk graph data:

```text
command → result
```

Для streaming/live mode позже:

```text
channel/events
```

То есть когда появится:

> real-time engine integration

архитектуру менять не придётся.

---

# Очень важный плюс Tauri: filesystem

Browser-only решение упирается в filesystem.

Tauri снимает проблему:

```text
Open...
Save
Save As...
recent files
watch file
layout sidecar
```

Есть официальный dialog plugin для native open/save dialogs и filesystem plugin для работы с файлами. ([Tauri][4])

Например:

```text
zebra2.ein
zebra2.ein.layout.json
```

или:

```text
.ein/
    layouts/
        zebra2.json
```

---

# Что тогда вообще даёт Tauri поверх browser app?

Это хороший вопрос, потому что UI действительно можно сделать просто:

```text
React + Monaco + Cytoscape
```

и открыть localhost.

Tauri добавляет ровно desktop boundary:

```text
             plain browser        Tauri

Rust API     HTTP/WebSocket       direct command
files        restricted           native
dialogs      browser              native
menus        limited              native
window       browser tab          application
shortcuts    constrained          desktop
packaging    web server           executable
processes    no                   yes
OS access    constrained          controlled
```

При этом Tauri предоставляет native window/menu facilities и плагины для dialog, filesystem, shell/process и т.п. ([Tauri][4])

То есть Tauri — **не UI framework**.

Он здесь:

> native application host + Rust/JS bridge + packaging/security boundary.

Это именно то, чего не хватает обычному browser frontend.

---

# И почему не Electron

Теперь разница особенно очевидна.

Electron:

```text
ein Rust engine

    ↕

Node.js

    ↕

Chromium

    ↕

React
```

Tauri:

```text
ein Rust engine
      │
      │ ordinary Rust crate calls
      ▼
Tauri Rust
      │
      │ invoke
      ▼
React
```

У тебя уже backend на языке, на котором написан Tauri.

Поэтому Node вообще не нужен в runtime.

npm/pnpm нужен **на этапе frontend build**, но production architecture от Node не зависит.

---

# WebView caveat

Вот это реальный минус Tauri, который стоит учитывать.

Electron несёт одну определённую версию Chromium.

Tauri использует platform WebView. На Windows это WebView2/Edge; Tauri официально требует WebView2 для разработки/работы платформы. ([Tauri][5])

Следовательно, rendering engine не идентичен на всех OS.

Для обычной IDE-like программы с:

```text
Monaco
Cytoscape
React
CSS grid
```

я бы считал это приемлемым.

Но testing Linux/Windows/macOS потребуется.

---

# Qt vs Tauri после твоего уточнения

Теперь я бы поставил так:

|                                | Qt/QML |  Tauri |
| ------------------------------ | -----: | -----: |
| Rust engine integration        |      7 | **10** |
| Monaco                         |      5 | **10** |
| Cytoscape                      |      5 | **10** |
| fCoSE                          |      5 | **10** |
| IDE UI                         |      9 |  **9** |
| Native filesystem              |     10 |     10 |
| Graph editor                   |      9 | **10** |
| Browser reuse                  |      3 | **10** |
| Development speed              |      6 |  **9** |
| Single semantic implementation |      8 | **10** |
| Future web viewer              |      3 | **10** |

Qt теперь имеет смысл только если ты решишь:

> «Я принципиально хочу native rendering и native widgets, Cytoscape/Monaco больше не нужны».

Но это уже совсем другая архитектура.

---

# Более важное следствие

С Tauri **M1b GUI почти целиком становится reusable web application**.

Можно спроектировать:

```text
packages/
    ein-ui/
        Monaco
        Cytoscape
        SearchTree
        session model

apps/
    ein-web/
    ein-tauri/
```

и иметь:

```text
             ein-ui
             /    \
            /      \
       browser     Tauri
                     │
                   Rust
```

Browser version может открывать заранее сохранённый artefact:

```text
.ein
search-tree.json
states.json
```

а Tauri version получает полноценный editable/run-capable backend.

Это хорошо соответствует твоему исходному разделению:

> first cut is load-saved-artefact

и позже:

> live mode lands when there's a use case.

---

## Я бы в итоге зафиксировал M1b stack так

```text
Desktop shell:
    Tauri 2

Backend / semantic model:
    Rust
    existing ein crates

Frontend:
    TypeScript
    React
    Vite

View 1:
    Monaco Editor

View 2:
    Cytoscape.js
    cytoscape-fcose

View 3:
    React tree + SVG/graph renderer

Application state:
    Rust Session — semantic source of truth
    Zustand — UI projection/state

IPC:
    Tauri commands
    Tauri channels/events where asynchronous

Persistence:
    .ein — semantic authoritative format
    separate *.layout.json — purely visual state

Auto-layout:
    Graphviz — existing DOT/FDP/SFDP/OSAGE
    fCoSE — interactive layout

Packaging:
    Tauri
```

И ещё я бы слегка поправил формулировку open question в M1b: **`Qt vs Electron vs browser` уже не совсем правильный выбор**. Реальный выбор теперь скорее **`Tauri + web frontend` vs `pure browser frontend`**. Electron не даёт существенного архитектурного преимущества, а Qt заставляет либо отказаться от Monaco/Cytoscape, либо использовать Qt как оболочку для той же web-технологии, которую Tauri оборачивает существенно естественнее. ([Tauri][1])

В контексте текущей архитектуры Ein я бы выбрал **Tauri + React + Monaco + Cytoscape, причём сам `ein` engine подключал бы как обычные Rust crates, а не запускал CLI subprocess**.

[1]: https://v2.tauri.app/concept/architecture/?utm_source=chatgpt.com "Tauri Architecture"
[2]: https://v2.tauri.app/develop/calling-rust/?utm_source=chatgpt.com "Calling Rust from the Frontend"
[3]: https://js.cytoscape.org/?utm_source=chatgpt.com "Cytoscape.js"
[4]: https://v2.tauri.app/plugin/dialog/?utm_source=chatgpt.com "Dialog"
[5]: https://v2.tauri.app/start/prerequisites/?utm_source=chatgpt.com "Prerequisites"
