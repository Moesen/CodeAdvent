from pathlib import Path
from typing import Optional, Tuple

fp = Path("./01.txt")

total = 0

digit_map = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
    "zero": "0",
}


def check_for_digit(segment: str) -> list[str]:
    return [digit_map[k] for k in digit_map.keys() if segment[: len(k)] == k]


def get_digits(line: str, window_len: int = 5) -> list[str]:
    digits = []
    for i in range(len(line)):
        end = min(i + window_len, len(line))
        if line[i].isdigit():
            digits.append(line[i])
        digits.extend(check_for_digit(line[i:end]))
    return digits


total = 0
for line in fp.open().readlines():
    line = line.rstrip()
    digits = get_digits(line)
    if len(digits) == 1:
        total += int(digits[0] + digits[0])
    elif len(digits) > 1:
        total += int(digits[0] + digits[-1])
print(total)
