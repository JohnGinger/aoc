const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
const lines = input.split("\n").map(line => line.split("\t").map(Number));
const part1 = lines.reduce((p, c) => {
  const minMax = c.reduce(
    (p, c) => {
      if (c > p.max) {
        p.max = c;
      }
      if (c < p.min) {
        p.min = c;
      }
      return p;
    },
    { min: 10000000000, max: 0 }
  );
  return p + Number(minMax.max - minMax.min);
}, 0);

console.log("Part1", part1);

const part2 = lines.reduce((p, c) => {
  const evenDivide = c.reduce((p, c, i, arr) => {
    for (let checkIndex = i + 1; checkIndex < arr.length; checkIndex++) {
      if (Number.isInteger(c / arr[checkIndex])) {
        return c / arr[checkIndex];
      }
      if (Number.isInteger(arr[checkIndex] / c)) {
        return arr[checkIndex] / c;
      }
    }
    return p;
  }, 0);
  return p + evenDivide;
}, 0);

console.log("Part1", part2);
