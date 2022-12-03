from src.lib.parsing import read_text
from src.lib.solution import SolutionABC


def char_to_num(char: str) -> int:
    if (num := ord(char)) >= 97:
        return num - 96
    else:
        return num - 38


def test_char_to_num():
    assert char_to_num("a") == 1
    assert char_to_num("z") == 26
    assert char_to_num("A") == 27
    assert char_to_num("Z") == 52


class Solution(SolutionABC):
    def _part_a(self) -> str:  # 12 minutes
        score = 0
        for line in read_text(3).splitlines():
            half = len(line) // 2
            set_1 = set(line[: half])
            set_2 = set(line[half:])
            double = set_1.intersection(set_2).pop()
            score += char_to_num(double)

        return str(score)  # 7848

    def _part_b(self) -> str:
        txt = read_text(3).splitlines()
        score = 0
        for ind in range(0, len(txt), 3):
            set_char = set(txt[ind]).intersection(set(txt[ind + 1])).intersection(set(txt[ind + 2]))
            score += char_to_num(set_char.pop())

        return str(score)  # 2616

