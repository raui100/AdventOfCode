from contextlib import suppress
from functools import cache

from src.lib.solution import SolutionABC


class Solution(SolutionABC):
    @cache
    def _common(self) -> list[int]:
        tmp = []
        directories = []
        for line in self._data.splitlines():
            match line.split():
                case ["$", "cd", ".."]:
                    directories.append(tmp.pop()),
                case ["$", "cd", _]:
                    tmp.append(0),
                case [size, _] if size.isdigit():
                    tmp = [num + int(size) for num in tmp]

        directories += tmp

        return directories

    def _part_a(self) -> str:
        result = sum([num for num in self._common() if num < 100_000])

        return str(result)  # 2031851

    def _part_b(self) -> str:
        root = max(self._common())
        required = root + 30_000_000 - 70_000_000
        result = min([num for num in self._common() if num >= required])

        return str(result)  # 2568781
