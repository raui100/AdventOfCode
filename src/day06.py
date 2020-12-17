from typing import List, Set

from lib.paths import get_day
import re


def intersecting_entries(group: str) -> int:
    """Counts the number of chars that are in every entry of the list"""
    entries = group.splitlines()  # Splits the group into their members
    entries: List[str] = [
        re.sub(r"[^a-z]", "", entry) for entry in entries
    ]  # Deletes every char that is not a-z
    entries: List[Set[str]] = [set(entry) for entry in entries]

    return len(set.intersection(*entries))


if __name__ == "__main__":
    data: List[str] = get_day(6).read_text().split("\n\n")  # Splits data into groups
    print(sum([intersecting_entries(group) for group in data]))
