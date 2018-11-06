const util = require("./util");
function generateNextRow(row) {
  nextRow = [];
  for (let i = 0; i < row.length; i++) {
    if (i === 0) {
      nextRow.push(isSafe(true, row[0], row[1]));
    } else if (i === row.length - 1) {
      nextRow.push(isSafe(row[i - 1], row[i], true));
    } else {
      nextRow.push(isSafe(row[i - 1], row[i], row[i + 1]));
    }
  }
  return nextRow;
}

function isSafe(left, center, right) {
  if (left && center && !right) {
    return false;
  }
  if (!left && center && right) {
    return false;
  }
  if (left && !center && !right) {
    return false;
  }
  if (!left && !center && right) {
    return false;
  }
  return true;
}

function parseInput(input) {
  return input.split("").map(c => c !== "^");
}

function displayRow(row) {
  console.log(row.map(x => (x ? "." : "^")).join(""));
}

let safeTiles = 0;
let puzzleInput = util.input(18);
let row = parseInput(puzzleInput);
for (let i = 0; i < 40; i++) {
  safeTiles += row.reduce((sum, tile) => sum + (tile ? 1 : 0), 0);
  displayRow(row);
  row = generateNextRow(row);
}
console.log("Part 1 is ", safeTiles);

safeTiles = 0;
row = parseInput(puzzleInput);
for (let i = 0; i < 400000; i++) {
  safeTiles += row.reduce((sum, tile) => sum + (tile ? 1 : 0), 0);
  row = generateNextRow(row);
}
console.log("Part 2 is ", safeTiles);
