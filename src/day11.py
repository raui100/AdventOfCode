"""Conways Game of Life"""
from enum import Enum
from pathlib import Path
from typing import List


class Seat(Enum):
    """Models the different field types"""

    EMPTY = "L"
    FLOOR = "."
    OCCUPIED = "#"


class GameOfLife:
    """Models Conways Game of life"""

    def __init__(self, path: Path):
        data = path.read_text().splitlines()
        self.data: str = "".join(data)
        self.width = len(data[0])  # Determines width of the field

    def is_in_boundary(self, index):
        """Checks if an index is within the boundary of the array"""
        return 0 <= index < len(self.data)

    def get_adjacent_indices(self, index) -> List[int]:
        """Determines the indices that are adjacent to a certain index"""
        indices = [  # All 8 adjacent positions around the index
            index - self.width - 1,  # Upper left corner
            index - self.width + 0,  # Upper center
            index - self.width + 1,  # Upper right corner
            index - 1,  # Left
            index + 1,  # Right
            index + self.width - 1,  # Lower left corner
            index + self.width + 0,  # Lower center
            index + self.width + 1,  # Lower right corner
        ]

        if index % self.width == 0:  # The seat is on the left hand side of the field
            indices[0], indices[3], indices[5] = -1, -1, -1  # These values are filtered later on

        if index % self.width == self.width - 1:  # Right hand side of the field
            indices[2], indices[4], indices[7] = -1, -1, -1  # These values are filtered later on

        if index < self.width:  # First Row
            indices[0], indices[1], indices[2] = -1, -1, -1  # These values are filtered later on

        if index >= len(self.data) - self.width:  # Last row
            indices[5], indices[6], indices[7] = -1, -1, -1  # These values are filtered later on

        # Filters indices that don't exist. E.g: There is no row above the first row
        indices = [index for index in indices if self.is_in_boundary(index)]

        return indices

    def count_occupied_adjacent(self, index: int) -> int:
        """Counts the number of adjacent positions that are occupied"""
        count = 0
        for ind in self.get_adjacent_indices(index):
            if self.data[ind] == Seat.OCCUPIED.value:
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
                char == Seat.OCCUPIED.value and self.count_occupied_adjacent(index) >= 4
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
