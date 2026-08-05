import { createFileRoute, notFound } from "@tanstack/react-router";
import { ConfigApp } from "@/components/config-builder/config-app";
import { getSchema } from "@/lib/schemas";
import { isProductId } from "@/lib/schema-types";

export const Route = createFileRoute("/$product")({
  loader: ({ params }) => {
    if (!isProductId(params.product)) throw notFound();
    return { product: params.product, schema: getSchema(params.product) };
  },
  head: ({ loaderData }) => {
    const title = loaderData?.schema.productTitle ?? loaderData?.product ?? "Config";
    return {
      meta: [
        {
          title: `${title} Config Builder`,
        },
        {
          name: "description",
          content:
            loaderData?.schema.productTagline ||
            loaderData?.schema.product_tagline ||
            "Titanium config builder",
        },
      ],
    };
  },
  component: ProductPage,
  notFoundComponent: () => (
    <div className="mx-auto max-w-md px-6 py-24 text-center">
      <h1 className="text-lg font-semibold">Unknown product</h1>
      <p className="mt-2 text-sm text-fg-muted">
        Use /grok, /codex, or /opencode.
      </p>
      <a href="/" className="mt-6 inline-block text-sm underline">
        Back to hub
      </a>
    </div>
  ),
  errorComponent: ({ error }) => (
    <div className="mx-auto max-w-md px-6 py-24 text-center">
      <h1 className="text-lg font-semibold">Failed to load</h1>
      <p className="mt-2 text-sm text-fg-muted">
        {error instanceof Error ? error.message : String(error)}
      </p>
    </div>
  ),
});

function ProductPage() {
  const { product, schema } = Route.useLoaderData();
  return <ConfigApp product={product} schema={schema} />;
}
