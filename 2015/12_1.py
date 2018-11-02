import aoc_util
import re
input_string = aoc_util.get_puzzle_text("12")

numbers = re.sub(r'[^\d-]+', "+", input_string)

print(eval(numbers + "0"))
