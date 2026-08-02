#!/usr/bin/env node
/**
 * Capture a preview thumbnail with agent-browser (not Playwright).
 * Usage: node scripts/preview-thumbnail.mjs [url] [out.png]
 */
import { mkdirSync } from "node:fs";
import { dirname } from "node:path";
import { spawnSync } from "node:child_process";

const url = process.argv[2] || "http://127.0.0.1:8080/";
const outPng = process.argv[3] || "/workspace/screenshots/preview-thumbnail.png";
mkdirSync(dirname(outPng), { recursive: true });

function ab(args) {
  return spawnSync("agent-browser", args, { encoding: "utf8", timeout: 60000, env: process.env });
}

const open = ab(["open", url]);
if (open.status !== 0) {
  console.error(open.stderr || open.stdout || "open failed");
  process.exit(1);
}
ab(["wait", "--load", "networkidle"]);
ab(["wait", "1000"]);
const shot = ab(["screenshot", outPng]);
ab(["close"]);
if (shot.status !== 0) {
  console.error(shot.stderr || "screenshot failed");
  process.exit(1);
}
console.log(JSON.stringify({ url, screenshot: outPng, tool: "agent-browser" }));
