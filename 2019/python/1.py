from util import get_puzzle_lines
from math import floor

def fuel_needed_a(line):
    return max(floor(int(line) / 3) - 2, 0)

def fuel_needed_b(line):
    fuel = 0
    to_add = fuel_needed_a(line)
    while to_add > 0:
        fuel += to_add
        to_add = fuel_needed_a(to_add)
    return fuel

score_part_a = 0
score_part_b = 0

for line in get_puzzle_lines(1):
    score_part_a += fuel_needed_a(line)
    score_part_b += fuel_needed_b(line)

print(f'Part A is {score_part_a}')
print(f'Part B is {score_part_b}')
