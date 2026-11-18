# erdos-swing

An attempt on the open Erdős problems that a finite computation could settle.

**Thesis.** A problem is only worth a search if (a) one example or counterexample would settle it and
(b) nobody has already run that search. Every search here ships with a control that could have failed,
and nothing is called a result until a second, independently written path agrees.

**Status: no open problem resolved, and the headline #699 sweep is not a record.** A prior-work check
run after the searches (2026-11-18, section "Prior work on 699") found a public sweep to 1e12 and an
i = 3 search to 1e19, both from October 2026. My screening missed them. What exists is below, with the
limits stated.

## What was done

| Problem | What ran | Outcome |
|---|---|---|
| all 628 not-yet-settled problems | read every statement and the discussion threads of the candidates | see `notes/TRIAGE.md`; the cheap searches have been run by others |
| [#699](https://www.erdosproblems.com/699) | exact search for a counterexample, all n below 1e11 | no counterexample for 2 <= n < 1e11 (all 1000 blocks done; 87,870,299,149 (n, i) pairs tested). Superseded: Paul J. Lee's sweep covers n < 1e12 |
| #699, small i | structured enumeration of every n that can fail; two independent programs | no counterexample with j < n/2 for i <= 4 (n <= 1e18), i <= 6 (n <= 1e17), i <= 10 (n <= 1e15); `notes/EP699-I3.md` |
| [#273](https://www.erdosproblems.com/273) | 14 moduli p - 1 that cover every even integer, checked exactly | half a covering; the odd half is the open part |
| #273 | reduction to three coupled coverings, exact search on the pieces | one piece is easy (9 moduli), the pieces together stay open; `notes/EP273.md` |

## Problem 699

Statement: for 1 <= i < j <= n/2 some prime p >= i divides gcd(C(n,i), C(n,j)).

When this started I believed the public check stood at n <= 1e7 (Cong Lu). That was out of date; see
"Prior work on 699" below. `search/ep699` avoids testing most (n, i, j) at all:

1. By Kummer's theorem, p divides C(n,j) exactly when some base-p digit of j exceeds that digit of n.
   So (n, i, j) is a counterexample exactly when j is digit-dominated by n in base p for every prime
   p >= i dividing C(n,i).
2. A prime in (n - i, n] divides both binomials, so only i <= n - prevprime(n) can fail.
3. Each prime p > i dividing C(n,i) has n mod p < i, which forces j mod p <= n mod p. Two or three of
   the largest such primes pin j <= n/2 down to a handful of residue tuples (CRT), about 0.2
   candidates per (n, i) on average.

Controls, all run:

- `brute` mode tests every (n, i, j) against every prime with no pruning. For n <= 500 the fast path
  returns the same set in both modes: 5 failing (n, i) pairs for the strict version (p > i), 0 for the
  problem as stated.
- The strict version is known to fail. The fast path finds exactly nine failures below 1e7:
  (10,3,5) (16,2,6) (28,3,14) (28,5,14) (244,3,122) (512,2,147) (2048,2,713) (2188,3,1094)
  (1594324,3,797162). This includes the triple (28,5,14) that Erdős and Szekeres recorded.
- Two enumeration routines that share no code (digit descent on one prime, CRT on several) give the
  same output on [2, 1e7) strict and [2, 1e8) as stated.
- `spot_check.py`, a Python checker sharing no code with the Rust scan, prints the eight strict
  failures below 3000 and finds nothing as stated in 40 random windows of 300 inside [1e7, 4e10)
  (11,489 composite n, seed 699).

The case i = 3 has a second, independent program (`search/ep699_i3`): a counterexample forces the
6-smooth part s of n to satisfy b * c <= 0.0963 s^3, which leaves so few n that 1e18 takes 28 minutes.
Its control is a brute-force list of the pairs meeting the divisibility conditions, identical for
n <= 3,000,000 (160 pairs).

Reproduce:

```
cd search/ep699 && cargo build --release
target/release/ep699 strict brute 500
target/release/ep699 strict scan 2 10000000 8 strict.jsonl
target/release/ep699 weak scan 2 100000000000 8 ../../results/ep699-weak-1e11.jsonl
```

The scan appends one JSON line per block of 1e8 and skips finished blocks on restart.

## Problem 273

Statement: is there a covering system whose moduli are all of the form p - 1, p >= 5 prime?

Every such modulus is even, so a covering is a covering of the evens plus a covering of the odds with
disjoint moduli. `search/ep273/half_cover.py` checks that
{4, 6, 10, 12, 16, 18, 30, 36, 40, 60, 72, 108, 180, 432} covers every even integer. It spends 0.852
of reciprocal sum on half the integers. `search/ep273/pool_sum.py` shows why that is too expensive:
the usable moduli dividing an lcm of 8.5e21 sum to only 1.419, so the odd half would be left with
0.567 to cover a set of density 0.5.

## Prior work on 699

Checked on 2026-11-18, after the searches, from the authors' own repositories (the problem site itself
refused automated fetches, so its forum was not re-read):

| Who | Claim | Source |
|---|---|---|
| Cong Lu, Jan 2026 | no counterexample, n <= 1e7, all i, j | github.com/conglu1997/erdos_699_rust |
| Paul J. Lee, Oct 2026 | no counterexample, n < 1e12, all i, j; none with j <= 1e9 for any n; any counterexample has n <= j^2 (Lean) | github.com/Paulee314/erdos699, OEIS A399402 |
| Beltran12138, Oct 2026 | i = 3: none with n <= 1e16, by the same 2-power reduction; local lemmas in Lean | github.com/Beltran12138/erdos-699-i3 |
| Plasma AI, 2026-10-19 | i = 3: none with n <= 1e19; i = 7 and i >= 9 claimed for all n (rests on unreviewed manuscripts) | github.com/plasma-ai/erdos, E0699 claim file |

Their READMEs say these are AI-reviewed and not checked by a human mathematician. I re-ran none of them.

What that leaves of the work here:

- The all-i sweep to 1e11 is an independent confirmation of the first tenth of Lee's range, nothing more.
- The i = 3 result to 1e18 is inside Plasma's 1e19 and uses the same idea as Beltran's reduction.
- For i = 4, 5, 6 and 8 I found no published bound beyond Lee's 1e12, so "i = 4 to 1e18, i = 5, 6 to
  1e17, i = 8 to 1e15 (j < n/2)" may be new. That is a statement about what one search turned up, not
  a verified priority claim.

## What this is not

- Not a proof or disproof of any Erdős problem.
- The #699 scans are evidence over finite ranges (all i to 1e11, i <= 4 to 1e18).
  They say nothing about larger n. A recent proof claim on the forum (unverified by me) reduces the
  problem to i = 3 plus a finite set, which is why i = 3 got its own search.
- The structured results exclude j = n/2. That case rests on the n = 2j proof the site lists as accepted.
- The #273 half-covering does not show a full covering exists or does not exist.
- No Lean formalisation was written. There was no new theorem to formalise.

## Layout

`search/` code, `results/` raw JSONL, `notes/TRIAGE.md` the screening, `HANDOFF.md` state and next steps.
`upstream/` (gitignored) holds third-party mirrors: the community database and fetched pages.
