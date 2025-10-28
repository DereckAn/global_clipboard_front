export interface AppSettings {
  // General
  maxLocalItems: number;
  autoSaveClipboard: boolean;

  // UI
  theme: "light" | "dark" | "system";

  // Hotkeys
  showHotkey: string;

  // Advanced
  enableAnalytics: boolean;
}

export const DEFAULT_SETTINGS: AppSettings = {
  maxLocalItems: 1000,
  autoSaveClipboard: true,
  theme: "system",
  showHotkey: "CommandOrControl+Shift+V",
  enableAnalytics: false,
};
