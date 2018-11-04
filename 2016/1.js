"use strict";
const util = require("./util");
const debug = console.log; //() => {};

let positions = ["up", "left", "down", "right"];

let getDistance = function(data, gridX = 0, gridY = 0, part2 = false) {
  let instructions = data.split(",").map(x => x.trim());
  let placesVisited = [];
  let position = 0;

  for (let instruction of instructions) {
    let direction = instruction[0];
    let magnitude = +instruction.slice(1);
    switch (direction) {
      case "L":
        position++;
        break;

      case "R":
        position--;
        break;
    }

    position = (position + 4) % 4;

    let pointing = positions[position];

    if (part2) {
      let xTransit = gridX;
      let yTransit = gridY;
      for (let i = 0; i < magnitude; i++) {
        if (pointing === "up") {
          yTransit += 1;
        } else if (pointing === "down") {
          yTransit -= 1;
        } else if (pointing === "left") {
          xTransit -= 1;
        } else if (pointing === "right") {
          xTransit += 1;
        }
        if (placesVisited.some(p => p.x === xTransit && p.y === yTransit)) {
          return {
            gridX,
            gridY,
            distance: Math.abs(xTransit) + Math.abs(yTransit)
          };
        }
        placesVisited.push({ x: xTransit, y: yTransit });
      }
    }

    if (pointing === "up") {
      gridY += magnitude;
    }

    if (pointing === "down") {
      gridY -= magnitude;
    }

    if (pointing === "left") {
      gridX -= magnitude;
    }

    if (pointing === "right") {
      gridX += magnitude;
    }
  }
  return { gridX, gridY, distance: Math.abs(gridX) + Math.abs(gridY) };
};

debug(getDistance(util.test(1, 1)));
debug(getDistance(util.test(1, 2)));
debug(getDistance(util.test(1, 3)));

const part1 = getDistance(util.input(1));
console.log(`Part 1 : ${part1.distance}`);

const part2 = getDistance(util.input(1), part1.x, part1.y, true);
console.log(`Part 2 : ${part2.distance}`);
