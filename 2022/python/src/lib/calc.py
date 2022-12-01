from typing import Optional


def sum_numbers(data: list[Optional[int]]) -> list[int]:
    """Sums adjacent numbers in a list, where groups are separated by `None`"""
    if len(data) == 0:  # Taking care of an edge case
        return []

    numbers = []
    current = 0
    for number in data:
        if number is not None:
            current += number
        else:
            numbers.append(current)
            current = 0

    # Group of numbers don't have to end on an `None`
    if data[-1] is not None:
        numbers.append(current)

    return numbers


def test_sum_numbers():
    assert sum_numbers([]) == []  # Empty list
    assert sum_numbers([1, 2, None, 3, 4, None]) == [3, 7]  # /w `None` at the end
    assert sum_numbers([1, 2, None, 3, 4]) == [3, 7]  # /wo `None` at the end




