export type FieldType = "boolean" | "number" | "string" | "enum" | "string-list";

export type ConfigField = {
  id: string;
  path: string;
  label: string;
  description: string;
  type: FieldType;
  group: string;
  section?: string;
  default?: unknown;
  options?: string[];
  env?: string;
  cli?: string;
  recommended?: boolean;
};

export type ConfigGroup = {
  id: string;
  title: string;
  description: string;
};

export type ConfigPreset = {
  id: string;
  name: string;
  description: string;
  enabled: string[];
  values: Record<string, unknown>;
};

export type CliFlag = {
  flag: string;
  category: string;
  description: string;
};

export type EnvVar = {
  name: string;
  category: string;
  description: string;
};

export type Subcommand = {
  cmd: string;
  desc: string;
};

export type ProductSchema = {
  product: string;
  productTitle?: string;
  product_title?: string;
  productTagline?: string;
  product_tagline?: string;
  versionNote?: string;
  version_note?: string;
  format?: "toml" | "json";
  configPath?: string;
  config_path?: string;
  fields: ConfigField[];
  groups: ConfigGroup[];
  presets: ConfigPreset[];
  cliFlags?: CliFlag[];
  cli_flags?: CliFlag[];
  envVars?: EnvVar[];
  env_vars?: EnvVar[];
  subcommands?: Subcommand[];
};

export type ProductId = "grok" | "codex" | "opencode";

export const PRODUCTS: {
  id: ProductId;
  title: string;
  path: string;
  configPath: string;
  blurb: string;
  accent: string;
  links?: { label: string; href: string }[];
}[] = [
  {
    id: "grok",
    title: "Grok Build",
    path: "/grok",
    configPath: "~/.grok/config.toml",
    blurb: "Titanium · xbgst + livepatch ban — always-approve, xbgst-stack marketplaces, flat subagents.",
    accent: "titanium",
    links: [
      { label: "livepatch", href: "https://github.com/VeigaPunk/grok-build-livepatch" },
      { label: "marketplace", href: "https://github.com/VeigaPunk/grok-marketplace" },
    ],
  },
  {
    id: "codex",
    title: "Codex Titanium",
    path: "/codex",
    configPath: "~/.codex/config.toml",
    blurb: "Unrestricted Titanium profile · multi_agent_v2@64 · optimal with Sekhmet swarms.",
    accent: "codex",
    links: [
      { label: "Sekhmet", href: "https://github.com/VeigaPunk/xbrd-spark" },
      { label: "codex-titanium", href: "https://github.com/VeigaPunk/codex-titanium" },
    ],
  },
  {
    id: "opencode",
    title: "OpenCode Titanium",
    path: "/opencode",
    configPath: "opencode.json",
    blurb: "Wild Titanium build — agents, MCP, plan mode, and sharp OpenCode defaults.",
    accent: "opencode",
  },
];

export function normalizeSchema(raw: ProductSchema): ProductSchema {
  return {
    ...raw,
    productTitle: raw.productTitle ?? raw.product_title ?? raw.product,
    productTagline: raw.productTagline ?? raw.product_tagline ?? "",
    versionNote: raw.versionNote ?? raw.version_note ?? "Titanium schemas",
    configPath: raw.configPath ?? raw.config_path ?? "",
    format: raw.format ?? "toml",
    cliFlags: raw.cliFlags ?? raw.cli_flags ?? [],
    envVars: raw.envVars ?? raw.env_vars ?? [],
    subcommands: raw.subcommands ?? [],
  };
}

export function isProductId(v: string): v is ProductId {
  return v === "grok" || v === "codex" || v === "opencode";
}
