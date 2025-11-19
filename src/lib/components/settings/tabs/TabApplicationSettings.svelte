<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";

  interface Props {
    showQuitConfirm: boolean;
  }

  let { showQuitConfirm = $bindable<boolean>() }: Props = $props();
</script>

<section
  class="relative bg-surface rounded-2xl border border-border/60 p-6 space-y-6"
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
          settingsStore.autoStartEnabled ? "translate-x-6" : "translate-x-1"
        }`}
      ></span>
    </button>
  </div>

  <div class="flex items-center justify-between">
    <div>
      <p class="block text-sm font-medium text-text mb-1">Show tray icon</p>
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
          settingsStore.trayIconVisible ? "translate-x-6" : "translate-x-1"
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
        <p class="text-sm font-medium text-text mb-1">Quit Application</p>
        <p class="text-xs text-text-muted">
          Close the app completely (stops background monitoring)
        </p>
      </div>
      {#if !showQuitConfirm}
        <Button
          variant="destructive"
          onclick={() => (showQuitConfirm = true)}
          class=""
        >
          <Icon name="x" size={16} />
          Quit App
        </Button>
      {:else}
        <div class="flex items-center flex-col gap-2">
          <Button
            variant="destructive"
            onclick={async () => {
              try {
                await settingsStore.quitApplication();
              } catch (error) {
                console.error("Error calling quitApplication:", error);
              }
            }}
          >
            <Icon name="check" size={16} />
            Yes, Quit
          </Button>
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
