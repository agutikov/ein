"""``python -m ein.cli`` entry point — dispatches to :func:`ein.cli.main`."""
from __future__ import annotations

from . import main

if __name__ == "__main__":
    raise SystemExit(main())
