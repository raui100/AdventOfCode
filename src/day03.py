from lib.paths import get_day
from math import prod

data = [c == "#" for c in "".join(get_day(3).read_text().split())]
width = len(get_day(3).read_text().split()[0])
length = int(len(data) / width)
paths = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
print(
    prod(
        map(
            lambda x: sum(
                [
                    data[ind]
                    for ind in [
                        x[1] * i * width + (x[0] * i) % width
                        for i in range(0, int(length / x[1]))
                    ]
                ]
            ),
            paths,
        )
    )
)

print(
    prod(
        [
            sum(
                [
                    data[ind]
                    for ind in [
                        i * width * x + (y * i) % width
                        for i in range(0, int(length / x))
                    ]
                ]
            )
            for y, x in paths
        ]
    )
)
