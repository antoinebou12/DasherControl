/** Default category key when a descriptor omits `category`. */
export const DEFAULT_APPLET_CATEGORY = "other";

/**
 * Display order and labels for create-picker folders. Unknown keys still work:
 * they sort after these, ordered alphabetically by label.
 * @type {{ key: string, label: string }[]}
 */
export const ORDERED_APPLET_CATEGORIES = [
  { key: "content", label: "Content" },
  { key: "metrics", label: "Metrics" },
  { key: "tools", label: "Tools" },
  { key: "ai", label: "AI" },
  { key: "other", label: "Other" },
];

/**
 * @param {string} key
 * @returns {string}
 */
export function getCategoryLabel(key) {
  const found = ORDERED_APPLET_CATEGORIES.find((c) => c.key === key);
  if (found) return found.label;
  if (!key) return getCategoryLabel(DEFAULT_APPLET_CATEGORY);
  return key.charAt(0).toUpperCase() + key.slice(1);
}
