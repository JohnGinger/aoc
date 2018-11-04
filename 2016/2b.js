const util = require("./util");

let values = [
  {
    x: 0,
    y: 0,
    v: 5
  },
  {
    x: 1,
    y: 0,
    v: 6
  },
  {
    x: 2,
    y: 0,
    v: 7
  },
  {
    x: 3,
    y: 0,
    v: 8
  },
  {
    x: 4,
    y: 0,
    v: 9
  },
  {
    x: 1,
    y: 1,
    v: 2
  },
  {
    x: 2,
    y: 1,
    v: 3
  },
  {
    x: 3,
    y: 1,
    v: 4
  },
  {
    x: 2,
    y: 2,
    v: 1
  },
  {
    x: 2,
    y: -2,
    v: "D"
  },
  {
    x: 1,
    y: -1,
    v: "A"
  },
  {
    x: 2,
    y: -1,
    v: "B"
  },
  {
    x: 3,
    y: -1,
    v: "C"
  }
];

function clamp(value, min, max) {
  value = value > max ? max : value;
  value = value < min ? min : value;
  return value;
}

let getNumber = (line, x, y) => {
  let instructions = line.split("");
  for (instruction of instructions) {
    if (instruction === "U" || instruction === "D") {
      let direction = instruction === "U" ? 1 : -1;
      if (x === 0 || x === 4) {
        y = 0;
      }
      if (x === 1 || x === 3) {
        y = clamp(y + direction, -1, 1);
      }
      if (x === 2) {
        y = clamp(y + direction, -2, 2);
      }
    }
    if (instruction === "L" || instruction === "R") {
      let direction = instruction === "R" ? 1 : -1;
      if (y === -2 || y === 2) {
        x = 2;
      }
      if (y === -1 || y === 1) {
        x = clamp(x + direction, 1, 3);
      }
      if (y === 0) {
        x = clamp(x + direction, 0, 4);
      }
    }
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

console.log(getCode(util.input(2)));
