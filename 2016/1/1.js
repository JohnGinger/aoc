"use strict";

let positions = ['up', 'left', 'down', 'right'];

let getDistance = function (data) {
    let instructions = data.split(",").map(x => x.trim());
    let placesVisted = [];
    let gridx = 0;
    let gridy = 0;
    let position = 0;

    for (let instruction of instructions) {
        let direction = instruction[0];
        let magnitude = +instruction.slice(1);
        switch (direction) {
            case 'L':
                position++;
                break;

            case 'R':
                position--;
                break;
        }

        position = (position + 4) % 4;

        let pointing = positions[position];

        let updateGridXValues = function (direction) {
            for (let i = 0; i < magnitude; i++) {
                gridx += direction;
                if (placesVisted.some(x => x.gridx  === gridx && x.gridy === gridy)) {
                    console.log(`When to ${gridx} : ${gridy} twice`)
                    break;
                }
                placesVisted.push({gridx, gridy})
            }
        }

        let updateGridYValues = function (direction) {
            for (let i = 0; i < magnitude; i++) {
                gridy += direction;
                if (placesVisted.some(x => x.gridx  === gridx && x.gridy === gridy)) {
                    console.log(`When to ${gridx} : ${gridy} twice`)
                    break;
                }
                placesVisted.push({gridx, gridy})
            }
        }

        if (pointing === 'up') {
            updateGridYValues(1)
        }

        if (pointing === 'down') {
            updateGridYValues(-1)
        }

        if (pointing === 'left') {
            updateGridXValues(1)
        }

        if (pointing === 'right') {
            updateGridXValues(-1)
        }
    };
    return Math.abs(gridx) + Math.abs(gridy);

}
console.log(getDistance(require("./test1-1")));
console.log(getDistance(require("./test1-2")));
console.log(getDistance(require("./test1-3")));
console.log(getDistance(require("./test1-4")));
console.log(getDistance(require("./data1")));