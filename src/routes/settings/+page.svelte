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

<div class="h-screen w-screen overflow-hidden flex flex-col bg-background">
  <!-- Header -->
  <div
    class="h-16 bg-surface border-b border-border px-6 flex items-center gap-4"
  >
    <Button variant="ghost" size="icon" onclick={handleBack}>
      <Icon name="chevronRight" size={20} class="rotate-180" />
    </Button>
    <h1 class="text-lg font-semibold">Settings</h1>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-y-auto">
    <div class="max-w-3xl mx-auto px-6 py-8">
      <!-- Account Section -->
      <section class="mb-8">
        <h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
          <Icon name="user" size={20} />
          Account
        </h2>
        <div class="bg-surface rounded-lg border border-border p-6">
          <div class="mb-4">
            <label
              for="email-input"
              class="block text-sm font-medium text-text-muted mb-2"
            >
              Email
            </label>
            <Input type="email" placeholder="Not signed in" disabled value="" />
            <p class="text-xs text-text-muted mt-1">
              Sign in to sync your clipboard across devices
            </p>
          </div>

          <div class="mb-4">
            <p class="block text-sm font-medium text-text-muted mb-2">
              Subscription
            </p>
            <div class="flex items-center justify-between">
              <span class="text-sm">Free Tier</span>
              <Button variant="outline" size="sm">Upgrade to Pro</Button>
            </div>
          </div>
        </div>
      </section>

      <!-- Clipboard Settings -->
      <section class="mb-8">
        <h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
          <Icon name="text" size={20} />
          Clipboard
        </h2>
        <div class="bg-surface rounded-lg border border-border p-6 space-y-6">
          <!-- Max local items -->
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

          <!-- Auto-save clipboard -->
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
        </div>
      </section>

      <!-- Límite de Items (NUEVO) -->
      <section class="mb-8">
        <h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
          <Icon name="trash" size={20} />
          Gestión de Items
        </h2>
        <div class="bg-surface rounded-lg border border-border p-6 space-y-6">
          <!-- Toggle límite -->
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
              onclick={() => {
                settingsStore.toggleMaxItemsEnabled();
              }}
              class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${settingsStore.maxItemsEnabled ? "bg-primary" : "bg-border"}`}
              role="switch"
              aria-checked={settingsStore.maxItemsEnabled}
              aria-label="Toggle limit saved items"
            >
              <span
                class={`inline-block h-4 w-4 transform rounded-full  bg-white transition-transform ${
                  settingsStore.maxItemsEnabled
                    ? "translate-x-6"
                    : "translate-x-1"
                }`}
              ></span>
            </button>
          </div>

          <!-- Slider límite -->
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
                  settingsStore.updateMaxItems(parseInt(e.currentTarget.value))}
                class="w-full"
              />
              <div class="flex justify-between text-xs text-text-muted mt-1">
                <span>100</span>
                <span>1000</span>
                <span>5000</span>
              </div>
            </div>
          {/if}

          <!-- Retención (NUEVO) -->
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
              aria-checked={settingsStore.retentionEnabled}
              aria-label="Toggle retention of old items"
              class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                settingsStore.retentionEnabled ? "bg-primary" : "bg-border"
              }`}
              role="switch"
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
                  class="px-3 py-2 text-sm rounded transition-colors {settingsStore.retentionDays ===
                  days
                    ? 'bg-primary text-white'
                    : 'bg-background text-text-muted hover:bg-surface-hover'}"
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
        </div>
      </section>

      <!-- Uso de Memoria (NUEVO) -->
      <section class="mb-8">
        <h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
          <Icon name="trash" size={20} />
          Uso de Memoria
        </h2>
        <div class="bg-surface rounded-lg border border-border p-6 space-y-4">
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
        </div>
      </section>

      <!-- Privacy Settings -->
      <section class="mb-8">
        <h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
          <Icon name="settings" size={20} />
          Privacy
        </h2>
        <div class="bg-surface rounded-lg border border-border p-6">
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
              aria-checked={enableAnalytics}
              aria-label="Toggle analytics"
            >
              <span
                class={`inline-block h-4 w-4 transform rounded-full  bg-white transition-transform ${
                  enableAnalytics ? "translate-x-6" : "translate-x-1"
                }`}
              ></span>
            </button>
          </div>
        </div>
      </section>

      <!-- Application Settings -->
      <section class="mb-8">
        <h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
          <Icon name="settings" size={20} />
          Application
        </h2>
        <div
          class="bg-surface rounded-lg border border-border p-6
  space-y-6"
        >
          <!-- Auto-start toggle -->
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
              aria-checked={settingsStore.autoStartEnabled}
              aria-label="Toggle auto-start"
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

          {#if settingsStore.error}
            <div
              class="px-3 py-2 rounded-lg text-sm bg-danger-20 text-danger flex items-center gap-2"
            >
              <Icon name="x" size={16} />
              <span>{settingsStore.error}</span>
            </div>
          {/if}
        </div>
      </section>

      <!-- About Section -->
      <section class="mb-8">
        <h2 class="text-xl font-semibold mb-4">About</h2>
        <div class="bg-surface rounded-lg border border-border p-6 space-y-3">
          <div class="flex justify-between">
            <span class="text-sm text-text-muted">Version</span>
            <span class="text-sm text-text">1.0.0</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-text-muted">Platform</span>
            <span class="text-sm text-text">Tauri v2</span>
          </div>
        </div>
      </section>

      <!-- Actions -->
      <div class="flex flex-col gap-3">
        <!-- Primary actions -->
        <div class="flex gap-3">
          <Button onclick={handleSave}>Save Changes</Button>
          <Button variant="outline" onclick={handleBack}>Cancel</Button>
          <Button variant="destructive" onclick={handleReset} class="ml-auto">
            Reset to Defaults
          </Button>
        </div>

        <!-- Danger zone -->
        <div class="border-t border-border pt-4 mt-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-text mb-1">Quit Application</p>
              <p class="text-xs text-text-muted">
                Close the app completely (stops background monitoring)
              </p>
            </div>

            {#if !showQuitConfirm}
              <!-- Initial Quit button -->
              <button
                type="button"
                class="inline-flex items-center justify-center gap-2
  rounded-lg px-4 py-2 text-sm font-medium transition-colors
  bg-danger hover:bg-danger-hover text-white"
                onclick={() => {
                  console.log("🔵 Quit button clicked - showing confirmation");
                  showQuitConfirm = true;
                }}
              >
                <Icon name="x" size={16} />
                Quit App
              </button>
            {:else}
              <!-- Confirmation buttons -->
              <div class="flex items-center gap-2">
                <span class="text-xs text-text-muted mr-2">Are you sure?</span>
                <button
                  type="button"
                  class="inline-flex items-center justify-center gap-2
  rounded-lg px-3 py-2 text-sm font-medium transition-colors
  bg-danger hover:bg-danger-hover text-white"
                  onclick={async () => {
                    console.log(
                      "🟢 User confirmed quit - calling quitApplication..."
                    );
                    try {
                      await settingsStore.quitApplication();
                      console.log("✅ quitApplication completed");
                    } catch (error) {
                      console.error("❌ Error calling quitApplication:", error);
                    }
                  }}
                >
                  <Icon name="check" size={16} />
                  Yes, Quit
                </button>
                <button
                  type="button"
                  class="inline-flex items-center justify-center gap-2
  rounded-lg px-3 py-2 text-sm font-medium transition-colors
  bg-surface hover:bg-surface-hover text-text"
                  onclick={() => {
                    console.log("🟡 User cancelled quit");
                    showQuitConfirm = false;
                  }}
                >
                  <Icon name="x" size={16} />
                  Cancel
                </button>
              </div>
            {/if}
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
