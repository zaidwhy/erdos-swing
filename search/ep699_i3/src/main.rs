//! Erdős problem 699, the case i = 3, by structure instead of by sieving every n.
//!
//! Suppose (n, 3, j) is a counterexample with 3 < j < n/2.  Write n = s * a with s the
//! 6-smooth part of n, and let b, c be the parts of n - 1 and n - 2 coprime to 6.
//! Kummer's theorem applied to the primes >= 5 gives:
//!   a | j, so j = a * t with 1 <= t < s/2;
//!   b | t (s - t);
//!   c | t (s - t) (s - 2t).
//! b and c are coprime, so b * c <= max of t (s - t) (s - 2t) < 0.0963 s^3.  Only n with a
//! very large 6-smooth part pass that bound, and they can be listed directly: fix how the
//! powers of 2 and 3 sit on n, n - 1, n - 2, then walk the resulting arithmetic progression.
//!
//! `necessary <limit>` prints every (n, t) meeting the three divisibility conditions; a
//! brute-force script must produce the same list.  `scan <limit> <threads>` reports the
//! (n, j) that survive every prime >= 5, and which of those also survive the prime 3.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// max of x(1-x)(1-2x) on (0, 1/2) is 0.09623; the slack absorbs floating-point rounding.
const SIZE_BOUND: f64 = 0.0964;

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut acc = 1 % m;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mul_mod(acc, base, m);
        }
        base = mul_mod(base, base, m);
        exp >>= 1;
    }
    acc
}

/// Miller-Rabin with the first twelve primes as bases, which is exact for every u64.
fn is_prime(n: u64) -> bool {
    const BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    if n < 2 {
        return false;
    }
    for p in BASES {
        if n % p == 0 {
            return n == p;
        }
    }
    let r = (n - 1).trailing_zeros();
    let d = (n - 1) >> r;
    'bases: for a in BASES {
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..r {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// A nontrivial factor of an odd composite n, by Pollard's rho with Brent's cycle detection.
/// Differences are multiplied together and one gcd is taken per batch.  A batch can swallow
/// every prime of n at once (always, when the cycles are short), so that seed is replayed
/// with a gcd at every step before moving to the next seed.
fn rho_factor(n: u64) -> u64 {
    let attempt = |seed: u64, batch: u64| -> u64 {
        let step = |x: u64| (mul_mod(x, x, n) + seed) % n;
        let (mut tortoise, mut hare, mut limit, mut taken) = (2u64, 2u64, 1u64, 0u64);
        let mut product = 1u64;
        loop {
            if taken == limit {
                tortoise = hare;
                limit *= 2;
                taken = 0;
            }
            hare = step(hare);
            taken += 1;
            product = mul_mod(product, tortoise.abs_diff(hare), n);
            if taken % batch == 0 || taken == limit {
                let found = gcd(product, n);
                if found != 1 {
                    return found;
                }
                product = 1;
            }
        }
    };
    let mut seed = 1u64;
    loop {
        for batch in [64, 1] {
            let found = attempt(seed, batch);
            if found != n {
                return found;
            }
        }
        seed += 1;
    }
}

/// Prime-power factors (p, p^e) of m, for m coprime to 6 or otherwise.
fn prime_powers(mut m: u64, out: &mut Vec<(u64, u64)>) {
    out.clear();
    let mut stack = Vec::new();
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47] {
        if m % p == 0 {
            let mut q = 1;
            while m % p == 0 {
                m /= p;
                q *= p;
            }
            out.push((p, q));
        }
    }
    if m > 1 {
        stack.push(m);
    }
    let mut primes = Vec::new();
    while let Some(x) = stack.pop() {
        if is_prime(x) {
            primes.push(x);
        } else {
            let f = rho_factor(x);
            stack.push(f);
            stack.push(x / f);
        }
    }
    primes.sort_unstable();
    primes.dedup();
    for p in primes {
        let mut q = 1;
        while m % p == 0 {
            m /= p;
            q *= p;
        }
        out.push((p, q));
    }
}

fn inverse_mod(a: u64, m: u64) -> u64 {
    let (mut r0, mut r1) = (m as i128, (a % m) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    assert!(r0 == 1, "{a} has no inverse mod {m}");
    t0.rem_euclid(m as i128) as u64
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

fn smooth_part(mut m: u64) -> u64 {
    let mut s = 1;
    for p in [2u64, 3] {
        while m % p == 0 {
            m /= p;
            s *= p;
        }
    }
    s
}

/// Every t in [1, s/2) with b | t(s-t) and c | t(s-t)(s-2t), given the prime powers of b and c.
/// Each prime power q of b needs t = 0 or s (mod q); each of c needs t = 0, s or s/2 (mod q).
fn solve_t(s: u64, b_powers: &[(u64, u64)], c_powers: &[(u64, u64)], visit: &mut dyn FnMut(u64)) {
    let mut choices: Vec<(u64, [u64; 3], usize)> = Vec::new();
    for &(_, q) in b_powers {
        choices.push((q, [0, s % q, 0], 2));
    }
    for &(_, q) in c_powers {
        let half = mul_mod(s % q, inverse_mod(2, q), q);
        choices.push((q, [0, s % q, half], 3));
    }
    choices.sort_unstable_by(|x, y| y.0.cmp(&x.0));
    let bound = (s - 1) / 2;
    fn extend(
        choices: &[(u64, [u64; 3], usize)],
        x: u128,
        modulus: u128,
        bound: u64,
        visit: &mut dyn FnMut(u64),
    ) {
        if modulus > bound as u128 {
            // t is now fixed; the remaining prime powers only have to agree with it.
            if x == 0 || x > bound as u128 {
                return;
            }
            let t = x as u64;
            if choices.iter().all(|(q, residues, count)| residues[..*count].contains(&(t % q))) {
                visit(t);
            }
            return;
        }
        let Some(((q, residues, count), rest)) = choices.split_first() else {
            let mut t = if x == 0 { modulus } else { x };
            while t <= bound as u128 {
                visit(t as u64);
                t += modulus;
            }
            return;
        };
        let inv = inverse_mod((modulus % *q as u128) as u64, *q) as u128;
        let x_mod = (x % *q as u128) as u64;
        let mut seen = [u64::MAX; 3];
        for (slot, &r) in residues[..*count].iter().enumerate() {
            if seen[..slot].contains(&r) {
                continue;
            }
            seen[slot] = r;
            let lift = ((r + q - x_mod) % q) as u128 * inv % *q as u128;
            extend(rest, x + modulus * lift, modulus * *q as u128, bound, visit);
        }
    }
    extend(&choices, 0, 1, bound, visit);
}

#[derive(Default)]
struct Tally {
    tested: u64,
    passed_size_bound: u64,
    necessary: Vec<(u64, u64)>,
    survive_large_primes: Vec<(u64, u64)>,
    counterexamples: Vec<(u64, u64)>,
}

fn examine(n: u64, tally: &mut Tally, scratch: &mut [Vec<(u64, u64)>; 3]) {
    tally.tested += 1;
    let s = smooth_part(n);
    if s < 3 {
        return;
    }
    let a = n / s;
    let b = (n - 1) / smooth_part(n - 1);
    let c = (n - 2) / smooth_part(n - 2);
    if (b as f64) * (c as f64) > SIZE_BOUND * (s as f64).powi(3) {
        return;
    }
    tally.passed_size_bound += 1;
    let [b_powers, c_powers, a_powers] = scratch;
    prime_powers(b, b_powers);
    prime_powers(c, c_powers);
    if b_powers.iter().chain(c_powers.iter()).any(|&(_, q)| q >= s) {
        return;
    }
    let mut solutions = Vec::new();
    solve_t(s, b_powers, c_powers, &mut |t| solutions.push(t));
    if solutions.is_empty() {
        return;
    }
    prime_powers(a, a_powers);
    let three_divides = (valuation3(n) + valuation3(n - 1) + valuation3(n - 2)) >= 2;
    for t in solutions {
        tally.necessary.push((n, t));
        let j = a * t;
        if j < 4 || 2 * j >= n {
            continue;
        }
        let large_ok = a_powers
            .iter()
            .chain(b_powers.iter())
            .chain(c_powers.iter())
            .all(|&(p, _)| dominated(j, n, p));
        if large_ok {
            tally.survive_large_primes.push((n, j));
            if !three_divides || dominated(j, n, 3) {
                tally.counterexamples.push((n, j));
            }
        }
    }
}

fn valuation3(mut m: u64) -> u32 {
    let mut v = 0;
    while m % 3 == 0 {
        m /= 3;
        v += 1;
    }
    v
}

/// One arithmetic progression of candidates: n = residue (mod modulus), n <= top.
struct Progression {
    residue: u128,
    modulus: u128,
    top: u64,
}

/// All ways the powers of 2 and 3 can sit on n, n-1, n-2, each with its size bound on n.
fn progressions(limit: u64) -> Vec<Progression> {
    let mut out = Vec::new();
    let mut two_cases = Vec::new(); // (offset of the number carrying 2^e, e)
    let mut e = 1u32;
    while (1u128 << e) <= 2 * limit as u128 {
        two_cases.push((1u64, e));
        if e >= 2 {
            two_cases.push((0, e));
            two_cases.push((2, e));
        }
        e += 1;
    }
    let mut three_cases = Vec::new();
    let mut e = 1u32;
    while 3u128.pow(e) <= 2 * limit as u128 {
        for offset in [0u64, 1, 2] {
            three_cases.push((offset, e));
        }
        e += 1;
    }
    for &(two_at, two_e) in &two_cases {
        for &(three_at, three_e) in &three_cases {
            let p2 = 1u128 << two_e;
            let p3 = 3u128.pow(three_e);
            // Smooth parts of n, n-1, n-2 implied by this placement.
            let mut parts = [1f64; 3];
            parts[two_at as usize] *= p2 as f64;
            if two_at == 0 {
                parts[2] *= 2.0;
            }
            if two_at == 2 {
                parts[0] *= 2.0;
            }
            parts[three_at as usize] *= p3 as f64;
            let top = (SIZE_BOUND * parts[0].powi(3) * parts[1] * parts[2]).sqrt() * (1.0 + 1e-9) + 4.0;
            let top = if top >= limit as f64 { limit } else { top as u64 };
            // n = two_at (mod 2^e) and n = three_at (mod 3^e).
            let inv = {
                let (mut r0, mut r1) = (p3 as i128, (p2 % p3) as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(p3 as i128) as u128
            };
            let diff = (three_at as u128 + p3 - (two_at as u128 % p3)) % p3;
            let residue = two_at as u128 + p2 * (diff * inv % p3);
            if residue <= top as u128 {
                out.push(Progression { residue, modulus: p2 * p3, top });
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: ep699_i3 necessary <limit> | scan <limit> <threads>");
        std::process::exit(2);
    }
    let limit: u64 = args[2].parse().expect("limit");
    assert!(limit <= 4_000_000_000_000_000_000, "limit must leave headroom in u64");
    let threads: usize = args.get(3).map(|t| t.parse().expect("threads")).unwrap_or(1);
    let work = progressions(limit);
    let next = AtomicUsize::new(0);
    let total = Mutex::new(Tally::default());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                let mut tally = Tally::default();
                let mut scratch = [Vec::new(), Vec::new(), Vec::new()];
                loop {
                    let k = next.fetch_add(1, Ordering::Relaxed);
                    let Some(prog) = work.get(k) else { break };
                    let mut n = prog.residue;
                    while n <= prog.top as u128 {
                        if n >= 9 {
                            examine(n as u64, &mut tally, &mut scratch);
                        }
                        n += prog.modulus;
                    }
                }
                let mut all = total.lock().unwrap();
                all.tested += tally.tested;
                all.passed_size_bound += tally.passed_size_bound;
                all.necessary.extend(tally.necessary);
                all.survive_large_primes.extend(tally.survive_large_primes);
                all.counterexamples.extend(tally.counterexamples);
            });
        }
    });
    let mut all = total.into_inner().unwrap();
    all.necessary.sort_unstable();
    all.necessary.dedup();
    all.survive_large_primes.sort_unstable();
    all.survive_large_primes.dedup();
    all.counterexamples.sort_unstable();
    all.counterexamples.dedup();
    if args[1] == "necessary" {
        for (n, t) in &all.necessary {
            println!("{n} {t}");
        }
        return;
    }
    println!(
        "{{\"limit\":{limit},\"progressions\":{},\"tested\":{},\"passed_size_bound\":{},\"necessary_pairs\":{},\"survive_primes_ge_5\":{:?},\"counterexamples\":{:?}}}",
        work.len(),
        all.tested,
        all.passed_size_bound,
        all.necessary.len(),
        all.survive_large_primes,
        all.counterexamples
    );
}
