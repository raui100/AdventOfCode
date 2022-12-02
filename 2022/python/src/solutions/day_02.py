from src.lib.parsing import read_text
from src.lib.solution import SolutionABC


SCORE: dict[str, tuple[int, int]] = {
    "A X": (4, 3),
    "A Y": (8, 4),
    "A Z": (3, 8),
    "B X": (1, 1),
    "B Y": (5, 5),
    "B Z": (9, 9),
    "C X": (7, 2),
    "C Y": (2, 6),
    "C Z": (6, 7),
}


class Solution(SolutionABC):
    def _calc_score(self, part: int) -> int:
        return sum([SCORE[line][part] for line in read_text(self._day).splitlines()])

    def _part_a(self) -> str:
        return str(self._calc_score(part=0))  # 10941

    def _part_b(self) -> str:
        return str(self._calc_score(part=1))  # 13071

