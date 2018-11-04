const util = require("./util");

let values = [
  {
    x: 0,
    y: 0,
    v: 5
  },
  {
    x: -1,
    y: 0,
    v: 4
  },
  {
    x: 1,
    y: 0,
    v: 6
  },
  {
    x: 0,
    y: -1,
    v: 8
  },
  {
    x: -1,
    y: -1,
    v: 7
  },
  {
    x: 1,
    y: -1,
    v: 9
  },
  {
    x: 0,
    y: 1,
    v: 2
  },
  {
    x: -1,
    y: 1,
    v: 1
  },
  {
    x: 1,
    y: 1,
    v: 3
  }
];
let getNumber = (line, x, y) => {
  let instructions = line.split("");
  for (instruction of instructions) {
    if (instruction === "U") {
      y++;
    }
    if (instruction === "D") {
      y--;
    }
    if (instruction === "L") {
      x--;
    }
    if (instruction === "R") {
      x++;
    }
    x = x > 1 ? 1 : x;
    x = x < -1 ? -1 : x;

    y = y > 1 ? 1 : y;
    y = y < -1 ? -1 : y;
  }
  return {
    result: values.find(v => v.x === x && v.y === y).v,
    x,
    y
  };
};

let getCode = input => {
  let lines = input.split("\n");
  let x = 0;
  let y = 0;
  return lines
    .map(line => {
      ({ result, x, y } = getNumber(line, x, y));
      return result;
    })
    .join("");
};

console.log(getCode(util.test(2, 1)));
console.log(getCode(util.input(2)));
