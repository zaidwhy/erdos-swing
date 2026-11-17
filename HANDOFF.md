# HANDOFF (2026-11-18, 02:40)

## State

- No Erdős problem resolved. README.md has the honest summary; notes/TRIAGE.md has the screening.
- **#699 scan is running in the background** (restarted on 4 threads to leave CPU for other sessions):
  `search/ep699/target/release/ep699.exe weak scan 2 100000000000 4 results/ep699-weak-1e11.jsonl`
  At the restart it had 412 of 1000 blocks, zero counterexamples. Expect many hours at 4 threads.
- If the process died: rerun the same command. It reads the JSONL and skips finished blocks.
- Local git only. No remote, nothing pushed, nothing posted.

- Also running, single thread each: `python search/ep699/spot_check.py weak 10000000 40000000000 40 300 699`
  (independent Python re-check of 40 random windows of 300 inside the swept range; its strict-mode
  control already reproduced the 8 known failures below 3000) and the R-only #273 search writing
  `results/ep273-r-only.txt`. Record both outcomes in the notes when they finish.

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

1. **#699 structured search: done** for i <= 4 to 1e18, i <= 6 to 1e15, i <= 10 to 1e12, confirmed by a
   second program written from the spec (`search/ep699_verify`). Still running when this was written:
   `ep699_i3 scan 1e15 4 2,3,5,7` then `scan 1e17 4 2,3,5`, writing `results/ep699-smooth2357-1e15.json`
   and `results/ep699-smooth235-1e17.json`. An empty file means that run has not finished. When they
   land, add the rows to the table in `notes/EP699-I3.md`; any survivor with n above 1e11 would need an
   exact check like the one described there.
2. **#273.** `notes/EP273.md` has the reduction to three coupled coverings. Next experiment: search the
   coupled problem directly (shared odd q, three disjoint sets of r) at lcm above 2e6 in w-terms.
3. Lean: not installed (disk). `lean/Erdos699Reduction.lean` holds the statement and proof route with
   `sorry` in every proof. It has NEVER been compiled; treat every Mathlib name in it as unconfirmed.
   Informal proofs of every lemma, written to be formalised step by step: `notes/EP699-PROOFS.md`.
   When about 7 GB is free: install elan, add Mathlib, compile, then fill the proofs.

## Peer note

Folder claimed in peer-sessions as `c:/users/asus/projects/erdos-swing`. `upstream/` is gitignored
third-party content (database clone, fetched pages); do not commit it.
