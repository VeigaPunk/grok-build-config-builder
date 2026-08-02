import { Outlet, createRootRoute, HeadContent, Scripts } from "@tanstack/react-router";
import { Toaster } from "sonner";
import appCss from "../styles.css?url";

export const Route = createRootRoute({
  head: () => ({
    meta: [
      { charSet: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      {
        title: "Grok Build Config Builder — flags, config.toml, patches",
      },
      {
        name: "description",
        content:
          "Interactive Grok Build CLI config reference. Toggle every setting and download a custom config.toml patch.",
      },
    ],
    links: [{ rel: "stylesheet", href: appCss }],
  }),
  component: RootComponent,
});

function RootComponent() {
  return (
    <html lang="en">
      <head>
        <HeadContent />
      </head>
      <body>
        <Outlet />
        <Toaster
          theme="dark"
          position="bottom-right"
          toastOptions={{
            className: "border border-border bg-bg-elevated text-fg",
          }}
        />
        <Scripts />
      </body>
    </html>
  );
}
