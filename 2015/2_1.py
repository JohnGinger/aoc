import aoc_util
input_lines = aoc_util.get_puzzle_lines("2")
paper_needed = 0
for line in input_lines:
    (l, w, h) = line.split('x')
    l = int(l)
    w = int(w)
    h = int(h)
    smallest_side = min([l*w, w*h, l*h])
    paper_needed += 2*l*w + 2*w*h + 2*h*l + smallest_side

print(f'You need {paper_needed} feet^2 of paper')
