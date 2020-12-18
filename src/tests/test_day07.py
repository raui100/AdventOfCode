from src.day07 import (
    get_bag_with_content_and_count,
    Bag,
    BagQuantity,
    get_primary_bag,
    structure_bags,
    get_secondary_bags,
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


def test_get_bag_content(line, subject_bag, bag_1, bag_2):
    assert get_bag_with_content_and_count(line)[0] == subject_bag
    assert get_bag_with_content_and_count(line)[1] == [
        BagQuantity(bag_1, 1),
        BagQuantity(bag_2, 2),
    ]

    assert (
        get_bag_with_content_and_count("light red bags contain no other bags.")[0]
        == subject_bag
    )
    assert (
        get_bag_with_content_and_count("light red bags contain no other bags.")[1] == []
    )


def test_get_primary_bag(line, subject_bag):
    assert get_primary_bag(line) == subject_bag


def test_get_secondary_bags(line, bag_1, bag_2):
    assert get_secondary_bags(line) == [bag_1, bag_2]


def test_structure_bags(line, subject_bag, bag_1, bag_2):
    assert structure_bags([line]) == {
        subject_bag: [BagQuantity(bag_1, 1), BagQuantity(bag_2, 2)]
    }
