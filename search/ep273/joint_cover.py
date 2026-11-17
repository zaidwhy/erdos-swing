"""Erdős 273 as three coupled coverings of Z/L (notes/EP273.md), by greedy plus local moves.

Copies 1 and 3 are the odd classes mod 4, copy 2 the class 2 mod 4.
  q (odd, 2q + 1 prime): either one class in copies 1 and 3 together, or one class in copy 2.
  r (4r + 1 prime):      one class in exactly one copy.
By translation the shared q may be given the same residue in copies 1 and 3.

Reports the uncovered fraction of the three copies.  Zero would be a covering system with
moduli p - 1; anything else is only a lower bound on what this lcm can do.
"""
import sys

import numpy as np


def is_prime(n):
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True


def free_by_residue(counts, modulus):
    return (counts == 0).reshape(len(counts) // modulus, modulus).sum(axis=0)


def options(kind):
    """Sets of copies an item may be placed on."""
    return [(0, 2), (1,)] if kind == "q" else [(0,), (1,), (2,)]


def place(copies, item, sign):
    _, modulus, target, residue = item
    for c in target:
        copies[c][residue::modulus] += sign


def best_placement(copies, kind, modulus):
    best_gain, best_target, best_residue = -1, options(kind)[0], 0
    for target in options(kind):
        gain = np.sum([free_by_residue(copies[c], modulus) for c in target], axis=0)
        residue = int(gain.argmax())
        if int(gain[residue]) > best_gain:
            best_gain, best_target, best_residue = int(gain[residue]), target, residue
    return best_target, best_residue


def main():
    period = int(sys.argv[1])
    rounds = int(sys.argv[2]) if len(sys.argv) > 2 else 8
    items = []
    for d in range(3, period + 1):
        if period % d:
            continue
        if d % 2 == 1 and is_prime(2 * d + 1):
            items.append(("q", d))
        if is_prime(4 * d + 1):
            items.append(("r", d))
    items.sort(key=lambda it: it[1])
    copies = [np.zeros(period, dtype=np.int16) for _ in range(3)]
    placed = []
    for kind, modulus in items:
        target, residue = best_placement(copies, kind, modulus)
        placed.append((kind, modulus, target, residue))
        place(copies, placed[-1], 1)
    left = [sum(int((c == 0).sum()) for c in copies)]
    for _ in range(rounds):
        for k, item in enumerate(placed):
            place(copies, item, -1)
            target, residue = best_placement(copies, item[0], item[1])
            placed[k] = (item[0], item[1], target, residue)
            place(copies, placed[k], 1)
        left.append(sum(int((c == 0).sum()) for c in copies))
        if left[-1] == left[-2]:
            break
    q_sum = sum(1 / m for kind, m in items if kind == "q")
    r_sum = sum(1 / m for kind, m in items if kind == "r")
    per_copy = [round(float((c == 0).mean()), 4) for c in copies]
    print(f"L={period} q={sum(k == 'q' for k, _ in items)} r={sum(k == 'r' for k, _ in items)} "
          f"S_R+2S_Q={r_sum + 2 * q_sum:.3f} uncovered={left[-1] / (3 * period):.4%} "
          f"per copy={per_copy} rounds={len(left) - 1} "
          f"q on even side={sum(1 for it in placed if it[0] == 'q' and it[2] == (1,))}")
    if left[-1] == 0:
        print("COVERING FOUND", placed)


main()
