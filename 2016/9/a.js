function getLengthOfLine(line) {
    let insideBrackets = false;
    let count = 0;
    let repeatCode = '';

    for (let i = 0; i < line.length; i++) {
        if (insideBrackets) {
            repeatCode += line[i];
            if (line[i] === ')') {
                insideBrackets = false;
                let [skip, repeat] = repeatCode.slice(0, -1).split('x');
                i = i + +skip;
                count += +skip * +repeat;
                repeatCode = '';
            }
        } else {
            if (line[i] === '(') {
                insideBrackets = true;
            } else if (line[i] !== ' ') {
                count++;
            }
        }
    }
    console.log(line, count)
    return count;
}

function getSize(input) {
    console.log(input.split('\n')
        .map(getLengthOfLine)
        .reduce((p, c) => p + c, 0))
}

getSize(require('./data'));