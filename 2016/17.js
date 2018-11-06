const crypto = require("crypto");

valid = new Set(["b", "c", "d", "e", "f"]);
function getWalkableDirections(pathSoFar, puzzleInput) {
  let hash = crypto
    .createHash("md5")
    .update(puzzleInput + pathSoFar)
    .digest("hex");
  let [up, down, left, right] = hash.slice(0, 4).split("");

  return {
    up: valid.has(up),
    down: valid.has(down),
    left: valid.has(left),
    right: valid.has(right)
  };
}

const puzzleInput = "veumntbg";
let thingsToEvaluate = [
  {
    path: "",
    position: { x: 0, y: 0 }
  }
];

let validPaths = [];
while (thingsToEvaluate.length > 0) {
  let { path, position } = thingsToEvaluate.pop();
  if (position.x === 3 && position.y === 3) {
    validPaths.push(path);
  } else {
    let { up, down, left, right } = getWalkableDirections(path, puzzleInput);
    if (up && position.y > 0) {
      thingsToEvaluate.unshift({
        position: { x: position.x, y: position.y - 1 },
        path: path + "U"
      });
    }
    if (down && position.y < 3) {
      thingsToEvaluate.unshift({
        position: { x: position.x, y: position.y + 1 },
        path: path + "D"
      });
    }
    if (left && position.x > 0) {
      thingsToEvaluate.unshift({
        position: { x: position.x - 1, y: position.y },
        path: path + "L"
      });
    }
    if (right && position.x < 3) {
      thingsToEvaluate.unshift({
        position: { x: position.x + 1, y: position.y },
        path: path + "R"
      });
    }
  }
}

validPaths.sort((a, b) => a.length - b.length);
console.log("Part 1 is ", validPaths[0]);
console.log("Part 2 is ", validPaths[validPaths.length - 1].length);
