const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });

const runInstructions = instructions => {
  let x = 0;
  let y = 0;
  let maxDistance = 0;
  let distance = 0;
  for (instruction of instructions.split(",")) {
    switch (instruction) {
      case "n":
        y += 1;
        break;
      case "ne":
        y += 0.5;
        x += 0.5;
        break;
      case "nw":
        y += 0.5;
        x -= 0.5;
        break;
      case "se":
        y -= 0.5;
        x += 0.5;
        break;
      case "sw":
        y -= 0.5;
        x -= 0.5;
        break;
      case "s":
        y -= 1;
        break;
    }
    distance = Math.abs(x) + Math.abs(y);
    if (distance > maxDistance) {
      maxDistance = distance;
    }
  }

  return {
    shortest: distance,
    maxDistance
  };
};

console.log(runInstructions("ne,ne,ne"));
console.log(runInstructions("ne,ne,sw,sw"));
console.log(runInstructions("ne,ne,s,s"));
console.log(runInstructions("se,sw,se,sw,sw"));
console.log("Part 1 is ", runInstructions(input).shortest);
console.log("Part 2 is ", runInstructions(input).maxDistance);
