from math import prod

from lib.paths import data_directory


def encounters(steps_right: int, steps_down: int):
    """Determines encounters with the given parameters"""
    count = 0
    print(f"r: {steps_right}, d: {steps_down}")
    for index, pos_x in enumerate(range(0, len(data), steps_down)):
        pos_y = (steps_right * index) % map_len
        char = data[pos_x][pos_y]
        if char == "#":
            count += 1
        print(" ".join(data[pos_x]))
        print("  " * pos_y + char + " <- " + str(count))

    print("-" * 80)

    return count


data = (data_directory / "day03.txt").read_text().split("\n")  # Splits the file at every newline
data.pop()  # The last line of data is an empty string
map_len = len(data[0])

lst_encounters = [encounters(right, down) for right, down in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]]
print(prod(lst_encounters))
