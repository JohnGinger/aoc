with open("input.txt") as input:
    target_num = int(input.read())
    x = 0
    y = 0
    level = 0
    corner = 1
    index = 2
    while index <= target_num:
        if index > corner:
            level += 1
            corner += 8 * level
            x += 1
        elif x == level and y < level:
            y += 1
        elif x == -level and y > -level:
            y -= 1
        elif y == level and x > -level:
            x -= 1
        elif y == -level and x < level:
            x += 1

        if index == target_num:
            print("Part 1 ", abs(x) + abs(y))
        index += 1
