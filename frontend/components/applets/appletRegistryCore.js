import UnknownApplet from "./UnknownApplet.vue";
import {
  DEFAULT_APPLET_CATEGORY,
  ORDERED_APPLET_CATEGORIES,
  getCategoryLabel,
} from "./catalog/categoryOrder.js";

/** @typedef {{ id: string, label?: string, category?: string, component: object, showInCreatePicker?: boolean, settingComponent?: object|null, applyCreate?: (parentVm: object, settingFormRef: object|null) => void, toolbar?: { showOpenInNewTab?: boolean, showReload?: boolean } }} AppletDescriptor */

/** @typedef {{ id: string, label: string }} CreatePickerEntry */

/** @typedef {{ categoryKey: string, categoryLabel: string, applets: CreatePickerEntry[] }} CreatePickerCategory */

/** @type {AppletDescriptor[]} */
const appletDescriptors = [];

const ID_ALIASES = {
  "Choose Applets": "CreateNew",
  Assistant: "Chat",
};

/**
 * @param {string|undefined|null} id
 * @returns {string}
 */
export function normalizeAppletId(id) {
  if (id == null || id === "") {
    return "CreateNew";
  }
  return ID_ALIASES[id] || id;
}

/**
 * @param {AppletDescriptor} descriptor
 * @returns {boolean}
 */
export function registerApplet(descriptor) {
  if (!descriptor || !descriptor.id || !descriptor.component) {
    console.warn("registerApplet: invalid descriptor", descriptor);
    return false;
  }
  if (appletDescriptors.some((d) => d.id === descriptor.id)) {
    console.warn(`Applet "${descriptor.id}" is already registered`);
    return false;
  }
  appletDescriptors.push(descriptor);
  return true;
}

/**
 * @param {string|undefined|null} id
 * @returns {AppletDescriptor|undefined}
 */
export function getAppletDescriptor(id) {
  const normalized = normalizeAppletId(id);
  return appletDescriptors.find((d) => d.id === normalized);
}

/**
 * @param {string|undefined|null} appletName
 * @returns {object}
 */
export function getAppletComponent(appletName) {
  const d = getAppletDescriptor(appletName);
  return d ? d.component : UnknownApplet;
}

/**
 * @returns {{ applet: string, label: string }[]}
 */
export function listAppletsForCreate() {
  return appletDescriptors
    .filter((d) => d.showInCreatePicker !== false)
    .map((d) => ({
      applet: d.id,
      label: d.label || d.id,
    }));
}

/**
 * Ordered folder list for the create-applet UI (plain data).
 * @returns {CreatePickerCategory[]}
 */
export function getCreatePickerCatalog() {
  const filtered = appletDescriptors.filter((d) => d.showInCreatePicker !== false);
  /** @type {Map<string, CreatePickerEntry[]>} */
  const byCategory = new Map();
  for (const d of filtered) {
    const cat = d.category && String(d.category).trim() !== "" ? d.category : DEFAULT_APPLET_CATEGORY;
    if (!byCategory.has(cat)) {
      byCategory.set(cat, []);
    }
    byCategory.get(cat).push({
      id: d.id,
      label: d.label || d.id,
    });
  }
  for (const arr of byCategory.values()) {
    arr.sort((a, b) => a.label.localeCompare(b.label));
  }
  const orderIndex = new Map(ORDERED_APPLET_CATEGORIES.map((c, i) => [c.key, i]));
  const unknownRank = ORDERED_APPLET_CATEGORIES.length;
  const keys = [...byCategory.keys()].sort((a, b) => {
    const ia = orderIndex.has(a) ? orderIndex.get(a) : unknownRank;
    const ib = orderIndex.has(b) ? orderIndex.get(b) : unknownRank;
    if (ia !== ib) return ia - ib;
    return getCategoryLabel(a).localeCompare(getCategoryLabel(b));
  });
  return keys.map((categoryKey) => ({
    categoryKey,
    categoryLabel: getCategoryLabel(categoryKey),
    applets: byCategory.get(categoryKey) || [],
  }));
}

/**
 * Naive UI `n-select` options with `type: 'group'` (derived from {@link getCreatePickerCatalog}).
 * @returns {object[]}
 */
export function getCreatePickerSelectOptions() {
  return getCreatePickerCatalog().map(({ categoryKey, categoryLabel, applets }) => ({
    type: "group",
    key: categoryKey,
    label: categoryLabel,
    children: applets.map((a) => ({ label: a.label, value: a.id })),
  }));
}
