import collections
import re
import aoc_util
import itertools
input_string = aoc_util.get_puzzle_lines("14")
reindeers = {}

time_racing = 2503


def parse_line(line):
    (remainder, pause_time) = line.rstrip()[
        :-9].split(" seconds, but then must rest for ")
    (remainder, speed_time) = remainder.split(" km/s for ")
    (reindeer, speed) = remainder.split(" can fly ")
    reindeers[reindeer] = {"distance": 0,
                           "speed": int(speed),
                           "score": 0,
                           "speed_time": int(speed_time),
                           "pause_time": int(pause_time),
                           }


for line in input_string:
    parse_line(line)

distances = [[0 for x in range(time_racing)]
             for y in range(len(reindeers.keys()))]

for idx, r in enumerate(reindeers):
    reindeer = reindeers[r]
    reindeer["steps"] = itertools.cycle(([reindeer["speed"]] * reindeer["speed_time"] +
                                         [0]*reindeer["pause_time"]))
    distances[idx] = list(itertools.accumulate(
        next(reindeer["steps"]) for _ in range(time_racing)))

winning_distances = list(max(distance_tuple)
                         for distance_tuple in zip(*distances))

score = [sum(1 for t in range(time_racing) if distances[idx][t] ==
             winning_distances[t]) for idx, r in enumerate(reindeers)]
print(f'Max is {max(score)}')
