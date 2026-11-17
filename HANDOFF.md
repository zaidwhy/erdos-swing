# HANDOFF (2026-11-18, 01:30)

## State

- No Erdős problem resolved. README.md has the honest summary; notes/TRIAGE.md has the screening.
- **#699 scan is running in the background** (8 threads, started about 00:50):
  `search/ep699/target/release/ep699.exe weak scan 2 100000000000 8 results/ep699-weak-1e11.jsonl`
  At 01:26 it had 146 of 1000 blocks, contiguous to n = 14,600,000,002, zero counterexamples.
  About 4 blocks a minute, so roughly 3.5 hours left if the machine stays this loaded.
- If the process died: rerun the same command. It reads the JSONL and skips finished blocks.
- Local git only. No remote, nothing pushed, nothing posted.

## Check progress

```
python -c "import json;R=[json.loads(l) for l in open('results/ep699-weak-1e11.jsonl')];print(len(R),'blocks, found',sum(len(r['found']) for r in R))"
```

A non-empty `found` list is a claimed counterexample (n, i, j). Before believing it, confirm with exact
integers: `python -c "from math import comb,gcd;print(gcd(comb(n,i),comb(n,j)))"` and factor the gcd.

## When the scan finishes

1. Confirm 1000 blocks, every `lo` equal to 2 + k * 1e8, `found` empty everywhere.
2. Put the final sentence in README.md: "no counterexample for n < 1e11".
3. Zaid decides whether to post it on the #699 forum thread. He writes that comment himself.
   The site requires an AI-use disclosure on comments.

## Next steps, in order of value

1. **#699, i = 3 only, to 1e18.** A recent forum proof claim says a counterexample can only have i = 3
   (or lie in a finite set). For i = 3 write n = s * A with s the 6-smooth part. Then A divides j, and
   with j = t * A, B the 6-free part of n - 1 and C the 6-free part of n - 2:
   B | t(s - t) and C | t(s - t)(s - 2t), so B * C < 0.0963 * s^3.
   That leaves about sqrt(s) candidates n per smooth s, so 1e18 is roughly 3e7 values of n, each
   needing a factorisation of n - 1 and n - 2 (Pollard rho). Not written yet.
2. **#273 odd half.** Needs a covering of the odd integers with moduli p - 1 other than 4 and other
   than whatever the even half uses. The even half in `search/ep273/half_cover.py` is too expensive
   (0.852); a cheaper even half is the first thing to look for.
3. Lean: only worth installing once there is a statement to check. Mathlib needs about 6 GB.

## Peer note

Folder claimed in peer-sessions as `c:/users/asus/projects/erdos-swing`. `upstream/` is gitignored
third-party content (database clone, fetched pages); do not commit it.
