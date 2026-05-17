window.cyStyle = [
  {
    "selector": "node",
    "style": {
      "label": "data(label)",
      "text-wrap": "wrap",
      "text-max-width": 140,
      "text-valign": "center",
      "text-halign": "center",
      "font-family": "Helvetica, Arial, sans-serif",
      "font-size": 10,
      "color": "#222",
      "background-color": "#f8f8f8",
      "border-width": 1,
      "border-color": "#666",
      "width": "label",
      "height": "label",
      "padding": "8px",
      "shape": "round-rectangle"
    }
  },
  {
    "selector": ":parent",
    "style": {
      "shape": "round-rectangle",
      "background-color": "data(fill)",
      "background-opacity": 0.45,
      "border-width": 1,
      "border-color": "data(border)",
      "padding": "18px",
      "text-valign": "top",
      "text-halign": "center",
      "font-size": 12,
      "font-weight": "bold",
      "color": "#333"
    }
  },
  {
    "selector": "node.cluster-d0",
    "style": {
      "font-size": 13,
      "background-opacity": 0.3,
      "border-width": 2
    }
  },
  {
    "selector": "node.cluster-d1",
    "style": {
      "font-size": 11,
      "background-opacity": 0.55,
      "border-style": "dashed"
    }
  },
  {
    "selector": "node[kind = \"concept\"]",
    "style": {
      "shape": "ellipse",
      "background-color": "#cfe6f5",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"algorithm\"]",
    "style": {
      "shape": "hexagon",
      "background-color": "#fff3a8",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"software\"]",
    "style": {
      "shape": "rectangle",
      "background-color": "#b8edb8",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"paper\"]",
    "style": {
      "shape": "tag",
      "background-color": "#ffd9b8",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"language\"]",
    "style": {
      "shape": "barrel",
      "background-color": "#ffd1da",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"data_structure\"]",
    "style": {
      "shape": "cut-rectangle",
      "background-color": "#ecc5ff",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"standard\"]",
    "style": {
      "shape": "round-tag",
      "background-color": "#dddddd",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"problem_class\"]",
    "style": {
      "shape": "diamond",
      "background-color": "#ffe066",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"cog_arch\"]",
    "style": {
      "shape": "octagon",
      "background-color": "#f7a4a4",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"notation\"]",
    "style": {
      "shape": "rhomboid",
      "background-color": "#d4f4f4",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"proof\"]",
    "style": {
      "shape": "round-rectangle",
      "background-color": "#efd7a8",
      "border-color": "#555"
    }
  },
  {
    "selector": "node[kind = \"domain\"]",
    "style": {
      "shape": "heptagon",
      "background-color": "#dbc6dc",
      "border-color": "#555"
    }
  },
  {
    "selector": "edge",
    "style": {
      "label": "data(label)",
      "font-size": 8,
      "font-family": "Helvetica, Arial, sans-serif",
      "color": "#444",
      "line-color": "#888888",
      "target-arrow-color": "#888888",
      "target-arrow-shape": "triangle",
      "curve-style": "bezier",
      "width": 1.2,
      "text-background-color": "#ffffff",
      "text-background-opacity": 0.85,
      "text-background-padding": "2px",
      "text-rotation": "autorotate"
    }
  },
  {
    "selector": "edge[eStyle = \"dashed\"]",
    "style": {
      "line-style": "dashed"
    }
  },
  {
    "selector": "edge[eStyle = \"dotted\"]",
    "style": {
      "line-style": "dotted"
    }
  },
  {
    "selector": "edge[eStyle = \"bold\"]",
    "style": {
      "width": 2.6
    }
  },
  {
    "selector": "edge[eColor = \"purple\"]",
    "style": {
      "line-color": "#7e3aa1",
      "target-arrow-color": "#7e3aa1"
    }
  },
  {
    "selector": "edge[eColor = \"red\"]",
    "style": {
      "line-color": "#c0392b",
      "target-arrow-color": "#c0392b"
    }
  },
  {
    "selector": "edge[eColor = \"blue\"]",
    "style": {
      "line-color": "#2a6fcf",
      "target-arrow-color": "#2a6fcf"
    }
  },
  {
    "selector": "edge[eColor = \"darkgreen\"]",
    "style": {
      "line-color": "#1f7a30",
      "target-arrow-color": "#1f7a30"
    }
  },
  {
    "selector": "edge[eColor = \"grey40\"]",
    "style": {
      "line-color": "#666666",
      "target-arrow-color": "#666666"
    }
  },
  {
    "selector": "edge[eColor = \"grey30\"]",
    "style": {
      "line-color": "#4d4d4d",
      "target-arrow-color": "#4d4d4d"
    }
  },
  {
    "selector": "edge[?invis]",
    "style": {
      "display": "none"
    }
  },
  {
    "selector": "edge[?cross]",
    "style": {
      "z-index": 10,
      "opacity": 0.95
    }
  }
];
