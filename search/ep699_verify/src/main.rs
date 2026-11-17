// Independent enumeration of necessary pairs (n, t) and survivors.
//
// Usage: ep699_verify <P as 2,3,5> <LIMIT> <threads> <out_necessary> <out_survivors>
//
// Candidate n are enumerated by the exact P-valuation pattern of (n, n-1, n-2):
// each prime of P is assigned a state (exponents in n, n-1, n-2), the states fix a
// residue class of n by CRT, and for a fixed pattern (s, u, v) of smooth parts the
// inequality (n-2)^2 <= (n-1)(n-2) <= K s^3 u v bounds n from above. Every n has
// exactly one pattern, so the enumeration is complete and has no duplicates.
// For each candidate, t is found by enumerating the unitary splits
// b*c = A*B*C with A | t, B | s-t, C | s-2t and solving the CRT system.

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

// max of x(1-x)(1-2x) on (0, 1/2) is 1/(6 sqrt 3) = 0.0962250...; the margin only
// adds candidates.
const KMAX: f64 = 0.09623 * 1.000001;
const CHUNK: u64 = 2048;

// ---------------------------------------------------------------- arithmetic

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

// inverse of a mod m (gcd must be 1); m >= 1
fn inv_mod(a: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let (mut r0, mut r1) = (m as i128, (a % m) as i128);
    let (mut s0, mut s1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        let r2 = r0 - q * r1;
        r0 = r1;
        r1 = r2;
        let s2 = s0 - q * s1;
        s0 = s1;
        s1 = s2;
    }
    assert!(r0 == 1, "inv_mod: not coprime a={} m={}", a, m);
    let mut x = s0 % m as i128;
    if x < 0 {
        x += m as i128;
    }
    x as u64
}

struct Mont {
    n: u64,
    nneg: u64, // -n^{-1} mod 2^64
    r2: u64,   // R^2 mod n
}

impl Mont {
    // n odd, n < 2^63
    fn new(n: u64) -> Mont {
        let mut inv: u64 = n;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(n.wrapping_mul(inv)));
        }
        debug_assert!(n.wrapping_mul(inv) == 1);
        let r = ((1u128 << 64) % n as u128) as u64;
        let r2 = mulmod(r, r, n);
        Mont { n, nneg: inv.wrapping_neg(), r2 }
    }
    #[inline]
    fn redc(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.nneg);
        let sum = t + (m as u128) * (self.n as u128);
        let res = (sum >> 64) as u64;
        if res >= self.n {
            res - self.n
        } else {
            res
        }
    }
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.redc(a as u128 * b as u128)
    }
    fn to(&self, a: u64) -> u64 {
        self.mul(a % self.n, self.r2)
    }
    fn pow(&self, mut b: u64, mut e: u64) -> u64 {
        let mut r = self.to(1);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
}

const SMALLP: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in SMALLP.iter() {
        if n % p == 0 {
            return n == p;
        }
    }
    if n < 37 * 37 {
        return true;
    }
    assert!(n < (1u64 << 63));
    let m = Mont::new(n);
    let one = m.to(1);
    let mone = m.to(n - 1);
    let mut d = n - 1;
    let r = d.trailing_zeros();
    d >>= r;
    'outer: for &a in SMALLP.iter() {
        let mut x = m.pow(m.to(a), d);
        if x == one || x == mone {
            continue;
        }
        for _ in 1..r {
            x = m.mul(x, x);
            if x == mone {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while (r as u128) * (r as u128) > n as u128 {
        r -= 1;
    }
    while ((r + 1) as u128) * ((r + 1) as u128) <= n as u128 {
        r += 1;
    }
    r
}

fn icbrt(n: u64) -> u64 {
    let mut r = (n as f64).cbrt() as u64;
    while r > 0 && (r as u128).pow(3) > n as u128 {
        r -= 1;
    }
    while ((r + 1) as u128).pow(3) <= n as u128 {
        r += 1;
    }
    r
}

// n odd composite, no prime factor below 1000, not a square or cube.
fn rho(n: u64) -> u64 {
    let m = Mont::new(n);
    for c0 in 1..64u64 {
        let c = m.to(c0);
        let f = |x: u64| -> u64 {
            let y = m.mul(x, x) + c; // both < n < 2^63
            if y >= n {
                y - n
            } else {
                y
            }
        };
        let mut y = m.to(2 + c0);
        let mut x = y;
        let mut ys = y;
        let mut q = m.to(1);
        let mut g: u64 = 1;
        let mut r: u64 = 1;
        let blk: u64 = 128;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0;
            while k < r && g == 1 {
                ys = y;
                let lim = blk.min(r - k);
                for _ in 0..lim {
                    y = f(y);
                    let d = if x > y { x - y } else { y - x };
                    q = m.mul(q, d);
                }
                g = gcd(q, n);
                k += lim;
            }
            r <<= 1;
            if r > (1u64 << 24) {
                break;
            }
        }
        if g == n || g == 0 {
            g = 1;
            let mut cnt: u64 = 0;
            while g == 1 {
                ys = f(ys);
                let d = if x > ys { x - ys } else { ys - x };
                g = gcd(d, n);
                cnt += 1;
                if cnt > (1u64 << 25) {
                    break;
                }
            }
        }
        if g != 1 && g != n && g != 0 {
            return g;
        }
    }
    // Fallback: plain trial division (never expected to run).
    eprintln!("WARNING: rho fallback on {}", n);
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            return d;
        }
        d += 2;
    }
    panic!("rho fallback found no factor of {}", n);
}

fn small_primes(limit: u64) -> Vec<u64> {
    let mut v = Vec::new();
    for k in 2..=limit {
        if v.iter().take_while(|&&p| p * p <= k).all(|&p| k % p != 0) {
            v.push(k);
        }
    }
    v
}

fn factor_rec(m: u64, out: &mut Vec<u64>) {
    if m == 1 {
        return;
    }
    if is_prime(m) {
        out.push(m);
        return;
    }
    let r = isqrt(m);
    if r * r == m {
        factor_rec(r, out);
        factor_rec(r, out);
        return;
    }
    let c = icbrt(m);
    if c * c * c == m {
        factor_rec(c, out);
        factor_rec(c, out);
        factor_rec(c, out);
        return;
    }
    let d = rho(m);
    factor_rec(d, out);
    factor_rec(m / d, out);
}

// returns sorted (prime, exponent)
fn factor(mut m: u64, tp: &[u64]) -> Vec<(u64, u32)> {
    let mut ps: Vec<u64> = Vec::new();
    for &p in tp {
        if p * p > m {
            break;
        }
        while m % p == 0 {
            m /= p;
            ps.push(p);
        }
    }
    if m > 1 {
        if m < 1_000_000 || is_prime(m) {
            // below 1000^2 with no factor < 1000 means prime
            ps.push(m);
        } else {
            factor_rec(m, &mut ps);
        }
    }
    ps.sort_unstable();
    let mut res: Vec<(u64, u32)> = Vec::new();
    for p in ps {
        if let Some(last) = res.last_mut() {
            if last.0 == p {
                last.1 += 1;
                continue;
            }
        }
        res.push((p, 1));
    }
    res
}

fn ppow(f: &(u64, u32)) -> u64 {
    f.0.pow(f.1)
}

// ---------------------------------------------------------------- t search

// Unitary splits: each prime power of b goes to A (| t) or B (| s-t); each prime
// power of c goes to A, B or C (| s-2t). A < s/2, B < s, C < s prune.
struct Split<'a> {
    s: u64,
    items: &'a [(u64, bool)], // (q, from_c)
    out: &'a mut Vec<u64>,
}

fn split_dfs(sp: &mut Split, i: usize, a: u64, b: u64, c: u64) {
    if i == sp.items.len() {
        split_leaf(sp, a, b, c);
        return;
    }
    let (q, from_c) = sp.items[i];
    let s = sp.s as u128;
    let na = a as u128 * q as u128;
    if 2 * na < s {
        split_dfs(sp, i + 1, na as u64, b, c);
    }
    let nb = b as u128 * q as u128;
    if nb < s {
        split_dfs(sp, i + 1, a, nb as u64, c);
    }
    if from_c {
        let nc = c as u128 * q as u128;
        if nc < s {
            split_dfs(sp, i + 1, a, b, nc as u64);
        }
    }
}

fn split_leaf(sp: &mut Split, a: u64, b: u64, c: u64) {
    let s = sp.s;
    // t = a*x, a*x == s (mod b)
    let x0 = if b == 1 { 0 } else { mulmod(s % b, inv_mod(a % b, b), b) };
    let t1: u128 = a as u128 * x0 as u128;
    let m1: u128 = a as u128 * b as u128;
    // t = t1 + m1*y, 2t == s (mod c)
    let y = if c == 1 {
        0u64
    } else {
        let t1c = (t1 % c as u128) as u64;
        let m1c = (m1 % c as u128) as u64;
        let rhs = ((s % c) as u128 + 2 * c as u128 - 2 * t1c as u128) % c as u128;
        let inv = inv_mod(((2 * m1c as u128) % c as u128) as u64, c);
        mulmod(rhs as u64, inv, c)
    };
    let m: u128 = m1 * c as u128;
    let t0: u128 = (t1 + m1 * y as u128) % m;
    let tmax = ((s - 1) / 2) as u128;
    let mut t = if t0 == 0 { m } else { t0 };
    while t <= tmax {
        sp.out.push(t as u64);
        t += m;
    }
}

fn prod_mod(xs: &[u128], m: u64) -> u64 {
    let mut r = 1 % m;
    for &x in xs {
        r = mulmod(r, (x % m as u128) as u64, m);
    }
    r
}

fn digits_ok(j: u64, n: u64, p: u64) -> bool {
    let (mut j, mut n) = (j, n);
    while j > 0 {
        if j % p > n % p {
            return false;
        }
        j /= p;
        n /= p;
    }
    true
}

// ---------------------------------------------------------------- per n

fn smooth_split(mut m: u64, ps: &[u64]) -> (u64, u64) {
    let mut s = 1u64;
    for &p in ps {
        if p == 2 {
            let z = m.trailing_zeros();
            m >>= z;
            s <<= z;
        } else {
            while m % p == 0 {
                m /= p;
                s *= p;
            }
        }
    }
    (s, m)
}

struct Out {
    pairs: Vec<(u64, u64)>,
    surv: Vec<(u64, u64)>,
    cands: u64,
    tested_n: u64,
}

fn passes_ineq(n: u64, s: u64, u: u64, v: u64) -> bool {
    let lhs = (n - 1) as f64 * (n - 2) as f64;
    let sf = s as f64;
    let rhs = KMAX * sf * sf * sf * u as f64 * v as f64;
    lhs <= rhs * (1.0 + 1e-9)
}

fn process_n(n: u64, ps: &[u64], tp: &[u64], out: &mut Out) {
    let (s, a) = smooth_split(n, ps);
    let (_u, b) = smooth_split(n - 1, ps);
    let (_v, c) = smooth_split(n - 2, ps);
    if s < 3 {
        return;
    }
    out.cands += 1;
    let fb = factor(b, tp);
    let fc = factor(c, tp);
    let mut items: Vec<(u64, bool)> = Vec::with_capacity(fb.len() + fc.len());
    for f in &fb {
        items.push((ppow(f), false));
    }
    for f in &fc {
        items.push((ppow(f), true));
    }
    items.sort_unstable_by(|x, y| y.0.cmp(&x.0));
    let mut ts: Vec<u64> = Vec::new();
    {
        let mut sp = Split { s, items: &items, out: &mut ts };
        split_dfs(&mut sp, 0, 1, 1, 1);
    }
    if ts.is_empty() {
        return;
    }
    ts.sort_unstable();
    ts.dedup();
    let mut fa: Option<Vec<(u64, u32)>> = None;
    for &t in &ts {
        // direct check of the definition
        assert!(t >= 1 && 2 * t < s);
        let x1 = t as u128;
        let x2 = (s - t) as u128;
        let x3 = (s - 2 * t) as u128;
        let okb = (x1 * x2) % b as u128 == 0;
        let okc = prod_mod(&[x1, x2, x3], c) == 0;
        assert!(okb && okc, "split produced a bad t: n={} t={}", n, t);
        out.pairs.push((n, t));
        // survivor test
        let j = a as u128 * t as u128;
        if j >= 4 && 2 * j < n as u128 {
            let j = j as u64;
            if fa.is_none() {
                fa = Some(factor(a, tp));
            }
            let mut ok = true;
            for f in fa.as_ref().unwrap().iter().chain(fb.iter()).chain(fc.iter()) {
                if !digits_ok(j, n, f.0) {
                    ok = false;
                    break;
                }
            }
            if ok {
                out.surv.push((n, j));
            }
        }
    }
}

// ---------------------------------------------------------------- pattern DFS

#[derive(Clone, Copy)]
struct State {
    e: [u32; 3],
    pe: [u64; 3], // p^e in n, n-1, n-2
    modulus: u64,
    residue: u64,
}

fn states_for(p: u64, limit: u64) -> Vec<State> {
    let mut v = Vec::new();
    let mut pows = vec![1u64];
    while let Some(&last) = pows.last() {
        if (last as u128) * (p as u128) > limit as u128 {
            break;
        }
        pows.push(last * p);
    }
    let maxe = pows.len() - 1;
    if p == 2 {
        // n odd: 2^e || n-1, e >= 1
        for e in 1..=maxe {
            v.push(State { e: [0, e as u32, 0], pe: [1, pows[e], 1], modulus: pows[e], residue: 1 });
        }
        // n even, 2^e || n with e >= 2, then 2 || n-2
        for e in 2..=maxe {
            v.push(State { e: [e as u32, 0, 1], pe: [pows[e], 1, 2], modulus: pows[e], residue: 0 });
        }
        // 2 || n, 2^f || n-2 with f >= 2
        for f in 2..=maxe {
            v.push(State { e: [1, 0, f as u32], pe: [2, 1, pows[f]], modulus: pows[f], residue: 2 % pows[f] });
        }
    } else {
        v.push(State { e: [0, 0, 0], pe: [1, 1, 1], modulus: 1, residue: 0 });
        for slot in 0..3 {
            for e in 1..=maxe {
                let mut ee = [0u32; 3];
                let mut pe = [1u64; 3];
                ee[slot] = e as u32;
                pe[slot] = pows[e];
                v.push(State { e: ee, pe, modulus: pows[e], residue: (slot as u64) % pows[e] });
            }
        }
    }
    v
}

fn vp(mut m: u64, p: u64) -> u32 {
    let mut e = 0;
    while m % p == 0 {
        m /= p;
        e += 1;
    }
    e
}

struct Search<'a> {
    ps: &'a [u64],
    tp: &'a [u64],
    states: Vec<Vec<State>>,
    limit: u64,
    tid: u64,
    nthreads: u64,
    leaf_id: u64,
    stack: Vec<State>,
    out: Out,
}

fn dfs(se: &mut Search, i: usize, r: u128, m: u128, s: u64, u: u64, v: u64) {
    let limit = se.limit;
    if i == se.ps.len() {
        leaf(se, r as u64, m as u64, s, u, v);
        return;
    }
    let nst = se.states[i].len();
    for k in 0..nst {
        let st = se.states[i][k];
        let ns = s as u128 * st.pe[0] as u128;
        let nu = u as u128 * st.pe[1] as u128;
        let nv = v as u128 * st.pe[2] as u128;
        // s <= n, u <= n-1, v <= n-2, all <= LIMIT
        if ns > limit as u128 || nu > limit as u128 || nv > limit as u128 {
            continue;
        }
        let (r2, m2) = if st.modulus == 1 {
            (r, m)
        } else {
            let mi = st.modulus;
            let rm = (r % mi as u128) as u64;
            let diff = (st.residue as u128 + mi as u128 - rm as u128) % mi as u128;
            let inv = inv_mod((m % mi as u128) as u64, mi);
            let kk = mulmod(diff as u64, inv, mi);
            (r + m * kk as u128, m * mi as u128)
        };
        se.stack.push(st);
        if m2 > limit as u128 {
            // at most one n <= LIMIT in this class
            if r2 <= limit as u128 {
                single(se, r2 as u64, i);
            }
        } else {
            dfs(se, i + 1, r2, m2, ns as u64, nu as u64, nv as u64);
        }
        se.stack.pop();
    }
}

// Exactly one n in range; the prime-states 0..=i are fixed by the branch, the
// rest are whatever n actually has.
fn single(se: &mut Search, n: u64, i: usize) {
    let id = se.leaf_id;
    se.leaf_id += 1;
    if id % se.nthreads != se.tid || n < 9 {
        return;
    }
    for j in 0..=i {
        let p = se.ps[j];
        let st = se.stack[j];
        if vp(n, p) != st.e[0] || vp(n - 1, p) != st.e[1] || vp(n - 2, p) != st.e[2] {
            return;
        }
    }
    let (s, _) = smooth_split(n, se.ps);
    let (u, _) = smooth_split(n - 1, se.ps);
    let (v, _) = smooth_split(n - 2, se.ps);
    se.out.tested_n += 1;
    if s >= 3 && passes_ineq(n, s, u, v) {
        process_n(n, se.ps, se.tp, &mut se.out);
    }
}

fn leaf(se: &mut Search, r: u64, m: u64, s: u64, u: u64, v: u64) {
    let id = se.leaf_id;
    se.leaf_id += 1;
    if s < 3 {
        return;
    }
    let limit = se.limit;
    let sf = s as f64;
    let bound = 2.0 + (KMAX * sf * sf * sf * u as f64 * v as f64).sqrt();
    let nmax: u64 = if bound * (1.0 + 1e-9) + 4.0 >= limit as f64 {
        limit
    } else {
        (bound * (1.0 + 1e-9)) as u64 + 4
    };
    let nmax = nmax.min(limit);
    // n >= max(9, s, u+1, v+2); r is the least residue
    let lo = 9u64.max(s).max(u + 1).max(v + 2);
    let n0 = if r >= lo { r } else { r + ((lo - r + m - 1) / m) * m };
    if n0 > nmax {
        return;
    }
    let cnt = (nmax - n0) / m + 1;
    let nchunks = (cnt + CHUNK - 1) / CHUNK;
    for ci in 0..nchunks {
        if (id + ci) % se.nthreads != se.tid {
            continue;
        }
        let kstart = ci * CHUNK;
        let kend = (kstart + CHUNK).min(cnt);
        for k in kstart..kend {
            let n = n0 + k * m;
            let (sa, _) = smooth_split(n, se.ps);
            if sa != s {
                continue;
            }
            let (ua, _) = smooth_split(n - 1, se.ps);
            if ua != u {
                continue;
            }
            let (va, _) = smooth_split(n - 2, se.ps);
            if va != v {
                continue;
            }
            se.out.tested_n += 1;
            if passes_ineq(n, s, u, v) {
                process_n(n, se.ps, se.tp, &mut se.out);
            }
        }
    }
}

fn parse_limit(x: &str) -> u64 {
    if let Some(pos) = x.find(['e', 'E']) {
        let mant: u64 = x[..pos].parse().expect("mantissa");
        let ex: u32 = x[pos + 1..].parse().expect("exponent");
        mant.checked_mul(10u64.pow(ex)).expect("limit overflow")
    } else {
        x.parse().expect("limit")
    }
}

// Literal check of the definition for every n <= limit (no inequality, no
// factoring): only the necessary-pair list, used as a larger-range cross-check.
fn brute(ps: &[u64], limit: u64, nthreads: u64, path: &str) {
    let mut handles = Vec::new();
    for tid in 0..nthreads {
        let ps = ps.to_vec();
        handles.push(thread::spawn(move || {
            let mut v = Vec::new();
            let mut n = 9 + tid;
            while n <= limit {
                let (s, _) = smooth_split(n, &ps);
                let (_, b) = smooth_split(n - 1, &ps);
                let (_, c) = smooth_split(n - 2, &ps);
                let mut t = 1u64;
                while 2 * t < s {
                    let x = t as u128 * (s - t) as u128;
                    if x % b as u128 == 0 && (x % c as u128) * ((s - 2 * t) as u128 % c as u128) % c as u128 == 0 {
                        v.push((n, t));
                    }
                    t += 1;
                }
                n += nthreads;
            }
            v
        }));
    }
    let mut all: Vec<(u64, u64)> = Vec::new();
    for h in handles {
        all.extend(h.join().unwrap());
    }
    all.sort_unstable();
    let mut f = BufWriter::new(File::create(path).unwrap());
    for (n, t) in &all {
        writeln!(f, "{} {}", n, t).unwrap();
    }
    eprintln!("brute P={:?} LIMIT={} necessary={}", ps, limit, all.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 6 && args[1] == "brute" {
        let ps: Vec<u64> = args[2].split(',').map(|x| x.trim().parse().unwrap()).collect();
        brute(&ps, parse_limit(&args[3]), args[4].parse().unwrap(), &args[5]);
        return;
    }
    if args.len() < 6 {
        eprintln!("usage: {} <P e.g. 2,3,5> <LIMIT> <threads> <out_necessary> <out_survivors>", args[0]);
        std::process::exit(2);
    }
    let mut ps: Vec<u64> = args[1].split(',').map(|x| x.trim().parse().unwrap()).collect();
    ps.sort_unstable();
    ps.dedup();
    assert!(ps.contains(&2) && ps.contains(&3));
    let limit = parse_limit(&args[2]);
    assert!(limit < (1u64 << 62));
    let nthreads: u64 = args[3].parse().unwrap();
    let tp = small_primes(1000);
    let t0 = Instant::now();

    let results: Arc<Mutex<Vec<Out>>> = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for tid in 0..nthreads {
        let ps = ps.clone();
        let tp = tp.clone();
        let results = results.clone();
        handles.push(thread::spawn(move || {
            let states: Vec<Vec<State>> = ps.iter().map(|&p| states_for(p, limit)).collect();
            let mut se = Search {
                ps: &ps,
                tp: &tp,
                states,
                limit,
                tid,
                nthreads,
                leaf_id: 0,
                stack: Vec::new(),
                out: Out { pairs: Vec::new(), surv: Vec::new(), cands: 0, tested_n: 0 },
            };
            dfs(&mut se, 0, 0, 1, 1, 1, 1);
            let leaves = se.leaf_id;
            results.lock().unwrap().push(se.out);
            leaves
        }));
    }
    let mut leaves = 0;
    for h in handles {
        leaves = h.join().unwrap();
    }
    let outs = std::mem::take(&mut *results.lock().unwrap());
    let mut pairs: Vec<(u64, u64)> = Vec::new();
    let mut surv: Vec<(u64, u64)> = Vec::new();
    let (mut cands, mut tested) = (0u64, 0u64);
    for o in outs {
        pairs.extend(o.pairs);
        surv.extend(o.surv);
        cands += o.cands;
        tested += o.tested_n;
    }
    pairs.sort_unstable();
    let before = pairs.len();
    pairs.dedup();
    assert!(before == pairs.len(), "duplicate pairs produced");
    surv.sort_unstable();
    surv.dedup();
    let mut f = BufWriter::new(File::create(&args[4]).unwrap());
    for (n, t) in &pairs {
        writeln!(f, "{} {}", n, t).unwrap();
    }
    let mut g = BufWriter::new(File::create(&args[5]).unwrap());
    for (n, j) in &surv {
        writeln!(g, "{} {}", n, j).unwrap();
    }
    eprintln!(
        "P={:?} LIMIT={} threads={} leaves={} n_exact_pattern={} n_passing_ineq={} necessary={} survivors={} distinct_n={} wall={:.2}s",
        ps,
        limit,
        nthreads,
        leaves,
        tested,
        cands,
        pairs.len(),
        surv.len(),
        {
            let mut ns: Vec<u64> = pairs.iter().map(|x| x.0).collect();
            ns.dedup();
            ns.len()
        },
        t0.elapsed().as_secs_f64()
    );
}
