const fs = require('mz/fs');
const path = require('path');
const http = require('http');
const url = require('url');
const { Readable } = require('stream');
const colors = require('colors/safe');

// Setup frames in memory
let frames;

(async () => {
  const framesPath = 'frames';
  const files = await fs.readdir(framesPath);

  frames = await Promise.all(files.map(async (file) => {
    const frame = await fs.readFile(path.join(framesPath, file));
    return frame.toString();
  }));
})().catch((err) => {
  console.log('Error loading frames');
  console.log(err);
});

const colorsOptions = [
  'red',
  'yellow',
  'green',
  'blue',
  'magenta',
  'cyan',
  'white'
];
const numColors = colorsOptions.length;
const selectColor = previousColor => {
  let color;

  do {
    color = Math.floor(Math.random() * numColors);
  } while (color === previousColor);

  return color;
};

const streamer = (stream) => {
  let index = 0;
  let lastColor;

  return setInterval(() => {
    // clear the screen
    stream.push('\033[2J\033[H');

    const newColor = lastColor = selectColor(lastColor);

    stream.push(colors[colorsOptions[newColor]](frames[index]));

    index = (index + 1) % frames.length;
  }, 100);
};

const server = http.createServer(async (req, res) => {
  if (
    req.headers &&
    req.headers['user-agent'] &&
    !req.headers['user-agent'].includes('curl')
  ) {
    res.write(await fs.readFile(path.resolve('./static/index.html')));
    return res.end();
  }
  const stream = new Readable();
  stream._read = function noop() {};
  stream.pipe(res);
  const interval = streamer(stream);

  req.on('close', () => {
    stream.destroy();
    clearInterval(interval);
  });
});

const port = process.env.COFFEE_PORT || 3000;
server.listen(port, err => {
  if (err) throw err;
  console.log(`Listening on localhost:${port}`);
});
