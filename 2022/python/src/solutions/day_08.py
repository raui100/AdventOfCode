from functools import cache
from itertools import takewhile

from src.lib.solution import SolutionABC


class Solution(SolutionABC):

    @property
    def dim_x(self) -> int:
        return self._common()[0].__len__()

    @property
    def dim_y(self) -> int:
        return self._common().__len__()

    @cache
    def _common(self) -> list[list[int]]:
        return [[int(char) for char in line] for line in self._data.splitlines()]

    def trees_in_sight(self, pos_x: int, pos_y: int) -> list[list[tuple[int, int]]]:
        return [
            [(x, pos_y) for x in reversed(range(0, pos_x))],
            [(x, pos_y) for x in range(pos_x + 1, self.dim_x)],
            [(pos_x, y) for y in reversed(range(0, pos_y))],
            [(pos_x, y) for y in range(pos_y + 1, self.dim_y)],
        ]

    def is_edge(self, pos_x: int, pos_y: int) -> bool:
        return pos_x == 0 or pos_y == 0 or pos_x == self.dim_x - 1 or pos_y == self.dim_y - 1

    def is_visible(self, pos_x: int, pos_y: int) -> bool:
        if self.is_edge(pos_x, pos_y):
            return True

        tree_map = self._common()
        my_tree = tree_map[pos_y][pos_x]
        for trees in self.trees_in_sight(pos_x, pos_y):
            if all((tree_map[pos[1]][pos[0]] < my_tree for pos in trees)):
                return True

        return False

    def visibility(self, pos_x: int, pos_y: int) -> int:
        if self.is_edge(pos_x, pos_y):
            return 0

        tree_map = self._common()
        my_tree = tree_map[pos_y][pos_x]
        visibility = 1

        for trees in self.trees_in_sight(pos_x, pos_y):
            counter = 0
            for pos in trees:
                counter += 1

                tree = tree_map[pos[1]][pos[0]]
                if tree >= my_tree:

                    visibility *= counter
                    break
            else:
                visibility *= counter

        return visibility

    def _part_a(self) -> str:
        result = 0
        for pos_x in range(0, self.dim_x):
            for pos_y in range(0, self.dim_y):
                result += self.is_visible(pos_x, pos_y)

        return str(result)  # 1801

    def _part_b(self) -> str:
        result = []
        for pos_x in range(0, self.dim_x):
            for pos_y in range(0, self.dim_y):
                result.append(self.visibility(pos_x, pos_y))

        return str(max(result))  # 209880
