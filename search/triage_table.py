"""Compact one-line-per-problem table from the mirrored pages, for choosing targets."""
import glob, html, re, sys, yaml

meta = {p["number"]: p for p in yaml.safe_load(open("erdosproblems/data/problems.yaml", encoding="utf8"))}
rows = []
for path in glob.glob("pages/*.html"):
    num = path.replace("\\", "/").split("/")[-1][:-5]
    page = open(path, encoding="utf8", errors="replace").read()
    start = page.find('<div id="content">')
    if start < 0:
        continue
    body = re.sub(r"<script.*?</script>", "", page[start:], flags=re.S)
    text = html.unescape(re.sub(r"\s+", " ", re.sub(r"<[^>]+>", " ", body)))
    comments = re.search(r"Comments \((\d+)\)", text)
    claims = re.search(r"Proof claims \((\d+)\)", text)
    statement = re.split(r"#\d+ : ", text)[0].strip()
    remarks = text.split("Proof expositions")[0]
    remarks = re.sub(r"^.*?#\d+ : (\[[^\]]*\] ?)+", "", remarks)
    rows.append((int(num), meta[num]["status"]["state"], ",".join(meta[num]["tags"]),
                 int(comments.group(1)) if comments else -1, int(claims.group(1)) if claims else -1,
                 statement, remarks.strip()))
rows.sort()
width = int(sys.argv[1]) if len(sys.argv) > 1 else 400
with open("triage.tsv", "w", encoding="utf8") as out:
    for num, state, tags, comments, claims, statement, remarks in rows:
        out.write(f"{num}\t{state}\t{tags}\tc{comments}\tp{claims}\t{statement[:width]}\t{remarks[:width]}\n")
print(len(rows))
