from functools import reduce
input_num = 33100000


def factors(n):
    return set(reduce(list.__add__,
                      ([i, n//i] for i in range(1, int(n**0.5) + 1) if n % i == 0)))


'''
i = 1
factor_count = {}
while True:
    number_factors = list(factors(i))
    num_presents = 0
    for factor in number_factors:
        if str(factor) not in factor_count:
            num_presents += factor * 11
            factor_count[str(factor)] = 1
        elif factor_count[str(factor)] < 50:
            num_presents += factor * 11
            factor_count[str(factor)] += 1

    if num_presents >= input_num:
        print(f'It is house {i}')
        break
    else:
        i += 1
'''
houses = [0 for x in range(1, input_num)]
for i in range(1, input_num):
    j = i
    while j < int(input_num/10):
        houses[j] += 10 * i
        if houses[i] >= input_num:
            print(f'It is house {i}')
        j += i

for i in range(1, int(input_num/10)):
    if houses[i] >= input_num:
        print(f'It is house {i}')
