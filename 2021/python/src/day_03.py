"""Solves day 03 of advent of code"""
from .lib import input_to_list_str


def solve_part_a():
    """Solves the first halve of the puzzle"""

    number_rows = input_to_list_str("day_03.txt")
    LEN_INPUT = len(number_rows)
    LEN_ROW = len(number_rows[0])
    counter = {index: 0 for index in range(LEN_ROW)}

    # Counting number of "1" for each index in the binary number
    for row in number_rows:
        for index, number in enumerate(row):
            if number == "1":
                counter[index] += 1

    # Calculating epsilon and gamma
    epsilon = ["1" if value * 2 > LEN_INPUT else "0" for value in counter.values()]
    epsilon = int("".join(epsilon), 2)
    gamma = int("1" * LEN_ROW, 2) - epsilon

    # Output of  the solution (2035764)
    print(f"Part A: {epsilon * gamma}")


def solve_part_b():
    """Solves the second halve of the puzzle"""
    pass


if __name__ == "__main__":
    solve_part_a()
    solve_part_b()
