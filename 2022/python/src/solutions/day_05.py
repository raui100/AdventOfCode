from typing import NamedTuple

from src.lib.parsing import read_text
from src.lib.solution import SolutionABC


class Move(NamedTuple):
    start: int
    end: int
    number: int


class Solution(SolutionABC):
    @staticmethod
    def common_parse() -> (list[list[str]], list[Move]):
        stack, movements = read_text(5).split("\n\n")
        stacks: list[list[str]] = [[] for _ in stack.splitlines()]
        for line in stack.splitlines()[:-1][::-1]:  # Skipping first line and reversing order
            for ind in range(0, len(line), 4):  # Each group is 4 characters wide
                group = line[ind: ind + 4]
                if group[1] != " ":
                    stacks[ind // 4].append(group[1])

        moves: list[Move] = []
        for line in movements.splitlines():  # eg: move 5 from 4 to 5
            numbers = line.split()
            number = int(numbers[1])  # eg: 5
            start = int(numbers[3]) - 1  # eg: 4  || Converting 1-index from input to 0-index
            end = int(numbers[5]) - 1  # eg: 5  || Converting 1-index from input to 0-index
            moves.append(Move(start, end, number))

        return stacks, moves

    def common_solve(self, reverse: bool) -> str:
        stacks, moves = self.common_parse()
        for move in moves:
            work_load = []
            for _ in range(move.number):
                crate = stacks[move.start].pop()
                work_load.append(crate)

            work_load = work_load[::-1] if reverse else work_load
            stacks[move.end] += work_load

        result = "".join([stack[-1] for stack in stacks])

        return result

    def _part_a(self) -> str:
        return self.common_solve(reverse=False)

    def _part_b(self) -> str:
        return self.common_solve(reverse=True)
