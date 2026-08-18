#!/usr/bin/env bash
# The M1a bench environment (T1a.6.1.1) — a fingerprint, then the command.
#
#     utils/bench_env.sh cargo bench                  # in ein.rs/
#     utils/bench_env.sh python3 utils/bench_baseline.py
#     utils/bench_env.sh --core 6 ./target/release/ein solve ...
#     utils/bench_env.sh --report                     # fingerprint only
#
# Two jobs, and the first is the one that matters: **print the machine state
# every number was taken under**, to stderr, so no artefact in this phase can
# be read without it. A ratio measured on an E-core against one measured on a
# P-core is a ratio between two machines.
#
# The second is to pin: `taskset` onto one P-core hyperthread, `LC_ALL=C`, and
# `EIN_STDLIB` cleared so both implementations resolve the same checkout
# (design/11 § Resolution order).
#
# What it cannot do here: set the governor. `scaling_governor` is root-owned on
# this box, and asking for `sudo` in a bench script is worse than reporting
# `powersave` honestly and letting the variance column carry the consequence.
# That is why every bench in this phase reports best-of-N *and* spread, and why
# the fingerprint names the governor it ran under.
set -euo pipefail

CORE=4          # a P-core sibling on the dev machine; --core overrides
REPORT_ONLY=0

while [ $# -gt 0 ]; do
    case "$1" in
        --core) CORE="$2"; shift 2 ;;
        --report) REPORT_ONLY=1; shift ;;
        --) shift; break ;;
        *) break ;;
    esac
done

sysfile() { cat "$1" 2>/dev/null || echo "?"; }

cpu_kind() {
    # Hybrid Intel: the P-cores are the ones with a higher `cpuinfo_max_freq`
    # and an SMT sibling; `cpu_capacity` reads 1024 for both and cannot tell
    # them apart. Compare against the machine's maximum instead.
    local c="$1"
    local mine top sibs
    mine=$(sysfile "/sys/devices/system/cpu/cpu$c/cpufreq/cpuinfo_max_freq")
    top=$(cat /sys/devices/system/cpu/cpu*/cpufreq/cpuinfo_max_freq 2>/dev/null \
          | sort -n | tail -1)
    sibs=$(sysfile "/sys/devices/system/cpu/cpu$c/topology/thread_siblings_list")
    if [ "$mine" = "?" ]; then echo "unknown"; return; fi
    # Within ~15 % of the fastest core, and hyperthreaded ⇒ P-core.
    if [ "$((mine * 100 / top))" -ge 85 ] && [ "${sibs#*[,-]}" != "$sibs" ]; then
        echo "P-core (max $((mine / 1000)) MHz, siblings $sibs)"
    else
        echo "E-core (max $((mine / 1000)) MHz, siblings $sibs)"
    fi
}

{
    echo "── bench environment ─────────────────────────────────────"
    echo "  date            $(date -Is)"
    echo "  host            $(uname -sr) $(uname -m)"
    echo "  cpu             $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ *//')"
    echo "  pinned to       cpu$CORE — $(cpu_kind "$CORE")"
    echo "  governor        $(sysfile "/sys/devices/system/cpu/cpu$CORE/cpufreq/scaling_governor")" \
         "(epp $(sysfile "/sys/devices/system/cpu/cpu$CORE/cpufreq/energy_performance_preference"))"
    echo "  turbo           $([ "$(sysfile /sys/devices/system/cpu/intel_pstate/no_turbo)" = "0" ] \
                              && echo "on (no_turbo=0)" || echo "off")"
    echo "  cur / max MHz   $(( $(sysfile "/sys/devices/system/cpu/cpu$CORE/cpufreq/scaling_cur_freq") / 1000 ))" \
         "/ $(( $(sysfile "/sys/devices/system/cpu/cpu$CORE/cpufreq/scaling_max_freq") / 1000 ))"
    echo "  loadavg         $(cut -d' ' -f1-3 /proc/loadavg)"
    echo "  perf_paranoid   $(sysfile /proc/sys/kernel/perf_event_paranoid)" \
         "(2 = user-space samples only, enough for a self-time table)"
    echo "  git             $(git -C "$(dirname "$0")/.." rev-parse --short HEAD 2>/dev/null || echo '?')"
    echo "──────────────────────────────────────────────────────────"
} >&2

[ "$REPORT_ONLY" = "1" ] && exit 0
[ $# -eq 0 ] && { echo "usage: bench_env.sh [--core N] CMD…" >&2; exit 2; }

unset EIN_STDLIB
export LC_ALL=C
exec taskset -c "$CORE" "$@"
