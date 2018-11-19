const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
const parseRegex = /([0-9]+) <-> ([0-9, ]+)/;
let rows = input.split("\n");
let commands = rows.map(line => parseRegex.exec(line)).map(regex => ({
  id: Number(regex[1]),
  follow: regex[2].split(",").map(Number)
}));

let reducedGroups = [];
for (let command of commands) {
  let set = new Set();
  command.follow.forEach(id => set.add(id));
  reducedGroups.push({ id: command.id, set });
}

let lastGroups = 0;
//while (true) {
reducedGroups = reducedGroups.reduce((groups, group) => {
  console.log("Before", groups.map(x => x.set.size).slice(0, 10));

  let idsToRemove = groups.filter(x => group.set.has(x.id)).map(x => x.id);
  console.log("To Remove", idsToRemove.length);

  let groupsLinkedToThisSet = groups
    .filter(x => group.set.has(x.id))
    .reduce(
      (p, c) => [...p, ...Array.from(c.set).filter(x => x !== group.id)],
      []
    );
  console.log("Linked", groupsLinkedToThisSet.length);

  groupsLinkedToThisSet.forEach(id => group.set.add(id));
  groups = groups.filter(x => !idsToRemove.some(toRemove => x.id === toRemove));
  console.log("After", groups.length);
  return groups;
}, reducedGroups);
console.log(reducedGroups.length);
/*
  if (reducedGroups.length == lastGroups) {
    break;
  } else {
    lastGroups = reducedGroups.length;
  }
}*/
/*
console.log("Part 1 is ", reducedGroups[0].set.size);
console.log("Part 2 is ", reducedGroups.length);*/
