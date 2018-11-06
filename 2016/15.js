const util = require("./util");

function parseInstructions(input) {
  var re = /Disc #([0-9]+) has ([0-9]+) positions; at time=0, it is at position ([0-9]+)./;
  let disks = [];
  let i = 0;
  for (line of input) {
    [_, _, positions, startingPosition] = re.exec(line);
    disks.push({
      positions: Number(positions),
      position: (Number(startingPosition) + i) % Number(positions)
    });
    i += 1;
  }
  return disks;
}

function rotate(disks) {
  disks.forEach(disk => {
    disk.position = (disk.position + 1) % disk.positions;
  });
  return disks;
}

function match(disks) {
  return disks.every(disk => disk.position === 0);
}

let disks = parseInstructions(util.inputLinesArray(15));
for (let i = 0; i < 1000000; i++) {
  disks = rotate(disks);
  if (match(disks)) {
    console.log("Part 1 is", i);
    break;
  }
}

let disks2 = parseInstructions(util.inputLinesArray(15));
disks2.splice(0, 0, { positions: 11, position: disks2.length });
for (let i = 0; i < 100000000; i++) {
  disks2 = rotate(disks2);
  if (match(disks2)) {
    console.log("Part 2 is", i);
    break;
  }
}
