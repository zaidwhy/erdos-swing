# HANDOFF (2026-11-18, 02:40)

## State

- No Erdős problem resolved. README.md has the honest summary; notes/TRIAGE.md has the screening.
- **#699 scan is running in the background** (8 threads, started about 00:50):
  `search/ep699/target/release/ep699.exe weak scan 2 100000000000 8 results/ep699-weak-1e11.jsonl`
  At 02:38 it had 278 of 1000 blocks, zero counterexamples. About 2 blocks a minute while the
  i = 3 run shared the CPU; expect 3 to 6 more hours.
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

1. **#699, i = 3: done to 1e18** (`notes/EP699-I3.md`, zero survivors). Possible extension: the same
   reduction for i = 4, where the smooth part only has to be about n^(1/3), so 1e12 is realistic.
2. **#273 odd half.** Needs a covering of the odd integers with moduli p - 1 other than 4 and other
   than whatever the even half uses. The even half in `search/ep273/half_cover.py` is too expensive
   (0.852); a cheaper even half is the first thing to look for.
3. Lean: only worth installing once there is a statement to check. Mathlib needs about 6 GB.

## Peer note

Folder claimed in peer-sessions as `c:/users/asus/projects/erdos-swing`. `upstream/` is gitignored
third-party content (database clone, fetched pages); do not commit it.
