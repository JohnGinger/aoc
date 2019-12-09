import logging
from util import get_puzzle_text
# logging.basicConfig(level=logging.DEBUG)


def get(program, instruction_pointer, modes, ident):
    val = None
    a_mode = modes % 10
    b_mode = int((modes % 100) / 10)
    c_mode = int(modes / 100)

    if ident == 'a':
        val = program[instruction_pointer+1]
        immediate_mode = a_mode == 1
    elif ident == 'b':
        val = program[instruction_pointer+2]
        immediate_mode = b_mode == 1
    elif ident == 'c':
        val = program[instruction_pointer+3]
        immediate_mode = c_mode == 1
    elif ident == 'raw':
        return program[instruction_pointer+3]

    if immediate_mode:
        logging.debug('%s = %s', ident, val)
        return val
    else:
        logging.debug('%s = m[%s] = %s', ident, val, program[val])
        return program[val]


def run_program(program_string, input_array=[]):
    program = [int(x) for x in program_string.strip().split(',')]
    instruction_pointer = 0
    while True:
        opcode_var = program[instruction_pointer]
        opcode = opcode_var % 100
        modes = int(opcode_var / 100)
        logging.debug('Opcode is %s', opcode)

        if opcode == 99:
            break
        elif opcode == 0:
            instruction_pointer += 1
            logging.debug('Raw memory %s', opcode_var)
            continue

        if opcode == 1:
            a = get(program, instruction_pointer, modes, 'a')
            b = get(program, instruction_pointer, modes, 'b')
            c = get(program, instruction_pointer, modes, 'raw')
            logging.debug('m[%s] = %s + %s', c, a, b)
            program[c] = a + b
            instruction_pointer += 4
        elif opcode == 2:
            a = get(program, instruction_pointer, modes, 'a')
            b = get(program, instruction_pointer, modes, 'b')
            c = get(program, instruction_pointer, modes, 'raw')
            logging.debug('m[%s] = %s * %s', c, a, b)
            program[c] = a * b
            instruction_pointer += 4
        elif opcode == 3:
            a = program[instruction_pointer + 1]
            logging.debug('Get input put in %s', a)
            program[a] = input_array.pop(0)
            instruction_pointer += 2
        elif opcode == 4:
            a = get(program, instruction_pointer, modes, 'a')
            logging.debug('Output %s', a)
            yield a
            instruction_pointer += 2
        elif opcode == 5:
            a = get(program, instruction_pointer, modes, 'a')
            b = get(program, instruction_pointer, modes, 'b')
            logging.debug('if %s != 0 goto %s', a, b)
            if a != 0:
                instruction_pointer = b
            else:
                instruction_pointer += 3
        elif opcode == 6:
            a = get(program, instruction_pointer, modes, 'a')
            b = get(program, instruction_pointer, modes, 'b')
            logging.debug('if %s == 0 goto %s', a, b)
            if a == 0:
                instruction_pointer = b
            else:
                instruction_pointer += 3
        elif opcode == 7:
            a = get(program, instruction_pointer, modes, 'a')
            b = get(program, instruction_pointer, modes, 'b')
            c = get(program, instruction_pointer, modes, 'raw')
            logging.debug('m[%s] = %s < %s', c, a, b)
            if a < b:
                program[c] = 1
            else:
                program[c] = 0
            instruction_pointer += 4
        elif opcode == 8:
            a = get(program, instruction_pointer, modes, 'a')
            b = get(program, instruction_pointer, modes, 'b')
            c = get(program, instruction_pointer, modes, 'raw')
            logging.debug('m[%s] = %s == %s', c, a, b)
            if a == b:
                program[c] = 1
            else:
                program[c] = 0
            instruction_pointer += 4
        else:
            print('Bad Instruction')
            print(opcode, instruction_pointer, program)
            break
    yield program


def test_basic():
    program = '1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,2,9,23,27,1,6,27,31,1,31,9,35,2,35,10,39,1,5,39,43,2,43,9,47,1,5,47,51,1,51,5,55,1,55,9,59,2,59,13,63,1,63,9,67,1,9,67,71,2,71,10,75,1,75,6,79,2,10,79,83,1,5,83,87,2,87,10,91,1,91,5,95,1,6,95,99,2,99,13,103,1,103,6,107,1,107,5,111,2,6,111,115,1,115,13,119,1,119,2,123,1,5,123,0,99,2,0,14,0'
    result = next(run_program(program))
    assert result[0] == 3101844


def test_multiple_immediate():
    program = '1002,4,3,4,33'
    result = next(run_program(program))
    assert result == [1002, 4, 3, 4, 99]


def test_multiple_immediate_two():
    program = '1101,100,-1,4,0'
    result = next(run_program(program))
    assert result == [1101, 100, -1, 4, 99]


def test_debug():
    program = get_puzzle_text(5)
    result = run_program(program, [1])
    output = next(result)
    while True:
        if output != 0:
            break
        output = next(result)
    assert output == 13087969


def test_equal_to():
    program = '3,9,8,9,10,9,4,9,99,-1,8'
    result = run_program(program, [8])
    output = next(result)
    print(next(result))
    assert output == 1
    result = run_program(program, [7])
    output = next(result)
    assert output == 0


def test_less_than():
    program = '3,9,7,9,10,9,4,9,99,-1,8'
    result = run_program(program, [8])
    output = next(result)
    assert output == 0
    result = run_program(program, [7])
    output = next(result)
    assert output == 1


def test_equal_to_immediate():
    program = '3,3,1108,-1,8,3,4,3,99'
    result = run_program(program, [8])
    output = next(result)
    assert output == 1
    result = run_program(program, [7])
    output = next(result)
    assert output == 0


def test_less_than_immediate():
    program = '3,3,1107,-1,8,3,4,3,99'
    result = run_program(program, [8])
    output = next(result)
    assert output == 0
    result = run_program(program, [7])
    output = next(result)
    assert output == 1


def test_jump():
    program = '3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9'
    result = run_program(program, [0])
    output = next(result)
    assert output == 0
    result = run_program(program, [7])
    output = next(result)
    assert output == 1


def test_jump_immediate():
    program = '3,3,1105,-1,9,1101,0,0,12,4,12,99,1'
    result = run_program(program, [0])
    output = next(result)
    assert output == 0
    result = run_program(program, [7])
    output = next(result)
    assert output == 1


def test_combined():
    program = '3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99'
    result = run_program(program, [1])
    output = next(result)
    assert output == 999
    result = run_program(program, [8])
    output = next(result)
    assert output == 1000
    result = run_program(program, [10])
    output = next(result)
    assert output == 1001
