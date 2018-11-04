const util = require("./util");
var GIFEncoder = require("gifencoder");
const { createCanvas, createImageData } = require("canvas");
var fs = require("fs");
const numRows = 6;
const numColumns = 50;

function doOperation(lights, line) {
  let commandType = line.slice(0, 4);
  if (commandType === "rect") {
    let rest = line.slice(5);
    let points = rest.trim().split("x");
    return rect(lights, points[0], points[1]);
  }
  let rowOrColumn = line.slice(7, 10);

  if (rowOrColumn === "row") {
    let amount = Number(line.slice(18)) % numColumns;
    let row = Number(line.slice(13, 14));
    lights = rotateRow(lights, row, amount);
  } else {
    let amount = Number(line.slice(21)) % numRows;
    let column = Number(line.slice(16, 18));
    lights = rotateColumn(lights, column, amount);
  }
  return lights;
}

function rotateColumn(lights, column, amount) {
  let numRows = lights.length;
  let newColumn = [];
  for (let row = 0; row < numRows; row++) {
    let positionToCopyFrom = (row + numRows - amount) % numRows;
    newColumn[row] = lights[positionToCopyFrom][column];
  }
  for (let row = 0; row < numRows; row++) {
    lights[row][column] = newColumn[row];
  }
  return lights;
}

function rotateRow(lights, start, num) {
  let line = lights[start];
  let leftOver = line.splice(-num, num);
  lights[start] = leftOver.concat(line);
  return lights;
}

function rect(lights, x, y) {
  let xVal = Number(x);
  let yVal = Number(y);
  for (let i = 0; i < yVal; i++) {
    lights[i] = [...Array(xVal).fill(true), ...lights[i].slice(xVal)];
  }
  return lights;
}

function showStatus(lights) {
  for (let i = 0; i < numRows; i++) {
    console.log(lights[i].map(x => (x ? "#" : ".")).join(""));
  }
  console.log("");
}
function get2dMatrix(numRows, numColumns, fill) {
  grid = [];
  for (let i = 0; i < numRows; i++) {
    grid[i] = Array(numColumns).fill(fill);
  }
  return grid;
}

let encoder = new GIFEncoder(numColumns * 10, numRows * 10);
// stream the results as they are available into myanimated.gif
encoder.createReadStream().pipe(fs.createWriteStream("myanimated.gif"));
encoder.start();
encoder.setRepeat(-1); // 0 for repeat, -1 for no-repeat
encoder.setDelay(50); // frame delay in ms
encoder.setQuality(10); // image quality. 10 is default.
var canvas = createCanvas(numColumns * 10, numRows * 10);
var ctx = canvas.getContext("2d");

function getVoltage(input) {
  let lights = get2dMatrix(numRows, numColumns, false);
  showStatus(lights);
  input.on("line", l => {
    lights = doOperation(lights, l);
    console.clear();
    showStatus(lights);
    encoder.addFrame(imageToCanvas(ctx, lights));
  });
  input.on("close", l => {
    let voltage = lights.reduce(
      (voltage, row) =>
        voltage + row.reduce((sum, col) => (col ? sum + 1 : sum), 0),
      0
    );
    console.log(`Total voltage is`, voltage);
    encoder.finish();
  });
}

getVoltage(util.inputLines("8"));

function imageToCanvas(ctx, frame) {
  //var id = createImageData(1, 1);
  //var d = id.data;
  let s = 10;
  for (let y = 0; y < numRows; y++) {
    for (let x = 0; x < numColumns; x++) {
      // d[0] = frame[y][x] ? 255 : 0;
      // d[1] = frame[y][x] ? 125 : 0;
      // d[2] = frame[y][x] ? 255 : 0;
      // d[3] = 255;
      // ctx.putImageData(id, x , y);

      if (frame[y][x]) {
        ctx.fillStyle = "green";
      } else {
        ctx.fillStyle = "black";
      }
      ctx.fillRect(x * s, y * s, s, s);
    }
  }

  return ctx;
}
