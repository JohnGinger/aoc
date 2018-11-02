let getMessage = function (input) {
    let lines = input.split('\n').map(x => x.trim());
    let mostFrequent = [{},{},{},{},{},{},{},{}];
    lines.forEach(line => {
        let letters = line.split("");
        letters.forEach((l, i) => {
            let letterObj = mostFrequent[i];
            letterObj[l] ?
                letterObj[l]++ :
                letterObj[l] = 1;
        })
    });
    let answer = mostFrequent
        .map(getMostUsedLetter)
        .join("")
    console.log(answer);
}

function getMostUsedLetter(letters) {
    return Object.keys(letters)
        .sort((a, b) => {
            if (letters[b] === letters[a]) {
                if (a > b) {
                    return -1;
                } else if (a < b) {
                    return 1;
                } else {
                    return 0;
                }
            } else {
                return letters[a] - letters[b]
            }
        }).slice(0,1)[0];
}

getMessage(require('./data'))