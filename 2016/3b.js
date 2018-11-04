const util = require("./util");

function getPoints(line) {
  let a = +line.slice(2, 6).trim();
  let b = +line.slice(7, 11).trim();
  let c = +line.slice(12, 16).trim();
  return { a, b, c };
}

function isTriangle(values) {
  let sorted = values.sort((a, b) => a - b);
  return sorted[0] + sorted[1] > sorted[2];
}

function checkData(input) {
  let possible = 0;

  let linesInThrees = [];
  input.on("line", line => {
    let newLines = [];
    linesInThrees.push(getPoints(line));
    if (linesInThrees.length == 3) {
      newLines.push([
        linesInThrees[0].a,
        linesInThrees[1].a,
        linesInThrees[2].a
      ]);
      newLines.push([
        linesInThrees[0].b,
        linesInThrees[1].b,
        linesInThrees[2].b
      ]);
      newLines.push([
        linesInThrees[0].c,
        linesInThrees[1].c,
        linesInThrees[2].c
      ]);
      linesInThrees = [];
    }

    for (line of newLines) {
      if (isTriangle(line)) {
        possible++;
      }
    }
  });
  input.on("close", () => console.log(possible));
}

checkData(util.inputLines("3t1"));
checkData(util.inputLines("3t2"));
checkData(util.inputLines(3));
