const PF = require("pathfinding");
const input = 1350;

const target = { x: 31, y: 39 };
const extend = 10;

function isWalkable(x, y, inputNumber) {
  let num = x * x + 3 * x + 2 * x * y + y + y * y;
  let binary = Number(num + inputNumber).toString(2);
  let numBits = binary
    .split("")
    .map(Number)
    .reduce((s, n) => s + n);
  return numBits % 2 === 0;
}

function drawMap(input, biggestX, biggestY, path) {
  line = "  ";
  for (let x = 0; x < biggestX; x++) {
    line += x % 9;
  }
  console.log(line);

  for (let y = 0; y < target.y + extend; y++) {
    line = (y % 9) + " ";
    for (let x = 0; x < target.x + extend; x++) {
      if (path.some(p => p[0] === x && p[1] === y)) {
        line += "0";
      } else {
        line += isWalkable(x, y, input) ? "." : "#";
      }
    }
    console.log(line);
  }
  console.log();
}

drawMap(input, target.x + extend, target.y + extend, []);
var grid = new PF.Grid(target.x + extend, target.y + extend);

for (let y = 0; y < target.y + extend; y++) {
  for (let x = 0; x < target.x + extend; x++) {
    grid.setWalkableAt(x, y, isWalkable(x, y, input));
  }
}

var finder = new PF.AStarFinder();
var path = finder.findPath(1, 1, target.x, target.y, grid);
drawMap(input, target.x + extend, target.y + extend, path);
console.log("Part 1 is", path.length - 1);

var grid2 = new PF.Grid(51, 51);

for (let y = 0; y < 51; y++) {
  for (let x = 0; x < 51; x++) {
    grid2.setWalkableAt(x, y, isWalkable(x, y, input));
  }
}

let reachLessThan50 = 0;
for (let y = 0; y < 51; y++) {
  for (let x = 0; x < 51; x++) {
    var finder = new PF.AStarFinder();
    let gridClone = grid2.clone();
    let path = finder.findPath(1, 1, x, y, gridClone);
    if (path.length > 0 && path.length <= 51) {
      reachLessThan50 += 1;
    }
  }
}
console.log("Part 2 is", reachLessThan50);
