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
                           "speed_time": int(speed_time),
                           "pause_time": int(pause_time),
                           }


for line in input_string:
    parse_line(line)

for r in reindeers:
    reindeer = reindeers[r]
    steps = itertools.cycle(([reindeer["speed"]] * reindeer["speed_time"] +
                             [0]*reindeer["pause_time"]))
    reindeer["distance"] = sum((next(steps) for _ in range(time_racing)))


for r in reindeers:
    print(f'{r} - {reindeers[r]["distance"]}')

print(f'Max is {max(reindeers[r]["distance"] for r in reindeers)}')
