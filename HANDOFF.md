# HANDOFF (2026-11-18, 10:50)

## State

- No Erdős problem resolved. README.md has the honest summary; notes/TRIAGE.md has the screening.
- **#699 all-i scan is finished**: no counterexample for 2 <= n < 1e11. `results/ep699-weak-1e11.jsonl`
  has 1000 contiguous blocks from 2 to 100,000,000,000, mode weak, every `found` list empty,
  87,870,299,149 (n, i) pairs tested. Command, for a rerun (it skips finished blocks):
  `search/ep699/target/release/ep699.exe weak scan 2 100000000000 4 results/ep699-weak-1e11.jsonl`
- Pushed to github.com/zaidwhy/erdos-swing (public since 2026-11-18, branch main).
- Nothing posted on the Erdős site. Zaid approved one comment on the #699 thread, but the thread page
  (read 2026-11-18) says "The ability to post new comments has been suspended." Its newest comment is
  from 25 Jul 2026 and it does not mention the 1e12 sweep or any bound for i = 4, 5, 6, 8.
  Decision (Zaid, 2026-11-18): wait until the thread reopens. The earlier one-time approval to post
  does not carry over; ask him again then, and re-read the thread first.
- Prior work found on 2026-11-18 (README, "Prior work on 699"): the 1e11 sweep is superseded by a
  public 1e12 sweep; only the i = 4, 5, 6, 8 ranges may be new.

- Finished: `python search/ep699/spot_check.py weak 10000000 40000000000 40 300 699` checked 11,489
  composite n in 40 random windows and found nothing (128 minutes). The R-only #273 search also finished:
  gave up (15,000,000 nodes) at all four lcms, see `results/ep273-r-only.txt`. No background jobs remain.

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
3. Lean: `lean/Erdos699Reduction.lean` proves `Erdos699.reduction` (no `sorry`). It is compiled by the
   `lean` GitHub Actions workflow (Mathlib v4.36.0, manifest committed), not locally; Lean is still not
   installed on this machine. Check a change with `gh run list --workflow lean` after pushing.
   Possible next step: generalise from {2,3} to a smooth set P (notes/EP699-I3.md).

## Peer note

Folder claimed in peer-sessions as `c:/users/asus/projects/erdos-swing`. `upstream/` is gitignored
third-party content (database clone, fetched pages); do not commit it.
