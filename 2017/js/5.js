const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
let numbers = Array.from(input.split("\n"), Number);
let position = 0;
let count = 0;
while (position < numbers.length && position >= 0) {
  let jump = numbers[position];
  if (jump >= 3) {
    numbers[position] -= 1;
  } else {
    numbers[position] += 1;
  }
  position += jump;
  count++;
}
console.log("Part 2 is ", count);
