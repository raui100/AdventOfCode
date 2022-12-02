import sys
from src import solutions

if __name__ == "__main__":
    try:
        day = int(sys.argv[1])
    except IndexError:
        day = 0

    if day not in [0, 1]:
        raise NotImplementedError(f"Day {day} has not been implemented yet")

    if day in [0, 1]:
        print(solutions.day_01.Solution(1).solution())

    if day in [0, 2]:
        print(solutions.day_02.Solution(2).solution())