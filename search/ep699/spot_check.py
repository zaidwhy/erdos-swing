"""Independent spot check of the #699 sweep, in Python, sharing no code with the Rust scan.

For each sampled n and every i from 3 (2 in strict mode) up to n - prevprime(n), list the primes
p >= i (p > i in strict mode) dividing C(n,i) by trial division, then look for j in (i, n/2]
that no such prime divides, using Kummer digit by digit.  Prints every (n, i, j) found.

  spot_check.py strict 2 3000            control: must print the known strict failures
  spot_check.py weak <lo> <hi> <samples> <width> <seed>   random windows inside [lo, hi)
"""
import random
import sys


def factor(m):
    out = {}
    d = 2
    while d * d <= m:
        while m % d == 0:
            out[d] = out.get(d, 0) + 1
            m //= d
        d += 1 if d == 2 else 2
    if m > 1:
        out[m] = out.get(m, 0) + 1
    return out


def is_prime(m):
    return m > 1 and factor(m) == {m: 1}


def digit_le(j, n, p):
    while j:
        if j % p > n % p:
            return False
        j //= p
        n //= p
    return True


def dominated_values(n, p, lo, hi):
    """All j in [lo, hi] whose base-p digits are each <= those of n."""
    values = [0]
    scale = 1
    m = n
    while m:
        digit = m % p
        values = [v + d * scale for v in values for d in range(digit + 1) if v + d * scale <= hi]
        scale *= p
        m //= p
    return [v for v in values if lo <= v <= hi]


def failures(n, strict):
    prev = n
    while not is_prime(prev):
        prev -= 1
    gap = n - prev
    found = []
    for i in range(2 if strict else 3, gap + 1):
        if 2 * (i + 1) > n:
            break
        exponents = {}
        for k in range(i):
            for p, e in factor(n - k).items():
                exponents[p] = exponents.get(p, 0) + e
        for k in range(2, i + 1):
            for p, e in factor(k).items():
                exponents[p] -= e
        primes = sorted(p for p, e in exponents.items() if e > 0 and (p > i or (p == i and not strict)))
        if not primes:
            found.append((n, i, i + 1))
            continue
        for j in dominated_values(n, primes[-1], i + 1, n // 2):
            if all(digit_le(j, n, p) for p in primes):
                found.append((n, i, j))
                break
    return found


def main():
    strict = sys.argv[1] == "strict"
    lo, hi = int(sys.argv[2]), int(sys.argv[3])
    if len(sys.argv) > 4:
        samples, width = int(sys.argv[4]), int(sys.argv[5])
        rng = random.Random(int(sys.argv[6]))
        starts = sorted(rng.randrange(lo, hi - width) for _ in range(samples))
    else:
        starts, width = [lo], hi - lo
    checked = 0
    for start in starts:
        for n in range(max(start, 4), start + width):
            if is_prime(n):
                continue
            checked += 1
            for hit in failures(n, strict):
                print("FOUND", *hit, flush=True)
    print(f"mode={'strict' if strict else 'weak'} windows={len(starts)} width={width} composite n checked={checked}")


main()
