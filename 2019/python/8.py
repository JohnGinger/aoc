from util import get_puzzle_text
import textwrap

def chunks(list, n):
    for i in range(0, len(list) - 1, n):
        yield list[i:i + n]

def part_one(puzzle_text):
    width = 25
    height = 6
    characters_per_layer = width * height

    result = (100,0)
    for layer in chunks(puzzle_text,characters_per_layer):
        num_zeros = sum((1 if x == '0' else 0 for x in layer))
        num_ones = sum((1 if x == '1' else 0 for x in layer))
        num_twos = sum((1 if x == '2' else 0 for x in layer))
        if num_zeros < result[0]:
            result = (num_zeros, num_ones * num_twos)
    return result[1]
 
def part_two(puzzle_text):
    width = 25
    height = 6
    characters_per_layer = width * height
    image = ['2'] * characters_per_layer
    for layer in chunks(puzzle_text,characters_per_layer):
        for i, pixel in enumerate(layer):
            if image[i] == '2':
                if pixel == '1':
                    image[i] = '█'
                elif pixel == '0':
                    image[i] = ' '
    return "\n" + "\n".join("".join(x) for x in chunks(image, 25))

if __name__ == "__main__":
    print(f'Part 1 is {part_one(get_puzzle_text(8))}')
    print(f'Part 2 is {part_two(get_puzzle_text(8))}')