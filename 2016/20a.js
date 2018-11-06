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
  let blockedEnd = 0;
  for (range of ranges) {
    let { start, end } = range;
    console.log(blockedEnd, range);
    if (start <= blockedEnd + 1) {
      blockedEnd = Math.max(blockedEnd, end);
    } else if (start > blockedEnd) {
      console.log("Lowest safe ip is ", blockedEnd + 1);
      break;
    }
  }
});
