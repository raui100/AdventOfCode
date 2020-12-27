from pathlib import Path

from src.day11 import GameOfLife
from pytest import fixture


@fixture
def initial_field(tmp_path) -> Path:
    """Produces the initial gaming field"""
    file = tmp_path / "game_of_life.txt"
    file.write_text(
        """L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
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
        assert game_of_life.count_occupied_seat() == 0
        assert game_of_life.width == 10

    def test_is_in_boundary(self, game_of_life):
        assert not game_of_life.is_in_boundary(-1)
        assert game_of_life.is_in_boundary(0)
        assert not game_of_life.is_in_boundary(100)

    def test_get_adjacent_indices(self, game_of_life):
        # General case, there are 8 adjacent indices
        assert game_of_life.get_adjacent_indices(11) == [0, 1, 2, 10, 12, 20, 21, 22]
        # Edge Cases
        assert game_of_life.get_adjacent_indices(0) == [1, 10, 11]  # Upper left corner
        assert game_of_life.get_adjacent_indices(9) == [8, 18, 19]  # Upper right corner
        assert game_of_life.get_adjacent_indices(90) == [80, 81, 91]  # Lower left corner
        assert game_of_life.get_adjacent_indices(99) == [88, 89, 98]  # Lower right corner
