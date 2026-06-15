"""``ein kb`` — knowledge-base utilities: render the loaded KB as DOT."""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

from ..kb import Layer
from ._common import _LAYER_BY_NAME, _load_kb_or_exit


def _cmd_kb_dot(args: argparse.Namespace) -> int:
    """Render the loaded KB as a single unified Graphviz digraph."""
    kb = _load_kb_or_exit(Path(args.file))
    if kb is None:
        return 1
    # Parse --layers spec; default to all.
    if args.layers:
        try:
            layer_set = tuple(
                _LAYER_BY_NAME[name.strip().lower()]
                for name in args.layers.split(",")
            )
        except KeyError as e:
            valid = ",".join(sorted(_LAYER_BY_NAME))
            print(f"unknown layer {e}; pick from {valid}", file=sys.stderr)
            return 2
    else:
        layer_set = (Layer.ONTOLOGY, Layer.FACT, Layer.REASONING)
    sys.stdout.write(kb.to_dot(
        layers=layer_set,
        colour_by=args.colour_by,
        include_types=not args.no_types,
        include_instances=not args.no_instances,
        name=args.graph_name,
    ))
    sys.stdout.write("\n")
    return 0


def add_parser(sub) -> None:
    kb_cmd = sub.add_parser("kb", help="Knowledge-base utilities")
    kb_sub = kb_cmd.add_subparsers(dest="kb_cmd", required=True)

    kb_dot = kb_sub.add_parser(
        "dot",
        help="render the loaded KB as a unified DOT graph (S1.2.4)",
    )
    kb_dot.add_argument("file")
    kb_dot.add_argument(
        "--layers",
        default=None,
        help="comma-separated layer subset (ontology,facts,reasoning); default: all",
    )
    kb_dot.add_argument(
        "--colour-by",
        choices=["relation", "layer", "none"],
        default="relation",
        help="colour edges by relation (default), by layer, or none",
    )
    kb_dot.add_argument(
        "--no-types",
        action="store_true",
        help="omit Type nodes (boxes)",
    )
    kb_dot.add_argument(
        "--no-instances",
        action="store_true",
        help="omit Instance nodes (ovals)",
    )
    kb_dot.add_argument(
        "--graph-name",
        default="kb",
        help="DOT graph name (default 'kb')",
    )
    kb_dot.set_defaults(func=_cmd_kb_dot)
