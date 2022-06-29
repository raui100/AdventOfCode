import sys
from src import day_01, day_02, day_03, day_15

if __name__ == "__main__":
    day = 0 if len(sys.argv) == 1 else int(sys.argv[1])  # Expected usage "python main.py DAY" with eg: DAY = 1

    if day not in [0, 1, 2, 3, 15]:
        raise ValueError(f"Day {day} has not been solved yet")

    if day == 0 or day == 1:
        print("Day 1: Sonar Sweep")
        day_01.solve_part_a()
        day_01.solve_part_b()
        print()

    if day == 0 or day == 2:
        print("Day 2: Dive!")
        day_02.solve_part_a()
        day_02.solve_part_b()
        print()

    if day == 0 or day == 3:
        print("Day 3: Binary Diagnostic")
        day_03.solve_part_a()
        day_03.solve_part_b()
        print()

    if day == 0 or day == 15:
        print("Day 15: Chiton")
        day_15.solve_part_a()
        day_15.solve_part_b()
        print()

