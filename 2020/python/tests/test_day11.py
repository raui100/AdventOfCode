from pathlib import Path

from ..day11 import GameOfLife, Seat
from pytest import fixture


@fixture
def initial_field(tmp_path) -> Path:
    """Produces the initial gaming field"""
    file = tmp_path / "game_of_life.txt"
    file.write_text(
        """#.LL.LL.LL
.X.#LLL.LL
#.L.L..L..
L#LL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"""
    )

    return file


@fixture
def game_of_life(initial_field) -> GameOfLife:
    return GameOfLife(initial_field)


class TestGameOfLife:
    """Tests the class `Game of life`"""

    def test_game_of_life(self, game_of_life):
        """Tests the game of life fixture"""
        assert game_of_life.count_occupied_seat() == 4
        assert game_of_life.width == 10

    def test_get_seat_by_direction(self, game_of_life):
        assert game_of_life.get_seat_by_direction(11, "N") == Seat.BORDER.value
        assert game_of_life.get_seat_by_direction(11, "NE") == Seat.EMPTY.value
        assert game_of_life.get_seat_by_direction(11, "E") == Seat.OCCUPIED.value
        assert game_of_life.get_seat_by_direction(11, "SE") == Seat.EMPTY.value
        assert game_of_life.get_seat_by_direction(11, "S") == Seat.OCCUPIED.value
        assert game_of_life.get_seat_by_direction(11, "SW") == Seat.OCCUPIED.value
        assert game_of_life.get_seat_by_direction(11, "W") == Seat.BORDER.value
        assert game_of_life.get_seat_by_direction(11, "NW") == Seat.OCCUPIED.value
