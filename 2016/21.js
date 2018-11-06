const util = require("./util");

let puzzleInputLines = util.inputLinesArray("21");

function scramble(lines, input) {
  let instructions = [];
  input = input.split("");
  for (line of lines) {
    let instruction = () => {};
    let [command, ...remaining] = line.split(" ");
    if (command === "swap") {
      if (remaining[0] === "position") {
        let position1 = Number(remaining[1]);
        let position2 = Number(remaining[4]);

        let newTwo = input[position2];
        let newOne = input[position1];

        input[position2] = newOne;
        input[position1] = newTwo;
      } else if (remaining[0] === "letter") {
        let letter1 = remaining[1];
        let letter2 = remaining[4];

        let position1 = input.indexOf(letter2);
        let position2 = input.indexOf(letter1);
        input[position1] = letter1;
        input[position2] = letter2;
      } else {
        console.warn("Confusing instruction", line);
      }
    } else if (command === "reverse") {
      let from = Number(remaining[1]);
      let to = Number(remaining[3]);
      let sectionToReverse = input.slice(from, to + 1);
      let sectionReversed = sectionToReverse.reverse();
      input = [
        ...input.slice(0, from),
        ...sectionReversed,
        ...input.slice(to + 1)
      ];
    } else if (command === "rotate") {
      let amount = Number(remaining[1]);
      if (remaining[0] === "left") {
        amount = amount % input.length;
        input = [...input.slice(amount), ...input.slice(0, amount)];
      } else if (remaining[0] === "right") {
        amount = amount % input.length;
        input = [...input.slice(-amount), ...input.slice(0, -amount)];
      } else if (remaining[0] === "based") {
        let letter = remaining[5];
        let position = input.indexOf(letter);
        if (position >= 4) {
          position += 1;
        }
        let amount = (1 + position) % input.length;
        input = [...input.slice(-amount), ...input.slice(0, -amount)];
      } else {
        console.warn("confusing", line);
      }
    } else if (command === "move") {
      let from = remaining[1];
      let to = remaining[4];
      let removed = input.splice(from, 1)[0];
      input.splice(to, 0, removed);
    } else {
      console.warn("confusing command ", command);
    }
  }
  return input.join("");
}

function* getAllPermutations(string) {
  if (string.length === 1) {
    yield string;
  }

  for (let i = 0; i < string.length; i++) {
    let charsLeft = string.substring(0, i) + string.substring(i + 1);
    for (permutation of getAllPermutations(charsLeft)) {
      yield string[i] + permutation;
    }
  }
}

let input = "abcdefgh";
console.log("Part 1 is ", scramble(puzzleInputLines, input));

for (permutation of getAllPermutations("abcdefgh")) {
  if (scramble(puzzleInputLines, permutation) === "fbgdceah") {
    console.log("Part 2 is ", permutation);
    break;
  }
}

/*
let input = "abcdefgh";
for (instruction of instructions) {
  console.log(input);
  console.log(instruction.toString());
  input = instruction(input);
}
console.log(input);
*/
