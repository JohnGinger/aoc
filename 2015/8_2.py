import aoc_util
input_string = aoc_util.get_puzzle_lines("8")

string_count = 0
encoded_count = 0
for l in input_string:
    line = l.rstrip()
    string_count += len(line)
    new_line = line.replace('\\', '\\\\').replace('"', '\\"')
    encoded_count += len(new_line) + 2

print(encoded_count - string_count)
