import aoc_util
import copy
from PIL import Image

input_string = aoc_util.get_puzzle_lines("18")

grid = [[False] + [y == "#" for y in line.rstrip()] + [False]
        for line in input_string]
grid_size = len(grid)
grid = [[False] * (grid_size+2)] + grid + [[False] * (grid_size+2)]


def print_grid(grid):
    print("\n".join(["".join("#" if grid[y][x] else "." for x in range(1, grid_size + 1))
                     for y in range(1, grid_size + 1)]))
    print(sum(sum([1 for y in row if y]) for row in grid))


def sum_neighbors(y, x):
    return (grid[y-1][x-1] + grid[y][x-1] + grid[y+1][x-1] +
            grid[y-1][x] + grid[y+1][x] +
            grid[y-1][x+1] + grid[y][x+1] + grid[y+1][x+1])


new_grid = copy.deepcopy(grid)


def toggle(y, x):
    neighbors = sum_neighbors(y, x)
    if grid[y][x]:
        if neighbors == 2 or neighbors == 3:
            new_grid[y][x] = True
        else:
            new_grid[y][x] = False
    else:
        if neighbors == 3:
            new_grid[y][x] = True
        else:
            new_grid[y][x] = False


im = Image.new("RGB", (grid_size, grid_size))

steps = 100
frames = [[[(0, 0, 0) for y in range(grid_size)]
           for x in range(grid_size)] for f in range(steps)]


for i in range(steps):
    new_grid = copy.deepcopy(grid)
    # print_grid(grid)

    for y in range(1, grid_size + 1):
        for x in range(1, grid_size + 1):
            toggle(y, x)
    grid = new_grid

    im = Image.new("RGB", (grid_size, grid_size))
    im.putdata([(255, 255, 255) if grid[y][x] else (0, 0, 0) for y in range(grid_size)
                for x in range(grid_size)])
    frames[i] = im

print(sum(sum([1 for y in row if y]) for row in grid))
im.save("out.gif", save_all=True, append_images=frames)
