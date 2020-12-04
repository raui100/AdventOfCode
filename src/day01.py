from lib.paths import data_directory


years: list = [int(d) for d in (data_directory / "input").read_text().split()]  # Casts file to list of integer
print(next((year * (2020 - year) for year in years if (2020 - year) in years)))  # Generator function
