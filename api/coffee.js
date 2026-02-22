import { readFileSync, readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

const frames = readdirSync(join(__dirname, "frames"))
  .sort()
  .map((f) => readFileSync(join(__dirname, "frames", f)));

const ANSI_CLEAR = "\x1b[H\x1b[2J";
const ANSI_RESET = "\x1b[0m";
const ANSI_COLORS = [
  "\x1b[1;31m",
  "\x1b[1;32m",
  "\x1b[1;33m",
  "\x1b[1;34m",
  "\x1b[1;35m",
  "\x1b[1;36m",
];

const FRAME_MS = 100;
const DURATION_MS = 10_000;

const encoder = new TextEncoder();

export default {
  fetch() {
    let idx = 0;
    let interval;

    const stream = new ReadableStream({
      start(controller) {
        interval = setInterval(() => {
          const frame = `${ANSI_CLEAR}${ANSI_COLORS[idx % ANSI_COLORS.length]}${frames[idx % frames.length]}${ANSI_RESET}\r\n`;
          controller.enqueue(encoder.encode(frame));
          idx++;
        }, FRAME_MS);

        setTimeout(() => {
          clearInterval(interval);
          controller.enqueue(encoder.encode("no more coffee :(\n"));
          controller.close();
        }, DURATION_MS);
      },
      cancel() {
        clearInterval(interval);
      },
    });

    return new Response(stream, {
      headers: { "Content-Type": "text/event-stream" },
    });
  },
};
