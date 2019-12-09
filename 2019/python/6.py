from util import get_puzzle_lines, get_puzzle_tests
from collections import defaultdict
import logging
# logging.basicConfig(level=logging.DEBUG)

def build_tree(input_lines):
    connections = defaultdict(list)
    for line in input_lines:
        is_orbiting, name = line.strip().split(')')
        connections[is_orbiting].append(name)
    return connections

def build_tree_parents(input_lines):
    connections = {}
    for line in input_lines:
        is_orbiting, name = line.strip().split(')')
        connections[name] = is_orbiting
    return connections

def part_one(input_lines):
    checksum = 0
    connections = build_tree(input_lines)

    children = [(child, 0) for child in connections['COM']]
    for child, orbiting in children:
        logging.debug('%s at level %s',child, orbiting)
        checksum += 1 + orbiting
        children += [(child, orbiting+1) for child in connections[child]]

    return checksum

def get_to_com(connections, position):
    current_position = position
    length = -1
    while current_position != 'COM':
        yield (current_position, length)
        current_position = connections[current_position]
        length += 1

    yield ('COM', length)

def part_two(input_lines):
    connections = build_tree_parents(input_lines)

    your_path = get_to_com(connections, 'YOU')
    santa_path = get_to_com(connections, 'SAN')

    you_visited = defaultdict(int)
    santa_visited = defaultdict(int)

    while True:
        your_position, your_length = next(your_path)
        santa_position, santa_length = next(santa_path)
        you_visited[your_position] = your_length
        santa_visited[santa_position] = santa_length
        if you_visited[santa_position]:
            return you_visited[santa_position] + santa_length
        
        if santa_visited[your_position]:
            return santa_visited[your_position] + your_length


# Code
if __name__ == "__main__":
    print(f'Part 1 is {part_one(get_puzzle_lines(6))}')
    print(f'Part 2 is {part_two(get_puzzle_lines(6))}')


def test_example1():
    answer, lines = get_puzzle_tests(6,1)
    assert int(answer[0]) == part_one(lines)

def test_example2():
    answer, lines = get_puzzle_tests(6,2)
    assert int(answer[0]) == part_two(lines)