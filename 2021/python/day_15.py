"""Solves day 03 of advent of code"""
import math

import networkx as nx
import numpy as np

from lib import path_to_input


def shortest_path(
    matrix: np.array, start: tuple[int, int], end: tuple[int, int]
) -> float:
    """Calculates the shortest path through the matrix"""
    graph = nx.DiGraph()

    for x, y in np.ndindex(matrix.shape):
        weight = matrix[x][y]

        if x > 0:
            graph.add_edge((x - 1, y), (x, y), weight=weight)

        if y > 0:
            graph.add_edge((x, y - 1), (x, y), weight=weight)

        if x < matrix.shape[0]:
            graph.add_edge((x + 1, y), (x, y), weight=weight)

        if y < matrix.shape[1]:
            graph.add_edge((x, y + 1), (x, y), weight=weight)

    return nx.dijkstra_path_length(graph, start, end)


def increase_matrix(matrix: np.array, increase: int) -> np.array:
    """Increases all numbers in the matrix by `increase`. They wrap to 1 after 9"""
    times = math.ceil(increase / 9)
    matrix = matrix + increase
    matrix[matrix > 9] -= 9 * times

    return matrix


def solve_part_a():
    """Solves the first halve of day 15"""
    path = path_to_input("day_15.txt")
    matrix = np.genfromtxt(path, delimiter=1, dtype=int)

    print(
        f"Part A: {shortest_path(matrix, (0, 0), (matrix.shape[0] - 1, matrix.shape[1] - 1))}"
    )


def concatenate_with_increase_range(
    matrix: np.ndarray, inc_start: int, inc_stop: int, axis: int
) -> np.ndarray:
    """Concatenates a matrix n-times on the specified axis

    Args:
        matrix: 2D numpy array
        inc_start: Integer where the increase should start. Is inclusive. (eg: 0)
        inc_stop: Integer where the increase should stop. Is exclusive. (eg: 5)
        axis: Axis of the numpy array where the concatenation should be (eg: 0 for along the 'y' axis)

    Returns:
        The concatenated numpy array

    Examples:
        >>> a = np.array([[1, 2], [3, 4]])
        >>> concatenate_with_increase_range(a, inc_start=0, inc_stop=2, axis=1)
        array([[1, 2, 2, 3], [3, 4, 4, 5]])
    """
    return np.concatenate(
        [
            increase_matrix(matrix, increase=increase)
            for increase in range(inc_start, inc_stop)
        ],
        axis=axis,
    )


def solve_part_b():
    """Solves the second halve of day 15"""
    path = path_to_input("day_15.txt")
    matrix = np.genfromtxt(path, delimiter=1, dtype=int)

    # Extending the matrix on the x axis
    matrix = concatenate_with_increase_range(
        matrix, inc_start=0, inc_stop=5, axis=0
    )

    # Extending the matrix on the y axid
    matrix = concatenate_with_increase_range(
        matrix, inc_start=0, inc_stop=5, axis=1
    )

    print(
        f"Part b: {shortest_path(matrix, (0, 0), (matrix.shape[0] - 1, matrix.shape[1] - 1))}"
    )


if __name__ == "__main__":
    solve_part_a()  # 811
    solve_part_b()  # 3012
