const util = require("./util");

let input = util.inputLines("20");
let ips = [];
let ranges = [];
input.on("line", line => {
  let [s, e] = line.split("-");
  let start = Number(s);
  let end = Number(e);
  ips[start] = end;
  ranges.push({ start, end });
});

input.on("close", () => {
  ranges.sort((a, b) => a.start - b.start);
  let blockedEnd = ranges[0].end;
  let blockedStart = ranges[0].start;
  let blockedLength = 0;
  let freeCount = 0;
  for (range of ranges) {
    let { start, end } = range;
    let rangeOverlaps = start <= blockedEnd + 1;

    if (rangeOverlaps) {
      blockedEnd = Math.max(end, blockedEnd);
    } else {
      // Between one after the blockedEnd and this start must be free
      freeCount += start - (blockedEnd + 1);
      blockedStart = start;
      blockedEnd = end;
    }
  }
  freeCount += 4294967295 - blockedEnd;
  console.log("Part 2 is ", freeCount);
});
