const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
const parseRegex = /([0-9]+) <-> ([0-9, ]+)/;
let rows = input.split("\n");
let commands = rows.map(line => parseRegex.exec(line)).map(regex => ({
  id: Number(regex[1]),
  follow: regex[2].split(",").map(Number)
}));

let groups = {};
for (let command of commands) {
  groups[command.id] = new Set();
  command.follow.forEach(id => groups[command.id].add(id));
}

let reducedGroups = {};
let lastGroups = 0;
let timesThrough = 0;
while (true) {
  reducedGroups = Object.entries(groups).reduce((p, c) => {
    let existing = false;

    for (let group of Object.values(p)) {
      if (group.has(Number(c[0]))) {
        c[1].forEach(id => group.add(id));
        existing = true;
        break;
      }
    }
    if (!existing) {
      p[c[0]] = c[1];
    }
    return p;
  }, {});
  if (Object.values(reducedGroups).length == lastGroups) {
    break;
    timesThrough += 1;
  } else {
    lastGroups = Object.values(reducedGroups).length;
  }

  if (timesThrough == 5) {
    break;
  }
}

console.log("Part 1 is ", reducedGroups[0].size);
console.log("Part 2 is ", Object.values(reducedGroups).length);
