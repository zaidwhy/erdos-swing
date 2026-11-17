# Problem 699 reduction: informal proofs for the Lean file

These are the proofs behind `lean/Erdos699Reduction.lean`. Nothing here has been machine-checked; the
Lean file has never been compiled. They are written so that each step is one Mathlib-sized fact.

Notation. p is a prime, p >= 5. For m >= 1, smooth(m) = 2^(v_2 m) 3^(v_3 m) and free(m) = m / smooth(m),
so free(m) is the product of p^(v_p m) over primes p >= 5. Fix n and j with 3 < j and 2j < n (so n >= 9),
and assume (H): no prime p >= 5 divides both C(n,3) and C(n,j).
Write s = smooth(n), a = free(n), b = free(n-1), c = free(n-2).

## Lemma 0 (digit criterion)

For a prime p: p does not divide C(n,j) iff for every k the k-th base-p digit of j is at most the
k-th base-p digit of n.

Proof. Lucas: C(n,j) = product over k of C(n_k, j_k) (mod p), where n_k, j_k are the digits. Each
factor has 0 <= n_k, j_k < p, so it is nonzero mod p exactly when j_k <= n_k (if j_k <= n_k < p then
C(n_k, j_k) divides n_k!, which p does not divide; if j_k > n_k the factor is 0). A product mod p is
nonzero iff every factor is. Lean route: induction on n using
`Choose.choose_modEq_choose_mod_mul_choose_div_nat`, with digits written as `n / p^k % p`.

## Lemma A (which primes are in play)

If p >= 5 and p divides n(n-1)(n-2), then p divides C(n,3), hence by (H) p does not divide C(n,j),
hence by Lemma 0 the digits of j are dominated by those of n in base p.

Proof. 6 C(n,3) = n(n-1)(n-2) and p does not divide 6, so p | C(n,3).

## Lemma 1 (a divides j)

Proof. Let p >= 5 with p^e exactly dividing n, e >= 1. The digits of n in positions 0..e-1 are 0. By
Lemma A the digits of j in those positions are 0, so p^e | j. The prime powers p^(v_p n), p >= 5, are
pairwise coprime and their product is a, so a | j.

Define t = j / a. Then t >= 1 since j > 0, and 2t < s because 2 a t = 2j < n = a s and a > 0.

## Lemma 2 (b divides t (s - t))

Proof. Let p >= 5 with p^e exactly dividing n - 1. Then n = 1 (mod p^e): digit 0 of n is 1 (as p > 1)
and digits 1..e-1 are 0. By Lemma A, digit 0 of j is 0 or 1 and digits 1..e-1 of j are 0, so
j = d (mod p^e) with d in {0, 1}. Multiply by s: s j = s a t = n t = t (mod p^e). So t = s d (mod p^e):
either p^e | t (d = 0) or p^e | s - t (d = 1; note 0 < t < s). In both cases p^e | t (s - t).
Coprime prime powers again give b | t (s - t).

## Lemma 3 (c divides t (s - t) (s - 2t))

Proof. Let p >= 5 with p^e exactly dividing n - 2. Then n = 2 (mod p^e): digit 0 of n is 2 (this needs
p > 2, true) and digits 1..e-1 are 0. So j = d (mod p^e) with d in {0, 1, 2}. Multiply by s:
s j = n t = 2t (mod p^e), so 2t = s d (mod p^e).
- d = 0: p^e | 2t, and p is odd, so p^e | t.
- d = 1: p^e | s - 2t (which is positive).
- d = 2: p^e | 2(s - t), so p^e | s - t.
In every case p^e | t (s - t) (s - 2t), and coprimality gives c | t (s - t) (s - 2t).

## Lemma 4 (size bound: 10 b c <= s^3)

Proof. gcd(n-1, n-2) = 1, so gcd(b, c) = 1. b | t(s-t) implies b | t(s-t)(s-2t); with Lemma 3 and
coprimality, b c | t(s-t)(s-2t). The right side is a positive integer (0 < t, 2t < s), so
b c <= t (s-t) (s-2t). It remains to show 10 t (s-t) (s-2t) <= s^3 for real 0 <= 2t <= s. Put x = t/s
in [0, 1/2] and f(x) = x(1-x)(1-2x) = x - 3x^2 + 2x^3. f'(x) = 1 - 6x + 6x^2 vanishes in the interval
only at x0 = (3 - sqrt 3)/6, where f(x0) = sqrt(3)/18 < 0.0963 < 1/10; f(0) = f(1/2) = 0.
Lean route: substitute u = s - 2t >= 0, so s = 2t + u, s - t = t + u, and the claim becomes
(2t + u)^3 >= 10 t (t + u) u, i.e. 8t^3 + 2t^2 u - 4t u^2 + u^3 >= 0 for t, u >= 0; try `nlinarith`.

## What the Lean statement should say (changes from the first sketch)

- Keep natural subtraction: every subtraction above is of a smaller number (n >= 9, 0 < t, 2t < s).
- `reduction` should return t = j / freePart n explicitly rather than an existential, so the search
  code and the statement talk about the same t.
- State Lemma A separately; it is the only place the hypothesis (H) is used.
- The generalisation to a smooth set P = primes <= q (notes/EP699-I3.md) needs only "p > 2" in
  Lemma 3 and "p does not divide i!" in Lemma A; formalise {2,3} first.
