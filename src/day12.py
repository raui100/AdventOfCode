from enum import Enum
from typing import NamedTuple, Dict


class Direction(Enum):
    NORTH = "N"
    EAST = "E"
    SOUTH = "S"
    WEST = "W"
    LEFT = "L"
    RIGHT = "R"
    FORWARD = "F"


class Movement(NamedTuple):
    direction: Direction
    distance: int  # Moving this far into `direction`

    is_north = property(lambda self: self.direction == Direction.NORTH)
    is_east = property(lambda self: self.direction == Direction.EAST)
    is_south = property(lambda self: self.direction == Direction.SOUTH)
    is_west = property(lambda self: self.direction == Direction.WEST)
    is_left = property(lambda self: self.direction == Direction.LEFT)
    is_right = property(lambda self: self.direction == Direction.RIGHT)
    is_forward = property(lambda self: self.direction == Direction.FORWARD)


class Ship:
    DIRECTIONS: Dict[int, Direction] = {
        0: Direction.NORTH,
        90: Direction.EAST,
        180: Direction.SOUTH,
        270: Direction.WEST,
    }

    def __init__(self):
        self.direction = 90
        self.north = 0  # Position on the x-axis
        self.east = 0  # Position on the y-axis

    @staticmethod
    def parse_movement(line) -> Movement:
        """Parses a line into an `Movement`. E.g: F10 -> Instruction(Direction.FORWARD, 10)"""
        direction = Direction(line[:1])
        distance = int(line[1:])

        return Movement(direction, distance)

    def move(self, line: str):
        movement = self.parse_movement(line)

        if movement.is_forward:
            direction = self.DIRECTIONS[self.direction]
            movement = Movement(direction, movement.distance)

        if movement.is_north:
            self.north += movement.distance

        elif movement.is_east:
            self.east += movement.distance

        elif movement.is_south:
            self.north -= movement.distance

        elif movement.is_west:
            self.east -= movement.distance

        elif movement.is_left:
            self.direction = abs((self.direction - movement.distance) % 360)

        elif movement.is_right:
            self.direction = abs((self.direction + movement.distance) % 360)

    def determine_manhattan_distance(self) -> int:
        return abs(self.north) + abs(self.east)


if __name__ == "__main__":
    from lib.paths import get_day

    data = get_day(12).read_text().splitlines()
    ship = Ship()
    for line in data:
        ship.move(line)

    print(ship.determine_manhattan_distance())
