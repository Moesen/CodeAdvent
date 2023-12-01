from pathlib import Path
import re

pat = re.compile(r"\d")

elves = []

fp = Path("./01.txt")
with fp.open() as f:
    c = 0
    for line in f.readlines():
        if not pat.match(line):
            elves.append(c)
            c = 0
            continue
        c += int(line)

print(sum(sorted(elves, reverse=True)[:3]))
