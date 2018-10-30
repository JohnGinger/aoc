import aoc_util
input_string = aoc_util.get_puzzle_text("1")
floors = 0
position = 0
for x in list(input_string):
    position += 1
    if x == '(':
        floors += 1
    elif x == ')':
        floors -= 1
    else:
        raise ValueError('Unknown Character')
    if floors == -1:
        print(f'Entered basement at position {position}')
        break
