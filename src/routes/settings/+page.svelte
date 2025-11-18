<script lang="ts">
  import { goto } from "$app/navigation";
  import SettingsContent from "$lib/components/settings/SettingsContent.svelte";
  import SettingsSidebar from "$lib/components/settings/SettingsSidebar.svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";
  import type { TabIcon, TabId } from "$lib/types";
  import { onMount } from "svelte";

  let globalHotkey = $state(settingsStore.hotkey);
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
</script>

<div
  class="h-screen w-screen overflow-hidden flex flex-col bg-surface rounded-3xl relative"
>
  <div
    class="absolute bg-transparent w-full h-full"
    data-tauri-drag-region
  ></div>
  <div class="flex-1 flex overflow-hidden">
    <!-- Sidebar -->
    <SettingsSidebar
      {tabs}
      {activeTab}
      onTabChange={(tabId) => (activeTab = tabId as TabId)}
      onBack={handleBack}
    />

    <!-- Content -->
    <SettingsContent {activeTab} onBack={handleBack} />
  </div>
</div>
