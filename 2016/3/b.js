function getPoints(line) {
    let a = +line.slice(2, 6).trim();
    let b = +line.slice(7, 11).trim();
    let c = +line.slice(12, 16).trim();
    return {a, b, c}
}

function isTriangle(values) {
    let sorted = values.sort((a, b) => a - b)
    return (sorted[0] + sorted[1]) > sorted[2];
}

function checkData(input) {
    let lines = input.split('\n').map(getPoints);
    let newLines = [];
    for (let i = 0; i < lines.length; i += 3) {
        newLines.push([lines[i].a, lines[i+1].a, lines[i+2].a]);
        newLines.push([lines[i].b, lines[i+1].b, lines[i+2].b]);
        newLines.push([lines[i].c, lines[i+1].c, lines[i+2].c]);
    }

    let possible = 0;
    for (line of newLines) {
        if (isTriangle(line)) {
            possible++;
        }
    }
    console.log(possible);
}

checkData(require('./test1'))
checkData(require('./test2'))
checkData(require('./data'))