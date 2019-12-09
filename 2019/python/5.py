from util import get_puzzle_text
from intcode import run_program
import logging

def part_one(input_text):
    result = run_program(input_text, [1])
    output = next(result)
    while True:
        if output != 0:
            return output
        output = next(result)

def part_two(input_text):
    return next(run_program(input_text, [5]))

# Code
if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    print(f'Part 1 is {part_one(get_puzzle_text(5))}')
    print(f'Part 2 is {part_two(get_puzzle_text(5))}')