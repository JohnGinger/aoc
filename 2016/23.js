const util = require("./util");

function simulate(inputRegisters, instructions) {
  let registers = inputRegisters;

  let i = 0;
  let wg = 0;
  while (i >= 0 && i < instructions.length && wg < 100000000000000000) {
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
      console.log(address, registers);
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
  tgl: 4
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
    }
  }
  return instructions;
}

let instructions = getInstructionsFromInput(util.inputLinesArray("23"));
console.log("Part 1", simulate([7, 0, 0, 0], instructions));

instructions = getInstructionsFromInput(util.inputLinesArray("23"));
console.log("Part 2", simulate([12, 0, 0, 0], instructions));

// Translate code
/*
// cpy a b
b = a;
// jnz 1 c // This instruction needs to be changed by the toggle
while (true) {
  // dec b
  b -= 1; // b reduces by 1 each loop
  // cpy a d
  d = a;
  // cpy 0 a
  a = 0;

  // after these points a=0,b=input-loops, c= 0, d = previousLoop
  // d is a pattern like
  // input
  // input * (input - 1)
  // input * (input - 1) * (input - 2)
  // input * (input - 1) * (input - 2) * (input - 3)

  // jnz d -5
  while (d !== 0) {
    // cpy b c
    b = c;
    // jnz c -2
    while (c !== 0) {
      // inc a
      a += 1;
      // dec c
      c -= 1;
    }
    // dec d
    d -= 1;
  }
  // after this loop d=0, c=0, a=previousInput * (input - loops), b= input - loops

  // dec b
  b -= 1;
  // cpy b c
  c = b;
  // cpy c d
  d = c;
  // after these points a=previousInput * (input - loops), b = input - loops - 1, c = input - loops - 1, d = input - loops - 1

  // jnz d -2
  while (d !== 0) {
    // dec d
    d -= 1;
    // inc c
    c += 1;
  }
  // after this loop a = inputLoop * (input - loops), b = input - 2, c = 2(input - loops - 2), d = 0

  // tgl c (2(input - loops - 2) (second time this is 2(12 - 2) = 20, first time this is )
  // 1 - part 1 16 + 2(7-2) => change 26 // This is noop
  // 1 - part 2 16 + 2(12-2) => change 36 // This is noop
  // 2 - part 1 16 + 2(6-2) => change 24 // This is noop
  // 2 - part 1 16 + 2(11-2) => change 34 // This is noop
  // We need for line 18 to change
  // => part 1 has 4 loops
  // => part 2 has 9 loops
  // Both will change lines 20, 22, 24, (nothing else)
  // jnz 77 d => cpy 77 d
  // inc d => dec d
  // inc c => dec c

  // cpy -16 c
  c = -16;
  // after this point a = previousInput * (input - 1), b = input - 2, c = -16, d = 0

  // cpy 80 c
}
// When this changes jnz 1 c => cpy 1 c
// after this loop a = input * (input - 1) * (input - 2) * (input - 3) ...,
//b = input - loops, c = 1, d = 0
// For part 1 a = 7 * 6 * 5 * 4 * 3 * 2 = 2520 = 7!
// For part 2 a = 12 * 11 * 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 = 239500800 = 12!
// Therefore, with generality a = input!

c = 80;
// jnz c -5
while (c === 0) {
  // jnz 77 d ??? => cpy 77 d
  d = 77;

  // jnz d -2
  if (d === 0) {
    // inc a
    a += 1;
    // inc d => dec d
    d -= 1;
  }
  // inc c => dec c
  c -= 1;
}
// after this loop
// a=start + 80 * 77  b=input-loops c=0 d=0

// Therefore I predict
// Part 1 a = 7! + 6160 = 11200
// Part 2 a= 12! + 6160 = 479007760
*/
