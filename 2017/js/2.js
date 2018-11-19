const { inputLinesArray } = require("./aoc_util");

const lines = inputLinesArray("2").map(line => line.split("\t").map(Number));
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

console.log("Part 1 is", part1);

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

console.log("Part 2 is", part2);
