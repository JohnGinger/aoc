const crypto = require('crypto');

function getCode(input) {
    let i = 0;
    let code = '';
    while (code.length < 8){
        let hash = crypto.createHash('md5').update(input + i).digest('hex');
        if (hash.slice(0,5) === '00000'){
            code += hash.slice(5,6);
            console.log(hash, i, code);
        }
        i++;
    }
    console.log(code);
}

getCode('ojvtpuvg')
