import re


with open('input.txt', 'r') as f:
    input_data = f.read().splitlines()


def substring(x: int, y1: int, y2: int) -> str:  # y1 is inclusive, y2 is exclusive
    return input_data[x][max(y1, 0):min(y2, len(input_data[0]))] if x >= 0 and x < len(input_data) else ''


part_numbers = []
for x, line in enumerate(input_data):
    for number in re.finditer(r'\d+', line):
        left = number.start() - 1
        right = number.end()
        adjacent_characters = \
            substring(x-1, left, right + 1) + \
            substring(x, left, left + 1) + \
            substring(x, right, right + 1) + \
            substring(x+1, left, right + 1)
        is_part_number = set(adjacent_characters) != {'.'}

        if is_part_number:
            part_number = int(number.group())
            print(part_number)
            part_numbers.append(part_number)

print(sum(part_numbers))
