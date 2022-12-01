from functools import cached_property

from src.lib.parsing import read_text
from src.lib.solution import SolutionABC


class Solution(SolutionABC):

    @cached_property
    def _sorted_and_summed_numbers(self) -> list[int]:
        numbers: list[int] = []
        for group in read_text(1).split("\n\n"):
            numbers.append(sum([int(num) for num in group.splitlines()]))

        return sorted(numbers)

    def _part_a(self) -> str:
        highest: int = self._sorted_and_summed_numbers[-1]  # Highest number

        return str(highest)  # 75501

    def _part_b(self) -> str:
        highest_three: int = sum(self._sorted_and_summed_numbers[-3:])  # Sum of three highest numbers

        return str(highest_three)  # 215594

