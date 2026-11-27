# P2.2f's environment: source it before any other script of this folder (`. tools/env.sh`).
#
# Everything the environment writes goes under $NLP, the gitignored build/nlp/ of the checkout (the host's rule:
# nothing under ~). Every cache a package or a model hub keeps is pointed there, and HOME itself is pointed at
# $NLP/home, so that whatever a tool would still write under ~ lands there, where it can be seen; the stage checks
# afterwards that the real ~ gained nothing.
#
#   EIN_NLP       the environment's folder; default <checkout>/build/nlp. A worktree points it at the main
#                 checkout's, so that the environment outlives the worktree.
#   EIN_NLP_RUNS  where the runs write; default <checkout>/plans/m2_nl_to_ir/p2.2f_text_analysis/check/runs
#                 (gitignored).

_here="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
_root="$(cd "$_here/../../../.." && pwd)"

export NLP="${EIN_NLP:-$_root/build/nlp}"
export NLP_RUNS="${EIN_NLP_RUNS:-$_root/plans/m2_nl_to_ir/p2.2f_text_analysis/check/runs}"
export NLP_TOOLS="$_here"
export NLP_REAL_HOME="${NLP_REAL_HOME:-$HOME}"

# uv: its cache, the Pythons it fetches and their links, no user configuration read.
export UV_CACHE_DIR="$NLP/cache/uv"
export UV_PYTHON_INSTALL_DIR="$NLP/python"
export UV_PYTHON_BIN_DIR="$NLP/python/bin"
export UV_TOOL_DIR="$NLP/uv-tools"
export UV_TOOL_BIN_DIR="$NLP/uv-tools/bin"
export UV_PYTHON_PREFERENCE=only-managed
export UV_NO_CONFIG=1
export UV_LINK_MODE=copy

# pip, the model hubs, the frameworks' caches.
export PIP_CACHE_DIR="$NLP/cache/pip"
export PIP_DISABLE_PIP_VERSION_CHECK=1
export HF_HOME="$NLP/cache/huggingface"
export HF_HUB_DISABLE_TELEMETRY=1
export TRANSFORMERS_NO_ADVISORY_WARNINGS=1
export TORCH_HOME="$NLP/cache/torch"
export TRITON_CACHE_DIR="$NLP/cache/triton"
export TORCHINDUCTOR_CACHE_DIR="$NLP/cache/inductor"
export CUDA_CACHE_PATH="$NLP/cache/nv"
export STANZA_RESOURCES_DIR="$NLP/models/stanza"
export NLTK_DATA="$NLP/models/nltk_data"   # NLTK is not used (Q11); a guard if a package imports it
export WN_DATA_DIR="$NLP/models/wn"          # the `wn` package's WordNet data (a lexicon, not NLTK's)
export MPLCONFIGDIR="$NLP/cache/matplotlib"
export XDG_CACHE_HOME="$NLP/cache/xdg"
export XDG_CONFIG_HOME="$NLP/config"
export XDG_DATA_HOME="$NLP/data"
export TMPDIR="$NLP/tmp"
export PYTHONNOUSERSITE=1
export PYTHONDONTWRITEBYTECODE=1   # no __pycache__ beside the scripts in the checkout

# The guard: anything that still writes under ~ writes here.
export HOME="$NLP/home"

mkdir -p "$NLP"/{cache/uv,cache/pip,cache/huggingface,cache/torch,cache/triton,cache/inductor,cache/nv,cache/matplotlib,cache/xdg} \
         "$NLP"/{python,uv-tools,models/stanza,models/amrlib,models/wn,config,data,tmp,home} "$NLP_RUNS"

# The venvs the setup makes (setup.sh): the main one, and one for a parser that needs an older stack.
export NLP_VENV="$NLP/venv"
export NLP_PY="$NLP_VENV/bin/python"
