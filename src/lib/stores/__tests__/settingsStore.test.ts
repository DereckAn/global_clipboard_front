import { beforeEach, describe, expect, it, vi } from "vitest";

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
    tauriGetDatabaseStats: vi.fn().mockResolvedValue(null),
    tauriGetSetting: vi.fn().mockResolvedValue("CommandOrControl+Shift+V"),
    tauriIsAutoStartEnabled: vi.fn().mockResolvedValue(false),
    tauriIsTrayVisible: vi.fn().mockResolvedValue(false),
    tauriOptimizeDatabase: vi.fn().mockResolvedValue(undefined),
    tauriQuitApp: vi.fn().mockResolvedValue(undefined),
    tauriSaveCleanupSettings: vi.fn().mockResolvedValue(undefined),
    tauriSetTrayVisible: vi.fn().mockResolvedValue(undefined),
    tauriUpdateGlobalHotkey: vi.fn().mockResolvedValue(undefined),
    tauriGetCleanupSettings: vi.fn().mockResolvedValue(
      resolvedCleanupSettings,
    ),
  };
});

import {
  tauriGetCleanupSettings,
  tauriSaveCleanupSettings,
} from "$lib/tauri/commands";
import { SettingsStore } from "$lib/stores/settings.svelte";

const mockLocalStorage = {
  getItem: vi.fn(),
  setItem: vi.fn(),
};

const createStore = () => {
  Object.assign(globalThis, {
    window: { localStorage: mockLocalStorage },
    localStorage: mockLocalStorage,
  });

  return new SettingsStore();
};

describe("SettingsStore cleanup settings", () => {
  beforeEach(() => {
    mockLocalStorage.getItem.mockReturnValue(null);
    mockLocalStorage.setItem.mockReset();
    vi.clearAllMocks();
  });

  it("loads persisted cleanup settings from backend", async () => {
    const store = createStore();

    await store.loadSettings();

    expect(tauriGetCleanupSettings).toHaveBeenCalledTimes(1);
    expect(store.maxItemsEnabled).toBe(true);
    expect(store.maxLocalItems).toBe(500);
    expect(store.retentionEnabled).toBe(true);
    expect(store.retentionDays).toBe(30);
  });

  it("persists values when toggling max item limit", () => {
    const store = createStore();

    store.toggleMaxItemsEnabled();

    expect(store.maxItemsEnabled).toBe(true);
    expect(tauriSaveCleanupSettings).toHaveBeenCalledWith(
      true,
      store.maxLocalItems,
      store.retentionEnabled,
      store.retentionDays,
    );
  });

  it("persists new retention days", () => {
    const store = createStore();

    store.updateRetentionDays(90);

    expect(store.retentionDays).toBe(90);
    expect(tauriSaveCleanupSettings).toHaveBeenCalledWith(
      store.maxItemsEnabled,
      store.maxLocalItems,
      store.retentionEnabled,
      90,
    );
  });
});
