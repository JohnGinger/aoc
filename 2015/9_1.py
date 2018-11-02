import aoc_util
import itertools
input_string = aoc_util.get_puzzle_lines("9")

places_dict = {}
distances_dict = {}
for l in input_string:
    line = l.rstrip()
    (places, distance) = line.split(" = ")
    (from_place, to_place) = places.split(" to ")

    places_dict[from_place] = True
    places_dict[to_place] = True
    distances_dict[(from_place, to_place)] = int(distance)
    distances_dict[(to_place, from_place)] = int(distance)


def get_distance(places):
    distances = [distances_dict[x] for x in list(zip(places, places[1:]))]
    distance = 0
    for d in distances:
        distance += d
    return distance


all_distances = [get_distance(x) for x in list(
    itertools.permutations(places_dict.keys()))]

print(f'Min distance is {min(all_distances)}')
