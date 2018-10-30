import aoc_util
input_lines = aoc_util.get_puzzle_lines("5")

vowels = list("aeiou")
banned_letters = ["ab", "cd", "pq", "xy"]


def is_nice(input):
    vowel_count = 0
    previous_letter = ''
    has_repeated_letter = False
    has_banned_letters = False
    for c in input:
        if c in vowels:
            vowel_count += 1
        if c == previous_letter:
            has_repeated_letter = True
        if f'{previous_letter}{c}' in banned_letters:
            has_banned_letters = True
        previous_letter = c

    return vowel_count >= 3 and has_repeated_letter and not has_banned_letters


nice_strings = 0
for line in input_lines:
    if is_nice(line):
        nice_strings += 1

print(f'There are {nice_strings} lines that are nice')
