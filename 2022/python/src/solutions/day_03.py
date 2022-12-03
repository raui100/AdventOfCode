from src.lib.parsing import read_text
from src.lib.solution import SolutionABC
from typing import Iterable


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


def intersection(sets: Iterable) -> str:
    return set.intersection(*(set(entry) for entry in sets)).pop()


def split_in_half(string: str) -> (str, str):
    assert string.__len__() % 2 == 0
    return string[:len(string) // 2], string[len(string) // 2:]


class Solution(SolutionABC):
    def _part_a(self) -> str:  # 12 minutes
        score = sum([char_to_num(intersection(split_in_half(line))) for line in read_text(3).splitlines()])

        return str(score)  # 7848

    def _part_b(self) -> str:
        txt = read_text(3).splitlines()
        score = sum([char_to_num(intersection(txt[ind: ind + 3])) for ind in range(0, len(txt), 3)])

        return str(score)  # 2616

