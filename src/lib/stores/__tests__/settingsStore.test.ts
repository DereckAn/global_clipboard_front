import { beforeEach, describe, expect, it, vi } from "vitest";

// Mock de los comandos de Tauri ANTES de importar el store
vi.mock("$lib/tauri/commands", () => {
  const resolvedCleanupSettings = {
    maxItemsEnabled: true,
    maxLocalItems: 500,
    retentionEnabled: true,
    retentionDays: 30,
  };

  return {
    tauriCleanupExcessItems: vi.fn().mockResolvedValue(0),
    tauriCleanupOldItems: vi.fn().mockResolvedValue(0),
    tauriDisableAutoStart: vi.fn().mockResolvedValue(undefined),
    tauriEnableAutoStart: vi.fn().mockResolvedValue(undefined),
    tauriEnableFeature: vi.fn().mockResolvedValue(undefined),
    tauriGetDatabaseStats: vi.fn().mockResolvedValue(null),
    tauriGetLabFeatures: vi.fn().mockResolvedValue([]),
    tauriGetSetting: vi.fn().mockResolvedValue("CommandOrControl+Shift+V"),
    tauriInstallFeature: vi.fn().mockResolvedValue(undefined),
    tauriIsAutoStartEnabled: vi.fn().mockResolvedValue(false),
    tauriIsTrayVisible: vi.fn().mockResolvedValue(false),
    tauriOptimizeDatabase: vi.fn().mockResolvedValue(undefined),
    tauriQuitApp: vi.fn().mockResolvedValue(undefined),
    tauriSaveCleanupSettings: vi.fn().mockResolvedValue(undefined),
    tauriSetTrayVisible: vi.fn().mockResolvedValue(undefined),
    tauriUninstallFeature: vi.fn().mockResolvedValue(undefined),
    tauriUpdateGlobalHotkey: vi.fn().mockResolvedValue(undefined),
    tauriGetCleanupSettings: vi.fn().mockResolvedValue(resolvedCleanupSettings),
  };
});

// Mock del evento listen de Tauri
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

import { SettingsStore } from "$lib/stores/settings.svelte";
import {
  tauriGetCleanupSettings,
  tauriSaveCleanupSettings,
} from "$lib/tauri/commands";

describe("SettingsStore cleanup settings", () => {
  let store: SettingsStore;

  beforeEach(() => {
    vi.clearAllMocks();
    // Limpiar localStorage antes de cada test
    localStorage.clear();
    store = new SettingsStore();
  });

  it("loads persisted cleanup settings from backend", async () => {
    await store.loadSettings();

    expect(tauriGetCleanupSettings).toHaveBeenCalledTimes(1);
    expect(store.maxItemsEnabled).toBe(true);
    expect(store.maxLocalItems).toBe(500);
    expect(store.retentionEnabled).toBe(true);
    expect(store.retentionDays).toBe(30);
  });

  it("persists values when toggling max item limit", () => {
    // Verificar estado inicial (false por defecto)
    const initialValue = store.maxItemsEnabled;

    store.toggleMaxItemsEnabled();

    // Después del toggle, debe ser el opuesto
    expect(store.maxItemsEnabled).toBe(!initialValue);
    expect(tauriSaveCleanupSettings).toHaveBeenCalledWith(
      !initialValue,
      store.maxLocalItems,
      store.retentionEnabled,
      store.retentionDays
    );
  });

  it("persists new retention days", () => {
    store.updateRetentionDays(90);

    expect(store.retentionDays).toBe(90);
    expect(tauriSaveCleanupSettings).toHaveBeenCalledWith(
      store.maxItemsEnabled,
      store.maxLocalItems,
      store.retentionEnabled,
      90
    );
  });
});
