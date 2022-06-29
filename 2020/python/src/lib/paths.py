from pathlib import Path

data_directory = Path(__file__).resolve().parent.parent.parent / "input"


def get_day(day: int) -> Path:
    """Returns Path object associated the specified day"""
    return data_directory / f"day{str(day).zfill(2)}.txt"
