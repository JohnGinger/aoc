puzzle_input = "vzbxxyzz"


def increment_char(letter):
    code = ord(letter)
    if code >= 97 and code <= 121:
        return chr(code + 1)
    elif code == 122:
        return "z"
    else:
        print("code is", code)


def increment_word(word):
    for idx in range(len(word)):
        index = len(word) - 1 - idx
        letter = word[index]
        if letter == "z":
            word[index] = "a"
        else:
            word[index] = increment_char(letter)
            return word


def is_repeating_sequence(word, idx):
    return (ord(word[idx]) + 1 == ord(word[idx + 1])) and (ord(word[idx + 1]) + 1 == ord(word[idx + 2]))


def valid_word(word):
    repeating_sequence = False
    pair_count = 0
    skip_index = -1

    if "i" in word or "o" in word or "l" in word:
        return False

    for idx, letter in enumerate(word):
        if idx + 2 < len(word) and is_repeating_sequence(word, idx):
            repeating_sequence = True
        if idx + 1 < len(word) and letter == word[idx + 1] and idx != skip_index:
            pair_count += 1
            skip_index = idx + 1

    return repeating_sequence and pair_count >= 2


new_word = list(puzzle_input)
while True:
    new_word = increment_word(new_word)
    if valid_word(new_word):
        break
print("".join(new_word))
