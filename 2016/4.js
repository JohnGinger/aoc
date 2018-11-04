const util = require("./util");

function getSectorId(line) {
  let checksum = line.slice(-6, -1);
  let sectorId = +line.match(/[0-9]+/)[0];
  let letters = {};
  let shift = sectorId % 26;
  let decoded = "";
  for (let i = 0; i < line.length - 8; i++) {
    if (line[i].match(/[0-9]/)) {
      break;
    }
    if (line[i] !== "-") {
      letters[line[i]] ? letters[line[i]]++ : (letters[line[i]] = 1);

      let charcode = line[i].charCodeAt();
      let letter = charcode - 97;
      let newLetter = ((letter + shift) % 26) + 97;
      decoded += String.fromCharCode(newLetter);
    } else {
      decoded += " ";
    }
  }
  let calulatedChecksum = Object.keys(letters)
    .sort((a, b) => {
      if (letters[b] === letters[a]) {
        if (a < b) {
          return -1;
        } else if (a > b) {
          return 1;
        } else {
          return 0;
        }
      } else {
        return letters[b] - letters[a];
      }
    })
    .slice(0, 5)
    .join("");
  if (calulatedChecksum === checksum) {
    return { valid: true, sectorId, northPole: decoded.search("north") !== -1 };
  } else {
    return { valid: false };
  }
}

function checkData(input) {
  let sum = 0;

  input.on("line", line => {
    let { valid, sectorId, northPole } = getSectorId(line);
    if (valid) {
      if (northPole) {
        console.log(`Part 2: `, sectorId);
      }
      sum += sectorId;
    }
  });

  input.on("close", () => console.log(`Part 1: ${sum}`));
}

//console.log("Tests");
//checkData(util.inputLines("4t1"));
//checkData(util.inputLines("4t2"));

checkData(util.inputLines("4"));
