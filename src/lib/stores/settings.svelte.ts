import { DEFAULT_SETTINGS } from "$lib/types";

class SettingsStore {
  maxLocalItems = $state(DEFAULT_SETTINGS.maxLocalItems);
  autoSaveClipboard = $state(DEFAULT_SETTINGS.autoSaveClipboard);
  showHotkey = $state(DEFAULT_SETTINGS.showHotkey);
  enableAnalytics = $state(DEFAULT_SETTINGS.enableAnalytics);

  constructor() {
    // Load from localStorage
    if (typeof window !== "undefined") {
      const saved = localStorage.getItem("settings-store");
      if (saved) {
        const parsed = JSON.parse(saved);
        this.maxLocalItems =
          parsed.maxLocalItems ?? DEFAULT_SETTINGS.maxLocalItems;
        this.autoSaveClipboard =
          parsed.autoSaveClipboard ?? DEFAULT_SETTINGS.autoSaveClipboard;
        this.showHotkey = parsed.showHotkey ?? DEFAULT_SETTINGS.showHotkey;
        this.enableAnalytics =
          parsed.enableAnalytics ?? DEFAULT_SETTINGS.enableAnalytics;
      }

      // Watch for changes and save
      $effect(() => {
        localStorage.setItem(
          "settings-store",
          JSON.stringify({
            maxLocalItems: this.maxLocalItems,
            autoSaveClipboard: this.autoSaveClipboard,
            showHotkey: this.showHotkey,
            enableAnalytics: this.enableAnalytics,
          })
        );
      });
    }
  }

  updateMaxLocalItems(value: number) {
    this.maxLocalItems = value;
  }

  toggleAutoSave() {
    this.autoSaveClipboard = !this.autoSaveClipboard;
  }

  updateShowHotkey(key: string) {
    this.showHotkey = key;
  }

  toggleAnalytics() {
    this.enableAnalytics = !this.enableAnalytics;
  }

  reset() {
    this.maxLocalItems = DEFAULT_SETTINGS.maxLocalItems;
    this.autoSaveClipboard = DEFAULT_SETTINGS.autoSaveClipboard;
    this.showHotkey = DEFAULT_SETTINGS.showHotkey;
    this.enableAnalytics = DEFAULT_SETTINGS.enableAnalytics;
  }
}

export const settingsStore = new SettingsStore();
