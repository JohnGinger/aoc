from functools import reduce
import itertools
input_num = 33100000


def factors(n):
    return set(reduce(list.__add__,
                      ([i, n//i] for i in range(1, int(n**0.5) + 1) if n % i == 0)))


i = 1
while True:
    number_factors = list(factors(i))
    num_presents = sum([10 * x for x in number_factors])
    if num_presents >= input_num:
        print(f'It is house {i}')
        break
    else:
        i += 1
