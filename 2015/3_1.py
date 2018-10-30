import aoc_util
input_string = aoc_util.get_puzzle_text("3")


class Position:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __str__(self):
        return f'x{self.x}y{self.y}'


position = Position(0, 0)
presents = {str(position): 1}
at_least_one_present = 1
for char in list(input_string):
    if char == "^":
        position.y += 1
    elif char == "v":
        position.y -= 1
    elif char == "<":
        position.x -= 1
    elif char == ">":
        position.x += 1
    else:
        raise ValueError('Unknown Character')

    if str(position) in presents:
        presents[str(position)] += 1
    else:
        presents[str(position)] = 1
        at_least_one_present += 1

print(f'{at_least_one_present} houses have at least 1 present')
