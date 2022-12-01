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
