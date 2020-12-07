from lib.paths import get_day
import re


def decode(seat: str) -> int:
    """Decodes the binary representation the seat ID to a base 10 integer one"""
    seat = re.sub("[BR]", "1", seat)  # Replaces "B" or "R" with 1
    seat = re.sub("[FL]", "0", seat)  # Replaces "F" or "L" with 0
    seat_id = int("0b" + seat, 2)  # Casts binary representation to integer base 10

    return seat_id


data = get_day(5).read_text().splitlines()
print(max(map(decode, data)))
