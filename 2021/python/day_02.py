"""Solves day 02 of advent of code"""
from enum import Enum
from collections import Counter
from dataclasses import dataclass

from lib import input_to_list_str


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

@dataclass
class Position:
    x_pos: int = 0
    y_pos: int = 0
    aim: int = 0

    def move(self, command: Command):
        """Changes the position according to `self.aim` and `command`"""
        # Totally using `match` to use match

        match command.direction:
            case Direction.DOWN:
                self.aim += command.distance
            
            case Direction.UP: 
                self.aim -= command.distance

            case Direction.FORWARD:
                self.x_pos += command.distance
                self.y_pos += command.distance * self.aim
    


def solve_part_a():
    """Solves the first halve of the puzzle"""
    directions = Counter()  # Counts the number for each direction
    commands = input_to_list_str("day_02.txt")
    commands = [Command(command) for command in commands]

    for command in commands:
        directions += Counter({command.direction: command.distance})

    x_pos = directions.get(Direction.FORWARD, 0)
    y_pos = directions.get(Direction.DOWN, 0) - directions.get(Direction.UP, 0)

    print(f"Part A: {x_pos * y_pos}")  # Solution: 2070300;

    pass


def solve_part_b():
    """Solves the second halve of the puzzle"""
    position = Position()
    commands = input_to_list_str("day_02.txt")
    commands = [Command(command) for command in commands]
    for command in commands:
        position.move(command)

    print(f"Part B: {position.x_pos * position.y_pos}")  # Solution: 2078985210


if __name__ == "__main__":
    solve_part_a()
    solve_part_b()
