<script lang="ts">
  import type { TabIcon } from "$lib/types";
  import Icon from "../icons/Icon.svelte";
  import Button from "../ui/Button.svelte";

  type Tab = { id: string; label: string; icon: string };

  interface Props {
    tabs: Tab[];
    activeTab: string;
    onTabChange?: (tabId: string) => void;
    onBack?: () => void;
    version?: string;
    tauriVersion?: string;
  }

  let {
    tabs,
    activeTab,
    onTabChange,
    onBack,
    version = "1.0.1",
    tauriVersion = "Tauri v2",
  }: Props = $props();
</script>

<aside
  class="w-64 bg-surface backdrop-blur-xs border rounded-2xl border-border/60 p-4 flex flex-col gap-4"
>
  <div class="flex items-center gap-2 px-2 text-sm text-text-muted">
    <Button
      variant="ghost"
      size="icon"
      onclick={() => onBack?.()}
      class="rounded-full"
    >
      <Icon name="chevronRight" size={20} class="rotate-180 " />
    </Button>
    <span class="font-semibold text-text">Navigation</span>
  </div>
  <nav class="flex-1 space-y-1 overflow-y-auto">
    {#each tabs as tab}
      <button
        class={`w-full flex items-center gap-3 px-3 py-2 rounded-xl text-sm transition-colors focus:outline-none focus:underline focus:underline-offset-2 ${
          activeTab === tab.id
            ? "bg-primary/10 text-primary"
            : "text-text-muted hover:bg-surface-hover"
        }`}
        onclick={() => onTabChange?.(tab.id)}
      >
        <Icon name={tab.icon as TabIcon} size={18} />
        <span>{tab.label}</span>
      </button>
    {/each}
  </nav>
  <div class="px-2 text-xs text-text-muted">
    Version {version} · {tauriVersion}
  </div>
</aside>
