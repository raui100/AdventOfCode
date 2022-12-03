from abc import ABC

from src.lib.parsing import read_text


class SolutionABC(ABC):
    """Interface (abstract basic class) to ease implementation"""
    def __init__(self, day: int):
        self._day = day
        self._data = read_text(day)

    def _part_a(self) -> str:
        """Solution to part B"""
        raise NotImplementedError("Implement solution in class")

    def _part_b(self) -> str:
        """Solution to part B"""
        raise NotImplementedError("Implement solution in class")

    def solution(self) -> str:
        """Returns the solution to part A and part B"""
        try:
            solution_a = self._part_a()
        except NotImplementedError:
            solution_a = None

        try:
            solution_b = self._part_b()
        except NotImplementedError:
            solution_b = None

        if solution_a is None and solution_b is None:
            raise NotImplementedError("Neither solution A or B has been implemented")

        solution = f"Day {self._day}\n"
        if solution_a is not None:
            solution += f"Part A: {solution_a}\n"

        if solution_b is not None:
            solution += f"Part B: {solution_b}\n"

        return solution
