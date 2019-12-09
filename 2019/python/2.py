from util import get_puzzle_text
from math import floor

program_base = [int(x) for x in get_puzzle_text(2).strip().split(',')]


def run_program(noun, verb):
    program = program_base.copy()
    program[1] = noun
    program[2] = verb
    instruction_pointer = 0
    while True:
        opcode, a, b, destination = program[instruction_pointer:instruction_pointer+4]
        if opcode == 1:
            program[destination] = program[a] + program[b]
        elif opcode == 2:
            program[destination] = program[a] * program[b]
        elif opcode == 99:
            break
        else:
            print('Bad Instruction')
            break
        instruction_pointer += 4
    return program[0]


print(f'Part A is {run_program(12,2)}')

noun = 0
verb = 0
for noun in range(0, 100):
    for verb in range(0, 100):
        result = run_program(noun, verb)
        if result == 19690720:
            print(f'Part B is {100*noun + verb}')
            break
