import aoc_util
input_string = aoc_util.get_puzzle_lines("8")

string_count = 0
memory_count = 0
for l in input_string:
    line = l.rstrip()
    string_count += len(line)
    memory_count += len(eval(line))

print(string_count - memory_count)
