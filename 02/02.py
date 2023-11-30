from pathlib import Path


# Rock (A/X):   1 point
# Paper (B/Y):  2 points
# Scisor (C/Z): 3 points
# Win: 6 points
# Draw: 3 points
# Lose: 0 points


def part_one(fp: Path):
    me_to_o = {"X": "A", "Y": "B", "Z": "C"}

    def result(opponent: str, me: str) -> int:
        if opponent == me_to_o[me]:
            return 3
        if opponent == "A" and me == "Z":
            return 0
        if opponent == "B" and me == "X":
            return 0
        if opponent == "C" and me == "Y":
            return 0
        return 6

    shape_val = {"X": 1, "Y": 2, "Z": 3}
    total = 0
    for line in fp.open().readlines():
        o, m = line.rstrip().split()
        r = result(o, m)
        val = shape_val[m]
        total += r + val
    print(total)


def part_two(fp: Path):
    total = 0
    for o, s in [x.rstrip().split() for x in fp.open().readlines()]:
        direction = ord(s) - 89
        move = (ord(o) - 65 + direction) % 3
        move_val = move + 1
        str_val = direction * 3 + 3
        total += move_val + str_val
    print(total)


filepath = Path("./02.txt")
# part_one(fp)
part_two(filepath)
