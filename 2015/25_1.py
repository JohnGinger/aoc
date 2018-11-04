import re
import aoc_util
input_string = [l.rstrip() for l in aoc_util.get_puzzle_lines("25")][0]
print(input_string)
regex = r'To continue, please consult the code grid in the manual.  Enter the code at row ([\d]+), column ([\d]+).'
(r, c) = re.match(regex, input_string).groups()
row = int(r)
column = int(c)
print(row, column)

triangle_base = row + column
triangle_height = triangle_base - 1

position_in_sequence = int(
    (triangle_base * triangle_height) / 2) - (row - 1) - 1

print((pow(252533, position_in_sequence, 33554393) * 20151125) % 33554393)

row_counter = 1
column_counter = 1
value = 20151125
while True:
    if row_counter == row and column_counter == column:
        print('value is', value)
        break
    else:
        value = value * 252533 % 33554393
        if row_counter == 1:
            row_counter = column_counter + 1
            column_counter = 1
        else:
            row_counter -= 1
            column_counter += 1
