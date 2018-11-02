import re
import aoc_util
import itertools
input_string = aoc_util.get_puzzle_lines("16")

signature = {
    'children': 3, 'cats': 7, 'samoyeds': 2, 'pomeranians': 3, 'akitas': 0,
    'vizslas': 0, 'goldfish': 5, 'trees': 3, 'cars': 2, 'perfumes': 1,
}


def parse_line(line):
    regex = re.compile(
        r'^Sue ([0-9]+): ([A-Za-z]+): ([0-9]+), ([A-Za-z]+): ([0-9]+), ([A-Za-z]+): ([0-9]+)$')
    matches = re.match(regex, line).groups()
    for match in list(zip(matches[1::2], matches[2::2])):
        if signature[match[0]] != int(match[1]):
            return

    print(f'sue {matches[0]} is a good match')


for line in input_string:
    parse_line(line)
