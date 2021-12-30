from pathlib import Path


def path_to_input(day: str) -> Path:
    """Returns the paths to the input file"""

    return Path(__file__).parent.parent / "input" / day


def input_to_list_str(day: str) -> list[str]:
    """Reads the content of `day` and formats it as `list[str]`"""

    return path_to_input(day).read_text().splitlines()


def input_to_list_int(day: str) -> list[int]:
    """Reads the content of `day` and formats it as `list[int]`"""

    return [int(number) for number in input_to_list_str(day)]
