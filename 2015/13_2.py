import aoc_util
import itertools
input_string = aoc_util.get_puzzle_lines("13")

people_dict = {"me": True}
happiness_dict = {}


def parse_line(line):
    (what, person1) = line.rstrip()[:-1].split(
        " happiness units by sitting next to ")
    (person2, action) = what.split(" would ")
    (gain_or_lose, amount) = action.split(" ")
    people_dict[person1] = True
    people_dict[person2] = True
    happiness_dict[(person1, "me")] = 0
    happiness_dict[("me", person1)] = 0
    happiness_dict[(person1, person2)] = int(
        f'{"+" if gain_or_lose == "gain" else "-" }{amount}')


for line in input_string:
    parse_line(line)


def score_arrangement(arrangement):
    happinesses1 = [happiness_dict[x]
                    for x in list(zip(arrangement, arrangement[1:]))]
    happinesses1.append(happiness_dict[(arrangement[-1], arrangement[0])])
    total_happiness = 0
    for h in happinesses1:
        total_happiness += h

    happinesses2 = [happiness_dict[x]
                    for x in list(zip(arrangement[1:], arrangement))]
    happinesses2.append(happiness_dict[(arrangement[0], arrangement[-1])])

    for h in happinesses2:
        total_happiness += h
    return total_happiness


all_happinesses = [score_arrangement(x) for x in list(
    itertools.permutations(people_dict.keys()))]

print(max(all_happinesses))
