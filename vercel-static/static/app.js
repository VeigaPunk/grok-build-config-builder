(() => {
  const PRODUCT =
    (document.body && document.body.getAttribute("data-product")) ||
    location.pathname.replace(/^\/+|\/+$/g, "").split("/")[0] ||
    "grok";
  const state = {
    schema: null,
    enabled: new Set(),
    values: {},
    tab: "builder",
    query: "",
    activeGroup: "all",
    collapsed: new Set(),
    product: PRODUCT,
  };
  const $ = (s, el = document) => el.querySelector(s);
  const $$ = (s, el = document) => [...el.querySelectorAll(s)];
  const esc = (s) => {
    const e = (n) => String.fromCharCode(38) + n + ";";
    return String(s)
      .replace(/&/g, e("amp"))
      .replace(/</g, e("lt"))
      .replace(/>/g, e("gt"))
      .replace(/"/g, e("quot"));
  };
  const toast = (msg) => {
    let t = $("#toast");
    if (!t) {
      t = document.createElement("div");
      t.id = "toast";
      Object.assign(t.style, {
        position: "fixed", bottom: "24px", right: "24px", zIndex: "100",
        padding: "10px 14px", borderRadius: "8px", background: "#1a1a1e",
        color: "#f4f4f5", fontSize: "13px", fontFamily: "inherit",
      });
      document.body.appendChild(t);
    }
    t.textContent = msg;
    t.style.opacity = "1";
    clearTimeout(t._tm);
    t._tm = setTimeout(() => { t.style.opacity = "0"; }, 1400);
  };
  const dl = (name, content, mime) => {
    const b = new Blob([content], { type: mime });
    const u = URL.createObjectURL(b);
    const a = document.createElement("a");
    a.href = u; a.download = name; a.click();
    URL.revokeObjectURL(u);
  };
  function defaults() {
    const v = {};
    for (const f of state.schema.fields) if (f.default != null) v[f.id] = f.default;
    const p = state.schema.presets && state.schema.presets[0];
    if (p && p.values) Object.assign(v, p.values);
    return v;
  }
  async function gen() {
    const r = await fetch("/api/" + state.product + "/generate", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ enabled: [...state.enabled], values: state.values }),
    });
    if (!r.ok) throw new Error("gen");
    const j = await r.json();
    if (j.config && !j.toml) j.toml = j.config;
    return j;
  }
  function sw(on) {
    return `<label class="switch"><input type="checkbox"${on ? " checked" : ""} /><span class="track"><span class="thumb"></span></span></label>`;
  }
  function fieldHtml(f) {
    const on = state.enabled.has(f.id);
    const val = state.values[f.id];
    let c = "";
    if (on) {
      if (f.type === "boolean") {
        c = `<div class="field-controls row">${sw(!!val)}<span style="font-size:13px;color:var(--fg-muted)">${val ? "true" : "false"}</span></div>`;
      } else if (f.type === "enum" && f.options) {
        const cur = String(val ?? f.default ?? f.options[0]);
        c = `<div class="field-controls"><select data-field="${f.id}">${f.options.map((o) => `<option value="${esc(o)}"${cur === o ? " selected" : ""}>${esc(o)}</option>`).join("")}</select></div>`;
      } else if (f.type === "string-list") {
        const v = Array.isArray(val) ? val.join(", ") : val ?? "";
        c = `<div class="field-controls"><input type="text" data-field="${f.id}" data-list="1" value="${esc(String(v))}" /></div>`;
      } else {
        const t = f.type === "number" ? "number" : "text";
        c = `<div class="field-controls"><input type="${t}" data-field="${f.id}" value="${esc(val == null ? "" : String(val))}" /></div>`;
      }
    }
    return `<div class="field ${on ? "on" : "off"}" data-id="${f.id}"><div class="field-row">${sw(on)}<div class="field-main"><div class="field-title"><span>${esc(f.label)}</span>${f.recommended ? `<span class="badge success">recommended</span>` : ""}<code>${esc(f.path)}</code></div><p>${esc(f.description)}</p>${f.env || f.cli ? `<div class="field-meta">${f.env ? `<code>env ${esc(f.env)}</code>` : ""}${f.cli ? `<code>cli ${esc(f.cli)}</code>` : ""}</div>` : ""}${c}</div></div></div>`;
  }
  function groupsFiltered() {
    const q = state.query.trim().toLowerCase();
    return state.schema.groups.map((g) => {
      let fields = state.schema.fields.filter((f) => f.group === g.id);
      if (state.activeGroup !== "all" && g.id !== state.activeGroup) fields = [];
      if (q) fields = fields.filter((f) => [f.label, f.path, f.description, f.env || ""].join(" ").toLowerCase().includes(q));
      return { group: g, fields };
    }).filter((x) => x.fields.length);
  }
  function builder() {
    const groups = groupsFiltered();
    const presets = state.schema.presets.map((p) => `<button type="button" class="preset-btn" data-preset="${p.id}"><strong>${esc(p.name)}</strong><span>${esc(p.description)}</span></button>`).join("");
    const mpres = state.schema.presets.map((p) => `<button type="button" class="btn sm pill" data-preset="${p.id}">${esc(p.name)}</button>`).join("");
    const secs = state.schema.groups.map((g) => `<button type="button" class="btn ghost sm" data-group="${g.id}" style="width:100%;justify-content:flex-start;border-radius:8px">${esc(g.title)}</button>`).join("");
    const body = groups.map(({ group, fields }) => {
      const col = state.collapsed.has(group.id);
      return `<section class="panel group ${col ? "collapsed" : ""}"><button type="button" class="group-head" data-toggle-group="${group.id}"><div><h2>${esc(group.title)}</h2><p>${esc(group.description)}</p></div><svg class="chev" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg></button><div class="group-body">${fields.map(fieldHtml).join("")}</div></section>`;
    }).join("");
    return `<div class="layout"><div class="mobile-strip"><div class="mobile-presets">${mpres}</div><div class="row" style="margin-top:12px"><button type="button" class="btn outline sm" data-action="enable-all">Include all</button><button type="button" class="btn outline sm" data-action="clear">Clear</button><button type="button" class="btn ghost sm" data-action="reset">Reset</button></div></div><aside class="sidebar"><div class="panel"><h3>Presets</h3>${presets}</div><div class="panel"><h3>Bulk</h3><button type="button" class="btn outline sm" style="width:100%;margin-bottom:8px" data-action="enable-all">Include all</button><button type="button" class="btn outline sm" style="width:100%;margin-bottom:8px" data-action="clear">Clear all</button><button type="button" class="btn ghost sm" style="width:100%" data-action="reset">Reset defaults</button></div><div class="panel"><h3>Sections</h3><button type="button" class="btn ghost sm" data-group="all" style="width:100%;justify-content:flex-start;border-radius:8px">All sections</button>${secs}</div></aside><div style="min-width:0"><div class="search-wrap"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg><input class="search" type="search" placeholder="Search settings" id="search" value="${esc(state.query)}" /></div>${groups.length ? body : `<div class="panel" style="padding:32px;text-align:center;color:var(--fg-muted)">No matches.</div>`}<div class="sticky-bar"><button type="button" class="btn" data-action="download-toml">Download patch</button><button type="button" class="btn secondary" data-action="tab-preview">Preview</button></div></div></div>`;
  }
  async function preview() {
    let g;
    try { g = await gen(); } catch { return `<div class="panel" style="padding:24px">Generate failed</div>`; }
    window.__gen = g;
    const cfgName = state.schema.format === "json" ? "opencode.json" : "config.toml";
    const path = state.schema.configPath || state.schema.config_path || "";
    const p = (t, s, c) => `<div class="panel preview-panel"><header><div><strong>${t}</strong><p>${s}</p></div><div class="row"><button type="button" class="btn outline sm" data-copy="1">Copy</button><button type="button" class="btn sm" data-dl="1">Download</button></div></header><pre>${esc(c)}</pre></div>`;
    return `<div class="preview-grid">${p(cfgName, path, g.toml || g.config)}${p("env", "exports", g.env)}${p("cli", "launch", g.cli)}${p("markdown", "reference", g.markdown)}</div>`;
  }
  function reference() {
    const s = state.schema;
    const flags = (s.cliFlags || []).map((f) => `<tr><td>${esc(f.flag)}</td><td>${esc(f.category)}</td><td>${esc(f.description)}</td></tr>`).join("");
    const envs = (s.envVars || []).map((e) => `<tr><td>${esc(e.name)}</td><td>${esc(e.category)}</td><td>${esc(e.description)}</td></tr>`).join("");
    const subs = (s.subcommands || []).map((c) => `<div class="panel" style="padding:8px 12px;background:var(--bg)"><code>${esc(c.cmd)}</code><p style="margin:4px 0 0;font-size:12px;color:var(--fg-muted)">${esc(c.desc)}</p></div>`).join("");
    return `<div><div class="row" style="justify-content:space-between;margin-bottom:16px"><p style="margin:0;color:var(--fg-muted);font-size:13px">Reference · ${esc(s.configPath || s.config_path || "")}</p><button type="button" class="btn secondary sm" data-action="download-md">Download .md</button></div><article class="panel ref-article"><section><h2>Subcommands</h2><div style="display:grid;gap:8px;grid-template-columns:repeat(auto-fill,minmax(220px,1fr))">${subs}</div></section><section><h2>Flags</h2><div class="table-scroll"><table><thead><tr><th>Flag</th><th>Cat</th><th>Desc</th></tr></thead><tbody>${flags}</tbody></table></div></section><section><h2>Env</h2><div class="table-scroll"><table><thead><tr><th>Var</th><th>Cat</th><th>Desc</th></tr></thead><tbody>${envs}</tbody></table></div></section></article></div>`;
  }
  async function render() {
    const root = $("#app");
    if (!root || !state.schema) return;
    const n = state.enabled.size;
    const title = state.schema.productTitle || state.schema.product_title || "Config builder";
    const tag = state.schema.productTagline || state.schema.product_tagline || "";
    const note = state.schema.versionNote || state.schema.version_note || "pure Rust";
    const tab = (id, lab) => `<button type="button" class="btn pill ${state.tab === id ? "active" : ""}" data-tab="${id}">${lab}</button>`;
    let body = state.tab === "builder" ? builder() : state.tab === "preview" ? await preview() : reference();
    root.innerHTML = `<header class="app"><div class="wrap"><div class="header-top"><div style="min-width:0"><div class="eyebrow"><a href="/" style="color:inherit;text-decoration:none">← hub</a> · ${esc(title)} · Rust</div><h1>${esc(title)}</h1><p class="lede">${esc(tag)} · JetBrainsMonoNL Nerd Font Mono</p></div><div class="row"><button type="button" class="btn secondary sm" data-action="download-md">Download .md</button><button type="button" class="btn sm" data-action="download-toml">Download patch</button></div></div><div class="tabs">${tab("builder", "Builder")}${tab("preview", "Preview")}${tab("reference", "Reference")}<span class="badge hide-sm ml-auto">${n} in patch</span></div></div></header><main class="wrap">${body}</main><footer class="app">${esc(note)} · JetBrainsMonoNL Nerd Font Mono</footer>`;
    bind(root);
  }
  function bind(root) {
    $$("[data-tab]", root).forEach((el) => el.addEventListener("click", () => { state.tab = el.getAttribute("data-tab"); render(); }));
    $$("[data-preset]", root).forEach((el) => el.addEventListener("click", () => {
      const p = state.schema.presets.find((x) => x.id === el.getAttribute("data-preset"));
      if (!p) return;
      state.enabled = new Set(p.enabled);
      Object.assign(state.values, p.values);
      toast("Applied: " + p.name);
      render();
    }));
    $$("[data-group]", root).forEach((el) => el.addEventListener("click", () => { state.activeGroup = el.getAttribute("data-group"); render(); }));
    $$("[data-toggle-group]", root).forEach((el) => el.addEventListener("click", () => {
      const id = el.getAttribute("data-toggle-group");
      if (state.collapsed.has(id)) state.collapsed.delete(id); else state.collapsed.add(id);
      render();
    }));
    $$("[data-action]", root).forEach((el) => el.addEventListener("click", async () => {
      const a = el.getAttribute("data-action");
      if (a === "enable-all") { state.enabled = new Set(state.schema.fields.map((f) => f.id)); toast("All included"); render(); }
      else if (a === "clear") { state.enabled = new Set(); toast("Cleared"); render(); }
      else if (a === "reset") { state.enabled = new Set(state.schema.presets[0].enabled); state.values = defaults(); toast("Reset"); render(); }
      else if (a === "tab-preview") { state.tab = "preview"; render(); }
      else if (a === "download-toml") {
        const g = await gen();
        const name = state.schema.format === "json" ? "opencode.json" : "config.toml";
        const mime = state.schema.format === "json" ? "application/json" : "application/toml";
        dl(name, g.toml || g.config, mime);
      } else if (a === "download-md") {
        const g = await gen();
        dl(state.product + "-config-reference.md", g.markdown, "text/markdown");
      }
    }));
    $$(".field", root).forEach((fieldEl) => {
      const id = fieldEl.getAttribute("data-id");
      const switches = $$(".switch input", fieldEl);
      if (!switches[0]) return;
      switches[0].addEventListener("change", () => {
        if (switches[0].checked) state.enabled.add(id); else state.enabled.delete(id);
        render();
      });
      if (switches[1]) switches[1].addEventListener("change", () => { state.values[id] = switches[1].checked; render(); });
    });
    $$("input[data-field], select[data-field]", root).forEach((el) => {
      const apply = () => {
        const id = el.getAttribute("data-field");
        if (el.getAttribute("data-list") === "1") state.values[id] = el.value.split(",").map((x) => x.trim()).filter(Boolean);
        else if (el.type === "number") state.values[id] = el.value === "" ? "" : Number(el.value);
        else state.values[id] = el.value;
      };
      el.addEventListener("change", apply);
      el.addEventListener("input", apply);
    });
    const search = $("#search", root);
    if (search) search.addEventListener("keydown", (e) => { if (e.key === "Enter") { state.query = search.value; render(); } });
    $$(".preview-panel", root).forEach((panel, i) => {
      const keys = ["toml", "env", "cli", "markdown"];
      const isJson = state.schema && state.schema.format === "json";
      const files = [
        [isJson ? "opencode.json" : "config.toml", isJson ? "application/json" : "application/toml"],
        ["env.sh", "text/plain"],
        ["launch.sh", "text/plain"],
        [state.product + "-config-reference.md", "text/markdown"],
      ];
      let content = (window.__gen && window.__gen[keys[i]]) || "";
      if (i === 0 && !content && window.__gen) content = window.__gen.config || "";
      const btns = $$("button", panel);
      if (btns[0]) btns[0].addEventListener("click", async () => { try { await navigator.clipboard.writeText(content); toast("Copied"); } catch { toast("Copy failed"); } });
      if (btns[1]) btns[1].addEventListener("click", () => dl(files[i][0], content, files[i][1]));
    });
  }
  (async () => {
    const r = await fetch("/api/" + state.product + "/schema");
    state.schema = await r.json();
    if (state.schema.cli_flags) state.schema.cliFlags = state.schema.cli_flags;
    if (state.schema.env_vars) state.schema.envVars = state.schema.env_vars;
    state.enabled = new Set(state.schema.presets[0].enabled);
    state.values = defaults();
    await render();
  })().catch((e) => {
    document.getElementById("app").textContent = String(e);
  });
})();
