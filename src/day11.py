"""Conways Game of Life"""
from enum import Enum, auto
from pathlib import Path
from typing import List, Generator


class Seat(Enum):
    """Models the different field types"""

    EMPTY = "L"
    FLOOR = "."
    OCCUPIED = "#"
    BORDER = " "


class GameOfLife:
    """Models Conways Game of life"""

    DIRECTIONS = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"]

    def __init__(self, path: Path):
        data = path.read_text().splitlines()
        self.data: str = "".join(data)
        self.width = len(data[0])  # Determines width of the field

    def get_seat_by_direction(self, index: int, direction: str) -> str:
        index += -self.width if "N" in direction else 0
        index += +1 if "E" in direction else 0
        index += +self.width if "S" in direction else 0
        index += -1 if "W" in direction else 0

        if (
            index < 0  # Exceeded Northern Border
            or index >= len(self.data)  # South
            or ("W" in direction and index % self.width == self.width - 1)  # East
            or ("E" in direction and index % self.width == 0)  # West
        ):  # Exceeded upper or lower border
            return Seat.BORDER.value

        if self.data[index] == Seat.FLOOR.value:

            return self.get_seat_by_direction(index, direction)

        else:

            return self.data[index]

    def count_occupied_adjacent(self, index: int) -> int:
        """Counts the number of adjacent positions that are occupied"""
        count = 0
        for direction in self.DIRECTIONS:
            if self.get_seat_by_direction(index, direction) == Seat.OCCUPIED.value:
                count += 1

        return count

    def iterate_field(self):
        """Applies the rules of the game simultaneously to every seat"""
        new_data = ""
        for index, char in enumerate(self.data):
            # Floor can't be changed
            if char == Seat.FLOOR.value:
                new_data += Seat.FLOOR.value

            # An empty field with no adjacent occupied fields becomes occupied
            elif char == Seat.EMPTY.value and self.count_occupied_adjacent(index) == 0:
                new_data += Seat.OCCUPIED.value

            # An occupied field with 4 or more adjacent occupied fields becomes empty
            elif (
                char == Seat.OCCUPIED.value and self.count_occupied_adjacent(index) >= 5
            ):
                new_data += Seat.EMPTY.value

            # The field doesn't change
            else:
                new_data += self.data[index]

        self.data = new_data  # Replaces the of data with the new data

    def count_occupied_seat(self):
        return self.data.count(Seat.OCCUPIED.value)


if __name__ == "__main__":
    from lib.paths import get_day

    game = GameOfLife(get_day(11))
    iteration = 0
    while True:
        occupied_seats = game.count_occupied_seat()
        game.iterate_field()
        iteration += 1
        print(
            f"Iteration: {str(iteration).zfill(3)} -> Delta: {game.count_occupied_seat() - occupied_seats}"
        )
        if occupied_seats == game.count_occupied_seat():  # Nothing changes anymore
            print(occupied_seats)
            break
