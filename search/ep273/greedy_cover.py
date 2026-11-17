"""Erdős 273: how much of Z/L can each pool cover? (see notes/EP273.md for the pools)

  q : odd q with 2q + 1 prime   (shared by the two odd classes)
  r : r with 4r + 1 prime       (each r serves one class)

Greedy placement (smallest modulus first, residue with the most uncovered points), followed by
rounds that lift one modulus out and put it back at its best residue.  The result is a lower
bound on the best coverage, not the optimum.  One thread, numpy only.
"""
import sys

import numpy as np


def is_prime(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def pool_for(period, kind):
    out = []
    for d in range(3, period + 1):
        if period % d:
            continue
        if kind == "q" and d % 2 == 1 and is_prime(2 * d + 1):
            out.append(d)
        if kind == "r" and is_prime(4 * d + 1):
            out.append(d)
    return out


def best_residue(counts, modulus, period):
    """counts[x] = how many chosen classes contain x.  Returns the residue whose class holds
    the most points with count 0."""
    free = (counts == 0).reshape(period // modulus, modulus).sum(axis=0)
    return int(free.argmax()), int(free.max())


def main():
    period = int(sys.argv[1])
    kind = sys.argv[2]
    rounds = int(sys.argv[3]) if len(sys.argv) > 3 else 6
    moduli = pool_for(period, kind)
    counts = np.zeros(period, dtype=np.int16)
    residues = {}
    for m in moduli:
        a, _ = best_residue(counts, m, period)
        residues[m] = a
        counts[a::m] += 1
    history = [int((counts == 0).sum())]
    for _ in range(rounds):
        for m in moduli:
            counts[residues[m]::m] -= 1
            a, _ = best_residue(counts, m, period)
            residues[m] = a
            counts[a::m] += 1
        history.append(int((counts == 0).sum()))
        if history[-1] == history[-2]:
            break
    left = history[-1]
    print(f"L={period} pool={kind} moduli={len(moduli)} sum={sum(1 / m for m in moduli):.4f} "
          f"uncovered={left} ({left / period:.4%}) rounds={len(history) - 1}")


main()
