from lib.paths import data_directory


years: list = [int(d) for d in (data_directory / "input").read_text().split()]  # Casts file to list of integer
year_threshold = 2020 - min(years)
for year_1 in years:
    for year_2 in years:
        if year_1 + year_2 > year_threshold:  # Skipping year combinations that can't have a solution
            continue

        for year_3 in years:
            if year_1 + year_2 + year_3 == 2020:
                print(year_1 * year_2 * year_3)
