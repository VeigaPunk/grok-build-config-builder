import type { ProductId, ProductSchema } from "./schema-types";
import { normalizeSchema } from "./schema-types";
import grok from "../../public/schemas/grok.json";
import codex from "../../public/schemas/codex.json";
import opencode from "../../public/schemas/opencode.json";

const SCHEMAS: Record<ProductId, ProductSchema> = {
  grok: normalizeSchema(grok as ProductSchema),
  codex: normalizeSchema(codex as ProductSchema),
  opencode: normalizeSchema(opencode as ProductSchema),
};

export function getSchema(product: ProductId): ProductSchema {
  return SCHEMAS[product];
}
