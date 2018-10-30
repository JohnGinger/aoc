import aoc_util
input_string = aoc_util.get_puzzle_text("3")


class Position:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __str__(self):
        return f'x{self.x}y{self.y}'


def mutatingDeliver(presents, position, at_least_one_present, char):
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
    return at_least_one_present


presents = {}
at_least_one_present = 1

even_chars = list(input_string)[::2]
odd_chars = list(input_string)[1::2]

positionSanta = Position(0, 0)
presents[str(positionSanta)] = 1
for char in even_chars:
    at_least_one_present = mutatingDeliver(
        presents, positionSanta, at_least_one_present, char)

positionRobotSanta = Position(0, 0)
for char in odd_chars:
    at_least_one_present = mutatingDeliver(
        presents, positionRobotSanta, at_least_one_present, char)

print(f'{at_least_one_present} houses have at least 1 present')
