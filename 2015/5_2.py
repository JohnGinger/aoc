import aoc_util
input_lines = aoc_util.get_puzzle_lines("5")


def is_nice(input):
    previous_letter = ''
    previous_but_one = ''
    has_repeated_letter = False
    has_valid_repeat = False
    for position, c in enumerate(input):
        if c == previous_but_one:
            has_repeated_letter = True
        if input[position:position+2] in input[position+2:]:
            has_valid_repeat = True

        previous_but_one = previous_letter
        previous_letter = c

    return has_repeated_letter and has_valid_repeat


nice_strings = 0
for line in input_lines:
    if is_nice(line):
        nice_strings += 1

print(f'There are {nice_strings} lines that are nice')
