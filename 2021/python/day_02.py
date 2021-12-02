"""Solves day 02 of advent of code"""

from lib import input_to_list_str
from typing import NamedTuple
from enum import Enum


class Direction(Enum):
    FORWARD = "forward"
    UP = "up"
    DOWN = "down"


class Command:
    """Models a command"""
    def __init__(self, command: str):
        # Splits command (eg: "forward 2") in it's two components
        direction, distance = command.split(" ")

        self.direction: Direction = Direction(direction)
        self.distance: int = int(distance)

def solve_part_a():
    """Solves the first halve of the puzzle"""
    directions: dict[str, int] = {
        Direction.FORWARD: 0,
        Direction.UP: 0, 
        Direction.DOWN: 0,}

    commands = input_to_list_str("day_02.txt")
    commands = [Command(command) for command in commands]

    for command in commands:
        directions[command.direction] += command.distance

    x_pos = directions[Direction.FORWARD]
    y_pos = directions[Direction.UP] - directions[Direction.DOWN]

    print(x_pos * y_pos)

    pass

def solve_part_b():
    """Solves the second halve of the puzzle"""
    pass

if __name__ == "__main__":
    solve_part_a()
    solve_part_b()
