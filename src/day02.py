from contextlib import suppress

from lib.paths import data_directory
import re


data = (data_directory / "day02.txt").read_text().split("\n")  # Splits the file at every newline
regex = re.compile(r"(\d+)-(\d+) (\w): (\w+)")
valid_password = 0
for entry in data:
    match = regex.match(entry)
    with suppress(AttributeError):
        minimum = int(match.group(1))
        maximum = int(match.group(2))
        validator = match.group(3)
        password = match.group(4)

        if minimum <= password.count(validator) <= maximum:
            valid_password += 1

print(valid_password)

