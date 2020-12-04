from collections import namedtuple
from math import prod

from lib.paths import data_directory


def encounters(steps_right: int, steps_down: int):
    """Determines encounters with the given parameters"""
    position = namedtuple("Position", "x y")
    positions = [position(x_pos, (steps_right * x_pos) % map_len) for x_pos in range(0, len(data), steps_down)]
    return sum(map(lambda pos: data[pos.x][pos.y] == "#", positions))


data = (data_directory / "day03.txt").read_text().split("\n")  # Splits the file at every newline
data.pop()  # The last line of data is an empty string
map_len = len(data[0])

lst_encounters = [encounters(right, down) for right, down in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]]
print(prod(lst_encounters))
