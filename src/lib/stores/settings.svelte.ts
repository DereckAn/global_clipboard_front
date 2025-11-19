import {
  tauriCleanupExcessItems,
  tauriCleanupOldItems,
  tauriDisableAutoStart,
  tauriEnableAutoStart,
  tauriGetDatabaseStats,
  tauriGetSetting,
  tauriGetCleanupSettings,
  tauriIsAutoStartEnabled,
  tauriIsTrayVisible,
  tauriOptimizeDatabase,
  tauriQuitApp,
  tauriSaveCleanupSettings,
  tauriSetTrayVisible,
  tauriUpdateGlobalHotkey,
  type DatabaseStats,
} from "$lib/tauri/commands";
import { DEFAULT_SETTINGS } from "$lib/types";

interface Settings {
  hotkey: string;
}
export class SettingsStore {
  maxLocalItems = $state(DEFAULT_SETTINGS.maxLocalItems);
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

  autoStartEnabled = $state(false);
  trayIconVisible = $state(false);

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
        } catch (err) {
          console.error("Failed to load settings store:", err);
        }
      }
    }
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
      console.log("No saved hotkey found, using default");
    }

    // Load autostart status
    try {
      this.autoStartEnabled = await tauriIsAutoStartEnabled();
    } catch (err) {
      console.log("Could not check autostart status");
    } finally {
      this.isLoading = false;
    }

    try {
      this.trayIconVisible = await tauriIsTrayVisible();
    } catch (err) {
      console.log("Could not check tray icon visibility");
    }

    try {
      const cleanupSettings = await tauriGetCleanupSettings();
      this.maxItemsEnabled = cleanupSettings.maxItemsEnabled;
      this.maxLocalItems = cleanupSettings.maxLocalItems;
      this.retentionEnabled = cleanupSettings.retentionEnabled;
      this.retentionDays = cleanupSettings.retentionDays;
      this.save();
    } catch (err) {
      console.error("Failed to load cleanup settings:", err);
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

  async toggleAutoStart() {
    this.isLoading = true;
    this.error = null;

    try {
      if (this.autoStartEnabled) {
        await tauriDisableAutoStart();
        this.autoStartEnabled = false;
        console.log("Auto-start disabled");
      } else {
        await tauriEnableAutoStart();
        this.autoStartEnabled = true;
        console.log("Auto-start enabled");
      }
    } catch (err) {
      this.error =
        err instanceof Error ? err.message : "Failed to toggle auto-start";
      console.error("Failed to toggle auto-start:", err);
      // Revert state on error
      this.autoStartEnabled = !this.autoStartEnabled;
    } finally {
      this.isLoading = false;
    }
  }

  async toggleTrayIcon() {
    this.isLoading = true;
    this.error = null;

    try {
      const newVisibility = !this.trayIconVisible;
      await tauriSetTrayVisible(newVisibility);
      this.trayIconVisible = newVisibility;
      console.log(`Tray icon visibility set to ${newVisibility}`);
    } catch (err) {
      this.error =
        err instanceof Error
          ? err.message
          : "Could not toggle tray icon visibility";
      console.log("Could not toggle tray icon visibility", err);
    } finally {
      this.isLoading = false;
    }
  }

  async quitApplication() {
    console.log("🔵 quitApplication() called in store");
    try {
      console.log("🟢 Calling tauriQuitApp()...");
      await tauriQuitApp();
      console.log("✅ tauriQuitApp() returned successfully");
    } catch (err) {
      console.error("❌ Failed to quit app:", err);
      throw err; // Re-throw para que el error suba
    }
  }

  private save() {
    if (typeof window !== "undefined") {
      try {
        // Save to localStorage (for frontend)
        localStorage.setItem(
          "settings-store",
          JSON.stringify({
            maxLocalItems: this.maxLocalItems,
            showHotkey: this.showHotkey,
            enableAnalytics: this.enableAnalytics,
            maxItemsEnabled: this.maxItemsEnabled,
            retentionEnabled: this.retentionEnabled,
            retentionDays: this.retentionDays,
            clipboardMonitorInterval: this.clipboardMonitorInterval,
            notificationsEnabled: this.notificationsEnabled,
            notificationSound: this.notificationSound,
          })
        );

        // Also save cleanup settings to file (for background task)
        tauriSaveCleanupSettings(
          this.maxItemsEnabled,
          this.maxLocalItems,
          this.retentionEnabled,
          this.retentionDays
        ).catch((err) => {
          console.error("Failed to save cleanup settings to file:", err);
        });
      } catch (err) {
        console.error("Failed to save settings store:", err);
      }
    }
  }
}

export const settingsStore = new SettingsStore();
