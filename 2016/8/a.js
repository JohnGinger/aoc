let lights = [];

function doOperation(line) {
    let commandType = line.slice(0, 4);
    if (commandType === "rect") {
        let rest = line.slice(5);
        let points = rest.trim().split("x");
        rect(points[0], points[1]);
        return;
    }
    let rowOrColumn = line.slice(7, 10);
    if (rowOrColumn === "row") {
        rotateRow(+line.slice(13, 14), +line.slice(18))
    } else {
        rotateColumn(+line.slice(16, 18),
            line.slice(21))
    }
}

function rotateColumn(start, num) { 
    let array = [];

    for(let i =0; i < lights.length; i++){
        array.push(lights[i][start]);
    }

    let leftOver = array.splice(-num, +num);
    array = leftOver.concat(array);

    for(let i =0; i < lights.length; i++){
        lights[i][start] = array[i];
    }
}

function rotateRow(start, num) {
    let line = lights[start];
    let leftOver = line.splice(-num,num);
    lights[start] = leftOver.concat(line);
}

function getFilledArray(num, val) {
    let array = [];
    for(let i =0 ; i < num; i++){
        array.push(JSON.parse(JSON.stringify(val)))
    }
    return array;
}

function rect(x, y) {
    let xVal = +x;
    let yVal = +y;
    for (let i = 0; i < +yVal; i++) {
        lights[i].splice(0, xVal);
        lights[i] = getFilledArray(xVal, 1).concat(lights[i])
    }
}

function showStatus() {
    console.log(lights[0].join(""));
    console.log(lights[1].join(""));
    console.log(lights[2].join(""));
    console.log(lights[3].join(""));
    console.log(lights[4].join(""));
    console.log(lights[5].join(""));
    console.log("");
}

function getVoltage(input) {
    lights = getFilledArray(6, getFilledArray(50, 0));
    let lines = input.split('\n');
    lines.forEach(l => {
        console.log(l);
        doOperation(l);
        showStatus();
    })

    let voltage = lights.reduce((p, c) => {
        return p + +c.reduce((p, c) => {
            return p + +c;
        }, 0)
    }, 0)
    console.log(voltage);
}

getVoltage(require('./data'))