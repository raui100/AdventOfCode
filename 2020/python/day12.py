from enum import Enum
from typing import NamedTuple, Dict, List


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


class Waypoint:
    def __init__(self, north=1, east=10):
        self.north = north
        self.east = east

    def rotate(self, rotation: int):

        if rotation == 0:
            pass

        elif rotation == 90:
            self.north, self.east = -self.east, self.north

        elif rotation == 180:
            self.north, self.east = -self.north, -self.east

        elif rotation == 270:
            self.north, self.east = self.east, -self.north

        else:
            raise NotImplementedError(f"Can't rotate {rotation}°")

    def move(self, movement: Movement):
        if movement.is_north:
            self.north += movement.distance

        elif movement.is_east:
            self.east += movement.distance

        elif movement.is_south:
            self.north -= movement.distance

        elif movement.is_west:
            self.east -= movement.distance

        elif movement.is_left:
            self.rotate(
                -movement.distance % 360
            )  # Casting counterclockwise rotation to clockwise rotation

        elif movement.is_right:
            self.rotate(movement.distance % 360)

    def move_forward(self, movement: Movement) -> List[str]:
        _dist = movement.distance * self.north
        north = "N" + str(_dist) if _dist >= 0 else "S" + str(abs(_dist))

        _dist = movement.distance * self.east
        east = "E" + str(_dist) if _dist >= 0 else "W" + str(abs(_dist))
        return [north, east]


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


class Navigator:
    """Adapter: Navigates `Ship`"""

    def __init__(self):
        self.waypoint = Waypoint()
        self.ship = Ship()

    north = property(lambda self: self.ship.north)
    east = property(lambda self: self.ship.east)

    def move(self, line):
        movement = Ship.parse_movement(line)

        if movement.is_forward:
            for _movement in self.waypoint.move_forward(movement):
                self.ship.move(_movement)
        else:
            self.waypoint.move(movement)

    def determine_manhattan_distance(self) -> int:
        return self.ship.determine_manhattan_distance()


if __name__ == "__main__":
    from lib.paths import get_day

    data = get_day(12).read_text().splitlines()
    ship = Navigator()
    for line in data:
        ship.move(line)

    print(ship.determine_manhattan_distance())
