"""Erdős 273: fourteen moduli p - 1 (p >= 5 prime) that cover every even integer.

All moduli p - 1 are even, so a covering splits into one covering of the even integers and
one of the odd integers, with disjoint sets of moduli.  This checks the even half exactly.
The odd half, which may not use the modulus 4, is the part that stays open.
"""
from math import lcm

# (q, residue): the class y = residue (mod q) on y = x / 2, i.e. x = 2 * residue (mod 2q).
HALVED_CLASSES = [
    (2, 0),
    (3, 1), (6, 3),
    (9, 5), (18, 11),
    (36, 17), (54, 17), (8, 3), (216, 143),
    (5, 2), (15, 5), (30, 23), (90, 71), (20, 19),
]


def is_prime(n):
    return n > 1 and all(n % d for d in range(2, int(n ** 0.5) + 1))


def main():
    moduli = [2 * q for q, _ in HALVED_CLASSES]
    assert len(set(moduli)) == len(moduli), "moduli must be distinct"
    assert all(m >= 4 and is_prime(m + 1) for m in moduli), "every modulus must be p - 1"
    period = lcm(*moduli)
    uncovered_even = [x for x in range(0, period, 2)
                      if not any(x % (2 * q) == 2 * r for q, r in HALVED_CLASSES)]
    covered_odd = sum(any(x % (2 * q) == 2 * r for q, r in HALVED_CLASSES) for x in range(1, period, 2))
    print("moduli:", sorted(moduli))
    print("period:", period)
    print("sum of 1/m:", round(sum(1 / m for m in moduli), 4))
    print("even residues left uncovered:", len(uncovered_even))
    print("odd residues covered:", covered_odd)
    assert not uncovered_even


main()
