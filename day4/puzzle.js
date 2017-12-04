const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
const lines = input.split("\n");

let validPassPhrasesPart1 = 0;
let validPassPhrasesPart2 = 0;

for (line of lines) {
  let validPart1 = true;
  let validPart2 = true;
  const wordsSet = new Set();
  const sortedWordsSet = new Set();
  const words = line.split(" ");
  for (word of words) {
    if (!validPart1 && !validPart2) {
      continue;
    }
    const sortedWord = word
      .split("")
      .sort()
      .join("");
    if (sortedWordsSet.has(sortedWord)) {
      validPart2 = false;
    }
    sortedWordsSet.add(sortedWord);

    if (wordsSet.has(word)) {
      validPart1 = false;
    }
    wordsSet.add(word);
  }
  if (validPart1) {
    validPassPhrasesPart1 += 1;
  }
  if (validPart2) {
    validPassPhrasesPart2 += 1;
  }
}
console.log("Part 1 is ", validPassPhrasesPart1);
console.log("Part 2 is ", validPassPhrasesPart2);
