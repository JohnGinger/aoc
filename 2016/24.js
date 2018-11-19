const PF = require("pathfinding");
const memoize = require("memoizee");

const { inputLinesArray, getAllPermutations } = require("./util");

let puzzleInputLines = inputLinesArray("24");
const gridHeight = puzzleInputLines.length;
const gridWidth = puzzleInputLines[0].length;
const grid = new PF.Grid(gridWidth, gridHeight);
const locations = [];
let start = {};

for (let y = 0; y < gridHeight; y++) {
  for (let x = 0; x < gridWidth; x++) {
    const tile = puzzleInputLines[y][x];
    if (tile === ".") {
      grid.setWalkableAt(x, y, true);
    } else if (tile === "#") {
      grid.setWalkableAt(x, y, false);
    } else {
      grid.setWalkableAt(x, y, true);
      if (puzzleInputLines[y][x] == "0") {
        start = { x, y };
      } else {
        locations.push({ x, y });
      }
    }
  }
}

const getDistance = (start, end) => {
  const gridBackup = grid.clone();
  const finder = new PF.AStarFinder();
  const path = finder.findPath(start.x, start.y, end.x, end.y, gridBackup);
  return path.length - 1;
};

const getDistanceMemo = memoize(getDistance);

let lengths = [];
for (permutation of getAllPermutations(locations)) {
  let length = getDistanceMemo(start, permutation[0]);
  for (let i = 0; i < permutation.length - 1; i++) {
    length += getDistanceMemo(permutation[i], permutation[i + 1]);
  }

  lengths.push(length);
}
console.log(`Part 1 is `, Math.min(...lengths));

let i = 0;
for (permutation of getAllPermutations(locations)) {
  lengths[i] += getDistanceMemo(permutation[permutation.length - 1], start);
  i++;
}
console.log(`Part 2 is `, Math.min(...lengths));
