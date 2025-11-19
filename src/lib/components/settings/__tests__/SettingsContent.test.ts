import { cleanup, fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("$lib/stores/settings.svelte", () => {
  const store = {
    hotkey: "CommandOrControl+Shift+V",
    enableAnalytics: false,
    maxItemsEnabled: true,
    maxLocalItems: 400,
    retentionEnabled: true,
    retentionDays: 30,
    toggleMaxItemsEnabled: vi.fn(),
    updateMaxItems: vi.fn(),
    toggleRetentionEnabled: vi.fn(),
    updateRetentionDays: vi.fn(),
    toggleAnalytics: vi.fn(),
    saveHotkey: vi.fn().mockResolvedValue(true),
    reset: vi.fn(),
  };
  return { settingsStore: store };
});

import { settingsStore } from "$lib/stores/settings.svelte";
import SettingsContent from "../SettingsContent.svelte";

const renderComponent = () => render(SettingsContent, { activeTab: "items" });

afterEach(() => {
  cleanup();
  vi.clearAllMocks();
});

describe("SettingsContent → Item Management tab", () => {
  it("calls settingsStore actions when interacting with controls", () => {
    const { getByLabelText, container } = renderComponent();

    const limitToggle = getByLabelText(
      "Toggle saved items limit",
    ) as HTMLButtonElement;
    fireEvent.click(limitToggle);
    expect(settingsStore.toggleMaxItemsEnabled).toHaveBeenCalledTimes(1);

    const slider = container.querySelector(
      "#max-items-range",
    ) as HTMLInputElement | null;
    if (slider) {
      slider.value = "900";
      slider.dispatchEvent(new Event("input", { bubbles: true }));
    }
    expect(settingsStore.updateMaxItems).toHaveBeenCalledWith(900);

    const retentionButton = Array.from(
      container.querySelectorAll("button"),
    ).find((el) => el.textContent?.includes("1 año"));
    retentionButton?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    expect(settingsStore.updateRetentionDays).toHaveBeenCalledWith(365);
  });
});
