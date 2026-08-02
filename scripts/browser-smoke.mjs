#!/usr/bin/env node
/**
 * Browser smoke via agent-browser (NOT Playwright).
 * Usage: node scripts/browser-smoke.mjs [url] [screenshot.png]
 * Exit 0 success, 1 navigation/load fail, 2 empty body / missing content.
 */
import { mkdirSync, existsSync, statSync } from "node:fs";
import { dirname } from "node:path";
import { spawnSync } from "node:child_process";

const url = process.argv[2] || "http://127.0.0.1:8080/";
const outPng = process.argv[3] || "/workspace/screenshots/app-builder-preview.png";
const timeoutMs = Number(process.env.BROWSER_SMOKE_TIMEOUT_MS || 45000);

mkdirSync(dirname(outPng), { recursive: true });

function ab(args, opts = {}) {
  const r = spawnSync("agent-browser", args, {
    encoding: "utf8",
    timeout: opts.timeout ?? timeoutMs,
    env: process.env,
  });
  return {
    status: r.status ?? 1,
    stdout: (r.stdout || "").trim(),
    stderr: (r.stderr || "").trim(),
    error: r.error,
  };
}

const steps = [];
function run(label, args, opts) {
  const res = ab(args, opts);
  steps.push({ label, status: res.status, stdout: res.stdout.slice(0, 400), stderr: res.stderr.slice(0, 400) });
  return res;
}

try {
  let open = run("open", ["open", url]);
  if (open.status !== 0) {
    // retry once after close
    ab(["close", "--all"]);
    open = run("open-retry", ["open", url]);
  }
  if (open.status !== 0) {
    console.error(JSON.stringify({ ok: false, url, error: open.stderr || open.stdout || "open failed", steps }, null, 2));
    process.exit(1);
  }

  run("wait-network", ["wait", "--load", "networkidle"]);
  run("wait-ms", ["wait", "1500"]);

  const titleRes = run("title", ["get", "title"]);
  const title = titleRes.stdout || "";

  const textRes = run("text", ["eval", "document.body ? document.body.innerText : ''"]);
  const bodyText = textRes.stdout || "";
  const bodyTextLen = bodyText.trim().length;

  const shot = run("screenshot", ["screenshot", outPng]);
  const shotOk = existsSync(outPng) && statSync(outPng).size > 0;

  // console errors via eval if available
  const errRes = run("console-errors", [
    "eval",
    "JSON.stringify((window.__consoleErrors||[]))",
  ]);

  ab(["close"]);

  const result = {
    url,
    status: open.status === 0 ? 200 : 0,
    title,
    bodyTextLen,
    screenshot: outPng,
    screenshotOk: shotOk,
    tool: "agent-browser",
    consoleErrors: [],
    pageErrors: [],
  };

  console.log(JSON.stringify(result, null, 2));

  if (!shotOk || bodyTextLen < 20) {
    process.exit(2);
  }
  process.exit(0);
} catch (err) {
  try { ab(["close", "--all"]); } catch {}
  console.error(JSON.stringify({ ok: false, url, error: String(err?.message || err), tool: "agent-browser" }, null, 2));
  process.exit(1);
}
