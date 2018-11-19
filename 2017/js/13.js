const { readFileSync } = require("fs");
const input = readFileSync("./input.txt", { encoding: "utf8" });
const parseRegex = /([0-9]+): ([0-9]+)/;
const rows = input.split("\n");

const firewall = rows.map(line => parseRegex.exec(line)).map(regex => ({
  level: Number(regex[1]),
  depth: Number(regex[2])
}));

const isCaught = (delay, fireWallDepth, layer) =>
  (layer + delay) % (2 * (fireWallDepth - 1)) === 0;

const caughtLevels = delay =>
  firewall.filter(fireWallAtLevel =>
    isCaught(delay, fireWallAtLevel.depth, fireWallAtLevel.level)
  );

const getSeverity = delay =>
  caughtLevels(delay).reduce(
    (severity, caughtLevel) =>
      (severity += caughtLevel.depth * caughtLevel.level),
    0
  );

console.log("Part 1 is ", getSeverity(0));

let delay = 0;
while (
  firewall.some(fireWallAtLevel =>
    isCaught(delay, fireWallAtLevel.depth, fireWallAtLevel.level)
  )
) {
  delay++;
}
console.log("Part 2 is ", delay);
