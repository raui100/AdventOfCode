from pathlib import Path


def input_to_list_str(day: str) -> list[str]:
    """Reads the content of `day` and formats it as `list[str]`"""
    path_input = Path(__file__).parent.parent / "input" / day
    return path_input.read_text().splitlines()

def input_to_list_int(day: str) -> list[int]:
    """Reads the content of `day` and formats it as `list[int]`"""
    return [int(number) for number in input_to_list_str(day)]
