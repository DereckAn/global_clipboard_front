import { tauriGetSetting, tauriUpdateGlobalHotkey } from "$lib/tauri/commands";
import { DEFAULT_SETTINGS } from "$lib/types";

interface Settings {
  hotkey: string;
}
class SettingsStore {
  maxLocalItems = $state(DEFAULT_SETTINGS.maxLocalItems);
  autoSaveClipboard = $state(DEFAULT_SETTINGS.autoSaveClipboard);
  showHotkey = $state(DEFAULT_SETTINGS.showHotkey);
  enableAnalytics = $state(DEFAULT_SETTINGS.enableAnalytics);
  hotkey = $state<string>("CommandOrControl+Shift+V");
  isLoading = $state(false);
  error = $state<string | null>(null);

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

  async loadSettings() {
    this.isLoading = true;
    this.error = null;

    try {
      const savedHotkey = await tauriGetSetting("hotkey");
      if (savedHotkey) {
        this.hotkey = savedHotkey;
      }
    } catch (err) {
      // Si no existe el archivo, usar el default
      console.log("No saved hotkey found, using default");
    } finally {
      this.isLoading = false;
    }
  }

  async saveHotkey(newHotkey: string) {
    this.isLoading = true;
    this.error = null;

    try {
      await tauriUpdateGlobalHotkey(newHotkey);
      this.hotkey = newHotkey;
      return true;
    } catch (err) {
      this.error = err instanceof Error ? err.message : "Failed to save hotkey";
      console.error("Failed to save hotkey:", err);
      return false;
    } finally {
      this.isLoading = false;
    }
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
