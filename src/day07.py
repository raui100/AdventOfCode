"""Solves day 7 puzzle"""
import re
from typing import NamedTuple, List


class Bag(NamedTuple):
    """Models a bag"""

    description: str

    def __eq__(self, other: "Bag"):
        return self.description == other.description


class BagContent(NamedTuple):
    bag: Bag
    quantity: int


def get_bag_content(line: str) -> List[BagContent]:
    """Takes a line of 'data' as strings and returns it as a list of BagContent"""
    matches = re.findall(r"((\d+) (\w+ \w+))", line)
    return [BagContent(Bag(bag), int(count)) for _, count, bag in matches]
