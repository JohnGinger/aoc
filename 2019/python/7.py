from util import get_puzzle_text
from intcode import run_program
from itertools import permutations

import logging

def part_one(input_text):
    max_value = 0
    for phase_settings in permutations([0,1,2,3,4], 5):
        value_so_far = 0
        for phase_setting in phase_settings:
            value_so_far = next(run_program(input_text, [phase_setting, value_so_far]))
        max_value = max(max_value, value_so_far)
    return max_value

def part_two(input_text):
    max_value = 0
    for phase_settings in permutations([5,6,7,8,9], 5):
        value_so_far = 0
        last_val = 0

        a_list = [phase_settings[0], 0]
        b_list = [phase_settings[1]]
        c_list = [phase_settings[2]]
        d_list = [phase_settings[3]]
        e_list = [phase_settings[4]]

        a = run_program(input_text, a_list)
        b = run_program(input_text, b_list)
        c = run_program(input_text, c_list)
        d = run_program(input_text, d_list)
        e = run_program(input_text, e_list)
        
        i = 0
        while i < 100:

            b_list.append(next(a))
            c_list.append(next(b))
            d_list.append(next(c))
            e_list.append(next(d))
            value_so_far = next(e)

            if type(value_so_far) == int:
                last_val = value_so_far
                a_list.append(value_so_far)
            else:
                break
            i +=1

        max_value = max(max_value, last_val)
    return max_value

# Code
if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    print(f'Part 1 is {part_one(get_puzzle_text(7))}')
    print(f'Part 2 is {part_two(get_puzzle_text(7))}')


def test_example1():
    answer = part_one('3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0')
    assert answer == 43210

def test_example2():
    answer = part_one('3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0')
    assert answer == 54321

def test_example3():
    answer = part_one('3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0')
    assert answer == 65210

def test_example4():
    answer = part_two('3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5')
    assert answer == 139629729

def test_example5():
    answer = part_two('3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10')
    assert answer == 18216