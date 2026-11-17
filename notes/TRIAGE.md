# Screening the open list (2026-11-18)

Source: github.com/teorth/erdosproblems at its commit of 2026-11-12 (1221 problems) plus the problem
pages and forum threads on erdosproblems.com.

Counts from the database: 588 open, 25 falsifiable, 8 decidable, 7 verifiable (628 not settled).
All 628 statements were read. Threads were read for the problems named below.

## Finding

The searches a laptop can run have been run. Almost every problem that one example would settle has a
thread with exhaustive or large-scale computation from 2026, usually AI-assisted, often with Lean.

| Problem | Why it looked reachable | What the thread already has |
|---|---|---|
| 617 | r = 5 is one SAT instance on K_26 | r = 5 through 9 claimed proved, several independent proofs, Lean for r = 5 |
| 993 | counterexample is one tree | exhaustive to 32 vertices (1.1e11 trees), sampling to 101, two proof claims |
| 488 | counterexample is one finite set | a counterexample claim with a Lean proof (2026-09-05), not yet reflected in the database |
| 273 | a covering is a finite certificate | exhaustive: none with lcm below 8e6; local search stalls at 10 to 14 percent uncovered |
| 203 | a covering of Z^2 is a finite certificate | pools to 1006 primes searched, coverage stalls near 72 to 76 percent |
| 850 | one triple settles it | OEIS A343101 lists a 2025 paper (arXiv:2506.01099) that is a dedicated search for the two-term version |
| 699 | counterexample is one triple | checked to 1e7; proof claims for j <= 3i/2, n = 2j, and (unverified) all but i = 3 |
| 835 | colouring of J(2k,k) | settled for every k except k = p - 1; by my reduction k = 10 needs 11 disjoint Steiner systems S(9,10,20) |
| 364, 366, 242, 398 | finite witness | searched to 7.38e28, 1e22, 1e18 and n = 1e9 respectively, per the pages |

Problems where the remaining finite range is astronomically large (thresholds from "sufficiently
large" proofs): 19, 475, 551, 556, 580, 742, 848.

## What was still worth running

- **699**: nobody had used the prime-gap and CRT pruning, so the range could move from 1e7 to 1e11.
  Heuristically a counterexample is not expected: for i = 3 the expected number of solutions j for a
  given n is about n * 6^w / rad(n(n-1)(n-2)), which abc-type bounds put near 1/n.
- **273**: the parity split and the "effective modulus" trick (a modulus m acts on a class mod D as
  m / gcd(m, D), so deep classes see many copies of each small modulus) give half a covering by hand.
  The budget table says a full covering needs an lcm far beyond 1e16 and waste under 40 percent, which
  is a construction project, not a search.

## Decisions taken without asking (one line each)

- Chose 699 as the compute target over 273: 699 had an unexploited algorithm, 273 needs a design idea.
- Did not install Lean: no new theorem to check, and the disk had 0.3 to 16 GB free during the session.
- Did not create a GitHub repo or post to the forum: both go out under Zaid's name.
