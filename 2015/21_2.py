from itertools import combinations
import itertools
import sys

your_hit_points = 100
boss_hit_points = 109
boss_damage = 8
boss_armour = 2

weapons = [
    ['Dagger',       8, 4, 0],
    ['Shortsword',  10, 5, 0],
    ['Warhammer',   25, 6, 0],
    ['Longsword',   40, 7, 0],
    ['Greataxe',    74, 8, 0]]
armours = [
    ['Leather',     13, 0, 1],
    ['Chainmail',   31, 0, 2],
    ['Splintmail',  53, 0, 3],
    ['Bandedmail',  75, 0, 4],
    ['Platemail',  102, 0, 5],
    ['Nothing', 0, 0, 0]
]
rings = [
    ['Damage +1',   25, 1, 0],
    ['Damage +2',   50, 2, 0],
    ['Damage +3',  100, 3, 0],
    ['Defense +1',  20, 0, 1],
    ['Defense +2',  40, 0, 2],
    ['Defense +3',  80, 0, 3],
    ['Nothing', 0, 0, 0],
    ['Nothing', 0, 0, 0]
]


def score_round(weapon, armour, rings):
    damage_you_deal = max(
        [weapon[2] + sum([x[2] for x in rings]) - boss_armour, 1])
    damage_you_get = max(
        [boss_damage - armour[3] - sum([x[3] for x in rings]), 1])
    turn_you_die = int(your_hit_points / damage_you_get)
    turn_they_die = int(boss_hit_points / damage_you_deal)
    cost = weapon[1] + armour[1] + sum([x[1] for x in rings])

    if turn_you_die >= turn_they_die:
        return 0
    else:
        return cost


costs = [score_round(weapon, armour, rings) for rings in combinations(
    rings, 2) for armour in armours for weapon in weapons]

print(max(costs))
