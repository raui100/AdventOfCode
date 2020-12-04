from lib.paths import data_directory


data = (data_directory / "day03.txt").read_text().split("\n")  # Splits the file at every newline
data.pop()  # The last line of data is an empty string

encounters: int = 0
for index, row in enumerate(data):
    y_position = (3 * index) % len(row)
    if row[y_position] == "#":
        encounters += 1

print(encounters)
