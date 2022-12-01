from pathlib import Path
from typing import Optional

import pytest


def read_text(day: int) -> str:
    """Reads an input file"""
    path: Path = Path(__file__).parent.parent.parent / "input" / (str(day).zfill(2) + ".txt")

    return path.read_text()


def test_read_text():
    txt = read_text(1)
    assert len(txt) > 0
    with pytest.raises(FileNotFoundError):
        read_text(0)


def input_to_int(data: str) -> list[Optional[int]]:
    """Converts an input to a list of int. If a line is empty `None` will be inserted"""
    return [int(num) if len(num) else None for num in data.splitlines()]


def test_input_to_int():
    assert input_to_int("1\n2\n\n3\n") == [1, 2, None, 3]

