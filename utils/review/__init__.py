"""The review bundle's modules — M1e2 P1e2.1.

`utils/review_bundle.py` is the entry point; this package is what it is made
of. The line `utils/README.md` § Renderers draws holds here too: `ein` writes
DOT, markdown, JSON and `.ein` for one run, and these modules **assemble** —
they run the binary, keep what it wrote verbatim under `raw/`, and lay it out
as pages. A table here is a transcription of an engine file; a diff here is a
set difference of fact lines the engine wrote, and says so where it is shown.
"""
