from functools import cached_property
from typing import Optional

from src.lib.parsing import input_to_int
from src.lib.calc import sum_numbers
from src.lib.solution import SolutionABC


class Solution(SolutionABC):

    @cached_property
    def _sorted_and_summed_numbers(self) -> list[int]:
        data: list[Optional[int]] = input_to_int(self._data)
        summed: list[int] = sorted(sum_numbers(data))

        return sorted(summed)

    def _part_a(self) -> str:
        highest: int = self._sorted_and_summed_numbers[-1]  # Highest number

        return str(highest)  # 75501

    def _part_b(self) -> str:
        highest_three: int = sum(self._sorted_and_summed_numbers[-3:])  # Sum of three highest numbers

        return str(highest_three)  # 215594

