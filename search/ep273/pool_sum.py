"""Erdős 273: reciprocal sum of the usable moduli (d | L, d >= 4, d + 1 prime) for candidate lcms L.

A covering needs this sum to be at least 1; how far above 1 it gets says how much overlap
a construction with that lcm can afford.
"""
import itertools, sys

def isprime(n):
    """Deterministic Miller-Rabin; these twelve bases are exact below 3.3e24."""
    if n < 2:
        return False
    small = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for p in small:
        if n % p == 0:
            return n == p
    d, r = n - 1, 0
    while d % 2 == 0:
        d //= 2
        r += 1
    for a in small:
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(r - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True

def pool(exponents):
    primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37][: len(exponents)]
    divisors = [1]
    for p, e in zip(primes, exponents):
        divisors = [d * p ** k for d in divisors for k in range(e + 1)]
    return sorted(d for d in divisors if d >= 4 and isprime(d + 1))

for spec in sys.argv[1:]:
    exps = [int(c) for c in spec.split(",")]
    moduli = pool(exps)
    L = 1
    for p, e in zip([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37], exps):
        L *= p ** e
    print(f"{spec:24s} L={L:.3e} moduli={len(moduli):6d} sum={sum(1 / d for d in moduli):.4f} "
          f"sum(<=1000)={sum(1 / d for d in moduli if d <= 1000):.4f}")
