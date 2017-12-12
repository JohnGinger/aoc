const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
let chars = input.split("");
let stack = [];
let inGarbage = false;
let ignoreNext = false;
let scorePart1 = 0;
let scorePart2 = 0;
for (char of chars) {
  if (inGarbage) {
    if (ignoreNext) {
      ignoreNext = false;
      continue;
    } else if (char === "!") {
      ignoreNext = true;
    } else if (char === ">") {
      inGarbage = false;
    } else {
      scorePart2 += 1;
    }
  } else if (char === "<") {
    inGarbage = true;
  } else if (char === "{") {
    stack.unshift("{");
  } else if (char === "}") {
    if (stack[0] == "{") {
      stack.shift();
      scorePart1 += stack.length + 1;
    }
  }
}
console.log("Part 1 is ", scorePart1);
console.log("Part 2 is ", scorePart2);
