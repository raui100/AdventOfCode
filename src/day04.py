from lib.paths import data_directory


data = (data_directory / "day04.txt").read_text().split("\n\n")
matches = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
matches = [match + ":" for match in matches]  # Appends ':' to every string

print(sum([all(match in passport for match in matches) for passport in data]))

