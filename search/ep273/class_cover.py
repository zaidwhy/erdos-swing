"""Erdős 273 after dividing out the modulus 4.

Moduli are m = p - 1 with p >= 5 prime.  m = 4 covers x = 0 (mod 4).  On each other class
x = 4w + c a modulus m = 4r acts as r (and touches only that class), while m = 2q with q odd
acts as q on both classes of its parity.  So a covering is three coverings of the integers w:

  class 2:  moduli r with 4r + 1 prime               (plus odd q given to the even side)
  class 1 and class 3:  the same odd q (2q + 1 prime), with residues that are translates of
                        each other, each completed by its own, disjoint, set of r.

This script searches exactly (depth first, lowest uncovered residue first) for a covering of
Z/L by distinct moduli drawn from a pool, to see which of the three pieces exist at small lcm.
"""
import sys


def is_prime(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def divisors(n):
    return [d for d in range(1, n + 1) if n % d == 0]


def find_cover(period, pool, node_limit):
    """pool: list of (label, modulus) with modulus | period.  Returns a list of
    (label, modulus, residue) covering every residue mod period, or None."""
    full = (1 << period) - 1
    masks = {}

    def mask(modulus, residue):
        key = (modulus, residue)
        if key not in masks:
            base = 0
            for x in range(residue, period, modulus):
                base |= 1 << x
            masks[key] = base
        return masks[key]

    pool = sorted(pool, key=lambda item: item[1])
    used = [False] * len(pool)
    chosen = []
    nodes = [0]

    def search(covered):
        if covered == full:
            return True
        nodes[0] += 1
        if nodes[0] > node_limit:
            return False
        free = period - bin(covered).count("1")
        capacity = sum(period // pool[k][1] for k in range(len(pool)) if not used[k])
        if capacity < free:
            return False
        gap = ~covered & full
        x = (gap & -gap).bit_length() - 1
        tried = set()
        for k, (label, modulus) in enumerate(pool):
            if used[k] or modulus in tried:
                continue
            tried.add(modulus)
            used[k] = True
            chosen.append((label, modulus, x % modulus))
            if search(covered | mask(modulus, x % modulus)):
                return True
            chosen.pop()
            used[k] = False
        return False

    found = search(0)
    return (list(chosen) if found else None), nodes[0]


def main():
    period = int(sys.argv[1])
    which = sys.argv[2]
    node_limit = int(sys.argv[3]) if len(sys.argv) > 3 else 2_000_000
    r_pool = [("r", d) for d in divisors(period) if d >= 3 and is_prime(4 * d + 1)]
    q_pool = [("q", d) for d in divisors(period) if d >= 3 and d % 2 == 1 and is_prime(2 * d + 1)]
    pool = {"r": r_pool, "q": q_pool, "rq": r_pool + q_pool}[which]
    total = sum(1 / m for _, m in pool)
    cover, nodes = find_cover(period, pool, node_limit)
    status = "COVER" if cover else ("none" if nodes <= node_limit else "gave up")
    print(f"L={period} pool={which} size={len(pool)} sum={total:.3f} nodes={nodes} -> {status}")
    if cover:
        print("  ", sorted((m, a, label) for label, m, a in cover))


main()
