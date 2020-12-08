from lib.paths import get_day
import re


def re_range(prefix: str, low: int, up: int, postfix: str = ""):
    """Creates a regex for a numeric range"""  # e.g. re_range("byr:", 1920, 2002),
    lst_range = [str(i) for i in range(low, up + 1)]
    str_range = f"({'|'.join(lst_range)})"
    return re.compile(prefix + str_range + postfix)


data = get_day(4).read_text().split("\n\n")
patterns = \
    {
        "birth_year": re_range("byr:", 1920, 2002),
        "issue_year": re_range("iyr:", 2010, 2020),
        "expiration_year": re_range("eyr:", 2020, 2030),
        "height": re.compile(re_range("hgt:", 150, 193, "cm").pattern + "|" + re_range("hgt:", 59, 76, "in").pattern),
        "hair_color": re.compile(r"hcl:#([0-9a-f]{6})"),
        "eye_color": re.compile(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)"),
        "passport_identity": re.compile(r"\b\d{9}\b")
    }

print(sum([all(map(lambda x: x.search(passport) is not None, patterns.values())) for passport in data]))
