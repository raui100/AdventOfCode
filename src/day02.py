from contextlib import suppress

from lib.paths import get_day
import re


if __name__ == "__main__":
    data = get_day(2).read_text().split("\n")  # Splits the file at every newline
    regex = re.compile(r"(\d+)-(\d+) (\w): (\w+)")
    valid_password = 0
    for entry in data:
        match = regex.match(entry)
        with suppress(AttributeError):
            index_1 = (
                int(match.group(1)) - 1
            )  # Cast one index to the glorious zero index
            index_2 = (
                int(match.group(2)) - 1
            )  # Cast one index to the glorious zero index
            character = match.group(3)
            password = match.group(4)

            if (
                sum((password[index_1] == character, password[index_2] == character))
                == 1
            ):
                valid_password += 1

    print(valid_password)
