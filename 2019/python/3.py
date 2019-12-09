from util import get_puzzle_lines, get_puzzle_tests
from dataclasses import dataclass
import copy
import logging
logging.basicConfig(level=logging.INFO)


@dataclass
class Position:
    x: int
    y: int

    def __hash__(self):
        return self.x + 1000000 * self.y

    def __copy__(self):
        return Position(self.x, self.y)

    def manhattan(self):
        return abs(self.x) + abs(self.y)


@dataclass
class Point:
    direction: str
    distance: int


def parse_path(path):
    points = []
    for point in path.split(','):
        direction = point[0]
        distance = int(point[1:])
        points.append(Point(direction, distance))
    return points


def part_one(lines):
    paths = [parse_path(path) for path in lines]

    visited_one, _ = get_set_of_visited_points_for_path(paths[0])
    visited_two, _ = get_set_of_visited_points_for_path(paths[1])
    logging.debug(('One', visited_one))
    logging.debug(('Two', visited_two))
    logging.debug(('Intersection', visited_one.intersection(visited_two)))
    overlap_dist = [x.manhattan() for x in visited_one.intersection(visited_two)]
    logging.debug(('Overlap Dist', overlap_dist))
    return min(overlap_dist)


def part_two(lines):
    paths = [parse_path(path) for path in lines]

    visited_one, map_of_visited_points_one = get_set_of_visited_points_for_path(paths[0])
    visited_two, map_of_visited_points_two = get_set_of_visited_points_for_path(paths[1])

    overlap = visited_one.intersection(visited_two)

    score = 100000
    for point in overlap:
        possible_score = map_of_visited_points_one[point] + map_of_visited_points_two[point]
        score = min(score, possible_score)
    return score


def get_set_of_visited_points_for_path(path):
    position = Position(0, 0)
    set_of_visited_points = set()
    map_of_visited_points = {}
    distance_to_point = 0
    for point in path:
        direction = point.direction
        distance = point.distance

        def get_new_position(position): return position
        if direction == 'U':
            def get_new_position(position): return Position(position.x, position.y + 1)

        elif direction == 'D':
            def get_new_position(position): return Position(position.x, position.y - 1)

        elif direction == 'L':
            def get_new_position(position): return Position(position.x - 1, position.y)

        elif direction == 'R':
            def get_new_position(position): return Position(position.x + 1, position.y)

        else:
            logging.critical('Illegal Instruction')
        position, map_of_visited_points, distance_to_point, set_of_visited_points = updateMap(
            distance,
            get_new_position,
            position,
            map_of_visited_points,
            distance_to_point,
            set_of_visited_points
        )
    return set_of_visited_points, map_of_visited_points


def updateMap(distance,
              get_new_position,
              position,
              map_of_visited_points,
              distance_to_point,
              set_of_visited_points):
    position = position.__copy__()
    for _ in range(distance):
        distance_to_point += 1
        position = get_new_position(position)
        if position not in map_of_visited_points:
            map_of_visited_points[position] = distance_to_point
        set_of_visited_points.add(position)
    return position, map_of_visited_points, distance_to_point, set_of_visited_points


# Tests

def test_example1():
    answer, lines = get_puzzle_tests(3, 1)
    result = part_one(lines)
    expected_answer = int(answer[0])
    assert result == expected_answer


def test_example2():
    answer, lines = get_puzzle_tests(3, 2)
    result = part_one(lines)
    expected_answer = int(answer[0])
    assert result == expected_answer


def test_example3():
    answer, lines = get_puzzle_tests(3, 3)
    result = part_one(lines)
    expected_answer = int(answer[0])
    assert result == expected_answer


def test_example1_part2():
    answer, lines = get_puzzle_tests(3, 1)
    result = part_two(lines)
    expected_answer = int(answer[1])
    assert result == expected_answer

def test_example2_part2():
    answer, lines = get_puzzle_tests(3, 2)
    result = part_two(lines)
    expected_answer = int(answer[1])
    assert result == expected_answer

def test_example3_part2():
    answer, lines = get_puzzle_tests(3, 3)
    result = part_two(lines)
    expected_answer = int(answer[1])
    assert result == expected_answer

# Code
if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    print(f'Part 1 is {part_one(get_puzzle_lines(3))}')
    print(f'Part 2 is {part_two(get_puzzle_lines(3))}')