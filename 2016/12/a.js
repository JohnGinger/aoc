function processCommand(registers, line) {
    //console.log(line);
    let commands = line.split(' ');
    let jump = 0;
    switch (commands[0]) {
        case 'cpy':
            let incrementTo = parseInt(commands[1], 10);
            if (isNaN(incrementTo)) {
                incrementTo = registers[commands[1]]
            }
            registers[commands[2]] = incrementTo;
            break;
        case 'inc':
            registers[commands[1]]++
                break;
        case 'dec':
            registers[commands[1]]--
                break;
        case 'jnz':
            if (registers[commands[1]] !== 0 &&
                +commands[1] !== 0) {
                jump = +commands[2] - 1;
            }
            break;
    }
    return {
        registers,
        jump
    };
}

function processInput(input) {
    let lines = input.split('\n');
    let registers = {
        'a': 0,
        'b': 0,
        'c': 1,
        'd': 0
    }
    let jump = 0;
    for (let i = 0; i < lines.length; i += 0) {
        ({
            registers,
            jump
        } = processCommand(registers, lines[i]));
        i += 2 + (jump - 1);
        //console.log(registers);
    }    
    console.log(registers);
}

processInput(require('./data'))