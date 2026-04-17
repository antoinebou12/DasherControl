/** One vue-grid-layout row (matches `GridLayoutMain` / `addItemCustom` shape). */
export type GuestGridItem = {
  x: number;
  y: number;
  w: number;
  h: number;
  i: number;
  static: boolean;
  draggable: boolean;
  resizable: boolean;
  appletData: Record<string, unknown>;
  extra: Record<string, unknown>;
  isFloating: boolean;
  floatLeft: number | null;
  floatTop: number | null;
  floatWidth: number | null;
  floatHeight: number | null;
  floatZ: number | null;
};

const float = {
  isFloating: false,
  floatLeft: null,
  floatTop: null,
  floatWidth: null,
  floatHeight: null,
  floatZ: null,
} as const;

/** Default read-only guest dashboard: host metrics + embedded search (DuckDuckGo embeds more reliably than Google). */
export function buildGuestDashboardLayout(): GuestGridItem[] {
  return [
    {
      x: 0,
      y: 0,
      w: 6,
      h: 16,
      i: 0,
      static: true,
      draggable: false,
      resizable: false,
      appletData: { appletName: "SystemMonitor" },
      extra: {},
      ...float,
    },
    {
      x: 6,
      y: 0,
      w: 6,
      h: 16,
      i: 1,
      static: true,
      draggable: false,
      resizable: false,
      appletData: { appletName: "IFrame", src: "https://duckduckgo.com" },
      extra: {},
      ...float,
    },
  ];
}
