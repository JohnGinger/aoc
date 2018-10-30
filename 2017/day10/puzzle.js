const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
let inputLengths = Array.from(input.split(","), Number);

const hash = ({ inputLengths, position = 0, skip = 0 }) => {
  let indexes = Array(256)
    .fill(0)
    .map((_, i) => i);
  for (length of inputLengths) {
    let arrayToReverse = indexes.slice(position, position + length);
    let before = indexes.slice(0, position);

    if (position + length > indexes.length) {
      let overflow = (position + length) % indexes.length;
      arrayToReverse = [...arrayToReverse, ...indexes.slice(0, overflow)];
      arrayToReverse.reverse();
      before = [
        ...arrayToReverse.slice(length - overflow),
        ...indexes.slice(overflow, position)
      ];
      arrayToReverse = arrayToReverse.slice(0, length - overflow);
    } else {
      arrayToReverse.reverse();
    }

    indexes = [
      ...before,
      ...arrayToReverse.slice(0, length),
      ...indexes.slice(length + position)
    ];
    position = (position + length + skip) % indexes.length;
    skip += 1;
  }

  return { indexes, skip, position };
};

let { indexes } = hash({ inputLengths });
console.log("Part 1 is ", indexes[0] * indexes[1]);

let inputChars = Array.from(input.split(""), v => v.charCodeAt(0));
const suffix = [17, 31, 73, 47, 23];
const lengths = [...suffix];

let hashRounds = 63;
let globalIndexes = [];
let skip = 0;
let position = 0;
for (let i = 0; i < hashRounds; i++) {
  let result = hash({
    inputLengths: lengths,
    skip,
    position
  });
  skip = result.skip;
  position = result.position;
  globalIndexes = result.indexes;
}

const xor16 = input => input.slice(1).reduce((p, c) => p ^ c, input[0]);

let result = [];
for (let i = 0; i < 16; i++) {
  result.push(xor16(globalIndexes.slice(i * 16, (i + 1) * 16)));
}
console.log(result.map(x => x.toString(16).padStart(2, "0")).join(""));
