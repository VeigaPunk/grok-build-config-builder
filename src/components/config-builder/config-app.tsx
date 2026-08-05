import { useCallback, useEffect, useMemo, useState } from "react";
import { Link } from "@tanstack/react-router";
import {
  ChevronDown,
  Copy,
  Download,
  ExternalLink,
  Search,
  RotateCcw,
  Check,
  Layers,
  FileCode2,
  BookOpen,
} from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { Badge } from "@/components/ui/badge";
import {
  applyPreset,
  defaultValues,
  generatePatch,
  type GenerateResult,
} from "@/lib/generate";
import {
  normalizeSchema,
  type ConfigField,
  type ProductId,
  type ProductSchema,
} from "@/lib/schema-types";
import { cn, copyText, downloadBlob } from "@/lib/utils";

type Tab = "builder" | "preview" | "reference";

type Props = {
  product: ProductId;
  schema: ProductSchema;
};

export function ConfigApp({ product, schema: raw }: Props) {
  const schema = useMemo(() => normalizeSchema(raw), [raw]);
  const [enabled, setEnabled] = useState(() => new Set(schema.presets[0]?.enabled ?? []));
  const [values, setValues] = useState(() => defaultValues(schema));
  const [tab, setTab] = useState<Tab>("builder");
  const [query, setQuery] = useState("");
  const [activeGroup, setActiveGroup] = useState("all");
  const [collapsed, setCollapsed] = useState<Set<string>>(() => new Set());
  const [activePreset, setActivePreset] = useState(schema.presets[0]?.id ?? "");

  const patch = useMemo(
    () => generatePatch(schema, enabled, values),
    [schema, enabled, values],
  );

  const title = schema.productTitle || product;
  const tagline = schema.productTagline || "";

  const onPreset = (id: string) => {
    const next = applyPreset(schema, id, values);
    setEnabled(next.enabled);
    setValues(next.values);
    setActivePreset(id);
    const name = schema.presets.find((p) => p.id === id)?.name ?? id;
    toast.success(`Applied: ${name}`);
  };

  const toggleField = (id: string, on: boolean) => {
    setEnabled((prev) => {
      const n = new Set(prev);
      if (on) n.add(id);
      else n.delete(id);
      return n;
    });
  };

  const setValue = (id: string, v: unknown) => {
    setValues((prev) => ({ ...prev, [id]: v }));
  };

  const groupsFiltered = useMemo(() => {
    const q = query.trim().toLowerCase();
    return schema.groups
      .map((g) => {
        let fields = schema.fields.filter((f) => f.group === g.id);
        if (activeGroup !== "all" && g.id !== activeGroup) fields = [];
        if (q) {
          fields = fields.filter((f) =>
            [f.label, f.path, f.description, f.env || ""].join(" ").toLowerCase().includes(q),
          );
        }
        return { group: g, fields };
      })
      .filter((x) => x.fields.length);
  }, [schema, query, activeGroup]);

  const downloadConfig = () => {
    downloadBlob(patch.filename, patch.config, patch.mime);
    toast.success(`Downloaded ${patch.filename}`);
  };

  const downloadMd = () => {
    downloadBlob(`${product}-config-reference.md`, patch.markdown, "text/markdown");
    toast.success("Downloaded reference markdown");
  };

  return (
    <div className="flex min-h-dvh flex-col">
      <header className="sticky top-0 z-40 border-b border-border bg-bg/90 backdrop-blur-md">
        <div className="mx-auto w-full max-w-7xl px-4 py-4 sm:px-6">
          <div className="flex flex-wrap items-start justify-between gap-4">
            <div className="min-w-0 max-w-3xl">
              <div className="flex flex-wrap items-center gap-2 text-[11px] font-medium tracking-[0.08em] text-fg-subtle uppercase">
                <Link to="/" className="text-fg-muted transition-colors hover:text-fg">
                  Hub
                </Link>
                <span aria-hidden>·</span>
                <span>{title}</span>
                <span aria-hidden>·</span>
                <span>Titanium</span>
              </div>
              <h1 className="mt-1 text-balance text-[clamp(1.35rem,3vw,1.75rem)] font-semibold tracking-tight text-fg">
                {title}
              </h1>
              {tagline ? (
                <p className="mt-1 max-w-2xl text-[13px] leading-relaxed text-fg-muted">{tagline}</p>
              ) : null}
              {product === "grok" ? (
                <p className="mt-2 max-w-2xl text-[12px] leading-relaxed text-fg-muted">
                  <strong className="font-medium text-fg">Titanium · xbgst</strong> ships
                  xbgst-stack marketplaces + always-approve, hard-bans{" "}
                  <code className="rounded-xs bg-bg-subtle px-1 py-0.5 text-[11px]">
                    general-purpose
                  </code>
                  /
                  <code className="rounded-xs bg-bg-subtle px-1 py-0.5 text-[11px]">explore</code>{" "}
                  via{" "}
                  <a
                    href="https://github.com/VeigaPunk/grok-build-livepatch"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="underline decoration-border-strong underline-offset-2 hover:text-fg"
                  >
                    grok-build-livepatch
                  </a>
                  .{" "}
                  <a
                    href="/recommended-grok-config.toml"
                    className="underline decoration-border-strong underline-offset-2 hover:text-fg"
                  >
                    Recommended config.toml
                  </a>
                </p>
              ) : null}
              {product === "codex" ? (
                <p className="mt-2 max-w-2xl text-[12px] leading-relaxed text-fg-muted">
                  <strong className="font-medium text-fg">Optimal with Sekhmet</strong> (
                  <a
                    href="https://github.com/VeigaPunk/xbrd-spark"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="underline decoration-border-strong underline-offset-2 hover:text-fg"
                  >
                    VeigaPunk/xbrd-spark
                  </a>
                  ). Titanium unrestricted pairs with concurrent{" "}
                  <code className="rounded-xs bg-bg-subtle px-1 py-0.5 text-[11px]">
                    sekhmet swarm -j N
                  </code>{" "}
                  (cap 64).
                </p>
              ) : null}
            </div>
            <div className="flex flex-wrap items-center gap-2">
              <Button variant="secondary" size="sm" onClick={downloadMd}>
                <BookOpen className="size-3.5" />
                Docs
              </Button>
              <Button size="sm" onClick={downloadConfig}>
                <Download className="size-3.5" />
                Download patch
              </Button>
            </div>
          </div>

          <div className="mt-4 flex flex-wrap items-center gap-2">
            <TabButton active={tab === "builder"} onClick={() => setTab("builder")} icon={Layers}>
              Builder
            </TabButton>
            <TabButton
              active={tab === "preview"}
              onClick={() => setTab("preview")}
              icon={FileCode2}
            >
              Preview
            </TabButton>
            <TabButton
              active={tab === "reference"}
              onClick={() => setTab("reference")}
              icon={BookOpen}
            >
              Reference
            </TabButton>
            <Badge className="ml-auto hidden sm:inline-flex">{enabled.size} in patch</Badge>
          </div>
        </div>
      </header>

      <main className="mx-auto w-full max-w-7xl flex-1 px-4 py-6 sm:px-6 sm:py-8">
        {tab === "builder" ? (
          <BuilderView
            schema={schema}
            enabled={enabled}
            values={values}
            groupsFiltered={groupsFiltered}
            query={query}
            setQuery={setQuery}
            activeGroup={activeGroup}
            setActiveGroup={setActiveGroup}
            collapsed={collapsed}
            setCollapsed={setCollapsed}
            activePreset={activePreset}
            onPreset={onPreset}
            toggleField={toggleField}
            setValue={setValue}
            onEnableAll={() => {
              setEnabled(new Set(schema.fields.map((f) => f.id)));
              toast.message("All settings included");
            }}
            onClear={() => {
              setEnabled(new Set());
              toast.message("Cleared");
            }}
            onReset={() => {
              onPreset(schema.presets[0]?.id ?? "");
            }}
            onPreview={() => setTab("preview")}
            onDownload={downloadConfig}
          />
        ) : null}
        {tab === "preview" ? <PreviewView patch={patch} product={product} /> : null}
        {tab === "reference" ? (
          <ReferenceView schema={schema} onDownloadMd={downloadMd} />
        ) : null}
      </main>

      <footer className="border-t border-border px-4 py-8 text-center text-[11px] text-fg-subtle">
        {schema.versionNote || "Titanium schemas"} · Agent Config Builders ·{" "}
        <a
          href="https://github.com/VeigaPunk/grok-build-config-builder"
          target="_blank"
          rel="noopener noreferrer"
          className="underline decoration-border underline-offset-2 hover:text-fg-muted"
        >
          source
        </a>
      </footer>
    </div>
  );
}

function TabButton({
  active,
  onClick,
  children,
  icon: Icon,
}: {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
  icon: React.ComponentType<{ className?: string }>;
}) {
  return (
    <Button
      type="button"
      size="sm"
      variant={active ? "pill-active" : "pill"}
      onClick={onClick}
      className="gap-1.5"
    >
      <Icon className="size-3.5 opacity-80" />
      {children}
    </Button>
  );
}

function BuilderView({
  schema,
  enabled,
  values,
  groupsFiltered,
  query,
  setQuery,
  activeGroup,
  setActiveGroup,
  collapsed,
  setCollapsed,
  activePreset,
  onPreset,
  toggleField,
  setValue,
  onEnableAll,
  onClear,
  onReset,
  onPreview,
  onDownload,
}: {
  schema: ProductSchema;
  enabled: Set<string>;
  values: Record<string, unknown>;
  groupsFiltered: { group: { id: string; title: string; description: string }; fields: ConfigField[] }[];
  query: string;
  setQuery: (q: string) => void;
  activeGroup: string;
  setActiveGroup: (g: string) => void;
  collapsed: Set<string>;
  setCollapsed: React.Dispatch<React.SetStateAction<Set<string>>>;
  activePreset: string;
  onPreset: (id: string) => void;
  toggleField: (id: string, on: boolean) => void;
  setValue: (id: string, v: unknown) => void;
  onEnableAll: () => void;
  onClear: () => void;
  onReset: () => void;
  onPreview: () => void;
  onDownload: () => void;
}) {
  return (
    <div className="grid min-w-0 gap-6 lg:grid-cols-[240px_minmax(0,1fr)]">
      {/* Mobile presets */}
      <div className="min-w-0 space-y-3 lg:hidden">
        <div className="flex gap-2 overflow-x-auto pb-1 scrollbar-thin">
          {schema.presets.map((p) => (
            <Button
              key={p.id}
              size="sm"
              variant={activePreset === p.id ? "pill-active" : "pill"}
              className="shrink-0"
              onClick={() => onPreset(p.id)}
            >
              {p.name}
            </Button>
          ))}
        </div>
        <div className="flex flex-wrap gap-2">
          <Button variant="outline" size="sm" onClick={onEnableAll}>
            Include all
          </Button>
          <Button variant="outline" size="sm" onClick={onClear}>
            Clear
          </Button>
          <Button variant="ghost" size="sm" onClick={onReset}>
            <RotateCcw className="size-3.5" />
            Reset
          </Button>
        </div>
      </div>

      <aside className="hidden min-w-0 lg:block">
        <div className="sticky top-36 space-y-4">
          <section className="surface rounded-xl p-4">
            <h3 className="mb-3 text-[11px] font-medium tracking-[0.08em] text-fg-subtle uppercase">
              Presets
            </h3>
            <div className="space-y-2">
              {schema.presets.map((p) => (
                <button
                  key={p.id}
                  type="button"
                  onClick={() => onPreset(p.id)}
                  className={cn(
                    "w-full rounded-md border px-3 py-2.5 text-left transition-colors",
                    activePreset === p.id
                      ? "border-border-strong bg-bg-subtle"
                      : "border-border bg-bg-subtle/40 hover:border-border-strong",
                  )}
                >
                  <span className="block text-[13px] font-medium text-fg">{p.name}</span>
                  <span className="mt-0.5 block text-[11px] leading-snug text-fg-muted">
                    {p.description}
                  </span>
                </button>
              ))}
            </div>
          </section>

          <section className="surface rounded-xl p-4">
            <h3 className="mb-3 text-[11px] font-medium tracking-[0.08em] text-fg-subtle uppercase">
              Bulk
            </h3>
            <div className="space-y-2">
              <Button variant="outline" size="sm" className="w-full" onClick={onEnableAll}>
                Include all
              </Button>
              <Button variant="outline" size="sm" className="w-full" onClick={onClear}>
                Clear all
              </Button>
              <Button variant="ghost" size="sm" className="w-full" onClick={onReset}>
                <RotateCcw className="size-3.5" />
                Reset defaults
              </Button>
            </div>
          </section>

          <section className="surface rounded-xl p-4">
            <h3 className="mb-3 text-[11px] font-medium tracking-[0.08em] text-fg-subtle uppercase">
              Sections
            </h3>
            <div className="space-y-1">
              <Button
                variant="ghost"
                size="sm"
                className={cn(
                  "w-full justify-start rounded-sm",
                  activeGroup === "all" && "bg-bg-subtle text-fg",
                )}
                onClick={() => setActiveGroup("all")}
              >
                All sections
              </Button>
              {schema.groups.map((g) => (
                <Button
                  key={g.id}
                  variant="ghost"
                  size="sm"
                  className={cn(
                    "w-full justify-start rounded-sm",
                    activeGroup === g.id && "bg-bg-subtle text-fg",
                  )}
                  onClick={() => setActiveGroup(g.id)}
                >
                  {g.title}
                </Button>
              ))}
            </div>
          </section>
        </div>
      </aside>

      <div className="min-w-0">
        <div className="relative mb-4">
          <Search className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-fg-subtle" />
          <input
            type="search"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search settings"
            className="min-h-10 w-full rounded-sm border border-border bg-bg-subtle py-2 pr-3 pl-10 text-[13px] text-fg placeholder:text-fg-subtle focus:border-border-strong focus:outline-none"
          />
        </div>

        {groupsFiltered.length === 0 ? (
          <div className="surface rounded-xl px-8 py-12 text-center text-fg-muted">
            No matches for your search.
          </div>
        ) : (
          <div className="space-y-4">
            {groupsFiltered.map(({ group, fields }) => {
              const isCollapsed = collapsed.has(group.id);
              return (
                <section key={group.id} className="surface overflow-hidden rounded-xl">
                  <button
                    type="button"
                    className="flex w-full items-center justify-between gap-3 px-5 py-4 text-left transition-colors hover:bg-bg-subtle/40"
                    onClick={() =>
                      setCollapsed((prev) => {
                        const n = new Set(prev);
                        if (n.has(group.id)) n.delete(group.id);
                        else n.add(group.id);
                        return n;
                      })
                    }
                  >
                    <div>
                      <h2 className="text-sm font-semibold text-fg">{group.title}</h2>
                      <p className="mt-0.5 text-xs text-fg-muted">{group.description}</p>
                    </div>
                    <ChevronDown
                      className={cn(
                        "size-5 shrink-0 text-fg-muted transition-transform duration-200",
                        !isCollapsed && "rotate-180",
                      )}
                    />
                  </button>
                  {!isCollapsed ? (
                    <div className="space-y-3 border-t border-border px-5 py-4">
                      {fields.map((f) => (
                        <FieldCard
                          key={f.id}
                          field={f}
                          on={enabled.has(f.id)}
                          value={values[f.id]}
                          onToggle={(on) => toggleField(f.id, on)}
                          onChange={(v) => setValue(f.id, v)}
                        />
                      ))}
                    </div>
                  ) : null}
                </section>
              );
            })}
          </div>
        )}

        <div className="sticky bottom-4 z-30 mt-6 flex flex-wrap gap-2 rounded-xl border border-border bg-bg-elevated/95 p-3 shadow-[var(--shadow-panel)] backdrop-blur-sm lg:hidden">
          <Button className="min-h-11 flex-1" onClick={onDownload}>
            Download patch
          </Button>
          <Button variant="secondary" className="min-h-11 flex-1" onClick={onPreview}>
            Preview
          </Button>
        </div>
      </div>
    </div>
  );
}

function FieldCard({
  field,
  on,
  value,
  onToggle,
  onChange,
}: {
  field: ConfigField;
  on: boolean;
  value: unknown;
  onToggle: (on: boolean) => void;
  onChange: (v: unknown) => void;
}) {
  return (
    <div
      className={cn(
        "rounded-lg border border-border p-3.5 transition-[background,opacity] duration-150 sm:p-4",
        on ? "bg-bg-elevated" : "bg-bg/40 opacity-85",
      )}
    >
      <div className="flex gap-3">
        <Switch
          checked={on}
          onCheckedChange={onToggle}
          aria-label={`Include ${field.label}`}
        />
        <div className="min-w-0 flex-1">
          <div className="flex flex-wrap items-center gap-2">
            <span className="text-[13px] font-medium text-fg">{field.label}</span>
            {field.recommended ? <Badge variant="success">recommended</Badge> : null}
            <code className="break-all text-[11px] text-fg-subtle">{field.path}</code>
          </div>
          <p className="mt-1.5 text-xs leading-relaxed text-fg-muted">{field.description}</p>
          {(field.env || field.cli) && (
            <div className="mt-2 flex flex-wrap gap-2">
              {field.env ? (
                <code className="rounded-xs bg-bg-subtle px-1.5 py-0.5 text-[11px] text-fg-subtle">
                  env {field.env}
                </code>
              ) : null}
              {field.cli ? (
                <code className="rounded-xs bg-bg-subtle px-1.5 py-0.5 text-[11px] text-fg-subtle">
                  cli {field.cli}
                </code>
              ) : null}
            </div>
          )}
          {on ? (
            <div className="mt-2.5">
              <FieldControl field={field} value={value} onChange={onChange} />
            </div>
          ) : null}
        </div>
      </div>
    </div>
  );
}

function FieldControl({
  field,
  value,
  onChange,
}: {
  field: ConfigField;
  value: unknown;
  onChange: (v: unknown) => void;
}) {
  if (field.type === "boolean") {
    return (
      <div className="flex items-center gap-2">
        <Switch checked={!!value} onCheckedChange={onChange} aria-label={field.label} />
        <span className="text-[13px] text-fg-muted">{value ? "true" : "false"}</span>
      </div>
    );
  }
  if (field.type === "enum" && field.options) {
    const cur = String(value ?? field.default ?? field.options[0] ?? "");
    return (
      <select
        value={cur}
        onChange={(e) => onChange(e.target.value)}
        className="min-h-10 w-full max-w-md rounded-sm border border-border bg-bg-subtle px-3 text-[13px] text-fg"
      >
        {field.options.map((o) => (
          <option key={o} value={o}>
            {o}
          </option>
        ))}
      </select>
    );
  }
  if (field.type === "string-list") {
    const v = Array.isArray(value) ? value.join(", ") : (value ?? "");
    return (
      <input
        type="text"
        value={String(v)}
        onChange={(e) =>
          onChange(
            e.target.value
              .split(",")
              .map((x) => x.trim())
              .filter(Boolean),
          )
        }
        className="min-h-10 w-full max-w-md rounded-sm border border-border bg-bg-subtle px-3 text-[13px] text-fg"
        placeholder="comma, separated, values"
      />
    );
  }
  return (
    <input
      type={field.type === "number" ? "number" : "text"}
      value={value == null ? "" : String(value)}
      onChange={(e) =>
        onChange(
          field.type === "number"
            ? e.target.value === ""
              ? ""
              : Number(e.target.value)
            : e.target.value,
        )
      }
      className="min-h-10 w-full max-w-md rounded-sm border border-border bg-bg-subtle px-3 text-[13px] text-fg"
    />
  );
}

function PreviewView({ patch, product }: { patch: GenerateResult; product: ProductId }) {
  const panels: {
    title: string;
    subtitle: string;
    content: string;
    file: string;
    mime: string;
  }[] = [
    {
      title: patch.filename,
      subtitle: product === "opencode" ? "opencode.json" : "config patch",
      content: patch.config,
      file: patch.filename,
      mime: patch.mime,
    },
    {
      title: "env",
      subtitle: "shell exports",
      content: patch.env,
      file: "env.sh",
      mime: "text/plain",
    },
    {
      title: "cli",
      subtitle: "launch command",
      content: patch.cli,
      file: "launch.sh",
      mime: "text/plain",
    },
    {
      title: "markdown",
      subtitle: "full reference",
      content: patch.markdown,
      file: `${product}-config-reference.md`,
      mime: "text/markdown",
    },
  ];

  return (
    <div className="grid min-w-0 gap-4 lg:grid-cols-2">
      {panels.map((p) => (
        <PreviewPanel key={p.title} {...p} />
      ))}
    </div>
  );
}

function PreviewPanel({
  title,
  subtitle,
  content,
  file,
  mime,
}: {
  title: string;
  subtitle: string;
  content: string;
  file: string;
  mime: string;
}) {
  const [copied, setCopied] = useState(false);
  const onCopy = useCallback(async () => {
    try {
      await copyText(content);
      setCopied(true);
      toast.success("Copied");
      setTimeout(() => setCopied(false), 1200);
    } catch {
      toast.error("Copy failed");
    }
  }, [content]);

  return (
    <div className="surface flex min-h-80 min-w-0 flex-col overflow-hidden rounded-xl">
      <header className="flex flex-wrap items-center justify-between gap-2 border-b border-border px-4 py-3">
        <div>
          <strong className="text-[13px] font-medium text-fg">{title}</strong>
          <p className="text-[11px] text-fg-muted">{subtitle}</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={onCopy}>
            {copied ? <Check className="size-3.5" /> : <Copy className="size-3.5" />}
            {copied ? "Copied" : "Copy"}
          </Button>
          <Button
            size="sm"
            onClick={() => {
              downloadBlob(file, content, mime);
              toast.success(`Downloaded ${file}`);
            }}
          >
            <Download className="size-3.5" />
            Download
          </Button>
        </div>
      </header>
      <pre className="scrollbar-thin flex-1 overflow-auto bg-bg p-4 text-xs leading-relaxed break-all whitespace-pre-wrap text-fg-muted">
        {content}
      </pre>
    </div>
  );
}

function ReferenceView({
  schema,
  onDownloadMd,
}: {
  schema: ProductSchema;
  onDownloadMd: () => void;
}) {
  const flags = schema.cliFlags || [];
  const envs = schema.envVars || [];
  const subs = schema.subcommands || [];

  return (
    <div>
      <div className="mb-4 flex flex-wrap items-center justify-between gap-3">
        <p className="text-[13px] text-fg-muted">
          Reference · <code className="text-fg">{schema.configPath}</code>
        </p>
        <Button variant="secondary" size="sm" onClick={onDownloadMd}>
          <Download className="size-3.5" />
          Download .md
        </Button>
      </div>

      <article className="surface space-y-8 rounded-xl p-4 sm:p-8">
        {subs.length > 0 ? (
          <section>
            <h2 className="mb-3 text-base font-semibold">Subcommands</h2>
            <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
              {subs.map((c) => (
                <div
                  key={c.cmd}
                  className="rounded-md border border-border bg-bg px-3 py-2"
                >
                  <code className="text-[12px] text-fg">{c.cmd}</code>
                  <p className="mt-1 text-xs text-fg-muted">{c.desc}</p>
                </div>
              ))}
            </div>
          </section>
        ) : null}

        {flags.length > 0 ? (
          <section>
            <h2 className="mb-3 text-base font-semibold">Flags</h2>
            <div className="scrollbar-thin overflow-x-auto">
              <table className="w-full min-w-[480px] border-collapse text-left text-[13px]">
                <thead>
                  <tr className="border-b border-border text-fg-muted">
                    <th className="py-2 pr-4 font-medium">Flag</th>
                    <th className="py-2 pr-4 font-medium">Cat</th>
                    <th className="py-2 font-medium">Desc</th>
                  </tr>
                </thead>
                <tbody>
                  {flags.map((f) => (
                    <tr
                      key={f.flag}
                      className="border-b border-border/60 text-fg-muted align-top"
                    >
                      <td className="py-2 pr-4">
                        <code className="text-[12px] text-fg">{f.flag}</code>
                      </td>
                      <td className="py-2 pr-4 whitespace-nowrap">{f.category}</td>
                      <td className="py-2">{f.description}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
        ) : null}

        {envs.length > 0 ? (
          <section>
            <h2 className="mb-3 text-base font-semibold">Environment</h2>
            <div className="scrollbar-thin overflow-x-auto">
              <table className="w-full min-w-[480px] border-collapse text-left text-[13px]">
                <thead>
                  <tr className="border-b border-border text-fg-muted">
                    <th className="py-2 pr-4 font-medium">Var</th>
                    <th className="py-2 pr-4 font-medium">Cat</th>
                    <th className="py-2 font-medium">Desc</th>
                  </tr>
                </thead>
                <tbody>
                  {envs.map((e) => (
                    <tr
                      key={e.name}
                      className="border-b border-border/60 text-fg-muted align-top"
                    >
                      <td className="py-2 pr-4">
                        <code className="text-[12px] text-fg">{e.name}</code>
                      </td>
                      <td className="py-2 pr-4 whitespace-nowrap">{e.category}</td>
                      <td className="py-2">{e.description}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </section>
        ) : null}

        {productLinks(schema.product)}
      </article>
    </div>
  );
}

function productLinks(product: string) {
  if (product === "grok") {
    return (
      <section>
        <h2 className="mb-3 text-base font-semibold">xbgst stack</h2>
        <ul className="space-y-2 text-[13px] text-fg-muted">
          <li>
            <a
              className="inline-flex items-center gap-1 text-fg underline decoration-border-strong underline-offset-2"
              href="https://github.com/VeigaPunk/grok-marketplace"
              target="_blank"
              rel="noopener noreferrer"
            >
              VeigaPunk/grok-marketplace <ExternalLink className="size-3" />
            </a>
            — xbgst-stack plugin host
          </li>
          <li>
            <a
              className="inline-flex items-center gap-1 text-fg underline decoration-border-strong underline-offset-2"
              href="https://github.com/VeigaPunk/grok-build-livepatch"
              target="_blank"
              rel="noopener noreferrer"
            >
              VeigaPunk/grok-build-livepatch <ExternalLink className="size-3" />
            </a>
            — hard-ban general-purpose / explore
          </li>
          <li>
            <a
              className="inline-flex items-center gap-1 text-fg underline decoration-border-strong underline-offset-2"
              href="/recommended-grok-config.toml"
            >
              recommended-grok-config.toml <Download className="size-3" />
            </a>
          </li>
        </ul>
      </section>
    );
  }
  if (product === "codex") {
    return (
      <section>
        <h2 className="mb-3 text-base font-semibold">Sekhmet</h2>
        <p className="text-[13px] leading-relaxed text-fg-muted">
          Sekhmet (
          <a
            href="https://github.com/VeigaPunk/xbrd-spark"
            target="_blank"
            rel="noopener noreferrer"
            className="text-fg underline decoration-border-strong underline-offset-2"
          >
            xbrd-spark
          </a>
          ) is the L3 swarm substrate that runs against{" "}
          <a
            href="https://github.com/VeigaPunk/codex-titanium"
            target="_blank"
            rel="noopener noreferrer"
            className="text-fg underline decoration-border-strong underline-offset-2"
          >
            Codex Titanium
          </a>
          . Use the Titanium preset so concurrent swarms match agent wiring.
        </p>
      </section>
    );
  }
  return null;
}

/** Client-only loader wrapper when schema is fetched async */
export function ConfigAppLoader({ product }: { product: ProductId }) {
  const [schema, setSchema] = useState<ProductSchema | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const res = await fetch(`/schemas/${product}.json`);
        if (!res.ok) throw new Error(`Failed to load schema (${res.status})`);
        const data = (await res.json()) as ProductSchema;
        if (!cancelled) setSchema(data);
      } catch (e) {
        if (!cancelled) setError(e instanceof Error ? e.message : String(e));
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [product]);

  if (error) {
    return (
      <div className="mx-auto max-w-lg px-6 py-24 text-center">
        <h1 className="text-lg font-semibold">Could not load schema</h1>
        <p className="mt-2 text-sm text-fg-muted">{error}</p>
        <Link to="/" className="mt-6 inline-block text-sm underline">
          Back to hub
        </Link>
      </div>
    );
  }
  if (!schema) {
    return (
      <div className="mx-auto max-w-lg px-6 py-24 text-center text-fg-muted">
        Loading {product} schema…
      </div>
    );
  }
  return <ConfigApp product={product} schema={schema} />;
}
