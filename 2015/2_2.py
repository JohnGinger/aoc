import aoc_util
input_lines = aoc_util.get_puzzle_lines("2")
ribbon_needed = 0
for line in input_lines:
    (l, w, h) = line.split('x')
    l = int(l)
    w = int(w)
    h = int(h)
    sorted_lengths = sorted([l, w, h])
    ribbon_needed += l*w*h + 2*sorted_lengths[0] + 2*sorted_lengths[1]

print(f'You need {ribbon_needed} feet of ribbon')
