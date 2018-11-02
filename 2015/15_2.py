import re
import aoc_util
import itertools
input_string = aoc_util.get_puzzle_lines("15")

ingredients = {}


def parse_line(line):
    regex = r'(\w+): capacity ([-\d]+), durability ([-\d]+), flavor ([-\d]+), texture ([-\d]+), calories ([-\d]+)'
    (name, capacity, durability, flavor, texture,
     calories) = re.match(regex, line).groups()
    ingredients[name] = [int(capacity), int(durability), int(
        flavor), int(texture), int(calories)]


for line in input_string:
    parse_line(line)


def get_value(recipe):
    batched_properties = [sum(s)
                          for s in zip(*[ingredients[x] for x in recipe])]
    score = 1
    for idx, property_value in enumerate(batched_properties):
        if idx + 1 == len(batched_properties):
            if property_value != 500:
                return 0
        else:
            if (property_value < 0):
                score = 0
            else:
                score *= property_value
    return score


print(max(get_value(x)
          for x in itertools.combinations_with_replacement(ingredients.keys(), 100)))
