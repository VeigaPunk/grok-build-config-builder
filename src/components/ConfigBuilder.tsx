import { useCallback, useMemo, useState } from "react";
import {
  BookOpen,
  Check,
  ChevronDown,
  Copy,
  Download,
  FileCode2,
  RotateCcw,
  Search,
  Settings2,
  Sparkles,
  Terminal,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import {
  CLI_FLAGS,
  ENV_VARS,
  FIELDS,
  GROUPS,
  PRESETS,
  SUBCOMMANDS,
  defaultEnabledIds,
  defaultValues,
  type ConfigField,
} from "@/lib/config/schema";
import {
  generateCliSnippet,
  generateEnv,
  generateToml,
} from "@/lib/config/generate";
import { generateFullMarkdown } from "@/lib/config/markdown";
import { cn } from "@/lib/utils";
import { toast } from "sonner";

type Tab = "builder" | "preview" | "reference";

function downloadText(filename: string, content: string, mime = "text/plain") {
  const blob = new Blob([content], { type: mime });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

async function copyText(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    toast.success("Copied to clipboard");
  } catch {
    toast.error("Could not copy");
  }
}

function FieldControl({
  field,
  enabled,
  value,
  onToggle,
  onChange,
}: {
  field: ConfigField;
  enabled: boolean;
  value: unknown;
  onToggle: (id: string, on: boolean) => void;
  onChange: (id: string, value: unknown) => void;
}) {
  return (
    <div
      className={cn(
        "rounded-[var(--radius-lg)] border border-border p-3 sm:p-4 transition-colors min-w-0",
        enabled ? "bg-bg-elevated" : "bg-bg/40 opacity-80",
      )}
    >
      <div className="flex items-start gap-3 min-w-0">
        <Switch
          checked={enabled}
          onCheckedChange={(on) => onToggle(field.id, on)}
          aria-label={`Include ${field.label}`}
          className="mt-0.5 shrink-0"
        />
        <div className="min-w-0 flex-1 space-y-2">
          <div className="flex flex-wrap items-center gap-2">
            <span className="text-sm font-medium text-fg">{field.label}</span>
            {field.recommended && <Badge variant="success">recommended</Badge>}
            <code className="mono break-all text-xs text-fg-subtle">
              {field.path}
            </code>
          </div>
          <p className="text-sm text-fg-muted leading-snug">{field.description}</p>
          {(field.env || field.cli) && (
            <div className="flex flex-wrap gap-2 text-xs text-fg-subtle">
              {field.env && (
                <span className="mono break-all rounded-[var(--radius-xs)] bg-bg-subtle px-1.5 py-0.5">
                  env {field.env}
                </span>
              )}
              {field.cli && (
                <span className="mono break-all rounded-[var(--radius-xs)] bg-bg-subtle px-1.5 py-0.5">
                  cli {field.cli}
                </span>
              )}
            </div>
          )}

          {enabled && (
            <div className="pt-1 min-w-0">
              {field.type === "boolean" && (
                <div className="flex items-center gap-3">
                  <Switch
                    checked={Boolean(value)}
                    onCheckedChange={(on) => onChange(field.id, on)}
                  />
                  <span className="text-sm text-fg-muted">
                    {value ? "true" : "false"}
                  </span>
                </div>
              )}
              {(field.type === "string" || field.type === "number") && (
                <Input
                  type={field.type === "number" ? "number" : "text"}
                  value={value === undefined || value === null ? "" : String(value)}
                  onChange={(e) =>
                    onChange(
                      field.id,
                      field.type === "number"
                        ? e.target.value === ""
                          ? ""
                          : Number(e.target.value)
                        : e.target.value,
                    )
                  }
                  className="w-full max-w-md mono text-sm"
                  placeholder={
                    field.default !== undefined
                      ? String(field.default)
                      : field.label
                  }
                />
              )}
              {field.type === "enum" && field.options && (
                <select
                  className="h-10 w-full max-w-md rounded-[var(--radius-sm)] border border-border bg-bg-subtle px-3 text-sm text-fg"
                  value={String(value ?? field.default ?? field.options[0])}
                  onChange={(e) => onChange(field.id, e.target.value)}
                >
                  {field.options.map((opt) => (
                    <option key={opt} value={opt}>
                      {opt}
                    </option>
                  ))}
                </select>
              )}
              {field.type === "string-list" && (
                <Input
                  value={
                    Array.isArray(value)
                      ? value.join(", ")
                      : value === undefined || value === null
                        ? ""
                        : String(value)
                  }
                  onChange={(e) =>
                    onChange(
                      field.id,
                      e.target.value
                        .split(",")
                        .map((x) => x.trim())
                        .filter(Boolean),
                    )
                  }
                  className="w-full max-w-lg mono text-sm"
                  placeholder="comma,separated,values"
                />
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function GroupSection({
  title,
  description,
  children,
  defaultOpen = true,
}: {
  title: string;
  description: string;
  children: React.ReactNode;
  defaultOpen?: boolean;
}) {
  const [open, setOpen] = useState(defaultOpen);
  return (
    <section className="panel overflow-hidden min-w-0">
      <button
        type="button"
        className="flex w-full items-center justify-between gap-3 px-4 py-4 text-left hover:bg-bg-subtle/40 sm:px-5"
        onClick={() => setOpen((o) => !o)}
      >
        <div className="min-w-0">
          <h2 className="text-base font-semibold tracking-tight text-fg">{title}</h2>
          <p className="mt-0.5 text-sm text-fg-muted">{description}</p>
        </div>
        <ChevronDown
          className={cn(
            "h-5 w-5 shrink-0 text-fg-muted transition-transform duration-200",
            open && "rotate-180",
          )}
        />
      </button>
      {open && (
        <div className="space-y-3 border-t border-border px-4 py-4 sm:px-5">
          {children}
        </div>
      )}
    </section>
  );
}

export function ConfigBuilder() {
  const [tab, setTab] = useState<Tab>("builder");
  const [enabled, setEnabled] = useState<Set<string>>(
    () => new Set(defaultEnabledIds()),
  );
  const [values, setValues] = useState<Record<string, unknown>>(() =>
    defaultValues(),
  );
  const [query, setQuery] = useState("");
  const [activeGroup, setActiveGroup] = useState<string | "all">("all");
  const [sidebarOpen, setSidebarOpen] = useState(false);

  const state = useMemo(() => ({ enabled, values }), [enabled, values]);
  const toml = useMemo(() => generateToml(state), [state]);
  const envFile = useMemo(() => generateEnv(state), [state]);
  const cliSnippet = useMemo(() => generateCliSnippet(state), [state]);
  const markdown = useMemo(() => generateFullMarkdown(), []);

  const onToggle = useCallback((id: string, on: boolean) => {
    setEnabled((prev) => {
      const next = new Set(prev);
      if (on) next.add(id);
      else next.delete(id);
      return next;
    });
  }, []);

  const onChange = useCallback((id: string, value: unknown) => {
    setValues((prev) => ({ ...prev, [id]: value }));
  }, []);

  const applyPreset = (presetId: string) => {
    const preset = PRESETS.find((p) => p.id === presetId);
    if (!preset) return;
    setEnabled(new Set(preset.enabled));
    setValues((prev) => ({ ...prev, ...preset.values }));
    toast.success(`Applied preset: ${preset.name}`);
  };

  const resetAll = () => {
    setEnabled(new Set(defaultEnabledIds()));
    setValues(defaultValues());
    toast.message("Reset to privacy-first defaults");
  };

  const enableAll = () => {
    setEnabled(new Set(FIELDS.map((f) => f.id)));
    toast.message("All settings included in patch");
  };

  const disableAll = () => {
    setEnabled(new Set());
    toast.message("Cleared all toggles");
  };

  const filteredGroups = useMemo(() => {
    const q = query.trim().toLowerCase();
    return GROUPS.map((g) => {
      let fields = FIELDS.filter((f) => f.group === g.id);
      if (activeGroup !== "all" && g.id !== activeGroup) fields = [];
      if (q) {
        fields = fields.filter(
          (f) =>
            f.label.toLowerCase().includes(q) ||
            f.path.toLowerCase().includes(q) ||
            f.description.toLowerCase().includes(q) ||
            (f.env?.toLowerCase().includes(q) ?? false),
        );
      }
      return { group: g, fields };
    }).filter((x) => x.fields.length > 0);
  }, [query, activeGroup]);

  const enabledCount = enabled.size;

  return (
    <div className="min-h-dvh overflow-x-hidden bg-bg text-fg">
      <header className="sticky top-0 z-40 border-b border-border bg-bg/90 backdrop-blur-md">
        <div className="mx-auto flex w-full max-w-7xl flex-col gap-4 px-4 py-4 sm:px-6">
          <div className="flex flex-wrap items-start justify-between gap-4">
            <div className="min-w-0">
              <div className="flex items-center gap-2 text-xs font-medium uppercase tracking-wider text-fg-subtle">
                <Settings2 className="h-3.5 w-3.5 shrink-0" />
                Grok Build
              </div>
              <h1 className="mt-1 text-2xl font-semibold tracking-tight sm:text-3xl">
                Config reference & patch builder
              </h1>
              <p className="mt-1 max-w-2xl text-sm text-fg-muted">
                Toggle every setting, preview a custom{" "}
                <code className="mono text-fg">config.toml</code> patch, and
                download it for{" "}
                <code className="mono text-fg">~/.grok/config.toml</code>. Full
                markdown reference included.
              </p>
            </div>
            <div className="flex flex-wrap gap-2">
              <Button
                variant="secondary"
                size="sm"
                onClick={() =>
                  downloadText(
                    "grok-build-cli-config-reference.md",
                    markdown,
                    "text/markdown",
                  )
                }
              >
                <BookOpen className="h-4 w-4" />
                Download .md
              </Button>
              <Button
                size="sm"
                onClick={() =>
                  downloadText("config.toml", toml, "application/toml")
                }
              >
                <Download className="h-4 w-4" />
                Download patch
              </Button>
            </div>
          </div>

          <div className="flex flex-wrap items-center gap-2">
            {(
              [
                ["builder", "Builder", Settings2],
                ["preview", "Preview", FileCode2],
                ["reference", "Reference", BookOpen],
              ] as const
            ).map(([id, label, Icon]) => (
              <button
                key={id}
                type="button"
                onClick={() => setTab(id)}
                className={cn(
                  "inline-flex h-10 items-center gap-2 rounded-full border px-4 text-sm font-medium transition-colors",
                  tab === id
                    ? "border-transparent bg-accent text-accent-fg"
                    : "border-border bg-bg-elevated text-fg-muted hover:text-fg",
                )}
              >
                <Icon className="h-4 w-4" />
                {label}
              </button>
            ))}
            <Badge className="ml-auto hidden sm:inline-flex">
              {enabledCount} setting{enabledCount === 1 ? "" : "s"} in patch
            </Badge>
          </div>
        </div>
      </header>

      <main className="mx-auto w-full max-w-7xl min-w-0 px-4 py-6 sm:px-6 sm:py-8">
        {tab === "builder" && (
          <div className="grid min-w-0 gap-6 lg:grid-cols-[240px_minmax(0,1fr)]">
            {/* Mobile presets strip */}
            <div className="min-w-0 space-y-3 lg:hidden">
              <div className="flex gap-2 overflow-x-auto pb-1 scrollbar-thin">
                {PRESETS.map((p) => (
                  <button
                    key={p.id}
                    type="button"
                    onClick={() => applyPreset(p.id)}
                    className="shrink-0 rounded-full border border-border bg-bg-elevated px-3 py-2 text-xs font-medium text-fg"
                  >
                    {p.name}
                  </button>
                ))}
              </div>
              <div className="flex flex-wrap gap-2">
                <Button variant="outline" size="sm" onClick={enableAll}>
                  Include all
                </Button>
                <Button variant="outline" size="sm" onClick={disableAll}>
                  Clear
                </Button>
                <Button variant="ghost" size="sm" onClick={resetAll}>
                  <RotateCcw className="h-3.5 w-3.5" />
                  Reset
                </Button>
                <Button
                  variant="secondary"
                  size="sm"
                  onClick={() => setSidebarOpen((o) => !o)}
                >
                  Sections
                </Button>
              </div>
              {sidebarOpen && (
                <div className="panel flex flex-wrap gap-2 p-3">
                  <button
                    type="button"
                    onClick={() => setActiveGroup("all")}
                    className={cn(
                      "rounded-full px-3 py-1.5 text-xs",
                      activeGroup === "all"
                        ? "bg-accent text-accent-fg"
                        : "bg-bg-subtle text-fg-muted",
                    )}
                  >
                    All
                  </button>
                  {GROUPS.map((g) => (
                    <button
                      key={g.id}
                      type="button"
                      onClick={() => setActiveGroup(g.id)}
                      className={cn(
                        "rounded-full px-3 py-1.5 text-xs",
                        activeGroup === g.id
                          ? "bg-accent text-accent-fg"
                          : "bg-bg-subtle text-fg-muted",
                      )}
                    >
                      {g.title}
                    </button>
                  ))}
                </div>
              )}
            </div>

            {/* Desktop sidebar */}
            <aside className="hidden min-w-0 space-y-4 lg:block lg:sticky lg:top-36 lg:self-start">
              <div className="panel p-4 space-y-3">
                <div className="text-xs font-medium uppercase tracking-wider text-fg-subtle">
                  Presets
                </div>
                <div className="flex flex-col gap-2">
                  {PRESETS.map((p) => (
                    <button
                      key={p.id}
                      type="button"
                      onClick={() => applyPreset(p.id)}
                      className="rounded-[var(--radius-md)] border border-border bg-bg-subtle px-3 py-2.5 text-left hover:border-border-strong"
                    >
                      <div className="flex items-center gap-2 text-sm font-medium">
                        <Sparkles className="h-3.5 w-3.5 text-fg-muted" />
                        {p.name}
                      </div>
                      <p className="mt-0.5 text-xs text-fg-muted leading-snug">
                        {p.description}
                      </p>
                    </button>
                  ))}
                </div>
              </div>

              <div className="panel p-4 space-y-2">
                <div className="text-xs font-medium uppercase tracking-wider text-fg-subtle">
                  Bulk
                </div>
                <Button variant="outline" size="sm" className="w-full" onClick={enableAll}>
                  Include all
                </Button>
                <Button variant="outline" size="sm" className="w-full" onClick={disableAll}>
                  Clear all
                </Button>
                <Button variant="ghost" size="sm" className="w-full" onClick={resetAll}>
                  <RotateCcw className="h-3.5 w-3.5" />
                  Reset defaults
                </Button>
              </div>

              <div className="panel p-4 space-y-2">
                <div className="text-xs font-medium uppercase tracking-wider text-fg-subtle">
                  Sections
                </div>
                <button
                  type="button"
                  onClick={() => setActiveGroup("all")}
                  className={cn(
                    "w-full rounded-[var(--radius-sm)] px-2 py-1.5 text-left text-sm",
                    activeGroup === "all"
                      ? "bg-bg-subtle text-fg"
                      : "text-fg-muted hover:text-fg",
                  )}
                >
                  All sections
                </button>
                {GROUPS.map((g) => (
                  <button
                    key={g.id}
                    type="button"
                    onClick={() => setActiveGroup(g.id)}
                    className={cn(
                      "w-full rounded-[var(--radius-sm)] px-2 py-1.5 text-left text-sm",
                      activeGroup === g.id
                        ? "bg-bg-subtle text-fg"
                        : "text-fg-muted hover:text-fg",
                    )}
                  >
                    {g.title}
                  </button>
                ))}
              </div>
            </aside>

            <div className="min-w-0 space-y-4">
              <div className="relative min-w-0">
                <Search className="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-fg-subtle" />
                <Input
                  value={query}
                  onChange={(e) => setQuery(e.target.value)}
                  placeholder="Search settings, paths, env vars…"
                  className="pl-10"
                />
              </div>

              {filteredGroups.length === 0 && (
                <div className="panel p-8 text-center text-fg-muted">
                  No settings match your search.
                </div>
              )}

              {filteredGroups.map(({ group, fields }) => (
                <GroupSection
                  key={group.id}
                  title={group.title}
                  description={group.description}
                >
                  {fields.map((field) => (
                    <FieldControl
                      key={field.id}
                      field={field}
                      enabled={enabled.has(field.id)}
                      value={values[field.id]}
                      onToggle={onToggle}
                      onChange={onChange}
                    />
                  ))}
                </GroupSection>
              ))}

              <div className="sticky bottom-4 z-30 flex flex-wrap gap-2 rounded-[var(--radius-xl)] border border-border bg-bg-elevated/95 p-3 shadow-[var(--shadow-soft)] backdrop-blur lg:hidden">
                <Button
                  className="min-h-11 flex-1"
                  onClick={() =>
                    downloadText("config.toml", toml, "application/toml")
                  }
                >
                  <Download className="h-4 w-4" />
                  Download patch
                </Button>
                <Button
                  variant="secondary"
                  className="min-h-11"
                  onClick={() => setTab("preview")}
                >
                  View preview
                </Button>
              </div>
            </div>
          </div>
        )}

        {tab === "preview" && (
          <div className="grid min-w-0 gap-4 lg:grid-cols-2">
            <PreviewPanel
              title="config.toml patch"
              subtitle="Merge into ~/.grok/config.toml"
              content={toml}
              filename="config.toml"
              mime="application/toml"
              icon={<FileCode2 className="h-4 w-4" />}
            />
            <PreviewPanel
              title="Environment exports"
              subtitle="Shell / CI overrides"
              content={envFile}
              filename="grok-env.sh"
              mime="text/x-shellscript"
              icon={<Terminal className="h-4 w-4" />}
            />
            <PreviewPanel
              title="CLI launch snippet"
              subtitle="Flags derived from your selection"
              content={cliSnippet}
              filename="grok-launch.sh"
              mime="text/x-shellscript"
              icon={<Terminal className="h-4 w-4" />}
            />
            <PreviewPanel
              title="Full markdown reference"
              subtitle="Complete docs compiled from this app"
              content={markdown}
              filename="grok-build-cli-config-reference.md"
              mime="text/markdown"
              icon={<BookOpen className="h-4 w-4" />}
            />
          </div>
        )}

        {tab === "reference" && (
          <div className="min-w-0 space-y-6">
            <div className="flex flex-wrap items-center justify-between gap-3">
              <p className="text-sm text-fg-muted">
                Full reference also available as a downloadable{" "}
                <code className="mono">.md</code> file.
              </p>
              <Button
                variant="secondary"
                size="sm"
                onClick={() =>
                  downloadText(
                    "grok-build-cli-config-reference.md",
                    markdown,
                    "text/markdown",
                  )
                }
              >
                <Download className="h-4 w-4" />
                Download markdown
              </Button>
            </div>

            <article className="panel max-w-none space-y-8 overflow-hidden p-4 sm:p-8">
              <section className="space-y-3">
                <h2 className="text-xl font-semibold tracking-tight">
                  Install
                </h2>
                <pre className="overflow-x-auto rounded-[var(--radius-md)] bg-bg p-4 text-sm mono text-fg-muted">
{`curl -fsSL https://x.ai/cli/install.sh | bash
grok version
grok update --stable`}
                </pre>
              </section>

              <section className="space-y-3">
                <h2 className="text-xl font-semibold tracking-tight">
                  Config layers
                </h2>
                <div className="overflow-x-auto">
                  <table className="w-full min-w-[480px] text-left text-sm">
                    <thead>
                      <tr className="border-b border-border text-fg-muted">
                        <th className="py-2 pr-4 font-medium">Priority</th>
                        <th className="py-2 pr-4 font-medium">Source</th>
                        <th className="py-2 font-medium">Purpose</th>
                      </tr>
                    </thead>
                    <tbody className="text-fg-muted">
                      <tr className="border-b border-border/60">
                        <td className="py-2 pr-4">1</td>
                        <td className="py-2 pr-4 mono text-xs">/etc/grok/managed_config.toml</td>
                        <td className="py-2">System managed</td>
                      </tr>
                      <tr className="border-b border-border/60">
                        <td className="py-2 pr-4">2</td>
                        <td className="py-2 pr-4 mono text-xs">~/.grok/managed_config.toml</td>
                        <td className="py-2">User managed</td>
                      </tr>
                      <tr className="border-b border-border/60">
                        <td className="py-2 pr-4">3</td>
                        <td className="py-2 pr-4 mono text-xs">~/.grok/config.toml</td>
                        <td className="py-2">User preferences</td>
                      </tr>
                      <tr className="border-b border-border/60">
                        <td className="py-2 pr-4">4</td>
                        <td className="py-2 pr-4 mono text-xs">~/.grok/requirements.toml</td>
                        <td className="py-2">User pins</td>
                      </tr>
                      <tr>
                        <td className="py-2 pr-4">5</td>
                        <td className="py-2 pr-4 mono text-xs">/etc/grok/requirements.toml</td>
                        <td className="py-2">System pins (highest)</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </section>

              <section className="space-y-3">
                <h2 className="text-xl font-semibold tracking-tight">
                  Subcommands
                </h2>
                <div className="grid gap-2 sm:grid-cols-2">
                  {SUBCOMMANDS.map((s) => (
                    <div
                      key={s.cmd}
                      className="min-w-0 rounded-[var(--radius-md)] border border-border bg-bg px-3 py-2"
                    >
                      <code className="mono break-all text-xs text-fg">{s.cmd}</code>
                      <p className="mt-1 text-xs text-fg-muted">{s.desc}</p>
                    </div>
                  ))}
                </div>
              </section>

              <section className="space-y-3">
                <h2 className="text-xl font-semibold tracking-tight">
                  Launch flags
                </h2>
                <div className="overflow-x-auto">
                  <table className="w-full min-w-[560px] text-left text-sm">
                    <thead>
                      <tr className="border-b border-border text-fg-muted">
                        <th className="py-2 pr-4 font-medium">Flag</th>
                        <th className="py-2 pr-4 font-medium">Category</th>
                        <th className="py-2 font-medium">Description</th>
                      </tr>
                    </thead>
                    <tbody>
                      {CLI_FLAGS.map((f) => (
                        <tr key={f.flag} className="border-b border-border/60">
                          <td className="py-2 pr-4 mono text-xs text-fg whitespace-nowrap">
                            {f.flag}
                          </td>
                          <td className="py-2 pr-4 text-fg-muted">{f.category}</td>
                          <td className="py-2 text-fg-muted">{f.description}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </section>

              <section className="space-y-3">
                <h2 className="text-xl font-semibold tracking-tight">
                  Environment variables
                </h2>
                <div className="overflow-x-auto">
                  <table className="w-full min-w-[560px] text-left text-sm">
                    <thead>
                      <tr className="border-b border-border text-fg-muted">
                        <th className="py-2 pr-4 font-medium">Variable</th>
                        <th className="py-2 pr-4 font-medium">Category</th>
                        <th className="py-2 font-medium">Description</th>
                      </tr>
                    </thead>
                    <tbody>
                      {ENV_VARS.map((e) => (
                        <tr key={e.name} className="border-b border-border/60">
                          <td className="py-2 pr-4 mono text-xs text-fg whitespace-nowrap">
                            {e.name}
                          </td>
                          <td className="py-2 pr-4 text-fg-muted">{e.category}</td>
                          <td className="py-2 text-fg-muted">{e.description}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </section>

              <section className="space-y-3">
                <h2 className="text-xl font-semibold tracking-tight">
                  All config.toml keys
                </h2>
                {GROUPS.map((g) => (
                  <div key={g.id} className="space-y-2">
                    <h3 className="text-base font-medium text-fg">{g.title}</h3>
                    <div className="space-y-2">
                      {FIELDS.filter((f) => f.group === g.id).map((f) => (
                        <div
                          key={f.id}
                          className="min-w-0 rounded-[var(--radius-md)] border border-border bg-bg px-3 py-2"
                        >
                          <div className="flex flex-wrap items-center gap-2">
                            <code className="mono break-all text-xs text-fg">
                              {f.path}
                            </code>
                            <Badge>{f.type}</Badge>
                            {f.recommended && (
                              <Badge variant="success">recommended</Badge>
                            )}
                          </div>
                          <p className="mt-1 text-sm text-fg-muted">
                            {f.description}
                          </p>
                        </div>
                      ))}
                    </div>
                  </div>
                ))}
              </section>
            </article>
          </div>
        )}
      </main>

      <footer className="border-t border-border py-8 text-center text-xs text-fg-subtle">
        Grok Build config builder · not affiliated as an official xAI product ·
        verify against docs.x.ai for your installed version
      </footer>
    </div>
  );
}

function PreviewPanel({
  title,
  subtitle,
  content,
  filename,
  mime,
  icon,
}: {
  title: string;
  subtitle: string;
  content: string;
  filename: string;
  mime: string;
  icon: React.ReactNode;
}) {
  const [copied, setCopied] = useState(false);

  return (
    <div className="panel flex min-h-[320px] min-w-0 flex-col overflow-hidden">
      <div className="flex flex-wrap items-center justify-between gap-2 border-b border-border px-4 py-3">
        <div className="min-w-0">
          <div className="flex items-center gap-2 text-sm font-medium text-fg">
            {icon}
            {title}
          </div>
          <p className="text-xs text-fg-muted">{subtitle}</p>
        </div>
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={async () => {
              await copyText(content);
              setCopied(true);
              setTimeout(() => setCopied(false), 1500);
            }}
          >
            {copied ? (
              <Check className="h-3.5 w-3.5" />
            ) : (
              <Copy className="h-3.5 w-3.5" />
            )}
            Copy
          </Button>
          <Button
            size="sm"
            onClick={() => downloadText(filename, content, mime)}
          >
            <Download className="h-3.5 w-3.5" />
            Download
          </Button>
        </div>
      </div>
      <pre className="scrollbar-thin flex-1 overflow-auto bg-bg p-4 text-xs leading-relaxed mono text-fg-muted whitespace-pre-wrap break-all">
        {content}
      </pre>
    </div>
  );
}
