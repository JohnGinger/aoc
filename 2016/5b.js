const crypto = require('crypto');

function getCode(input) {
    let i = 0;
    let code = Array(8);
    let lettersGot = 0;
    while (lettersGot < 8) {
        let hash = crypto.createHash('md5').update(input + i).digest('hex');
        if (hash.slice(0, 5) === '00000') {
            let position = hash.slice(5, 6);
            if (position.match(/[0-7]/)) {
                let letter = hash.slice(6, 7);
                if (!code[position]) {
                    lettersGot++;
                    code[position] = letter;
                }
                console.log(hash, i, code);
            }
        }
        i++;
    }
    console.log(code.join(""));
}

getCode('ojvtpuvg')