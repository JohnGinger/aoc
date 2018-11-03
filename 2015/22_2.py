from itertools import permutations, combinations_with_replacement
from aoc_util import next_permutation
import sys

your_hit_points = 50
your_mana = 500
boss_hit_points = 55
boss_damage = 8

# [name,cost,damage,healing,armour,mana,effect_length]
attacks = [
    ['Magic Missile', 53, 4, 0, 0, 0, 1],
    ['Drain',  73, 2, 2, 1, 0, 0, 1],
    ['Shield',   113, 0, 0, 7, 0, 6],
    ['Poison',   173, 3, 0, 0, 0, 6],
    ['Recharge', 229, 0, 0, 0, 101, 5]]


def cost_combination(combination):
    return sum([x[1] for x in combination])


def apply_effects(counters, hps, mana, has_shield):
    if counters["shield"] > 0:
        counters["shield"] -= 1
        debug_print(f"Shield's timer is now {counters['shield']}.")
        if counters["shield"] == 0:
            debug_print(
                f"Shield wears off.")
    else:
        has_shield = False

    if counters["poison"] > 0:
        counters["poison"] -= 1
        hps["boss"] -= attacks[3][2]
        debug_print(
            f"Poison deals {attacks[3][2]} damage; its timer is now {counters['poison']}.")
        if counters["poison"] == 0:
            debug_print(
                f"Poison wears off.")

    if counters["recharge"] > 0:
        counters["recharge"] -= 1
        mana += 101
        debug_print(
            f"Recharge provides 101 mana its timer is now {counters['recharge']}.")
        if counters["recharge"] == 0:
            debug_print(
                f"Recharge wears off.")
    return (counters, hps, has_shield, mana)


def apply_attack(attack, counters, hps, has_shield):
    valid_attack = True
    if attack[0] == 'Magic Missile':
        hps["boss"] -= attack[2]

    if attack[0] == 'Drain':
        hps["you"] += attack[3]
        hps["boss"] -= attack[2]

    if attack[0] == 'Shield':
        if counters["shield"] > 0:
            valid_attack = False
        else:
            counters["shield"] = attack[6]
            has_shield = True

    if attack[0] == 'Poison':
        if counters["poison"] > 0:
            valid_attack = False
        else:
            counters["poison"] = attack[6]

    if attack[0] == 'Recharge':
        if counters["recharge"] > 0:
            valid_attack = False
        else:
            counters["recharge"] = attack[6]

    return (valid_attack, counters, hps, has_shield)


def debug_print(*text):
    return
    print(*text)


def print_stats(hps, has_shield, mana):
    debug_print(
        f"- Player has {hps['you']} hit points, {7 if has_shield else 0} armor, {mana} mana")
    debug_print(f"- Boss has {hps['boss']} hit points")


def valid_combination(combination):
    for attacks in next_permutation(list(combination)):
        hps = {"you": your_hit_points, "boss": boss_hit_points}
        mana = your_mana
        has_shield = False
        counters = {"shield": 0, "poison": 0, "recharge": 0}
        debug_print("attacks", [x[0] for x in attacks])
        for attack in attacks:
            debug_print("-- Player turn --")
            hps["you"] -= 1
            if hps["you"] <= 0:
                debug_print("You die")
                break

            print_stats(hps, has_shield, mana)
            (counters, hps, has_shield, mana) = apply_effects(
                counters, hps, mana, has_shield)

            if mana < attack[1]:
                debug_print("You run out of mana")
                break
            debug_print(f"Player casts {attack[0]}")
            (valid_attack, counters, hps, has_shield) = apply_attack(
                attack, counters, hps, has_shield)
            if not valid_attack:
                debug_print("attack invalid")
                break
            else:
                mana -= attack[1]

            if hps["boss"] <= 0:
                debug_print("You win")
                return True

            debug_print()
            debug_print("-- Boss turn --")
            print_stats(hps, has_shield, mana)
            (counters, hps, has_shield, mana) = apply_effects(
                counters, hps, mana, has_shield)

            if hps["boss"] <= 0:
                debug_print("You win")
                return True

            armour = 7 if has_shield else 0
            debug_print(
                f"Boss attacks for {boss_damage} - {armour} = {(boss_damage - armour)} damage!")

            hps["you"] -= (boss_damage - armour)
            if hps["you"] <= 0:
                debug_print("You die")
                break
            debug_print()

    debug_print("You lose")
    return False


min_cost = 10000000
for number_of_rounds in range(9, 10):
    combinations = combinations_with_replacement(
        attacks, number_of_rounds)
    for combination in combinations:
        cost = cost_combination(combination)
        if cost < min_cost:
            if valid_combination(combination):
                min_cost = cost
    print(min_cost)
