function getPoints(line){
    let a = +line.slice(2,6).trim();
    let b = +line.slice(7,11).trim();
    let c = +line.slice(12,16).trim();
    return [a,b,c]
}

function isTriangle(values){
    let sorted = values.sort((a,b) => a - b)
    return (sorted[0] + sorted[1]) > sorted[2];
}

function checkData(input){
    let lines = input.split('\n');
    let possible = 0;
    for (line of lines){
        let [a, b, c] = getPoints(line);
        if (isTriangle([a, b, c])){
            possible++;
        }
    }
    console.log(possible);
}

checkData(require('./test1'))
checkData(require('./data'))