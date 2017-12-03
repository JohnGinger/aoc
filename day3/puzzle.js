const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
const numberToFind = Number(input);

function getLevel(target) {
  let count = 1;
  let level = 0;
  while (target > count) {
    //console.log(level, count, target);
    level += 1;
    const lengthOfSide = 2 * level - 1;
    count += 4 * lengthOfSide + 4;
    if (target < count) {
      const distFromCorner = count - target;
      const sideDist = distFromCorner % (lengthOfSide + 1) - level;
      return level + Math.abs(sideDist);
    }
  }
}

console.log(getLevel(numberToFind));

let grid = new Map();
let index = 2;
let x = 0;
let y = 0;
let level = 0;
let corner = 1;
let currentCount = 1;
grid.set("x0y0", 1);
while (currentCount < numberToFind) {
  if (index > corner) {
    level += 1;
    corner += 4 * (2 * level - 1) + 4;
    x += 1;
  } else {
    const goUp = () => {
      return x == level && y < level;
    };
    const goDown = () => {
      return x == -level && y > -level;
    };
    const goLeft = () => {
      return y == level && x > -level;
    };
    const goRight = () => {
      return y == -level && x < level;
    };

    if (goUp()) {
      y += 1;
    } else if (goDown()) {
      y -= 1;
    } else if (goLeft()) {
      x -= 1;
    } else if (goRight()) {
      x += 1;
    }
  }
  const newValue =
    (grid.get(`x${x - 1}y${y}`) || 0) +
    (grid.get(`x${x}y${y - 1}`) || 0) +
    (grid.get(`x${x - 1}y${y - 1}`) || 0) +
    (grid.get(`x${x + 1}y${y}`) || 0) +
    (grid.get(`x${x}y${y + 1}`) || 0) +
    (grid.get(`x${x + 1}y${y + 1}`) || 0) +
    (grid.get(`x${x - 1}y${y + 1}`) || 0) +
    (grid.get(`x${x + 1}y${y - 1}`) || 0);

  currentCount = newValue;
  grid.set(`x${x}y${y}`, newValue);
  let line = "";
  for (let ydraw = level + 1; ydraw >= -level - 1; ydraw--) {
    for (let xdraw = -level - 1; xdraw <= level + 1; xdraw++) {
      if (grid.has(`x${xdraw}y${ydraw}`)) {
        line += `${grid.get(`x${xdraw}y${ydraw}`)}`.padStart(7);
      } else {
        line += `.......`;
      }
    }
    line += "\n";
  }
  console.log(line);
  index += 1;
}
//console.log(grid);
