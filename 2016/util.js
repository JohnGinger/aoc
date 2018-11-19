const fs = require("fs");
const readline = require("readline");

const input = day => {
  return fs.readFileSync(`./data/${day}.txt`, { encoding: "utf8" });
};

const inputLines = day => {
  return readline.createInterface({
    input: fs.createReadStream(`./data/${day}.txt`),
    crlfDelay: Infinity
  });
};

const inputLinesArray = day => {
  return input(day).split("\n");
};

const test = (day, testNum) => {
  return fs.readFileSync(`./data/${day}t${testNum}.txt`, { encoding: "utf8" });
};

function* getAllPermutations(array) {
  if (array.length === 1) {
    yield array;
  }

  for (let i = 0; i < array.length; i++) {
    let elementsLeft = [...array.slice(0, i), ...array.slice(i + 1)];
    for (permutation of getAllPermutations(elementsLeft)) {
      yield [array[i], ...permutation];
    }
  }
}

module.exports = {
  input,
  test,
  inputLines,
  inputLinesArray,
  getAllPermutations
};
