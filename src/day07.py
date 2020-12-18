"""Solves day 7 puzzle"""
import re
from typing import NamedTuple, List, Tuple, Dict
from lib.paths import get_day


class Bag(NamedTuple):
    """Models a bag"""

    description: str

    def __eq__(self, other: "Bag"):
        return self.description == other.description


class BagQuantity(NamedTuple):
    """Couples bags with their quantity"""

    bag: Bag
    count: int


def get_primary_bag(line: str) -> Bag:
    """Determines the primary bag of that row"""
    return Bag(re.match(r"^(\w+ \w+)", line).group(1))


def get_secondary_bags(line: str) -> List[Bag]:
    """Determines the secondary bags of that row"""
    return [Bag(bag) for bag in re.findall(r"\d+ (\w+ \w+)", line)]


def get_bag_with_content_and_count(line: str) -> Tuple[Bag, List[BagQuantity]]:
    """Determines which other bags one subject bag can have"""
    subject_bag = get_primary_bag(line)
    matches = re.findall(r"((\d+) (\w+ \w+))", line)
    return subject_bag, [BagQuantity(Bag(bag), int(count)) for _, count, bag in matches]


def structure_bags(data: List[str]) -> Dict[Bag, List[BagQuantity]]:
    """Transforms a list of bag descriptions into a dictionary structure"""
    bag_structure = dict()
    for line in data:
        bag, bags = get_bag_with_content_and_count(line)
        bag_structure[bag] = bags

    return bag_structure


def part_1(data: List[str]):
    """Executes pa"""

    bags = {get_primary_bag(line): get_secondary_bags(line) for line in data}

    assert len(bags) == len(_data)  # Asserts that no data is being lost

    wanted_bag = Bag("shiny gold")

    has_wanted_bag = set()
    iteration = 0
    while True:
        len_before_iteration = len(has_wanted_bag)
        for primary_bag, secondary_bags in bags.items():
            if primary_bag in has_wanted_bag:
                continue

            if wanted_bag in secondary_bags:
                has_wanted_bag.add(primary_bag)
                continue

            for secondary_bag in secondary_bags:
                if secondary_bag in has_wanted_bag:
                    has_wanted_bag.add(primary_bag)
                    continue

        iteration += 1
        print(
            f"Iteration {str(iteration).zfill(2)}: Suitable bags found -> {str(len(has_wanted_bag)).zfill(3)}"
        )

        if len(has_wanted_bag) == len_before_iteration:
            break


if __name__ == "__main__":
    _data = get_day(7).read_text().splitlines()
    part_1(_data)
