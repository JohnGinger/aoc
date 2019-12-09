def get_puzzle_text(day):
    file = open(f'../data/{day}.txt', "r")
    return file.read()


def get_puzzle_lines(day):
    return open(f'../data/{day}.txt', "r")


def get_puzzle_tests(day, test):
    file = open(f'../data/{day}t{test}.txt', "r")
    lines = file.read().splitlines()
    answer = lines[0].split(' ')
    data = lines[1:]
    return answer, data