const crypto = require("crypto");

function getTriplesAndFives(string) {
  let triples = new Set();
  let fives = new Set();
  for (let i = 2; i < string.length; i++) {
    if (string[i] == string[i - 1] && string[i] == string[i - 2]) {
      if (triples.size === 0) {
        triples.add(string[i].toString());
      }

      if (i >= 4 && string[i] == string[i - 3] && string[i] == string[i - 4]) {
        fives.add(string[i].toString());
      }
    }
  }
  return {
    triples,
    fives
  };
}

const input = "ahsbgdzn";
let i = 0;
let searchFor = new Map();
let keys = [];

while (keys.length < 64) {
  let hash = crypto
    .createHash("md5")
    .update(input + i)
    .digest("hex");
  let { triples, fives } = getTriplesAndFives(hash);

  searchFor.forEach((v, letter) => {
    let value = v[0];
    while (v.length > 0 && i >= value.stop) {
      v.shift();
      searchFor.set(letter, v);
      value = v[0];
    }

    if (v.length == 0) {
      searchFor.delete(letter);
    } else if (fives.has(letter)) {
      v.forEach(value => {
        keys.push({
          letter,
          value
        });
        if (keys.length === 64) {
          console.log(value.start);
        }
      });
      searchFor.delete(letter);
    }
  });

  triples.forEach(letter => {
    if (searchFor.has(letter)) {
      let searches = searchFor.get(letter);
      searches.push({
        stop: i + 1001,
        start: i
      });
      searchFor.set(letter, searches);
    } else {
      searchFor.set(letter, [
        {
          stop: i + 1001,
          start: i
        }
      ]);
    }
  });

  i++;
}
