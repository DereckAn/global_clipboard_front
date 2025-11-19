import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render } from "@testing-library/svelte";
import TabItemsSettings from "../TabItemsSettings.svelte";

const baseProps = () => ({
  maxItemsEnabled: false,
  maxLocalItems: 100,
  retentionEnabled: false,
  retentionDays: 30,
  onToggleMaxItems: vi.fn(),
  onUpdateMaxItems: vi.fn(),
  onToggleRetention: vi.fn(),
  onUpdateRetentionDays: vi.fn(),
});

afterEach(() => {
  cleanup();
});

describe("TabItemsSettings", () => {
  it("calls onToggleMaxItems when toggle is clicked", () => {
    const props = baseProps();
    const { getByLabelText } = render(TabItemsSettings, { props });

    const toggle = getByLabelText(
      "Toggle saved items limit",
    ) as HTMLButtonElement;
    fireEvent.click(toggle);

    expect(props.onToggleMaxItems).toHaveBeenCalledTimes(1);
  });

  it("propagates slider changes when limit is enabled", () => {
    const props = baseProps();
    props.maxItemsEnabled = true;
    const { container } = render(TabItemsSettings, { props });

    const slider = container.querySelector(
      "#max-items-range",
    ) as HTMLInputElement | null;
    expect(slider).toBeTruthy();
    if (slider) {
      slider.value = "800";
      slider.dispatchEvent(new Event("input", { bubbles: true }));
    }

    expect(props.onUpdateMaxItems).toHaveBeenCalledWith(800);
  });

  it("calls onUpdateRetentionDays when an option is selected", () => {
    const props = baseProps();
    props.retentionEnabled = true;
    const { container } = render(TabItemsSettings, { props });

    const button = Array.from(
      container.querySelectorAll("button"),
    ).find((el) => el.textContent?.includes("1 mes"));
    button?.dispatchEvent(new MouseEvent("click", { bubbles: true }));

    expect(props.onUpdateRetentionDays).toHaveBeenCalledWith(30);
  });
});
