from ..day12 import Direction, Movement, Ship, Waypoint, Navigator


def get_ship() -> Ship:

    return Ship()


def test_movement():
    assert Movement(Direction.NORTH, 0).is_north
    assert Movement(Direction.EAST, 0).is_east
    assert Movement(Direction.SOUTH, 0).is_south
    assert Movement(Direction.WEST, 0).is_west
    assert Movement(Direction.LEFT, 0).is_left
    assert Movement(Direction.RIGHT, 0).is_right
    assert Movement(Direction.FORWARD, 0).is_forward


def test_ship():
    ship = get_ship()
    assert ship.direction == 90  # Ships starts facing east
    assert ship.north == ship.east == 0  # Ship starts at (0, 0)

    ship.move("N90")
    assert ship.north == 90 and ship.east == 0

    ship.move("E90")
    assert ship.north == 90 and ship.east == 90

    ship.move("S90")
    assert ship.north == 0 and ship.east == 90

    ship.move("W90")
    assert ship.north == 0 and ship.east == 0

    ship.move("L90")  # Ship looks NORTH
    ship.move("F90")
    assert ship.north == 90 and ship.east == 0

    ship.move("L90")  # Ship looks WEST
    ship.move("F90")
    assert ship.north == 90 and ship.east == -90

    ship.move("R270")  # Ship looks South
    ship.move("F90")
    assert ship.north == 0 and ship.east == -90

    ship.move("L90")  # Ship looks EAST
    ship.move("F90")
    assert ship.north == 0 and ship.east == 0


def test_waypoint():
    wp = Waypoint()
    assert wp.north == 1
    assert wp.east == 10

    wp.rotate(90)
    assert wp.north == -10 and wp.east == 1

    wp.rotate(180)
    assert wp.north == 10 and wp.east == -1

    wp.rotate(270)
    assert wp.north == -1 and wp.east == -10


def test_navigator():
    nav = Navigator()
    assert nav.determine_manhattan_distance() == 0

    nav.move("N10")
    assert nav.north == 0 and nav.east == 0
    assert nav.waypoint.north == 11 and nav.waypoint.east == 10

    nav.move("E10")
    assert nav.north == 0 and nav.east == 0
    assert nav.waypoint.north == 11 and nav.waypoint.east == 20

    nav.move("S10")
    assert nav.north == 0 and nav.east == 0
    assert nav.waypoint.north == 1 and nav.waypoint.east == 20

    nav.move("W10")
    assert nav.north == 0 and nav.east == 0
    assert nav.waypoint.north == 1 and nav.waypoint.east == 10

    nav.move("F10")
    assert nav.north == 10 and nav.east == 100
    assert nav.waypoint.north == 1 and nav.waypoint.east == 10
