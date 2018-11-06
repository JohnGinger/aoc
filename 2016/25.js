const util = require("./util");

function simulate(inputRegisters, instructions) {
  let registers = inputRegisters;

  let i = 0;
  let wg = 0;
  let outString = "";
  let toCheck = "010101010101010101010101010101";
  while (i >= 0 && i < instructions.length && wg < 1000000000) {
    let command = instructions[i][0];
    let amount = instructions[i][1];
    let register = instructions[i][2];
    let arg = instructions[i][3];

    if (command === instructs.cpy) {
      // Two types cpy num register and cpy register register (cpy register num, cpy number number are invalid)

      if (arg === args.registerRegister) {
        // cpy register register
        registers[register] = registers[amount];
      } else if (arg === args.numRegister) {
        registers[register] = amount;
      } else {
        // This is invalid - can ignore
        console.warn("Invalid - ignore");
      }
    } else if (command === instructs.inc) {
      registers[register] += amount;
    } else if (command === instructs.dec) {
      registers[register] -= amount;
    } else if (command === instructs.jnz) {
      // 4 types jnz num register,  jnz register num, jnz register register , jnz number number
      // Because it is annoying the order is unusual
      let first = amount;
      let second = register;

      if (arg === args.registerRegister) {
        if (registers[first] !== 0) {
          i += registers[second] - 1;
        }
      } else if (arg === args.registerNum) {
        if (registers[first] !== 0) {
          i += second - 1;
        }
      } else if (arg === args.numRegister) {
        if (first !== 0) {
          i += registers[second] - 1;
        }
      } else if (arg === args.numNum) {
        if (first !== 0) {
          i += second - 1;
        }
      } else {
        console.warn("This shouldn't ever happen");
      }
    } else if (command === instructs.tgl) {
      let address = registers[register] + i;
      if (address < instructions.length) {
        let command = instructions[address][0];
        if (command === instructs.inc) {
          instructions[address][0] = instructs.dec;
        } else if (command === instructs.dec || command === instructs.tgl) {
          instructions[address][0] = instructs.inc;
        } else if (command === instructs.jnz) {
          instructions[address][0] = instructs.cpy;
        } else if (command === instructs.cpy) {
          instructions[address][0] = instructs.jnz;
        }
      }
    } else if (command === instructs.out) {
      outString += registers[register].toString();
      if (toCheck.startsWith(outString)) {
        if (outString.length === toCheck.length) {
          return true;
        }
      } else {
        return false;
      }
    }

    wg += 1;
    i += 1;
  }

  return registers;
}

const instructs = {
  cpy: 0,
  inc: 1,
  dec: 2,
  jnz: 3,
  tgl: 4,
  out: 5
};

const args = {
  num: 0,
  register: 1,
  registerNum: 2,
  numRegister: 3,
  registerRegister: 4,
  numNum: 5
};

const registerNames = ["a", "b", "c", "d"];

function getInstructionsFromInput(lines) {
  let instructions = [];
  for (line of lines) {
    let [command, ...remaining] = line.split(" ");
    if (command === "cpy") {
      let [amount, register] = remaining;
      if (isFinite(amount)) {
        instructions.push([
          instructs.cpy,
          Number(amount),
          registerNames.indexOf(register),
          args.numRegister
        ]);
      } else {
        instructions.push([
          instructs.cpy,
          registerNames.indexOf(amount),
          registerNames.indexOf(register),
          args.registerRegister
        ]);
      }
    } else if (command === "inc") {
      let register = remaining[0];
      instructions.push([
        instructs.inc,
        1,
        registerNames.indexOf(register),
        args.register
      ]);
    } else if (command === "dec") {
      let register = remaining[0];
      instructions.push([
        instructs.dec,
        1,
        registerNames.indexOf(register),
        args.register
      ]);
    } else if (command === "jnz") {
      let [first, second] = remaining;
      if (isFinite(first) && isFinite(second)) {
        instructions.push([
          instructs.jnz,
          Number(first),
          Number(second),
          args.numNum
        ]);
      } else if (isFinite(first)) {
        instructions.push([
          instructs.jnz,
          Number(first),
          registerNames.indexOf(second),
          args.numRegister
        ]);
      } else if (isFinite(second)) {
        instructions.push([
          instructs.jnz,
          registerNames.indexOf(first),
          Number(second),
          args.registerNum
        ]);
      } else {
        instructions.push([
          instructs.jnz,
          Number(first),
          Number(second),
          args.numNum
        ]);
      }
    } else if (command === "tgl") {
      let [register] = remaining;
      instructions.push([
        instructs.tgl,
        1,
        registerNames.indexOf(register),
        args.register
      ]);
    } else if (command === "out") {
      let [register] = remaining;
      instructions.push([
        instructs.out,
        1,
        registerNames.indexOf(register),
        args.register
      ]);
    } else {
      console.warn("Bad instruction is", line);
    }
  }
  return instructions;
}

let instructions = getInstructionsFromInput(util.inputLinesArray("25"));
for (let i = 0; i < 100000; i++) {
  console.log("Input is ", i);

  if (simulate([i, 0, 0, 0], instructions)) {
    console.log("Part 1 is ", i);
    break;
  }
}
