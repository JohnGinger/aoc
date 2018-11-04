const util = require("./util");

function simulate(inputRegisters, instructions) {
  let registers = inputRegisters;

  let i = 0;
  while (i >= 0 && i < instructions.length) {
    let command = instructions[i][0];
    let amount = instructions[i][1];
    let register = instructions[i][2];
    if (command === 0) {
      registers[register] = amount;
    } else if (command === 1) {
      registers[register] += amount;
    } else if (command === 2) {
      registers[register] -= amount;
    } else if (command === 3) {
      if (registers[register] !== 0) {
        i += amount - 1;
      }
    } else if (command === 4) {
      registers[register] = registers[amount];
    } else if (command === 5) {
      i += amount - 1;
    }
    i += 1;
  }
  return registers;
}

const instructionTypes = ["cpy", "inc", "dec", "jnz", "cp2", "jmp"];
const registerNames = ["a", "b", "c", "d"];
function processInput(lines) {
  let instructions = [];
  for (line of lines) {
    let [command, ...remaining] = line.split(" ");
    const commandIndex = instructionTypes.indexOf(command);
    if (commandIndex === 0) {
      let [amount, register] = remaining;
      if (isFinite(amount)) {
        instructions.push([
          commandIndex,
          Number(amount),
          registerNames.indexOf(register)
        ]);
      } else {
        instructions.push([
          instructionTypes.indexOf("cp2"),
          registerNames.indexOf(amount),
          registerNames.indexOf(register)
        ]);
      }
    } else if (commandIndex === 1) {
      let register = remaining[0];
      instructions.push([commandIndex, 1, registerNames.indexOf(register)]);
    } else if (commandIndex === 2) {
      let register = remaining[0];
      instructions.push([commandIndex, 1, registerNames.indexOf(register)]);
    } else if (commandIndex === 3) {
      let [register, amount] = remaining;
      if (isFinite(register)) {
        instructions.push([instructionTypes.indexOf("jmp"), Number(amount), 0]);
      } else {
        instructions.push([
          commandIndex,
          Number(amount),
          registerNames.indexOf(register)
        ]);
      }
    }
  }

  console.log("Part 1", simulate([0, 0, 0, 0], instructions));
  console.log("Part 2", simulate([0, 0, 1, 0], instructions));
}

processInput(util.inputLinesArray("12"));
