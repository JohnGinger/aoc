const util = require("./util");

let puzzleInputLines = util.inputLinesArray("22").slice(2);

const re = /([0-9]+)/g;
let nodes = new Map();
let maxX = 0;
let maxY = 0;
for (line of puzzleInputLines) {
  let [x, y, space, used, free] = line.match(re);
  nodes.set(`${x}-${y}`, {
    space: Number(space),
    used: Number(used),
    free: Number(free)
  });
  if (Number(x) > maxX) {
    maxX = Number(x);
  }
  if (Number(y) > maxY) {
    maxY = Number(y);
  }
}
console.log(maxX, maxY);

// https://stackoverflow.com/a/39092843
function* generateCombinations(arr, size) {
  function* doGenerateCombinations(offset, combo) {
    if (combo.length == size) {
      yield combo;
    } else {
      for (let i = offset; i < arr.length; i++) {
        yield* doGenerateCombinations(i + 1, combo.concat(arr[i]));
      }
    }
  }
  yield* doGenerateCombinations(0, []);
}

let viablePairs = new Set();
for (combination of generateCombinations(Array.from(nodes.keys()), 2)) {
  let node1 = nodes.get(combination[0]);
  let node2 = nodes.get(combination[1]);

  if (
    (node1.free >= node2.used && node2.used !== 0) ||
    (node2.free >= node1.used && node1.used !== 0)
  ) {
    viablePairs.add(`${combination[0]}-${combination[1]}`);
  }
}
console.log(Array.from(viablePairs).length);
