from typing import NamedTuple

from src.lib.solution import SolutionABC


class Move(NamedTuple):
    start: int
    end: int
    number: int


class Solution(SolutionABC):
    def _solve(self, window: int) -> str:
        assert window >= 1, "Window size must be a positive integer"
        for ind in range(0, len(self._data)):
            if len(set(self._data[ind: ind + window])) == window:
                return str(ind + window)
        else:
            raise RuntimeError(f"Malformed input for day: {self._day}")

    def _part_a(self) -> str:
        return self._solve(4)  # 1034

    def _part_b(self) -> str:
        return self._solve(14)  # 2472
