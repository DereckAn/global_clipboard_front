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
        try {
          const parsed = JSON.parse(saved);
          this.maxLocalItems =
            parsed.maxLocalItems ?? DEFAULT_SETTINGS.maxLocalItems;
          this.autoSaveClipboard =
            parsed.autoSaveClipboard ?? DEFAULT_SETTINGS.autoSaveClipboard;
          this.showHotkey = parsed.showHotkey ?? DEFAULT_SETTINGS.showHotkey;
          this.enableAnalytics =
            parsed.enableAnalytics ?? DEFAULT_SETTINGS.enableAnalytics;
        } catch (err) {
          console.error("Failed to load settings store:", err);
        }
      }
    }
  }

  updateMaxLocalItems(value: number) {
    this.maxLocalItems = value;
    this.save();
  }

  toggleAutoSave() {
    this.autoSaveClipboard = !this.autoSaveClipboard;
    this.save();
  }

  updateShowHotkey(key: string) {
    this.showHotkey = key;
    this.save();
  }

  toggleAnalytics() {
    this.enableAnalytics = !this.enableAnalytics;
    this.save();
  }

  reset() {
    this.maxLocalItems = DEFAULT_SETTINGS.maxLocalItems;
    this.autoSaveClipboard = DEFAULT_SETTINGS.autoSaveClipboard;
    this.showHotkey = DEFAULT_SETTINGS.showHotkey;
    this.enableAnalytics = DEFAULT_SETTINGS.enableAnalytics;
    this.save();
  }

  private save() {
    if (typeof window !== "undefined") {
      try {
        localStorage.setItem(
          "settings-store",
          JSON.stringify({
            maxLocalItems: this.maxLocalItems,
            autoSaveClipboard: this.autoSaveClipboard,
            showHotkey: this.showHotkey,
            enableAnalytics: this.enableAnalytics,
          })
        );
      } catch (err) {
        console.error("Failed to save settings store:", err);
      }
    }
  }
}

export const settingsStore = new SettingsStore();
