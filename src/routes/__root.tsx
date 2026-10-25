import type { ReactNode } from "react";
import { Outlet, createRootRoute, HeadContent, Scripts } from "@tanstack/react-router";
import { Toaster } from "sonner";
import { CreatedWithGrokBanner } from "@/components/created-with-grok-banner";
import appCss from "@/styles.css?url";

const SITE_URL = "https://veigapunk.github.io/grok-build-config-builder/";
const SITE_TITLE = "Agent Config Builders — Titanium";
const SITE_DESCRIPTION =
  "Interactive config builders for Grok Build, Codex Titanium, and OpenCode. Toggle settings, apply Titanium · xbgst presets, download patches.";

export const Route = createRootRoute({
  head: () => ({
    meta: [
      { charSet: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      {
        title: SITE_TITLE,
      },
      {
        name: "description",
        content: SITE_DESCRIPTION,
      },
      { name: "theme-color", content: "#0a0a0b" },
      { property: "og:title", content: SITE_TITLE },
      { property: "og:description", content: SITE_DESCRIPTION },
      { property: "og:type", content: "website" },
      { property: "og:url", content: SITE_URL },
      { property: "og:image", content: `${SITE_URL}og.png` },
      { name: "twitter:card", content: "summary_large_image" },
      { name: "twitter:title", content: SITE_TITLE },
      { name: "twitter:description", content: SITE_DESCRIPTION },
      { name: "twitter:image", content: `${SITE_URL}og.png` },
    ],
    links: [
      { rel: "canonical", href: SITE_URL },
      { rel: "preconnect", href: "https://cdn.jsdelivr.net", crossOrigin: "anonymous" },
      { rel: "stylesheet", href: appCss },
      { rel: "icon", href: "/favicon.svg", type: "image/svg+xml" },
    ],
  }),
  component: RootComponent,
});

function RootComponent() {
  return (
    <RootDocument>
      <CreatedWithGrokBanner />
      <Outlet />
      <Toaster
        theme="dark"
        position="bottom-right"
        toastOptions={{
          style: {
            background: "#1a1a1e",
            border: "1px solid color-mix(in oklab, #f4f4f5 12%, transparent)",
            color: "#f4f4f5",
            fontFamily: "inherit",
          },
        }}
      />
    </RootDocument>
  );
}

function RootDocument({ children }: Readonly<{ children: ReactNode }>) {
  return (
    <html lang="en">
      <head>
        <HeadContent />
      </head>
      <body className="min-h-dvh bg-bg text-fg antialiased">
        {children}
        <Scripts />
      </body>
    </html>
  );
}
