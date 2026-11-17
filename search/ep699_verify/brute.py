"""Literal brute force of the spec: no inequality, no factoring shortcuts.
usage: python brute.py 2,3,5 300000 out_necessary out_survivors"""
import sys

P = sorted(int(x) for x in sys.argv[1].split(","))
N = int(sys.argv[2])


def split(m):
    s = 1
    for p in P:
        while m % p == 0:
            m //= p
            s *= p
    return s, m


def primes_of(m):
    out, d = set(), 2
    while d * d <= m:
        while m % d == 0:
            out.add(d)
            m //= d
        d += 1
    if m > 1:
        out.add(m)
    return out


def digits_ok(j, n, p):
    while j > 0:
        if j % p > n % p:
            return False
        j //= p
        n //= p
    return True


pairs, surv = [], []
for n in range(9, N + 1):
    s, a = split(n)
    b = split(n - 1)[1]
    c = split(n - 2)[1]
    t = 1
    while 2 * t < s:
        x = t * (s - t)
        if x % b == 0 and (x * (s - 2 * t)) % c == 0:
            pairs.append((n, t))
            j = a * t
            if j >= 4 and 2 * j < n:
                ps = (primes_of(n) | primes_of(n - 1) | primes_of(n - 2)) - set(P)
                if all(digits_ok(j, n, p) for p in ps):
                    surv.append((n, j))
        t += 1

with open(sys.argv[3], "w") as f:
    for n, t in pairs:
        f.write(f"{n} {t}\n")
with open(sys.argv[4], "w") as f:
    for n, j in surv:
        f.write(f"{n} {j}\n")
print(f"P={P} N={N} necessary={len(pairs)} survivors={len(surv)}")
