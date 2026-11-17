"""Brute-force reference for the structured i = 3 enumerator.

Prints every (n, t) with n <= limit, 1 <= t < s/2, b | t(s-t) and c | t(s-t)(s-2t), where s is
the 6-smooth part of n and b, c are the parts of n-1, n-2 coprime to 6.  No size bound and no
factoring is used, so agreement with `ep699_i3 necessary` checks that the enumerator's
progressions and its residue search miss nothing.
"""
import sys


def split(m):
    smooth = 1
    for p in (2, 3):
        while m % p == 0:
            m //= p
            smooth *= p
    return smooth, m


limit = int(sys.argv[1])
for n in range(9, limit + 1):
    s, _ = split(n)
    if s < 3:
        continue
    _, b = split(n - 1)
    _, c = split(n - 2)
    for t in range(1, (s + 1) // 2):
        if 2 * t < s and t * (s - t) % b == 0 and t * (s - t) * (s - 2 * t) % c == 0:
            print(n, t)
