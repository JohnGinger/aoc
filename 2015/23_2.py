import aoc_util

input_string = aoc_util.get_puzzle_lines("23")

code = []
instructions = ["hlf", "tfl", "inc", "jmp", "jie", "jio", "inc", "tpl"]
register_names = ["a", "b", "-"]
registers = [1, 0]

num_instructions = 0
for idx, line in enumerate(input_string):
    parts = line.rstrip().split(" ")
    amount = 0
    register = "-"
    if len(parts) == 2:
        (instruction, other) = parts
        if instruction == "jmp":
            amount = int(other)
        else:
            register = other
    else:
        (instruction, register, amount) = parts
        register = register[:-1]
        amount = int(amount)

    code = code + [(instructions.index(instruction),
                    register_names.index(register), amount)]
    num_instructions = idx


i = 0
while i >= 0 and i <= num_instructions:
    instruction = code[i][0]
    register = code[i][1]
    if instruction == 0:
        registers[register] = registers[register] / 2
    elif instruction == 1:
        registers[register] = registers[register] * 2
    elif instruction == 2:
        registers[register] += 1
    elif instruction == 3:
        i += (code[i][2] - 1)
    elif instruction == 4:
        if registers[register] % 2 == 0:
            i += (code[i][2] - 1)
    elif instruction == 5:
        if registers[register] == 1:
            i += (code[i][2] - 1)
    elif instruction == 6:
        registers[register] += 1
    elif instruction == 7:
        registers[register] *= 3
    else:
        raise ValueError("Something has gone badly wrong")
    i += 1

print("direct", registers[1])

# Translated code
a = 1
b = 0
if a == 1:
    # a = (((((((((((a + 1) * 3 + 1) * 18)) + 1) * 3) + 1) * 3) + 2) * 18) + 1)
    a = 31911
else:
    # a = ((((((((((((((3 * a) + 1) * 3) + 2) * 3) + 1) * 3) + 2) * 9) + 2) * 3) + 2) * 3) + 2)
    a = 26623

while a != 1:
    b += 1
    if a % 2 == 0:
        a /= 2
    else:
        a *= 3
        a += 1

print("translated", b)
