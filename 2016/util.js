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

module.exports = {
  input,
  test,
  inputLines,
  inputLinesArray
};
