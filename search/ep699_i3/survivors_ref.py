"""Reference for the i = 3 case of Erdős 699, by direct enumeration.

A pair (n, j) with 4 <= j < n/2 is a "5-survivor" when j is digit-dominated by n in
base p for every prime p >= 5 dividing n(n-1)(n-2).  A counterexample with i = 3 is a
5-survivor that also passes the prime 3 (when 3 | C(n,3)).  The structured enumerator
must reproduce this list exactly; that is its positive control.
"""
import sys

def dominated(j, n, p):
    while j:
        if j % p > n % p:
            return False
        j //= p
        n //= p
    return True

def main(limit):
    spf = list(range(limit + 1))
    for p in range(2, int(limit ** 0.5) + 1):
        if spf[p] == p:
            for m in range(p * p, limit + 1, p):
                if spf[m] == m:
                    spf[m] = p

    def big_primes(m):
        out = set()
        while m > 1:
            p = spf[m]
            if p >= 5:
                out.add(p)
            m //= p
        return out

    for n in range(9, limit + 1):
        cofactor = n
        while cofactor % 2 == 0:
            cofactor //= 2
        while cofactor % 3 == 0:
            cofactor //= 3
        smooth = n // cofactor
        if smooth < 4:
            continue
        primes = sorted(big_primes(n) | big_primes(n - 1) | big_primes(n - 2), reverse=True)
        for t in range(1, (smooth + 1) // 2):
            j = cofactor * t
            if j >= 4 and 2 * j < n and all(dominated(j, n, p) for p in primes):
                three = (n * (n - 1) * (n - 2) // 6) % 3 == 0
                passes_three = (not three) or dominated(j, n, 3)
                print(n, j, smooth, t, int(three), int(passes_three), flush=True)

main(int(sys.argv[1]))
