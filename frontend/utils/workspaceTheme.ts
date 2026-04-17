/** Preset palettes (Dracula-inspired defaults match App.vue :root). */
export const THEME_PRESETS: Record<string, { bg: string; fg: string; accent: string; panel: string }> = {
  dracula: { bg: "#282a36", fg: "#f8f8f2", accent: "#bd93f9", panel: "#44475a" },
  nord: { bg: "#2e3440", fg: "#eceff4", accent: "#88c0d0", panel: "#3b4252" },
  midnight: { bg: "#0d1117", fg: "#e6edf3", accent: "#58a6ff", panel: "#161b22" },
  paper: { bg: "#faf8f5", fg: "#1a1a1a", accent: "#2563eb", panel: "#e8e4dc" },
};

export interface WorkspaceAppearance {
  workspace_icon: string;
  theme_preset: string;
  bg_color: string | null;
  fg_color: string | null;
  accent_color: string | null;
  panel_color: string | null;
  background_image_url: string | null;
}

function hexToRgbTriplet(hex: string): string | null {
  if (!hex || typeof hex !== "string") return null;
  const h = hex.replace("#", "").trim();
  if (h.length === 3) {
    const r = parseInt(h[0] + h[0], 16);
    const g = parseInt(h[1] + h[1], 16);
    const b = parseInt(h[2] + h[2], 16);
    if (Number.isNaN(r)) return null;
    return `${r}, ${g}, ${b}`;
  }
  if (h.length !== 6) return null;
  const r = parseInt(h.slice(0, 2), 16);
  const g = parseInt(h.slice(2, 4), 16);
  const b = parseInt(h.slice(4, 6), 16);
  if ([r, g, b].some((n) => Number.isNaN(n))) return null;
  return `${r}, ${g}, ${b}`;
}

function pickColor(override: string | null | undefined, fallback: string): string {
  const o = override && String(override).trim();
  if (o && /^#?[0-9a-fA-F]{3,8}$/.test(o.replace("#", ""))) {
    return o.startsWith("#") ? o : `#${o}`;
  }
  return fallback;
}

/**
 * Apply workspace appearance to the document (CSS variables + Naive/Vuesax-friendly tokens).
 */
export function applyWorkspaceThemeToDocument(appearance: WorkspaceAppearance): void {
  const presetName = appearance.theme_preset || "dracula";
  const base = THEME_PRESETS[presetName] || THEME_PRESETS.dracula;
  const bg = pickColor(appearance.bg_color, base.bg);
  const fg = pickColor(appearance.fg_color, base.fg);
  const accent = pickColor(appearance.accent_color, base.accent);
  const panel = pickColor(appearance.panel_color, base.panel);
  const bgRgb = hexToRgbTriplet(bg) || "40, 42, 54";
  const fgRgb = hexToRgbTriplet(fg) || "248, 248, 242";

  const root = document.documentElement;
  root.style.setProperty("--bg", bg);
  root.style.setProperty("--fg", fg);
  root.style.setProperty("--cl", panel);
  root.style.setProperty("--comment", panel);
  root.style.setProperty("--bg-rgb", bgRgb);
  root.style.setProperty("--fg-rgb", fgRgb);
  root.style.setProperty("--vs-primary", accent);
  root.style.setProperty("--vs-color", hexToRgbTriplet(accent) || "189, 147, 249");

  const url = appearance.background_image_url && String(appearance.background_image_url).trim();
  if (url) {
    const safe = url.replace(/"/g, "%22");
    root.style.setProperty("--workspace-bg-image", `url("${safe}")`);
  } else {
    root.style.removeProperty("--workspace-bg-image");
  }

  const appEl = document.getElementById("app");
  if (appEl) {
    appEl.setAttribute("vs-theme", presetName === "paper" ? "light" : "dark");
  }
}

export function defaultWorkspaceAppearance(): WorkspaceAppearance {
  return {
    workspace_icon: "",
    theme_preset: "dracula",
    bg_color: null,
    fg_color: null,
    accent_color: null,
    panel_color: null,
    background_image_url: null,
  };
}

function optColor(v: unknown): string | null {
  if (v == null) return null;
  const s = String(v).trim();
  return s.length ? s : null;
}

export function workspaceAppearanceFromApiRow(row: Record<string, unknown> | null | undefined): WorkspaceAppearance {
  const d = defaultWorkspaceAppearance();
  if (!row || typeof row !== "object") return d;
  return {
    workspace_icon: row.workspace_icon != null ? String(row.workspace_icon) : "",
    theme_preset: (row.theme_preset as string) || "dracula",
    bg_color: optColor(row.bg_color),
    fg_color: optColor(row.fg_color),
    accent_color: optColor(row.accent_color),
    panel_color: optColor(row.panel_color),
    background_image_url: optColor(row.background_image_url),
  };
}

export function appearancePayloadForApi(appearance: WorkspaceAppearance) {
  const opt = (v: string | null | undefined) => {
    if (v == null) return null;
    const s = String(v).trim();
    return s.length ? s : null;
  };
  return {
    workspace_icon: appearance.workspace_icon != null ? String(appearance.workspace_icon) : "",
    theme_preset: appearance.theme_preset || "dracula",
    bg_color: opt(appearance.bg_color),
    fg_color: opt(appearance.fg_color),
    accent_color: opt(appearance.accent_color),
    panel_color: opt(appearance.panel_color),
    background_image_url: opt(appearance.background_image_url),
  };
}

/** True if the icon field should be rendered as Boxicons classes (not emoji/text). */
export function isBoxiconClass(icon: string): boolean {
  const s = icon && String(icon).trim();
  if (!s) return false;
  if (/^bx\s/i.test(s)) return true;
  if (/^bx-/i.test(s)) return true;
  return false;
}

/** Build class list for `<i>`: always includes `bx`, plus user-supplied tokens. */
export function boxiconClassList(icon: string): string[] {
  const t = String(icon || "")
    .trim()
    .split(/\s+/)
    .filter(Boolean);
  if (!t.length) return ["bx", "bx-grid-alt"];
  if (t[0] === "bx") return t;
  return ["bx", ...t];
}
