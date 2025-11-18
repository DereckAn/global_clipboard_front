<script lang="ts">
  import { goto } from "$app/navigation";
  import Icon from "$lib/components/icons/Icon.svelte";
  import HotkeyRecorder from "$lib/components/settings/HotkeyRecorder.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";
  import { onMount } from "svelte";

  let maxLocalItems = $state(settingsStore.maxLocalItems.toString());
  let autoSaveClipboard = $state(settingsStore.autoSaveClipboard);
  let showHotkey = $state(settingsStore.showHotkey);
  let enableAnalytics = $state(settingsStore.enableAnalytics);
  let globalHotkey = $state(settingsStore.hotkey);
  let hotkeyFeedback = $state<{
    type: "success" | "error";
    message: string;
  } | null>(null);
  let showQuitConfirm = $state(false);
  type TabId =
    | "account"
    | "clipboard"
    | "items"
    | "memory"
    | "privacy"
    | "application"
    | "laboratory"
    | "about";

  type TabIcon =
    | "user"
    | "text"
    | "trash"
    | "database"
    | "lock"
    | "settings"
    | "flask"
    | "info";

  let activeTab = $state<TabId>("account");

  const tabs: { id: TabId; label: string; icon: TabIcon }[] = [
    { id: "account", label: "Account", icon: "user" },
    { id: "clipboard", label: "Clipboard", icon: "text" },
    { id: "items", label: "Item Management", icon: "trash" },
    { id: "memory", label: "Usage", icon: "database" },
    { id: "privacy", label: "Privacy", icon: "lock" },
    { id: "application", label: "Application", icon: "settings" },
    { id: "laboratory", label: "Laboratory", icon: "flask" },
    { id: "about", label: "About", icon: "info" },
  ];

  onMount(async () => {
    await settingsStore.loadSettings();
    await settingsStore.loadDatabaseStats();
    globalHotkey = settingsStore.hotkey;
  });

  const handleBack = () => {
    goto("/");
  };

  const handleSave = async () => {
    // AGREGADO: async
    // Convert string to number
    const maxItems = parseInt(maxLocalItems, 10);
    if (!isNaN(maxItems)) {
      settingsStore.updateMaxLocalItems(maxItems);
    }

    if (autoSaveClipboard !== settingsStore.autoSaveClipboard) {
      settingsStore.toggleAutoSave();
    }

    if (enableAnalytics !== settingsStore.enableAnalytics) {
      settingsStore.toggleAnalytics();
    }

    settingsStore.updateShowHotkey(showHotkey);

    // Save global hotkey if changed
    if (globalHotkey !== settingsStore.hotkey) {
      const success = await settingsStore.saveHotkey(globalHotkey);
      if (success) {
        hotkeyFeedback = {
          type: "success",
          message: "Hotkey saved! Restart the app for changes to take effect.",
        };
      } else {
        hotkeyFeedback = {
          type: "error",
          message: "Failed to save hotkey. Please try again.",
        };
      }

      // Clear feedback after 5 seconds
      setTimeout(() => {
        hotkeyFeedback = null;
      }, 5000);
    }

    // Show success message
    console.log("Settings saved!");

    // Go back
    goto("/");
  };

  const handleReset = () => {
    if (confirm("Are you sure you want to reset all settings to default?")) {
      settingsStore.reset();
      maxLocalItems = settingsStore.maxLocalItems.toString();
      autoSaveClipboard = settingsStore.autoSaveClipboard;
      showHotkey = settingsStore.showHotkey;
      enableAnalytics = settingsStore.enableAnalytics;
      globalHotkey = "CommandOrControl+Shift+V";
    }
  };
</script>

<div
  class="h-screen w-screen overflow-hidden flex flex-col bg-surface rounded-3xl"
>
  <div class="flex-1 flex overflow-hidden ">
    <!-- Sidebar -->
    <aside
      class="w-64 bg-surface backdrop-blur-xs border m-3 rounded-2xl border-border/60 p-4 flex flex-col gap-4"
      data-tauri-drag-region
    >
      <div class="flex items-center gap-2 px-2 text-sm text-text-muted">
        <Button variant="ghost" size="icon" onclick={handleBack}>
          <Icon name="chevronRight" size={20} class="rotate-180" />
        </Button>
        <span class="font-semibold text-text">Settings</span>
      </div>
      <nav class="flex-1 space-y-1 overflow-y-auto">
        {#each tabs as tab}
          <button
            class={`w-full flex items-center gap-3 px-3 py-2 rounded-xl text-sm transition-colors ${
              activeTab === tab.id
                ? "bg-primary/10 text-primary"
                : "text-text-muted hover:bg-surface-hover"
            }`}
            onclick={() => (activeTab = tab.id)}
          >
            <Icon name={tab.icon} size={18} />
            <span>{tab.label}</span>
          </button>
        {/each}
      </nav>
      <div class="px-2 text-xs text-text-muted">Version 1.0.0 · Tauri v2</div>
    </aside>

    <!-- Content -->
    <div class="flex-1 flex flex-col " data-tauri-drag-region>
      

      <div class="flex-1 overflow-y-auto py-4 pr-3 space-y-10 ">
        {#if activeTab === "account"}
          <section class="bg-surface rounded-2xl border border-border/60 p-6">
            <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Icon name="user" size={20} />
              Account
            </h2>
            <div class="space-y-4">
              <div>
                <label
                  for="email-input"
                  class="block text-sm font-medium text-text-muted mb-2"
                >
                  Email
                </label>
                <Input
                  type="email"
                  placeholder="Not signed in"
                  disabled
                  value=""
                />
                <p class="text-xs text-text-muted mt-1">
                  Sign in to sync your clipboard across devices
                </p>
              </div>

              <div class="flex items-center justify-between">
                <div>
                  <p class="block text-sm font-medium text-text-muted mb-1">
                    Subscription
                  </p>
                  <p class="text-xs text-text-muted">Free Tier</p>
                </div>
                <Button variant="outline" size="sm">Upgrade to Pro</Button>
              </div>
            </div>
          </section>
        {/if}

        {#if activeTab === "clipboard"}
          <section
            class="bg-surface rounded-2xl border border-border/60 p-6 space-y-6"
          >
            <h2 class="text-lg font-semibold flex items-center gap-2">
              <Icon name="text" size={20} />
              Clipboard
            </h2>
            <div>
              <label
                for="max-items-input"
                class="block text-sm font-medium text-text mb-2"
              >
                Maximum local items
              </label>
              <Input
                type="text"
                bind:value={maxLocalItems}
                placeholder="1000"
                class="w-32"
              />
              <p class="text-xs text-text-muted mt-1">
                Maximum number of items to store locally
              </p>
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="block text-sm font-medium text-text mb-1">
                  Auto-save clipboard
                </p>
                <p class="text-xs text-text-muted">
                  Automatically save copied content to history
                </p>
              </div>
              <button
                onclick={() => (autoSaveClipboard = !autoSaveClipboard)}
                class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${autoSaveClipboard ? "bg-primary" : "bg-border"}`}
                role="switch"
                aria-checked={autoSaveClipboard}
                aria-label="Toggle auto-save clipboard"
              >
                <span
                  class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    autoSaveClipboard ? "translate-x-6" : "translate-x-1"
                  }`}
                ></span>
              </button>
            </div>

            <div>
              <label
                for="global-hotkey"
                class="block text-sm font-medium text-text mb-2"
              >
                Global window hotkey
              </label>
              <HotkeyRecorder
                id="global-hotkey"
                value={globalHotkey}
                onChange={(newHotkey) => (globalHotkey = newHotkey)}
              />

              {#if hotkeyFeedback}
                <div
                  class="mt-3 px-3 py-2 rounded-lg text-sm flex items-center gap-2"
                  class:bg-primary-20={hotkeyFeedback.type === "success"}
                  class:text-primary={hotkeyFeedback.type === "success"}
                  class:bg-danger-20={hotkeyFeedback.type === "error"}
                  class:text-danger={hotkeyFeedback.type === "error"}
                >
                  {#if hotkeyFeedback.type === "success"}
                    <Icon name="check" size={16} />
                  {:else}
                    <Icon name="x" size={16} />
                  {/if}
                  <span>{hotkeyFeedback.message}</span>
                </div>
              {/if}
            </div>
          </section>
        {/if}

        {#if activeTab === "items"}
          <section
            class="bg-surface rounded-2xl border border-border/60 p-6 space-y-6"
          >
            <h2 class="text-lg font-semibold flex items-center gap-2">
              <Icon name="trash" size={20} />
              Item Management
            </h2>
            <div class="flex items-center justify-between">
              <div>
                <p class="block text-sm font-medium text-text mb-1">
                  Limitar items guardados
                </p>
                <p class="text-xs text-text-muted">
                  Auto-eliminar items antiguos cuando se supera el límite
                </p>
              </div>
              <button
                onclick={() => settingsStore.toggleMaxItemsEnabled()}
                class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${settingsStore.maxItemsEnabled ? "bg-primary" : "bg-border"}`}
                role="switch"
                aria-label="Toggle saved items limit"
                aria-checked={settingsStore.maxItemsEnabled}
              >
                <span
                  class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    settingsStore.maxItemsEnabled
                      ? "translate-x-6"
                      : "translate-x-1"
                  }`}
                ></span>
              </button>
            </div>

            {#if settingsStore.maxItemsEnabled}
              <div>
                <label
                  for="max-items-range"
                  class="block text-sm font-medium text-text mb-2"
                >
                  Máximo: {settingsStore.maxLocalItems} items
                </label>
                <input
                  id="max-items-range"
                  type="range"
                  min="100"
                  max="5000"
                  step="100"
                  value={settingsStore.maxLocalItems}
                  oninput={(e) =>
                    settingsStore.updateMaxItems(
                      parseInt(e.currentTarget.value)
                    )}
                  class="w-full"
                />
                <div class="flex justify-between text-xs text-text-muted mt-1">
                  <span>100</span>
                  <span>1000</span>
                  <span>5000</span>
                </div>
              </div>
            {/if}

            <div class="flex items-center justify-between">
              <div>
                <p class="block text-sm font-medium text-text mb-1">
                  Auto-eliminar items antiguos
                </p>
                <p class="text-xs text-text-muted">
                  Eliminar items después de cierto tiempo
                </p>
              </div>
              <button
                onclick={() => settingsStore.toggleRetentionEnabled()}
                class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  settingsStore.retentionEnabled ? "bg-primary" : "bg-border"
                }`}
                role="switch"
                aria-label="Toggle retention policy"
                aria-checked={settingsStore.retentionEnabled}
              >
                <span
                  class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    settingsStore.retentionEnabled
                      ? "translate-x-6"
                      : "translate-x-1"
                  }`}
                ></span>
              </button>
            </div>

            {#if settingsStore.retentionEnabled}
              <div class="grid grid-cols-3 gap-2">
                {#each [7, 30, 90, 180, 365] as days}
                  <button
                    onclick={() => settingsStore.updateRetentionDays(days)}
                    class={`px-3 py-2 text-sm rounded transition-colors ${
                      settingsStore.retentionDays === days
                        ? "bg-primary text-white"
                        : "bg-background text-text-muted hover:bg-surface-hover"
                    }`}
                  >
                    {days === 7
                      ? "7 días"
                      : days === 30
                        ? "1 mes"
                        : days === 90
                          ? "3 meses"
                          : days === 180
                            ? "6 meses"
                            : "1 año"}
                  </button>
                {/each}
              </div>
            {/if}
          </section>
        {/if}

        {#if activeTab === "memory"}
          <section
            class="bg-surface rounded-2xl border border-border/60 p-6 space-y-4"
          >
            <h2 class="text-lg font-semibold flex items-center gap-2">
              <Icon name="database" size={20} />
              Memory Usage
            </h2>
            <button
              onclick={async () => await settingsStore.loadDatabaseStats()}
              class="text-sm text-primary hover:underline"
            >
              Actualizar estadísticas
            </button>
            {#if settingsStore.dbStats}
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-text-muted">Total de items:</span>
                  <span class="text-text font-medium">
                    {settingsStore.dbStats.total_items}
                  </span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-text-muted">Favoritos:</span>
                  <span class="text-text font-medium">
                    {settingsStore.dbStats.favorites}
                  </span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-text-muted">Snippets:</span>
                  <span class="text-text font-medium">
                    {settingsStore.dbStats.snippets}
                  </span>
                </div>
                <div class="flex justify-between text-sm">
                  <span class="text-text-muted">Tamaño en disco:</span>
                  <span class="text-text font-medium">
                    {settingsStore.dbStats.database_size_mb.toFixed(2)} MB
                  </span>
                </div>
              </div>
              <div class="flex gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onclick={async () => {
                    const deleted = await settingsStore.cleanupOldItems();
                    alert(`${deleted} items eliminados`);
                  }}
                >
                  Limpiar items antiguos
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onclick={async () => {
                    await settingsStore.optimizeDatabase();
                    alert("Base de datos optimizada");
                  }}
                >
                  Optimizar DB
                </Button>
              </div>
            {/if}
          </section>
        {/if}

        {#if activeTab === "privacy"}
          <section class="bg-surface rounded-2xl border border-border/60 p-6">
            <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Icon name="lock" size={20} />
              Privacy
            </h2>
            <div class="flex items-center justify-between">
              <div>
                <p class="block text-sm font-medium text-text mb-1">
                  Enable analytics
                </p>
                <p class="text-xs text-text-muted">
                  Help improve the app by sharing anonymous usage data
                </p>
              </div>
              <button
                onclick={() => (enableAnalytics = !enableAnalytics)}
                class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${enableAnalytics ? "bg-primary" : "bg-border"}`}
                role="switch"
                aria-label="Toggle analytics"
                aria-checked={enableAnalytics}
              >
                <span
                  class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    enableAnalytics ? "translate-x-6" : "translate-x-1"
                  }`}
                ></span>
              </button>
            </div>
          </section>
        {/if}

        {#if activeTab === "application"}
          <section
            class="bg-surface rounded-2xl border border-border/60 p-6 space-y-6"
          >
            <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Icon name="settings" size={20} />
              Application
            </h2>
            <div class="flex items-center justify-between">
              <div>
                <p class="block text-sm font-medium text-text mb-1">
                  Auto-start on system boot
                </p>
                <p class="text-xs text-text-muted">
                  Launch the app automatically when your computer starts
                </p>
              </div>
              <button
                onclick={async () => await settingsStore.toggleAutoStart()}
                class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  settingsStore.autoStartEnabled ? "bg-primary" : "bg-border"
                }`}
                role="switch"
                aria-label="Toggle auto-start"
                aria-checked={settingsStore.autoStartEnabled}
                disabled={settingsStore.isLoading}
              >
                <span
                  class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    settingsStore.autoStartEnabled
                      ? "translate-x-6"
                      : "translate-x-1"
                  }`}
                ></span>
              </button>
            </div>

            <div class="flex items-center justify-between">
              <div>
                <p class="block text-sm font-medium text-text mb-1">
                  Show tray icon
                </p>
                <p class="text-xs text-text-muted">
                  Display icon in system tray/menu bar
                </p>
              </div>
              <button
                onclick={async () => await settingsStore.toggleTrayIcon()}
                class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  settingsStore.trayIconVisible ? "bg-primary" : "bg-border"
                }`}
                role="switch"
                aria-label="Toggle tray icon visibility"
                aria-checked={settingsStore.trayIconVisible}
                disabled={settingsStore.isLoading}
              >
                <span
                  class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                    settingsStore.trayIconVisible
                      ? "translate-x-6"
                      : "translate-x-1"
                  }`}
                ></span>
              </button>
            </div>

            {#if settingsStore.error}
              <div
                class="px-3 py-2 rounded-lg text-sm bg-danger-20 text-danger flex items-center gap-2"
              >
                <Icon name="x" size={16} />
                <span>{settingsStore.error}</span>
              </div>
            {/if}

            <div class="border-t border-border pt-4">
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-sm font-medium text-text mb-1">
                    Quit Application
                  </p>
                  <p class="text-xs text-text-muted">
                    Close the app completely (stops background monitoring)
                  </p>
                </div>
                {#if !showQuitConfirm}
                  <button
                    type="button"
                    class="inline-flex items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium bg-danger text-white"
                    onclick={() => (showQuitConfirm = true)}
                  >
                    <Icon name="x" size={16} />
                    Quit App
                  </button>
                {:else}
                  <div class="flex items-center gap-2">
                    <span class="text-xs text-text-muted mr-2"
                      >Are you sure?</span
                    >
                    <button
                      type="button"
                      class="inline-flex items-center gap-2 rounded-lg px-3 py-2 text-sm font-medium bg-danger text-white"
                      onclick={async () => {
                        try {
                          await settingsStore.quitApplication();
                        } catch (error) {
                          console.error(
                            "Error calling quitApplication:",
                            error
                          );
                        }
                      }}
                    >
                      <Icon name="check" size={16} />
                      Yes, Quit
                    </button>
                    <Button
                      variant="outline"
                      size="sm"
                      onclick={() => (showQuitConfirm = false)}
                    >
                      Cancel
                    </Button>
                  </div>
                {/if}
              </div>
            </div>
          </section>
        {/if}

        {#if activeTab === "laboratory"}
          <section
            class="bg-surface rounded-2xl border border-border/60 p-6 space-y-4"
          >
            <h2 class="text-lg font-semibold mb-2 flex items-center gap-2">
              <Icon name="flask" size={20} />
              Laboratory
            </h2>
            <p class="text-sm text-text-muted">
              Feature playground for experimental tools (OCR downloads, emoji
              packs, LAN features). Coming soon.
            </p>
          </section>
        {/if}

        {#if activeTab === "about"}
          <section
            class="bg-surface rounded-2xl border border-border/60 p-6 space-y-3"
          >
            <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
              <Icon name="info" size={20} />
              About
            </h2>
            <div class="flex justify-between">
              <span class="text-sm text-text-muted">Version</span>
              <span class="text-sm text-text">1.0.0</span>
            </div>
            <div class="flex justify-between">
              <span class="text-sm text-text-muted">Platform</span>
              <span class="text-sm text-text">Tauri v2</span>
            </div>
          </section>
        {/if}

        <div class="flex gap-3">
          <Button onclick={handleSave}>Save Changes</Button>
          <Button variant="outline" onclick={handleBack}>Cancel</Button>
          <Button variant="destructive" onclick={handleReset} class="ml-auto">
            Reset to Defaults
          </Button>
        </div>
      </div>
    </div>
  </div>
</div>
