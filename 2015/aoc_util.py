def get_puzzle_text(day):
    file = open(f'./{day}.txt', "r")
    return file.read()


def get_puzzle_lines(day):
    return open(f'./{day}.txt', "r")


def next_permutation_in_place(l):
    n = len(l)
    # Step 1: Find tail
    last = n-1  # tail is from `last` to end
    while last > 0:
        if l[last-1] < l[last]:
            break
        last -= 1
    # Step 2: Increase the number just before tail
    if last > 0:
        small = l[last-1]
        big = n-1
        while l[big] <= small:
            big -= 1
        l[last-1], l[big] = l[big], small
    # Step 3: Reverse tail
    i = last
    j = n-1
    while i < j:
        l[i], l[j] = l[j], l[i]
        i += 1
        j -= 1
    return last > 0


def next_permutation(l):
    yield l
    while next_permutation_in_place(l):
        yield l


def split_chunks(l, n):
    """ 
       Splits list l into n chunks with approximately equals sum of values
       see  http://stackoverflow.com/questions/6855394/splitting-list-in-chunks-of-balanced-weight
    """
    result = [[] for i in range(n)]
    sums = {i: 0 for i in range(n)}
    c = 0
    for e in l:
        for i in sums:
            if c == sums[i]:
                result[i].append(e)
                break
        sums[i] += e
        c = min(sums.values())
    return result
