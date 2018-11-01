import math
from PIL import Image
import aoc_util
input_lines = aoc_util.get_puzzle_lines("6")


class Position:
    def __init__(self, x, y):
        self.x = int(x)
        self.y = int(y)


def clamp(value):
    if value < 0:
        return 0
    else:
        return value


def get_start_and_end(position):
    (s, e) = position.split(' through ')
    start = Position(*s.split(','))
    end = Position(*e.split(','))
    return (start, end)


w, h = 1000, 1000
grid = [[0 for x in range(w)] for y in range(h)]


def parse_line(line):
    value_to_add = 0
    if line[:8] == 'turn on ':
        (start, end) = get_start_and_end(line[8:])
        value_to_add = 1
    elif line[:9] == 'turn off ':
        (start, end) = get_start_and_end(line[9:])
        value_to_add = -1
    elif line[:7] == 'toggle ':
        (start, end) = get_start_and_end(line[7:])
        value_to_add = 2
    else:
        raise ValueError('Unknown Instruction')

    for x in range(start.x, end.x+1):
        for y in range(start.y, end.y+1):
            grid[x][y] = clamp(value_to_add + grid[x][y])


for line in input_lines:
    parse_line(line)


lights_brightness = 0
max_brightness = 0
for x in range(0, 1000):
    for y in range(0, 1000):
        lights_brightness += grid[x][y]
        if grid[x][y] > max_brightness:
            max_brightness = grid[x][y]
print(f'Total brightness is {lights_brightness}')


def get_pixel_value(brightness):
    value = math.floor((brightness /
                        max_brightness) * 255)
    return (value, value, value)


image_data = [get_pixel_value(grid[x][y]) for x in range(w) for y in range(h)]
im = Image.new("RGB", (1000, 1000))
im.putdata(image_data)
im.save("6_pattern.png", "PNG")
