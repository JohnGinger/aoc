import aoc_util
from itertools import combinations
import operator
from functools import reduce
input_string = aoc_util.get_puzzle_lines("24")

presents = [int(x.rstrip()) for x in list(input_string)]
groups_to_split_into = 4
weight = sum(presents)/groups_to_split_into


def valid_group(presents, weight, lists_to_split_into, first_group=True, qe=0):
    for group_size in range(1, len(presents) + 1):
        possible_groups = [c for c in combinations(
            presents, group_size) if sum(c) == weight]
        if len(possible_groups) == 0:
            pass
        elif lists_to_split_into == 1:
            return qe
        for group in possible_groups:
            if first_group:
                qe = reduce(operator.mul, group, 1)
            remaining_presents = list(set(presents) - set(group))
            return valid_group(remaining_presents, weight, lists_to_split_into - 1, False, qe)
    return False


print(valid_group(presents, weight, groups_to_split_into))
