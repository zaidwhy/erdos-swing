# erdos-swing

An attempt on the open Erdős problems that a finite computation could settle.

**Thesis.** A problem is only worth a search if (a) one example or counterexample would settle it and
(b) nobody has already run that search. Every search here ships with a control that could have failed,
and nothing is called a result until a second, independently written path agrees.

**Status: no open problem resolved.** What exists is below, with the limits stated.

## What was done

| Problem | What ran | Outcome |
|---|---|---|
| all 628 not-yet-settled problems | read every statement and the discussion threads of the candidates | see `notes/TRIAGE.md`; the cheap searches have been run by others |
| [#699](https://www.erdosproblems.com/699) | exact search for a counterexample, all n below 1e11 | none found in the part completed so far; see `HANDOFF.md` for the live count |
| #699, case i = 3 | direct enumeration with the prime 3 ignored, n up to 3,000,000 | no j below n/2 survives even the primes >= 5 |
| [#273](https://www.erdosproblems.com/273) | 14 moduli p - 1 that cover every even integer, checked exactly | half a covering; the odd half is the open part |

## Problem 699

Statement: for 1 <= i < j <= n/2 some prime p >= i divides gcd(C(n,i), C(n,j)).

The public check stood at n <= 1e7. `search/ep699` goes further by not testing most (n, i, j) at all:

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

## What this is not

- Not a proof or disproof of any Erdős problem.
- The #699 scan is evidence over a finite range. It says nothing about larger n, and a recent proof
  claim on the forum (unverified by me) reduces the problem to i = 3 plus a finite set.
- The #273 half-covering does not show a full covering exists or does not exist.
- No Lean formalisation was written. There was no new theorem to formalise.

## Layout

`search/` code, `results/` raw JSONL, `notes/TRIAGE.md` the screening, `HANDOFF.md` state and next steps.
`upstream/` (gitignored) holds third-party mirrors: the community database and fetched pages.
