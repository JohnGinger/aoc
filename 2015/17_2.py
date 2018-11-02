import aoc_util
import itertools
input_string = aoc_util.get_puzzle_lines("17")

sizes = []
for l in input_string:
    line = l.rstrip()
    sizes.append(int(line))

for size in range(0, len(sizes)):
    number_of_ways = len([sum(x) for x in itertools.combinations(
        sizes, size) if sum(x) == 150])
    if number_of_ways > 0:
        print(f'{number_of_ways} is number of ways you can do min_matches')
        break
