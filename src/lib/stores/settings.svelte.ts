import {
  tauriCleanupExcessItems,
  tauriCleanupOldItems,
  tauriDisableAutoStart,
  tauriEnableAutoStart,
  tauriEnableFeature,
  tauriGetCleanupSettings,
  tauriGetDatabaseStats,
  tauriGetLabFeatures,
  tauriGetSetting,
  tauriInstallFeature,
  tauriIsAutoStartEnabled,
  tauriIsTrayVisible,
  tauriOptimizeDatabase,
  tauriQuitApp,
  tauriSaveCleanupSettings,
  tauriSetTrayVisible,
  tauriUninstallFeature,
  tauriUpdateGlobalHotkey,
  type DatabaseStats,
} from "$lib/tauri/commands";
import type { LabFeatureId, LabFeatureWithMeta } from "$lib/types";
import { DEFAULT_SETTINGS } from "$lib/types";
import { listen } from "@tauri-apps/api/event";

interface Settings {
  hotkey: string;
}

const HOTKEY_MODIFIERS = ["Command", "Control", "Alt", "Option", "Shift"];
const normalizeHotkey = (hotkey: string) => {
  return hotkey
    .split("+")
    .map((p) => p.trim())
    .filter(Boolean);
};

const isValidHotkey = (hotkey: string) => {
  if (!hotkey) return false;
  const parts = normalizeHotkey(hotkey);
  if (parts.length < 2) return false;
  const hasModifier = parts.some((p) => HOTKEY_MODIFIERS.includes(p));
  const hasMain = parts.some((p) => !HOTKEY_MODIFIERS.includes(p));
  return hasModifier && hasMain;
};

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
  screenshotHotkeyFull = $state("CommandOrControl+Shift+3");
  screenshotHotkeyRegion = $state("CommandOrControl+Shift+4");

  autoStartEnabled = $state(false);
  trayIconVisible = $state(false);

  dbStats = $state<DatabaseStats | null>(null);
  labFeatures = $state<LabFeatureWithMeta[]>([]);

  constructor() {
    // Load from localStorage
    if (typeof window !== "undefined") {
      listen("lab://install-progress", (event) => {
        const { id, progress } = event.payload as {
          id: LabFeatureId;
          progress: number;
        };
        this.labFeatures = this.labFeatures.map((feature) =>
          feature.id === id ? { ...feature, progress } : feature
        );
      });

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
          this.screenshotHotkeyFull =
            parsed.screenshotHotkeyFull && isValidHotkey(parsed.screenshotHotkeyFull)
              ? parsed.screenshotHotkeyFull
              : "CommandOrControl+Shift+3";
          this.screenshotHotkeyRegion =
            parsed.screenshotHotkeyRegion && isValidHotkey(parsed.screenshotHotkeyRegion)
              ? parsed.screenshotHotkeyRegion
              : "CommandOrControl+Shift+4";
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

  updateScreenshotHotkeys(full: string, region: string) {
    if (isValidHotkey(full)) {
      this.screenshotHotkeyFull = full;
    }
    if (isValidHotkey(region)) {
      this.screenshotHotkeyRegion = region;
    }
    this.save();
  }

  reset() {
    this.maxLocalItems = DEFAULT_SETTINGS.maxLocalItems;
    this.showHotkey = DEFAULT_SETTINGS.showHotkey;
    this.enableAnalytics = DEFAULT_SETTINGS.enableAnalytics;
    this.maxItemsEnabled = DEFAULT_SETTINGS.maxItemsEnabled;
    this.retentionEnabled = DEFAULT_SETTINGS.retentionEnabled;
    this.retentionDays = DEFAULT_SETTINGS.retentionDays;
    this.screenshotHotkeyFull = "CommandOrControl+Shift+3";
    this.screenshotHotkeyRegion = "CommandOrControl+Shift+4";
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

    try {
      this.labFeatures = await tauriGetLabFeatures();
    } catch (err) {
      console.error("Failed to load lab features:", err);
    }
  }

  async saveHotkey(newHotkey: string) {
    this.isLoading = true;
    this.error = null;

    try {
      if (!isValidHotkey(newHotkey)) {
        this.error = "Hotkey must include a modifier and a key";
        return false;
      }
      console.debug("[hotkey] registering", newHotkey);
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

  async refreshLabFeatures() {
    try {
      this.labFeatures = await tauriGetLabFeatures();
    } catch (err) {
      console.error("Failed to refresh lab features:", err);
      throw err;
    }
  }

  async installFeature(id: LabFeatureId) {
    try {
      const updated = await tauriInstallFeature(id);
      this.labFeatures = this.labFeatures
        .filter((f) => f.id !== id)
        .concat(updated);
      return updated;
    } catch (err) {
      console.error("Failed to install feature:", err);
      throw err;
    }
  }

  async uninstallFeature(id: LabFeatureId) {
    try {
      const updated = await tauriUninstallFeature(id);
      this.labFeatures = this.labFeatures
        .filter((f) => f.id !== id)
        .concat(updated);
      return updated;
    } catch (err) {
      console.error("Failed to uninstall feature:", err);
      throw err;
    }
  }

  async enableFeature(id: LabFeatureId, enabled: boolean) {
    try {
      const updated = await tauriEnableFeature(id, enabled);
      this.labFeatures = this.labFeatures
        .filter((f) => f.id !== id)
        .concat(updated);
      return updated;
    } catch (err) {
      console.error("Failed to enable feature:", err);
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
            screenshotHotkeyFull: this.screenshotHotkeyFull,
            screenshotHotkeyRegion: this.screenshotHotkeyRegion,
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
