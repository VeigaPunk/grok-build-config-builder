import { createFileRoute, Link } from "@tanstack/react-router";
import { ArrowUpRight, Boxes, Cpu, Sparkles, Terminal } from "lucide-react";
import { PRODUCTS } from "@/lib/schema-types";
import { Badge } from "@/components/ui/badge";

export const Route = createFileRoute("/")({
  component: HubPage,
  head: () => ({
    meta: [{ title: "Agent Config Builders — Titanium" }],
  }),
});

const icons = {
  grok: Sparkles,
  codex: Terminal,
  opencode: Cpu,
} as const;

function HubPage() {
  return (
    <div className="relative min-h-dvh overflow-hidden">
      <div
        aria-hidden
        className="pointer-events-none absolute inset-0 opacity-[0.35]"
        style={{
          background:
            "radial-gradient(ellipse 80% 50% at 50% -20%, color-mix(in oklab, #c8ccd4 12%, transparent), transparent 70%)",
        }}
      />

      <main className="relative mx-auto w-full max-w-5xl px-4 py-12 sm:px-6 sm:py-16 md:py-20">
        <div className="flex flex-wrap items-center gap-2">
          <Badge>Titanium profiles</Badge>
          <Badge variant="subtle">multi-product</Badge>
          <Badge variant="subtle">Vercel-ready</Badge>
        </div>

        <h1 className="mt-5 max-w-2xl text-balance text-[clamp(2rem,5vw,3rem)] font-semibold tracking-[-0.03em] leading-[1.1] text-fg">
          Agent config builders
        </h1>
        <p className="mt-4 max-w-xl text-pretty text-[15px] leading-relaxed text-fg-muted">
          Toggle every setting. Download patches. Titanium profiles are the
          sensible defaults — Grok ships{" "}
          <strong className="font-medium text-fg">xbgst + livepatch ban</strong>,
          Codex pairs with Sekhmet, OpenCode gets a wild Titanium build.
        </p>

        <div className="mt-10 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {PRODUCTS.map((p) => {
            const Icon = icons[p.id];
            return (
              <Link
                key={p.id}
                to="/$product"
                params={{ product: p.id }}
                className="group surface relative flex flex-col rounded-xl p-5 transition-[border-color,background-color,transform] duration-200 hover:border-border-strong hover:bg-bg-subtle/40 active:scale-[0.99]"
              >
                <div className="flex items-start justify-between gap-3">
                  <div className="flex size-10 items-center justify-center rounded-md border border-border bg-bg-subtle text-fg-muted transition-colors group-hover:text-fg">
                    <Icon className="size-5" strokeWidth={1.75} />
                  </div>
                  <ArrowUpRight className="size-4 text-fg-subtle transition-transform duration-200 group-hover:translate-x-0.5 group-hover:-translate-y-0.5 group-hover:text-fg" />
                </div>
                <h2 className="mt-4 text-base font-semibold tracking-tight text-fg">
                  {p.title}
                </h2>
                <p className="mt-1.5 flex-1 text-[13px] leading-relaxed text-fg-muted">
                  {p.blurb}
                </p>
                <p className="mt-4 font-mono text-[11px] text-fg-subtle">
                  {p.configPath}
                </p>
                {p.links?.length ? (
                  <div className="mt-3 flex flex-wrap gap-2">
                    {p.links.map((l) => (
                      <span
                        key={l.href}
                        className="rounded-full border border-border bg-bg px-2 py-0.5 text-[10px] text-fg-subtle"
                      >
                        {l.label}
                      </span>
                    ))}
                  </div>
                ) : null}
              </Link>
            );
          })}
        </div>

        <section className="surface mt-10 rounded-xl p-5 sm:p-6">
          <div className="flex items-start gap-3">
            <div className="flex size-9 shrink-0 items-center justify-center rounded-md border border-border bg-bg text-fg-muted">
              <Boxes className="size-4" strokeWidth={1.75} />
            </div>
            <div>
              <h3 className="text-sm font-semibold text-fg">Sekhmet · swarm substrate</h3>
              <p className="mt-1.5 max-w-2xl text-[13px] leading-relaxed text-fg-muted">
                <a
                  href="https://github.com/VeigaPunk/xbrd-spark"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-fg underline decoration-border-strong underline-offset-2"
                >
                  VeigaPunk/xbrd-spark
                </a>{" "}
                is the pure L3 swarm substrate that runs against{" "}
                <a
                  href="https://github.com/VeigaPunk/codex-titanium"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-fg underline decoration-border-strong underline-offset-2"
                >
                  Codex Titanium
                </a>
                . Use the Titanium preset so concurrent{" "}
                <code className="rounded-xs bg-bg-subtle px-1 py-0.5 text-[11px]">
                  sekhmet swarm -j N
                </code>{" "}
                (cap 64) matches agent wiring.
              </p>
            </div>
          </div>
        </section>

        <div className="mt-10 flex flex-wrap items-center gap-x-4 gap-y-2 text-[12px] text-fg-subtle">
          <span>Default Grok preset: Titanium · xbgst + livepatch ban</span>
          <span className="hidden sm:inline" aria-hidden>
            ·
          </span>
          <a
            href="https://github.com/VeigaPunk/grok-build-config-builder"
            target="_blank"
            rel="noopener noreferrer"
            className="underline decoration-border underline-offset-2 hover:text-fg-muted"
          >
            GitHub
          </a>
          <span aria-hidden>·</span>
          <a
            href="/recommended-grok-config.toml"
            className="underline decoration-border underline-offset-2 hover:text-fg-muted"
          >
            recommended-grok-config.toml
          </a>
        </div>
      </main>
    </div>
  );
}
