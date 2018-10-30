import aoc_util
input_string = aoc_util.get_puzzle_text("1")
floors = 0
for x in list(input_string):
    if x == '(':
        floors += 1
    elif x == ')':
        floors -= 1
    else:
        raise ValueError('Unknown Character')

print(f'Traversed {floors} floors')
