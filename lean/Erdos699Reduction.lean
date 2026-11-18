/-
The reduction behind `notes/EP699-I3.md`, for the smooth set {2, 3}.

Fix `n` and `j` with `0 < j` and `2 * j < n`, and suppose no prime `p ≥ 5` divides both `C(n,3)`
and `C(n,j)`. Write `a`, `b`, `c` for the parts of `n`, `n - 1`, `n - 2` coprime to 6 and
`s = n / a`. Then `a ∣ j`, and with `t = j / a`:
  `0 < t`, `2 * t < s`, `b ∣ t * (s - t)`, `c ∣ t * (s - t) * (s - 2 * t)`, `10 * (b * c) ≤ s ^ 3`.

This file is checked by CI (`.github/workflows/lean.yml`), not on the author's machine.
Informal proofs: `notes/EP699-PROOFS.md`.
-/
import Mathlib

namespace Erdos699

/-! ### Kummer, in the form used here -/

/-- If `p ∤ C(n,j)` then `j mod p^e ≤ n mod p^e` for every `e` (no carry when adding `j` and
`n - j` in base `p`). -/
theorem mod_le_of_not_dvd_choose {p n j : ℕ} (hp : p.Prime) (hjn : j ≤ n)
    (h : ¬ p ∣ n.choose j) (e : ℕ) : j % p ^ e ≤ n % p ^ e := by
  rcases Nat.eq_zero_or_pos e with rfl | he
  · simp [Nat.mod_one]
  have hb : Nat.log p n < max (Nat.log p n) e + 1 := Nat.lt_succ_of_le (le_max_left _ _)
  have hv : emultiplicity p (n.choose j) = 0 := emultiplicity_eq_zero.mpr h
  rw [hp.emultiplicity_choose hjn hb, Nat.cast_eq_zero, Finset.card_eq_zero,
    Finset.filter_eq_empty_iff] at hv
  have hlt : j % p ^ e + (n - j) % p ^ e < p ^ e :=
    not_le.mp (hv (Finset.mem_Ico.mpr ⟨he, Nat.lt_succ_of_le (le_max_right _ _)⟩))
  have h1 : (j + (n - j)) % p ^ e = j % p ^ e + (n - j) % p ^ e := by
    rw [Nat.add_mod]; exact Nat.mod_eq_of_lt hlt
  rw [Nat.add_sub_of_le hjn] at h1
  rw [h1]; exact Nat.le_add_right _ _

/-- If moreover `p^e ∣ n - c` then `j mod p^e ≤ c`. -/
theorem mod_le_const {p n j c e : ℕ} (hp : p.Prime) (hjn : j ≤ n) (h : ¬ p ∣ n.choose j)
    (hc : c ≤ n) (hq : p ^ e ∣ n - c) : j % p ^ e ≤ c := by
  have h1 := mod_le_of_not_dvd_choose hp hjn h e
  obtain ⟨k, hk⟩ := hq
  have hn : n % p ^ e = c % p ^ e := by
    rw [(Nat.sub_eq_iff_eq_add hc).mp hk, Nat.mul_add_mod]
  rw [hn] at h1
  exact h1.trans (Nat.mod_le _ _)

/-! ### From prime powers to divisors -/

theorem dvd_of_prime_pow_dvd {a m : ℕ} (ha : a ≠ 0) (hm : m ≠ 0)
    (h : ∀ p, p.Prime → p ∣ a → p ^ a.factorization p ∣ m) : a ∣ m := by
  rw [← Nat.factorization_prime_le_iff_dvd ha hm]
  intro p hp
  by_cases hpa : p ∣ a
  · exact (hp.pow_dvd_iff_le_factorization hm).mp (h p hp hpa)
  · rw [Nat.factorization_eq_zero_of_not_dvd hpa]; exact Nat.zero_le _

/-! ### The congruence step -/

theorem int_key (q s a t n j d k u c : ℤ) (e1 : j - d = q * u) (e2 : n - c = q * k)
    (e3 : n = s * a) (e4 : j = a * t) : c * t - s * d = q * (s * u - k * t) := by
  linear_combination s * e1 - t * e2 + t * e3 - s * e4

/-- With `n = s a`, `j = a t` and `q ∣ n - c`: `q ∣ c t - s (j mod q)` over the integers. -/
theorem pp_dvd {n j s a t q c : ℕ} (hns : n = s * a) (hja : j = a * t) (hq : q ∣ n - c)
    (hc : c ≤ n) : (q : ℤ) ∣ ((c : ℕ) : ℤ) * (t : ℤ) - (s : ℤ) * ((j % q : ℕ) : ℤ) := by
  obtain ⟨k, hk⟩ := hq
  have e2 : (n : ℤ) - (c : ℤ) = (q : ℤ) * (k : ℤ) := by
    have h := (Nat.sub_eq_iff_eq_add hc).mp hk
    have h' : (n : ℤ) = (q : ℤ) * (k : ℤ) + (c : ℤ) := by exact_mod_cast h
    linarith
  have e1 : (j : ℤ) - ((j % q : ℕ) : ℤ) = (q : ℤ) * ((j / q : ℕ) : ℤ) := by
    have h := Nat.mod_add_div j q
    have h' : ((j % q : ℕ) : ℤ) + (q : ℤ) * ((j / q : ℕ) : ℤ) = (j : ℤ) := by exact_mod_cast h
    linarith
  have e3 : (n : ℤ) = (s : ℤ) * (a : ℤ) := by exact_mod_cast hns
  have e4 : (j : ℤ) = (a : ℤ) * (t : ℤ) := by exact_mod_cast hja
  exact ⟨_, int_key _ _ _ _ _ _ _ _ _ _ e1 e2 e3 e4⟩

/-- Prime-power step for `n - 1`. -/
theorem pp_pred {n j s a t q : ℕ} (hns : n = s * a) (hja : j = a * t) (hts : t < s)
    (hn : 1 ≤ n) (hq : q ∣ n - 1) (hd : j % q ≤ 1) : q ∣ t * (s - t) := by
  have key := pp_dvd hns hja hq hn
  obtain ⟨d, hdd⟩ : ∃ d, j % q = d := ⟨_, rfl⟩
  rw [hdd] at key hd
  obtain ⟨w, hw⟩ := key
  interval_cases d
  · push_cast at hw
    have h1 : (q : ℤ) ∣ (t : ℤ) := ⟨w, by linear_combination hw⟩
    exact dvd_mul_of_dvd_left (Int.natCast_dvd_natCast.mp h1) _
  · push_cast at hw
    have h1 : (q : ℤ) ∣ ((s - t : ℕ) : ℤ) :=
      ⟨-w, by push_cast [Nat.cast_sub hts.le]; linear_combination -hw⟩
    exact dvd_mul_of_dvd_right (Int.natCast_dvd_natCast.mp h1) _

/-- Prime-power step for `n - 2`, for `q` odd. -/
theorem pp_pred_two {n j s a t q : ℕ} (hns : n = s * a) (hja : j = a * t) (h2ts : 2 * t < s)
    (hn : 2 ≤ n) (hq : q ∣ n - 2) (hq2 : Nat.Coprime q 2) (hd : j % q ≤ 2) :
    q ∣ t * (s - t) * (s - 2 * t) := by
  have hts : t < s := by omega
  have key := pp_dvd hns hja hq hn
  obtain ⟨d, hdd⟩ : ∃ d, j % q = d := ⟨_, rfl⟩
  rw [hdd] at key hd
  obtain ⟨w, hw⟩ := key
  interval_cases d
  · push_cast at hw
    have h1 : (q : ℤ) ∣ ((2 * t : ℕ) : ℤ) := ⟨w, by push_cast; linear_combination hw⟩
    have h2 : q ∣ t := hq2.dvd_of_dvd_mul_left (Int.natCast_dvd_natCast.mp h1)
    exact dvd_mul_of_dvd_left (dvd_mul_of_dvd_left h2 _) _
  · push_cast at hw
    have h1 : (q : ℤ) ∣ ((s - 2 * t : ℕ) : ℤ) :=
      ⟨-w, by push_cast [Nat.cast_sub h2ts.le]; linear_combination -hw⟩
    exact dvd_mul_of_dvd_right (Int.natCast_dvd_natCast.mp h1) _
  · push_cast at hw
    have h1 : (q : ℤ) ∣ ((2 * (s - t) : ℕ) : ℤ) :=
      ⟨-w, by push_cast [Nat.cast_sub hts.le]; linear_combination -hw⟩
    have h2 : q ∣ s - t := hq2.dvd_of_dvd_mul_left (Int.natCast_dvd_natCast.mp h1)
    exact dvd_mul_of_dvd_left (dvd_mul_of_dvd_right h2 _) _

/-! ### The three divisibility statements, for arbitrary divisors -/

/-- A divisor `a` of `n` all of whose primes miss `C(n,j)` divides `j`. -/
theorem dvd_of_dvd_self {n j a : ℕ} (hj : 0 < j) (hjn : j ≤ n) (hn : n ≠ 0) (han : a ∣ n)
    (h : ∀ p, p.Prime → p ∣ a → ¬ p ∣ n.choose j) : a ∣ j := by
  have ha : a ≠ 0 := fun h0 => hn (zero_dvd_iff.mp (h0 ▸ han))
  apply dvd_of_prime_pow_dvd ha hj.ne'
  intro p hp hpa
  have hq : p ^ a.factorization p ∣ n - 0 := by
    rw [Nat.sub_zero]; exact (Nat.ordProj_dvd a p).trans han
  have h0 := mod_le_const hp hjn (h p hp hpa) (Nat.zero_le n) hq
  exact Nat.dvd_of_mod_eq_zero (Nat.le_zero.mp h0)

theorem dvd_of_dvd_pred {n j s a t b : ℕ} (hjn : j ≤ n) (hn : 1 ≤ n) (hns : n = s * a)
    (hja : j = a * t) (ht : 0 < t) (hts : t < s) (hb : b ∣ n - 1) (hb0 : b ≠ 0)
    (h : ∀ p, p.Prime → p ∣ b → ¬ p ∣ n.choose j) : b ∣ t * (s - t) := by
  apply dvd_of_prime_pow_dvd hb0 (Nat.mul_ne_zero ht.ne' (Nat.sub_ne_zero_of_lt hts))
  intro p hp hpb
  have hq : p ^ b.factorization p ∣ n - 1 := (Nat.ordProj_dvd b p).trans hb
  exact pp_pred hns hja hts hn hq (mod_le_const hp hjn (h p hp hpb) hn hq)

theorem dvd_of_dvd_pred_two {n j s a t c : ℕ} (hjn : j ≤ n) (hn : 2 ≤ n) (hns : n = s * a)
    (hja : j = a * t) (ht : 0 < t) (h2ts : 2 * t < s) (hc : c ∣ n - 2) (hc0 : c ≠ 0)
    (hodd : ¬ 2 ∣ c) (h : ∀ p, p.Prime → p ∣ c → ¬ p ∣ n.choose j) :
    c ∣ t * (s - t) * (s - 2 * t) := by
  have hpos : t * (s - t) * (s - 2 * t) ≠ 0 :=
    Nat.mul_ne_zero (Nat.mul_ne_zero ht.ne' (by omega)) (by omega)
  apply dvd_of_prime_pow_dvd hc0 hpos
  intro p hp hpc
  have hq : p ^ c.factorization p ∣ n - 2 := (Nat.ordProj_dvd c p).trans hc
  have hp2 : p ≠ 2 := fun h2 => hodd (h2 ▸ hpc)
  have hcop : Nat.Coprime (p ^ c.factorization p) 2 :=
    Nat.Coprime.pow_left _ ((Nat.coprime_primes hp Nat.prime_two).mpr hp2)
  exact pp_pred_two hns hja h2ts hn hq hcop (mod_le_const hp hjn (h p hp hpc) hn hq)

/-! ### The size bound -/

theorem int_cubic (t u : ℤ) (ht : 0 ≤ t) (hu : 0 ≤ u) :
    10 * (t * (t + u) * u) ≤ (2 * t + u) ^ 3 := by
  nlinarith [mul_nonneg ht (sq_nonneg (5 * t - 2 * u)), mul_nonneg hu (sq_nonneg (5 * t - 2 * u)),
    pow_nonneg ht 3, mul_nonneg (mul_nonneg ht ht) hu]

theorem size_bound_of_dvd {s t b c : ℕ} (ht : 0 < t) (h2ts : 2 * t < s) (hbc : Nat.Coprime b c)
    (hb : b ∣ t * (s - t)) (hc : c ∣ t * (s - t) * (s - 2 * t)) : 10 * (b * c) ≤ s ^ 3 := by
  have hdvd : b * c ∣ t * (s - t) * (s - 2 * t) :=
    hbc.mul_dvd_of_dvd_of_dvd (dvd_mul_of_dvd_left hb _) hc
  obtain ⟨u, rfl⟩ : ∃ u, s = 2 * t + u := ⟨s - 2 * t, by omega⟩
  have h1 : 2 * t + u - t = t + u := by omega
  have h2 : 2 * t + u - 2 * t = u := by omega
  rw [h1, h2] at hdvd
  have hu : 0 < u := by omega
  have hle : b * c ≤ t * (t + u) * u :=
    Nat.le_of_dvd (Nat.mul_pos (Nat.mul_pos ht (by omega)) hu) hdvd
  have hcub : 10 * (t * (t + u) * u) ≤ (2 * t + u) ^ 3 := by
    have h := int_cubic (t : ℤ) (u : ℤ) (Int.natCast_nonneg t) (Int.natCast_nonneg u)
    exact_mod_cast h
  exact (Nat.mul_le_mul_left 10 hle).trans hcub

/-! ### The parts coprime to 6 -/

/-- The part of `m` coprime to 6: divide out the powers of 2 and then of 3. -/
def freePart (m : ℕ) : ℕ := ordCompl[3] (ordCompl[2] m)

/-- The 6-smooth part of `m`. -/
def smoothPart (m : ℕ) : ℕ := m / freePart m

theorem freePart_dvd (m : ℕ) : freePart m ∣ m :=
  (Nat.ordCompl_dvd _ 3).trans (Nat.ordCompl_dvd m 2)

theorem freePart_ne_zero {m : ℕ} (hm : m ≠ 0) : freePart m ≠ 0 :=
  (Nat.ordCompl_pos 3 (Nat.ordCompl_pos 2 hm).ne').ne'

theorem smoothPart_mul_freePart (m : ℕ) : smoothPart m * freePart m = m :=
  Nat.div_mul_cancel (freePart_dvd m)

theorem five_le_of_dvd_freePart {m p : ℕ} (hm : m ≠ 0) (hp : p.Prime) (h : p ∣ freePart m) :
    5 ≤ p := by
  have h2 : ¬ 2 ∣ freePart m := fun h2 =>
    Nat.not_dvd_ordCompl Nat.prime_two hm (h2.trans (Nat.ordCompl_dvd _ 3))
  have h3 : ¬ 3 ∣ freePart m := Nat.not_dvd_ordCompl Nat.prime_three (Nat.ordCompl_pos 2 hm).ne'
  by_contra hlt
  have hp2 := hp.two_le
  have hp5 : p < 5 := not_le.mp hlt
  interval_cases p
  · exact h2 h
  · exact h3 h
  · exact absurd hp (by norm_num)

/-! ### Primes dividing `C(n,3)` -/

theorem six_mul_choose_three (n : ℕ) : 6 * n.choose 3 = n * (n - 1) * (n - 2) := by
  have h : n.descFactorial 3 = Nat.factorial 3 * n.choose 3 :=
    Nat.descFactorial_eq_factorial_mul_choose n 3
  have h3 : n.descFactorial 3 = (n - 2) * ((n - 1) * ((n - 0) * 1)) := rfl
  have h6 : Nat.factorial 3 = 6 := rfl
  rw [h3, h6, Nat.sub_zero, Nat.mul_one] at h
  rw [← h]; ring

theorem dvd_choose_three {p n : ℕ} (hp : p.Prime) (h5 : 5 ≤ p)
    (h : p ∣ n * (n - 1) * (n - 2)) : p ∣ n.choose 3 := by
  rw [← six_mul_choose_three] at h
  rcases (Nat.Prime.dvd_mul hp).mp h with h6 | hc
  · exfalso
    have hle : p ≤ 6 := Nat.le_of_dvd (by norm_num) h6
    interval_cases p
    · omega
    · exact absurd hp (by norm_num)
  · exact hc

/-! ### The reduction -/

/-- No prime `p ≥ 5` divides both `C(n,3)` and `C(n,j)`. For `3 < j` this is what a
counterexample `(n, 3, j)` or `(n, 4, j)` to problem 699 must satisfy. -/
def NoCommonLargePrime (n j : ℕ) : Prop :=
  ∀ p : ℕ, p.Prime → 5 ≤ p → p ∣ n.choose 3 → ¬ p ∣ n.choose j

theorem reduction (n j : ℕ) (hj : 0 < j) (hjn : 2 * j < n) (h : NoCommonLargePrime n j) :
    freePart n ∣ j ∧ 0 < j / freePart n ∧ 2 * (j / freePart n) < smoothPart n ∧
      freePart (n - 1) ∣ (j / freePart n) * (smoothPart n - j / freePart n) ∧
      freePart (n - 2) ∣ (j / freePart n) * (smoothPart n - j / freePart n) *
        (smoothPart n - 2 * (j / freePart n)) ∧
      10 * (freePart (n - 1) * freePart (n - 2)) ≤ smoothPart n ^ 3 := by
  have hn0 : n ≠ 0 := by omega
  have hn1 : n - 1 ≠ 0 := by omega
  have hn2 : n - 2 ≠ 0 := by omega
  have hjle : j ≤ n := by omega
  have hA : ∀ m, m ≠ 0 → m ∣ n * (n - 1) * (n - 2) →
      ∀ p, p.Prime → p ∣ freePart m → ¬ p ∣ n.choose j := by
    intro m hm0 hm p hp hpm
    have h5 := five_le_of_dvd_freePart hm0 hp hpm
    exact h p hp h5 (dvd_choose_three hp h5 ((hpm.trans (freePart_dvd m)).trans hm))
  have ha : freePart n ∣ j :=
    dvd_of_dvd_self hj hjle hn0 (freePart_dvd n)
      (hA n hn0 (Dvd.dvd.mul_right (dvd_mul_right n (n - 1)) (n - 2)))
  refine ⟨ha, ?_⟩
  obtain ⟨t, hja⟩ := ha
  have hapos : 0 < freePart n := Nat.pos_of_ne_zero (freePart_ne_zero hn0)
  have htdef : j / freePart n = t := by rw [hja]; exact Nat.mul_div_cancel_left t hapos
  rw [htdef]
  have hns : n = smoothPart n * freePart n := (smoothPart_mul_freePart n).symm
  have ht : 0 < t := Nat.pos_of_ne_zero (fun h0 => by rw [h0, Nat.mul_zero] at hja; omega)
  have h2ts : 2 * t < smoothPart n := by
    have e : 2 * t * freePart n = 2 * j := by rw [hja]; ring
    have hlt : 2 * t * freePart n < smoothPart n * freePart n := by
      rw [e, smoothPart_mul_freePart]; exact hjn
    exact Nat.lt_of_mul_lt_mul_right hlt
  have hB : freePart (n - 1) ∣ t * (smoothPart n - t) :=
    dvd_of_dvd_pred hjle (by omega) hns hja ht (by omega) (freePart_dvd _) (freePart_ne_zero hn1)
      (hA (n - 1) hn1 (Dvd.dvd.mul_right (dvd_mul_left (n - 1) n) (n - 2)))
  have hodd : ¬ 2 ∣ freePart (n - 2) := fun h2 => by
    have := five_le_of_dvd_freePart hn2 Nat.prime_two h2
    omega
  have hC : freePart (n - 2) ∣ t * (smoothPart n - t) * (smoothPart n - 2 * t) :=
    dvd_of_dvd_pred_two hjle (by omega) hns hja ht h2ts (freePart_dvd _) (freePart_ne_zero hn2)
      hodd (hA (n - 2) hn2 (dvd_mul_left (n - 2) (n * (n - 1))))
  have hcop : Nat.Coprime (freePart (n - 1)) (freePart (n - 2)) := by
    have h12 : Nat.Coprime (n - 1) (n - 2) := by
      have e : n - 1 = n - 2 + 1 := by omega
      rw [e]
      exact Nat.coprime_self_add_left.mpr (Nat.coprime_one_left _)
    exact Nat.Coprime.coprime_dvd_left (freePart_dvd _)
      (Nat.Coprime.coprime_dvd_right (freePart_dvd _) h12)
  exact ⟨ht, h2ts, hB, hC, size_bound_of_dvd ht h2ts hcop hB hC⟩

end Erdos699
