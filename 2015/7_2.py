import aoc_util
input_string = aoc_util.get_puzzle_lines("7")
input_lines = list(input_string)
wires = {}


def has_value(wire):
    return wire.isdigit() or wire in wires


def get_value(wire):
    if wire.isdigit():
        return int(wire)
    elif wire in wires:
        return wires[wire]
    else:
        return 0


def got_both_inputs(first, second):
    return has_value(first) and has_value(second)


def process_line(line):
    (instruction, wire) = line.rstrip().split(" -> ")
    if wire == "b":
        wires["b"] = 16076
        return
    if instruction.isdigit():
        wires[wire] = int(instruction)
    elif "NOT " in instruction:
        (first, second) = instruction.split("NOT ")
        if not has_value(second):
            input_lines.append(line)
            return
        wires[wire] = 65535 - get_value(second)
    elif " AND " in instruction:
        (first, second) = instruction.split(" AND ")
        if not got_both_inputs(first, second):
            input_lines.append(line)
            return
        wires[wire] = get_value(first) & get_value(second)
    elif " OR " in instruction:
        (first, second) = instruction.split(" OR ")
        if not got_both_inputs(first, second):
            input_lines.append(line)
            return
        wires[wire] = get_value(first) | get_value(second)
    elif " LSHIFT " in instruction:
        (first, second) = instruction.split(" LSHIFT ")
        if not got_both_inputs(first, second):
            input_lines.append(line)
            return
        wires[wire] = get_value(first) << get_value(second)
    elif " RSHIFT " in instruction:
        (first, second) = instruction.split(" RSHIFT ")
        if not got_both_inputs(first, second):
            input_lines.append(line)
            return
        wires[wire] = get_value(first) >> get_value(second)
    else:
        if not has_value(instruction):
            input_lines.append(line)
            return
        wires[wire] = get_value(instruction)


for line in input_lines:
    process_line(line)

'''
for key in sorted(wires):
    print("%s: %s" % (key, wires[key]))
'''

print(f'wire a has signal {wires["a"]}')
