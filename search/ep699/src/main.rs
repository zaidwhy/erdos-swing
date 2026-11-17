//! Erdős problem 699: for 1 <= i < j <= n/2, is there a prime p >= i dividing
//! gcd(C(n,i), C(n,j))?  This searches for a triple (n, i, j) with no such prime.
//!
//! Kummer: p | C(n,j) iff some base-p digit of j exceeds the same digit of n.
//! So (n, i, j) is a counterexample iff j is digit-dominated by n in base p for
//! every prime p >= i dividing C(n,i).  Two facts make the search cheap:
//!   1. a prime q in (n-i, n] divides both binomials, so only i <= n - prevprime(n) matter;
//!   2. the largest prime P of C(n,i) leaves few dominated j (about (n/P) * i of them).
//!
//! Modes: `weak` is the problem as stated (p >= i).  `strict` asks for p > i, which is
//! known to fail at listed triples and is the positive control.  `brute` checks every
//! (n, i, j) directly and is the reference the fast path must reproduce.

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

const SEGMENT: u64 = 1 << 17;
/// Left overlap so the window n, n-1, .., n-i+1 and the previous prime are inside the sieve.
/// A prime gap larger than this aborts the run instead of silently skipping values of i.
const MARGIN: u64 = 4096;
const MAX_ODD_PRIMES: usize = 15;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Weak,
    Strict,
}

fn dominated(mut j: u64, mut n: u64, p: u64) -> bool {
    while j > 0 {
        if j % p > n % p {
            return false;
        }
        j /= p;
        n /= p;
    }
    true
}

fn valuation(mut m: u64, p: u64) -> u32 {
    let mut v = 0;
    while m % p == 0 {
        m /= p;
        v += 1;
    }
    v
}

fn base_primes(limit: u64) -> Vec<u64> {
    let limit = limit as usize;
    let mut composite = vec![false; limit + 1];
    let mut primes = Vec::new();
    for p in 2..=limit {
        if !composite[p] {
            primes.push(p as u64);
            let mut m = p * p;
            while m <= limit {
                composite[m] = true;
                m += p;
            }
        }
    }
    primes
}

/// Odd prime factors of every integer in [start, start + len).
struct Factored {
    start: u64,
    count: Vec<u8>,
    primes: Vec<[u64; MAX_ODD_PRIMES]>,
}

impl Factored {
    fn sieve(start: u64, len: usize, base: &[u64]) -> Self {
        let mut rest: Vec<u64> = (0..len as u64).map(|k| start + k).collect();
        let mut count = vec![0u8; len];
        let mut primes = vec![[0u64; MAX_ODD_PRIMES]; len];
        let end = start + len as u64;
        for r in rest.iter_mut() {
            if *r > 0 {
                *r >>= r.trailing_zeros();
            }
        }
        for &p in base.iter().skip(1) {
            if p * p >= end {
                break;
            }
            let mut m = start.div_ceil(p) * p;
            if m == 0 {
                m = p;
            }
            while m < end {
                let k = (m - start) as usize;
                while rest[k] % p == 0 {
                    rest[k] /= p;
                }
                primes[k][count[k] as usize] = p;
                count[k] += 1;
                m += p;
            }
        }
        for k in 0..len {
            if rest[k] > 1 {
                primes[k][count[k] as usize] = rest[k];
                count[k] += 1;
            }
        }
        Factored { start, count, primes }
    }

    fn odd_primes(&self, n: u64) -> &[u64] {
        let k = (n - self.start) as usize;
        &self.primes[k][..self.count[k] as usize]
    }

    fn is_prime(&self, n: u64) -> bool {
        n == 2 || (n % 2 == 1 && n > 2 && self.odd_primes(n) == [n])
    }
}

/// Calls `visit` on every j in [lo, hi] that is digit-dominated by n in base p.
fn for_dominated(n: u64, p: u64, lo: u64, hi: u64, visit: &mut dyn FnMut(u64)) {
    let mut digits = Vec::new();
    let mut m = n;
    while m > 0 {
        digits.push(m % p);
        m /= p;
    }
    fn descend(digits: &[u64], p: u64, prefix: u64, lo: u64, hi: u64, visit: &mut dyn FnMut(u64)) {
        let Some((&top, lower)) = digits.split_last() else {
            if prefix >= lo && prefix <= hi {
                visit(prefix);
            }
            return;
        };
        let scale = p.pow(lower.len() as u32);
        for d in 0..=top {
            let next = prefix + d * scale;
            if next > hi {
                break;
            }
            descend(lower, p, next, lo, hi, visit);
        }
    }
    descend(&digits, p, 0, lo, hi, visit);
}

fn inverse_mod(a: u64, p: u64) -> u64 {
    let (mut r0, mut r1) = (p as i128, (a % p) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    assert!(r0 == 1, "inverse of {a} mod {p} does not exist");
    t0.rem_euclid(p as i128) as u64
}

/// Calls `visit` on every j in [lo, hi] with j mod p <= n mod p for each given prime.
/// The primes are distinct and their product exceeds hi, so each residue tuple gives one j.
fn for_residue_tuples(n: u64, primes: &[u64], lo: u64, hi: u64, visit: &mut dyn FnMut(u64)) {
    fn extend(n: u64, primes: &[u64], x: u128, modulus: u128, lo: u64, hi: u64, visit: &mut dyn FnMut(u64)) {
        let Some((&p, rest)) = primes.split_first() else {
            if x >= lo as u128 && x <= hi as u128 {
                visit(x as u64);
            }
            return;
        };
        let inv = inverse_mod((modulus % p as u128) as u64, p) as u128;
        let x_mod = (x % p as u128) as u64;
        for t in 0..=n % p {
            let step = ((t + p - x_mod) % p) as u128 * inv % p as u128;
            extend(n, rest, x + modulus * step, modulus * p as u128, lo, hi, visit);
        }
    }
    extend(n, primes, 0, 1, lo, hi, visit);
}

struct Tally {
    composites_with_gap: u64,
    pairs: u64,
    candidates: u64,
    found: Vec<(u64, u64, u64)>,
}

fn scan_segment(lo: u64, hi: u64, base: &[u64], mode: Mode, tally: &mut Tally) {
    let start = lo.saturating_sub(MARGIN).max(2);
    let fac = Factored::sieve(start, (hi - start) as usize, base);
    let first_i: u64 = if mode == Mode::Weak { 3 } else { 2 };

    let mut last_prime = 0u64;
    let mut m = lo.max(2);
    while m >= start {
        if fac.is_prime(m) {
            last_prime = m;
            break;
        }
        if m == start {
            break;
        }
        m -= 1;
    }
    assert!(last_prime > 0 || lo <= 3, "no prime within MARGIN below {lo}");

    let mut window: Vec<(u64, u64)> = Vec::new();
    let mut divisors: Vec<u64> = Vec::new();
    for n in lo.max(2)..hi {
        if fac.is_prime(n) {
            last_prime = n;
            continue;
        }
        let gap = n - last_prime;
        assert!(gap < MARGIN, "prime gap {gap} at {n} exceeds MARGIN");
        if gap < first_i {
            continue;
        }
        tally.composites_with_gap += 1;
        window.clear();
        for k in 0..first_i - 1 {
            window.extend(fac.odd_primes(n - k).iter().map(|&p| (p, k)));
        }
        for i in first_i..=gap {
            if 2 * (i + 1) > n {
                break;
            }
            let k = i - 1;
            window.extend(fac.odd_primes(n - k).iter().map(|&p| (p, k)));
            divisors.clear();
            for &(p, at) in &window {
                let keep = match mode {
                    Mode::Weak => p > i || (p == i && valuation(n - at, p) >= 2),
                    Mode::Strict => p > i,
                };
                if keep {
                    divisors.push(p);
                }
            }
            tally.pairs += 1;
            let Some(&largest) = divisors.iter().max() else {
                // C(n,i) has no admissible prime at all: every j is a counterexample.
                tally.found.push((n, i, i + 1));
                continue;
            };
            // Primes above the gap stay in the divisor set for every later i, so if they
            // alone already exclude every j > i, no later i can produce a counterexample.
            divisors.sort_unstable_by(|a, b| b.cmp(a));
            let mut survives_large_primes = false;
            let mut hit = None;
            let mut seen = 0u64;
            let mut check = |j: u64| {
                seen += 1;
                let mut large_ok = true;
                let mut all_ok = true;
                for &p in &divisors {
                    if !dominated(j, n, p) {
                        all_ok = false;
                        if p > gap {
                            large_ok = false;
                            break;
                        }
                    }
                }
                survives_large_primes |= large_ok;
                if all_ok && hit.is_none() {
                    hit = Some(j);
                }
            };
            // j <= n/2 is fixed by its residues modulo enough of the largest primes, and each
            // residue is at most n mod p < i, so there are at most i^2 or so candidates.
            let mut used = 0;
            let mut modulus: u128 = 1;
            while used < divisors.len() && modulus <= (n / 2) as u128 {
                modulus *= divisors[used] as u128;
                used += 1;
            }
            let by_residues = modulus > (n / 2) as u128;
            let basis_is_large = if by_residues {
                divisors[..used].iter().all(|&p| p > gap)
            } else {
                largest > gap
            };
            if by_residues {
                for_residue_tuples(n, &divisors[..used], i + 1, n / 2, &mut check);
            } else {
                for_dominated(n, largest, i + 1, n / 2, &mut check);
            }
            if !basis_is_large {
                survives_large_primes = true;
            }
            tally.candidates += seen;
            if let Some(j) = hit {
                tally.found.push((n, i, j));
            }
            if largest > gap && !survives_large_primes {
                break;
            }
        }
    }
}

/// Reference: test every (n, i, j) against every prime, no pruning.
fn brute(limit: u64, mode: Mode) -> Vec<(u64, u64, u64)> {
    let primes = base_primes(limit);
    let mut found = Vec::new();
    for n in 2..=limit {
        for i in 1..=n / 2 {
            for j in i + 1..=n / 2 {
                let shared = primes.iter().take_while(|&&p| p <= n).any(|&p| {
                    let admissible = if mode == Mode::Weak { p >= i } else { p > i };
                    admissible && !dominated(i, n, p) && !dominated(j, n, p)
                });
                if !shared {
                    found.push((n, i, j));
                }
            }
        }
    }
    found
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: ep699 <weak|strict> scan <lo> <hi> <threads> <out.jsonl>\n       ep699 <weak|strict> brute <limit>");
        std::process::exit(2);
    }
    let mode = match args[1].as_str() {
        "weak" => Mode::Weak,
        "strict" => Mode::Strict,
        other => panic!("mode must be weak or strict, got {other}"),
    };
    if args[2] == "brute" {
        let limit: u64 = args[3].parse().expect("limit");
        for (n, i, j) in brute(limit, mode) {
            println!("{n} {i} {j}");
        }
        return;
    }
    let lo: u64 = args[3].parse().expect("lo");
    let hi: u64 = args[4].parse().expect("hi");
    let threads: usize = args[5].parse().expect("threads");
    let out_path = &args[6];
    assert!(hi < (1u64 << 62), "hi must fit the u64 arithmetic used here");

    let block = 100_000_000u64.min(((hi - lo) / 64).max(SEGMENT));
    let done: std::collections::HashSet<u64> = std::fs::read_to_string(out_path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.split("\"lo\":").nth(1)?.split(',').next()?.trim().parse().ok())
        .collect();
    let base = base_primes((hi as f64).sqrt() as u64 + 2);
    let out = Mutex::new(OpenOptions::new().create(true).append(true).open(out_path).expect("open out"));
    let next = AtomicU64::new(lo);
    let label = if mode == Mode::Weak { "weak" } else { "strict" };

    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let b_lo = next.fetch_add(block, Ordering::Relaxed);
                if b_lo >= hi {
                    break;
                }
                if done.contains(&b_lo) {
                    continue;
                }
                let b_hi = (b_lo + block).min(hi);
                let mut tally = Tally { composites_with_gap: 0, pairs: 0, candidates: 0, found: Vec::new() };
                let mut s = b_lo;
                while s < b_hi {
                    let e = (s + SEGMENT).min(b_hi);
                    scan_segment(s, e, &base, mode, &mut tally);
                    s = e;
                }
                let found: Vec<String> = tally.found.iter().map(|(n, i, j)| format!("[{n},{i},{j}]")).collect();
                let line = format!(
                    "{{\"lo\":{b_lo},\"hi\":{b_hi},\"mode\":\"{label}\",\"n_with_gap\":{},\"pairs\":{},\"candidates\":{},\"found\":[{}]}}\n",
                    tally.composites_with_gap, tally.pairs, tally.candidates, found.join(",")
                );
                out.lock().unwrap().write_all(line.as_bytes()).expect("write block");
            });
        }
    });
}
