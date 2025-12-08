<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import type { TabIcon } from "$lib/types";
  import HotkeyRecorder from "./HotkeyRecorder.svelte";

  interface Props {
    id: string;
    icon: TabIcon;
    title: string;
    description: string;
    value: string;
    onChange: (hotkey: string) => void;
    disabled?: boolean;
  }

  let {
    id,
    icon,
    title,
    description,
    value,
    onChange,
    disabled = false,
  }: Props = $props();

  const iconContainerClass = $derived.by(() => {
    return "size-10 rounded-xl bg-white/5 flex items-center justify-center";
  });
</script>

<div class={`flex items-center justify-between`}>
  <div class="flex items-center gap-3">
    <div class={iconContainerClass}>
      <Icon name={icon as TabIcon} size={18} />
    </div>
    <div>
      <p class="text-sm font-semibold text-text">{title}</p>
      <p class="text-xs text-text-muted">{description}</p>
    </div>
  </div>
  <HotkeyRecorder {id} {value} {onChange} {disabled} />
</div>
