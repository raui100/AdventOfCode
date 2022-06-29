if __name__ == "__main__":
    number = {12: 1, 1: 2, 16: 3, 3: 4, 11: 5}
    last_spoken = int
    new_number = 0
    for index in range(max(number.values()) + 1, 30000000):
        last_spoken = new_number
        try:
            old_index = number[new_number]
            new_number = index - old_index
        except KeyError:
            new_number = 0

        number[last_spoken] = index

    print(new_number)
