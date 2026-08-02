/* Grok Build Config Builder — client UI (schema from Rust /api/schema) */
(() => {
  const state = {
    schema: null,
    enabled: new Set(),
    values: {},
    tab: "builder",
    query: "",
    activeGroup: "all",
    collapsed: new Set(),
  };

  const $ = (sel, el = document) => el.querySelector(sel);
  const $$ = (sel, el = document) => [...el.querySelectorAll(sel)];

  function downloadText(filename, content, mime = "text/plain") {
    const blob = new Blob([content], { type: mime });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  }

  async function copyText(text) {
    try {
      await navigator.clipboard.writeText(text);
      toast("Copied");
    } catch {
      toast("Could not copy");
    }
  }

  function toast(msg) {
    let t = $("#toast");
    if (!t) {
      t = document.createElement("div");
      t.id = "toast";
      t.style.cssText =
        "position:fixed;bottom:24px;right:24px;z-index:100;padding:10px 14px;border-radius:8px;background:#1a1a1e;border:1px solid color-mix(in oklab,#f4f4f5 12%,transparent);color:#f4f4f5;font-size:13px;opacity:0;transition:opacity 150ms";
      document.body.appendChild(t);
    }
    t.textContent = msg;
    t.style.opacity = "1";
    clearTimeout(t._tm);
    t._tm = setTimeout(() => {
      t.style.opacity = "0";
    }, 1600);
  }

  function defaultValues() {
    const values = {};
    for (const f of state.schema.fields) {
      if (f.default !== undefined && f.default !== null) values[f.id] = f.default;
    }
    const p0 = state.schema.presets[0];
    if (p0 && p0.values) Object.assign(values, p0.values);
    return values;
  }

  function defaultEnabled() {
    return new Set(state.schema.presets[0]?.enabled || []);
  }

  async function generate() {
    const res = await fetch("/api/generate", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        enabled: [...state.enabled],
        values: state.values,
      }),
    });
    if (!res.ok) throw new Error("generate failed");
    return res.json();
  }

  function applyPreset(id) {
    const p = state.schema.presets.find((x) => x.id === id);
    if (!p) return;
    state.enabled = new Set(p.enabled);
    state.values = { ...state.values, ...p.values };
    toast(`Applied: ${p.name}`);
    render();
  }

  function setTab(tab) {
    state.tab = tab;
    render();
  }

  function switchEl(checked, onChange, aria) {
    const id = "sw_" + Math.random().toString(36).slice(2, 9);
    return `<label class="switch" title="${aria || ""}">
      <input type="checkbox" id="${id}" ${checked ? "checked" : ""} data-aria="${aria || ""}" />
      <span class="track"><span class="thumb"></span></span>
    </label>`;
  }

  function fieldControl(f) {
    const on = state.enabled.has(f.id);
    const val = state.values[f.id];
    let controls = "";
    if (on) {
      if (f.type === "boolean") {
        controls = `<div class="field-controls row">${switchEl(!!val, null, f.label)}
          <span style="font-size:14px;color:var(--fg-muted)">${val ? "true" : "false"}</span></div>`;
      } else if (f.type === "enum" && f.options) {
        controls = `<div class="field-controls"><select data-field="${f.id}">
          ${f.options
            .map(
              (o) =>
                `<option value="${o}" ${String(val ?? f.default ?? f.options[0]) === o ? "selected" : ""}>${o}</option>`
            )
            .join("")}
        </select></div>`;
      } else if (f.type === "string-list") {
        const v = Array.isArray(val) ? val.join(", ") : val ?? "";
        controls = `<div class="field-controls"><input type="text" data-field="${f.id}" data-list="1" value="${escapeAttr(String(v))}" placeholder="comma,separated,values" /></div>`;
      } else {
        const t = f.type === "number" ? "number" : "text";
        controls = `<div class="field-controls"><input type="${t}" data-field="${f.id}" value="${escapeAttr(val === undefined || val === null ? "" : String(val))}" placeholder="${escapeAttr(String(f.default ?? f.label))}" /></div>`;
      }
    }
    return `<div class="field ${on ? "on" : "off"}" data-id="${f.id}">
      <div class="field-row">
        ${switchEl(on, null, "Include " + f.label)}
        <div class="field-main">
          <div class="field-title">
            <span>${escapeHtml(f.label)}</span>
            ${f.recommended ? '<span class="badge success">recommended</span>' : ""}
            <code>${escapeHtml(f.path)}</code>
          </div>
          <p>${escapeHtml(f.description)}</p>
          ${
            f.env || f.cli
              ? `<div class="field-meta">${f.env ? `<code>env ${escapeHtml(f.env)}</code>` : ""}${f.cli ? `<code>cli ${escapeHtml(f.cli)}</code>` : ""}</div>`
              : ""
          }
          ${controls}
        </div>
      </div>
    </div>`;
  }

  function escapeHtml(s) {
    return String(s)
      .replace(/&/g, "&")
      .replace(/</g, "<")
      .replace(/>/g, ">")
      .replace(/"/g, """);
  }
  function escapeAttr(s) {
    return escapeHtml(s).replace(/'/g, "&#39;");
  }

  function filteredGroups() {
    const q = state.query.trim().toLowerCase();
    return state.schema.groups
      .map((g) => {
        let fields = state.schema.fields.filter((f) => f.group === g.id);
        if (state.activeGroup !== "all" && g.id !== state.activeGroup) fields = [];
        if (q) {
          fields = fields.filter(
            (f) =>
              f.label.toLowerCase().includes(q) ||
              f.path.toLowerCase().includes(q) ||
              f.description.toLowerCase().includes(q) ||
              (f.env && f.env.toLowerCase().includes(q))
          );
        }
        return { group: g, fields };
      })
      .filter((x) => x.fields.length > 0);
  }

  function renderBuilder() {
    const groups = filteredGroups();
    const presets = state.schema.presets
      .map(
        (p) =>
          `<button type="button" class="preset-btn" data-preset="${p.id}"><strong>${escapeHtml(p.name)}</strong><span>${escapeHtml(p.description)}</span></button>`
      )
      .join("");

    const mobilePresets = state.schema.presets
      .map((p) => `<button type="button" class="btn sm pill" data-preset="${p.id}">${escapeHtml(p.name)}</button>`)
      .join("");

    const sections = state.schema.groups
      .map(
        (g) =>
          `<button type="button" class="btn ghost sm ${state.activeGroup === g.id ? "active" : ""}" data-group="${g.id}" style="width:100%;justify-content:flex-start;border-radius:8px">${escapeHtml(g.title)}</button>`
      )
      .join("");

    const groupHtml = groups
      .map(({ group, fields }) => {
        const collapsed = state.collapsed.has(group.id);
        return `<section class="panel group ${collapsed ? "collapsed" : ""}" data-group-id="${group.id}">
          <button type="button" class="group-head" data-toggle-group="${group.id}">
            <div><h2>${escapeHtml(group.title)}</h2><p>${escapeHtml(group.description)}</p></div>
            <svg class="chev" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m6 9 6 6 6-6"/></svg>
          </button>
          <div class="group-body">${fields.map(fieldControl).join("")}</div>
        </section>`;
      })
      .join("");

    return `<div class="layout">
      <div class="mobile-strip">
        <div class="mobile-presets">${mobilePresets}</div>
        <div class="row" style="margin-top:12px">
          <button type="button" class="btn outline sm" data-action="enable-all">Include all</button>
          <button type="button" class="btn outline sm" data-action="clear">Clear</button>
          <button type="button" class="btn ghost sm" data-action="reset">Reset</button>
        </div>
      </div>
      <aside class="sidebar">
        <div class="panel"><h3>Presets</h3>${presets}</div>
        <div class="panel">
          <h3>Bulk</h3>
          <button type="button" class="btn outline sm" style="width:100%;margin-bottom:8px" data-action="enable-all">Include all</button>
          <button type="button" class="btn outline sm" style="width:100%;margin-bottom:8px" data-action="clear">Clear all</button>
          <button type="button" class="btn ghost sm" style="width:100%" data-action="reset">Reset defaults</button>
        </div>
        <div class="panel">
          <h3>Sections</h3>
          <button type="button" class="btn ghost sm ${state.activeGroup === "all" ? "active" : ""}" data-group="all" style="width:100%;justify-content:flex-start;border-radius:8px">All sections</button>
          ${sections}
        </div>
      </aside>
      <div class="min-w-0" style="min-width:0">
        <div class="search-wrap">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
          <input class="search" type="search" placeholder="Search settings, paths, env vars…" value="${escapeAttr(state.query)}" id="search" />
        </div>
        ${groups.length === 0 ? `<div class="panel" style="padding:32px;text-align:center;color:var(--fg-muted)">No settings match your search.</div>` : groupHtml}
        <div class="sticky-bar">
          <button type="button" class="btn" data-action="download-toml">Download patch</button>
          <button type="button" class="btn secondary" data-action="tab-preview">View preview</button>
        </div>
      </div>
    </div>`;
  }

  async function renderPreview() {
    let gen;
    try {
      gen = await generate();
    } catch {
      return `<div class="panel" style="padding:24px;color:var(--fg-muted)">Could not generate patch.</div>`;
    }
    const panel = (title, sub, content, file, mime) =>
      `<div class="panel preview-panel">
        <header>
          <div><strong>${title}</strong><p>${sub}</p></div>
          <div class="row">
            <button type="button" class="btn outline sm" data-copy="${escapeAttr(content)}">Copy</button>
            <button type="button" class="btn sm" data-dl-name="${escapeAttr(file)}" data-dl-mime="${mime}" data-dl-body="${escapeAttr(content)}">Download</button>
          </div>
        </header>
        <pre>${escapeHtml(content)}</pre>
      </div>`;
    // data attributes with full content can be huge — use store instead
    window.__gen = gen;
    return `<div class="preview-grid">
      ${panel("config.toml patch", "Merge into ~/.grok/config.toml", gen.toml, "config.toml", "application/toml")}
      ${panel("Environment exports", "Shell / CI overrides", gen.env, "grok-env.sh", "text/x-shellscript")}
      ${panel("CLI launch snippet", "Flags from your selection", gen.cli, "grok-launch.sh", "text/x-shellscript")}
      ${panel("Full markdown reference", "Compiled from Rust schema", gen.markdown, "grok-build-cli-config-reference.md", "text/markdown")}
    </div>`;
  }

  function renderReference() {
    const s = state.schema;
    const flags = s.cliFlags
      .map(
        (f) =>
          `<tr><td class="mono" style="white-space:nowrap;color:var(--fg)">${escapeHtml(f.flag)}</td><td>${escapeHtml(f.category)}</td><td>${escapeHtml(f.description)}</td></tr>`
      )
      .join("");
    const envs = s.envVars
      .map(
        (e) =>
          `<tr><td class="mono" style="white-space:nowrap;color:var(--fg)">${escapeHtml(e.name)}</td><td>${escapeHtml(e.category)}</td><td>${escapeHtml(e.description)}</td></tr>`
      )
      .join("");
    const subs = s.subcommands
      .map(
        (c) =>
          `<div class="panel" style="padding:8px 12px;border-radius:12px;background:var(--bg)"><code class="mono" style="font-size:12px">${escapeHtml(c.cmd)}</code><p style="margin:4px 0 0;font-size:12px;color:var(--fg-muted)">${escapeHtml(c.desc)}</p></div>`
      )
      .join("");
    const keys = s.groups
      .map((g) => {
        const fs = s.fields
          .filter((f) => f.group === g.id)
          .map(
            (f) =>
              `<div style="border:1px solid var(--border);border-radius:12px;background:var(--bg);padding:8px 12px;margin-bottom:8px;min-width:0">
                <div class="row"><code class="mono" style="font-size:12px;word-break:break-all">${escapeHtml(f.path)}</code>
                <span class="badge">${escapeHtml(f.type)}</span>${f.recommended ? '<span class="badge success">recommended</span>' : ""}</div>
                <p style="margin:4px 0 0;font-size:14px;color:var(--fg-muted)">${escapeHtml(f.description)}</p>
              </div>`
          )
          .join("");
        return `<div style="margin-bottom:24px"><h3 style="margin:0 0 8px;font-size:16px">${escapeHtml(g.title)}</h3>${fs}</div>`;
      })
      .join("");

    return `<div>
      <div class="row" style="justify-content:space-between;margin-bottom:16px">
        <p style="margin:0;font-size:14px;color:var(--fg-muted)">Full reference also downloadable as <code class="mono">.md</code>.</p>
        <button type="button" class="btn secondary sm" data-action="download-md">Download markdown</button>
      </div>
      <article class="panel ref-article">
        <section>
          <h2>Install</h2>
          <pre>curl -fsSL https://x.ai/cli/install.sh | bash
grok version
grok update --stable</pre>
        </section>
        <section>
          <h2>Subcommands</h2>
          <div style="display:grid;gap:8px;grid-template-columns:repeat(auto-fill,minmax(240px,1fr))">${subs}</div>
        </section>
        <section>
          <h2>Launch flags</h2>
          <div class="table-scroll"><table><thead><tr><th>Flag</th><th>Category</th><th>Description</th></tr></thead><tbody>${flags}</tbody></table></div>
        </section>
        <section>
          <h2>Environment variables</h2>
          <div class="table-scroll"><table><thead><tr><th>Variable</th><th>Category</th><th>Description</th></tr></thead><tbody>${envs}</tbody></table></div>
        </section>
        <section>
          <h2>All config.toml keys</h2>
          ${keys}
        </section>
      </article>
    </div>`;
  }

  async function render() {
    const root = $("#app");
    if (!root || !state.schema) return;
    const n = state.enabled.size;
    const tabBtn = (id, label) =>
      `<button type="button" class="btn pill ${state.tab === id ? "active" : ""}" data-tab="${id}">${label}</button>`;

    let body = "";
    if (state.tab === "builder") body = renderBuilder();
    else if (state.tab === "preview") body = await renderPreview();
    else body = renderReference();

    root.innerHTML = `
      <header class="app">
        <div class="wrap">
          <div class="header-top">
            <div style="min-width:0">
              <div class="eyebrow">⚙ Grok Build · Rust</div>
              <h1>Config reference & patch builder</h1>
              <p class="lede">Toggle every setting, preview a custom <code class="mono">config.toml</code> patch, download for <code class="mono">~/.grok/config.toml</code>. Schema & generation run in Rust.</p>
            </div>
            <div class="row">
              <button type="button" class="btn secondary sm" data-action="download-md">Download .md</button>
              <button type="button" class="btn sm" data-action="download-toml">Download patch</button>
            </div>
          </div>
          <div class="tabs">
            ${tabBtn("builder", "Builder")}
            ${tabBtn("preview", "Preview")}
            ${tabBtn("reference", "Reference")}
            <span class="badge hide-sm ml-auto">${n} setting${n === 1 ? "" : "s"} in patch</span>
          </div>
        </div>
      </header>
      <main class="wrap" id="main">${body}</main>
      <footer class="app">Grok Build config builder · pure Rust · verify against docs.x.ai for your installed version</footer>
    `;
    bind(root);
  }

  function bind(root) {
    $$("[data-tab]", root).forEach((el) =>
      el.addEventListener("click", () => setTab(el.getAttribute("data-tab")))
    );
    $$("[data-preset]", root).forEach((el) =>
      el.addEventListener("click", () => applyPreset(el.getAttribute("data-preset")))
    );
    $$("[data-group]", root).forEach((el) =>
      el.addEventListener("click", () => {
        state.activeGroup = el.getAttribute("data-group");
        render();
      })
    );
    $$("[data-toggle-group]", root).forEach((el) =>
      el.addEventListener("click", () => {
        const id = el.getAttribute("data-toggle-group");
        if (state.collapsed.has(id)) state.collapsed.delete(id);
        else state.collapsed.add(id);
        render();
      })
    );
    $$("[data-action]", root).forEach((el) =>
      el.addEventListener("click", async () => {
        const a = el.getAttribute("data-action");
        if (a === "enable-all") {
          state.enabled = new Set(state.schema.fields.map((f) => f.id));
          toast("All settings included");
          render();
        } else if (a === "clear") {
          state.enabled = new Set();
          toast("Cleared");
          render();
        } else if (a === "reset") {
          state.enabled = defaultEnabled();
          state.values = defaultValues();
          toast("Reset to privacy-first defaults");
          render();
        } else if (a === "tab-preview") {
          setTab("preview");
        } else if (a === "download-toml") {
          const g = await generate();
          downloadText("config.toml", g.toml, "application/toml");
        } else if (a === "download-md") {
          const g = await generate();
          downloadText("grok-build-cli-config-reference.md", g.markdown, "text/markdown");
        }
      })
    );

    // field toggles
    $$(".field", root).forEach((fieldEl) => {
      const id = fieldEl.getAttribute("data-id");
      const sw = fieldEl.querySelector(".switch input");
      if (!sw) return;
      // first switch is include toggle
      sw.addEventListener("change", () => {
        if (sw.checked) state.enabled.add(id);
        else state.enabled.delete(id);
        render();
      });
    });

    $$("input[data-field], select[data-field]", root).forEach((el) => {
      el.addEventListener("change", () => {
        const id = el.getAttribute("data-field");
        if (el.getAttribute("data-list") === "1") {
          state.values[id] = el.value
            .split(",")
            .map((x) => x.trim())
            .filter(Boolean);
        } else if (el.type === "number") {
          state.values[id] = el.value === "" ? "" : Number(el.value);
        } else if (el.type === "checkbox") {
          state.values[id] = el.checked;
        } else {
          state.values[id] = el.value;
        }
        // boolean value switches are second switch — handled below
      });
      el.addEventListener("input", () => {
        const id = el.getAttribute("data-field");
        if (el.getAttribute("data-list") === "1") {
          state.values[id] = el.value
            .split(",")
            .map((x) => x.trim())
            .filter(Boolean);
        } else if (el.type === "number") {
          state.values[id] = el.value === "" ? "" : Number(el.value);
        } else {
          state.values[id] = el.value;
        }
      });
    });

    // boolean value switches (second switch in field)
    $$(".field", root).forEach((fieldEl) => {
      const id = fieldEl.getAttribute("data-id");
      const switches = $$(".switch input", fieldEl);
      if (switches.length < 2) return;
      const valSw = switches[1];
      valSw.addEventListener("change", () => {
        state.values[id] = valSw.checked;
        render();
      });
    });

    const search = $("#search", root);
    if (search) {
      search.addEventListener("input", () => {
        state.query = search.value;
      });
      search.addEventListener("change", () => {
        state.query = search.value;
        render();
      });
      search.addEventListener("keydown", (e) => {
        if (e.key === "Enter") {
          state.query = search.value;
          render();
        }
      });
    }

    // preview copy/download without embedding huge attrs — use __gen
    $$(".preview-panel", root).forEach((panel, i) => {
      const keys = ["toml", "env", "cli", "markdown"];
      const files = [
        ["config.toml", "application/toml"],
        ["grok-env.sh", "text/x-shellscript"],
        ["grok-launch.sh", "text/x-shellscript"],
        ["grok-build-cli-config-reference.md", "text/markdown"],
      ];
      const key = keys[i];
      const content = window.__gen?.[key] || "";
      const btns = $$("button", panel);
      if (btns[0]) btns[0].addEventListener("click", () => copyText(content));
      if (btns[1])
        btns[1].addEventListener("click", () =>
          downloadText(files[i][0], content, files[i][1])
        );
    });
  }

  async function init() {
    const res = await fetch("/api/schema");
    state.schema = await res.json();
    // normalize camelCase from serde
    if (state.schema.cli_flags) state.schema.cliFlags = state.schema.cli_flags;
    if (state.schema.env_vars) state.schema.envVars = state.schema.env_vars;
    state.enabled = defaultEnabled();
    state.values = defaultValues();
    await render();
  }

  init().catch((e) => {
    document.getElementById("app").innerHTML =
      `<div class="wrap" style="padding:48px;color:#f87171">Failed to load schema: ${escapeHtml(String(e))}</div>`;
  });
})();
