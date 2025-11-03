import {
  tauriCleanupExcessItems,
  tauriCleanupOldItems,
  tauriGetDatabaseStats,
  tauriGetSetting,
  tauriOptimizeDatabase,
  tauriUpdateGlobalHotkey,
  type DatabaseStats,
} from "$lib/tauri/commands";
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

  maxItemsEnabled = $state(DEFAULT_SETTINGS.maxItemsEnabled);
  retentionEnabled = $state(DEFAULT_SETTINGS.retentionEnabled);
  retentionDays = $state(DEFAULT_SETTINGS.retentionDays);
  clipboardMonitorInterval = $state(DEFAULT_SETTINGS.clipboardMonitorInterval);
  notificationsEnabled = $state(DEFAULT_SETTINGS.notificationsEnabled);
  notificationSound = $state(DEFAULT_SETTINGS.notificationSound);
  autoStart = $state(DEFAULT_SETTINGS.autoStart);

  dbStats = $state<DatabaseStats | null>(null);

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

          // NUEVOS campos
          this.maxItemsEnabled =
            parsed.maxItemsEnabled ?? DEFAULT_SETTINGS.maxItemsEnabled;
          this.retentionEnabled =
            parsed.retentionEnabled ?? DEFAULT_SETTINGS.retentionEnabled;
          this.retentionDays =
            parsed.retentionDays ?? DEFAULT_SETTINGS.retentionDays;
          this.clipboardMonitorInterval =
            parsed.clipboardMonitorInterval ??
            DEFAULT_SETTINGS.clipboardMonitorInterval;
          this.notificationsEnabled =
            parsed.notificationsEnabled ??
            DEFAULT_SETTINGS.notificationsEnabled;
          this.notificationSound =
            parsed.notificationSound ?? DEFAULT_SETTINGS.notificationSound;
          this.autoStart = parsed.autoStart ?? DEFAULT_SETTINGS.autoStart;
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

  toggleMaxItemsEnabled() {
    this.maxItemsEnabled = !this.maxItemsEnabled;
    this.save();
  }

  updateMaxItems(value: number) {
    this.maxLocalItems = value;
    this.save();
  }

  toggleRetentionEnabled() {
    this.retentionEnabled = !this.retentionEnabled;
    this.save();
  }

  updateRetentionDays(days: number) {
    this.retentionDays = days;
    this.save();
  }

  reset() {
    this.maxLocalItems = DEFAULT_SETTINGS.maxLocalItems;
    this.autoSaveClipboard = DEFAULT_SETTINGS.autoSaveClipboard;
    this.showHotkey = DEFAULT_SETTINGS.showHotkey;
    this.enableAnalytics = DEFAULT_SETTINGS.enableAnalytics;
    this.maxItemsEnabled = DEFAULT_SETTINGS.maxItemsEnabled;
    this.retentionEnabled = DEFAULT_SETTINGS.retentionEnabled;
    this.retentionDays = DEFAULT_SETTINGS.retentionDays;
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

  async loadDatabaseStats() {
    try {
      this.dbStats = await tauriGetDatabaseStats();
    } catch (err) {
      console.error("Failed to load database stats:", err);
    }
  }

  async cleanupOldItems() {
    if (!this.retentionEnabled) return 0;

    try {
      const deleted = await tauriCleanupOldItems(
        this.retentionDays > 0 ? this.retentionDays : null
      );
      await this.loadDatabaseStats();
      return deleted;
    } catch (err) {
      console.error("Failed to cleanup old items:", err);
      throw err;
    }
  }

  async cleanupExcessItems() {
    if (!this.maxItemsEnabled) return 0;

    try {
      const deleted = await tauriCleanupExcessItems(
        this.maxLocalItems > 0 ? this.maxLocalItems : null
      );
      await this.loadDatabaseStats();
      return deleted;
    } catch (err) {
      console.error("Failed to cleanup excess items:", err);
      throw err;
    }
  }

  async optimizeDatabase() {
    try {
      await tauriOptimizeDatabase();
      await this.loadDatabaseStats();
    } catch (err) {
      console.error("Failed to optimize database:", err);
      throw err;
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
            maxItemsEnabled: this.maxItemsEnabled,
            retentionEnabled: this.retentionEnabled,
            retentionDays: this.retentionDays,
            clipboardMonitorInterval: this.clipboardMonitorInterval,
            notificationsEnabled: this.notificationsEnabled,
            notificationSound: this.notificationSound,
            autoStart: this.autoStart,
          })
        );
      } catch (err) {
        console.error("Failed to save settings store:", err);
      }
    }
  }
}

export const settingsStore = new SettingsStore();
