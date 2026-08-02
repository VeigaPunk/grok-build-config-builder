#!/usr/bin/env node
/**
 * Browser smoke via agent-browser (NOT Playwright).
 * Usage: node scripts/browser-smoke.mjs [url] [screenshot.png]
 * Exit 0 success, 1 navigation/load fail, 2 empty body / missing content.
 */
import { mkdirSync, existsSync, statSync } from "node:fs";
import { dirname } from "node:path";
import { spawnSync } from "node:child_process";
import { randomBytes } from "node:crypto";

const url = process.argv[2] || "http://127.0.0.1:8080/";
const outPng = process.argv[3] || "/workspace/screenshots/app-builder-preview.png";
const timeoutMs = Number(process.env.BROWSER_SMOKE_TIMEOUT_MS || 90000);
const session = process.env.SMOKE_SESSION || `smoke-${randomBytes(4).toString("hex")}`;

mkdirSync(dirname(outPng), { recursive: true });

function ab(args, opts = {}) {
  const full = ["--session", session, ...args];
  const r = spawnSync("agent-browser", full, {
    encoding: "utf8",
    timeout: opts.timeout ?? timeoutMs,
    env: process.env,
    maxBuffer: 8 * 1024 * 1024,
  });
  return {
    status: r.status ?? 1,
    stdout: (r.stdout || "").trim(),
    stderr: (r.stderr || "").trim(),
    error: r.error,
  };
}

function run(label, args, opts) {
  const res = ab(args, opts);
  if (process.env.SMOKE_DEBUG) {
    console.error(
      JSON.stringify({
        label,
        status: res.status,
        out: res.stdout.slice(0, 240),
        err: res.stderr.slice(0, 240),
      }),
    );
  }
  return res;
}

function parseMaybeJsonString(s) {
  const t = String(s || "").trim();
  if (!t) return "";
  if ((t.startsWith('"') && t.endsWith('"')) || (t.startsWith("'") && t.endsWith("'"))) {
    try {
      return JSON.parse(t);
    } catch {
      return t.slice(1, -1);
    }
  }
  return t;
}

try {
  let open = run("open", ["open", url]);
  if (open.status !== 0) {
    open = run("open-retry", ["open", url]);
  }
  if (open.status !== 0) {
    console.error(
      JSON.stringify(
        { ok: false, url, error: open.stderr || open.stdout || "open failed", tool: "agent-browser", session },
        null,
        2,
      ),
    );
    process.exit(1);
  }

  run("wait-network", ["wait", "--load", "networkidle"]);
  // wait until #app has real content (not just Loading…)
  for (let i = 0; i < 20; i++) {
    const probe = run("probe", [
      "eval",
      "(() => { const t=(document.body&&document.body.innerText)||''; return String(t.length)+'|'+t.slice(0,40); })()",
    ]);
    const p = parseMaybeJsonString(probe.stdout);
    const len = Number(String(p).split("|")[0]) || 0;
    if (len > 80 && !/Loading/i.test(String(p))) break;
    run("wait-tick", ["wait", "500"]);
  }

  const titleRes = run("title", ["get", "title"]);
  const title = titleRes.stdout || "";

  const textRes = run("text", [
    "eval",
    "(document.body && document.body.innerText || '').slice(0, 20000)",
  ]);
  const bodyText = parseMaybeJsonString(textRes.stdout);
  const bodyTextLen = String(bodyText).trim().length;

  run("screenshot", ["screenshot", outPng]);
  const shotOk = existsSync(outPng) && statSync(outPng).size > 500;

  ab(["close"]);

  const result = {
    url,
    status: open.status === 0 ? 200 : 0,
    title,
    bodyTextLen,
    screenshot: outPng,
    screenshotOk: shotOk,
    tool: "agent-browser",
    session,
    sample: String(bodyText).trim().slice(0, 160),
    consoleErrors: [],
    pageErrors: [],
  };

  console.log(JSON.stringify(result, null, 2));

  if (!shotOk || bodyTextLen < 40) {
    process.exit(2);
  }
  process.exit(0);
} catch (err) {
  try {
    ab(["close"]);
  } catch {}
  console.error(
    JSON.stringify({ ok: false, url, error: String(err?.message || err), tool: "agent-browser", session }, null, 2),
  );
  process.exit(1);
}
