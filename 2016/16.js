const util = require("./util");

function dragonIt(input) {
  let a = input;
  let b = JSON.parse(JSON.stringify(a));
  b = b.split("").reverse();
  b = b.map(c => (c === "1" ? "0" : "1")).join("");
  return a + "0" + b;
}

function checksum(input) {
  let newSlice = "";
  for (let i = 0; i < input.length - 1; i += 2) {
    let slice = input.slice(i, i + 2);
    if (slice === "11" || slice === "00") {
      newSlice += "1";
    } else {
      newSlice += "0";
    }
  }
  if (newSlice.length % 2 === 0) {
    return checksum(newSlice);
  } else {
    return newSlice;
  }
}
/*
Tests
console.log(dragonIt("1"));
console.log(dragonIt("0"));
console.log(dragonIt("11111"));
console.log(dragonIt("111100001010"));
console.log("checksum is ", checksum("110010110100"));
*/

function generateAndCheck(input, length) {
  while (true) {
    if (input.length >= length) {
      return checksum(input.slice(0, length));
      break;
    }
    input = dragonIt(input);
  }
}

let puzzleInput = "10111100110001111";

console.log("Part 1 is ", generateAndCheck(puzzleInput, 272));
console.log("Part 2 is ", generateAndCheck(puzzleInput, 35651584));
