<script lang="ts">
  import { settingsStore } from "$lib/stores/settings.svelte";
  import SetttingsSaveButtons from "./SetttingsSaveButtons.svelte";
  import TabAboutSettings from "./tabs/TabAboutSettings.svelte";
  import TabAccountSettings from "./tabs/TabAccountSettings.svelte";
  import TabApplicationSettings from "./tabs/TabApplicationSettings.svelte";
  import TabHotkeysSettings from "./tabs/TabHotkeysSettings.svelte";
  import TacItemsSettings from "./tabs/TabItemsSettings.svelte";
  import TabLaboratorySettings from "./tabs/TabLaboratorySettings.svelte";
  import TabMemorySettings from "./tabs/TabMemorySettings.svelte";
  import TabPrivacySettings from "./tabs/TabPrivacySettings.svelte";

  interface Props {
    activeTab: string;
    onBack?: () => void;
  }

  let { activeTab, onBack }: Props = $props();
  let enableAnalytics = $state(settingsStore.enableAnalytics);
  let globalHotkey = $state(settingsStore.hotkey);
  let screenshotHotkeyFull = $state(settingsStore.screenshotHotkeyFull);
  let screenshotHotkeyRegion = $state(settingsStore.screenshotHotkeyRegion);
  let hotkeyFeedback = $state<{
    type: "success" | "error";
    message: string;
  } | null>(null);
  let showQuitConfirm = $state(false);

  // settingsStore.hotkey loads asynchronously from the backend, so the initial
  // snapshot above is the placeholder default. Mirror the store once the real
  // value arrives (the global hotkey saves immediately, so they stay in sync).
  $effect(() => {
    globalHotkey = settingsStore.hotkey;
  });

  const handleSave = async () => {
    if (enableAnalytics !== settingsStore.enableAnalytics) {
      settingsStore.toggleAnalytics();
    }
    if (
      screenshotHotkeyFull !== settingsStore.screenshotHotkeyFull ||
      screenshotHotkeyRegion !== settingsStore.screenshotHotkeyRegion
    ) {
      settingsStore.updateScreenshotHotkeys(
        screenshotHotkeyFull,
        screenshotHotkeyRegion
      );
    }
    if (globalHotkey !== settingsStore.hotkey) {
      const ok = await settingsStore.saveHotkey(globalHotkey);
      hotkeyFeedback = {
        type: ok ? "success" : "error",
        message: ok ? "Hotkey saved" : "Failed to save hotkey",
      };
      setTimeout(() => (hotkeyFeedback = null), 2500);
    }
  };

  const handleReset = () => {
    settingsStore.reset();
    enableAnalytics = settingsStore.enableAnalytics;
    globalHotkey = settingsStore.hotkey;
    screenshotHotkeyFull = settingsStore.screenshotHotkeyFull;
    screenshotHotkeyRegion = settingsStore.screenshotHotkeyRegion;
  };

  const handleCancel = () => {
    enableAnalytics = settingsStore.enableAnalytics;
    globalHotkey = settingsStore.hotkey;
    screenshotHotkeyFull = settingsStore.screenshotHotkeyFull;
    screenshotHotkeyRegion = settingsStore.screenshotHotkeyRegion;
    onBack?.();
  };
</script>

<div class="flex-1 overflow-y-auto  space-y-10">
  {#if activeTab === "account"}
    <TabAccountSettings />
  {/if}

  {#if activeTab === "hotkeys"}
    <TabHotkeysSettings
      bind:globalHotkey
      bind:screenshotHotkeyFull
      bind:screenshotHotkeyRegion
      {hotkeyFeedback}
    />
  {/if}

  {#if activeTab === "items"}
    <TacItemsSettings
      maxItemsEnabled={settingsStore.maxItemsEnabled}
      maxLocalItems={settingsStore.maxLocalItems}
      retentionEnabled={settingsStore.retentionEnabled}
      retentionDays={settingsStore.retentionDays}
      onToggleMaxItems={() => settingsStore.toggleMaxItemsEnabled()}
      onUpdateMaxItems={(value) => settingsStore.updateMaxItems(value)}
      onToggleRetention={() => settingsStore.toggleRetentionEnabled()}
      onUpdateRetentionDays={(days) => settingsStore.updateRetentionDays(days)}
    />
  {/if}

  {#if activeTab === "memory"}
    <TabMemorySettings />
  {/if}

  {#if activeTab === "privacy"}
    <TabPrivacySettings bind:enableAnalytics />
  {/if}

  {#if activeTab === "application"}
    <TabApplicationSettings bind:showQuitConfirm />
  {/if}

  {#if activeTab === "laboratory"}
    <TabLaboratorySettings />
  {/if}

  {#if activeTab === "about"}
    <TabAboutSettings />
  {/if}

  <SetttingsSaveButtons
    onSave={handleSave}
    onReset={handleReset}
    onCancel={handleCancel}
  />
</div>
