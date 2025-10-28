class UiStore {
  // Selected item for right panel
  selectedItemId = $state<string | null>(null);

  constructor() {
    // Load from localStorage
    if (typeof window !== "undefined") {
      const saved = localStorage.getItem("ui-store");
      if (saved) {
        try {
          const parsed = JSON.parse(saved);
          this.selectedItemId = parsed.selectedItemId ?? null;
        } catch (err) {
          console.error("Failed to load UI store:", err);
        }
      }
    }
  }

  selectItem(id: string | null) {
    this.selectedItemId = id;
    this.save();
  }

  private save() {
    if (typeof window !== "undefined") {
      try {
        localStorage.setItem(
          "ui-store",
          JSON.stringify({
            selectedItemId: this.selectedItemId,
          })
        );
      } catch (err) {
        console.error("Failed to save UI store:", err);
      }
    }
  }
}

export const uiStore = new UiStore();
