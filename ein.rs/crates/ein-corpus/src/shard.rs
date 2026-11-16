//! **A sweep's shards**: a sweep too long for one job is split into
//! parallel jobs, each a shard of it, which together do what the whole sweep
//! does.
//!
//! `EIN_TEST_SHARD=k/n`, `1 ≤ k ≤ n`, makes a run shard `k` of `n`. A sweep
//! that is sharded keeps the programs [`assign`] deals to its shard, through
//! [`keep`], and does everything else as it did: every program is in exactly
//! one shard of `n`, so the `n` jobs together hold every program to every
//! claim the unsharded run holds it to. A test that is one computation rather
//! than a sweep runs whole in one shard, [`whole_in`]. Unset, a run is the
//! whole run, and a value that is not a shard is a panic, not the whole run,
//! as `EIN_TEST_JOBS` is: a mistyped shard that ran everything would be a
//! job doing another's work with nothing to say so.
//!
//! **How the programs are dealt.** In the order the pool starts them (the
//! heavy lane first, then the programs with a long unit, then by the
//! manifest's `cost_ms`, then by name), in turn: the first `n` programs to
//! shards 1 to `n`, the next `n` to shards 1 to `n` again, and so on. The
//! long programs, which decide a shard's time, go one to a shard before any
//! shard takes a second, and two long ones next to each other in that order
//! (a program and its variant, `zebra2-minus-15` and its `-obligations`) go
//! to two shards. Dealing reads nothing a result depends on, and it is the
//! same on every machine: what is dealt is the sweep's own list of programs.

use crate::pool;

/// One shard of a sharded run: shard `k` of `n`, `1 ≤ k ≤ n`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shard {
    pub k: usize,
    pub n: usize,
}

impl std::fmt::Display for Shard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.k, self.n)
    }
}

/// `k/n`, read; `None` for anything that is not one.
pub fn parse(s: &str) -> Option<Shard> {
    let (k, n) = s.trim().split_once('/')?;
    let (k, n) = (k.trim().parse().ok()?, n.trim().parse().ok()?);
    (1 <= k && k <= n).then_some(Shard { k, n })
}

/// **This run's shard**, `EIN_TEST_SHARD`, or `None`: the whole run.
pub fn current() -> Option<Shard> {
    let s = std::env::var("EIN_TEST_SHARD").ok()?;
    Some(
        parse(&s)
            .unwrap_or_else(|| panic!("EIN_TEST_SHARD={s:?} is not a shard k/n with 1 <= k <= n")),
    )
}

/// **The shard, 1 to `n`, each program is dealt to**: `programs` are a
/// sweep's programs, repo-relative, in the sweep's order. See the module
/// note for the rule.
pub fn assign(programs: &[String], n: usize) -> Vec<usize> {
    assert!(n >= 1, "a sharding into {n} shards");
    let mut out = vec![0; programs.len()];
    for (j, &i) in pool::start_order(programs).iter().enumerate() {
        out[i] = 1 + j % n;
    }
    out
}

/// **The items of this run's shard**, in their order; every item with no
/// shard. `program` names an item's program, repo-relative: two items of
/// one program are in one shard.
pub fn keep<T>(items: Vec<T>, program: impl Fn(&T) -> String) -> Vec<T> {
    let Some(shard) = current() else {
        return items;
    };
    keep_in(items, program, shard)
}

/// [`keep`] for a given shard.
pub fn keep_in<T>(items: Vec<T>, program: impl Fn(&T) -> String, shard: Shard) -> Vec<T> {
    let names: Vec<String> = items.iter().map(&program).collect();
    let mut distinct = names.clone();
    distinct.sort();
    distinct.dedup();
    let dealt = assign(&distinct, shard.n);
    items
        .into_iter()
        .zip(&names)
        .filter(|(_, p)| {
            let at = distinct.binary_search(p).expect("a program of the list");
            dealt[at] == shard.k
        })
        .map(|(t, _)| t)
        .collect()
}

/// **Whether a test that is one computation runs in this shard**: in shard
/// `k` of any sharding (`k` taken round the `n` shards), and in every
/// unsharded run. A sharded test binary's sweeps split by program, and a test
/// of it that is not a sweep runs whole, once among the `n` jobs.
pub fn whole_in(k: usize) -> bool {
    assert!(k >= 1, "shards count from 1");
    current().is_none_or(|s| (k - 1) % s.n + 1 == s.k)
}

/// A floor a sweep holds over every program, held by one shard of it: the
/// floor's share, `floor / n`, at least one. With no shard, the floor.
pub fn share(floor: usize) -> usize {
    current().map_or(floor, |s| (floor / s.n).max(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shard_is_read_and_a_non_shard_is_not() {
        assert_eq!(parse("2/7"), Some(Shard { k: 2, n: 7 }));
        assert_eq!(parse(" 1 / 1 "), Some(Shard { k: 1, n: 1 }));
        for bad in ["0/3", "4/3", "3", "a/b", "", "1/0"] {
            assert_eq!(parse(bad), None, "{bad:?}");
        }
    }

    /// Every program in exactly one shard, every shard holding one, the long
    /// programs one to a shard before any shard takes a second.
    #[test]
    fn the_shards_partition_the_corpus() {
        let root = crate::repo_root();
        let programs: Vec<String> = crate::corpus_files()
            .iter()
            .map(|p| p.strip_prefix(&root).unwrap_or(p).display().to_string())
            .collect();
        for n in 1..=9 {
            let mut seen = vec![0usize; programs.len()];
            for k in 1..=n {
                let kept = keep_in(
                    (0..programs.len()).collect::<Vec<_>>(),
                    |&i| programs[i].clone(),
                    Shard { k, n },
                );
                assert!(!kept.is_empty(), "shard {k}/{n} holds no program");
                for i in kept {
                    seen[i] += 1;
                }
            }
            assert!(
                seen.iter().all(|&c| c == 1),
                "at {n} shards, a program is in no shard or in two"
            );
            let dealt = assign(&programs, n);
            let long: Vec<usize> = programs
                .iter()
                .zip(&dealt)
                .filter(|(p, _)| pool::LONG.iter().chain(pool::HEAVY).any(|l| p.ends_with(l)))
                .map(|(_, &s)| s)
                .collect();
            let mut per = vec![0usize; n + 1];
            for s in &long {
                per[*s] += 1;
            }
            let (most, least) = (
                per[1..].iter().max().copied().unwrap_or(0),
                per[1..].iter().min().copied().unwrap_or(0),
            );
            assert!(
                most - least <= 1,
                "at {n} shards the long programs are dealt {:?}",
                &per[1..]
            );
        }
    }

    #[test]
    fn a_whole_test_runs_once_among_the_shards() {
        // No shard set in a unit test run unless the caller set one; the
        // rule itself, for each k and n.
        for n in 1..=5 {
            for k in 1..=7 {
                let runs: Vec<usize> = (1..=n).filter(|&s| (k - 1) % n + 1 == s).collect();
                assert_eq!(runs.len(), 1, "{k} in {n}");
            }
        }
    }
}
