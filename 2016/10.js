const util = require("./util");
const _ = require("underscore");

function getBots(instructions) {
  return _.chain(instructions)
    .map(x => x.match(/[0-9]+/g))
    .groupBy(x => x[1])
    .map((x, i) => {
      return {
        bot: i,
        boxes: x.map(x => x[0])
      };
    })
    .value();
}

function getMoves(instructions) {
  return _.chain(instructions)
    .map(x => {
      let nums = x.match(/[0-9]+/g);
      return {
        bot: nums[0],
        lowOutput: !!x.match(/low to output/g),
        highOutput: !!x.match(/high to output/g),
        low: nums[1],
        high: nums[2]
      };
    })
    .groupBy(x => x.bot)
    .value();
}

function process(instructions) {
  let bots = getBots(instructions.split("\n").filter(x => x[0] === "v"));
  //console.log(bots)

  let moves = getMoves(instructions.split("\n").filter(x => x[0] !== "v"));
  // console.log(moves)

  let currentBots = bots.filter(x => x.boxes.length === 2);

  let i = 0;
  console.log(i, currentBots);

  let outputs = [];
  while (!weHaveTheOnesWeWant(currentBots)) {
    currentBots.forEach(x => {
      let move = moves[x.bot][0];
      let lowBox = x.boxes.sort((a, b) => +a - +b)[0];
      let highBox = x.boxes.sort((a, b) => +b - +a)[0];
      let bot;

      ({ bots, bot } = getBot(bots, move.low));

      if (!move.lowOutput) {
        bot.boxes.push(lowBox);
      } else {
        outputs[bot.bot] = outputs[bot.bot]
          ? outputs[bot.bot].push(lowBox)
          : [lowBox];
      }

      ({ bots, bot } = getBot(bots, move.high));

      if (!move.highOutput) {
        bot.boxes.push(highBox);
      } else {
        outputs[bot.bot] = outputs[bot.bot]
          ? outputs[bot.bot].push(highBox)
          : [highBox];
      }
      x.boxes = [];
    });
    currentBots = bots.filter(x => x.boxes.length === 2);
    i++; //console.log(currentBots)

    /* if (currentBots.some(x => {
                let pair = x.boxes
                return (
                    pair[0] == '61' ||
                    pair[1] == '61')
            })) { */
    console.log(i, currentBots);

    //}
  }

  console.log(i, currentBots);
  console.log(outputs);
}

function getBot(bots, id) {
  let bot = bots.find(x => x.bot === id);

  if (!bot) {
    bots.push({
      bot: id,
      boxes: []
    });
    bot = bots.find(x => x.bot === id);
  }
  return {
    bots,
    bot
  };
}

function weHaveTheOnesWeWant(bots) {
  /*bots.map(x => x.boxes)
        .some(pair => {
            return (pair[0] == '17' ||
                pair[1] == '17') && (pair[0] == '61' ||
                pair[1] == '61')
        }) */
  return bots.length == 0;
}

process(util.input(10));
