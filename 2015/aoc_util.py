def get_puzzle_text(day):
    file = open(f'./{day}.txt', "r")
    return file.read()


def get_puzzle_lines(day):
    return open(f'./{day}.txt', "r")
