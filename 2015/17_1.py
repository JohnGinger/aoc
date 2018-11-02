import aoc_util
import itertools
input_string = aoc_util.get_puzzle_lines("17")

sizes = []
for l in input_string:
    line = l.rstrip()
    sizes.append(int(line))

fits = 0
for size in range(0, len(sizes)):
    fits += len([sum(x) for x in itertools.combinations(
        sizes, size) if sum(x) == 150])

print(f'{fits} boxes fit')
