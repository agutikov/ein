#!/usr/bin/env bash
# The M1a bench environment (T1a.6.1.1) — a fingerprint, then the command.
#
#     utils/bench_env.sh cargo bench                  # in ein.rs/
#     utils/bench_env.sh python3 utils/e2e_baseline.py
#     utils/bench_env.sh --core 6 ./target/release/ein solve ...
#     utils/bench_env.sh --cores P:8 ./target/release/ein solve ... --jobs 8
#     utils/bench_env.sh --report                     # fingerprint only
#     utils/bench_env.sh --cores E:4 --report         # ...for a named core set
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
# `--cores` (P1a.7 S1a.7.1) is the multi-core form, and it exists because
# `--core` cannot state a scaling measurement. On a hybrid CPU "8 cores" names
# three different machines — 8 physical P-cores, 8 P-core *threads* on 4
# physical cores, or 8 E-cores — which differ by more than the speedup being
# measured, so a `--jobs N` number that does not say which is not a
# measurement (docs/history/m1a_rust/measurements/scaling.md, preamble). The
# spec is `P:N` / `PT:N` / `E:N` / `ET:N`, or a literal taskset list:
#
#   P:8    8 physical P-cores  — one thread per core, no SMT sibling shared
#   PT:8   8 P-core threads    — ascending cpu order, so 4 physical cores
#   E:8    8 physical E-cores  — the E-cores have no SMT, so E == ET
#   0,2,4  whatever you meant, reported back with each core classified
#
# `P` / `PT` / `E` with no count take every core of that kind. The fingerprint
# prints the resolved list *and* how many distinct physical cores it covers,
# which is the number a reader needs and the one a `--jobs` flag never says.
#
# What it cannot do here: set the governor. `scaling_governor` is root-owned on
# this box, and asking for `sudo` in a bench script is worse than reporting
# `powersave` honestly and letting the variance column carry the consequence.
# That is why every bench in this phase reports best-of-N *and* spread, and why
# the fingerprint names the governor it ran under.
set -euo pipefail

CORE=4          # a P-core sibling on the dev machine; --core overrides
CORES=""        # --cores spec; overrides --core when given
REPORT_ONLY=0

while [ $# -gt 0 ]; do
    case "$1" in
        --core) CORE="$2"; CORES=""; shift 2 ;;
        --cores) CORES="$2"; shift 2 ;;
        --report) REPORT_ONLY=1; shift ;;
        --) shift; break ;;
        *) break ;;
    esac
done

sysfile() { cat "$1" 2>/dev/null || echo "?"; }

all_cpus() {
    for d in /sys/devices/system/cpu/cpu[0-9]*; do
        echo "${d##*/cpu}"
    done | sort -n
}

# "P" or "E" for one cpu — the classification `cpu_kind` prints in words.
kind_of() {
    local c="$1" mine top sibs
    mine=$(sysfile "/sys/devices/system/cpu/cpu$c/cpufreq/cpuinfo_max_freq")
    top=$(cat /sys/devices/system/cpu/cpu*/cpufreq/cpuinfo_max_freq 2>/dev/null \
          | sort -n | tail -1)
    sibs=$(sysfile "/sys/devices/system/cpu/cpu$c/topology/thread_siblings_list")
    [ "$mine" = "?" ] && { echo "?"; return; }
    if [ "$((mine * 100 / top))" -ge 85 ] && [ "${sibs#*[,-]}" != "$sibs" ]; then
        echo "P"
    else
        echo "E"
    fi
}

# The first sibling of a cpu's SMT group — its physical core's representative.
# Sorting numerically matters: `thread_siblings_list` is "0,1" here and "0-1"
# on other kernels, so both separators are cut.
leader_of() {
    local sibs
    sibs=$(sysfile "/sys/devices/system/cpu/cpu$1/topology/thread_siblings_list")
    [ "$sibs" = "?" ] && { echo "$1"; return; }
    echo "$sibs" | tr ',-' '\n\n' | sort -n | head -1
}

# Resolve a --cores spec to a comma-separated taskset list.
#
# `P:N` takes one thread per *physical* core and `PT:N` takes threads in cpu
# order; on this machine those are cpu0,2,4,… and cpu0,1,2,… respectively, and
# confusing them is the whole reason this function exists.
resolve_cores() {
    local spec="$1" want kind n picked seen leader
    case "$spec" in
        [0-9]*) echo "$spec"; return ;;
        P|P:*)   kind=P; want=one-per-core ;;
        PT|PT:*) kind=P; want=per-thread ;;
        E|E:*)   kind=E; want=one-per-core ;;
        ET|ET:*) kind=E; want=per-thread ;;
        *) echo "bench_env.sh: unknown --cores spec '$spec'" >&2; exit 2 ;;
    esac
    n="${spec#*:}"
    [ "$n" = "$spec" ] && n=0          # no ":N" — take them all
    picked=""; seen=" "
    for c in $(all_cpus); do
        [ "$(kind_of "$c")" = "$kind" ] || continue
        if [ "$want" = one-per-core ]; then
            leader=$(leader_of "$c")
            case "$seen" in *" $leader "*) continue ;; esac
            seen="$seen$leader "
        fi
        picked="${picked:+$picked,}$c"
        [ "$n" -gt 0 ] && [ "$(echo "$picked" | tr ',' '\n' | wc -l)" -ge "$n" ] && break
    done
    [ -z "$picked" ] && {
        echo "bench_env.sh: no $kind-cores found for --cores '$spec'" >&2; exit 2; }
    local got
    got=$(echo "$picked" | tr ',' '\n' | wc -l)
    [ "$n" -gt 0 ] && [ "$got" -lt "$n" ] && {
        echo "bench_env.sh: --cores '$spec' wanted $n, this machine has $got" >&2; exit 2; }
    echo "$picked"
}

# "8 cpus, 8 physical cores, all P" — the line a scaling table needs.
describe_cores() {
    local list="$1" cpus=0 kinds="" leaders=" " leader n_phys=0
    for c in $(echo "$list" | tr ',' ' '); do
        cpus=$((cpus + 1))
        kinds="$kinds$(kind_of "$c")"
        leader=$(leader_of "$c")
        case "$leaders" in *" $leader "*) ;; *) leaders="$leaders$leader "; n_phys=$((n_phys + 1)) ;; esac
    done
    local p e
    p=$(echo "$kinds" | tr -cd P | wc -c)
    e=$(echo "$kinds" | tr -cd E | wc -c)
    printf '%s cpu(s), %s physical core(s), %s' "$cpus" "$n_phys" \
        "$( [ "$e" = 0 ] && echo "all P" || { [ "$p" = 0 ] && echo "all E" || echo "$p P + $e E"; } )"
}

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

if [ -n "$CORES" ]; then
    CORE_LIST=$(resolve_cores "$CORES")
    REPORT_CORE=${CORE_LIST%%,*}
else
    CORE_LIST="$CORE"
    REPORT_CORE="$CORE"
fi

{
    echo "── bench environment ─────────────────────────────────────"
    echo "  date            $(date -Is)"
    echo "  host            $(uname -sr) $(uname -m)"
    echo "  cpu             $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ *//')"
    if [ -n "$CORES" ]; then
        echo "  pinned to       cpu$CORE_LIST — $(describe_cores "$CORE_LIST")"
        echo "  core spec       --cores $CORES"
    else
        echo "  pinned to       cpu$CORE — $(cpu_kind "$CORE")"
    fi
    echo "  governor        $(sysfile "/sys/devices/system/cpu/cpu$REPORT_CORE/cpufreq/scaling_governor")" \
         "(epp $(sysfile "/sys/devices/system/cpu/cpu$REPORT_CORE/cpufreq/energy_performance_preference"))"
    echo "  turbo           $([ "$(sysfile /sys/devices/system/cpu/intel_pstate/no_turbo)" = "0" ] \
                              && echo "on (no_turbo=0)" || echo "off")"
    echo "  cur / max MHz   $(( $(sysfile "/sys/devices/system/cpu/cpu$REPORT_CORE/cpufreq/scaling_cur_freq") / 1000 ))" \
         "/ $(( $(sysfile "/sys/devices/system/cpu/cpu$REPORT_CORE/cpufreq/scaling_max_freq") / 1000 ))"
    echo "  loadavg         $(cut -d' ' -f1-3 /proc/loadavg)"
    echo "  perf_paranoid   $(sysfile /proc/sys/kernel/perf_event_paranoid)" \
         "(2 = user-space samples only, enough for a self-time table)"
    echo "  git             $(git -C "$(dirname "$0")/.." rev-parse --short HEAD 2>/dev/null || echo '?')"
    echo "──────────────────────────────────────────────────────────"
} >&2

[ "$REPORT_ONLY" = "1" ] && exit 0
[ $# -eq 0 ] && {
    echo "usage: bench_env.sh [--core N | --cores P:8|PT:8|E:8|0,2,4] CMD…" >&2; exit 2; }

unset EIN_STDLIB
export LC_ALL=C
exec taskset -c "$CORE_LIST" "$@"
