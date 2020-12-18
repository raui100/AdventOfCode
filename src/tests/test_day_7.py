from src.day07 import get_bag_content, Bag, BagContent


def test_get_bag_content():
    line = "light red bags contain 1 bright white bag, 2 muted yellow bags."
    assert get_bag_content(line) == [
        BagContent(Bag("bright white"), 1),
        BagContent(Bag("muted yellow"), 2),
    ]
