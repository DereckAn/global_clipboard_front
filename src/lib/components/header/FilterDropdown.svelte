<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import type { ContentType } from "$lib/types";
  import { cn } from "$lib/utils/cn";

  interface Props {
    selected: "all" | "favorites" | ContentType;
    onSelect: (value: "all" | "favorites" | ContentType) => void;
  }

  let { selected, onSelect }: Props = $props();

  let isOpen = $state(false);

  const options: Array<{
    value: "all" | "favorites" | ContentType;
    label: string;
    icon: keyof typeof import("$lib/components/icons/icons.svelte").iconPaths;
  }> = [
    { value: "all", label: "All Types", icon: "text" },
    { value: "favorites", label: "Favorites Only", icon: "star" },
    { value: "text", label: "Text Only", icon: "text" },
    { value: "code", label: "Code Only", icon: "code" },
    { value: "link", label: "Links Only", icon: "link" },
    { value: "svg", label: "SVG Only", icon: "image" },
    { value: "image", label: "Images Only", icon: "image" },
    { value: "file", label: "Files Only", icon: "file" },
    { value: "color", label: "Colors Only", icon: "color" },
  ];

  const selectedOption = $derived(
    options.find((opt) => opt.value === selected) || options[0]
  );

  const handleSelect = (value: string) => {
    onSelect(value as "all" | "favorites" | ContentType);
    isOpen = false;
  };

  const toggleDropdown = () => {
    isOpen = !isOpen;
  };

  // Close on click outside
  $effect(() => {
    if (!isOpen) return;

    const handleClickOutside = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (!target.closest("[data-filter-dropdown]")) {
        isOpen = false;
      }
    };

    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  });
</script>

<div class="relative bg-surface" data-filter-dropdown>
  <!-- Trigger button -->
  <button
    onclick={toggleDropdown}
    class="flex items-center gap-2 px-3 py-1 w-48 rounded-md border border-border hover:bg-surface-hover transition-colors text-sm"
  >
    <Icon name={selectedOption.icon} size={18} />
    <span class="w-full text-start">{selectedOption.label}</span>
    <Icon
      name="chevronDown"
      size={18}
      class={cn("transition-transform", isOpen && "rotate-180")}
    />
  </button>

  <!-- Dropdown menu -->
  {#if isOpen}
    <div
      class="absolute top-full mt-2 right-0 px-2 py-1 w-48 bg-surface backdrop-blur-sm border border-border rounded-md shadow-lg z-50"
    >
      {#each options as option}
        <button
          onclick={() => handleSelect(option.value)}
          class={cn(
            "flex items-center w-full gap-3 px-4 py-1.5 rounded-xl text-sm hover:bg-white/10 transition-colors text-left",
            selected === option.value && "bg-primary/10 text-primary"
          )}
        >
          <Icon name={option.icon} size={16} />
          <span>{option.label}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>
<style></style>