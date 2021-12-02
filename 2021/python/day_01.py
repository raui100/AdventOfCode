"""Solves day 01 of advent of code"""

from lib import input_to_list_int


def count_increases(numbers) -> int:
    """Counts the number of increases in `numbers`

    Args:
        numbers (list[int]): List of numbers (eg: [1, 3, 2])

    Returns:
        int: The number of occurences where `numbers[i + 1] > numbers[i]`
    """
    return sum([next > prev for next, prev in zip(numbers[1:], numbers[:-1])])


def solve_part_a():
    """Solves the first halve of the puzzle"""
    numbers = input_to_list_int("day_01.txt")
    number_of_increases = count_increases(numbers)
    print(f"Part A: {number_of_increases}")


def solve_part_b():
    """Solves the second halve of the puzzle"""
    numbers: list[int] = input_to_list_int("day_01.txt")
    numbers: list[tuple[int, int, int]] = zip(numbers[:-2], numbers[1:-1], numbers[2:])
    numbers: list[int] = [sum(number_tuple) for number_tuple in numbers]
    number_of_increases = count_increases(numbers)
    print(f"Part B: {number_of_increases}")


if __name__ == "__main__":
    solve_part_a()
    solve_part_b()
