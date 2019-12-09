from util import get_puzzle_text
import logging


def run(input_string, password_checker):
    start_range, end_range = parse_input(input_string)

    valid_passwords = 0
    for x in range(start_range, end_range + 1):
        if password_checker(str(x)):
            valid_passwords += 1

    return valid_passwords

def part_one(input_string):
    return run(input_string, valid_password_part_one)

def part_two(input_string):
    return run(input_string, valid_password_part_two)

def valid_password_part_one(possible_password: str):
    if len(possible_password) != 6:
        return False

    repeating_numbers = False
    last_number = -1
    for letter in str(possible_password):
        number = int(letter)
        if number == last_number:
            repeating_numbers = True
        if number < last_number:
            return False
        last_number = number 
    return repeating_numbers

def valid_password_part_two(possible_password: str):
    if len(possible_password) != 6:
        return False

    is_repeating = False
    repeating_length = 1
    last_number = -1
    for letter in str(possible_password):
        number = int(letter)
        if number == last_number:
            repeating_length += 1
        else:
            if repeating_length == 2:
                is_repeating = True
            repeating_length = 1
        if number < last_number:
            return False
        last_number = number 

    return is_repeating or repeating_length == 2

def parse_input(input_string):
    start_range, end_range = input_string.split('-')
    return int(start_range), int(end_range)


# Code
if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    print(f'Part 1 is {part_one(get_puzzle_text(4))}')
    print(f'Part 2 is {part_two(get_puzzle_text(4))}')

def test_one():
    assert valid_password_part_two('112233')

def test_two():
    assert not valid_password_part_two('123444')

def test_three():
    assert valid_password_part_two('111122')