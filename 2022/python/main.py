import sys
from src.solutions import day_01

if __name__ == "__main__":
    try:
        day = int(sys.argv[1])
    except IndexError:
        day = 0

    if day not in [0, 1]:
        raise NotImplementedError(f"Day {day} has not been implemented yet")

    if day in [0, 1]:
        print(day_01.Solution(1).solution())