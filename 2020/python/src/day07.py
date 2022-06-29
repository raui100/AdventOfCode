"""Solves day 7 puzzle"""
import re
from typing import NamedTuple, List
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


def get_secondary_bags_with_count(line: str) -> List[BagQuantity]:
    """Determines which other bags one subject bag can have"""
    matches = re.findall(r"((\d+) (\w+ \w+))", line)
    return [BagQuantity(Bag(bag), int(count)) for _, count, bag in matches]


class Bags:
    """Counts the bags inside the `wanted_bag`"""

    def __init__(self, data: List[str]):
        self.bags = {
            get_primary_bag(line): get_secondary_bags_with_count(line) for line in data
        }
        self._count = 0

    def count_bags_inside(self, bag: Bag) -> int:
        self._count = 0
        self._count_bags_inside(bag, 1)

        return self._count

    def _count_bags_inside(self, bag: Bag, factor: int):
        """Counts recursively the number of bags"""
        for bags_count in self.bags[bag]:
            bags_count: BagQuantity
            self._count += bags_count.count * factor
            self._count_bags_inside(bags_count.bag, bags_count.count * factor)


def part_1(data: List[str]):
    """Executes part 1 of day 7"""
    print("#" * 16 + " Part 1 " + "#" * 17)

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
            f"Iteration {str(iteration).zfill(1)}: Suitable bags found -> {str(len(has_wanted_bag)).zfill(3)} #"
        )

        if len(has_wanted_bag) == len_before_iteration:
            print("#" * 41)
            break


def part_2(data: List[str]):
    """Executes part 2 of day 7"""
    print("\n" + "#" * 16 + " Part 2 " + "#" * 17)
    wanted_bag = Bag("shiny gold")
    bags = Bags(data)
    count = bags.count_bags_inside(wanted_bag)
    print(wanted_bag.description + " has " + str(count) + " other bags inside.  #")
    print("#" * 41)


if __name__ == "__main__":
    _data = get_day(7).read_text().splitlines()
    part_1(_data)
    part_2(_data)
