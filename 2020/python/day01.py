from itertools import combinations_with_replacement
from math import prod

from .lib.paths import get_day


def iter_list(number_of_entries: int) -> int:
    """Tries to determine a result that sums 'number_of_entries' to 2020 and returns their product

    Args:
        number_of_entries: Number of entries that should sum to 2020

    Returns:
        Product of the years that sum two 2020 if a solution is possible

    Raises:
        RuntimeError: There is no possible solution
    """
    for indices in combinations_with_replacement(iterable_years, number_of_entries):
        if sum(valid_years := [years[index] for index in indices]) == 2020:

            return prod(valid_years)

    else:

        raise RuntimeError(
            f"There is no possible solution for {number_of_entries} number of entries"
        )


if __name__ == "__main__":
    years: list = [
        int(d) for d in get_day(1).read_text().split()
    ]  # Casts file to list of integer
    iterable_years = list(range(len(years)))
    print(iter_list(3))
