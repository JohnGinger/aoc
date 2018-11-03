import aoc_util
import re

input_string = aoc_util.get_puzzle_lines("19")

medicine = ""
replacers = []
for line in input_string:
    regex = re.compile(r'([A-Za-z]+) => ([A-Za-z]+)')
    if regex.match(line):
        matches = re.match(regex, line)
        (from_element, to_element) = matches.groups()
        replacers.append((from_element, to_element))
    elif line.rstrip() == "":
        pass
    else:
        medicine = line.rstrip()

replacements = {}
for replacer in replacers:
    remaining_string = medicine
    before = ""
    position_to_check_from = 0
    while medicine[position_to_check_from:].count(replacer[0]) > 0:
        found_position = medicine[position_to_check_from:].find(replacer[0])
        converted_string = medicine[:(position_to_check_from + found_position)] + replacer[1] + \
            medicine[(position_to_check_from +
                      found_position + len(replacer[0])):]
        position_to_check_from += found_position + len(replacer[0])
        replacements[converted_string] = True

print(len(replacements.keys()))
