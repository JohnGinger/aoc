let getWinnerPart1 = function(numberOfElves) {
  let elves = Array(numberOfElves)
    .fill(1)
    .map((_, i) => i + 1);

  let elfToStealFromPosition = 1;
  while (elves.length > 1) {
    elves.splice(elfToStealFromPosition, 1);
    elfToStealFromPosition = (elfToStealFromPosition + 1) % elves.length;
  }
  console.log(
    `With ${numberOfElves} elves, elf ${elves[0]} gets all the presents`
  );
};

let getWinnerPart1Fast = function(numberOfElves) {
  // Spotting the pattern
  let elvesBinary = Number(numberOfElves).toString(2);
  let winner = Number.parseInt(
    elvesBinary.slice(1) + elvesBinary.slice(0, 1),
    2
  );
  console.log(
    `With ${numberOfElves} elves, elf ${winner} gets all the presents`
  );
};

const input = 3014603;
getWinnerPart1(1);
getWinnerPart1(2);
getWinnerPart1(3);
getWinnerPart1(4);
getWinnerPart1(5);
getWinnerPart1(6);
getWinnerPart1(7);
getWinnerPart1(8);
getWinnerPart1(9);

getWinnerPart1Fast(input);

let getWinnerPart2 = function(numberOfElves) {
  let elves = Array(numberOfElves)
    .fill(1)
    .map((_, i) => i + 1);
  //console.log(`numberOfElves ${numberOfElves}`);
  let elfStealing = 0;
  let elfToStealFromPosition = Math.floor(elves.length / 2) % elves.length;
  while (elves.length > 1) {
    /* console.log(
      `${elves[elfStealing]} steals from  ${
        elves[elfToStealFromPosition]
      } - ${elves}`
    );
    */
    elves.splice(elfToStealFromPosition, 1);
    if (elfToStealFromPosition > elfStealing) {
      elfStealing = (elfStealing + 1) % elves.length;
    } else {
      elfStealing = elfStealing % elves.length;
    }

    elfToStealFromPosition =
      (elfStealing + Math.floor(elves.length / 2)) % elves.length;
  }

  return elves[0];
};
/*
for (let i = 1; i < 50; i++) {
  let winner = getWinnerPart2(i);
  console.log(`With ${i} elves, elf ${winner} gets all the presents`);
}

console.log("Resets");

for (let i = 1; i < 90; i++) {
  let winner = getWinnerPart2(i);
  if (winner === 1) {
    console.log(`Resets at ${i - 1}`);
  }
}

for (let i = 1; i < 50; i++) {
  let winner = getWinnerPart2(i);
  let threePower = (Math.log(i) / Math.log(3)).toFixed(4);
  let prediction = 1;
  if (threePower % 1 === 0) {
    prediction = 3 ** threePower;
  } else {
    prediction = i - 3 ** Math.floor(threePower);
  }
  console.log(
    `i ${i}, ${3 **
      Math.floor(threePower)} Actual ${winner}, Prediction ${prediction}`
  );
}
*/
function getWinnerPart2Fast(i) {
  let threePower = (Math.log(i) / Math.log(3)).toFixed(4);
  let prediction = 1;
  if (threePower % 1 === 0) {
    prediction = 3 ** threePower;
  } else {
    prediction = i - 3 ** Math.floor(threePower);
  }
  return prediction;
}

console.log(
  `Part 2 - With ${input} elves, elf ${getWinnerPart2Fast(
    input
  )} gets all the presents`
);
