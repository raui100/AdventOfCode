"""Testing Day 6"""
from ..day06 import intersecting_entries


def test_intersecting_entries():
    """Tests intersecting entries"""
    assert intersecting_entries("Hallo\nWorld\n") == 2
