class UiStore {
  // Theme
  theme = $state<"light" | "dark" | "system">("system");

  // Sidebar
  isSidebarCollapsed = $state(false);

  // Modals
  isSettingsOpen = $state(false);

  constructor() {
    // Load from localStorage
    if (typeof window !== "undefined") {
      const saved = localStorage.getItem("ui-store");
      if (saved) {
        const parsed = JSON.parse(saved);
        this.theme = parsed.theme ?? "system";
        this.isSidebarCollapsed = parsed.isSidebarCollapsed ?? false;
      }

      // Apply theme
      this.applyTheme();

      // Watch for changes and save
      $effect(() => {
        localStorage.setItem(
          "ui-store",
          JSON.stringify({
            theme: this.theme,
            isSidebarCollapsed: this.isSidebarCollapsed,
          })
        );

        this.applyTheme();
      });
    }
  }

  setTheme(theme: "light" | "dark" | "system") {
    this.theme = theme;
  }

  toggleSidebar() {
    this.isSidebarCollapsed = !this.isSidebarCollapsed;
  }

  private applyTheme() {
    if (typeof window === "undefined") return;

    const root = document.documentElement;
    root.classList.remove("light", "dark");

    if (this.theme === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)")
        .matches
        ? "dark"
        : "light";
      root.classList.add(systemTheme);
    } else {
      root.classList.add(this.theme);
    }
  }
}

export const uiStore = new UiStore();
