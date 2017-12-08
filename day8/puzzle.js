const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
const lines = input.split("\n");

let indexes = {};
let maxValue = 0;
for (let line of lines) {
  const [register, instruction, amount, _, compareRegister, compareType, compareAmount] = line.split(" ");
  if (!indexes[register]){
    indexes[register] = 0;
  }
  if (!indexes[compareRegister]){
    indexes[compareRegister] = 0;
  }
  if (eval(`${indexes[compareRegister]} ${compareType}  ${Number(compareAmount)}`)){
    if (instruction === "inc"){
      indexes[register] += Number(amount);
    } else if (instruction === "dec"){
      indexes[register] -= Number(amount)
    } else {
      console.warn(`Instruction ${instruction} is not supported`)
    }
    if (indexes[register] > maxValue){
      maxValue = indexes[register];
    }
  }

}
console.log(`Part 1 is `,Math.max(...Object.values(indexes)))
console.log(`Part 2 is `,maxValue)