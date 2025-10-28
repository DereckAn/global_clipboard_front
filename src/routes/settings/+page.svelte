<script lang="ts">
  import { goto } from "$app/navigation";
  import Icon from "$lib/components/icons/Icon.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";

  let maxLocalItems = $state(settingsStore.maxLocalItems.toString());
  let autoSaveClipboard = $state(settingsStore.autoSaveClipboard);
  let showHotkey = $state(settingsStore.showHotkey);
  let enableAnalytics = $state(settingsStore.enableAnalytics);

  const handleBack = () => {
    goto("/");
  };

  const handleSave = () => {
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

    // Show success message (simple console for now)
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

          <!-- Show hotkey -->
          <div>
            <label
              for="hotkey-input"
              class="block text-sm font-medium text-text mb-2"
            >
              Show window hotkey
            </label>
            <Input
              type="text"
              bind:value={showHotkey}
              placeholder="Ctrl+Shift+V"
              class="w-48"
            />
            <p class="text-xs text-text-muted mt-1">
              Keyboard shortcut to show the clipboard manager
            </p>
          </div>
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
