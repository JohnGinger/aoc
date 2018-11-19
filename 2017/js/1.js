const { input } = require("./aoc_util");
const total1 = Array.from(input(1), Number).reduce(
  (p, c, i, nums) => (c == nums[(i + 1) % nums.length] ? p + c : p),
  0
);

console.log("Part one is ", total1);

const total2 = Array.from(input(1), Number).reduce(
  (p, c, i, nums) =>
    c == nums[(i + nums.length / 2) % nums.length] ? p + c : p,
  0
);

console.log("Part two is", total2);
