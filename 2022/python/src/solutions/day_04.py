from functools import cache

from src.lib.solution import SolutionABC
from typing import NamedTuple


class Section(NamedTuple):
    min: int
    max: int

    def contains(self, other: "Section") -> bool:
        return self.min <= other.min and self.max >= other.max

    def overlap(self, other: "Section") -> bool:
        return not (self.max < other.min or self.min > other.max)


class Solution(SolutionABC):
    @cache
    def common(self) -> list[Section]:
        data: list[Section] = []
        for line in self._data.splitlines():
            for elf in line.split(","):
                a, b = elf.split("-")
                data.append(Section(min=int(a), max=int(b)))

        return data

    def _part_a(self) -> str:  # 12 minutes
        data: list[Section] = self.common()
        score = 0
        for ind in range(0, len(data), 2):
            if data[ind].contains(data[ind + 1]) or data[ind + 1].contains(data[ind]):
                score += 1

        return str(score)  # 7848

    def _part_b(self) -> str:
        data: list[Section] = self.common()
        score = 0
        for ind in range(0, len(data), 2):
            if data[ind].overlap(data[ind + 1]):
                score += 1

        return str(score)  # 7848

