from itertools import product
from math import prod

from lib.paths import data_directory


def iter_list(number_of_entries: int) -> int:
    """Tries to determine a result that sums 'number_of_entries' to 2020 and returns their product

    Args:
        number_of_entries: Number of entries that should sum to 2020

    Returns:
        Product of the years that sum two 2020 if a solution is possible

    Raises:
        RuntimeError: There is no possible solution
    """
    for indices in product(*(iterable_years,) * number_of_entries):
        if sum(valid_years := [years[index] for index in indices]) == 2020:

            return prod(valid_years)

    else:

        raise RuntimeError(f"There is no possible solution for {number_of_entries} number of entries")


def day01():
    """Solution to day 1"""
    return iter_list(2)


if __name__ == "__main__":
    years: list = [int(d) for d in (data_directory / "day01.txt").read_text().split()]  # Casts file to list of integer
    iterable_years = range(len(years) - 1)
    print(iter_list(2))
    print(iter_list(3))
