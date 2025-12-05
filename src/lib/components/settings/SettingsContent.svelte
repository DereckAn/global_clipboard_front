<script lang="ts">
  import { settingsStore } from "$lib/stores/settings.svelte";
  import SetttingsSaveButtons from "./SetttingsSaveButtons.svelte";
  import TabAboutSettings from "./tabs/TabAboutSettings.svelte";
  import TabAccountSettings from "./tabs/TabAccountSettings.svelte";
  import TabApplicationSettings from "./tabs/TabApplicationSettings.svelte";
  import TabAccountClipboard from "./tabs/TabClipboadSettings.svelte";
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
  let hotkeyFeedback = $state<{
    type: "success" | "error";
    message: string;
  } | null>(null);
  let showQuitConfirm = $state(false);

  const handleSave = async () => {
    if (enableAnalytics !== settingsStore.enableAnalytics) {
      settingsStore.toggleAnalytics();
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
  };

  const handleCancel = () => {
    enableAnalytics = settingsStore.enableAnalytics;
    globalHotkey = settingsStore.hotkey;
    onBack?.();
  };
</script>

<div class="flex-1 overflow-y-auto  space-y-10">
  {#if activeTab === "account"}
    <TabAccountSettings />
  {/if}

  {#if activeTab === "clipboard"}
    <TabAccountClipboard
      bind:globalHotkey
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
