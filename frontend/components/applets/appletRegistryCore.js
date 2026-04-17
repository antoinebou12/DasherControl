import UnknownApplet from "./UnknownApplet.vue";

/** @typedef {{ id: string, label?: string, component: object, showInCreatePicker?: boolean, settingComponent?: object|null, applyCreate?: (parentVm: object, settingFormRef: object|null) => void, toolbar?: { showOpenInNewTab?: boolean, showReload?: boolean } }} AppletDescriptor */

/** @type {AppletDescriptor[]} */
const appletDescriptors = [];

const ID_ALIASES = {
  "Choose Applets": "CreateNew",
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
