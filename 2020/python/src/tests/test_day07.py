from typing import List

from ..day07 import (
    Bag,
    Bags,
    BagQuantity,
    get_primary_bag,
    get_secondary_bags,
    get_secondary_bags_with_count,
)
from pytest import fixture


@fixture
def line() -> str:
    return "light red bags contain 1 bright white bag, 2 muted yellow bags."


@fixture
def subject_bag() -> Bag:
    return Bag("light red")


@fixture
def bag_1() -> Bag:
    return Bag("bright white")


@fixture
def bag_2() -> Bag:
    return Bag("muted yellow")


@fixture
def example() -> List[str]:
    return """shiny gold bags contain 2 dark red bags, 3 muted yellow bags.
dark red bags contain no other bags.
muted yellow bags contain 1 dark red bags.""".splitlines()


def test_bag(bag_1, bag_2):
    """Tests the 'Bag' class"""
    assert Bag("Test").description == "Test"  # Has attribute `description`
    assert bag_1 == bag_1  # Can be compared
    assert bag_1 != bag_2  # Can be compared


def test_bag_quantity(bag_1):
    """Tests the BagQuantity class"""
    bag_quantity = BagQuantity(bag_1, 1)
    assert bag_quantity.bag == bag_1  # Has attribute `bag`
    assert bag_quantity.count == 1  # Has attribute `count`
    assert bag_quantity == bag_quantity  # Is comparable


def test_get_primary_bag(line, subject_bag):
    assert get_primary_bag(line) == subject_bag


def test_get_secondary_bags(line, bag_1, bag_2):
    assert get_secondary_bags(line) == [bag_1, bag_2]


def test_get_secondary_bags_with_count(line, bag_1, bag_2):
    assert get_secondary_bags_with_count(line) == [
        BagQuantity(bag_1, 1),
        BagQuantity(bag_2, 2),
    ]


def test_bags(example):
    """Tests the `Bags` class"""
    wanted_bag = Bag("shiny gold")
    bags = Bags(example)
    assert bags.count_bags_inside(wanted_bag) == 8
