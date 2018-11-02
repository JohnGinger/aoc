function supportsTLS(line) {
    let insideBrackets = false;
    let hasABBA = false;
    for (let i = 0; i < line.length - 3; i++) {
        if (line[i] === "[") {
            insideBrackets = true;
        }
        if (line[i] === "]") {
            insideBrackets = false;
        }
        if (isABBA(line.slice(i, i + 4))) {
            if (insideBrackets) {
                hasABBA = false;
                break;
            }
            hasABBA = true;
        }
    }
    return hasABBA ? 1 : 0;
}


function isABBA(chars) {
    if (chars[0] === chars[1] &&
        chars[0] === chars[2] &&
        chars[0] === chars[3]) {
        return false;
    }
    if (chars[0] === chars[3] &&
        chars[1] === chars[2]) {
        return true;
    }
    return false;
}

function checkInput(input) {
    let lines = input.split('\n');
    let validNum = lines.reduce((p, c) => {
        return p + supportsTLS(c)
    }, 0);
    console.log(validNum);
}

checkInput(require('./data'));