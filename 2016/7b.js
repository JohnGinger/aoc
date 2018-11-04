const util = require("./util");

function supportsSSL(line) {
  let insideBrackets = false;
  let hasSSL = false;
  let BABs = new Set();
  let ABAs = new Set();
  for (let i = 0; i < line.length - 2; i++) {
    if (line[i] === "[") {
      insideBrackets = true;
    }
    if (line[i] === "]") {
      insideBrackets = false;
    }
    let string = line.slice(i, i + 3);

    if (!insideBrackets) {
      if (isABA(string)) {
        BABs.add(getBAB(string));
      }
      if (ABAs.has(string)) {
        hasSSL = true;
        break;
      }
    } else {
      if (isABA(string)) {
        ABAs.add(getBAB(string));
      }
      if (BABs.has(string)) {
        hasSSL = true;
        break;
      }
    }
  }
  return hasSSL ? 1 : 0;
}

function getBAB(chars) {
  return chars[1] + chars[0] + chars[1];
}

function isABA(chars) {
  return chars[0] === chars[2] && chars[0] !== chars[1];
}

function checkInput(input) {
  let lines = input.split("\n");
  let validNum = lines.reduce((p, c) => {
    return p + supportsSSL(c);
  }, 0);
  console.log(validNum);
}

checkInput(util.input(7));
