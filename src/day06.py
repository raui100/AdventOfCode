from lib.paths import get_day
import re


data = get_day(6).read_text().split("\n\n")  # Splits list at double newline
data = [re.sub(r"[^a-z]", "", entry) for entry in data]  # Deletes every character that is not a-z

print(sum(map(lambda x: len(set(x)), data)))
