import aoc_util
input_lines = aoc_util.get_puzzle_lines("6")


class Position:
    def __init__(self, x, y):
        self.x = int(x)
        self.y = int(y)


def get_start_and_end(position):
    (s, e) = position.split(' through ')
    start = Position(*s.split(','))
    end = Position(*e.split(','))
    return (start, end)


w, h = 1000, 1000
grid = [[False for x in range(w)] for y in range(h)]


def parse_line(line):
    value_to_set = False
    toggle_lights = False
    if line[:8] == 'turn on ':
        (start, end) = get_start_and_end(line[8:])
        value_to_set = True
    elif line[:9] == 'turn off ':
        (start, end) = get_start_and_end(line[9:])
        value_to_set = False
    elif line[:7] == 'toggle ':
        (start, end) = get_start_and_end(line[7:])
        toggle_lights = True
    else:
        raise ValueError('Unknown Instruction')

    for x in range(start.x, end.x+1):
        for y in range(start.y, end.y+1):
            if toggle_lights:
                grid[x][y] = not grid[x][y]
            else:
                grid[x][y] = value_to_set


for line in input_lines:
    parse_line(line)


lights_on = 0
for x in range(0, 1000):
    for y in range(0, 1000):
        if grid[x][y]:
            lights_on += 1

for x in range(0, 1000):
    for y in range(0, 1000):
        if grid[x][y]:
            print('0', end='')
        else:
            print('.', end='')
    print('')

print(f'There are {lights_on} lights on')
