from typing import List

from .lib.paths import get_day
import re


def decode(seat: str) -> int:
    """Decodes the binary representation of the seat ID to a base 10 integer one"""
    seat = re.sub("[BR]", "1", seat)  # Replaces "B" or "R" with 1
    seat = re.sub("[FL]", "0", seat)  # Replaces "F" or "L" with 0
    seat = int("0b" + seat, 2)  # Casts binary representation to integer base 10

    return seat


if __name__ == "__main__":
    data: List[str] = (
        get_day(5).read_text().splitlines()
    )  # Reads the data as list of lines
    data: List[int] = list(map(decode, data))  # Decodes the data to base 10 integer

    for seat_id in range(1, max(data)):
        if seat_id - 1 in data and seat_id not in data and seat_id + 1 in data:
            print(seat_id)
