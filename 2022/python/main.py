import sys
from src import solutions

if __name__ == "__main__":
    try:
        day = int(sys.argv[1])
    except IndexError:
        day = 0

    if day in [0, 1]:
        print(solutions.day_01.Solution(1).solution())

    if day in [0, 2]:
        print(solutions.day_02.Solution(2).solution())

    if day in [0, 3]:
        print(solutions.day_03.Solution(3).solution())

    if day in [0, 4]:
        print(solutions.day_04.Solution(4).solution())

    if day in [0, 5]:
        print(solutions.day_05.Solution(5).solution())

    if day in [0, 6]:
        print(solutions.day_06.Solution(6).solution())

    if day in [0, 7]:
        print(solutions.day_07.Solution(7).solution())

    if day in [0, 8]:
        print(solutions.day_08.Solution(8).solution())
