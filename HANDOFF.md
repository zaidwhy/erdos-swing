# HANDOFF (2026-11-18, 02:40)

## State

- No Erdős problem resolved. README.md has the honest summary; notes/TRIAGE.md has the screening.
- **#699 all-i scan is finished**: no counterexample for 2 <= n < 1e11. `results/ep699-weak-1e11.jsonl`
  has 1000 contiguous blocks from 2 to 100,000,000,000, mode weak, every `found` list empty,
  87,870,299,149 (n, i) pairs tested. Command, for a rerun (it skips finished blocks):
  `search/ep699/target/release/ep699.exe weak scan 2 100000000000 4 results/ep699-weak-1e11.jsonl`
- Local git only. No remote, nothing pushed, nothing posted.

- Finished: `python search/ep699/spot_check.py weak 10000000 40000000000 40 300 699` checked 11,489
  composite n in 40 random windows and found nothing (128 minutes). Still running, single thread: the
  R-only #273 search writing `results/ep273-r-only.txt` (lcm 15,120 and 30,240 gave up; 60,480 and
  151,200 pending).

## Check progress

```
python -c "import json;R=[json.loads(l) for l in open('results/ep699-weak-1e11.jsonl')];print(len(R),'blocks, found',sum(len(r['found']) for r in R))"
```

A non-empty `found` list is a claimed counterexample (n, i, j). Before believing it, confirm with exact
integers: `python -c "from math import comb,gcd;print(gcd(comb(n,i),comb(n,j)))"` and factor the gcd.

## Open decision

Zaid decides whether to post the #699 results on the forum thread. He writes that comment himself.
The site requires an AI-use disclosure on comments.

## Next steps, in order of value

1. **#699 structured search: done** for i <= 4 to 1e18, i <= 6 to 1e17, i <= 10 to 1e15. The second
   program (`search/ep699_verify`) confirms 1e18, 1e15 and 1e12 respectively; the 1e17 and 1e15 extensions
   are single-program runs with unchanged survivor lists (5 and 64 pairs, all n <= 4376).
2. **#273.** `notes/EP273.md` has the reduction to three coupled coverings. Next experiment: search the
   coupled problem directly (shared odd q, three disjoint sets of r) at lcm above 2e6 in w-terms.
3. Lean: not installed (disk). `lean/Erdos699Reduction.lean` holds the statement and proof route with
   `sorry` in every proof. It has NEVER been compiled; treat every Mathlib name in it as unconfirmed.
   Informal proofs of every lemma, written to be formalised step by step: `notes/EP699-PROOFS.md`.
   When about 7 GB is free: install elan, add Mathlib, compile, then fill the proofs.

## Peer note

Folder claimed in peer-sessions as `c:/users/asus/projects/erdos-swing`. `upstream/` is gitignored
third-party content (database clone, fetched pages); do not commit it.
