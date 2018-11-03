import aoc_util
import re

input_string = aoc_util.get_puzzle_lines("19")

medicine = ""
elements = {}
for line in input_string:
    regex = re.compile(r'([A-Za-z]+) => ([A-Za-z]+)')
    if regex.match(line):
        matches = re.match(regex, line)
        (from_element, to_element) = matches.groups()
        elements[from_element] = (from_element, to_element)
    elif line.rstrip() == "":
        pass
    else:
        medicine = line

num_elements = 0
for element in elements.keys():
    num_elements += medicine.count(element)

num_commas = medicine.count("Y")

print(num_elements - num_commas - 1)
