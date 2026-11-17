/-
NOT COMPILED. Lean and Mathlib are not installed on this machine (disk space), so this file
has never been checked. It records the statement to formalise and the proof route, nothing more.
Names of Mathlib lemmas below are from memory and must be confirmed against the pinned version.

Target: the reduction behind `notes/EP699-I3.md`. If no prime p >= 5 divides both C(n,3) and
C(n,j), with 3 < j < n/2, then writing n = s * a (s the 6-smooth part of n):
  a ∣ j, and with j = a * t:  b ∣ t (s - t)  and  c ∣ t (s - t) (s - 2t),
where b, c are the parts of n - 1, n - 2 coprime to 6.
-/
import Mathlib

namespace Erdos699

/-- The part of `m` made of the primes 2 and 3. -/
def smoothPart (m : ℕ) : ℕ := 2 ^ m.factorization 2 * 3 ^ m.factorization 3

/-- The part of `m` coprime to 6. -/
def freePart (m : ℕ) : ℕ := m / smoothPart m

/-- No prime `p ≥ 5` divides both `C(n,3)` and `C(n,j)`. -/
def NoCommonLargePrime (n j : ℕ) : Prop :=
  ∀ p : ℕ, p.Prime → 5 ≤ p → p ∣ n.choose 3 → ¬ p ∣ n.choose j

/-- Step 0 (Kummer / Lucas): for a prime `p`, `p ∤ C(n,j)` iff every base-`p` digit of `j` is at
most the matching digit of `n`. Route: `Nat.Prime.multiplicity_choose` (Kummer) or Lucas'
theorem `Choose.choose_modEq_choose_mod_mul_choose_div_nat`, by induction on digits. -/
theorem not_dvd_choose_iff_digits_le (p n j : ℕ) (hp : p.Prime) :
    ¬ p ∣ n.choose j ↔ ∀ k, j / p ^ k % p ≤ n / p ^ k % p := by
  sorry

/-- Step 1: a prime power `p ^ e ∣ n` with `p ≥ 5` divides `j` (the low `e` digits of `n` are 0). -/
theorem freePart_dvd (n j : ℕ) (hj : 3 < j) (hjn : 2 * j < n) (h : NoCommonLargePrime n j) :
    freePart n ∣ j := by
  sorry

/-- Steps 2 and 3: the divisibility conditions on `t = j / freePart n`.
For `p ^ e ∣ n - 1` the low digits of `n` are `1, 0, .., 0`, so `j ≡ 0` or `1 (mod p ^ e)`;
for `p ^ e ∣ n - 2` they are `2, 0, .., 0`, so `j ≡ 0, 1` or `2`. Multiply through by `s`. -/
theorem reduction (n j : ℕ) (hj : 3 < j) (hjn : 2 * j < n) (h : NoCommonLargePrime n j) :
    ∃ t : ℕ, j = freePart n * t ∧ 0 < t ∧ 2 * t < smoothPart n ∧
      freePart (n - 1) ∣ t * (smoothPart n - t) ∧
      freePart (n - 2) ∣ t * (smoothPart n - t) * (smoothPart n - 2 * t) := by
  sorry

/-- Consequence used by the search: the size bound. `freePart (n-1)` and `freePart (n-2)` are
coprime, and `t (s - t) (s - 2t) ≤ s ^ 3 / 10` for `0 < 2t < s` (the true maximum is 0.0962 s^3). -/
theorem size_bound (n j : ℕ) (hj : 3 < j) (hjn : 2 * j < n) (h : NoCommonLargePrime n j) :
    10 * (freePart (n - 1) * freePart (n - 2)) ≤ smoothPart n ^ 3 := by
  sorry

end Erdos699
