const util = require("./util");

function getMostUsedLetter(letters) {
  return Object.keys(letters)
    .sort((a, b) => {
      if (letters[b] === letters[a]) {
        if (a > b) {
          return -1;
        } else if (a < b) {
          return 1;
        } else {
          return 0;
        }
      } else {
        return letters[b] - letters[a];
      }
    })
    .slice(0, 1)[0];
}

function getLeastUsedLetter(letters) {
  return Object.keys(letters)
    .sort((a, b) => {
      if (letters[b] === letters[a]) {
        if (a > b) {
          return -1;
        } else if (a < b) {
          return 1;
        } else {
          return 0;
        }
      } else {
        return letters[a] - letters[b];
      }
    })
    .slice(0, 1)[0];
}

let getMessage = function(input) {
  let mostFrequent = [{}, {}, {}, {}, {}, {}, {}, {}];

  input.on("line", line => {
    let letters = line.trim().split("");
    letters.forEach((l, i) => {
      let letterObj = mostFrequent[i];
      letterObj[l] ? letterObj[l]++ : (letterObj[l] = 1);
    });
  });
  input.on("close", () => {
    console.log(`Part1 is`, mostFrequent.map(getMostUsedLetter).join(""));
    console.log(`Part2 is`, mostFrequent.map(getLeastUsedLetter).join(""));
  });
};

getMessage(util.inputLines(6));
