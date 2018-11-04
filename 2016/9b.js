const util = require("./util");
function getLengthOfLine(line) {
  let insideBrackets = false;
  let count = 0;
  let repeatCode = "";

  for (let i = 0; i < line.length; i++) {
    if (insideBrackets) {
      repeatCode += line[i];
      if (line[i] === ")") {
        insideBrackets = false;
        let [skip, repeat] = repeatCode.slice(0, -1).split("x");
        count += +repeat * getLengthOfLine(line.slice(i + 1, i + 1 + +skip));
        i = i + +skip;
        repeatCode = "";
      }
    } else {
      if (line[i] === "(") {
        insideBrackets = true;
      } else if (line[i] !== " ") {
        count++;
      }
    }
  }
  return count;
}

function getSize(input) {
  console.log(
    input
      .split("\n")
      .map(getLengthOfLine)
      .reduce((p, c) => p + c, 0)
  );
}

getSize(util.input(9));
