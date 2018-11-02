function getSectorIdOrZeroIfInvalid(line) {
    let checksum = line.slice(-6, -1);
    let sectorId = +line.match(/[0-9]+/)[0];
    let letters = {};
    let shift = sectorId % 26;
    let decoded = '';
    for (let i = 0; i < line.length - 8; i++) {
        if (line[i].match(/[0-9]/)) {
            break;
        }
        if (line[i] !== '-') {
            letters[line[i]] ?
                letters[line[i]]++ :
                letters[line[i]] = 1;

            let charcode = (line[i].charCodeAt());
            let letter = charcode - 97;
            let newLetter = ((letter + shift) % 26) + 97;
            decoded += String.fromCharCode(newLetter);
        } else {
            decoded += ' ';
        }
    }
    let calulatedChecksum = Object.keys(letters)
        .sort((a, b) => {
            if (letters[b] === letters[a]) {
                if (a < b) {
                    return -1;
                } else if (a > b) {
                    return 1;
                } else {
                    return 0;
                }
            } else {
                return letters[b] - letters[a]
            }
        })
        .slice(0, 5)
        .join("")
        //console.log(decoded)
    if (calulatedChecksum === checksum) {
        if (decoded.search("north") !== -1) {
            console.log(sectorId)
        }
        return sectorId;
    } else {
        return 0;
    }

}

function checkData(input) {
    let lines = input.split('\n');
    let sum = 0;
    for (line of lines) {
        sum += getSectorIdOrZeroIfInvalid(line)
    }
    //console.log(sum);
}

//checkData(require('./test2'))
checkData(require('./data'))